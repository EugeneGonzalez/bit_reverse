#![feature(test)]

extern crate bit_reverse;
extern crate test;

macro_rules! benchmark_suite {
    ($name:ident, $algo:ident) => (
        #[cfg(test)]
        mod $name {
            use bit_reverse::$algo;
            use super::test::Bencher;

            #[bench]
            fn reverse_u8(b: &mut Bencher) {
                b.iter(|| {
                    0xEDu8.swap_bits()
                });
            }

            #[bench]
            fn reverse_u16(b: &mut Bencher) {
                b.iter(|| {
                    0xABCDu16.swap_bits()
                });
            }

            #[bench]
            fn reverse_u32(b: &mut Bencher) {
                b.iter(|| {
                    0xABCD2345u32.swap_bits()
                });
            }

            #[bench]
            fn reverse_u64(b: &mut Bencher) {
                b.iter(|| {
                    0xFEDCBA9876543210u64.swap_bits()
                });
            }
        }
    )
}

benchmark_suite!(benchmark_parallel_reverse, ParallelReverse);
benchmark_suite!(benchmark_lookup_reverse, LookupReverse);
benchmark_suite!(benchmark_bitwise_reverse, BitwiseReverse);