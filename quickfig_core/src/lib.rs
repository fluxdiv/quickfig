// #![allow(dead_code, unused)]
mod config;
mod allowed_type;
pub use config::*;
pub use allowed_type::*;

// quickfig_core/lib.rs

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
