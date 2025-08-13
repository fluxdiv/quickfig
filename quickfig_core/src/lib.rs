// #![allow(dead_code, unused)]
mod config;
mod allowed_type;
pub use config::*;
pub use allowed_type::*;

// quickfig/quickfig_core/lib.rs

pub trait ConfigFields {
    fn hello_macro();
}

// THIS defines the trait API that can be derived
// I define HOW it's derived in the proc macro
// This trait will be derived on the enum itself, so what methods will it need
// or actually this could just be the marker trait for bounds
