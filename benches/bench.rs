#![feature(test)]

extern crate bit_reverse;
extern crate test;

macro_rules! benchmark_suite {
    ($name:ident, $algo:ident) => (
        #[cfg(test)]
        mod $name {
            use super::test;
            use bit_reverse::$algo;

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

benchmark_suite!(benchmark_parallel_reverse, ParallelReverse);
benchmark_suite!(benchmark_lookup_reverse, LookupReverse);
benchmark_suite!(benchmark_bitwise_reverse, BitwiseReverse);