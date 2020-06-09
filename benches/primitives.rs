#![allow(overflowing_literals)]

extern crate bit_reverse;
extern crate criterion;

use bit_reverse::{BitwiseReverse, LookupReverse, ParallelReverse};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

static SEED: u64 = 0x0123456789ABCDEF;

fn bench_reverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("u8_reverse");
    let i = SEED as u8;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("i8_reverse");
    let i = SEED as i8;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("u16_reverse");
    let i = SEED as u16;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("i16_reverse");
    let i = SEED as i16;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("u32_reverse");
    let i = SEED as u32;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("i32_reverse");
    let i = SEED as i32;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("u64_reverse");
    let i = SEED as u64;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("i64_reverse");
    let i = SEED as i64;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    #[cfg(feature = "u128")]
    {
        let mut group = c.benchmark_group("u128_reverse");
        let i = SEED as u128;
        group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
            b.iter(|| BitwiseReverse::swap_bits(*i))
        });
        group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
            b.iter(|| LookupReverse::swap_bits(*i))
        });
        group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
            b.iter(|| ParallelReverse::swap_bits(*i))
        });
        group.finish();

        let mut group = c.benchmark_group("i128_reverse");
        let i = SEED as i128;
        group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
            b.iter(|| BitwiseReverse::swap_bits(*i))
        });
        group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
            b.iter(|| LookupReverse::swap_bits(*i))
        });
        group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
            b.iter(|| ParallelReverse::swap_bits(*i))
        });
        group.finish();
    }

    let mut group = c.benchmark_group("usize_reverse");
    let i = SEED as usize;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();

    let mut group = c.benchmark_group("isize_reverse");
    let i = SEED as isize;
    group.bench_with_input(BenchmarkId::new("bitwise", i), &i, |b, i| {
        b.iter(|| BitwiseReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("lookup", i), &i, |b, i| {
        b.iter(|| LookupReverse::swap_bits(*i))
    });
    group.bench_with_input(BenchmarkId::new("parallel", i), &i, |b, i| {
        b.iter(|| ParallelReverse::swap_bits(*i))
    });
    group.finish();
}

criterion_group!(benches, bench_reverse);
criterion_main!(benches);
