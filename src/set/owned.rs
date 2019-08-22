use {
    crate::{range::CharRange, set::CharSet},
    alloc::{vec, vec::Vec},
    core::{
        char, cmp,
        iter::FromIterator,
        ops::{Bound, Deref},
    },
};

/// A mutable set of characters represented by the compact ranges of characters.
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Hash)]
pub struct CharSetBuf {
    /// # Correctness
    ///
    /// - Must remain sorted
    /// - Ranges must not overlap or touch
    pub(self) ranges: Vec<CharRange>,
}

impl Deref for CharSetBuf {
    type Target = CharSet;

    fn deref(&self) -> &Self::Target {
        CharSet::from_raw(&*self.ranges)
    }
}

impl Ord for CharSetBuf {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| unreachable!("incomparable `CharRange`"))
    }
}

// sorry for the inference issues this causes I guess ¯\_(ツ)_/¯
impl<R: Into<CharRange>> From<R> for CharSetBuf {
    fn from(range: R) -> Self {
        Self {
            ranges: vec![range.into()],
        }
    }
}

impl CharSetBuf {
    /// An empty set.
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    /// Create a set with the specified capacity for compact ranges
    pub fn with_capacity(capacity: usize) -> Self {
        CharSetBuf {
            ranges: Vec::with_capacity(capacity),
        }
    }
}

impl CharSetBuf {
    /// Clear this set such that it is empty again.
    pub fn clear(&mut self) {
        self.ranges.clear()
    }

    /// Insert a single character to this set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mileage::set::CharSetBuf;
    /// let mut set = CharSetBuf::from('a'..='b');
    /// set.insert('d');
    /// set.insert('c');
    /// assert_eq!(set, CharSetBuf::from('a'..='d'));
    /// ```
    pub fn insert(&mut self, c: char) {
        if let Err(idx) = self.search(c) {
            if idx == self.ranges.len() {
                self.ranges.push(CharRange::singleton(c));
                return;
            }

            let above = &mut self.ranges[idx];
            debug_assert!(above.low > c);
            let high = above.high;

            if above.low as u32 - c as u32 == 1 {
                above.low = c;
            } else {
                self.ranges.insert(idx, CharRange::singleton(c));
            }

            if idx > 0 {
                let below = &mut self.ranges[idx - 1];
                if c as u32 - below.high as u32 <= 1 {
                    below.high = high;
                    self.ranges.remove(idx);
                }
            }
        }
    }

    pub fn insert_range(&mut self, r: CharRange) {
        if r.is_empty() {
            return;
        }

        // low_idx: inclusive index of lowest replaced range
        // low_char: lowest char of the new inserted range
        let (mut low_idx, mut low_char) = match self.search(r.low) {
            Ok(idx) => (idx, self.ranges[idx].low),
            Err(idx) => (idx, r.low),
        };
        // extend left if collapse needed
        if low_idx > 0 && low_char as u32 - self.ranges[low_idx - 1].high as u32 <= 1 {
            low_idx -= 1;
            low_char = self.ranges[low_idx].low;
        }

        // high_idx: exclusive index of highest replaced range
        // high_char: highest char of the new inserted range
        let (mut high_idx, mut high_char) = match self.search(r.high) {
            Ok(idx) => (idx + 1, self.ranges[idx].high),
            Err(idx) => (idx, r.high),
        };
        // extend right if collapse needed
        if high_idx < self.ranges.len() && self.ranges[high_idx].low as u32 - high_char as u32 <= 1
        {
            high_char = self.ranges[high_idx].high;
            high_idx += 1;
        }

        if low_idx == high_idx {
            // insert new range
            self.ranges
                .insert(low_idx, CharRange::from(low_char..=high_char));
        } else {
            // remove all but lowest range
            self.ranges
                .drain((Bound::Excluded(low_idx), Bound::Excluded(high_idx)));
            // fix the remaining range to cover entire new range
            self.ranges[low_idx] = CharRange::from(low_char..=high_char);
        }
    }

    /// Remove a single character to this set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {core::iter::FromIterator, mileage::{set::CharSetBuf, CharRange}};
    /// let mut set = CharSetBuf::from('a'..='c');
    /// set.remove('b');
    /// assert_eq!(set, CharSetBuf::from_iter(vec!['a', 'c']));
    /// ```
    pub fn remove(&mut self, c: char) {
        if let Ok(idx) = self.search(c) {
            let this = &mut self.ranges[idx];
            if this.len() == 1 {
                self.ranges.remove(idx);
            } else if this.low == c {
                *this = CharRange::from((Bound::Excluded(c), Bound::Included(this.high)));
            } else if this.high == c {
                *this = CharRange::from(this.low..=c);
            } else {
                let low = this.low;
                *this = CharRange::from((Bound::Excluded(c), Bound::Included(this.high)));
                self.ranges.insert(
                    idx, // insert before `this`
                    CharRange::from((Bound::Included(low), Bound::Excluded(c))),
                );
            }
        }
    }

    pub fn remove_range(&mut self, r: CharRange) {
        if r.is_empty() {
            return;
        }

        // inclusive index of lowest edited range
        let low = self.search(r.low).unwrap_or_else(|it| it);
        // exclusive index of highest edited range
        let high = match self.search(r.high) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        };

        if low == high {
            // no change, range not included
            debug_assert!(!self.contains(r.low));
            debug_assert!(!self.contains(r.high));
        } else if low + 1 == high {
            // one range to split
            let split = &mut self.ranges[low];
            if split.low == r.low && split.high == r.high {
                // remove entire range
                self.ranges.remove(low);
            } else if split.low == r.low {
                // shrink to top
                debug_assert!(split.high > r.high);
                *split = CharRange::from((Bound::Excluded(r.high), Bound::Included(split.high)));
            } else if split.high == r.high {
                // shrink to bottom
                debug_assert!(r.low > split.low);
                *split = CharRange::from((Bound::Included(split.low), Bound::Excluded(r.low)));
            } else {
                // split
                debug_assert!(split.high > r.high);
                debug_assert!(r.low > split.low);
                let high_char = split.high;
                *split = CharRange::from((Bound::Included(split.low), Bound::Excluded(r.low)));
                self.ranges.insert(
                    high, // insert after `split`
                    CharRange::from((Bound::Excluded(r.high), Bound::Included(high_char))),
                );
            }
        } else {
            let left = &mut self.ranges[low];
            *left = CharRange::from((Bound::Included(left.low), Bound::Excluded(r.low)));
            let high = high - 1; // inclusive
            let right = &mut self.ranges[high];
            *right = CharRange::from((Bound::Excluded(r.high), Bound::Included(right.high)));
            self.ranges
                .drain((Bound::Excluded(low), Bound::Excluded(high)));
        }
    }
}

impl Extend<CharRange> for CharSetBuf {
    fn extend<T: IntoIterator<Item = CharRange>>(&mut self, iter: T) {
        iter.into_iter().for_each(|r| self.insert_range(r));
    }
}

impl Extend<char> for CharSetBuf {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        iter.into_iter().for_each(|c| self.insert(c));
    }
}

impl FromIterator<CharRange> for CharSetBuf {
    fn from_iter<T: IntoIterator<Item = CharRange>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut set = Self::with_capacity(iter.size_hint().0);
        iter.for_each(|r| set.insert_range(r));
        set
    }
}

impl FromIterator<char> for CharSetBuf {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut set = Self::new();
        iter.into_iter().for_each(|c| set.insert(c));
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_range() {
        #[rustfmt::skip]
        let test_data = vec![
            (vec!['a'..='c'], 'z'..='a' /* empty */, vec!['a'..='c']),
            (vec![], 'm'..='m', vec!['m'..='m']),
            (vec!['m'..='m'], 'l'..='l', vec!['l'..='m']),
            (vec!['m'..='m'], 'n'..='n', vec!['m'..='n']),
            (vec!['m'..='m'], 'k'..='k', vec!['k'..='k', 'm'..='m']),
            (vec!['m'..='m'], 'o'..='o', vec!['m'..='m', 'o'..='o']),
            (vec!['l'..='l', 'n'..='n'], 'm'..='m', vec!['l'..='n']),
            (vec!['l'..='n'], 'm'..='m', vec!['l'..='n']),
            (vec!['a'..='c', 'e'..='g'], 'd'..='d', vec!['a'..='g']),
            (vec!['a'..='c', 'e'..='g'], 'b'..='f', vec!['a'..='g']),
            (vec!['c'..='e', 'g'..='i'], 'a'..='f', vec!['a'..='i']),
            (vec!['c'..='e', 'g'..='i'], 'f'..='k', vec!['c'..='k']),
            (vec!['a'..='b', 'h'..='j'], 'd'..='f', vec!['a'..='b', 'd'..='f', 'h'..='j']),
        ];

        for (set, diff, result) in test_data {
            let mut set = CharSetBuf {
                ranges: set.into_iter().map(Into::into).collect(),
            };
            set.insert_range(CharRange::from(diff));
            let result = CharSetBuf {
                ranges: result.into_iter().map(Into::into).collect(),
            };
            assert_eq!(set, result);
        }
    }

    #[test]
    fn remove_range() {
        #[rustfmt::skip]
        let test_data = vec![
            (vec!['a'..='c'], 'z'..='a' /* empty */, vec!['a'..='c']),
            (vec![], 'a'..='a', vec![]),
            (vec!['a'..='a'], 'a'..='a', vec![]),
            (vec!['a'..='c'], 'a'..='a', vec!['b'..='c']),
            (vec!['a'..='c'], 'b'..='b', vec!['a'..='a', 'c'..='c']),
            (vec!['a'..='c'], 'c'..='c', vec!['a'..='b']),
            (vec!['a'..='b', 'd'..='e'], 'b'..='d', vec!['a'..='a', 'e'..='e']),
        ];

        for (set, diff, result) in test_data {
            let mut set = CharSetBuf {
                ranges: set.into_iter().map(Into::into).collect(),
            };
            set.remove_range(CharRange::from(diff));
            let result = CharSetBuf {
                ranges: result.into_iter().map(Into::into).collect(),
            };
            assert_eq!(set, result);
        }
    }
}
