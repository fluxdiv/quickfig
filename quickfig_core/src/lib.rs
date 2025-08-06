#![allow(dead_code, unused)]
use std::{char, marker::PhantomData};
use std::path::PathBuf;
use std::convert::From;
use config_types::{DeserializedConfig, JSON};
use serde::de::DeserializeOwned;
use anyhow::{Result, anyhow};
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;
use syn::{GenericArgument, Type, TypePath, PathArguments};

// quickfig_core

// THIS defines the trait API that can be derived
// I define HOW it's derived in the proc macro
pub trait ConfigFields {
    fn hello_macro();
    // This trait will be derived on the enum itself, so what methods will it need
    // or actually this could just be the marker trait for bounds
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum AllowedType {
    String,
    Char,
    U8,U16,U32,U64,U128,
    I8,I16,I32,I64,I128,
    Bool,
    F32,F64,
    Vec(Box<AllowedType>),
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum AllowedTypeWrapper {
    String(String),
    Char(char),
    U8(u8), U16(u16), U32(u32), U64(u64), U128(u128),
    I8(i8), I16(i16), I32(i32), I64(i64), I128(i128),
    Bool(bool),
    F32(f32), F64(f64),
    Vec(Box<AllowedTypeWrapper>),
}

/// Re-export for brevity in proc-macro interpolation code
pub type AT = AllowedType;
pub type ATW = AllowedTypeWrapper;

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

impl GetInner for AllowedTypeWrapper {
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

// impl GetInner for Vec<AllowedType> {
//     fn get_string(&self) -> Option<String> {
//         for at in self.iter() {
//             // return first where at.get_string() returns Some
//         }
//         // If they all return None, return None
//         None
//     }
//
//     fn get_char(&self) -> Option<char> {
//         for at in self.iter() {
//             // return first where at.get_char returns Some
//         }
//         None
//     }
// }


impl AllowedType {
    pub fn from_type_path(type_path: &TypePath) -> Option<Self> {
        let segment = &type_path.path.segments.last().unwrap();
        let type_name = segment.ident.to_string();

        match type_name.as_str() {
            "String" => Some(AllowedType::String),
            "char" => Some(AllowedType::Char),
            "u8" => Some(AllowedType::U8),
            "u16" => Some(AllowedType::U16),
            "u32" => Some(AllowedType::U32),
            "u64" => Some(AllowedType::U64),
            "u128" => Some(AllowedType::U128),
            "i8" => Some(AllowedType::I8),
            "i16" => Some(AllowedType::I16),
            "i32" => Some(AllowedType::I32),
            "i64" => Some(AllowedType::I64),
            "i128" => Some(AllowedType::I128),
            "bool" => Some(AllowedType::Bool),
            "f32" => Some(AllowedType::F32),
            "f64" => Some(AllowedType::F64),
            "Vec" => {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(Type::Path(inner_path))) = args.args.first() {
                        let inner = AllowedType::from_type_path(inner_path)?;
                        Some(AllowedType::Vec(Box::new(inner)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    // pub fn from_str(s: &str) -> Option<Self> {
    //     match s.trim() {
    //         "String" => Some(AllowedType::String),
    //         "char" => Some(AllowedType::Char),
    //         "u8" => Some(AllowedType::U8),
    //         "u16" => Some(AllowedType::U16),
    //         "u32" => Some(AllowedType::U32),
    //         "u64" => Some(AllowedType::U64),
    //         "u128" => Some(AllowedType::U128),
    //         "i8" => Some(AllowedType::I8),
    //         "i16" => Some(AllowedType::I16),
    //         "i32" => Some(AllowedType::I32),
    //         "i64" => Some(AllowedType::I64),
    //         "i128" => Some(AllowedType::I128),
    //         "bool" => Some(AllowedType::Bool),
    //         "f32" => Some(AllowedType::F32),
    //         "f64" => Some(AllowedType::F64),
    //
    //         // "Vec" => {
    //         //
    //         // },
    //
    //         _ if s.starts_with("Vec<") && s.ends_with('>') => {
    //             let inner = &s[4..s.len() - 1];
    //             AllowedType::from_str(inner).map(|inner_ty| AllowedType::Vec(Box::new(inner_ty)))
    //         }
    //         _ => None,
    //     }
    // }
}


// Re-exports
pub mod config_types {
    use crate::{AllowedType, AllowedTypeWrapper};

    pub type JSON = serde_json::Value;
    pub type TOML = toml::Value;

    pub trait DeserializedConfig {
        fn get_at_str(&self, key: &str) -> Option<&Self>;
        fn get_at_idx(&self, idx: usize) -> Option<&Self>;
        fn as_str(&self) -> Option<&str>;
        fn has_key(&self, key: &str) -> bool;
        fn parse_allowed_type(&self, key: &str, at: AllowedType) -> Option<AllowedTypeWrapper>;
    }

    impl DeserializedConfig for JSON {
        fn get_at_str(&self, key: &str) -> Option<&Self> {
            self.get(key)
        }
        fn get_at_idx(&self, idx: usize) -> Option<&Self> {
            self.get(idx)
        }
        fn as_str(&self) -> Option<&str> {
            self.as_str()
        }
        fn has_key(&self, key: &str) -> bool {
            self.get(key).is_some()
        }
        fn parse_allowed_type(
            &self,
            key: &str,
            at: AllowedType
        ) -> Option<AllowedTypeWrapper> {
            // So I'm getting the key (enum variant) & type from user when they use
            // COnfig.get in their code
            let v = self.get(key)?;

            match at {
                AllowedType::String => {
                    v.as_str()
                        .map(|s| AllowedTypeWrapper::String(s.to_string()))
                },
                AllowedType::Char => {
                    v.as_str()
                        .and_then(|s| s.chars().next())
                        .map(|c| AllowedTypeWrapper::Char(c))
                },
                AllowedType::U8 => {
                    v.as_u64()
                        .and_then(|n| u8::try_from(n).ok())
                        .map(|u| AllowedTypeWrapper::U8(u))
                }
                AllowedType::U16 => {
                    v.as_u64()
                        .and_then(|n| u16::try_from(n).ok())
                        .map(|u| AllowedTypeWrapper::U16(u))
                },
                AllowedType::U32 => {
                    v.as_u64()
                        .and_then(|n| u32::try_from(n).ok())
                        .map(|u| AllowedTypeWrapper::U32(u))
                },
                AllowedType::U64 => {
                    v.as_u64()
                        .map(|u| AllowedTypeWrapper::U64(u))
                },
                AllowedType::U128 => {
                    v.as_number()
                        .and_then(|num| {
                            num.as_u128()
                        })
                        .map(|n| {
                            AllowedTypeWrapper::U128(n)
                        })
                },
                AllowedType::I8 => {
                    v.as_i64()
                        .and_then(|n| i8::try_from(n).ok())
                        .map(|u| AllowedTypeWrapper::I8(u))
                }
                AllowedType::I16 => {
                    v.as_i64()
                        .and_then(|n| i16::try_from(n).ok())
                        .map(|u| AllowedTypeWrapper::I16(u))
                },
                AllowedType::I32 => {
                    v.as_i64()
                        .and_then(|n| i32::try_from(n).ok())
                        .map(|u| AllowedTypeWrapper::I32(u))
                },
                AllowedType::I64 => {
                    v.as_i64()
                        .map(|u| AllowedTypeWrapper::I64(u))
                },
                AllowedType::I128 => {
                    v.as_number()
                        .and_then(|num| {
                            num.as_i128()
                        })
                        .map(|n| {
                            AllowedTypeWrapper::I128(n)
                        })
                },
                AllowedType::F32 => {
                    v.as_f64()
                        .map(|f| AllowedTypeWrapper::F32(f as f32))
                },
                AllowedType::F64 => {
                    v.as_f64()
                        .map(|f| AllowedTypeWrapper::F64(f))
                },
                AllowedType::Bool => {
                    v.as_bool().map(|b| AllowedTypeWrapper::Bool(b))
                },
                AllowedType::Vec(ref inner_at) => {
                    let arr = v.as_array()?;
                    let mut parsed = vec![];
                    for val in arr {
                        let inner_wrapper = self.parse_allowed_type(key, at.clone())?;
                        parsed.push(inner_wrapper);
                    }
                    // wrap recursively
                    parsed.into_iter()
                        .rev()
                        .reduce(|acc, x| {
                            AllowedTypeWrapper::Vec(Box::new(x))
                        })
                        .map(|y| Box::new(y))
                        .map(|z| AllowedTypeWrapper::Vec(z))
                }
                _ => unreachable!()
            }
            // None
        }
    }

    impl DeserializedConfig for TOML {
        fn get_at_str(&self, key: &str) -> Option<&Self> {
            self.get(key)
        }
        fn get_at_idx(&self, idx: usize) -> Option<&Self> {
            self.get(idx)
        }
        fn as_str(&self) -> Option<&str> {
            self.as_str()
        }
        fn has_key(&self, key: &str) -> bool {
            self.get(key).is_some()
        }
        fn parse_allowed_type(&self, key: &str, at: AllowedType) -> Option<AllowedTypeWrapper> {
            // TODO
            None
        }
    }
}

/// Wrapper around deserialized config file
pub struct Config<S>(S)
    where
        S: DeserializeOwned + DeserializedConfig;


impl<S: DeserializeOwned + DeserializedConfig> Config<S> {

    pub fn has_key(&self, key: &str) -> bool {
        let inner = &self.0;
        inner.has_key(key)
    }

    pub fn parse_allowed_type(
        &self,
        key: &str,
        at: AllowedType
    ) -> Option<AllowedTypeWrapper> {
        let inner = &self.0;
        inner.parse_allowed_type(key, at)
    }

    // pub fn get_field(&self, variant_stringified: String) -> String {
    //     // String::from("hello : {variant_stringified}")
    //     // variant_stringified
    //     let inner = &self.0;
    //     let f = inner.get_at_str(&variant_stringified);
    //
    //     match f {
    //         Some(v) => {
    //             let s = v.as_str();
    //             if let Some(ss) = s {
    //                 return String::from(ss);
    //             } else {
    //                 return String::from("was not string");
    //             }
    //         }
    //         None => String::from("Field does not exist")
    //     }
    // }

    fn new_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {

        let ext = path.as_ref().extension().ok_or_else(|| {
            anyhow!(format!(
                "File path \"{}\" does not have extension (.json, .toml, etc)",
                path.as_ref().to_str().unwrap_or_else(|| "Invalid unicode path")
            ))
        })?;

        let ext = ext.to_str().ok_or_else(|| anyhow!("Extension invalid unicode"))?;
        let file_str = std::fs::read_to_string(&path)?;
        match ext {
            "json" => {
                let json = serde_json::from_str::<S>(&file_str)?;
                Ok(Config(json))
            },
            "toml" => {
                let toml = toml::from_str::<S>(&file_str)?;
                Ok(Config(toml))
            },
            bad_ext => {
                anyhow::bail!("File extension \".{}\" not supported", bad_ext);
            }
        }
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Config<S>> {
        Config::<S>::new_from_file(path)
    }

    // Return Config<T> of the first path where `search` returns true
    // Default to first that exists
    pub fn open_first_match(
        paths: Vec<impl Into<PathBuf>>,
        // Iterate through `paths` and returns a Config of the first
        // such that search(path) returns Some(path)
        search: Option<Box<dyn Fn(PathBuf) -> Option<PathBuf>>>
    ) -> Result<Config<S>> {

        let search = search.unwrap_or_else(|| {
            Box::new(|path: PathBuf| -> Option<PathBuf> {
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            })
        });

        let maybe_path: Option<PathBuf> = paths
            .into_iter()
            .find_map(|path| {
                // returns first non-none
                search(path.into())
            });

        // If maybe_path is None, return Err("No path matched search function")
        // Else if Some(path)
        //   return the result of new_from_file(path)
        maybe_path.map(|path| {
            Config::<S>::new_from_file(path)
        }).ok_or_else(|| anyhow!("No path matched search function"))?
    }

    // fn default() -> Config<S> {
    //     // Config((S::default(), PhantomData))
    //     Config(S::default())
    // }
    //
    // fn new(file_data: S) -> Self {
    //     // Config((file_data, PhantomData))
    //     Config(file_data)
    // }
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
