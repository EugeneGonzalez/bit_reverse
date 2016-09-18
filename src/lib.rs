//! ## Library Objective
//! This library provides a number of ways to compute the bit reversal of all primitive integers.
//! There are currently 3 different algorithms implemented: Bitwise, Parallel, and Lookup reversal.
//!
//! ## YMMV Performance Comparison
//! Relatively bitwise is slower than parallel and lookup reversal. Parallel seems to be slightly
//! slower than lookup for small sized integers, but regaining a lead for larger sized integers.
//!
//! ## Memory Consumption
//! Bitwise uses the least amount of memory only using three integers to compute the reversal.
//! Parallel allocates 3 constants of the same size of the type being reversed.
//! Lookup allocates 256 u8s or 256 bytes to do its byte lookup reversal.
//!
//! ## How to use
//! This library is really simple to use just do the following:
//! 1. Import this crate
//! 2. Use the algorithm you want to use
//! 3. Call swap_bits()
//!
//! ## Example
//!
//! ```
//! use bit_reverse:: ParallelReverse;
//!
//! assert_eq!(0xA0u8.swap_bits(), 0x05u8);
//! ```


// This library abuse overflowing literals to be able to use macros to reduce duplicate code.
#![allow(overflowing_literals)]

// #![feature(test)]
// extern crate test;

/// Computes bit reversal by going bit by bit and setting the reverse position bit for the output.
pub trait BitwiseReverse<T> {
    /// Swaps the bits such that bit i is now bit N-i, where N is the length of the T in bits.
    fn swap_bits(self) -> T;
}

macro_rules! doit_bitwise { ($($ty:ty)*) => ($(
    impl BitwiseReverse<$ty> for $ty {
        // This algorithm uses the reverse variable as a like a stack to reverse the value.
        // The lesser significant bits are pushed onto the reverse variable and then the variable
        // is shifted down to make room for more significant bits. This algorithm has a shortcut,
        // that if there aren't anymore 1s to push onto the reverse variable the algorithm ends
        // early and shift the reverse to the correct position.
        fn swap_bits(self) -> $ty {
            let mut v = self;

            // By initializing the reversal to value, we have already loaded the largest
            // significant bit into the correct location.
            let mut r = self;

            // Compute how many bits are left to shift at the end of the algorithm.
            let mut s = 8 * std::mem::size_of::<$ty>() - 1;

            v >>= 1;
            while v != 0 {  // Quit early if there are no more 1s to shift in
                r <<= 1;    // Make room for the next significant bit
                r |= v & 1; // Add the bit to the reverse variable
                v >>= 1;    // Go to the next significant bit
                s -= 1;     // Decrement the leftover bit count
            }

            // Shift the reversal to the correct position and return the reversal
            return r << s;
        }
    })*)
}

doit_bitwise!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

/// Computes bit reversal by using a divide and conquer approach. Pairs of bits are swapped.
/// Then neighboring bit pairs are swapped. Each time swapping the next largest group of bits.
/// This is done until the entire data has been bit reversed.
pub trait ParallelReverse<T> {
    /// Swaps the bits such that bit i is now bit N-i, where N is the length of the T in bits.
    fn swap_bits(self) -> T;
}

macro_rules! doit_parallel { ($($ty:ty)*) => ($(
    impl ParallelReverse<$ty> for $ty {
        fn swap_bits(self) -> $ty {
            let mut v = self;
            // Swap odd and even bits
            v = ((v >> 1) & (0x5555555555555555 as $ty)) | ((v & (0x5555555555555555 as $ty)) << 1);
            // Swap consecutive pairs
            v = ((v >> 2) & (0x3333333333333333 as $ty)) | ((v & (0x3333333333333333 as $ty)) << 2);
            // Swap nibbles
            v = ((v >> 4) & (0x0F0F0F0F0F0F0F0F as $ty)) | ((v & (0x0F0F0F0F0F0F0F0F as $ty)) << 4);

            return v.swap_bytes();
        }
    })*)
}

doit_parallel!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

/// Computes bit reversal by using lookup table to translate a single byte into its reverse.
/// For multi-byte types, the byte order is swapped to complete the reversal.
pub trait LookupReverse<T> {
    /// Swaps the bits such that bit i is now bit N-i, where N is the length of the T in bits.
    fn swap_bits(self) -> T;
}

const REVERSE_LOOKUP: [u8; 256] =
    [0, 128, 64, 192, 32, 160, 96, 224, 16, 144, 80, 208, 48, 176, 112, 240, 8, 136, 72, 200, 40,
     168, 104, 232, 24, 152, 88, 216, 56, 184, 120, 248, 4, 132, 68, 196, 36, 164, 100, 228, 20,
     148, 84, 212, 52, 180, 116, 244, 12, 140, 76, 204, 44, 172, 108, 236, 28, 156, 92, 220, 60,
     188, 124, 252, 2, 130, 66, 194, 34, 162, 98, 226, 18, 146, 82, 210, 50, 178, 114, 242, 10,
     138, 74, 202, 42, 170, 106, 234, 26, 154, 90, 218, 58, 186, 122, 250, 6, 134, 70, 198, 38,
     166, 102, 230, 22, 150, 86, 214, 54, 182, 118, 246, 14, 142, 78, 206, 46, 174, 110, 238, 30,
     158, 94, 222, 62, 190, 126, 254, 1, 129, 65, 193, 33, 161, 97, 225, 17, 145, 81, 209, 49,
     177, 113, 241, 9, 137, 73, 201, 41, 169, 105, 233, 25, 153, 89, 217, 57, 185, 121, 249, 5,
     133, 69, 197, 37, 165, 101, 229, 21, 149, 85, 213, 53, 181, 117, 245, 13, 141, 77, 205, 45,
     173, 109, 237, 29, 157, 93, 221, 61, 189, 125, 253, 3, 131, 67, 195, 35, 163, 99, 227, 19,
     147, 83, 211, 51, 179, 115, 243, 11, 139, 75, 203, 43, 171, 107, 235, 27, 155, 91, 219, 59,
     187, 123, 251, 7, 135, 71, 199, 39, 167, 103, 231, 23, 151, 87, 215, 55, 183, 119, 247, 15,
     143, 79, 207, 47, 175, 111, 239, 31, 159, 95, 223, 63, 191, 127, 255];

impl LookupReverse<u8> for u8 {
    fn swap_bits(self) -> u8 {
        unsafe { *REVERSE_LOOKUP.get_unchecked(self as usize) }
    }
}

impl LookupReverse<u16> for u16 {
    fn swap_bits(self) -> u16 {
        unsafe {
            (*REVERSE_LOOKUP.get_unchecked(self as u8 as usize) as u16) << 8 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 8) as u8 as usize) as u16)
        }
    }
}

impl LookupReverse<u32> for u32 {
    fn swap_bits(self) -> u32 {
        unsafe {
            (*REVERSE_LOOKUP.get_unchecked(self as u8 as usize) as u32) << 24 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 8) as u8 as usize) as u32) << 16 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 16) as u8 as usize) as u32) << 8 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 24) as u8 as usize) as u32)
        }
    }
}

impl LookupReverse<u64> for u64 {
    fn swap_bits(self) -> u64 {
        unsafe {
            (*REVERSE_LOOKUP.get_unchecked(self as u8 as usize) as u64) << 56 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 8) as u8 as usize) as u64) << 48 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 16) as u8 as usize) as u64) << 40 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 24) as u8 as usize) as u64) << 32 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 32) as u8 as usize) as u64) << 24 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 40) as u8 as usize) as u64) << 16 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 48) as u8 as usize) as u64) << 8 |
            (*REVERSE_LOOKUP.get_unchecked((self >> 56) as u8 as usize) as u64)
        }
    }
}

impl LookupReverse<usize> for usize {
    #[cfg(target_pointer_width = "32")]
    fn swap_bits(self) -> usize {
        use LookupReverse;
        LookupReverse::swap_bits(self as u32) as usize
    }

    #[cfg(target_pointer_width = "64")]
    fn swap_bits(self) -> usize {
        LookupReverse::swap_bits(self as u64) as usize
    }
}

macro_rules! test_suite {
    ($name:ident, $algo:path) => (
        #[cfg(test)]
        mod $name {
            use $algo;

            #[test]
            fn reverse_u8() {
                assert_eq!(0xABu8.swap_bits(), 0xD5u8);
            }

            #[test]
            fn reverse_u16() {
                assert_eq!(0xABCDu16.swap_bits(), 0xB3D5u16);
            }

            #[test]
            fn reverse_u32() {
                assert_eq!(0xABCD2345u32.swap_bits(), 0xA2C4B3D5u32);
            }

            #[test]
            fn reverse_u64() {
                assert_eq!(0x0123456789ABCDEFu64.swap_bits(), 0xF7B3D591E6A2C480u64);
            }

            #[test]
            fn reverse_usize() {
                assert_eq!(0xFFusize.swap_bits(), 0xFFusize.swap_bytes());
            }
        }
    )
}

test_suite!(test_bitwise_reverse, BitwiseReverse);
test_suite!(test_parallel_reverse, ParallelReverse);
test_suite!(test_lookup_reverse, LookupReverse);

macro_rules! benchmark_suite {
    ($name:ident, $algo:path) => (
        #[cfg(test)]
        mod $name {
            use super::test;
            use $algo;

            #[bench]
            fn reverse_u8(b: &mut test::Bencher) {
                b.iter(|| {
                    let n = test::black_box(0xEDu8);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_u16(b: &mut test::Bencher) {
                b.iter(|| {
                    let n = test::black_box(0xABCDu16);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_u32(b: &mut test::Bencher) {
                b.iter(|| {
                    let n = test::black_box(0xABCD2345u32);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_u64(b: &mut test::Bencher) {
                b.iter(|| {
                    let n = test::black_box(0xFEDCBA9876543210u64);

                    n.swap_bits()
                });
            }
        }
    )
}

// Uncomment these lines and the test feature and crate at the top of the file to run the benchs.
// benchmark_suite!(benchmark_parallel_reverse, ParallelReverse);
// benchmark_suite!(benchmark_lookup_reverse, LookupReverse);
// benchmark_suite!(benchmark_bitwise_reverse, BitwiseReverse);
