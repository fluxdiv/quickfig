use anyhow::{Result, anyhow};
use syn::{GenericArgument, Type, TypePath, PathArguments};

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
/// * Contains its associated key, can be retrieved via `.get_key()`
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Field {
    String { key: String, value: String }, Char { key: String, value: char }, 
    U8 { key: String, value: u8 }, U16 { key: String, value: u16 }, 
    U32 { key: String, value: u32 }, U64 { key: String, value: u64 }, 
    U128 { key: String, value: u128 }, 

    I8 { key: String, value: i8 }, I16 { key: String, value: i16 },
    I32 { key: String, value: i32 }, I64 { key: String, value: i64 },
    I128 { key: String, value: i128 }, 

    F32 { key: String, value: f32 }, F64 { key: String, value: f64 }, 

    Bool { key: String, value: bool }, 
    Vec { key: String, value: Box<Field> },
}

impl FieldMarker {
    pub fn from_type_path(type_path: &TypePath) -> Option<Self> {
        let segment = &type_path.path.segments.last().unwrap();
        let type_name = segment.ident.to_string();

        match type_name.as_str() {
            "String" => Some(FieldMarker::String),
            "char" => Some(FieldMarker::Char),
            "u8" => Some(FieldMarker::U8),
            "u16" => Some(FieldMarker::U16),
            "u32" => Some(FieldMarker::U32),
            "u64" => Some(FieldMarker::U64),
            "u128" => Some(FieldMarker::U128),
            "i8" => Some(FieldMarker::I8),
            "i16" => Some(FieldMarker::I16),
            "i32" => Some(FieldMarker::I32),
            "i64" => Some(FieldMarker::I64),
            "i128" => Some(FieldMarker::I128),
            "bool" => Some(FieldMarker::Bool),
            "f32" => Some(FieldMarker::F32),
            "f64" => Some(FieldMarker::F64),
            "Vec" => {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(Type::Path(inner_path))) = args.args.first() {
                        let inner = FieldMarker::from_type_path(inner_path)?;
                        Some(FieldMarker::Vec(Box::new(inner)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}



pub trait VecField {

    fn only_one_key(self) -> Result<Vec<Field>>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `String` was unsuccessful, or
    /// * `Some(String)` where `String` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(String)` being the first non-none result of calling `field.get_string()`
    /// * `None` if all returned `None`
    fn get_string(&self) -> Option<String>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `char` was unsuccessful, or
    /// * `Some(char)` where `char` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(char)` being the first non-none result of calling `field.get_char()`
    /// * `None` if all returned `None`
    fn get_char(&self) -> Option<char>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `u8` was unsuccessful, or
    /// * `Some(u8)` where `u8` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(u8)` being the first non-none result of calling `field.get_u8()`
    /// * `None` if all returned `None`
    fn get_u8(&self) -> Option<u8>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `u16` was unsuccessful, or
    /// * `Some(u16)` where `u16` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(u16)` being the first non-none result of calling `field.get_u16()`
    /// * `None` if all returned `None`
    fn get_u16(&self) -> Option<u16>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `u32` was unsuccessful, or
    /// * `Some(u32)` where `u32` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(u32)` being the first non-none result of calling `field.get_u32()`
    /// * `None` if all returned `None`
    fn get_u32(&self) -> Option<u32>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `u64` was unsuccessful, or
    /// * `Some(u64)` where `u64` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(u64)` being the first non-none result of calling `field.get_u64()`
    /// * `None` if all returned `None`
    fn get_u64(&self) -> Option<u64>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `u128` was unsuccessful, or
    /// * `Some(u128)` where `u128` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(u128)` being the first non-none result of calling `field.get_u128()`
    /// * `None` if all returned `None`
    fn get_u128(&self) -> Option<u128>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `i8` was unsuccessful, or
    /// * `Some(i8)` where `i8` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(i8)` being the first non-none result of calling `field.get_i8()`
    /// * `None` if all returned `None`
    fn get_i8(&self) -> Option<i8>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `i16` was unsuccessful, or
    /// * `Some(i16)` where `i16` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(i16)` being the first non-none result of calling `field.get_i16()`
    /// * `None` if all returned `None`
    fn get_i16(&self) -> Option<i16>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `i32` was unsuccessful, or
    /// * `Some(i32)` where `i32` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(i32)` being the first non-none result of calling `field.get_i32()`
    /// * `None` if all returned `None`
    fn get_i32(&self) -> Option<i32>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `i64` was unsuccessful, or
    /// * `Some(i64)` where `i64` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(i64)` being the first non-none result of calling `field.get_i64()`
    /// * `None` if all returned `None`
    fn get_i64(&self) -> Option<i64>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `i128` was unsuccessful, or
    /// * `Some(i128)` where `i128` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(i128)` being the first non-none result of calling `field.get_i128()`
    /// * `None` if all returned `None`
    fn get_i128(&self) -> Option<i128>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `bool` was unsuccessful, or
    /// * `Some(bool)` where `bool` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(bool)` being the first non-none result of calling `field.get_bool()`
    /// * `None` if all returned `None`
    fn get_bool(&self) -> Option<bool>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `f32` was unsuccessful, or
    /// * `Some(f32)` where `f32` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(f32)` being the first non-none result of calling `field.get_f32()`
    /// * `None` if all returned `None`
    fn get_f32(&self) -> Option<f32>;

    /// # If you have validated the `Vec<Field>` using the `.only_one_key()` method,
    /// then this method is guaranteed to return either:
    /// * `None` if parsing the field into `f64` was unsuccessful, or
    /// * `Some(f64)` where `f64` is the result of parsing the key's associated field.
    /// # If you have not validated using `.only_one_key()` method,
    /// this method iterates through the `Vec<Field>` and returns:
    /// * `Some(f64)` being the first non-none result of calling `field.get_f64()`
    /// * `None` if all returned `None`
    fn get_f64(&self) -> Option<f64>;
}


impl VecField for Vec<Field> {

    /// Validates that all `Field`s have the same key.
    /// If this returns successfully, it is guaranteed that
    /// there will be *at most* 1 instance of each variant of Field.
    /// # Returns
    /// * `Ok(self)` if all `Field`s have the same key
    /// * `Err` if more than 1 key
    fn only_one_key(self) -> Result<Vec<Field>> {
        assert!(self.len() >= 1);
        let mut iter = self.iter();
        let key = iter.next().unwrap().get_key();
        for field in iter {
            let k = field.get_key();
            if k != key {
                return Err(anyhow!("Non equal keys found: \"{}\" and \"{}\"", k, key));
            }
        }
        Ok(self)
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


impl GetInner for Field {
    fn get_key(&self) -> String {
        match self {
            Self::String { key, .. } |
            Self::Char   { key, .. } |
            Self::U8     { key, .. } |
            Self::U16    { key, .. } |
            Self::U32    { key, .. } |
            Self::U64    { key, .. } |
            Self::U128   { key, .. } |
            Self::I8     { key, .. } |
            Self::I16    { key, .. } |
            Self::I32    { key, .. } |
            Self::I64    { key, .. } |
            Self::I128   { key, .. } |
            Self::Bool   { key, .. } |
            Self::F32    { key, .. } |
            Self::F64    { key, .. } |
            Self::Vec    { key, .. } => key.clone(),
        }
    }

    fn get_string(&self) -> Option<String> {
        match self {
            Self::String { value, .. } => Some(value.clone()),
            _ => None,
        }
    }

    fn get_char(&self) -> Option<char> {
        match self {
            Self::Char { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_u8(&self) -> Option<u8> {
        match self {
            Self::U8 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_u16(&self) -> Option<u16> {
        match self {
            Self::U16 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_u32(&self) -> Option<u32> {
        match self {
            Self::U32 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_u64(&self) -> Option<u64> {
        match self {
            Self::U64 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_u128(&self) -> Option<u128> {
        match self {
            Self::U128 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_i8(&self) -> Option<i8> {
        match self {
            Self::I8 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_i16(&self) -> Option<i16> {
        match self {
            Self::I16 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_i32(&self) -> Option<i32> {
        match self {
            Self::I32 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_i64(&self) -> Option<i64> {
        match self {
            Self::I64 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_i128(&self) -> Option<i128> {
        match self {
            Self::I128 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Bool { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_f32(&self) -> Option<f32> {
        match self {
            Self::F32 { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn get_f64(&self) -> Option<f64> {
        match self {
            Self::F64 { value, .. } => Some(*value),
            _ => None,
        }
    }
}

