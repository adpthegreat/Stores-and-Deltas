//! Contains a mock store for internal testing.
//!
//! Might make this public alter to users can test their store handlers.
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::{ops::{Add, AddAssign}, cmp::PartialOrd};
use substreams::{
    prelude::{BigInt, StoreDelete, StoreGet, StoreSet, StoreNew, StoreAppend, StoreMax, StoreMin, Appender, StoreSetIfNotExists, BigDecimal},
    store::{StoreAdd, StoreSetSum}
};
use std::marker::PhantomData;
use crate::mock_store::traits::*;

type BytesMockStore = HashMap<String, Vec<(u64, Vec<u8>)>>; // wait why is it a Vec of a tuple and not just a tuple 

#[derive(Debug, Clone)]
pub struct MockStore {
    data: Rc<RefCell<BytesMockStore>>,
}

impl StoreDelete for MockStore {
    fn delete_prefix(&self, _ord: i64, prefix: &String) {
        self.data
            .borrow_mut()
            .retain(|k, _| !k.starts_with(prefix));
    }
}

impl StoreNew for MockStore {
    fn new() -> Self {
        Self { data: Rc::new(RefCell::new(HashMap::new())) }
    }
}


impl <T: FromBytes> StoreGet<T> for MockStore {
    fn new(_idx: u32) -> Self {
        Self { data: Rc::new(RefCell::new(HashMap::new())) }
    }

    fn get_at<K: AsRef<str>>(&self, ord: u64, key: K) -> Option<T> {
        self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| {
                entries
                    .iter()
                    .find(|(current_ord, _)| *current_ord == ord)
                    .map(|(_, bytes)| get_value_from_bytes::<T>(bytes)) 
            })
    }

    fn get_last<K: AsRef<str>>(&self, key: K) -> Option<T> {
        self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| { // vec
                entries.last().map(|(_,bytes)| { // gets the last element of the slice (that is of &Vec<(u64,Vec<u8>)>), ignores the ord, thats the u64
                    get_value_from_bytes::<T>(bytes)
                })
            })
    }

    fn get_first<K: AsRef<str>>(&self, key: K) -> Option<T> {
            self.data
                .borrow()
                .get(&key.as_ref().to_string())
                .and_then(|entries| {
                    entries.first().map(|(_,bytes)| {
                        get_value_from_bytes::<T>(bytes)
                    })
                })
    }

    fn has_at<K: AsRef<str>>(&self, ord: u64, key: K) -> bool {
        self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .map(|v| v.iter().any(|(v, _)| *v == ord))
            .unwrap_or(false)
    } 

    fn has_last<K: AsRef<str>>(&self, key: K) -> bool {
        <MockStore as StoreGet<T>>::get_last::<K>(&self, key).is_some() // we specify the type explicitly because there is more than one trait bound
    }

    fn has_first<K: AsRef<str>>(&self, key: K) -> bool {
        <MockStore as StoreGet<T>>::get_first::<K>(&self, key).is_some()
    }
}



impl <T: ToString + ToBytes> StoreSet<T> for MockStore {  
    /// Set a given key to a given value, if the key existed before, it will be replaced.
    fn set<K: AsRef<str>>(&self, ord: u64, key: K, value: &T) {
        let mut guard = self.data.borrow_mut();
        guard
            .entry(key.as_ref().to_string())
            .or_insert(vec![(ord, convert_value_to_bytes(value))]); //how does it work without the generic type with starfish loll
    }
    
    /// Set many keys to a given value, if the key existed before, it will be replaced.
    fn set_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &T) {
        keys.iter().for_each(|key| self.set(ord, key, value));
    }
}


impl <T: ToString + ToBytes> StoreSetIfNotExists<T> for MockStore {
    fn set_if_not_exists<K: AsRef<str>>(&self, ord: u64, key: K, value: &T) {
        let mut guard = self.data.borrow_mut();

        if !guard.contains_key(key.as_ref()) {             
            guard
                .entry(key.as_ref().to_string())
                .or_insert(vec![(ord, convert_value_to_bytes(value))]); //how does it work without the generic type with starfish loll
        }
    }

    fn set_if_not_exists_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &T) {
        keys
            .iter()
            .for_each(|key| self.set_if_not_exists(ord, key, value)); // 
    }
}

//bigInt
//i64 
//f64
//BigDecimal for add min and max

//convert bytes to value
//add 
//convert back to bytes 
//store 
impl<T> StoreAdd<T> for MockStore 
where 
    T : FromBytes + ToBytes + Add + AddAssign + ToString + Clone { //try to reduce trait bounds here 
    // add a check for non negative values ?
    fn add<K: AsRef<str>>(&self, ord: u64, key: K, value: T) { // this 
        let mut guard = self.data.borrow_mut();
        guard
            .entry(key.as_ref().to_string())
            .and_modify(|v| {
                // first decode the value, add, then encode to bytes back
                let prev_value = v.last().unwrap().1.clone();
                let mut decoded_val = get_value_from_bytes::<T>(&prev_value);

                let val = format!("{:?}", decoded_val += value.clone()); // this might be wrong?

                let bytes_val = val.as_bytes(); 

                v.push((ord, bytes_val.to_vec())); 
            })
            .or_insert(vec![(ord, convert_value_to_bytes(&value))]);  //convert the T to a Vec<u8> 
    }

    fn add_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: T) {
        keys.iter().for_each(|key| self.add(ord, key, value.clone()));
    }
}


//means set_if_value is larger else don't 
  
/// max will set the provided key in the store only if the value received in
/// parameter is bigger than the one already present in the store, with
/// a default of the zero value when the key is absent.
impl <T> StoreMax<T> for MockStore
where 
    T : FromBytes + ToBytes + PartialOrd + ToString {
    fn max<K: AsRef<str>>(&self, ord: u64, key: K, value: T) {
        let mut guard = self.data.borrow_mut();
        let key_val = self.data // to abstract this in a macro or helper fn?
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| { // vec
                entries.last().map(|(_,bytes)| { // gets the last element of the slice (that is of &Vec<(u64,Vec<u8>)>), ignores the ord, thats the u64
                    get_value_from_bytes::<T>(bytes)
                })
            })
            .unwrap_or_else(|| panic!("cannot get_last value: key {} not found", key.as_ref()));
        //if hash_map does not contain the key, then insert 0 in its place
        if !guard.contains_key(key.as_ref()) {
             guard
                .entry(key.as_ref().to_string())
                .or_insert(vec![(ord, [0u8].to_vec())]); // is this correct
        }

        //if hash mpa contains the key and the new_value we want to insert is > 
        // our current value for that key then replace it with the new_value
        if guard.contains_key(key.as_ref()) && value > key_val {             
            guard
                .entry(key.as_ref().to_string())
                .or_insert(vec![(ord, convert_value_to_bytes(&value))]);
        } 
    }
}

/// Will set the provided key in the store only if the value received in
/// parameter is smaller than the one already present in the store, with
/// a default of the zero value when the key is absent.
impl <T> StoreMin<T> for MockStore
where 
    T : FromBytes + ToBytes + PartialOrd + ToString {
    fn min<K: AsRef<str>>(&self, ord: u64, key: K, value: T) {
        let mut guard = self.data.borrow_mut();
        let key_val = self.data 
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| { // vec
                entries.last().map(|(_,bytes)| { // gets the last element of the slice (that is of &Vec<(u64,Vec<u8>)>), ignores the ord, thats the u64
                    get_value_from_bytes::<T>(bytes)
                })
            })
            .unwrap_or_else(|| panic!("cannot get_last value: key {} not found", key.as_ref()));
            // .expect(&format!("failed to get last value for key {:?}", key)); /
        //if hash_map does not contain the key, then insert 0 in its place
        if !guard.contains_key(key.as_ref()) {
             guard
                .entry(key.as_ref().to_string())
                .or_insert(vec![(ord, [0u8].to_vec())]);
        }

        //if hash map contains the key and the new_value we want to insert is <
        // our current value for that key then replace it with the new_value
        if guard.contains_key(key.as_ref()) && value < key_val {             
            guard
                .entry(key.as_ref().to_string())
                .or_insert(vec![(ord, convert_value_to_bytes(&value))]);
        } 
    }
}


impl<T> Appender<T> for MockStore
where 
    T: Into<String>,
{  
    fn new() -> Self {
        MockStore {
            data: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    fn append<K: AsRef<str>>(&self, ord: u64, key: K, item: T) {
        let item_str: String = item.into();
        let mut formatted = format!("{};", &item_str).as_bytes().to_vec();
        let mut guard = self.data.borrow_mut();
        let entry = guard 
                     .entry(key.as_ref().to_string())
                     .and_modify(|existing| {
                        existing[0].0 = ord;
                        existing[0].1.append(&mut formatted)
                    })
                     .or_insert(vec![(ord, (formatted))]);
    }

    fn append_all<K: AsRef<str>>(&self, ord: u64, key: K, items: Vec<T>) {
         let key_str = key.as_ref().to_string();
        items.into_iter().for_each(|item| self.append(ord, &key_str, item)) // move to take ownership of item so we dont have a "From<&T> not implemented for String" trait bound error 
    }
}


//Proto trait impls 
//StoreSetProto
//StoreSetIfNotExistsProto

//we are using different Stores -> that is MockProtoStore instread of the MockStore so we don't have 
//conflicting trait implementations -> this should be obvious lol 

pub struct MockProtoStore<T> { 
    data: Rc<RefCell<BytesMockStore>>, 
    phantom: PhantomData<T> // do we need the <T> lol add the trait bound to MockProtoStore 
    //the extra trait bounds are not necessary for StoreNew and StoreDelete
    //okay i had to add it to impl this must_get_last method 
}
impl<T> StoreDelete for MockProtoStore<T> { } // no delete method for proto, but was impl so the trait bounds will be satisfied and stfu

impl<T> StoreNew for MockProtoStore<T> {
    fn new() -> Self {
        Self { 
            data: Rc::new(RefCell::new(HashMap::new())),
            phantom: PhantomData
        }
    }
}

impl <T: FromBytesProto> MockProtoStore<T> { 
    //honestly the only reason for the mockstore generic is because of this method loll
    //its also in the substreams code but we could have done without it 
     pub fn must_get_last<K: AsRef<str>>(&self, key: K) -> T {
        self.get_last(&key)
            .unwrap_or_else(|| panic!("cannot get_last value: key {} not found", key.as_ref()))
    }
}


 impl <T: FromBytesProto> StoreGet<T> for MockProtoStore<T> {
    fn new(_idx: u32) -> Self {
        Self { 
            data: Rc::new(RefCell::new(HashMap::new())),
            phantom: PhantomData
        }
    }

    fn get_at<K: AsRef<str>>(&self, ord: u64, key: K) -> Option<T> {
        self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| {
                entries
                    .iter()
                    .find(|(current_ord, _)| *current_ord == ord)
                    .map(|(_, bytes)| get_value_from_bytes_proto::<T>(bytes)) 
            })
    }

    fn get_last<K: AsRef<str>>(&self, key: K) -> Option<T> {
        self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| { // vec
                entries.last().map(|(_,bytes)| { // gets the last element of the slice (that is of &Vec<(u64,Vec<u8>)>), ignores the ord, thats the u64
                    get_value_from_bytes_proto::<T>(bytes)
                })
            })
    }

    fn get_first<K: AsRef<str>>(&self, key: K) -> Option<T> {
            self.data
                .borrow()
                .get(&key.as_ref().to_string())
                .and_then(|entries| {
                    entries.first().map(|(_,bytes)| {
                        get_value_from_bytes_proto::<T>(bytes)
                    })
                })
    }

    fn has_at<K: AsRef<str>>(&self, ord: u64, key: K) -> bool {
        self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .map(|v| v.iter().any(|(v, _)| *v == ord))
            .unwrap_or(false)
    } 

    fn has_last<K: AsRef<str>>(&self, key: K) -> bool {
        <MockProtoStore<T> as StoreGet<T>>::get_last::<K>(&self, key).is_some() // we specify the type explicitly because there is more than one trait bound
    }

    fn has_first<K: AsRef<str>>(&self, key: K) -> bool {
        <MockProtoStore<T> as StoreGet<T>>::get_first::<K>(&self, key).is_some()
    }
}



impl <T: ToBytesProto> StoreSet<T> for MockProtoStore<T>  {  
    /// Set a given key to a given value, if the key existed before, it will be replaced.
    fn set<K: AsRef<str>>(&self, ord: u64, key: K, value: &T) {
        let mut guard = self.data.borrow_mut();
        guard
            .entry(key.as_ref().to_string())
            .or_insert(vec![(ord, convert_value_to_bytes_proto(value))]); //::<T> ??
    }
    
    /// Set many keys to a given value, if the key existed before, it will be replaced.
    fn set_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &T) {
        keys.iter().for_each(|key| self.set(ord, key, value));
    }
}


impl <T: ToBytesProto> StoreSetIfNotExists<T> for MockProtoStore<T> {
    fn set_if_not_exists<K: AsRef<str>>(&self, ord: u64, key: K, value: &T) {
        let mut guard = self.data.borrow_mut();

        if !guard.contains_key(key.as_ref()) {             
            guard
                .entry(key.as_ref().to_string())
                .or_insert(vec![(ord, convert_value_to_bytes_proto(value))]);  //::<T> ??
        }
    }

    fn set_if_not_exists_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &T) {
        keys
            .iter()
            .for_each(|key| self.set_if_not_exists(ord, key, value)); // 
    }
}


//StoreGetArray

pub struct MockArrayStore {
    data: Rc<RefCell<BytesMockStore>>, 
}

impl<T: Into<String> + From<String>> StoreGet<Vec<T>> for MockArrayStore {
    fn new(idx: u32) -> Self {
        Self {
            data: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    fn get_at<K: AsRef<str>>(&self, ord: u64, key: K) -> Option<Vec<T>> {
         self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| {
                entries
                    .iter()
                    .find(|(current_ord, _)| *current_ord == ord)
                    .map(|(_, bytes)| split_array(bytes.clone()).unwrap_or_else(|| panic!("failed to split array")))
            })
    }

    fn get_last<K: AsRef<str>>(&self, key: K) -> Option<Vec<T>> {
         self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .and_then(|entries| { // vec
                entries.last().map(|(_,bytes)| { // gets the last element of the slice (that is of &Vec<(u64,Vec<u8>)>), ignores the ord, thats the u64
                    split_array(bytes.clone()).unwrap_or_else(|| panic!("failed to split array"))
                })
            })
    }

    fn get_first<K: AsRef<str>>(&self, key: K) -> Option<Vec<T>> {
         self.data
                .borrow()
                .get(&key.as_ref().to_string())
                .and_then(|entries| {
                    entries.first().map(|(_,bytes)| {
                        split_array(bytes.clone()).unwrap_or_else(|| panic!("failed to split array"))
                    })
                })
    }

    fn has_at<K: AsRef<str>>(&self, ord: u64, key: K) -> bool {
          self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .map(|v| v.iter().any(|(v, _)| *v == ord))
            .unwrap_or(false)
    }

    fn has_last<K: AsRef<str>>(&self, key: K) -> bool {
        <MockArrayStore as StoreGet<Vec<T>>>::get_last::<K>(&self, key).is_some() 
    }

    fn has_first<K: AsRef<str>>(&self, key: K) -> bool {
        <MockArrayStore as StoreGet<Vec<T>>>::get_last::<K>(&self, key).is_some() 
    }
}

fn split_array<T: Into<String> + From<String>>(bytes: Vec<u8>) -> Option<Vec<T>> {
    let chunks: Vec<_> = bytes
        .split(|b|*b == b';') // split slice by semicolon
        .filter(|x| x.len() > 0)
        .map(|part| {
            String::from_utf8(part.into())
                .unwrap_or_else(|_| panic!("Invalid UTF-8 sequence in store value"))
                .into()
        })
        .collect();

    match chunks.len() {
        0 => None,
        _ => Some(chunks),
    }
}











//something like this https://github.com/streamingfast/substreams-rs/blob/995a9bfcc15ebd59df63bdb2ce1b5d095d189d06/substreams-macro/src/store.rs

// https://github.com/streamingfast/substreams-rs/blob/995a9bfcc15ebd59df63bdb2ce1b5d095d189d06/substreams-macro/src/handler.rs

