#![allow(dead_code, unused)]
use std::{char, marker::PhantomData};
use std::path::PathBuf;
use std::convert::From;
use config_types::{DeserializedConfig, JSON};
use serde::de::DeserializeOwned;
use anyhow::{Result, anyhow};
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;
use syn::{GenericArgument, Type, TypePath, PathArguments};

mod config;
mod allowed_type;
pub use config::*;
pub use allowed_type::*;

// quickfig_core

// THIS defines the trait API that can be derived
// I define HOW it's derived in the proc macro
pub trait ConfigFields {
    fn hello_macro();
    // This trait will be derived on the enum itself, so what methods will it need
    // or actually this could just be the marker trait for bounds
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
