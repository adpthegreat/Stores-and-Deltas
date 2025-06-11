

## Overview
this repo contains a near canonical implementation of the stores and deltas in the substreams pipeline

in the stores we are simply using a `HashMap<String, Vec<(u64, Vec<u8>)>>` to map key value pairs and for the deltas we are 

using ___________


## Closing Remarks 

In the substreams pacakge looking at them implementing all the `StoreSet` and `StoreGet` traits individually looked repititive and verbose, 
so i decided to use macros to reduce the amound of code, but this involves checking the type passed in and serializing and deserializing byte arrays `Vec<u8>` to their right form , maybe eliminating this step was the tradeoff for verbosity that substreams devs gladly took , because if they followed this approach they would be doing that twice - serializing and deserializing at the macro trait generation level and then doing byte conversion and memory allocation at the `state` level, then the wasmbindings executed , anyways its still a fun experiment which can surely be improved 

