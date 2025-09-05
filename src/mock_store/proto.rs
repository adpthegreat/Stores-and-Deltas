//! Protobuf helpers for Substreams.
//!
//! This crate offers a few protobuf helper functions which
//! are used across Substreams
//!

use prost::{DecodeError, EncodeError};

/// Given an array of bytes, it will decode data in a Protobuf Message
pub fn decode<T: Default + prost::Message>(buf: &Vec<u8>) -> Result<T, DecodeError> {
    ::prost::Message::decode(&buf[..])
}

/// Given a Protobuf message it will encode it and return the byte array.
pub fn encode<M: prost::Message>(msg: &M) -> Result<Vec<u8>, EncodeError> {
    let mut buf = Vec::new();

    let encoded_len = msg.encoded_len();
    buf.reserve(encoded_len);

    match msg.encode(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    }
}

