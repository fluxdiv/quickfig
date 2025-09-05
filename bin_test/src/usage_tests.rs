// testing actual usage of the macro
#![allow(dead_code, unused)]
use anyhow::Result;
use quickfig::core::{
    config_types::{ JSON, TOML },
    // Field,
    Field,
    VecField,
    Config,
    GetInner,
};
use quickfig::derive::ConfigFields;
use super::utils::*;
use super::utils::TestFileType as TFT;

// MODS
// 
// json_main         : testing JSON configs
// toml_main         : testing TOML configs
//
// misc_tests_json   : overlapping keys,
// misc_tests_toml   : overlapping keys,

#[allow(non_camel_case_types)]
#[derive(ConfigFields)]
pub enum TestEnum {
    // Testing all types
    String, String_Empty,
    Char,
    Bool_False, Bool_True,
    U8_MAX, U16_MAX, U32_MAX, U64_MAX, U128_MAX,
    I8_MAX, I16_MAX, I32_MAX, I64_MAX, I128_MAX,
    U8_MIN, U16_MIN, U32_MIN, U64_MIN, U128_MIN,
    I8_MIN, I16_MIN, I32_MIN, I64_MIN, I128_MIN,

    F32_MIN_POSITIVE, F64_MIN_POSITIVE,
}

#[cfg(test)]
mod json_main {
    use super::*;
    const TEST_FILE_TYPE: TestFileType = TFT::JSON;

    // ---------------------------------------------------------------
    // ------------ Test all types
    #[test]
    fn test_string() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<JSON>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        let vals = config.get(TestEnum::String);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        vals.only_one_key().unwrap();

        let s = vals.get_string();
        assert!(s.is_some());
        let s = s.unwrap();
        assert!(s == String::from("i am string"));

        let vals = config.get(TestEnum::String_Empty);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        vals.only_one_key().unwrap();

        let s = vals.get_string();
        assert!(s.is_some());
        let s = s.unwrap();
        assert!(s == String::from(""));
    }

    #[test]
    fn test_char() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<JSON>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        let vals = config.get(TestEnum::Char).unwrap();
        vals.only_one_key().unwrap();
        let c = vals.get_char().unwrap();
        assert!(c == 'c');
    }

    // ------------------
    // bool
    #[test]
    fn test_bool() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<JSON>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        // Bool_False
        let vals = config.get(TestEnum::Bool_False).unwrap();
        vals.only_one_key().unwrap();
        let b = vals.get_bool().unwrap();
        assert!(!b);

        // Bool_True
        let vals = config.get(TestEnum::Bool_True).unwrap();
        vals.only_one_key().unwrap();
        let b = vals.get_bool().unwrap();
        assert!(b);
    }

    // ------------------
    // unsigned ints
    #[test]
    fn test_unsigned_integers() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<JSON>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        // U8
        let vals = config.get(TestEnum::U8_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u8().unwrap() == u8::MAX);
        let vals = config.get(TestEnum::U8_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u8().unwrap() == u8::MIN);

        // U16
        let vals = config.get(TestEnum::U16_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u16().unwrap() == u16::MAX);
        let vals = config.get(TestEnum::U16_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u16().unwrap() == u16::MIN);

        // U32
        let vals = config.get(TestEnum::U32_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u32().unwrap() == u32::MAX);
        let vals = config.get(TestEnum::U32_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u32().unwrap() == u32::MIN);

        // U64
        let vals = config.get(TestEnum::U64_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u64().unwrap() == u64::MAX);
        let vals = config.get(TestEnum::U64_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u64().unwrap() == u64::MIN);

        // U128
        let vals = config.get(TestEnum::U128_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u128().unwrap() == u128::MAX);
        let vals = config.get(TestEnum::U128_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u128().unwrap() == u128::MIN);
    }

    // --------------------
    // signed ints
    #[test]
    fn test_signed_integers() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<JSON>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        // I8
        let vals = config.get(TestEnum::I8_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i8().unwrap() == i8::MAX);
        let vals = config.get(TestEnum::I8_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i8().unwrap() == i8::MIN);

        // I16
        let vals = config.get(TestEnum::I16_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i16().unwrap() == i16::MAX);
        let vals = config.get(TestEnum::I16_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i16().unwrap() == i16::MIN);

        // I32
        let vals = config.get(TestEnum::I32_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i32().unwrap() == i32::MAX);
        let vals = config.get(TestEnum::I32_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i32().unwrap() == i32::MIN);

        // I64
        let vals = config.get(TestEnum::I64_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i64().unwrap() == i64::MAX);
        let vals = config.get(TestEnum::I64_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i64().unwrap() == i64::MIN);

        // I128
        let vals = config.get(TestEnum::I128_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i128().unwrap() == i128::MAX);
        let vals = config.get(TestEnum::I128_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i128().unwrap() == i128::MIN);
    }

    // --------------------
    // Floats 
    #[test]
    fn test_floats() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<JSON>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        let vals = config.get(TestEnum::F32_MIN_POSITIVE).unwrap();
        vals.only_one_key().unwrap();
        let v = vals.get_f32().unwrap();
        assert!(v == f32::MIN_POSITIVE);

        let vals = config.get(TestEnum::F64_MIN_POSITIVE).unwrap();
        vals.only_one_key().unwrap();
        let v = vals.get_f64().unwrap();
        assert!(v == f64::MIN_POSITIVE);
    }
}

#[cfg(test)]
mod toml_main {
    use super::*;
    const TEST_FILE_TYPE: TestFileType = TFT::TOML;

    // ---------------------------------------------------------------
    // ------------ Test all types
    #[test]
    fn test_string() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<TOML>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        let vals = config.get(TestEnum::String);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        vals.only_one_key().unwrap();

        let s = vals.get_string();
        assert!(s.is_some());
        let s = s.unwrap();
        assert!(s == String::from("i am string"));

        let vals = config.get(TestEnum::String_Empty);
        assert!(vals.is_some());
        let vals = vals.unwrap();
        vals.only_one_key().unwrap();

        let s = vals.get_string();
        assert!(s.is_some());
        let s = s.unwrap();
        assert!(s == String::from(""));
    }

    #[test]
    fn test_char() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<TOML>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        let vals = config.get(TestEnum::Char).unwrap();
        vals.only_one_key().unwrap();
        let c = vals.get_char().unwrap();
        assert!(c == 'c');
    }

    // ------------------
    // bool
    #[test]
    fn test_bool() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<TOML>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        // Bool_False
        let vals = config.get(TestEnum::Bool_False).unwrap();
        vals.only_one_key().unwrap();
        let b = vals.get_bool().unwrap();
        assert!(!b);

        // Bool_True
        let vals = config.get(TestEnum::Bool_True).unwrap();
        vals.only_one_key().unwrap();
        let b = vals.get_bool().unwrap();
        assert!(b);
    }

    // ------------------
    // unsigned ints
    #[test]
    fn test_unsigned_integers() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<TOML>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        // U8
        let vals = config.get(TestEnum::U8_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u8().unwrap() == u8::MAX);
        let vals = config.get(TestEnum::U8_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u8().unwrap() == u8::MIN);

        // U16
        let vals = config.get(TestEnum::U16_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u16().unwrap() == u16::MAX);
        let vals = config.get(TestEnum::U16_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u16().unwrap() == u16::MIN);

        // U32
        let vals = config.get(TestEnum::U32_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_u32().unwrap() == u32::MAX);
        let vals = config.get(TestEnum::U32_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_u32().unwrap() == u32::MIN);

        // // U64
        // let vals = config.get(TestEnum::U64_MAX).unwrap(); 
        // vals.only_one_key().unwrap();
        // assert!(vals.get_u64().unwrap() == u64::MAX);
        // let vals = config.get(TestEnum::U64_MIN).unwrap();
        // vals.only_one_key().unwrap();
        // assert!(vals.get_u64().unwrap() == u64::MIN);
        //
        // // U128
        // let vals = config.get(TestEnum::U128_MAX).unwrap(); 
        // vals.only_one_key().unwrap();
        // assert!(vals.get_u128().unwrap() == u128::MAX);
        // let vals = config.get(TestEnum::U128_MIN).unwrap();
        // vals.only_one_key().unwrap();
        // assert!(vals.get_u128().unwrap() == u128::MIN);
    }

    // --------------------
    // signed ints
    #[test]
    fn test_signed_integers() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<TOML>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        // I8
        let vals = config.get(TestEnum::I8_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i8().unwrap() == i8::MAX);
        let vals = config.get(TestEnum::I8_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i8().unwrap() == i8::MIN);

        // I16
        let vals = config.get(TestEnum::I16_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i16().unwrap() == i16::MAX);
        let vals = config.get(TestEnum::I16_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i16().unwrap() == i16::MIN);

        // I32
        let vals = config.get(TestEnum::I32_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i32().unwrap() == i32::MAX);
        let vals = config.get(TestEnum::I32_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i32().unwrap() == i32::MIN);

        // I64
        let vals = config.get(TestEnum::I64_MAX).unwrap(); 
        vals.only_one_key().unwrap();
        assert!(vals.get_i64().unwrap() == i64::MAX);
        let vals = config.get(TestEnum::I64_MIN).unwrap();
        vals.only_one_key().unwrap();
        assert!(vals.get_i64().unwrap() == i64::MIN);

        // // I128
        // let vals = config.get(TestEnum::I128_MAX).unwrap(); 
        // vals.only_one_key().unwrap();
        // assert!(vals.get_i128().unwrap() == i128::MAX);
        // let vals = config.get(TestEnum::I128_MIN).unwrap();
        // vals.only_one_key().unwrap();
        // assert!(vals.get_i128().unwrap() == i128::MIN);
    }

    // --------------------
    // Floats 
    #[test]
    fn test_floats() {
        let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
        testfile.add_all_type_entries(TEST_FILE_TYPE).unwrap();
        let config = Config::<TOML>::open(testfile.get_path()).unwrap();
        testfile.delete().unwrap();

        let vals = config.get(TestEnum::F32_MIN_POSITIVE).unwrap();
        vals.only_one_key().unwrap();
        let v = vals.get_f32().unwrap();
        assert!(v == f32::MIN_POSITIVE);

        let vals = config.get(TestEnum::F64_MIN_POSITIVE).unwrap();
        vals.only_one_key().unwrap();
        let v = vals.get_f64().unwrap();
        assert!(v == f64::MIN_POSITIVE);
    }

}


#[cfg(test)]
mod misc_tests_json {
    use anyhow::Result;
    use quickfig::core::{
        config_types::{ JSON, TOML },
        VecField,
        Field,
        Config,
        GetInner,
    };
    use quickfig::derive::ConfigFields;
    use super::super::utils::*;
    use super::super::utils::TestFileType as TFT;

    const TEST_FILE_TYPE: TestFileType = TFT::JSON;

    #[derive(ConfigFields)]
    enum MiscTestsEnum {
        // Testing default key
        NoKeysAttribute,
        // Config has both "A" and "B"
        #[keys("A", "B")]
        MultipleKeysInConfig,
    }

    #[test]
    fn test_default_key() {
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("NoKeysAttribute", 1u8)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("NoKeysAttribute"));
        let vals = config.get(MiscTestsEnum::NoKeysAttribute).unwrap();
        assert!(vals.len() == 1);
        let should_be_1 = vals.get_u8();
        assert!(should_be_1.is_some_and(|n| n == 1));
    }

    #[test]
    fn test_multiple_keys_exist_in_config_diff_types() {
        // keys("A", "B"), config has A: 1 and B: "string"
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("A", 1u8)).unwrap();
        testconfig.add_entry(("B", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("A"));
        assert!(config.has_key("B"));

        let vals = config.get(MiscTestsEnum::MultipleKeysInConfig).unwrap();
        assert!(vals.len() == 2);
        let should_be_1 = vals.get_u8();
        assert!(should_be_1.is_some_and(|n| n == 1));

        let should_be_foo = vals.get_string();
        assert!(should_be_foo.is_some_and(|f| f == String::from("foo")));
    }

    #[test]
    fn test_multiple_keys_exist_in_config_same_types() {
        // keys("A", "B"), config has A: 1 and B: 9
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("A", 1u8)).unwrap();
        testconfig.add_entry(("B", 9u8)).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("A"));
        assert!(config.has_key("B"));

        let vals = config.get(MiscTestsEnum::MultipleKeysInConfig).unwrap();
        assert!(vals.len() == 2);

        let could_be_either = vals.get_u8();
        assert!(could_be_either.is_some_and(|n| n == 1 || n == 9));
    }


    #[test]
    fn test_multiple_keys_exist_in_config_validation() {
        // keys("A", "B"), config has A: 1 and B: "string"
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("A", 1u8)).unwrap();
        testconfig.add_entry(("B", "foo")).unwrap();
        let config = Config::<JSON>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("A"));
        assert!(config.has_key("B"));

        let vals = config.get(MiscTestsEnum::MultipleKeysInConfig).unwrap();
        assert!(vals.len() == 2);
        let should_be_err = vals.only_one_key();
        assert!(should_be_err.is_err());
    }
}


#[cfg(test)]
mod misc_tests_toml {
    use anyhow::Result;
    use quickfig::core::{
        config_types::{ JSON, TOML },
        VecField,
        Field,
        Config,
        GetInner,
    };
    use quickfig::derive::ConfigFields;
    use super::super::utils::*;
    use super::super::utils::TestFileType as TFT;

    const TEST_FILE_TYPE: TestFileType = TFT::TOML;

    #[derive(ConfigFields)]
    enum MiscTestsEnum {
        // Testing default key
        NoKeysAttribute,
        // Config has both "A" and "B"
        #[keys("A", "B")]
        MultipleKeysInConfig,
    }

    #[test]
    fn test_default_key() {
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("NoKeysAttribute", 1u8)).unwrap();
        let config = Config::<TOML>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("NoKeysAttribute"));
        let vals = config.get(MiscTestsEnum::NoKeysAttribute).unwrap();
        assert!(vals.len() == 1);
        let should_be_1 = vals.get_u8();
        assert!(should_be_1.is_some_and(|n| n == 1));
    }

    #[test]
    fn test_multiple_keys_exist_in_config_diff_types() {
        // keys("A", "B"), config has A: 1 and B: "string"
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("A", 1u8)).unwrap();
        testconfig.add_entry(("B", "foo")).unwrap();
        let config = Config::<TOML>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("A"));
        assert!(config.has_key("B"));

        let vals = config.get(MiscTestsEnum::MultipleKeysInConfig).unwrap();
        assert!(vals.len() == 2);
        let should_be_1 = vals.get_u8();
        assert!(should_be_1.is_some_and(|n| n == 1));

        let should_be_foo = vals.get_string();
        assert!(should_be_foo.is_some_and(|f| f == String::from("foo")));
    }

    #[test]
    fn test_multiple_keys_exist_in_config_same_types() {
        // keys("A", "B"), config has A: 1 and B: 9
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("A", 1u8)).unwrap();
        testconfig.add_entry(("B", 9u8)).unwrap();
        let config = Config::<TOML>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("A"));
        assert!(config.has_key("B"));

        let vals = config.get(MiscTestsEnum::MultipleKeysInConfig).unwrap();
        assert!(vals.len() == 2);

        let could_be_either = vals.get_u8();
        assert!(could_be_either.is_some_and(|n| n == 1 || n == 9));
    }


    #[test]
    fn test_multiple_keys_exist_in_config_validation() {
        // keys("A", "B"), config has A: 1 and B: "string"
        let mut testconfig = TestFile::new(TEST_FILE_TYPE).unwrap();
        testconfig.add_entry(("A", 1u8)).unwrap();
        testconfig.add_entry(("B", "foo")).unwrap();
        let config = Config::<TOML>::open(testconfig.get_path()).unwrap();
        testconfig.delete().unwrap();

        assert!(config.has_key("A"));
        assert!(config.has_key("B"));

        let vals = config.get(MiscTestsEnum::MultipleKeysInConfig).unwrap();
        assert!(vals.len() == 2);
        let should_be_err = vals.only_one_key();
        assert!(should_be_err.is_err());
    }
}

