use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDescription {
    pub functions: HashMap<String, (u32, FunctionDescription)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BaseTypeDescription {
    Void,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
    String,
    ByteArray,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TypeInfo {
    BaseType(BaseTypeDescription),
    Enum(HashMap<String, TypeDescription>),
    Tuple(Vec<TypeDescription>),
    Option(Box<TypeDescription>),
    Result(Box<TypeDescription>),
    Array(Box<TypeDescription>),
    Object(HashMap<String, TypeDescription>),
    Ref,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeDescription {
    pub module_path: String,
    pub name: String,
    pub typeinfo: TypeInfo,
}

impl TypeDescription {
    pub fn void() -> Self {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(BaseTypeDescription::Void),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDescription {
    pub args_types: Vec<(bool, String, TypeDescription)>,
    pub return_type: TypeDescription,
}

pub trait GetTypeDescription {
    fn get_type_description() -> TypeDescription;
}