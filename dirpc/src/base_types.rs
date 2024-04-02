use crate::{description::{GetTypeDescription, TypeDescription, TypeInfo}, for_all_tuples};

pub struct SerializationHelper {
    pub(crate) chain: Vec<bytes::Bytes>,
}

impl SerializationHelper {
    pub(crate) fn new() -> Self {
        Self {
            chain: Vec::with_capacity(32),
        }
    }
}

macro_rules! get_type_description_number {
    ($($t0:ident $t1:ident;)*) => {
        $(
            impl GetTypeDescription for $t0 {
                fn get_type_description() -> TypeDescription {
                    TypeDescription {
                        module_path: String::new(),
                        name: String::new(),
                        typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::$t1),
                    }
                }
            }
            impl GetTypeDescription for &$t0 {
                fn get_type_description() -> TypeDescription {
                    TypeDescription {
                        module_path: String::new(),
                        name: String::new(),
                        typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::$t1),
                    }
                }
            }
        )*
    };
}

get_type_description_number! {
    i8 I8;
    u8 U8;
    i16 I16;
    u16 U16;
    i32 I32;
    u32 U32;
    i64 I64;
    u64 U64;
    f32 F32;
    f64 F64;
}

impl GetTypeDescription for () {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Void),
        }
    }
}

impl GetTypeDescription for &() {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Void),
        }
    }
}

impl GetTypeDescription for bool {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Bool),
        }
    }
}

impl GetTypeDescription for &bool {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Bool),
        }
    }
}

impl<T, E> GetTypeDescription for Result<T, E>
where
    T: GetTypeDescription,
    E: std::error::Error,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Result(Box::new(T::get_type_description())),
        }
    }
}
impl<T, E> GetTypeDescription for &Result<T, E>
where
    T: GetTypeDescription,
    E: std::error::Error,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Result(Box::new(T::get_type_description())),
        }
    }
}
impl<T> GetTypeDescription for Option<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Option(Box::new(T::get_type_description())),
        }
    }
}
impl<T> GetTypeDescription for &Option<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Option(Box::new(T::get_type_description())),
        }
    }
}
impl GetTypeDescription for bytes::Bytes {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl GetTypeDescription for &bytes::Bytes {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl GetTypeDescription for String {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::String),
        }
    }
}

impl GetTypeDescription for &String {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::String),
        }
    }
}
impl GetTypeDescription for &str {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::String),
        }
    }
}

impl GetTypeDescription for bytes::BytesMut {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl GetTypeDescription for &[u8] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl<const SIZE: usize> GetTypeDescription for [u8; SIZE] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl<const SIZE: usize> GetTypeDescription for &[u8; SIZE] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

macro_rules! ser_de_tuple {
    ( $( $t:ident $t_idx:tt; )* ) => {
        impl<$( $t, )*> GetTypeDescription for ($( $t, )*)
        where
            $($t: GetTypeDescription),*
        {
            fn get_type_description() -> TypeDescription {
                TypeDescription {
                    module_path: String::new(),
                    name: String::new(),
                    typeinfo: TypeInfo::Tuple(vec![$( $t::get_type_description(), )*]),
                }
            }
        }
    };
}

for_all_tuples!(ser_de_tuple);


impl<T> GetTypeDescription for Vec<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Array(Box::new(T::get_type_description())),
        }
    }
}


impl<T> GetTypeDescription for &Vec<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Array(Box::new(T::get_type_description())),
        }
    }
}
