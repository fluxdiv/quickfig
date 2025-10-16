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
// generics          : testing generic types
//                     with custom deserialization
// 
// json_main         : testing JSON configs
// toml_main         : testing TOML configs
//
// misc_tests_json   : overlapping keys,
// misc_tests_toml   : overlapping keys,


#[cfg(test)]
mod generics {
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
    use serde::Deserialize;

    /// Top level structure
    #[derive(Debug, Deserialize)]
    pub struct Root {
        pub courses: Vec<Course>,
        pub contact: Contact,
    }

    /// Each item in the courses array
    #[derive(Debug, Deserialize)]
    pub struct Course {
        pub title: String,
        pub credits: u32,
        pub details: Option<Details>,
    }

    /// Wrapper around coursrs array
    #[derive(Debug, Deserialize)]
    pub struct Courses(Vec<Course>);

    /// Nested object under details
    #[derive(Debug, Deserialize)]
    pub struct Details {
        pub room_number: u32,
        pub teacher: String,
        pub keywords: Vec<String>,
    }

    /// Contact info at the top level
    #[derive(Debug, Deserialize)]
    pub struct Contact {
        pub email: String,
        // JSON uses null, TOML uses ""
        pub phone: Option<String>, 
    }

    #[allow(non_camel_case_types)]
    #[derive(ConfigFields)]
    pub enum GenericTestEnum {
        #[keys("courses")]
        Courses,
        #[keys("contact")]
        Contact,
        #[keys("not_there")]
        NotThere,
    }

    #[cfg(test)]
    mod json_generics {
        use super::*;
        const TEST_FILE_TYPE: TestFileType = TFT::JSON;

        #[test]
        fn test_generic() {
            let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
            testfile.add_all_generic_entries(TEST_FILE_TYPE).unwrap();
            let config = Config::<JSON>::open(testfile.get_path()).unwrap();
            testfile.delete().unwrap();

            // Courses
            let courses = config.get(GenericTestEnum::Courses).unwrap();
            courses.only_one_key().unwrap();
            // Should be the array of courses
            let courses_inner = courses.get_generic_inner().unwrap();
            let courses_de = Courses::deserialize(courses_inner).unwrap();
            let c_vec = courses_de.0;
            assert!(c_vec.len() == 2);
            let history = &c_vec[0];
            assert_eq!(history.title, "History 101");
            assert_eq!(history.credits, 3);
            assert!(history.details.is_some());
            let details = history.details.as_ref().unwrap();
            assert_eq!(details.room_number, 413);
            assert_eq!(details.teacher, "Lopez");
            assert_eq!(details.keywords, vec!["US", "History", "Introduction"]);
            let math = &c_vec[1];
            assert_eq!(math.title, "Mathematics 201");
            assert_eq!(math.credits, 4);
            assert!(math.details.is_none());

            // Contact
            let contact = config.get(GenericTestEnum::Contact).unwrap();
            contact.only_one_key().unwrap();
            // Should deserialize into contact
            let contact_inner = contact.get_generic_inner().unwrap();
            let contact_de = Contact::deserialize(contact_inner).unwrap();
            assert_eq!(contact_de.email, String::from("john.smith@example.com"));
            assert_eq!(contact_de.phone, None);

            // Not there
            let e = config.get(GenericTestEnum::NotThere);
            assert!(e.is_none());
        }
    }

    #[cfg(test)]
    mod toml_generics {
        use super::*;
        const TEST_FILE_TYPE: TestFileType = TFT::TOML;

        #[test]
        fn test_generic() {
            let mut testfile = TestFile::new(TEST_FILE_TYPE).unwrap();
            testfile.add_all_generic_entries(TEST_FILE_TYPE).unwrap();
            let config = Config::<TOML>::open(testfile.get_path()).unwrap();
            testfile.delete().unwrap();

            // Courses
            let courses = config.get(GenericTestEnum::Courses).unwrap();
            courses.only_one_key().unwrap();
            // Should be the array of courses
            let courses_inner = courses.get_generic_inner().unwrap();
            let courses_de = Courses::deserialize(courses_inner.clone()).unwrap();
            let c_vec = courses_de.0;
            assert!(c_vec.len() == 2);
            let history = &c_vec[0];
            assert_eq!(history.title, "History 101");
            assert_eq!(history.credits, 3);
            assert!(history.details.is_some());
            let details = history.details.as_ref().unwrap();
            assert_eq!(details.room_number, 413);
            assert_eq!(details.teacher, "Lopez");
            assert_eq!(details.keywords, vec!["US", "History", "Introduction"]);
            let math = &c_vec[1];
            assert_eq!(math.title, "Mathematics 201");
            assert_eq!(math.credits, 4);
            assert!(math.details.is_none());

            // Contact
            let contact = config.get(GenericTestEnum::Contact).unwrap();
            contact.only_one_key().unwrap();
            // Should deserialize into contact
            let contact_inner = contact.get_generic_inner().unwrap();
            let contact_de = Contact::deserialize(contact_inner.clone()).unwrap();
            assert_eq!(contact_de.email, String::from("john.smith@example.com"));
            // toml uses empty strings not null
            assert!(contact_de.phone.is_some_and(|x| x.is_empty()));

            // Not there
            let e = config.get(GenericTestEnum::NotThere);
            assert!(e.is_none());
        }
    }

}

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

