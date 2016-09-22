# bit_reverse
[![Crates Shield](https://img.shields.io/crates/v/bit_reverse.svg "Crates.io")](https://crates.io/crates/bit_reverse) [![Build Shield](https://travis-ci.org/EugeneGonzalez/bit_reverse.svg?branch=master "TravisCI")](https://travis-ci.org/EugeneGonzalez/bit_reverse)

### Library Objective
This library provides a number of ways to compute the bit reversal of all primitive integers.
There are currently 3 different algorithms implemented: Bitwise, Parallel, and Lookup reversal.

### Example
```rust
use bit_reverse::ParallelReverse;

assert_eq!(0xA0u8.swap_bits(), 0x05u8);
```
This library is very simple to uses just import the crate and the algorithm you want to use.
Then you can call swap_bits() on any primitive integer. If you want to try a different
algorithm just change the use statement and now your program will use the algorithm instead.

### YMMV Performance Comparison
I wouldn't use BitwiseReverse as it is mainly there for completeness and is strictly inferior
to ParallelReverse, which is a Bitwise Parallel Reverse and thus an order of magnitude faster.
I would recommend you use the ParallelReverse as it performs equal to or better than
LookupReverse for all types. Plus as an added bonus doesn't eat your cache with a lookup table.

### Memory Consumption
Bitwise uses the least amount of memory only using three integers to compute the reversal.
Parallel allocates 3 constants of the same size of the type being reversed.
Lookup allocates 256 u8s or 256 bytes to do its byte lookup reversal.

### no_std Compatible
To link to core instead of STD, disable default features for this library in your Cargo.toml.
[Cargo choosing features](http://doc.crates.io/specifying-dependencies.html#choosing-features)
