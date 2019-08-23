use {
    crate::{map::CharMapRef, set, CharRange},
    core::{iter, slice::Iter as SliceIter},
};

pub use set::RangeIter;
use core::iter::FusedIterator;

#[derive(Clone, Debug)]
pub struct RangeValueIter<'a, T> {
    pub(crate) raw: iter::Zip<RangeIter<'a>, SliceIter<'a, T>>,
}

impl<'a, T> IntoIterator for CharMapRef<'a, T> {
    type Item = (CharRange, &'a T);
    type IntoIter = RangeValueIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.range_values()
    }
}

impl<'a, T> CharMapRef<'a, T> {
    /// Iterate the compact ranges of this mapping.
    pub fn ranges(self) -> RangeIter<'a> {
        RangeIter {
            raw: self.ranges.iter(),
        }
    }

    /// Iterate the codepoints of this mapping.
    pub fn chars(self) -> impl Iterator<Item = char> + 'a {
        self.ranges().flat_map(IntoIterator::into_iter)
    }

    /// Iterate the range-value mappings of this mapping.
    pub fn range_values(self) -> RangeValueIter<'a, T> {
        RangeValueIter {
            raw: self.ranges().zip(self.values.iter())
        }
    }
}

// forward zip iterators

impl<'a, T> Iterator for RangeValueIter<'a, T> {
    type Item = (CharRange, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth(n)
    }
}

impl<'a, T> DoubleEndedIterator for RangeValueIter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back()
    }
}

impl<'a, T> ExactSizeIterator for RangeValueIter<'a, T> {}

impl<'a, T> FusedIterator for RangeValueIter<'a, T> {}
