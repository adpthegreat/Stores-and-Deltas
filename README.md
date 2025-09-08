

## Overview
this repo contains a near canonical implementation of the stores and deltas in the substreams pipeline

in the stores we are simply using a `HashMap<String, Vec<(u64, Vec<u8>)>>` to map key value pairs and for the deltas we are 

using ___________


## Closing Remarks 

In the substreams pacakge looking at them implementing all the `StoreSet` and `StoreGet` traits individually looked repititive and verbose, 
so i decided to use macros to reduce the amound of code, but this involves checking the type passed in and serializing and deserializing byte arrays `Vec<u8>` to their right form , maybe eliminating this step was the tradeoff for verbosity that substreams devs gladly took , because if they followed this approach they would be doing that twice - serializing and deserializing at the macro trait generation level and then doing byte conversion and memory allocation at the `state` level, then the wasmbindings executed , anyways its still a fun experiment which can surely be improved 

https://docs.substreams.dev/reference-material/substreams-components/modules/keys-in-stores


Milestones
- https://github.com/streamingfast/substreams/tree/707f0b506c95bff9dc78c25aba23d8745b96e143/storage/store 
- recreate the golang code bindings 
and the substreams-rs wasm integration just to understand wasm and how it works better 

- for the rust impl with hashmaps, use wasm-bindgen to make a typescript wasm binding with rust 

- these are not open issues, not accepting PRs currently


```md 

Phantom data suggestions when you create generic type for struct but dont use it 
error[E0063]: missing field `phantom` in initializer of `MockProtoStore<{type error}>`
   --> src/mock_store/store.rs:282:9
    |
282 |         Self { data: Rc::new(RefCell::new(HashMap::new())) }
    |         ^^^^ missing `phantom`

error[E0063]: missing field `phantom` in initializer of `MockProtoStore<T>`
   --> src/mock_store/store.rs:298:9
    |
298 |         Self { data: Rc::new(RefCell::new(HashMap::new())) }
    |         ^^^^ missing `phantom
```


```
Wanted to use just Cursor in the `StoreGetArray` like the canonical implementation, but its unstable and i need nightly lol 
https://github.com/rust-lang/rust/issues/86369
```