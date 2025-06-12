use std::str::FromStr;
use substreams::{
    prelude::{BigInt,BigDecimal}
};

/// Converts &[u8] into the expected value type
pub fn get_value_from_bytes<T: FromBytes>(bytes: &[u8]) -> T {
    T::from_bytes(bytes)
}

/// Trait for custom deserialization from bytes
pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
}

// Implementations for each type

impl FromBytes for String {
    fn from_bytes(bytes: &[u8]) -> Self {
        String::from_utf8(bytes.to_vec()).expect("Invalid UTF-8")
    }
}

impl FromBytes for BigInt {
    fn from_bytes(bytes: &[u8]) -> Self {
        BigInt::from_unsigned_bytes_be(bytes)
    }
}

impl FromBytes for BigDecimal {
    fn from_bytes(bytes: &[u8]) -> Self {
        let s = String::from_utf8(bytes.to_vec()).expect("Invalid UTF-8");
        BigDecimal::from_str(&s).expect("Invalid BigDecimal string")
    }
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        let s = String::from_utf8(bytes.to_vec()).expect("Invalid UTF-8");
        s.parse::<f64>().expect("Invalid f64 string")
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        let s = String::from_utf8(bytes.to_vec()).expect("Invalid UTF-8");
        s.parse::<i64>().expect("Invalid i64 string")
    }
}
