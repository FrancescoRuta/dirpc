use crate::{request::Request, put_into_response::PutIntoResponse, response::Response, inject::Inject, for_all_functions};

pub type DynFunction<'ctx, Context> = Box<dyn Fn(&'ctx Context, Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<Response, Box<dyn std::error::Error + Send + Sync + 'static>>> + Send>>>;

pub trait IntoDynFunction<'ctx, Context, PhantomGeneric> {
    fn into_dyn_fn(self) -> DynFunction<'ctx, Context>;
}

impl<'ctx, Context, Fut, R, F> IntoDynFunction<'ctx, Context, (R, )> for F
where
    Fut: std::future::Future<Output = R> + Send + 'static,
    R: PutIntoResponse,
    F: FnOnce() -> Fut + Clone + Send + 'static,
{
    fn into_dyn_fn(self) -> DynFunction<'ctx, Context> {
        Box::new(move |_ctx, _req| {
            let function = self.clone();
            Box::pin(async move {
                let mut resp = Response::new();
                function().await.put_into_response(&mut resp)?;
                Ok(resp)
            })
        })
    }
}

macro_rules! dyn_fn_impl {
    ( $( $t:ident $t_idx:ident; )* ) => {
        impl<'ctx, Context, $($t,)* Fut, R, F> IntoDynFunction<'ctx, Context, ($($t,)* R)> for F
        where
            $($t: Inject<'ctx, Context> + Send + 'static,)*
            Fut: std::future::Future<Output = R> + Send + 'static,
            R: PutIntoResponse,
            F: FnOnce($($t,)*) -> Fut + Clone + Send + 'static,
        {
            fn into_dyn_fn(self) -> DynFunction<'ctx, Context> {
                Box::new(move |ctx, mut req| {
                    $(let $t_idx = $t::inject(ctx, &mut req);)*
                    let function = self.clone();
                    Box::pin(async move {
                        let mut resp = Response::new();
                        function($($t_idx?,)*).await.put_into_response(&mut resp)?;
                        Ok(resp)
                    })
                })
            }
        }
    };
}

for_all_functions!(dyn_fn_impl);