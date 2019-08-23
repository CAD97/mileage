use {
    crate::range::CharRange,
    core::{char, ops::Index},
};
use core::ops::Deref;

mod iter;

/// A mapping from unicode codepoints to values.
#[derive(Debug)]
pub struct CharMapRef<'a, T> {
    pub(self) ranges: &'a [CharRange],
    pub(self) values: &'a [T],
}

// avoid unneeded bounds
impl<'a, T> Clone for CharMapRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> Copy for CharMapRef<'a, T> {}

impl<'a, T> CharMapRef<'a, T> {
    /// Create a `CharMapRef` from raw slices. Intended for use by code generation.
    pub const fn from_raw(ranges: &'a [CharRange], values: &'a [T]) -> Self {
        CharMapRef { ranges, values }
    }
}

impl<'a, T> CharMapRef<'a, T> {
    /// An empty map.
    pub const fn empty() -> Self {
        // this trait mess to avoid disallowed unsizing cast in const fn
        trait EmptySlice<'a>: Sized + 'a {
            const SLICE: &'a [Self];
        }
        impl<'a, T: 'a> EmptySlice<'a> for T {
            const SLICE: &'a [T] = &[];
        }
        Self::from_raw(EmptySlice::SLICE, EmptySlice::SLICE)
    }

    /// Does this mapping include this codepoint?
    pub fn contains(self, c: char) -> bool {
        self.search(c).is_ok()
    }

    /// How many codepoints are in this mapping?
    pub fn len(self) -> usize {
        self.ranges().map(CharRange::len).sum()
    }

    /// Is this mapping empty?
    pub fn is_empty(self) -> bool {
        self.ranges.is_empty()
    }

    /// Binary search for where a codepoint should be in this mapping.
    ///
    /// If the value is found then `Ok` is returned, containing the index of
    /// the containing range. If no containing range is found then `Err` is
    /// returned, containing the index where the codepoint should be added.
    #[inline]
    fn search(self, c: char) -> Result<usize, usize> {
        self.ranges.binary_search_by(|r| r.cmp_char(c))
    }

    /// Get a value from this mapping.
    pub fn get(self, c: char) -> Option<&'a T> {
        let idx = self.search(c).ok()?;
        Some(&self.values[idx])
    }
}

impl<'a, T> Index<char> for CharMapRef<'a, T> {
    type Output = T;

    fn index(&self, c: char) -> &Self::Output {
        self.get(c).expect("no entry found for key")
    }
}
