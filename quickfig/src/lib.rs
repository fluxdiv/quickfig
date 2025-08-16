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
//! ```rust
//! use quickfig::derive::ConfigFields;
//! use quickfig::core::{
//!     Config, AllowedType,
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
//!     let id: Option<Vec<AllowedType>> = config.get(MyFields::Id);
//!     // id would be None if all 3 of the keys were missing ("id", "ID", "Ident")
//!     // We know that "id" existed though, so unwrap the inner Vec<AllowedType>
//!     let id_matches: Vec<AllowedType> = id.unwrap();
//!     // id_matches contains all matched keys where the associated 
//!     // value was able to be parsed into one of the `any_of` types.
//!     // In this case, it would have a maximum length of 6.
//!
//!     // While the full list of results is available if you need it, you most likely 
//!     // expect the user's config to contain only 1 matching key per enum variant,
//!     // and want to throw an error otherwise.
//!
//!     // As a helper, `Vec<AllowedType>` has a `.only_one(&self)` method,
//!     // and `AllowedType` has a `.get_key()` method.
//!     // Internally, `.only_one()` verifies that all AllowedType have
//!     // the same key (using `.get_key()`), and returns an Error otherwise.
//!
//!     let id_matches: Result<Vec<AllowedType>> = id_matches.only_one();
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
