use crate::{server::TypeDescription, type_encoding::TypeEncoding, for_all_tuples};

pub trait GetTypeDescription {
    fn get_type_description() -> TypeDescription;
}

impl GetTypeDescription for () {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("void")],
            description: None,
        }
    }
}

impl<T: GetTypeDescription, U: TypeEncoding<EncodedType = T>> GetTypeDescription for U {
    fn get_type_description() -> TypeDescription {
        let mut description = T::get_type_description();
        if description.encoding.is_none() {
            description.encoding = Some(String::from(U::NAME));
            description
        } else {
            TypeDescription {
                encoding: Some(String::from(U::NAME)),
                kind: String::from("container"),
                name: Vec::new(),
                description: Some([(String::from("content"), description)].into()),
            }
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
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("result")],
            description: Some([
                (String::from("result"), T::get_type_description()),
                (String::from("error"), String::get_type_description()),
            ].into()),
        }
    }
}

impl<T> GetTypeDescription for Option<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("option")],
            description: Some([
                (String::from("content"), T::get_type_description()),
            ].into()),
        }
    }
}

impl GetTypeDescription for bytes::Bytes {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("bytes")],
            description: None,
        }
    }
}

impl GetTypeDescription for bytes::BytesMut {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("bytes")],
            description: None,
        }
    }
}

impl GetTypeDescription for Vec<u8> {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("bytes")],
            description: None,
        }
    }
}

impl GetTypeDescription for &str {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("string")],
            description: None,
        }
    }
}

impl GetTypeDescription for String {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("string")],
            description: None,
        }
    }
}

impl<const SIZE: usize> GetTypeDescription for [u8; SIZE] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("bytes")],
            description: None,
        }
    }
}

impl<const SIZE: usize> GetTypeDescription for &[u8; SIZE] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("bytes")],
            description: None,
        }
    }
}

impl GetTypeDescription for bool {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("bool")],
            description: None,
        }
    }
}

impl GetTypeDescription for i8 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("i8")],
            description: None,
        }
    }
}

impl GetTypeDescription for u8 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("u8")],
            description: None,
        }
    }
}

impl GetTypeDescription for i16 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("i16")],
            description: None,
        }
    }
}

impl GetTypeDescription for u16 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("u16")],
            description: None,
        }
    }
}

impl GetTypeDescription for i32 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("i32")],
            description: None,
        }
    }
}

impl GetTypeDescription for u32 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("u32")],
            description: None,
        }
    }
}

impl GetTypeDescription for i64 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("i64")],
            description: None,
        }
    }
}

impl GetTypeDescription for u64 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("u64")],
            description: None,
        }
    }
}

impl GetTypeDescription for f32 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("f32")],
            description: None,
        }
    }
}

impl GetTypeDescription for f64 {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("base"),
            name: vec![String::from("f64")],
            description: None,
        }
    }
}

macro_rules! get_type_description_for_tuple {
    ( $( $t:ident $t_idx:tt; )* ) => {
        impl<$( $t, )*> GetTypeDescription for ($( $t, )*)
        where
            $($t: GetTypeDescription),*
        {
            fn get_type_description() -> TypeDescription {
                TypeDescription {
                    encoding: None,
                    kind: String::from("base"),
                    name: vec![String::from("tuple") + &(count!($( $t )*)).to_string()],
                    description: Some([
                        $(($t_idx.to_string(), $t::get_type_description()),)*
                    ].into()),
                }
            }
        }
    };
}

macro_rules! count {
    () => (0usize);
    ( $x:ident $($xs:ident)* ) => (1usize + count!($($xs)*));
}

for_all_tuples!(get_type_description_for_tuple);