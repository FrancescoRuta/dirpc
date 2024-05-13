use std::collections::HashMap;

use crate::{FunctionDescription, ServerDescription, TypeDescription, TypeInfo};

fn to_camelcase(str: &str) -> String {
    let mut result = String::with_capacity(str.len());
    for s in str.split('_') {
        if let Some(first) = s.chars().next() {
            result.push(first.to_ascii_uppercase());
            result.push_str(&s[1..]);
        }
    }
    result
}

enum Node<T> {
    Module(HashMap<String, Node<T>>),
    Element(T),
    Undefined,
}

impl<T> Node<T> {
    
    fn to_string(value: &HashMap<String, Node<T>>, module_begin: impl Fn(&str, &mut String), module_end: impl Fn(&str, &mut String), element: impl Fn(&str, &T, &mut String)) -> String {
        fn to_string_int<T>(value: &HashMap<String, Node<T>>, result: &mut String, module_begin: &impl Fn(&str, &mut String), module_end: &impl Fn(&str, &mut String), element: &impl Fn(&str, &T, &mut String)) {
            for (name, c) in value {
                match c {
                    Node::Module(module) => {
                        module_begin(name, result);
                        to_string_int(module, result, module_begin, module_end, element);
                        module_end(name, result);
                    },
                    Node::Element(el) => element(name, el, result),
                    Node::Undefined => unreachable!(),
                }
            }
        }
        let mut result = String::new();
        to_string_int(value, &mut result, &module_begin, &module_end, &element);
        result
    }
    
}

fn to_nested<T>(map: HashMap<Vec<String>, T>) -> anyhow::Result<HashMap<String, Node<T>>> {
    let mut root = Node::Module(HashMap::new());
    for (path, v) in map {
        let mut current_ref = &mut root;
        for part in path {
            match current_ref {
                Node::Module(module) => current_ref = module.entry(part).or_insert(Node::Undefined),
                Node::Element(_) => anyhow::bail!("Duplicate entry"),
                Node::Undefined => {
                    *current_ref = Node::Module(HashMap::new());
                    if let Node::Module(module) = current_ref {
                        current_ref = module.entry(part).or_insert(Node::Undefined);
                    } else {
                        unreachable!();
                    }
                },
            }
        }
        if let Node::Undefined = current_ref {
            *current_ref = Node::Element(v);
        } else {
            anyhow::bail!("Duplicate entry")
        }
    }
    if let Node::Module(root) = root {
        Ok(root)
    } else {
        unreachable!()
    }
}

fn get_all_types<'a>(mut stack: Vec<&'a TypeDescription>, types: &mut HashMap<Vec<String>, &'a TypeDescription>) {
    while let Some(t) = stack.pop() {
        if t.name != "" && t.module_path != "" {
            types.insert(t.module_path.split("::").map(|s| s.to_string()).chain([t.name.clone()]).collect(), t);
        }
        match &t.typeinfo {
            TypeInfo::BaseType(_) => (),
            TypeInfo::Enum(_) => (),
            TypeInfo::Tuple(t) => { stack.extend(t.iter()); }
            TypeInfo::Option(t) => { stack.push(&t); }
            TypeInfo::Result(t) => { stack.push(&t); }
            TypeInfo::Array(t) => { stack.push(&t); }
            TypeInfo::Object(t) => { stack.extend(t.iter().map(|(_, t)| t)); }
            TypeInfo::Ref => (),
        }
    }
}

fn serialize_type(ty: &TypeDescription, result: &mut String, force_name_ref: bool, typename_prefix: &str) {
    if force_name_ref && ty.module_path != "" && ty.name != "" {
        result.push_str(typename_prefix);
        result.push('.');
        for s in ty.module_path.split("::") {
            result.push_str(s);
            result.push('.');
        }
        result.push_str(&ty.name);
        return;
    }
    match &ty.typeinfo {
        TypeInfo::BaseType(t) => match t {
            crate::BaseTypeDescription::Void => result.push_str("void"),
            crate::BaseTypeDescription::U8 => result.push_str("number"),
            crate::BaseTypeDescription::I8 => result.push_str("number"),
            crate::BaseTypeDescription::U16 => result.push_str("number"),
            crate::BaseTypeDescription::I16 => result.push_str("number"),
            crate::BaseTypeDescription::U32 => result.push_str("number"),
            crate::BaseTypeDescription::I32 => result.push_str("number"),
            crate::BaseTypeDescription::U64 => result.push_str("number"),
            crate::BaseTypeDescription::I64 => result.push_str("number"),
            crate::BaseTypeDescription::F32 => result.push_str("number"),
            crate::BaseTypeDescription::F64 => result.push_str("number"),
            crate::BaseTypeDescription::Bool => result.push_str("boolean"),
            crate::BaseTypeDescription::String => result.push_str("string"),
            crate::BaseTypeDescription::ByteArray => result.push_str("UInt8Array"),
        }
        TypeInfo::Tuple(t) => {
            result.push('[');
            for a in t {
                serialize_type(a, result, true, typename_prefix);
                result.push(',');
            }
            result.push(']');
        }
        TypeInfo::Option(t) => {
            serialize_type(t, result, true, typename_prefix);
            result.push_str("|null|undefined");
        }
        TypeInfo::Result(t) => {
            result.push_str("Result<");
            serialize_type(t, result, true, typename_prefix);
            result.push('>');
        }
        TypeInfo::Array(t) => {
            serialize_type(t, result, true, typename_prefix);
            result.push('[');
            result.push(']');
        }
        TypeInfo::Enum(_) | TypeInfo::Object(_) => {
            if ty.module_path == "" && ty.name == "" { unreachable!() }
            result.push_str(typename_prefix);
            result.push('.');
            for s in ty.module_path.split("::") {
                result.push_str(s);
                result.push('.');
            }
            result.push_str(&ty.name);
        },
        TypeInfo::Ref => unreachable!(),
    }
}

fn serialize_type_declaration(name: &str, ty: &TypeDescription, result: &mut String, typename_prefix: &str) {
    if name != "" {
        if let TypeInfo::Object(ty) = &ty.typeinfo {
            result.push_str("export type ");
            result.push_str(name);
            result.push('=');
            result.push('{');
            for (name, a) in ty {
                result.push_str(name);
                result.push(':');
                serialize_type(a, result, true, typename_prefix);
                result.push(';');
            }
            result.push('}');
            result.push(';');
        } else if let TypeInfo::Enum(variants) = &ty.typeinfo {
            result.push_str("export enum ");
            result.push_str(name);
            result.push('{');
            for (name, discriminant) in variants {
                result.push_str(name);
                result.push('=');
                result.push_str(&discriminant.to_string());
                result.push(',');
            }
            result.push('}');
        } else {
            result.push_str("export type ");
            result.push_str(name);
            result.push('=');
            serialize_type(ty, result, false, typename_prefix);
            result.push(';');
        }
    }
}

fn types_to_camel_case(ty: &mut TypeDescription) {
    let mut module_path = String::with_capacity(ty.module_path.len());
    for p in ty.module_path.split("::") {
        if module_path.len() != 0 { module_path.push_str("::"); }
        for s in p.split('_') {
            if let Some(first) = s.chars().next() {
                module_path.push(first.to_ascii_uppercase());
                module_path.push_str(&s[1..]);
            }
        }
    }
    ty.module_path = module_path;
    ty.name = to_camelcase(&ty.name);
    match &mut ty.typeinfo {
        TypeInfo::BaseType(_) => (),
        TypeInfo::Enum(_) => (),
        TypeInfo::Tuple(ty) => ty.iter_mut().for_each(|ty| types_to_camel_case(ty)),
        TypeInfo::Option(ty) => types_to_camel_case(&mut *ty),
        TypeInfo::Result(ty) => types_to_camel_case(&mut *ty),
        TypeInfo::Array(ty) => types_to_camel_case(&mut *ty),
        TypeInfo::Object(ty) => ty.iter_mut().for_each(|(_, ty)| types_to_camel_case(ty)),
        TypeInfo::Ref => (),
    }
}

enum RuntimeItem<'a> {
    Function(&'a String, &'a FunctionDescription),
    Enum(TypeDescription, Vec<(String, u32)>),
}

pub fn get_code(main_namespace: &str, mut server_description: ServerDescription) -> anyhow::Result<String> {
    let typename_prefix = format!("{main_namespace}Types");
    for (_, (_ , f)) in &mut server_description.functions {
        f.args_types.iter_mut().for_each(|(_, ty)| types_to_camel_case(ty));
        types_to_camel_case(&mut f.return_type);
    }
    let mut runtime: HashMap<Vec<String>, RuntimeItem> = HashMap::new();
    let mut types: HashMap<Vec<String>, &TypeDescription> = HashMap::new();
    let mut types_stack = Vec::new();
    for (path, (_, function)) in &server_description.functions {
        types_stack.extend(function.args_types.iter().map(|(_, t)| t));
        types_stack.push(&function.return_type);
        runtime.insert(path.split("::").map(|s| to_camelcase(s)).collect(), RuntimeItem::Function(path, function));
    }
    get_all_types(types_stack, &mut types);
    runtime.extend(types.iter().filter_map(|(k, ty)| if let TypeInfo::Enum(variants) = &ty.typeinfo { Some((k.clone(), RuntimeItem::Enum((*ty).clone(), variants.clone()))) } else { None }));
    let runtime = to_nested(runtime)?;
    let types = to_nested(types)?;
    let types = Node::to_string(&types,
        |name, res| {
            res.push_str("export namespace ");
            res.push_str(name);
            res.push('{');
        },
        |_name, res| {
            res.push('}');
        },
        |name, el, res| {
            serialize_type_declaration(name, el, res, &typename_prefix);
        },
    );
    let runtime_interface = Node::to_string(&runtime,
        |name, res| {
            res.push_str(name);
            res.push(':');
            res.push('{');
        },
        |_name, res| {
            res.push('}');
            res.push(';');
        },
        |name, item, res| {
            match item {
                RuntimeItem::Function(_, el) => {
                    if let Some(first_char) = name.chars().next() {
                        res.push(first_char.to_ascii_lowercase());
                        res.push_str(&name[1..]);
                    }
                    res.push('(');
                    for (arg_name, arg_type) in &el.args_types {
                        res.push_str(&arg_name);
                        res.push(':');
                        serialize_type(arg_type, res, true, &typename_prefix);
                        res.push(',');
                    }
                    res.push_str("):FunctionCall<");
                    serialize_type(&el.return_type, res, true, &typename_prefix);
                    res.push('>');
                    res.push(';');
                },
                RuntimeItem::Enum(typedexcription, _) => {
                    res.push_str(&name);
                    res.push_str(":typeof ");
                    serialize_type(typedexcription, res, true, &typename_prefix);
                    res.push(';');
                },
            }
        },
    );
    let runtime_initialization = Node::to_string(&runtime,
        |name, res| {
            res.push_str(name);
            res.push(':');
            res.push('{');
        },
        |_name, res| {
            res.push('}');
            res.push(',');
        },
        |name, item, res| {
            match item {
                RuntimeItem::Function(path, el) => {
                    if let Some(first_char) = name.chars().next() {
                        res.push(first_char.to_ascii_lowercase());
                        res.push_str(&name[1..]);
                    }
                    res.push_str(":(function(_fn_index_:number){return function(");
                    for (arg_name, arg_type) in &el.args_types {
                        let arg_name = to_camelcase(arg_name);
                        if let Some(first_char) = arg_name.chars().next() {
                            res.push(first_char.to_ascii_lowercase());
                            res.push_str(&arg_name[1..]);
                        }
                        res.push(':');
                        serialize_type(arg_type, res, true, &typename_prefix);
                        res.push(',');
                    }
                    res.push_str("):FunctionCall<");
                    serialize_type(&el.return_type, res, true, &typename_prefix);
                    res.push_str(">{return({id:_fn_index_,args:[");
                    for (arg_name, _) in &el.args_types {
                        let arg_name = to_camelcase(arg_name);
                        if let Some(first_char) = arg_name.chars().next() {
                            res.push(first_char.to_ascii_lowercase());
                            res.push_str(&arg_name[1..]);
                        }
                        res.push(',');
                    }
                    res.push_str("]}as any)}})(d[\"");
                    res.push_str(&path);
                    res.push_str("\"][0]),");
                },
                RuntimeItem::Enum(_, variants) => {
                    res.push_str(&name);
                    res.push_str(":{");
                    for (name, value) in variants {
                        res.push_str(&format!("{name}:{value},"));
                    }
                    res.push_str("},");
                },
            }
        },
    );
    Ok(format!("interface FunctionCall<R>{{id:number;args:any[];__typeCheck__?:R;}}export declare namespace {typename_prefix}{{{types}}}export interface {main_namespace}{{{runtime_interface}}}export function init{main_namespace}(data:string):{main_namespace}{{let d=JSON.parse(data).functions;return{{{runtime_initialization}}};}}"))
}