// #![cfg_attr(test, doc = false)]
//! # Quickfig
//!
//! This crate is *mostly* a wrapper around [Serde](https://serde.rs/) and crates that
//! implement Serde's de/serialization model 
//! ([serde_json](https://github.com/serde-rs/json) and [toml](https://docs.rs/toml)).
//!
//! Quickfig utilizes these crates to define a straight forward API 
//! for reading values from configuration files in applications. 
//! The benefit of using Quickfig is that it can replace a lot
//! of boilerplate that you likely have/will have to write many 
//! times as an application developer.
//!
//!
//! ## Modules
//!
//! * `core` - Core exports for reading configuration files.
//! * `derive` - Proc macro
//!
//! ## Quickstart
//!
//! ```ignore
//! $ cargo add quickfig --features derive
//! ```
//!
//! Imagine that a user has their configuration file for your application at `/path/to/users_config.json` with the content:
//!
//! ```json
//! {
//!   "id": 9,
//!   "title": "foo"
//! }
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
//! // Define the fields you may want to access
//! #[derive(ConfigFields)]
//! enum MyFields {
//!     #[keys("id", "ID", "Ident")]
//!     #[any_of(u8, u16)] // must be parseable into u8 or u16
//!     Id,
//!
//!     // Missing `keys` attribute defaults to (case-sensitive) variant name "Title"
//!     #[must_be(String)] // must be parseable into String
//!     Title,
//!
//!     // Notice that none of these keys exist in the example `users_config.json`
//!     #[keys("Metadata", "metadata")]
//!     #[must_be(String)]
//!     Metadata
//!
//! }
//!
//! fn read_config_id() -> Result<()> {
//!     // create a "Config" instance
//!     let config = Config::<JSON>::open("/path/to/users_config.json").unwrap();
//!     
//!     // id would be None if all 3 of the keys were missing ("id", "ID", "Ident")
//!     let id: Option<Vec<Field>> = config.get(MyFields::Id);
//!     // We know that "id" existed though, so unwrap the inner Vec<Field>
//!     let id: Vec<Field> = id.unwrap();
//!
//!     // Think of this Vec<Field> as containing all entries in the Config where both:
//!     // 1) The key matched one of the values in `#[keys("id", "ID", "Ident")]`
//!     // 2) The value could be parsed into 1 or more types in `#[any_of(u8, u16)]`
//!     // In this case, the length of the Vec is 2.
//!     
//!     // NOTE: If the Config had multiple matching keys, for ex: {"id": 1,"ID": 2 },
//!     // Then the length of the Vec would be 4, and you would need to disambiguate
//!     // between the two.
//!
//!     // In most cases, you probably want the user's Config to contain only 1
//!     // matching key (per enum variant), and you want to throw an Error with
//!     // a helpful message otherwise.
//!
//!     // Verify that only 1 key was matched
//!     let id: Vec<Field> = id.only_one_key().map_err(|_| println!("Your err msg"))?;
//!
//!     // Lastly, to get u8/u16 value that was in the user's Config file:
//!     let id_u8: Option<u8> = id.get_u8();
//!     let id_u16: Option<u16> = id.get_u16();
//!     
//!     // If the value couldn't be parsed into a u8 or u16, then both of those
//!     // methods would have returned None. In this case, the config file contained
//!     // {"id": 9}, so they return `Some(9u8)` and `Some(9u16)` respectively.
//! }
//! ```
//!
//! ## Features
//!
//! * `derive` - Enables the procedural macro for automatic `ConfigFields` derivation.
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
