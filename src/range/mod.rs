use {
    crate::{AFTER_SURROGATE, BEFORE_SURROGATE},
    core::{
        char,
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
        ops::{Bound, RangeBounds, RangeInclusive},
    },
};

mod iter;

pub use self::iter::Iter;

#[cfg(feature = "par-iter")]
mod par_iter;

/// An inclusive range of characters.
///
/// The most idiomatic way to construct this range is by converting from a std range:
///
/// ```
/// # use { mileage::CharRange, core::{char, ops::RangeInclusive} };
/// assert_eq!(CharRange::from('a'..='z'), CharRange::closed('a', 'z'));
/// assert_eq!(RangeInclusive::from(CharRange::from(..)), '\0'..=char::MAX);
/// ```
///
/// If constructed in reverse order, such that `self.high` is ordered before `self.low`,
/// the range is empty. If you want to iterate in decreasing order, use `.iter().rev()`.
/// All empty ranges are considered equal no matter the internal state.
#[derive(Copy, Clone, Eq)]
pub struct CharRange {
    /// The lowest character in this range (inclusive).
    pub low: char,
    /// The highest character in this range (inclusive).
    pub high: char,
}

impl fmt::Debug for CharRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        RangeInclusive::from(*self).fmt(f)
    }
}

impl PartialEq for CharRange {
    fn eq(&self, other: &Self) -> bool {
        (self.is_empty() && other.is_empty()) || (self.low == other.low && self.high == other.high)
    }
}

/// Lexographic ordering.
///
/// An empty range does not compare.
impl PartialOrd for CharRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_empty() || other.is_empty() {
            None
        } else {
            (self.low, self.high).partial_cmp(&(other.low, other.high))
        }
    }
}

impl Hash for CharRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.is_empty() {
            RangeInclusive::from(CharRange::empty()).hash(state)
        } else {
            RangeInclusive::from(*self).hash(state)
        }
    }
}

impl From<CharRange> for RangeInclusive<char> {
    fn from(range: CharRange) -> Self {
        range.low..=range.high
    }
}

impl CharRange {
    /// A closed range `low..=high`.
    ///
    /// This method is provided as a `const`-capable constructor.
    /// In non-`const` contexts, prefer `CharRange::from(low..=high)` instead.
    pub const fn closed(low: char, high: char) -> CharRange {
        CharRange { low, high }
    }

    /// A range with exactly one member.
    pub const fn singleton(c: char) -> CharRange {
        CharRange::closed(c, c)
    }

    /// A canonical empty range.
    pub const fn empty() -> CharRange {
        CharRange {
            low: char::MAX,
            high: '\0',
        }
    }
}

impl<R: RangeBounds<char>> From<R> for CharRange {
    fn from(range: R) -> Self {
        let low = match range.start_bound() {
            Bound::Excluded(&c) => {
                if c == char::MAX {
                    return CharRange::empty();
                } else if c == BEFORE_SURROGATE {
                    AFTER_SURROGATE
                } else {
                    #[allow(unsafe_code)]
                        unsafe {
                        char::from_u32_unchecked(c as u32 + 1)
                    }
                }
            }
            Bound::Included(&c) => c,
            Bound::Unbounded => '\0',
        };
        let high = match range.end_bound() {
            Bound::Excluded(&c) => {
                if c == '\0' {
                    return CharRange::empty();
                } else if c == AFTER_SURROGATE {
                    BEFORE_SURROGATE
                } else {
                    #[allow(unsafe_code)]
                        unsafe {
                        char::from_u32_unchecked(c as u32 - 1)
                    }
                }
            }
            Bound::Included(&c) => c,
            Bound::Unbounded => char::MAX,
        };
        CharRange { low, high }
    }
}

impl CharRange {
    /// Does this range include this character?
    ///
    /// # Examples
    ///
    /// ```
    /// # use mileage::CharRange;
    /// assert!( CharRange::from('a'..='g').contains('d'));
    /// assert!(!CharRange::from('a'..='g').contains('z'));
    /// assert!(!CharRange::from('a'.. 'a').contains('a'));
    /// assert!(!CharRange::from('z'..='a').contains('g'));
    /// ```
    pub const fn contains(self, c: char) -> bool {
        (self.low <= c) & (c <= self.high)
    }

    /// Determine the ordering of a character compared to this range.
    ///
    /// # Panics
    ///
    /// Panics with debug assertions only if the range is empty. In optimized
    /// builds, arbitrarily returns an ordering that is not `Ordering::Equal`.
    ///
    /// For a partial order, you can simply check emptiness beforehand.
    pub fn cmp_char(self, c: char) -> Ordering {
        debug_assert!(!self.is_empty(), "cannot compare empty range's ordering");
        if self.high < c {
            Ordering::Less
        } else if self.low > c {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// How many characters are in this range?
    pub fn len(self) -> usize {
        self.iter().len()
    }

    /// Is this range empty?
    pub const fn is_empty(self) -> bool {
        self.low > self.high
    }

    /// An iterator over this range.
    pub fn iter(self) -> Iter {
        self.into_iter()
    }
}
