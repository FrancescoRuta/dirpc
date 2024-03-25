use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDescription {
    pub types: HashMap<Vec<String>, TypeDescription>,
    pub functions: HashMap<Vec<String>, FunctionDescription>,
    pub consts: HashMap<Vec<String>, ConstDescription>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeDescription {
    pub path: Vec<String>,
    pub typeinfo: TypeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDescription {
    pub args_types: Vec<TypeDescription>,
    pub return_type: TypeDescription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstDescription {
    
}

pub trait GetTypeDescription {
    fn get_type_description() -> TypeDescription;
}