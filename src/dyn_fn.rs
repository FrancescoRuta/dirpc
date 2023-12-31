use crate::{for_all_functions, io_bytes::{SerializeToBytes, SerializationHelper}, request::Request};

pub type DynFunction<Context, RequestState> = Box<dyn Fn(&Context, Request<RequestState>) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Vec<bytes::Bytes>>> + Send + Sync>> + Send + Sync>;

pub trait IntoDynFunction<Context, RequestState, PhantomGeneric> {
    fn into_dyn_fn(self) -> DynFunction<Context, RequestState>;
}

impl<Context, RequestState, Fut, R, F> IntoDynFunction<Context, RequestState, (R, )> for F
where
    Fut: std::future::Future<Output = R> + Send + Sync + 'static,
    R: SerializeToBytes,
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
}

macro_rules! dyn_fn_impl {
    ( $( $t:ident $t_idx:ident; )* ) => {
        impl<Context, RequestState, $($t,)* Fut, R, F> IntoDynFunction<Context, RequestState, ($($t,)* R)> for F
        where
            $($t: $crate::inject::Inject<Context, RequestState> + Send + Sync + 'static,)*
            Fut: std::future::Future<Output = R> + Send + Sync + 'static,
            R: SerializeToBytes,
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
        }
    };
}

for_all_functions!(dyn_fn_impl);
