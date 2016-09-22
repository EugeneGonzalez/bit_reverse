#![allow(overflowing_literals)]
#![feature(test)]

extern crate bit_reverse;
extern crate test;

macro_rules! benchmark_suite {
    ($name:ident, $algo:ident) => (
        #[cfg(test)]
        mod $name {
            use bit_reverse::$algo;
            use super::test::{Bencher, black_box};

            #[bench]
            fn reverse_u8(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xEDu8);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_u16(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xABCDu16);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_u32(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xABCD2345u32);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_u64(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xFEDCBA9876543210u64);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_usize(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xFFusize);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_i8(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xEDi8);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_i16(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xABCDi16);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_i32(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xABCD2345i32);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_i64(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xFEDCBA9876543210i64);

                    n.swap_bits()
                });
            }

            #[bench]
            fn reverse_isize(b: &mut Bencher) {
                b.iter(|| {
                    let n = black_box(0xFFisize);

                    n.swap_bits()
                });
            }
        }
    )
}

benchmark_suite!(benchmark_parallel_reverse, ParallelReverse);
benchmark_suite!(benchmark_lookup_reverse, LookupReverse);
benchmark_suite!(benchmark_bitwise_reverse, BitwiseReverse);