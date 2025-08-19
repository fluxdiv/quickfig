use anyhow::Result;
use quickfig::derive::ConfigFields;
mod config_tests;
mod usage_tests;
mod utils;

// # Quickfig
//
// ## Section Header
//
// ### Subsection Header
//
// **Bold text**  
// *Italic text*  
// ~~Strikethrough~~  
//
// [Rust homepage](https://www.google.com)
//
// ### Tables
//
// | Column 1 | Column 2 |
// |----------|----------|
// | Value 1  | Value 2  |
// | Value 3  | Value 4  |
//
// ### Horizontal Rule
//
// ---
//
fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

