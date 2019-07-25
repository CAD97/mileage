#![allow(clippy::type_complexity)]

use {
    char_range::{range::Iter, CharRange},
    core::{char, iter::*, ops::RangeInclusive},
    criterion::{
        black_box, criterion_group, criterion_main, BatchSize, Bencher, Benchmark, Criterion,
    },
};

pub(crate) const BEFORE_SURROGATE: char = '\u{D7FF}';
pub(crate) const AFTER_SURROGATE: char = '\u{E000}';

fn chain_segments(
) -> Chain<Map<RangeInclusive<u32>, fn(u32) -> char>, Map<RangeInclusive<u32>, fn(u32) -> char>> {
    let left = 0..=(BEFORE_SURROGATE as u32);
    let right = (AFTER_SURROGATE as u32)..=(char::MAX as u32);
    left.map((|c| unsafe { char::from_u32_unchecked(c) }) as fn(u32) -> char)
        .chain(right.map((|c| unsafe { char::from_u32_unchecked(c) }) as fn(u32) -> char))
}

fn chain_segments_() -> impl Iterator<Item = char> + Clone {
    let left = 0..=(BEFORE_SURROGATE as u32);
    let right = (AFTER_SURROGATE as u32)..=(char::MAX as u32);
    left.map(|c| unsafe { char::from_u32_unchecked(c) })
        .chain(right.map(|c| unsafe { char::from_u32_unchecked(c) }))
}

fn try_from() -> FilterMap<RangeInclusive<u32>, fn(u32) -> Option<char>> {
    let range = 0..=(char::MAX as u32);
    range.filter_map((|c| char::from_u32(c)) as fn(u32) -> Option<char>)
}

fn try_from_() -> impl Iterator<Item = char> + Clone {
    let range = 0..=(char::MAX as u32);
    range.filter_map(char::from_u32)
}

fn custom() -> impl Iterator<Item = char> + Clone {
    #[derive(Clone, Debug)]
    struct Iter {
        low: char,
        high: char,
    }

    impl Iter {
        #[inline]
        fn step_forward(&mut self) {
            if self.low == char::MAX {
                self.high = '\0';
            } else {
                self.low = if self.low == BEFORE_SURROGATE {
                    AFTER_SURROGATE
                } else {
                    #[allow(unsafe_code)]
                    unsafe {
                        char::from_u32_unchecked(self.low as u32 + 1)
                    }
                }
            }
        }

        #[inline]
        fn is_empty(&self) -> bool {
            self.low > self.high
        }
    }

    impl Iterator for Iter {
        type Item = char;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            // Avoid unstable `<&mut I as ExactSizeIterator>::is_empty`
            if (&*self).is_empty() {
                return None;
            }

            let c = self.low;
            self.step_forward();
            Some(c)
        }
    }

    Iter {
        low: '\0',
        high: char::MAX,
    }
}

fn actual() -> Iter {
    CharRange::from(..).iter()
}

fn black_hole<T>(t: T) {
    black_box(t);
}

fn bench_ranges(c: &mut Criterion) {
    fn bench(b: &mut Bencher, r: impl Iterator<Item = char> + Clone) {
        b.iter_batched(
            || r.clone(),
            |r| black_box(r).for_each(black_hole),
            BatchSize::SmallInput,
        )
    }

    c.bench(
        "CharIter",
        Benchmark::new("chain_segments (fn())", |b| bench(b, chain_segments()))
            .with_function("chain_segments ([closure])", |b| {
                bench(b, chain_segments_())
            })
            .with_function("try_from (fn())", |b| bench(b, try_from()))
            .with_function("try_from ([closure])", |b| bench(b, try_from_()))
            .with_function("custom", |b| bench(b, custom()))
            .with_function("actual", |b| bench(b, actual())),
    );
}

criterion_group!(benches, bench_ranges);
criterion_main!(benches);
