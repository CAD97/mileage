use {
    crate::range::CharRange,
    core::{char, cmp},
};

mod iter;
#[cfg(feature = "par-iter")]
mod par_iter;
#[cfg(feature = "owned-set")]
mod owned;

#[cfg(feature = "owned-set")]
pub use self::owned::CharSetBuf;

/// A set slice of characters represented by the compact ranges of characters.
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
    #[allow(unsafe_code)]
    pub /*const*/ fn from_raw(slice: &[CharRange]) -> &CharSet {
        unsafe { &*(slice as *const [CharRange] as *const CharSet) }
    }
}

impl CharSet {
    /// An empty set.
    pub fn empty() -> &'static Self {
        Self::from_raw(&[])
    }

    /// Does this set include this character?
    pub fn contains(&self, c: char) -> bool {
        self.search(c).is_ok()
    }

    /// How many characters are in this set?
    pub fn len(&self) -> usize {
        self.ranges().map(CharRange::len).sum()
    }

    /// Is this set empty?
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Binary search for where a character should be in this set.
    ///
    /// If the value is found then `Ok` is returned, containing the index of
    /// the containing range. If no containing range is found then `Err` is
    /// returned, containing the index where the character should be added.
    #[inline]
    fn search(&self, c: char) -> Result<usize, usize> {
        self.ranges.binary_search_by(|r| r.cmp_char(c))
    }
}
