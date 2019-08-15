#![allow(clippy::type_complexity)]

use criterion::BatchSize;
use {
    core::{char, ops::Range},
    criterion::{black_box, criterion_group, criterion_main, Bencher, Benchmark, Criterion},
    mileage::CharRange,
    rayon::{iter::*, prelude::IntoParallelIterator},
};

pub(crate) const BEFORE_SURROGATE: char = '\u{D7FF}';
pub(crate) const AFTER_SURROGATE: char = '\u{E000}';
pub(crate) const SURROGATE_RANGE: Range<u32> =
    (BEFORE_SURROGATE as u32 + 1)..(AFTER_SURROGATE as u32);

fn chain_segments() -> Chain<
    Map<rayon::range_inclusive::Iter<u32>, fn(u32) -> char>,
    Map<rayon::range_inclusive::Iter<u32>, fn(u32) -> char>,
> {
    let left = 0..=(BEFORE_SURROGATE as u32);
    let right = (AFTER_SURROGATE as u32)..=(char::MAX as u32);
    left.into_par_iter()
        .map((|c| unsafe { char::from_u32_unchecked(c) }) as fn(u32) -> char)
        .chain(
            right
                .into_par_iter()
                .map((|c| unsafe { char::from_u32_unchecked(c) }) as fn(u32) -> char),
        )
}

fn chain_segments_() -> impl ParallelIterator<Item = char> + Clone {
    let left = 0..=(BEFORE_SURROGATE as u32);
    let right = (AFTER_SURROGATE as u32)..=(char::MAX as u32);
    left.into_par_iter()
        .map(|c| unsafe { char::from_u32_unchecked(c) })
        .chain(
            right
                .into_par_iter()
                .map(|c| unsafe { char::from_u32_unchecked(c) }),
        )
}

fn try_from() -> FilterMap<rayon::range_inclusive::Iter<u32>, fn(u32) -> Option<char>> {
    let range = 0..=(char::MAX as u32);
    range
        .into_par_iter()
        .filter_map((|c| char::from_u32(c)) as fn(u32) -> Option<char>)
}

fn try_from_() -> impl ParallelIterator<Item = char> + Clone {
    let range = 0..=(char::MAX as u32);
    range.into_par_iter().filter_map(char::from_u32)
}

fn decompress() -> Map<rayon::range_inclusive::Iter<u32>, fn(u32) -> char> {
    let range = 0..=(char::MAX as u32 - SURROGATE_RANGE.len() as u32);
    range.into_par_iter().map(
        (|c| unsafe {
            if c < SURROGATE_RANGE.start {
                char::from_u32_unchecked(c)
            } else {
                char::from_u32_unchecked(c + SURROGATE_RANGE.len() as u32)
            }
        }) as fn(u32) -> char,
    )
}

fn decompress_() -> impl ParallelIterator<Item = char> + Clone {
    let range = 0..=(char::MAX as u32 - SURROGATE_RANGE.len() as u32);
    range.into_par_iter().map(|c| unsafe {
        if c < SURROGATE_RANGE.start {
            char::from_u32_unchecked(c)
        } else {
            char::from_u32_unchecked(c + SURROGATE_RANGE.len() as u32)
        }
    })
}

fn actual() -> impl ParallelIterator<Item = char> + Clone {
    CharRange::from(..).par_iter()
}

fn black_hole<T>(t: T) {
    black_box(t);
}

fn bench_ranges(c: &mut Criterion) {
    fn bench(b: &mut Bencher, r: impl ParallelIterator<Item = char> + Clone) {
        b.iter_batched(
            || r.clone(),
            |r| black_box(r).for_each(black_hole),
            BatchSize::SmallInput,
        )
    }

    c.bench(
        "CharParIter",
        Benchmark::new("chain_segments (fn())", |b| bench(b, chain_segments()))
            .with_function("chain_segments ([closure])", |b| {
                bench(b, chain_segments_())
            })
            .with_function("try_from (fn())", |b| bench(b, try_from()))
            .with_function("try_from ([closure])", |b| bench(b, try_from_()))
            .with_function("decompress (fn())", |b| bench(b, decompress()))
            .with_function("decompress ([closure])", |b| bench(b, decompress_()))
            .with_function("actual", |b| bench(b, actual())),
    );
}

criterion_group!(benches, bench_ranges);
criterion_main!(benches);
