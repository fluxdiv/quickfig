// use anyhow::{Result, anyhow};
use syn::{GenericArgument, Type, TypePath, PathArguments};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum AllowedTypeMarker {
    String,
    Char,
    U8,U16,U32,U64,U128,
    I8,I16,I32,I64,I128,
    Bool,
    F32,F64,
    Vec(Box<AllowedTypeMarker>),
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum AllowedType {
    String(String),
    Char(char),
    U8(u8), U16(u16), U32(u32), U64(u64), U128(u128),
    I8(i8), I16(i16), I32(i32), I64(i64), I128(i128),
    Bool(bool),
    F32(f32), F64(f64),
    Vec(Box<AllowedType>),
}

impl AllowedTypeMarker {
    pub fn from_type_path(type_path: &TypePath) -> Option<Self> {
        let segment = &type_path.path.segments.last().unwrap();
        let type_name = segment.ident.to_string();

        match type_name.as_str() {
            "String" => Some(AllowedTypeMarker::String),
            "char" => Some(AllowedTypeMarker::Char),
            "u8" => Some(AllowedTypeMarker::U8),
            "u16" => Some(AllowedTypeMarker::U16),
            "u32" => Some(AllowedTypeMarker::U32),
            "u64" => Some(AllowedTypeMarker::U64),
            "u128" => Some(AllowedTypeMarker::U128),
            "i8" => Some(AllowedTypeMarker::I8),
            "i16" => Some(AllowedTypeMarker::I16),
            "i32" => Some(AllowedTypeMarker::I32),
            "i64" => Some(AllowedTypeMarker::I64),
            "i128" => Some(AllowedTypeMarker::I128),
            "bool" => Some(AllowedTypeMarker::Bool),
            "f32" => Some(AllowedTypeMarker::F32),
            "f64" => Some(AllowedTypeMarker::F64),
            "Vec" => {
                // println!();
                // println!("segment for vec");
                // println!("{:#?}", segment);
                // println!();
                // println!("type_path for vec");
                // println!("{:#?}", type_path);
                // println!();
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(Type::Path(inner_path))) = args.args.first() {
                        let inner = AllowedTypeMarker::from_type_path(inner_path)?;
                        // println!("returning allowed type vec");
                        Some(AllowedTypeMarker::Vec(Box::new(inner)))
                    } else {
                        // println!("returning NONE instead of allowed type vec A");
                        None
                    }
                } else {
                    // println!("returning NONE instead of allowed type vec B");
                    None
                }
            },
            _ => None,
        }
    }

    // pub fn from_str(s: &str) -> Option<Self> {
    //     match s.trim() {
    //         "String" => Some(AllowedTypeMarker::String),
    //         "char" => Some(AllowedTypeMarker::Char),
    //         "u8" => Some(AllowedTypeMarker::U8),
    //         "u16" => Some(AllowedTypeMarker::U16),
    //         "u32" => Some(AllowedTypeMarker::U32),
    //         "u64" => Some(AllowedTypeMarker::U64),
    //         "u128" => Some(AllowedTypeMarker::U128),
    //         "i8" => Some(AllowedTypeMarker::I8),
    //         "i16" => Some(AllowedTypeMarker::I16),
    //         "i32" => Some(AllowedTypeMarker::I32),
    //         "i64" => Some(AllowedTypeMarker::I64),
    //         "i128" => Some(AllowedTypeMarker::I128),
    //         "bool" => Some(AllowedTypeMarker::Bool),
    //         "f32" => Some(AllowedTypeMarker::F32),
    //         "f64" => Some(AllowedTypeMarker::F64),
    //
    //         // "Vec" => {
    //         //
    //         // },
    //
    //         _ if s.starts_with("Vec<") && s.ends_with('>') => {
    //             let inner = &s[4..s.len() - 1];
    //             AllowedTypeMarker::from_str(inner).map(|inner_ty| AllowedType::Vec(Box::new(inner_ty)))
    //         }
    //         _ => None,
    //     }
    // }
}

pub trait GetInner {
    fn get_string(&self) -> Option<String>;
    fn get_char(&self) -> Option<char>;
    fn get_u8(&self) -> Option<u8>;
    fn get_u16(&self) -> Option<u16>;
    fn get_u32(&self) -> Option<u32>;
    fn get_u64(&self) -> Option<u64>;
    fn get_u128(&self) -> Option<u128>;
    fn get_i8(&self) -> Option<i8>;
    fn get_i16(&self) -> Option<i16>;
    fn get_i32(&self) -> Option<i32>;
    fn get_i64(&self) -> Option<i64>;
    fn get_i128(&self) -> Option<i128>;
    fn get_bool(&self) -> Option<bool>;
    fn get_f32(&self) -> Option<f32>;
    fn get_f64(&self) -> Option<f64>;
}

impl GetInner for AllowedType {
    fn get_string(&self) -> Option<String> {
        match self {
            Self::String(x) => Some(x.clone()),
            _ => None,
        }
    }

    fn get_char(&self) -> Option<char> {
        match self {
            Self::Char(c) => Some(*c),
            _ => None,
        }
    }

    fn get_u8(&self) -> Option<u8> {
        match self {
            Self::U8(x) => Some(*x),
            _ => None,
        }
    }

    fn get_u16(&self) -> Option<u16> {
        match self {
            Self::U16(x) => Some(*x),
            _ => None,
        }
    }

    fn get_u32(&self) -> Option<u32> {
        match self {
            Self::U32(x) => Some(*x),
            _ => None,
        }
    }

    fn get_u64(&self) -> Option<u64> {
        match self {
            Self::U64(x) => Some(*x),
            _ => None,
        }
    }

    fn get_u128(&self) -> Option<u128> {
        match self {
            Self::U128(x) => Some(*x),
            _ => None,
        }
    }

    fn get_i8(&self) -> Option<i8> {
        match self {
            Self::I8(x) => Some(*x),
            _ => None,
        }
    }

    fn get_i16(&self) -> Option<i16> {
        match self {
            Self::I16(x) => Some(*x),
            _ => None,
        }
    }

    fn get_i32(&self) -> Option<i32> {
        match self {
            Self::I32(x) => Some(*x),
            _ => None,
        }
    }

    fn get_i64(&self) -> Option<i64> {
        match self {
            Self::I64(x) => Some(*x),
            _ => None,
        }
    }

    fn get_i128(&self) -> Option<i128> {
        match self {
            Self::I128(x) => Some(*x),
            _ => None,
        }
    }

    fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(x) => Some(*x),
            _ => None,
        }
    }

    fn get_f32(&self) -> Option<f32> {
        match self {
            Self::F32(x) => Some(*x),
            _ => None,
        }
    }

    fn get_f64(&self) -> Option<f64> {
        match self {
            Self::F64(x) => Some(*x),
            _ => None,
        }
    }
}


