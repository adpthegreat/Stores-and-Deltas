use std::str::FromStr;
use substreams::{
    prelude::{BigInt,BigDecimal}
};
use prost::{Message, Name};
use crate::mock_store::proto;

/// Converts &[u8] into the expected value type
pub fn get_value_from_bytes<T: FromBytes>(bytes: &[u8]) -> T {
    T::from_bytes(bytes)
}

pub fn convert_value_to_bytes<T: ToBytes>(val: &T) -> Vec<u8> {
    T::to_bytes(val)
}
 
/// Trait for custom deserialization from bytes
pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait FromBytesProto: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
}

//&self -> an instance of the type that implements the trait 
pub trait ToBytes : Sized + FromBytes + ToString {
    fn to_bytes(&self) -> Vec<u8>;
}

// Make another trait definition for protos?
pub trait ToBytesProto : Sized + FromBytesProto + Message + Default {
    fn to_bytes(&self) -> Vec<u8>;
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
        let s = String::from_utf8(bytes.to_vec()).expect("Invalid UTF-8 string");
        s.parse::<i64>().expect("Invalid i64 string") 
    }
}

impl<T> FromBytesProto for T 
where 
    T: Message + Default,
{
    fn from_bytes(bytes: &[u8]) -> Self {
        let s = proto::decode(&bytes.to_vec()).expect("error when decoding");
        s
    }   
}

macro_rules! encode_to_bytes {
    ($type:ty) => {
        impl ToBytes for $type {
            fn to_bytes(&self) -> Vec<u8> {
                let val = self.to_string();
                let bytes_val = val.as_bytes().to_vec(); 
                bytes_val
            }   
        }
    }
}

encode_to_bytes!(String);
encode_to_bytes!(BigInt);
encode_to_bytes!(BigDecimal);
encode_to_bytes!(f64);
encode_to_bytes!(i64);

//trying to create an impl for ToBytes for Proto (Message) type leads to overlapping implementations
//so we are creating an entirely new trait for it 
impl<T> ToBytesProto for T 
where 
    T: Message + Default
{
    fn to_bytes(&self) -> Vec<u8> {
        let s = proto::encode(self).expect("error when encoding proto");
        s
    }   
}

  