// #![allow(dead_code, unused)]
mod config;
mod field;
pub use config::*;
pub use field::*;

// quickfig/quickfig_core/lib.rs

// Marker trait for bounding
pub trait ConfigFields {}

// THIS defines the trait API that can be derived
// I define HOW it's derived in the proc macro
// This trait will be derived on the enum itself, so what methods will it need
// or actually this could just be the marker trait for bounds
