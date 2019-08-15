use {
    crate::{CharRange, CharSet},
    rayon::{
        iter::plumbing::{Consumer, UnindexedConsumer},
        prelude::*,
    },
};

/// A parallel iterator over a set of unicode code points.
#[derive(Clone, Debug)]
pub struct Iter<'a> {
    raw: rayon::iter::Flatten<rayon::slice::Iter<'a, CharRange>>,
}

impl ParallelIterator for Iter<'_> {
    type Item = char;

    fn drive_unindexed<C>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.raw.drive_unindexed(consumer)
    }

    // Flatten doesn't override any default provided methods
}

impl<'a> IntoParallelIterator for &'a CharSet {
    type Iter = Iter<'a>;
    type Item = char;

    fn into_par_iter(self) -> Iter<'a> {
        Iter { raw: self.ranges.par_iter().flatten() }
    }
}
