// use anyhow::Result;
// use super::utils::*;
// use super::utils::TestFileType as TFT;


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

    #[test]
    fn utils_test() {
        let mut test1: TestFile = TestFile::new(TFT::JSON).unwrap();
        test1.add_entry(("foo", "bar")).unwrap();
        test1.add_entry(("baz", 69)).unwrap();
        test1.add_entry(("boop", true)).unwrap();
        test1.add_entry(("x", vec![1u8, 2u8])).unwrap();
        println!("path: {}", test1.get_path());
        test1.pretty_print().unwrap();
        test1.delete().unwrap();
        println!("----------------------------------");
        let mut test2: TestFile = TestFile::new(TFT::TOML).unwrap();
        test2.add_entry(("foo", "bar")).unwrap();
        test2.add_entry(("baz", 69)).unwrap();
        test2.add_entry(("boop", true)).unwrap();
        test2.add_entry(("x", vec![1u8, 2u8])).unwrap();
        println!("path: {}", test2.get_path());
        test2.pretty_print().unwrap();
        test2.delete().unwrap();
    }

}


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
