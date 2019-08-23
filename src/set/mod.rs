use {
    crate::range::CharRange,
    core::{char, cmp},
};

mod iter;
#[cfg(feature = "owned-set")]
mod owned;
#[cfg(feature = "par-iter")]
mod par_iter;

pub use self::iter::RangeIter;
#[cfg(feature = "owned-set")]
pub use self::owned::CharSetBuf;

/// A set slice of codepoints represented by the compact ranges of codepoints.
#[derive(Debug, Eq, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct CharSet {
    pub(self) ranges: [CharRange],
}

impl Ord for CharSet {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| unreachable!("incomparable `CharRange`"))
    }
}

impl CharSet {
    /// Create a `CharSet` from a raw slice of ranges. Intended for use by code generation.
    #[allow(unsafe_code)]
    pub fn from_raw(slice: &[CharRange]) -> &CharSet {
        unsafe { &*(slice as *const [CharRange] as *const CharSet) }
    }
}

impl CharSet {
    /// An empty set.
    pub fn empty() -> &'static Self {
        Self::from_raw(&[])
    }

    /// Does this set include this codepoint?
    pub fn contains(&self, c: char) -> bool {
        self.search(c).is_ok()
    }

    /// How many codepoints are in this set?
    pub fn len(&self) -> usize {
        self.ranges().map(CharRange::len).sum()
    }

    /// Is this set empty?
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Binary search for where a codepoint should be in this set.
    ///
    /// If the value is found then `Ok` is returned, containing the index of
    /// the containing range. If no containing range is found then `Err` is
    /// returned, containing the index where the codepoint should be added.
    #[inline]
    fn search(&self, c: char) -> Result<usize, usize> {
        self.ranges.binary_search_by(|r| r.cmp_char(c))
    }
}
