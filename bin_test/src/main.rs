#![allow(dead_code, unused)]
use std::marker::PhantomData;
use std::path::PathBuf;
use anyhow::Result;
use quickfig_core::{
    config_types::{ JSON, TOML },
    AllowedType,
    AllowedTypeWrapper,
    Config,
    ConfigFields,
    GetInner
};
use quickfig_derive::ConfigFields as ConfigFieldsMacro;

#[derive(ConfigFieldsMacro)]
enum MyConfigFields {
    // config.get(Name) will return Option<Result<String>>,
    // None      =>  if config doesn't have any of the keys in `keys()`
    // Some(Err) =>  if there but not parseable as String
    // Some(Ok(String)) =>  if there & parseable as String
    //
    // // if keys() not provided, default is simply the case sensitive variant name
    // #[keys("Name", "name", "nickname")]
    #[must_be(String)]
    Name,
    // .get(Age) will return Option<(Result<String>, Result<u32>)>
    // None        => keys not present
    // Some(x, y)  => x&y are Ok(x) if parseable into type, Err if not
    //
    // #[keys("Age", "age")]
    // #[any_of(String, u32)]
    #[any_of(String, u32, Vec<String>)]
    Age
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



fn config_usage_ex() -> Result<()> {
    // Opening a hardcoded path
    let cfg = Config::<JSON>::open("x.json");

    // Opening the first path that matches a closure
    let cfg2 = Config::<TOML>::open_first_match(
        vec!["x.toml", "y.toml", "z.toml"],
        Some(Box::new(|path_buf| {
            // can read the file, parse file names, do whatever in here to determine a match
            Some(path_buf)
        }))
    );

    Ok(())
}

