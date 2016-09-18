# bit_reverse

### Library Objective
This library provides a number of ways to compute the bit reversal of all primitive integers.
There are currently 3 different algorithms implemented: Bitwise, Parallel, and Lookup reversal.

### YMMV Performance Comparison
Relatively bitwise is slower than parallel and lookup reversal. Parallel seems to be slightly
slower than lookup for small sized integers, but regaining a lead for larger sized integers.

### Memory Consumption
Bitwise uses the least amount of memory only using three integers to compute the reversal.
Parallel allocates 3 constants of the same size of the type being reversed.
Lookup allocates 256 u8s or 256 bytes to do its byte lookup reversal.

### How to use
This library is really simple to use just do the following:
1. Import this crate
2. Use the algorithm you want to use
3. Call swap_bits()

### Example

```rust
use bit_reverse:: ParallelReverse;

assert_eq!(0xA0u8.swap_bits(), 0x05u8);
```