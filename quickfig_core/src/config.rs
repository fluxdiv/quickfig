use std::path::PathBuf;
use config_types::DeserializedConfig;
use serde::de::DeserializeOwned;
use anyhow::{Result, anyhow};
use crate::allowed_type::{AllowedTypeMarker, AllowedType};

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
        at: AllowedTypeMarker
    ) -> Option<AllowedType> {
        let inner = &self.0;
        inner.parse_allowed_type(key, at)
    }

    fn new_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Config<S>> {

        let ext = path.as_ref().extension().ok_or_else(|| {
            anyhow!(format!(
                "File path \"{}\" does not have extension (.json, .toml, etc)",
                path.as_ref().to_str().unwrap_or_else(|| "Invalid unicode path")
            ))
        })?;

        let ext = ext.to_str().ok_or_else(|| anyhow!("Extension invalid unicode"))?;
        let file_str = std::fs::read_to_string(&path)?;
        if file_str.is_empty() {
            anyhow::bail!("File was empty: {:#?}", path.as_ref());
        }
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

    /// Opens and returns `Config<S>`
    /// # Arguments
    /// `path` - **Full** path to file, `dirs` crate can help getting this
    /// # Returns
    /// `Result<Config<S>>`
    /// # Errors
    /// * If file at `path` is empty or non-existent
    /// * If file at `path` cannot be accessed (permissions, etc)
    /// * If file at `path` cannot be deserialized
    /// * If `path` itself is not valid UTF-8
    /// * If `path` itself does not have extension of `.json` or `.toml`
    /// # Usage
    /// ```rust,ignore
    /// # use std::error::Error;
    /// use quickfig::core::Config;
    /// use quickfig::core::config_types::JSON;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let full_path = "/home/user/.config/MyApp/config.json";
    /// let cfg = Config::<JSON>::open(full_path)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Config<S>> {
        Config::<S>::new_from_file(path)
    }

    /// Opens and returns `Config<S>` of the first path in `paths` where
    /// `search` returns `Some(path)`.
    ///
    /// If `search` not provided, defaults to first path in `paths` that exists
    /// and can be accessed
    /// # Arguments
    /// `paths` - List of **Full** file paths to run `search` on
    /// `search` - Optional fn to determine if each path should be used or not
    /// # Returns
    /// `Result<Config<S>>` - Errors if no path matches the search function OR if problem
    /// creating `Config` with matched path (file is empty, not accessible, 
    /// cannot be parsed as `<S>`, etc)
    /// # Usage
    /// ```rust,ignore
    /// // Equivalent to default, returns first path that exists & is accessible
    /// let my_paths = vec!["/path/to/x.toml", "/path/to/y.toml", "/path/to/z.toml"];
    /// let cfg = Config::<TOML>::open_first_match(
    ///     my_paths,
    ///     Some(Box::new(|path: std::path::PathBuf| {
    ///         // this logic will be called on each path in my_paths
    ///         // until you return Some(path) or they all return None
    ///         if path.exists() { Some(path) } else { None }
    ///     }))
    /// );
    /// ```
    pub fn open_first_match(
        paths: Vec<impl Into<PathBuf>>,
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
}


// Re-exports
pub mod config_types {
    use crate::{AllowedTypeMarker, AllowedType};

    pub type JSON = serde_json::Value;
    pub type TOML = toml::Value;

    // these are all helpers inside macro, user never calls directly
    pub trait DeserializedConfig {
        fn get_at_str(&self, key: &str) -> Option<&Self>;
        fn get_at_idx(&self, idx: usize) -> Option<&Self>;
        fn as_str(&self) -> Option<&str>;
        fn has_key(&self, key: &str) -> bool;
        fn parse_allowed_type(&self, key: &str, at: AllowedTypeMarker) -> Option<AllowedType>;
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
            at: AllowedTypeMarker
        ) -> Option<AllowedType> {
            // this is called with a key (defined on user's enum variant via `keys()` or variant name itself)
            // and allowed type (defined on user's enum variant via `must_be()` or `any_of()`
            let v = self.get(key)?;

            match at {
                AllowedTypeMarker::String => {
                    v.as_str()
                        .map(|s| AllowedType::String(s.to_string()))
                },
                AllowedTypeMarker::Char => {
                    v.as_str()
                        .and_then(|s| s.chars().next())
                        .map(|c| AllowedType::Char(c))
                },
                AllowedTypeMarker::U8 => {
                    v.as_u64()
                        .and_then(|n| u8::try_from(n).ok())
                        .map(|u| AllowedType::U8(u))
                }
                AllowedTypeMarker::U16 => {
                    v.as_u64()
                        .and_then(|n| u16::try_from(n).ok())
                        .map(|u| AllowedType::U16(u))
                },
                AllowedTypeMarker::U32 => {
                    v.as_u64()
                        .and_then(|n| u32::try_from(n).ok())
                        .map(|u| AllowedType::U32(u))
                },
                AllowedTypeMarker::U64 => {
                    v.as_u64()
                        .map(|u| AllowedType::U64(u))
                },
                AllowedTypeMarker::U128 => {
                    v.as_number()
                        .and_then(|num| {
                            num.as_u128()
                        })
                        .map(|n| {
                            AllowedType::U128(n)
                        })
                },
                AllowedTypeMarker::I8 => {
                    v.as_i64()
                        .and_then(|n| i8::try_from(n).ok())
                        .map(|u| AllowedType::I8(u))
                }
                AllowedTypeMarker::I16 => {
                    v.as_i64()
                        .and_then(|n| i16::try_from(n).ok())
                        .map(|u| AllowedType::I16(u))
                },
                AllowedTypeMarker::I32 => {
                    v.as_i64()
                        .and_then(|n| i32::try_from(n).ok())
                        .map(|u| AllowedType::I32(u))
                },
                AllowedTypeMarker::I64 => {
                    v.as_i64()
                        .map(|u| AllowedType::I64(u))
                },
                AllowedTypeMarker::I128 => {
                    v.as_number()
                        .and_then(|num| {
                            num.as_i128()
                        })
                        .map(|n| {
                            AllowedType::I128(n)
                        })
                },
                AllowedTypeMarker::F32 => {
                    v.as_f64()
                        .map(|f| AllowedType::F32(f as f32))
                },
                AllowedTypeMarker::F64 => {
                    v.as_f64()
                        .map(|f| AllowedType::F64(f))
                },
                AllowedTypeMarker::Bool => {
                    v.as_bool().map(|b| AllowedType::Bool(b))
                },
                // AllowedTypeMarker::Vec(ref inner_at) => {
                //     let arr = v.as_array()?;
                //     let mut parsed = vec![];
                //     for val in arr {
                //         let inner_wrapper = self.parse_allowed_type(key, at.clone())?;
                //         parsed.push(inner_wrapper);
                //     }
                //     // wrap recursively
                //     parsed.into_iter()
                //         .rev()
                //         .reduce(|acc, x| {
                //             AllowedType::Vec(Box::new(x))
                //         })
                //         .map(|y| Box::new(y))
                //         .map(|z| AllowedType::Vec(z))
                // }
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
        fn parse_allowed_type(
            &self,
            key: &str,
            at: AllowedTypeMarker
        ) -> Option<AllowedType> {
            let v = self.get(key)?;

            match at {
                AllowedTypeMarker::String => {
                    v.as_str()
                        .map(|s| AllowedType::String(s.to_string()))
                },
                AllowedTypeMarker::Char => {
                    v.as_str()
                        .and_then(|s| s.chars().next())
                        .map(|c| AllowedType::Char(c))
                },
                AllowedTypeMarker::U8 => {

                    v.as_integer()
                        .and_then(|n| u8::try_from(n).ok())
                        .map(|u| AllowedType::U8(u))
                }
                AllowedTypeMarker::U16 => {
                    v.as_integer()
                        .and_then(|n| u16::try_from(n).ok())
                        .map(|u| AllowedType::U16(u))
                },
                AllowedTypeMarker::U32 => {
                    v.as_integer()
                        .and_then(|n| u32::try_from(n).ok())
                        .map(|u| AllowedType::U32(u))
                },
                AllowedTypeMarker::U64 => {
                    v.as_integer()
                        .and_then(|n| u64::try_from(n).ok())
                        .map(|u| AllowedType::U64(u))
                },
                AllowedTypeMarker::U128 => {
                    v.as_integer()
                        .and_then(|n| u64::try_from(n).ok())
                        .map(|u| AllowedType::U128(u.into()))
                },
                AllowedTypeMarker::I8 => {
                    v.as_integer()
                        .and_then(|n| i8::try_from(n).ok())
                        .map(|u| AllowedType::I8(u))
                }
                AllowedTypeMarker::I16 => {
                    v.as_integer()
                        .and_then(|n| i16::try_from(n).ok())
                        .map(|u| AllowedType::I16(u))
                },
                AllowedTypeMarker::I32 => {
                    v.as_integer()
                        .and_then(|n| i32::try_from(n).ok())
                        .map(|u| AllowedType::I32(u))
                },
                AllowedTypeMarker::I64 => {
                    v.as_integer()
                        .map(|u| AllowedType::I64(u))
                },
                AllowedTypeMarker::I128 => {
                    v.as_integer()
                        .and_then(|n| u64::try_from(n).ok())
                        .map(|u| AllowedType::I128(u.into()))
                },
                AllowedTypeMarker::F32 => {
                    v.as_float()
                        .map(|f| AllowedType::F32(f as f32))
                },
                AllowedTypeMarker::F64 => {
                    v.as_float()
                        .map(|f| AllowedType::F64(f))
                },
                AllowedTypeMarker::Bool => {
                    v.as_bool().map(|b| AllowedType::Bool(b))
                },
                // AllowedTypeMarker::Vec(ref inner_at) => {
                //     let arr = v.as_array()?;
                //     let mut parsed = vec![];
                //     for val in arr {
                //         let inner_wrapper = self.parse_allowed_type(key, at.clone())?;
                //         parsed.push(inner_wrapper);
                //     }
                //     // wrap recursively
                //     parsed.into_iter()
                //         .rev()
                //         .reduce(|acc, x| {
                //             AllowedType::Vec(Box::new(x))
                //         })
                //         .map(|y| Box::new(y))
                //         .map(|z| AllowedType::Vec(z))
                // }
                _ => unreachable!()
            }
            // None
        }
    }
}


