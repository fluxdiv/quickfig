// #![cfg_attr(test, doc = false)]
//! Quickfig
//! 
//! Re-exports core library and optionally derive macros.
//!
//! TODO: Docs

// quickfig/quickfig/src/lib.rs

/// * Core library
/// * Only import `quickfig::core::ConfigFields` if you are manually implementing,
///   if you are deriving use `quickfig::derive::ConfigFields`
pub mod core {
    pub use quickfig_core::*;
}

// #[cfg(feature = "derive")]
// extern crate quickfig_derive;

/// # Requirements
/// Requires `derive` feature
/// ```toml,ignore
/// [dependencies]
/// quickfig = { version = "0.1.0", features = ["derive"] }
/// ```
/// # Usage
/// If deriving `config_fields`:
/// ```rust,ignore
/// use quickfig::derive::ConfigFields;
///
/// #[derive(ConfigFields)]
/// enum Foo {}
/// ```
#[cfg(feature = "derive")]
pub mod derive {
    pub use quickfig_derive::*;
}
