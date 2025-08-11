// testing actual usage of the macro
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
use super::utils::*;
use super::utils::TestFileType as TFT;

// MODS
// must_be_json    : testing must_be on json configs
// must_be_toml    : testing must_be on toml configs
// any_of_json     : testing any_of on json configs
// any_of_toml     : testing any_of on toml configs


#[allow(non_camel_case_types)]
#[derive(ConfigFieldsMacro)]
pub enum TestEnum {
    #[keys("String")]
    #[must_be(String)]
    String,
    #[must_be(char)]
    Char,
    #[must_be(u8)]
    U8,

    #[must_be(u16)]
    U16,
    #[must_be(u32)]
    U32,
    #[must_be(u64)]
    U64,
    #[must_be(u128)]
    U128,
    #[must_be(i8)]
    I8,
    #[must_be(i16)]
    I16,
    #[must_be(i32)]
    I32,
    #[must_be(i64)]
    I64,
    #[must_be(i128)]
    I128,
    // #[must_be(bool)]
    // Bool,
    // #[must_be(f32)]
    // F32,
    // #[must_be(f64)]
    // F64,
    // --------------- any_of
    // high variety from different groups
    // (String and char), (unsigned ints), (signed ints), (bool), (floats)
    // #[any_of(String, char, bool)]
    // String_Char_Bool,
    // #[any_of(u8, u16, u32)]
    // U8_U16_U32,
    // #[any_of(i8, i16, i32)]
    // I8_I16_I32,
    // #[any_of(u64, u128, f64)]
    // U64_U128_F64,
    // #[any_of(i64, f32, bool)]
    // I64_F32_Bool,
    // #[any_of(u8, f32, String)]
    // U8_F32_String,
    // #[any_of(i8, u32, f64)]
    // I8_U32_F64,
    // #[any_of(i16, bool, f64)]
    // I16_Bool_F64,
    // #[any_of(char, f32, u16)]
    // Char_F32_U16,
    // #[any_of(i128, bool, u64)]
    // I128_Bool_U64,
}


#[cfg(test)]
mod must_be_json {
    use super::*;

    // ---------------------------------------------------------------
    // ------------ String
    #[test]
    fn must_be_string_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("String", "is string")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        // testconfig.pretty_print().unwrap();
        testconfig.delete();

        assert!(config.has_key("String"));
        let vals = config.get(TestEnum::String);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner_s = vals.iter().find(|v| {
            v.get_string()
                .map(|str_val| str_val == String::from("is string"))
                .is_some()
        });
        assert!(inner_s.is_some());
    }

    #[test]
    fn must_be_string_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("String", 1)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("String"));
        let vals = config.get(TestEnum::String);
        assert!(vals.is_none());
    }

    // ---------------------------------------------------------------
    // ------------ char
    #[test]
    fn must_be_char_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("Char", 'c')).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        // testconfig.pretty_print().unwrap();
        testconfig.delete();

        assert!(config.has_key("Char"));
        let vals = config.get(TestEnum::Char);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_char()
                .map(|char_val| char_val == 'c')
                .is_some()
        });
        assert!(inner.is_some());
    }

    // TODO: this passes but might need more extensive tsts
    #[test]
    fn must_be_char_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("Char", 1)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("Char"));
        let vals = config.get(TestEnum::Char);
        assert!(vals.is_none());
    }

    // ---------------------------------------------------------------
    // ------------ u8, u16, u32, u64, u128
    #[test]
    fn must_be_u8_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U8", 255)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U8"));
        let vals = config.get(TestEnum::U8);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_u8()
                .map(|u8_val| u8_val == 255)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_u8_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U8", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U8"));
        let vals = config.get(TestEnum::U8);
        assert!(vals.is_none());
    }

    #[test]
    fn must_be_u16_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U16", 255)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U16"));
        let vals = config.get(TestEnum::U16);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_u16()
                .map(|u16_val| u16_val == 255)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_u16_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U16", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U16"));
        let vals = config.get(TestEnum::U16);
        assert!(vals.is_none());
    }

    #[test]
    fn must_be_u32_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U32", 255)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U32"));
        let vals = config.get(TestEnum::U32);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_u32()
                .map(|u32_val| u32_val == 255)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_u32_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U32", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U32"));
        let vals = config.get(TestEnum::U32);
        assert!(vals.is_none());
    }

    #[test]
    fn must_be_u64_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U64", 255)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U64"));
        let vals = config.get(TestEnum::U64);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_u64()
                .map(|u64_val| u64_val == 255)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_u64_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U64", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U64"));
        let vals = config.get(TestEnum::U64);
        assert!(vals.is_none());
    }

    #[test]
    fn must_be_u128_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U128", 255)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U128"));
        let vals = config.get(TestEnum::U128);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_u128()
                .map(|u128_val| u128_val == 255)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_u128_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("U128", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("U128"));
        let vals = config.get(TestEnum::U128);
        assert!(vals.is_none());
    }

    // ---------------------------------------------------------------
    // ------------ i8, i16, i32, i64, i128
    #[test]
    fn must_be_i8_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I8", -128)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I8"));
        let vals = config.get(TestEnum::I8);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_i8()
                .map(|i8_val| i8_val == -128)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_i8_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I8", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I8"));
        let vals = config.get(TestEnum::I8);
        assert!(vals.is_none());
    }

    #[test]
    fn must_be_i16_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I16", i16::MIN)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I16"));
        let vals = config.get(TestEnum::I16);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_i16()
                .map(|i16_val| i16_val == i16::MIN)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_i16_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I16", (i16::MIN as i32) - 1)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I16"));
        let vals = config.get(TestEnum::I16);
        assert!(vals.is_none());
    }

    #[test]
    fn must_be_i32_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I32", i32::MIN)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I32"));
        let vals = config.get(TestEnum::I32);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_i32()
                .map(|i32_val| i32_val == i32::MIN)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_i32_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I32", (i32::MIN as i64) - 1)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I32"));
        let vals = config.get(TestEnum::I32);
        assert!(vals.is_none());
    }

    #[test]
    fn must_be_i64_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I64", i64::MIN)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I64"));
        let vals = config.get(TestEnum::I64);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_i64()
                .map(|i64_val| i64_val == i64::MIN)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_i64_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I64", (i64::MIN as i128) - 1)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I64"));
        let vals = config.get(TestEnum::I64);
        assert!(vals.is_none());
    }

    // ERR
    #[test]
    fn must_be_i128_ok() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I128", i128::MIN)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I128"));
        let vals = config.get(TestEnum::I128);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        assert!(vals.len() == 1);
        let inner = vals.iter().find(|v| {
            v.get_i128()
                .map(|i128_val| i128_val == i128::MIN)
                .is_some()
        });
        assert!(inner.is_some());
    }

    #[test]
    fn must_be_i128_err() {
        let mut testconfig = TestFile::new(TFT::JSON).unwrap();
        testconfig.add_entry(("I128", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete();

        assert!(config.has_key("I128"));
        let vals = config.get(TestEnum::I128);
        assert!(vals.is_none());
    }
}

