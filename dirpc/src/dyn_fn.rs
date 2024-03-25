use crate::{description::{FunctionDescription, GetTypeDescription}, for_all_functions, base_types::{SerializationHelper, SerializeToBytes}, request::Request};

pub type DynFunction<Context, RequestState> = Box<dyn Fn(&Context, Request<RequestState>) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Vec<bytes::Bytes>>> + Send + Sync>> + Send + Sync>;

pub trait IntoDynFunction<Context, RequestState, PhantomGeneric> {
    fn into_dyn_fn(self) -> DynFunction<Context, RequestState>;
    fn get_type_description() -> FunctionDescription;
}

impl<Context, RequestState, Fut, R, F> IntoDynFunction<Context, RequestState, (R, )> for F
where
    Fut: std::future::Future<Output = R> + Send + Sync + 'static,
    R: SerializeToBytes + GetTypeDescription,
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
{
    fn into_dyn_fn(self) -> DynFunction<Context, RequestState> {
        Box::new(move |_ctx, _req| {
            let function = self.clone();
            Box::pin(async move {
                let mut ser = SerializationHelper::new();
                function().await.serialize_to_bytes(&mut ser)?;
                Ok(ser.chain)
            })
        })
    }
    fn get_type_description() -> FunctionDescription {
        FunctionDescription {
            args_types: vec![],
            return_type: R::get_type_description(),
        }
    }
}

macro_rules! dyn_fn_impl {
    ( $( $t:ident $t_idx:ident; )* ) => {
        impl<Context, RequestState, $($t,)* Fut, R, F> IntoDynFunction<Context, RequestState, ($($t,)* R)> for F
        where
            $($t: $crate::inject::Inject<Context, RequestState> + GetTypeDescription + Send + Sync + 'static,)*
            Fut: std::future::Future<Output = R> + Send + Sync + 'static,
            R: SerializeToBytes + GetTypeDescription,
            F: FnOnce($($t,)*) -> Fut + Clone + Send + Sync + 'static,
        {
            fn into_dyn_fn(self) -> DynFunction<Context, RequestState> {
                Box::new(move |ctx, mut req| {
                    $(let $t_idx = $t::inject(ctx, &mut req);)*
                    let function = self.clone();
                    Box::pin(async move {
                        let mut ser = SerializationHelper::new();
                        function($($t_idx?,)*).await.serialize_to_bytes(&mut ser)?;
                        Ok(ser.chain)
                    })
                })
            }
            fn get_type_description() -> FunctionDescription {
                FunctionDescription {
                    args_types: vec![$($t::get_type_description(),)*],
                    return_type: R::get_type_description(),
                }
            }
        }
    };
}

for_all_functions!(dyn_fn_impl);
