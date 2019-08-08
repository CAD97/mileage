use core::iter::FromIterator;
use {
    crate::CharRange,
    alloc::{vec, vec::Vec},
    core::{char, cmp, ops::Bound},
};

mod iter;
// mod par_iter;

/// A set of characters represented by the compact ranges of characters.
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Hash)]
pub struct CharSet {
    /// # Correctness
    ///
    /// - Must remain sorted
    /// - Ranges must not overlap or touch
    pub(self) ranges: Vec<CharRange>,
}

impl Ord for CharSet {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| unreachable!("incomparable `CharRange`"))
    }
}

impl From<CharRange> for CharSet {
    fn from(range: CharRange) -> Self {
        CharSet {
            ranges: vec![range],
        }
    }
}

impl CharSet {
    /// An empty set.
    pub fn new() -> Self {
        CharSet { ranges: Vec::new() }
    }

    /// Create a set with the specified capacity for compact ranges
    pub fn with_capacity(capacity: usize) -> Self {
        CharSet {
            ranges: Vec::with_capacity(capacity),
        }
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

    /// Insert a character at the given range index.
    ///
    /// # Correctness
    ///
    /// Assumes
    ///
    /// - `self.ranges[idx - 1].high < c` (or `idx == 0`)
    /// - `self.ranges[idx].low > c` (or `idx == self.ranges.len()`)
    fn insert_at(&mut self, idx: usize, c: char) {
        if idx == self.ranges.len() {
            self.ranges.push(CharRange::singleton(c));
            return;
        }

        let above = &mut self.ranges[idx];
        debug_assert!(above.low > c);
        let high = above.high;

        let diff = above.low as u32 - c as u32;
        if diff == 1 {
            above.low = c;
        } else {
            self.ranges.insert(idx, CharRange::singleton(c));
        }

        if idx > 0 {
            let below = &mut self.ranges[idx - 1];
            let diff = c as u32 - below.high as u32;
            if diff <= 1 {
                below.high = high;
                self.ranges.remove(idx);
            }
        }
    }

    /// Insert a character at the given range index.
    ///
    /// # Correctness
    ///
    /// Assumes
    ///
    /// - `self.ranges[idx].contains(c)`
    fn remove_at(&mut self, idx: usize, c: char) {
        let this = &mut self.ranges[idx];
        debug_assert!(this.contains(c));

        if this.len() == 1 {
            self.ranges.remove(idx);
            return;
        } else if this.low == c {
            *this = CharRange::from((Bound::Excluded(c), Bound::Included(this.high)));
        } else if this.high == c {
            *this = CharRange::from(this.low..=c);
        } else {
            let low = this.low;
            *this = CharRange::from((Bound::Excluded(c), Bound::Included(this.high)));
            // insert before `this`
            self.ranges.insert(idx, CharRange::from(low..=c));
        }
    }
}

/*// Set operations
impl CharSet {
    pub fn difference(&self, other: &Self) -> impl Iterator<Item = CharRange> {
        unimplemented!()
    }

    pub fn symmetric_difference(&self, other: &Self) -> impl Iterator<Item = CharRange> {
        unimplemented!()
    }

    pub fn intersection(&self, other: &Self) -> impl Iterator<Item = CharRange> {
        unimplemented!()
    }

    pub fn union(&self, other: &Self) -> impl Iterator<Item = CharRange> {
        unimplemented!()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        unimplemented!()
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        unimplemented!()
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        unimplemented!()
    }
}*/

impl CharSet {
    pub fn clear(&mut self) {
        self.ranges.clear()
    }

    pub fn insert(&mut self, c: char) {
        if let Err(idx) = self.search(c) {
            self.insert_at(idx, c);
        }
    }

    pub fn insert_range(&mut self, r: CharRange) {
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

    pub fn remove(&mut self, c: char) {
        if let Ok(idx) = self.search(c) {
            self.remove_at(idx, c);
        }
    }

    pub fn remove_range(&mut self, r: CharRange) {
        let idx = match self.search(r.low) {
            Err(idx) => idx,
            Ok(idx) => {
                self.remove_at(idx, r.low);
                idx
            }
        };
        self.remove(r.high);
        while self.ranges[idx].high < r.high {
            assert!(self.ranges[idx].low > r.low);
            self.ranges.remove(idx);
        }
    }
}

impl Extend<CharRange> for CharSet {
    fn extend<T: IntoIterator<Item = CharRange>>(&mut self, iter: T) {
        iter.into_iter().for_each(|r| self.insert_range(r));
    }
}

impl Extend<char> for CharSet {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        iter.into_iter().for_each(|c| self.insert(c));
    }
}

impl FromIterator<CharRange> for CharSet {
    fn from_iter<T: IntoIterator<Item = CharRange>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut set = Self::with_capacity(iter.size_hint().0);
        iter.for_each(|r| set.insert_range(r));
        set
    }
}

impl FromIterator<char> for CharSet {
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
            let mut set = CharSet {
                ranges: set.into_iter().map(Into::into).collect(),
            };
            set.insert_range(CharRange::from(diff));
            let result = CharSet {
                ranges: result.into_iter().map(Into::into).collect(),
            };
            assert_eq!(set, result);
        }
    }
}
