use bytes::Buf;
use serde::de::DeserializeOwned;

use crate::{for_all_tuples, request::Request, rpc_serde::RpcDeserializer, GetTypeDescription, TypeDescription};

pub trait Inject<Context, RequestState> where Self: Sized {
    type TExportDefinition: ExportDefinition;
    fn inject<Deserializer: RpcDeserializer>(ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self>;
    fn get_type_description() -> TypeDescription {
        TypeDescription::void()
    }
}

impl<'de, Context, RequestState, T> Inject<Context, RequestState> for T
where
    T: DeserializeOwned + GetTypeDescription,
{
    type TExportDefinition = ExportDefinitionTrue;
    fn inject<Deserializer: RpcDeserializer>(_ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self> {
        if request.data.len() < 4 {
            anyhow::bail!("Unexpected end of message.");
        }
        let size = request.data.get_u32() as usize;
        if size > request.data.len() { anyhow::bail!("Deserialization error: expected {} bytes, but only {} found", size, request.data.len()); }
        let result = request.data.slice(..size);
        request.data.advance(size);
        Ok(Deserializer::deserialize_unfallible(result)?)
    }
    fn get_type_description() -> TypeDescription {
        <Self as GetTypeDescription>::get_type_description()
    }
}

pub trait ExportDefinition {
    const VALUE: bool;
}

pub struct ExportDefinitionTrue;
pub struct ExportDefinitionFalse;

impl ExportDefinition for ExportDefinitionTrue { const VALUE: bool = true; }
impl ExportDefinition for ExportDefinitionFalse { const VALUE: bool = false; }

pub trait GetTupleForExport<T> {
    type Result: ToArgsDescription;
}

impl<T: Into<String>> GetTupleForExport<T> for ((), ExportDefinitionTrue) {
    type Result = (T, );
}

impl<T> GetTupleForExport<T> for ((), ExportDefinitionFalse) {
    type Result = ();
}


macro_rules! make_into_t {
    ($t:ident) => { T };
}

macro_rules! concat_tuple {
    (>) => {};
    (> $( $t:ident $t_idx:tt; )* ) => {
        impl<T: Into<String>> GetTupleForExport<T> for (($( make_into_t!($t), )*), ExportDefinitionTrue) {
            type Result = ($( make_into_t!($t), )* T);
        }

        impl<T: Into<String>> GetTupleForExport<T> for (($( make_into_t!($t), )*), ExportDefinitionFalse) {
            type Result = ($( make_into_t!($t), )*);
        }
    };
    (< $( $t:ident $t_idx:tt; )* ) => {
        impl<T: Into<String>> ToArgsDescription for ($( make_into_t!($t), )*) {
            fn to_args_description<const SIZE: usize>(self, tys: [(bool, TypeDescription); SIZE]) -> Vec<(String, TypeDescription)> {
                tys.into_iter().filter(|(v, _)| *v).zip([$(self.$t_idx.into()),*].into_iter()).map(|((_, ty), name)| (name, ty)).collect()
            }
        }
    };
    ( $ft:ident $ft_idx:tt; $( $t:ident $t_idx:tt; )* ) => {
        concat_tuple!(> $( $t $t_idx; )*);
        concat_tuple!(< $ft $ft_idx; $( $t $t_idx; )*);
    };
}

pub trait ToArgsDescription {
    fn to_args_description<const SIZE: usize>(self, tys: [(bool, TypeDescription); SIZE]) -> Vec<(String, TypeDescription)>;
}

impl ToArgsDescription for () {
    fn to_args_description<const SIZE: usize>(self, _: [(bool, TypeDescription); SIZE]) -> Vec<(String, TypeDescription)> {
        Vec::new()
    }
}

for_all_tuples!(concat_tuple);