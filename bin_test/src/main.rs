use anyhow::Result;
use quickfig::derive::ConfigFields;

mod config_tests;
mod usage_tests;
mod utils;

fn main() -> Result<()> {
    println!("Hello, world!");

    // let cfg = Config::<JSON>::open("QUICKFIG_TEST.json")?;
    //
    // println!("---------------- RUNNING NAME: must_be(String) ----------------");
    // let v: Option<Vec<AllowedTypeWrapper>> = cfg.get(MyConfigFields::Name);
    //
    // if let Some(foo) = v {
    //     for at in foo {
    //         let maybe_string = at.get_string();
    //         if let Some(s) = maybe_string {
    //             println!("field value is: {s}");
    //         }
    //     }
    // }
    //
    // println!("---------------- RUNNING AGE: anyof(String, u32) ----------------");
    // let v: Option<Vec<AllowedTypeWrapper>> = cfg.get(MyConfigFields::Age);
    //
    // // The key `Age` did exist in the config
    // // and the Vector consists of successful parse outputs of the value 
    // // into String/u32
    // if let Some(config_value_parse_results) = v {
    //     // I can iterate through each value and try to get the type I want like so
    //     for x in config_value_parse_results {
    //         // If some, the value was successfully parsed into a String
    //         // If none, it couldn't be parsed into a String
    //         let i_want_string: Option<String> = x.get_string();
    //         let r = i_want_string.unwrap_or(String::from("i_want_string empty"));
    //         println!("i_want_string: {r}");
    //
    //         let i_want_u32: Option<u32> = x.get_u32();
    //         match i_want_u32 {
    //             Some(r) => println!("i_want_u32: {r}"),
    //             None => println!("i_want_u32 empty")
    //         };
    //
    //     }
    // }

    Ok(())
}

