use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;
use crate::config_types::DeserializedConfig;

/// Marker type for Field which wraps the value
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum FieldMarker {
    String,
    Char,
    U8,U16,U32,U64,U128,
    I8,I16,I32,I64,I128,
    Bool,
    F32,F64,
    Vec(Box<FieldMarker>),
}

/// Field of a config
/// * Wraps the value held in that field
/// * Contains the associated key, can be retrieved via `.get_key()`
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Field<'config, S: DeserializeOwned + DeserializedConfig> {
    key: String,
    value: &'config S
}

impl<'a, S: DeserializeOwned + DeserializedConfig> Field<'a, S> {
    pub fn new(key: &str, value: &'a S) -> Field<'a, S> {
        Field { key: key.to_string(), value }
    }

    pub fn get_inner(&'a self) -> &'a S {
        self.value
    }
}

pub trait VecField<S: DeserializeOwned + DeserializedConfig> {
    fn only_one_key(&self) -> Result<()>;
    /// Get the inner deserializable value for custom deserialization
    fn get_generic_inner(&self) -> Option<&S>;
    fn get_wrapper(&self) -> Option<&Field<'_, S>>;
    fn get_string(&self) -> Option<String>;
    fn get_char(&self) -> Option<char>;
    fn get_u8(&self) -> Option<u8>;
    fn get_u16(&self) -> Option<u16>;
    fn get_u32(&self) -> Option<u32>;
    fn get_u64(&self) -> Option<u64>;
    fn get_u128(&self) -> Option<u128>;
    fn get_i8(&self) -> Option<i8>;
    fn get_i16(&self) -> Option<i16>;
    fn get_i32(&self) -> Option<i32>;
    fn get_i64(&self) -> Option<i64>;
    fn get_i128(&self) -> Option<i128>;
    fn get_bool(&self) -> Option<bool>;
    fn get_f32(&self) -> Option<f32>;
    fn get_f64(&self) -> Option<f64>;
}

impl<S: DeserializeOwned + DeserializedConfig> VecField<S> for Vec<Field<'_, S>> {

    // Validates that all `Field`s have the same key.
    // If this returns successfully, it is guaranteed that
    // there will be *at most* 1 instance of each variant of Field.
    // # Returns
    // * `Ok(())` if all `Field`s have the same key
    // * `Err` if more than 1 key
    fn only_one_key(&self) -> Result<()> {
        assert!(self.len() >= 1);
        let mut iter = self.iter();
        let key = iter.next().unwrap().get_key();
        for field in iter {
            let k = field.get_key();
            if k!= key {
                return Err(anyhow!("Non equal keys found: \"{}\" and \"{}\"", k, key));
            }
        }
        Ok(())
    }

    fn get_generic_inner(&self) -> Option<&S> {
        Some(self.get_wrapper()?.get_inner())
    }
    fn get_wrapper(&self) -> Option<&Field<'_, S>> {
        self.iter().find_map(|field| field.get_wrapper())
    }
    fn get_string(&self) -> Option<String> {
        self.iter().find_map(|field| field.get_string())
    }
    fn get_char(&self) -> Option<char> {
        self.iter().find_map(|f| f.get_char())
    }
    fn get_u8(&self) -> Option<u8> {
        self.iter().find_map(|f| f.get_u8())
    }
    fn get_u16(&self) -> Option<u16> {
        self.iter().find_map(|f| f.get_u16())
    }
    fn get_u32(&self) -> Option<u32> {
        self.iter().find_map(|f| f.get_u32())
    }
    fn get_u64(&self) -> Option<u64> {
        self.iter().find_map(|f| f.get_u64())
    }
    fn get_u128(&self) -> Option<u128> {
        self.iter().find_map(|f| f.get_u128())
    }
    fn get_i8(&self) -> Option<i8> {
        self.iter().find_map(|f| f.get_i8())
    }
    fn get_i16(&self) -> Option<i16> {
        self.iter().find_map(|f| f.get_i16())
    }
    fn get_i32(&self) -> Option<i32> {
        self.iter().find_map(|f| f.get_i32())
    }
    fn get_i64(&self) -> Option<i64> {
        self.iter().find_map(|f| f.get_i64())
    }
    fn get_i128(&self) -> Option<i128> {
        self.iter().find_map(|f| f.get_i128())
    }
    fn get_bool(&self) -> Option<bool> {
        self.iter().find_map(|f| f.get_bool())
    }
    fn get_f32(&self) -> Option<f32> {
        self.iter().find_map(|f| f.get_f32())
    }
    fn get_f64(&self) -> Option<f64> {
        self.iter().find_map(|f| f.get_f64())
    }

}

pub trait GetInner {
    /// Get the key associated with this `Field`
    fn get_key(&self) -> String;
    /// * Get the wrapper (Field) Deserializable value of this field
    /// * Use if you need custom deserialization
    fn get_wrapper(&self) -> Option<&Self>;
    /// * Get the parsed `String` of this `Field`
    /// * Returns `None` if field could not be parsed to String
    fn get_string(&self) -> Option<String>;
    /// * Get the parsed `char` of this `Field`
    /// * Returns `None` if field could not be parsed to `char`
    fn get_char(&self) -> Option<char>;
    /// * Get the parsed `u8` of this `Field`
    /// * Returns `None` if field could not be parsed to `u8`
    fn get_u8(&self) -> Option<u8>;
    /// * Get the parsed `u16` of this `Field`
    /// * Returns `None` if field could not be parsed to `u16`
    fn get_u16(&self) -> Option<u16>;
    /// * Get the parsed `u32` of this `Field`
    /// * Returns `None` if field could not be parsed to `u32`
    fn get_u32(&self) -> Option<u32>;
    /// * Get the parsed `u64` of this `Field`
    /// * Returns `None` if field could not be parsed to `u64`
    fn get_u64(&self) -> Option<u64>;
    /// * Get the parsed `u128` of this `Field`
    /// * Returns `None` if field could not be parsed to `u128`
    fn get_u128(&self) -> Option<u128>;
    /// * Get the parsed `i8` of this `Field`
    /// * Returns `None` if field could not be parsed to `i8`
    fn get_i8(&self) -> Option<i8>;
    /// * Get the parsed `i16` of this `Field`
    /// * Returns `None` if field could not be parsed to `i16`
    fn get_i16(&self) -> Option<i16>;
    /// * Get the parsed `i32` of this `Field`
    /// * Returns `None` if field could not be parsed to `i32`
    fn get_i32(&self) -> Option<i32>;
    /// * Get the parsed `i64` of this `Field`
    /// * Returns `None` if field could not be parsed to `i64`
    fn get_i64(&self) -> Option<i64>;
    /// * Get the parsed `i128` of this `Field`
    /// * Returns `None` if field could not be parsed to `i128`
    fn get_i128(&self) -> Option<i128>;
    /// * Get the parsed `bool` of this `Field`
    /// * Returns `None` if field could not be parsed to `bool`
    fn get_bool(&self) -> Option<bool>;
    /// * Get the parsed `f32` of this `Field`
    /// * Returns `None` if field could not be parsed to `f32`
    fn get_f32(&self) -> Option<f32>;
    /// * Get the parsed `f64` of this `Field`
    /// * Returns `None` if field could not be parsed to `f64`
    fn get_f64(&self) -> Option<f64>;
}


impl<S: DeserializeOwned + DeserializedConfig> GetInner for Field<'_, S> {
    fn get_key(&self) -> String {
        self.key.clone()
    }

    fn get_wrapper(&self) -> Option<&Self> {
        Some(self)
    }

    fn get_string(&self) -> Option<String> {
        self.value.get_string()
    }

    fn get_char(&self) -> Option<char> {
        self.value.get_char()
    }

    fn get_u8(&self) -> Option<u8> {
        self.value.get_u8()
    }

    fn get_u16(&self) -> Option<u16> {
        self.value.get_u16()
    }

    fn get_u32(&self) -> Option<u32> {
        self.value.get_u32()
    }

    fn get_u64(&self) -> Option<u64> {
        self.value.get_u64()
    }

    fn get_u128(&self) -> Option<u128> {
        self.value.get_u128()
    }

    fn get_i8(&self) -> Option<i8> {
        self.value.get_i8()
    }

    fn get_i16(&self) -> Option<i16> {
        self.value.get_i16()
    }

    fn get_i32(&self) -> Option<i32> {
        self.value.get_i32()
    }

    fn get_i64(&self) -> Option<i64> {
        self.value.get_i64()
    }

    fn get_i128(&self) -> Option<i128> {
        self.value.get_i128()
    }

    fn get_bool(&self) -> Option<bool> {
        self.value.get_bool()
    }

    fn get_f32(&self) -> Option<f32> {
        self.value.get_f32()
    }

    fn get_f64(&self) -> Option<f64> {
        self.value.get_f64()
    }

}

