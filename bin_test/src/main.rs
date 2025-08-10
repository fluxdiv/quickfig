#![allow(dead_code, unused)]
use anyhow::Result;
use quickfig_core::{
    config_types::{ JSON, TOML },
    // AllowedType,
    AllowedTypeWrapper,
    Config,
    ConfigFields,
    GetInner,
};
use quickfig_derive::ConfigFields as ConfigFieldsMacro;
mod config_tests;
mod usage_tests;
mod utils;

#[derive(ConfigFieldsMacro)]
enum MyConfigFields {
    // // if keys() not provided, default is simply the case sensitive variant name
    // // if keys has overlap with another keys definition, it will be returned
    // //    for both

    // no keys + must_be
    #[must_be(u32)]
    A,
    // keys overlaps with no keys default
    #[keys("A")]
    #[must_be(u32)]
    A2,

    // no keys + any_of
    #[any_of(String, u32)]
    B,
    // keys overlaps with no keys default
    #[keys("B")]
    #[any_of(String, u32)]
    B2,

    // keys + must_be
    #[keys("sirname", "nickname")]
    #[must_be(String)]
    Name,
    // overlapping keys, same must_be type
    #[keys("sirname", "foo")]
    #[must_be(String)]
    Name2,
    // overlapping keys, different must_be type
    #[keys("sirname", "foo")]
    #[must_be(u32)]
    Name3,

    // keys + any_of
    #[keys("young", "old")]
    #[any_of(String, u32)]
    Age,
    // overlapping keys, same any_of type
    #[keys("young", "old")]
    #[any_of(String, u32)]
    Age2,
    // overlapping keys, different any_of type
    #[keys("young", "old")]
    #[any_of(u8, bool)]
    Age3,
}


fn main() -> Result<()> {
    println!("Hello, world!");

    let cfg = Config::<JSON>::open("QUICKFIG_TEST.json")?;

    println!("---------------- RUNNING NAME: must_be(String) ----------------");
    let v: Option<Vec<AllowedTypeWrapper>> = cfg.get(MyConfigFields::Name);

    if let Some(foo) = v {
        for at in foo {
            let maybe_string = at.get_string();
            if let Some(s) = maybe_string {
                println!("field value is: {s}");
            }
        }
    }

    println!("---------------- RUNNING AGE: anyof(String, u32) ----------------");
    let v: Option<Vec<AllowedTypeWrapper>> = cfg.get(MyConfigFields::Age);

    // The key `Age` did exist in the config
    // and the Vector consists of successful parse outputs of the value 
    // into String/u32
    if let Some(config_value_parse_results) = v {
        // I can iterate through each value and try to get the type I want like so
        for x in config_value_parse_results {
            // If some, the value was successfully parsed into a String
            // If none, it couldn't be parsed into a String
            let i_want_string: Option<String> = x.get_string();
            let r = i_want_string.unwrap_or(String::from("i_want_string empty"));
            println!("i_want_string: {r}");

            let i_want_u32: Option<u32> = x.get_u32();
            match i_want_u32 {
                Some(r) => println!("i_want_u32: {r}"),
                None => println!("i_want_u32 empty")
            };

        }
    }

    Ok(())
}

