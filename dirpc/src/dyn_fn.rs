use serde::Serialize;

use crate::{context::{ResponseSerializer, ServerContext}, description::{FunctionDescription, GetTypeDescription}, for_all_functions, request::Request};

pub type DynFunction<Context, RequestState> = Box<dyn Fn(&Context, Request<RequestState>) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<bytes::Bytes>> + Send + Sync>> + Send + Sync>;

pub trait IntoDynFunction<Context, RequestState, PhantomGeneric> {
    type NameTuple;
    fn into_dyn_fn(self) -> DynFunction<Context, RequestState>;
    fn get_type_description(names: Self::NameTuple) -> FunctionDescription;
}

impl<Context, RequestState, Fut, R, F> IntoDynFunction<Context, RequestState, (R, )> for F
where
    Context: ServerContext,
    Fut: std::future::Future<Output = R> + Send + Sync + 'static,
    R: Serialize + GetTypeDescription,
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
{
    type NameTuple = ();
    fn into_dyn_fn(self) -> DynFunction<Context, RequestState> {
        Box::new(move |_ctx, _req| {
            let function = self.clone();
            Box::pin(async move {
                <Context::Serializer as ResponseSerializer>::serialize(function().await)
            })
        })
    }
    fn get_type_description(_names: Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: vec![],
            return_type: R::get_type_description(),
        }
    }
}

macro_rules! type_as_string {
    ( $t: ty ) => { StrType };
}

macro_rules! dyn_fn_impl {
    ( $( $t:ident $t_idx:ident; )* ) => {
        impl<Context, RequestState, $($t,)* Fut, R, F, StrType> IntoDynFunction<Context, RequestState, ($($t,)* R, StrType)> for F
        where
            Context: ServerContext,
            $($t: $crate::inject::Inject<Context, RequestState> + GetTypeDescription + Send + Sync + 'static,)*
            Fut: std::future::Future<Output = R> + Send + Sync + 'static,
            R: Serialize + GetTypeDescription,
            F: FnOnce($($t,)*) -> Fut + Clone + Send + Sync + 'static,
            StrType: Into<String>,
        {
            type NameTuple = ($(type_as_string!($t),)*);
            fn into_dyn_fn(self) -> DynFunction<Context, RequestState> {
                Box::new(move |ctx, mut req| {
                    $(let $t_idx = $t::inject(ctx, &mut req);)*
                    let function = self.clone();
                    Box::pin(async move {
                        <Context::Serializer as ResponseSerializer>::serialize(function($($t_idx?,)*).await)
                    })
                })
            }
            fn get_type_description(names: Self::NameTuple) -> FunctionDescription {
                #[allow(non_snake_case)]
                let ($($t,)*) = names;
                FunctionDescription {
                    args_types: vec![$(($t::EXPORT_DEFINITION, $t.into(), $t::get_type_description()),)*],
                    return_type: R::get_type_description(),
                }
            }
        }
    };
}

for_all_functions!(dyn_fn_impl);
