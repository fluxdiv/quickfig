// use anyhow::Result;
// use super::utils::*;
// use super::utils::TestFileType as TFT;

// MODS
// 
// tests_core        : testing quickfig_core (Config,)
// tests_misc        : misc Config tests
// tests_utils       : testing the test utils

#[cfg(test)]
mod tests_core {
    use super::super::utils::*;
    use super::super::utils::TestFileType as TFT;
    use quickfig::core::{
        config_types::{ JSON, TOML },
        Config,
    };

    #[test]
    fn test_open_json() {
        let mut test_json = TestFile::new(TFT::JSON).unwrap();
        test_json.add_entry(("foo", 69)).unwrap();
        let _config = Config::<JSON>::open(test_json.get_path()).unwrap();
        test_json.delete().unwrap();
    }

    #[test]
    fn test_open_toml() {
        let mut test_toml = TestFile::new(TFT::TOML).unwrap();
        test_toml.add_entry(("foo", 69)).unwrap();
        let _config = Config::<TOML>::open(test_toml.get_path()).unwrap();
        test_toml.delete().unwrap();
    }

    #[test]
    fn test_open_first_match_json() {
        let mut t1 = TestFile::new(TFT::JSON).unwrap();
        t1.add_entry(("foo", 1)).unwrap();
        let t1path = t1.get_path();

        let mut t2 = TestFile::new(TFT::JSON).unwrap();
        t2.add_entry(("bar", 2)).unwrap();
        let t2path = t2.get_path();

        let t3p_string = String::from("t3_test_open_first_match_json.json");
        let mut t3 = TestFile::new_at_path(t3p_string.clone(), TFT::JSON).unwrap();
        t3.add_entry(("tot", 3)).unwrap();
        let t3path = t3.get_path();

        let paths = vec![t1path, t2path, t3path.clone()];

        // Should be t1 since it is first & exists (default `search` fn)
        let config_default = Config::<JSON>::open_first_match(
            paths.clone(),
            None
        ).unwrap();
        assert!(config_default.has_key("foo"));
        // ------------------------------------------------------
        // Should be t3 
        let config_custom = Config::<JSON>::open_first_match(
            paths,
            Some(Box::new(move |path| {
                let t3p_aspath: std::path::PathBuf = t3path.clone().into();
                if path.eq(&t3p_aspath) {
                    // println!("path {:#?} == t3p_aspath {:#?}", path, t3p_aspath);
                    Some(path)
                } else {
                    // println!("path {:#?} != t3p_aspath {:#?}", path, t3p_aspath);
                    None
                }
            }))
        ).unwrap();
        assert!(config_custom.has_key("tot"));

        t1.delete().unwrap();
        t2.delete().unwrap();
        t3.delete().unwrap();
    }

    #[test]
    fn test_open_first_match_toml() {
        let mut t1 = TestFile::new(TFT::TOML).unwrap();
        t1.add_entry(("foo", 1)).unwrap();
        let t1path = t1.get_path();

        let mut t2 = TestFile::new(TFT::TOML).unwrap();
        t2.add_entry(("bar", 2)).unwrap();
        let t2path = t2.get_path();

        let t3p_string = String::from("t3_test_open_first_match_toml.toml");
        let mut t3 = TestFile::new_at_path(t3p_string.clone(), TFT::TOML).unwrap();
        t3.add_entry(("tot", 3)).unwrap();
        let t3path = t3.get_path();

        let paths = vec![t1path, t2path, t3path.clone()];

        // Should be t1 since it is first & exists (default `search` fn)
        let config_default = Config::<TOML>::open_first_match(
            paths.clone(),
            None
        ).unwrap();
        assert!(config_default.has_key("foo"));
        // ------------------------------------------------------
        // Should be t3 
        let config_custom = Config::<TOML>::open_first_match(
            paths,
            Some(Box::new(move |path| {
                let t3p_aspath: std::path::PathBuf = t3path.clone().into();
                if path.eq(&t3p_aspath) {
                    // println!("path {:#?} == t3p_aspath {:#?}", path, t3p_aspath);
                    Some(path)
                } else {
                    // println!("path {:#?} != t3p_aspath {:#?}", path, t3p_aspath);
                    None
                }
            }))
        ).unwrap();
        assert!(config_custom.has_key("tot"));

        t1.delete().unwrap();
        t2.delete().unwrap();
        t3.delete().unwrap();
    }
}


#[cfg(test)]
mod tests_misc {
    use super::super::utils::*;
    use super::super::utils::TestFileType as TFT;
    use quickfig::core::{
        config_types::{ JSON, TOML },
        Config,
    };

    #[test]
    fn test_empty_file_1() {
        // Trying to create Config w/ empty json file should error
        let test1: TestFile = TestFile::new(TFT::JSON).unwrap();
        let config_err = Config::<JSON>::open(test1.get_path());
        assert!(config_err.is_err());
        test1.delete().unwrap();
    }

    #[test]
    fn test_empty_file_2() {
        // Trying to create Config w/ empty toml file should error
        let test2: TestFile = TestFile::new(TFT::TOML).unwrap();
        let config_err2 = Config::<TOML>::open(test2.get_path());
        assert!(config_err2.is_err());
        test2.delete().unwrap();
    }
}


#[cfg(test)]
mod tests_utils {
    use super::super::utils::*;
    use super::super::utils::TestFileType as TFT;
    use quickfig::core::{
        config_types::{ JSON, TOML },
        Config,
    };

    #[test]
    fn test_add_all_type_entries_json() {
        let mut testfile = TestFile::new(TFT::JSON).unwrap();
        testfile.add_all_type_entries(TFT::JSON).unwrap();
        let config = Config::<JSON>::open(testfile.get_path()).unwrap();
        // testfile.pretty_print().unwrap();
        testfile.delete().unwrap();
        // String & char
        assert!(config.has_key("String"));
        assert!(config.has_key("String_Empty"));
        assert!(config.has_key("Char"));
        // Booleans
        assert!(config.has_key("Bool_False"));
        assert!(config.has_key("Bool_True"));
        // Unsigned integers
        assert!(config.has_key("U8_MAX"));
        assert!(config.has_key("U8_MIN"));
        assert!(config.has_key("U16_MAX"));
        assert!(config.has_key("U16_MIN"));
        assert!(config.has_key("U32_MAX"));
        assert!(config.has_key("U32_MIN"));
        assert!(config.has_key("U64_MAX"));
        assert!(config.has_key("U64_MIN"));
        assert!(config.has_key("U128_MAX"));
        assert!(config.has_key("U128_MIN"));
        // Signed integers
        assert!(config.has_key("I8_MAX"));
        assert!(config.has_key("I8_MIN"));
        assert!(config.has_key("I16_MAX"));
        assert!(config.has_key("I16_MIN"));
        assert!(config.has_key("I32_MAX"));
        assert!(config.has_key("I32_MIN"));
        assert!(config.has_key("I64_MAX"));
        assert!(config.has_key("I64_MIN"));
        assert!(config.has_key("I128_MAX"));
        assert!(config.has_key("I128_MIN"));
        // Floating point minimum positive values
        assert!(config.has_key("F32_MIN_POSITIVE"));
        assert!(config.has_key("F64_MIN_POSITIVE"));
    }

    #[test]
    fn test_add_all_type_entries_toml() {
        let mut testfile = TestFile::new(TFT::TOML).unwrap();
        testfile.add_all_type_entries(TFT::TOML).unwrap();
        let config = Config::<TOML>::open(testfile.get_path()).unwrap();
        // testfile.pretty_print().unwrap();
        testfile.delete().unwrap();
        // String & char
        assert!(config.has_key("String"));
        assert!(config.has_key("String_Empty"));
        assert!(config.has_key("Char"));
        // Booleans
        assert!(config.has_key("Bool_False"));
        assert!(config.has_key("Bool_True"));
        // Unsigned integers
        assert!(config.has_key("U8_MAX"));
        assert!(config.has_key("U8_MIN"));
        assert!(config.has_key("U16_MAX"));
        assert!(config.has_key("U16_MIN"));
        assert!(config.has_key("U32_MAX"));
        assert!(config.has_key("U32_MIN"));
        // assert!(config.has_key("U64_MAX"));
        // assert!(config.has_key("U64_MIN"));
        // assert!(config.has_key("U128_MAX"));
        // assert!(config.has_key("U128_MIN"));
        // Signed integers
        assert!(config.has_key("I8_MAX"));
        assert!(config.has_key("I8_MIN"));
        assert!(config.has_key("I16_MAX"));
        assert!(config.has_key("I16_MIN"));
        assert!(config.has_key("I32_MAX"));
        assert!(config.has_key("I32_MIN"));
        assert!(config.has_key("I64_MAX"));
        assert!(config.has_key("I64_MIN"));
        // assert!(config.has_key("I128_MAX"));
        // assert!(config.has_key("I128_MIN"));
        // Floating point minimum positive values
        assert!(config.has_key("F32_MIN_POSITIVE"));
        assert!(config.has_key("F64_MIN_POSITIVE"));
    }
}

