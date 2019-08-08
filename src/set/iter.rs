use {
    crate::{CharRange, CharSet},
    core::{iter, slice::Iter as SliceIter},
};

/// An iterator over ranges of `char`.
///
/// Constructed via `CharSet::ranges`. See `CharSet` for more information.
#[derive(Clone, Debug)]
pub struct RangeIter<'a> {
    raw: SliceIter<'a, CharRange>,
}

impl<'a> IntoIterator for &'a CharSet {
    type Item = CharRange;
    type IntoIter = RangeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges()
    }
}

impl CharSet {
    /// Iterate the compact ranges of this set.
    pub fn ranges(&self) -> RangeIter<'_> {
        RangeIter {
            raw: self.ranges.iter(),
        }
    }

    /// Iterate the characters of this set.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.ranges().flat_map(IntoIterator::into_iter)
    }
}

// forward slice iterators

impl ExactSizeIterator for RangeIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.raw.len()
    }
}

impl Iterator for RangeIter<'_> {
    type Item = CharRange;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().copied()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.raw.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth(n).copied()
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut f: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.raw.fold(init, |acc, &it| f(acc, it))
    }

    #[inline]
    fn position<P>(&mut self, mut predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.raw.position(|&it| predicate(it))
    }

    #[inline]
    fn rposition<P>(&mut self, mut predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.raw.rposition(|&it| predicate(it))
    }
}

impl DoubleEndedIterator for RangeIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().copied()
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut f: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.raw.rfold(init, |acc, &it| f(acc, it))
    }
}

impl iter::FusedIterator for RangeIter<'_> {}
