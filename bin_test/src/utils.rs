#![allow(non_camel_case_types)]
use std::fs::{File, remove_file};
use std::io::{Write, Read};
use std::path::Path;
use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use serde_json::{Value, to_writer};
use toml::{self, ser};
use dirs::home_dir;

pub type JSON_TEST = serde_json::Value;
pub type TOML_TEST = toml::value::Table;

#[derive(Debug, Clone)]
pub enum TestFileType {
    JSON, TOML
}

#[derive(Debug)]
pub struct TestFile {
    path: String,
    file_type: TestFileType
}

#[derive(Debug)]
pub enum FileError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    TomlError(String),
    InvalidFileType,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::IoError(err) => write!(f, "IO Error: {}", err),
            FileError::SerdeError(err) => write!(f, "Serde Error: {}", err),
            FileError::TomlError(err) => write!(f, "Toml Error: {}", err),
            FileError::InvalidFileType => write!(f, "Invalid file type"),
        }
    }
}

impl TestFile {
    fn random_file_path(ty: TestFileType) -> String {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        let timestamp = duration.as_nanos();
        let mut home_dir = home_dir().expect("cant get home dir");
        home_dir.push("quickfig/bin_test/tmp_configs");
        // home/user/quickfig/bin_test/tmp_configs/test_file_x.json
        match ty {
            TestFileType::JSON => {
                home_dir.push(format!("test_file_{}.json", timestamp));
            },
            TestFileType::TOML => {
                home_dir.push(format!("test_file_{}.toml", timestamp));
            }
        }
        println!("random_file_path: {:#?}", home_dir);
        home_dir.to_str().expect("non-unicode path").to_string()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_type(&self) -> TestFileType {
        self.file_type.clone()
    }

    // * Manually create toml file with u64/u128/i64/i128 entries
    // * Needed since `toml` crate cannot serialize numbers gthan i64::MAX
    //   or less than i64::MIN, but it can deserialize them
    // * File contains:
    // ```toml
    // U64  = 18446744073709551615
    // U128 = 340282366920938463463374607431768211455
    // I64  = 9223372036854775807
    // I128 = 170141183460469231731687303715884105727
    // ```
    // pub fn new_toml_big_nums() -> Result<Self, FileError> {
    //     let path = Self::random_file_path(TestFileType::TOML);
    //     let mut file = std::fs::OpenOptions::new()
    //         .write(true)
    //         .create_new(true)
    //         .open(&path)
    //         .map_err(FileError::IoError)?;
    //     let content = r#""#;
    //     file.write_all(content.as_bytes()).map_err(FileError::IoError)?;
    //     Ok(TestFile {
    //         path,
    //         file_type: TestFileType::TOML
    //     })
    // }

    /// * Generates random path from timestamp
    /// * Creates file at that path
    /// * Returns Self containing the path
    /// * Fails if file already exists
    pub fn new(test_file_type: TestFileType) -> Result<Self, FileError> {
        let path = Self::random_file_path(test_file_type.clone());

        let _file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(FileError::IoError)?;

        Ok(TestFile {
            path,
            file_type: test_file_type
        })
    }

    /// * Same as new but with a provided path
    /// * Fails if file already exists
    /// * **NOTE** will be converted to Full path using home_dir, so
    /// * `new_at_path("foo.json")` creates at
    /// * `/home/user/quickfig/bin_test/tmp_configs/foo.json`
    pub fn new_at_path(path: String, test_file_type: TestFileType) -> Result<Self, FileError> {

        let mut home_dir = home_dir().expect("cant get home dir");
        home_dir.push("quickfig/bin_test/tmp_configs");
        home_dir.push(path);
        // home/user/quickfig/bin_test/tmp_configs/{path}
        let path_str = home_dir.to_str().expect("non-unicode path").to_string();

        let _file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path_str)
            .map_err(FileError::IoError)?;

        Ok(TestFile {
            path: path_str,
            file_type: test_file_type
        })
    }

    /// Deletes file as long as its path contains `tmp_configs`
    pub fn delete(self) -> Result<(), FileError> {
        let p = self.path.to_string();
        // better to have it & not need it
        assert!(p.contains("tmp_configs"));
        remove_file(p).map_err(FileError::IoError)?;
        Ok(())
    }

    /// Pretty print the content of the file
    pub fn pretty_print(&self) -> Result<(), FileError> {
        let mut file = File::open(&self.path).map_err(FileError::IoError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(FileError::IoError)?;

        match self.file_type {
            TestFileType::JSON => {
                let mut json: Value = if contents.is_empty() {
                    Value::Object(serde_json::Map::new())
                } else {
                    serde_json::from_str(&contents).map_err(FileError::SerdeError)?
                };

                let pretty = serde_json::to_string_pretty(&json).map_err(FileError::SerdeError)?;
                println!("{}", pretty);
            },
            TestFileType::TOML => {
                let mut toml_value: toml::value::Table = if contents.is_empty() {
                    toml::value::Table::new()
                } else {
                    toml::de::from_str(&contents).map_err(|e| FileError::TomlError(format!("{:#?}", e)))?
                };

                let pretty = toml::ser::to_string_pretty(&toml_value).unwrap();
                println!("{}", pretty);
            }
        }

        Ok(())
    }


    pub fn add_entry<K, V>(&mut self, entry: (K, V)) -> Result<(), FileError>
        where
            K: Serialize + ToString,
            V: Serialize,
    {
        let mut file = File::open(&self.path).map_err(FileError::IoError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(FileError::IoError)?;

        match self.file_type {
            TestFileType::JSON => {
                let mut json: Value = if contents.is_empty() {
                    Value::Object(serde_json::Map::new())
                } else {
                    serde_json::from_str(&contents).map_err(FileError::SerdeError)?
                };

                let map = json.as_object_mut().unwrap();
                let key = entry.0.to_string();
                let value = serde_json::to_value(entry.1).map_err(FileError::SerdeError)?;
                map.insert(key, value);

                let mut file = File::create(&self.path).map_err(FileError::IoError)?;
                to_writer(&mut file, &json).map_err(FileError::SerdeError)?;

                Ok(())
            },
            TestFileType::TOML => {
                let mut toml_value: toml::value::Table = if contents.is_empty() {
                    toml::value::Table::new()
                } else {
                    toml::de::from_str(&contents).map_err(|e| FileError::TomlError(format!("{:#?}", e)))?
                };

                let key = entry.0.to_string();
                let value = toml::value::Value::try_from(entry.1).map_err(|e| FileError::TomlError(format!("{:#?}", e)))?;

                toml_value.insert(key, value);

                let mut file = File::create(&self.path).map_err(FileError::IoError)?;
                let toml_str = toml::ser::to_string_pretty(&toml_value).unwrap();
                file.write_all(toml_str.as_bytes()).map_err(FileError::IoError)?;

                Ok(())
            }
        }
    }
}
