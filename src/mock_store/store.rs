//! Contains a mock store for internal testing.
//!
//! Might make this public alter to users can test their store handlers.
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use substreams::{
    prelude::{BigInt, StoreDelete, StoreGet, StoreSet, StoreNew, StoreAppend, StoreMax, StoreMin, Appender, StoreSetIfNotExists},
    store::{StoreAdd, StoreSetSum, StoreGetString}
};

type MockStore = HashMap<String, Vec<(u64, Vec<u8>)>>; 

#[derive(Debug, Clone)]
pub struct BytesMockStore {
    data: Rc<RefCell<MockStore>>,
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

// so now because we have to construct either a f64, i64 , BigDecimal, BigInt, String from bytes,
// we have to use a quote macro with conditionals that match the type and do the proper conversion 

// no we don't need a quote macro lol we can do it directly here 

macro_rules! get_value_from_bytes {
    ($bytes:expr, $t:ty) => {{
        match std::any::type_name::<$t>() {
            "alloc::string::String" | "std::string::String" => {
                String::from_utf8($bytes).unwrap() as $t
            },
            "num_bigint::BigInt" => {
                let s = String::from_utf8($bytes).unwrap();
                BigInt::parse_bytes(s.as_bytes(), 10).unwrap() as $t
            },
            "bigdecimal::BigDecimal" => {
                let s = String::from_utf8($bytes).unwrap();
                BigDecimal::parse_bytes(s.as_bytes(), 10).unwrap() as $t
            },
            "f64" => {
                let s = String::from_utf8($bytes).unwrap();
                s.parse::<f64>().unwrap() as $t
            },
            "i64" => {
                let s = String::from_utf8($bytes).unwrap();
                s.parse::<i64>().unwrap() as $t
            },
            _ => todo!("Conversion for this type is not implemented"),
        }
    }};
}

impl StoreGet<V> for MockStore {
    fn new(_idx: u32) -> Self {
        Self { data: Rc::new(RefCell::new(HashMap::new())) }
    }

    fn get_at<K: AsRef<str>>(&self, ord: u64, key: K) -> Option<V> {
        let bytes = self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .map(|v| {
                v.iter()
                    .find(|(current_ord, _)| *current_ord == ord)
                    .unwrap()
                    .1
                    .clone()
            });
        let res = get_value_from_bytes!(bytes);
        res
    }

    fn get_last<K: AsRef<str>>(&self, key: K) -> Option<V> {
        let bytes = self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .map(|v| v.last().unwrap().1.clone());
        let res = get_value_from_bytes!(bytes);
        res
    }

    fn get_first<K: AsRef<str>>(&self, key: K) -> Option<V> {
        let bytes = self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .map(|v| v.first().unwrap().1.clone());
        let res = get_value_from_bytes!(bytes);
        res
    }

    fn has_at<K: AsRef<str>>(&self, ord: u64, key: K) -> bool {
        self.data
            .borrow()
            .get(&key.as_ref().to_string())
            .map(|v| v.iter().any(|(v, _)| *v == ord))
            .unwrap_or(false)
    }

    fn has_last<K: AsRef<str>>(&self, key: K) -> bool {
        self.get_last(key).is_some()
    }

    fn has_first<K: AsRef<str>>(&self, key: K) -> bool {
        self.get_first(key).is_some()
    }
}



/// lets make V impl AsRef<str> for more flexibility instead of putting Stirng
impl<V: AsRef<str>> StoreSet<V> for MockStore {
    fn set<K: AsRef<str>>(&self, ord: u64, key: K, value: &V) {
          let mut guard = self.data.borrow_mut();
          guard
            .entry(key.as_ref().to_string())
            .or_insert(vec![(ord, value.as_ref())]);
    }

    fn set_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &V) {
        let value = value.as_ref();
        keys.iter().for_each(|key| self.set(ord, key, value));
    }
}

macro_rules! impl_store_set {
    ($type:ty) => {
        impl StoreSet<$name> for MockStore {  
            /// Set a given key to a given value, if the key existed before, it will be replaced.
            fn set<K: AsRef<str>>(&self, ord: u64, key: K, value:&$type) {
                let value = value.to_string().as_bytes();
                let mut guard = self.data.borrow_mut();
                guard
                .entry(key.as_ref().to_string())
                .or_insert(vec![(ord, value)]);
            }
            
            /// Set many keys to a given value, if the key existed before, it will be replaced.
            fn set_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &$type) {
                keys.iter().for_each(|key| self.set(ord, key, value));
            }
        }
    }
}

impl_store_set!(String);
impl_store_set!(i64);
impl_store_set!(f64);
impl_store_set!(BigInt);
impl_store_set!(BigDecimal);


macro_rules! impl_store_set_sum {
    ($type:ty) => {
        impl StoreSetSum<V> for MockStore {
            fn set<K: AsRef<str>>(&self, ord: u64, key: K, value: &$type) {
                let mut guard = self.data.borrow_mut();
                //have to put a format string inside it too 
                guard
                    .entry(key.as_ref().to_string())
                    .or_insert(vec![(ord, value.clone())]);
            }
            fn sum<K: AsRef<str>>(&self, ord: u64, key: K, value: &$type) {
                let mut guard = self.data.borrow_mut(); 
                guard
                    .entry(key.as_ref().to_string())
                    .and_modify(|v| {
                        let prev_value = v.last().unwrap().1.clone();
                        v.push((ord, prev_value + value.clone()));
                    })
                    .or_insert(vec![(ord, v)]);
            }
       }
    }
}


impl_store_set_sum!(i64);
impl_store_set_sum!(f64);
impl_store_set_sum!(BigInt);
impl_store_set_sum!(BigDecimal);

macro_rules! impl_store_set_if_not_exists {
    ($type:ty) => {
        impl StoreSetIfNotExists<V> for MockStore {
            fn set_if_not_exists<K: AsRef<str>>(&self, ord: u64, key: K, value: &$type) {
                let value = value.as_ref();
                let mut guard = self.data.borrow_mut();

                if !guard.contains_key(key.as_ref()) {             
                    guard
                    .entry(key)
                    .or_insert(vec![(ord, value.)]);
                }
            }

            fn set_if_not_exists_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &$type) {
                let value = value.as_ref();
                keys
                    .iter()
                    .for_each(|key| self.set_if_not_exists(ord, key, value));
            }
        }
    }
}

impl_store_set_if_not_exists!(i64);
impl_store_set_if_not_exists!(f64);
impl_store_set_if_not_exists!(BigInt);
impl_store_set_if_not_exists!(BigDecimal);


macro_rules! impl_store_add {
    ($type:ty) => {
        impl StoreAdd<$type> for MockStore {
            fn add<K: AsRef<str>>(&self, ord: u64, key: K, value: &$type) { // this 
                let mut guard = self.data.borrow_mut();
                guard
                    .entry(key.as_ref().to_string())
                    .and_modify(|v| {
                        let prev_value = v.last().unwrap().1.clone();
                        v.push((ord, prev_value + value.clone()));
                    })
                    .or_insert(vec![(ord, value)]);
            }

            fn add_many<K: AsRef<str>>(&self, ord: u64, keys: &Vec<K>, value: &$type) {
                keys.iter().for_each(|key| self.add(ord, key, value.clone()));
            }
        }
    }
}

impl_store_add!(i64);
impl_store_add!(f64);
impl_store_add!(BigInt);
impl_store_add!(BigDecimal);

macro_rules! impl_store_max {
    ($type:ty) => {
       impl StoreMax<$type> for MockStore {
            fn max<K: AsRef<str>>(&self, ord: u64, key: K, value: &$type) {
               todo!()
            }
        }
    }
}

impl_store_max!(i64);
impl_store_max!(f64);
impl_store_max!(BigInt);
impl_store_max!(BigDecimal);

macro_rules! impl_store_min {
    ($type:ty) => {
       impl StoreMin<$type> for MockStore {
            fn min<K: AsRef<str>>(&self, ord: u64, key: K, value: &$type) {
                todo!()
            }
        }
    }
}

impl_store_min!(i64);
impl_store_min!(f64);
impl_store_min!(BigInt);
impl_store_min!(BigDecimal);


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
        let formatted = &format!("{};", &item_str).as_bytes();
        let mut guard = self.data.borrow_mut();
        let entry = guard 
                     .entry(key.as_ref().to_string())
                     .and_modify(|existing| existing.push_str(&formatted))
                     .or_insert(&formatted);
    }

    fn append_all<K: AsRef<str>>(&self, ord: u64, key: K, items: Vec<T>) {
        items.iter().cloned().for_each(|item| self.append(ord, key, item))
    }
}

//Protos
//Deltas -> to be implemented, from https://github.com/streamingfast/substreams-rs/blob/995a9bfcc15ebd59df63bdb2ce1b5d095d189d06/substreams/src/store.rs#L1241






// just for experiments, they aren't really much to implement

//something like this https://github.com/streamingfast/substreams-rs/blob/995a9bfcc15ebd59df63bdb2ce1b5d095d189d06/substreams-macro/src/store.rs

// https://github.com/streamingfast/substreams-rs/blob/995a9bfcc15ebd59df63bdb2ce1b5d095d189d06/substreams-macro/src/handler.rs