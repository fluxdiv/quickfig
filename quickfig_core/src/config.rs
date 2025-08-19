use std::path::PathBuf;
use config_types::DeserializedConfig;
use serde::de::DeserializeOwned;
use anyhow::{Result, anyhow};
use crate::field::{
    FieldMarker,
    // Field,
    Field2
};

/// Wrapper around deserialized config file
pub struct Config<S>(S)
    where
        S: DeserializeOwned + DeserializedConfig;

impl<S: DeserializeOwned + DeserializedConfig> Config<S> {

    pub fn create_field<'a>(&'a self, key: &str) -> Option<Field2<'a, S>> {
        let inner = &self.0;
        if let Some(field_value) = inner.get_at_str(key) {
            // create Field2 and return
            let f2 = Field2::new(key, field_value);
            return Some(f2);
        }
        None
    }

    pub fn has_key(&self, key: &str) -> bool {
        let inner = &self.0;
        inner.has_key(key)
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
    use crate::{
        FieldMarker,
        Field2
        // Field
    };

    pub type JSON = serde_json::Value;
    pub type TOML = toml::Value;

    // these are all helpers inside macro, user never calls directly
    pub trait DeserializedConfig {
        fn get_at_str(&self, key: &str) -> Option<&Self>;
        fn get_at_idx(&self, idx: usize) -> Option<&Self>;
        fn as_str(&self) -> Option<&str>;
        fn has_key(&self, key: &str) -> bool;
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
        fn get_string(&self) -> Option<String> {
            self.as_str().map(String::from)
            // println!("get_string in JSON | key: {}", key);
            // // it is `String("is string")`
            // println!("full object JSON get_string: {:#?}", self);
            // if self.is_null() {
            //     println!("self is null");
            // };
            // if self.is_object() {
            //     println!("self is object");
            // };
            // if self.is_string() {
            //     println!("self is string");
            //     // RIght here, this is printing the correct value "is string", 
            //     // without trying
            //     // to read the config at the key...
            //     let p = self.as_str();
            //     if p.is_none() {
            //         println!("this is a bug in serde_json");
            //     } else {
            //         let punwrap = p.unwrap();
            //         println!("punwrap: {}", punwrap);
            //     }
            // };
            // let v = self.get(key)?;
            // // HERE
            // // why is this returning None
            // // let v = self.get("String")?;
            // println!("get_string in JSON | key: {} | self.get(key)?: {:#?}", key, v);
            // v.as_str().map(String::from)
        }
        fn get_char(&self) -> Option<char> {
            self.as_str()
                .and_then(|s| s.chars().next())
        }

        fn get_u8(&self) -> Option<u8> {
            self.as_u64()
                .and_then(|n| u8::try_from(n).ok())
        }

        fn get_u16(&self) -> Option<u16> {
            self.as_u64()
                .and_then(|n| u16::try_from(n).ok())
        }

        fn get_u32(&self) -> Option<u32> {
            self.as_u64()
                .and_then(|n| u32::try_from(n).ok())
        }

        fn get_u64(&self) -> Option<u64> {
            self.as_u64()
        }

        fn get_u128(&self) -> Option<u128> {
            self.as_number()
                .and_then(|n| n.as_u128())
        }

        fn get_i8(&self) -> Option<i8> {
            self.as_i64()
                .and_then(|n| i8::try_from(n).ok())
        }

        fn get_i16(&self) -> Option<i16> {
            self.as_i64()
                .and_then(|n| i16::try_from(n).ok())
        }

        fn get_i32(&self) -> Option<i32> {
            self.as_i64()
                .and_then(|n| i32::try_from(n).ok())
        }

        fn get_i64(&self) -> Option<i64> {
            self.as_i64()
        }

        fn get_i128(&self) -> Option<i128> {
            self.as_number()
                .and_then(|n| n.as_i128())
        }

        fn get_bool(&self) -> Option<bool> {
            self.as_bool()
        }

        fn get_f32(&self) -> Option<f32> {
            self.as_f64()
                .and_then(|n| Some(n as f32))
        }

        fn get_f64(&self) -> Option<f64> {
            self.as_f64()
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

        fn get_string(&self) -> Option<String> {
            self.as_str().map(String::from)
        }

        fn get_char(&self) -> Option<char> {
            self.as_str()
                .and_then(|s| s.chars().next())
        }

        fn get_u8(&self) -> Option<u8> {
            self.as_integer()
                .and_then(|n| u8::try_from(n).ok())
        }

        fn get_u16(&self) -> Option<u16> {
            self.as_integer()
                .and_then(|n| u16::try_from(n).ok())
        }

        fn get_u32(&self) -> Option<u32> {
            self.as_integer()
                .and_then(|n| u32::try_from(n).ok())
        }

        fn get_u64(&self) -> Option<u64> {
            self.as_integer()
                .and_then(|n| u64::try_from(n).ok())
        }

        fn get_u128(&self) -> Option<u128> {
            self.as_integer()
                .and_then(|n| u64::try_from(n).ok())
                .map(|u| u.into())
        }

        fn get_i8(&self) -> Option<i8> {
            self.as_integer()
                .and_then(|n| i8::try_from(n).ok())
        }

        fn get_i16(&self) -> Option<i16> {
            self.as_integer()
                .and_then(|n| i16::try_from(n).ok())
        }

        fn get_i32(&self) -> Option<i32> {
            self.as_integer()
                .and_then(|n| i32::try_from(n).ok())
        }

        fn get_i64(&self) -> Option<i64> {
            self.as_integer()
        }

        fn get_i128(&self) -> Option<i128> {
            self.as_integer()
                .and_then(|n| u64::try_from(n).ok())
                .map(|u| u.into())
        }

        fn get_bool(&self) -> Option<bool> {
            self.as_bool()
        }

        fn get_f32(&self) -> Option<f32> {
            self.as_float()
                .and_then(|f| Some(f as f32))
        }

        fn get_f64(&self) -> Option<f64> {
            self.as_float()
        }
    }
}


