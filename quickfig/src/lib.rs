// #![cfg_attr(test, doc = false)]
//! # Quickfig
//!
//! ## Modules
//!
//! * `core` - Core exports for reading configuration files.
//! * `derive` - Proc macro
//!
//! ## Quickstart
//!
//! ```
//! $ cargo add quickfig --features derive
//! ```
//!
//! Example config located at `/path/to/users_config.json`:
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
//!     // Key does not exist in the example `users_config.json`
//!     #[keys("Metadata", "metadata")]
//!     #[must_be(String)]
//!     Metadata
//!
//! }
//!
//! fn read_config() {
//!     // create a "Config" instance
//!     let config = Config::<JSON>::open("/path/to/users_config.json").unwrap();
//!     
//!     let id: Option<Vec<Field>> = config.get(MyFields::Id);
//!     // id would be None if all 3 of the keys were missing ("id", "ID", "Ident")
//!     // We know that "id" existed though, so unwrap the inner Vec<Field>
//!     let id_matches: Vec<Field> = id.unwrap();
//!     // id_matches contains all matched keys where the associated 
//!     // value was able to be parsed into one of the `any_of` types.
//!     // In this case, it would have a maximum length of 6.
//!
//!     // While the full list of results is available if you need it, you most likely 
//!     // expect the user's config to contain only 1 matching key per enum variant,
//!     // and want to throw an error otherwise.
//!
//!     // As a helper, `Vec<Field>` has a `.only_one_key()` method,
//!     // and `Field` has a `.get_key()` method.
//!     // Internally, `.only_one_key()` verifies that all `Field`s have
//!     // the same key (using `.get_key()`), and returns an Error otherwise.
//!     let id_matches: Vec<Field> = id_matches.only_one_key().unwrap();
//!
//!     // id_matches now contains 1 `Field` for every type in the 
//!     // `#[any_of(u8, u16)]` annotation that was successfully parsed.
//!     // Again, you can manually iterate if you want, or you can call
//!     // additional helper methods implemented for `Vec<Field>`
//!
//!     let u8_val: Option<u8> = id_matches.get_u8();
//!     let u16_val: Option<u16> = id_matches.get_u16();
//!
//!     // *NOTE*: The `get_x()` methods on Vec<Field> return the *first* match,
//!     // by internally iterating and calling `.get_x()` on each `Field`.
//!     // If you validated with `only_one_key` prior to calling these methods,
//!     // it is guaranteed there will be only 1 possible return value.
//!     // If you did not, then a re-ordered Vec<Field> with identical elements
//!     // may have a different return value.
//!
//! }
//! ```
//!
//! ## Features
//!
//! * `derive` - Enables the procedural macro for automatic `ConfigFields` derivation.
//!
//! ## Notes
//!

// quickfig/quickfig/src/lib.rs


/// * Core library
/// * Only import `quickfig::core::ConfigFields` if you are manually implementing,
///   if you are deriving use `quickfig::derive::ConfigFields`
pub mod core {
    pub use quickfig_core::*;
}

// #[cfg(feature = "derive")]
// extern crate quickfig_derive;

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
