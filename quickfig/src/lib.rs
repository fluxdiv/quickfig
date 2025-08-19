//! # Quickfig
//!
//! ## Overview
//!
//! **Quickfig** defines a simple API for reading config files in applications.
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
//!     // and you may want to handle that situation explicitely.
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
//!
//! ## Cookbook
//! 
//! A few more usage examples to show full feature set
//! - Show open_first_match
//! - Show snippet with all the different `.get_u8()` methods
//!

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
