//Deltas -> to be implemented, from https://github.com/streamingfast/substreams-rs/blob/995a9bfcc15ebd59df63bdb2ce1b5d095d189d06/substreams/src/store.rs#L1241
use crate::mock_store::{
    key,
    traits::get_value_from_bytes,
};
use substreams::{
    prelude::{BigInt, BigDecimal},
    pb::substreams::{
        StoreDelta,
        store_delta::{Operation},
    },
    store::{StoreAdd, StoreSetSum, Delta, DeltaBigInt, DeltaBigDecimal, DeltaBool, DeltaBytes, DeltaFloat64, DeltaInt32, DeltaInt64},
    // key,
};

//Mock Deltas are a non issue because they are basically a Vec<Deltas> that you pass in to a 
//Delta::new() constructor, it has its own custom iterators for you and everythng you need outside the box

//We only had to implement stroes because we needed a way to access the key value state to mock it for testing 

// DeltaBigDecimal 
// DeltaBigInt 
// DeltaInt32 
// DeltaInt64 
// DeltaFloat64 
// DeltaBytes

// DeltaBool --
// DeltaString --
// DeltaProto --
// Delta Array --

//can also implement the structs with this cool macro lol
// macro_rules! generate_delta_struct {
//     ($name:ident, $type:ty) => {
//         #[derive(Debug, Clone, PartialEq)]
//         pub struct $name {
//             pub operation: Operation,
//             pub ordinal: u64,
//             pub key: String,
//             pub old_value: $type,
//             pub new_value: $type,
//         }
//         impl From<StoreDelta> for $name {
//             fn from(d: StoreDelta) -> Self {
//                 Self {
//                     operation: convert_i32_to_operation(d.operation),
//                     ordinal: d.ordinal,
//                     key: d.key,
//                     old_value: get_value_from_bytes::<$type>(&d.old_value),
//                     new_value: get_value_from_bytes::<$type>(&d.new_value),
//                 }
//             }
//         }
//     }
// }


// generate_delta_struct!(DeltaBigDecimal, BigDecimal);
// generate_delta_struct!(DeltaBigInt, BigInt);
// generate_delta_struct!(DeltaInt32, i32);
// generate_delta_struct!(DeltaInt64, i64);
// generate_delta_struct!(DeltaFloat64, f64);
// generate_delta_struct!(DeltaBytes, Vec<u8>);

//we can just import the deltas and the key::segment stuff and all that and write unit tests for them , if we really wanna test how they work 


//https://github.com/streamingfast/substreams-rs/blob/995a9bfcc15ebd59df63bdb2ce1b5d095d189d06/substreams/src/store.rs#L1319

// let my_deltas: Vec<DeltaBigInt> = vec![];

// let deltas = Delta<BigInt>::new(my_deltas);



