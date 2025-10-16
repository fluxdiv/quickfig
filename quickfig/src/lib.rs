//! # Quickfig
//!
//! ## Overview
//!
//! **Quickfig** defines a simple API for reading config files in applications.
//!
//! **Quickfig**'s goal is to replace a big chunk of boilerplate that you
//! most likely have/will have to write many times as an application developer.
//!
//! This crate is *mostly* a wrapper around [Serde](https://serde.rs/) and crates that
//! implement Serde's de/serialization model, currently including:
//! - [serde_json](https://github.com/serde-rs/json)
//! - [toml](https://docs.rs/toml)
//!
//! ## Modules
//!
//! * `quickfig::core`   - Core exports for reading configuration files
//! * `quickfig::derive` - Derive macro for config fields
//!
//! ## Features
//!
//! * `derive` - Enables the derive macro for ConfigFields
//! ---
//! ## Quickstart
//!
//! ```ignore
//! $ cargo add quickfig --features derive
//! ```
//!
//! Imagine you want to read a user's config file at `/path/to/config.json` with:
//! ```json
//! { "id": 9 , "title": "foo" }
//! ```
//!
//! In your project:
//!
//! ```rust,ignore
//! use quickfig::derive::ConfigFields;
//! use quickfig::core::{
//!     Config, Field,
//!     config_types::JSON
//! };
//!
//! // Define the fields you may want to read
//! #[derive(ConfigFields)]
//! enum MyFields {
//!     #[keys("id", "ID")]
//!     Id,
//!     // A missing `keys` attribute defaults to (case-sensitive) variant name "Title"
//!     Title,
//! }
//!
//! fn main() -> Result<()> {
//!     // create a "Config" instance, errors if file doesnt exist/no permissions/etc
//!     let config = Config::<JSON>::open("/path/to/config.json").unwrap();
//!     
//!     // Getting the id
//!     let Some(id): Option<Vec<Field<'_, JSON>>> = config.get(MyFields::Id) else {
//!         // Config didn't have "id" or "ID" key
//!         return Err(String::from("Config must have an id or ID key"));
//!     }
//!
//!     // Notice that id is a Vec<Field>. That is because the config could
//!     // contain multiple matching keys, for example {"id": 1, "ID": 2}
//!     // and you may want to handle that situation explicitly.
//!     // 
//!     // However, most of the time you probably only want to accept 1
//!     // matching key, and otherwise you want to error.
//!     if id.only_one_key().is_err() {
//!        return Err(String::from("Config must have an id or ID key, but not both"));
//!     };
//!
//!     // Lastly, getting out the value:
//!     // Reminder that the file contained {"id": 9, "title": "foo"}
//!
//!     let id_u8: Option<u8> = id.get_u8();
//!     if id_u8.is_none() {
//!         return Err(String::from("Config id must be a valid u8 integer"));
//!     };
//!     assert!(id.get_u8().is_some_and(|id| id == 9u8));
//!
//!     let id_string: Option<String> = id.get_string();
//!     assert!(id_string.is_none());
//! }
//! ```
//! ---
//!
//! ## Cookbook
//! 
//! A few more usage examples to show features/recommended usage:
//!
//! * `Config::open` requires a **FULL** path. 
//!   A crate like [dirs](https://crates.io/crates/dirs) can be helpful to create these
//! ```rust,ignore
//! use dirs::*;
//! use std::path::PathBuf;
//!
//! // Might be something like this on linux:
//! // "/home/username/.config/my_app/config.json"
//! let path_to_config: PathBuf = {
//!     let mut home_dir = dirs::config_dir().unwrap();
//!     home_dir.push("my_app/config.json");
//!     home_dir
//! };
//! ```
//!
//! ---
//!
//! * List of get methods available on Vec<Field>:
//! * **NOTE**: Any numbers outside of `i64` range will
//!   error on TOML files as TOML spec does not support them
//! ```rust,ignore
//!     let config = Config::<JSON>::open("/path/to/config.json").unwrap();
//!     let field = config.get(MyFields::SomeField).unwrap();
//!
//!     // If you need the underlying Value for custom deserialization
//!     let f: Option<&serde_json::Value> = field.get_generic_inner();
//!
//!     let f: Option<String>  = field.get_string();
//!     let f: Option<char>    = field.get_char();
//!     let f: Option<bool>    = field.get_bool();
//!     let f: Option<u8>      = field.get_u8();
//!     let f: Option<u16>     = field.get_u16();
//!     let f: Option<u32>     = field.get_u32();
//!     let f: Option<u64>     = field.get_u64();
//!     let f: Option<u128>    = field.get_u128();
//!     let f: Option<i8>      = field.get_i8();
//!     let f: Option<i16>     = field.get_i16();
//!     let f: Option<i32>     = field.get_i32();
//!     let f: Option<i64>     = field.get_i64();
//!     let f: Option<i128>    = field.get_i128();
//!     let f: Option<f32>     = field.get_f32();
//!     let f: Option<f64>     = field.get_f64();
//! ```
//!
//! ---
//!
//! * Sometimes a config's field isn't a basic type like String or u8.
//!
//!   In these cases, instead of using `field.get_u8()` etc., you can use
//!   `field.get_generic_inner()` to access the field value directly.
//!
//!   If the key requested is present, Quickfig will get you a reference
//!   to its field (as `&Value`) which you can then deserialize as needed.
//!
//!   Ex: You expect a config to have "colors" & "fonts" keys, and you
//!       open a `config.json` with this content:
//! ```json
//! {
//!     "colors": {
//!         "primary": "blue",
//!         "accents": ["purple", "cyan"],
//!         "filter": {
//!             "brightness": 7, 
//!             "inverted": false
//!         }
//!     },
//!     "fonts": [
//!         { "size": 1, "name": "roboto" },
//!         { "size": 2, "name": "verdana" }
//!     ]
//! }
//! ```
//!
//! In your application:
//! ```rust,ignore
//!     // Fields you expect to be in the config
//!     #[derive(ConfigFields)]
//!     enum AppConfig {
//!         #[keys("colors")]
//!         Colors,
//!         #[keys("fonts")]
//!         Fonts
//!     }
//!     
//!     // Types for your expected config structure
//!     #[derive(serde::Deserialize)]
//!     struct Colors {
//!         primary: String,
//!         accents: Vec<String>,
//!         filter: Filter
//!     }
//!     #[derive(serde::Deserialize)]
//!     struct Filter {
//!         brightness: u8,
//!         inverted: bool
//!     }
//!     #[derive(serde::Deserialize)]
//!     struct Fonts(Vec<Font>);
//!     #[derive(serde::Deserialize)]
//!     struct Font {
//!         size: u8,
//!         name: String
//!     }
//!
//!     // opening the config.json file 
//!     let config = Config::<JSON>::open("/path/to/config.json").unwrap();
//!     // Access "colors" key & verify only 1 match
//!     let colors_field = config.get(AppConfig::Colors).unwrap();
//!     colors_field.only_one_key().unwrap();
//!
//!     // Get the underlying value without trying to parse it
//!     let colors_inner: &serde_json::Value = colors_field
//!         .get_generic_inner()
//!         .unwrap();
//!
//!     // Deserialize it yourself
//!     let colors: Colors = Colors::deserialize(colors_inner).unwrap();
//! ```
//! 
//! ---
//!
//! * Sometimes you want to allow multiple possible paths for a user's config.
//!   
//!   For example, your docs might say:
//!   ```txt
//!   MyApp will first check for your config at "~/.config/MyApp/config.json",
//!   then "~/.MyApp/config.json", then "~/.local/share/MyApp/config.json"...
//!   ```
//!
//!   For that situation there is a helper method when creating a Config:
//!
//! ```rust,ignore
//!   // List of paths you want to check (order does matter!)
//!   let paths = vec![
//!       "~/.config/MyApp/config.json",
//!       "~/.MyApp/config.json",
//!       "~/.local/share/MyApp/config.json"
//!   ];
//!
//!   // Search function that determines whether a path should be used or not.
//!   // Return Some(path) to use a path or None to continue iterating.
//!   // Will short-circuit first Some(path) return.
//!   let search = Box::new(move |path: std::path::PathBuf| -> Option<PathBuf> {
//!       if path.exists() {
//!           Some(path)
//!       } else {
//!           None
//!       }
//!   });
//!
//!   // Will try to create a Config from the first path that your function returns
//!   // Some(path) on. Errors if there is no match or problem creating Config.
//!   // If no search function is provided then default is same as search above.
//!   let config: Result<Config<JSON>> = Config::<JSON>::open_first_match(
//!       paths,
//!       Some(search)
//!   );
//! ```

/// * Core library
/// * Only import `quickfig::core::ConfigFields` if you are manually implementing,
///   if you are deriving use `quickfig::derive::ConfigFields`
pub mod core {
    pub use quickfig_core::*;
}

/// Derive macro for `ConfigFields`
/// # Requirements
/// Requires `derive` feature
/// ```ignore
/// [dependencies]
/// quickfig = { version = "0.1.0", features = ["derive"] }
/// ```
/// # Usage
/// ```rust,ignore
/// use quickfig::derive::ConfigFields;
///
/// #[derive(ConfigFields)]
/// enum Foo {}
/// ```
#[cfg(feature = "derive")]
pub mod derive {
    pub use quickfig_derive::ConfigFields;
}
