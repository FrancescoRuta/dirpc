use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDescription {
    pub types: HashMap<Vec<String>, TypeDescription>,
    pub functions: HashMap<Vec<String>, FunctionDescription>,
    pub consts: HashMap<Vec<String>, ConstDescription>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeDescription {
    pub encoding: Option<String>,
    pub kind: String,
    pub name: Vec<String>,
    pub description: Option<HashMap<String, TypeDescription>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDescription {
    pub args_types: Vec<TypeDescription>,
    pub return_type: TypeDescription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstDescription {
    
}