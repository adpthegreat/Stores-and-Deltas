// https://github.com/streamingfast/substreams-rs/blob/ebaf5ebe0c03313fd3cfb144080f138f81367887/substreams/src/key.rs#L1
use substreams::{store::Delta};
use std::io::BufRead;

//The only relevant thing i changed here was removing the use of std::io::Cursor will just keep it here lol

pub fn segment_at_owned(key: String, index: usize) -> String {
    let mut parts = key.split(":");

    // Use of unwrap because those who want to check errors must use the try_ version
    let segment_result = parts.nth(index).unwrap_or_else(|| {
        panic!(
            "Unable to extract segment index {} for key {}",
            index,
            parts
                .map(|x| String::from_utf8(x.into())
                    .expect("Must be valid UTF-8 here as we split an initially valid String"))
                .collect::<Vec<_>>()
                .join(":"),
        )
    });

    String::from_utf8(segment_result.into())
        .expect("Must be valid UTF-8 here as we split an initially valid String")
}


