use {
    crate::CharRange,
    core::char,
    rayon::{
        iter::plumbing::{Consumer, UnindexedConsumer},
        prelude::*,
    },
};

type CompactCharRangeIter = rayon::iter::Map<rayon::range_inclusive::Iter<u32>, fn(u32) -> char>;

/// A parallel iterator over a range of unicode code points.
#[derive(Clone, Debug)]
pub struct Iter {
    raw: rayon::iter::Chain<CompactCharRangeIter, CompactCharRangeIter>,
}

impl ParallelIterator for Iter {
    type Item = char;

    fn drive_unindexed<C>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.raw.drive_unindexed(consumer)
    }

    // override those default provided methods which `rayon::iter::Chain` does

    fn opt_len(&self) -> Option<usize> {
        self.raw.opt_len()
    }
}

impl IntoParallelIterator for CharRange {
    type Iter = Iter;
    type Item = char;

    #[allow(unsafe_code)]
    fn into_par_iter(self) -> Self::Iter {
        let (left, right) = self.split_range();
        Iter {
            raw: left
                .into_par_iter()
                .map((|c| unsafe { char::from_u32_unchecked(c) }) as fn(u32) -> char)
                .chain(
                    right
                        .into_par_iter()
                        .map((|c| unsafe { char::from_u32_unchecked(c) }) as fn(u32) -> char),
                ),
        }
    }
}

impl IntoParallelIterator for &CharRange {
    type Iter = Iter;
    type Item = char;

    fn into_par_iter(self) -> Self::Iter {
        (*self).into_par_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {
        crate::{AFTER_SURROGATE, BEFORE_SURROGATE},
        alloc::vec::Vec,
    };

    #[test]
    fn full_agrees() {
        let r = CharRange::from(..);
        assert_eq!(r.par_iter().count(), r.iter().count());
        assert_eq!(
            r.par_iter().collect::<Vec<_>>(),
            r.iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn surrogate_hug_agrees() {
        let r = CharRange::from(BEFORE_SURROGATE..=AFTER_SURROGATE);
        assert_eq!(r.par_iter().count(), r.iter().count());
        assert_eq!(
            r.par_iter().collect::<Vec<_>>(),
            r.iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn alphabet_agrees() {
        let r = CharRange::from('a'..='z');
        assert_eq!(r.par_iter().count(), r.iter().count());
        assert_eq!(
            r.par_iter().collect::<Vec<_>>(),
            r.iter().collect::<Vec<_>>(),
        );
    }
}
