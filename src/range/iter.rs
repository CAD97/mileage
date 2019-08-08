use {
    crate::{CharRange, AFTER_SURROGATE, BEFORE_SURROGATE},
    core::{char, iter::FusedIterator},
};

/// An iterator over a range of unicode code points.
///
/// Constructed via `CharRange::iter`. See `CharRange` for more information.
#[derive(Clone, Debug)]
pub struct Iter {
    low: char,
    high: char,
}

impl IntoIterator for CharRange {
    type Item = char;
    type IntoIter = Iter;

    #[allow(unsafe_code)]
    fn into_iter(self) -> Iter {
        Iter {
            low: self.low,
            high: self.high,
        }
    }
}

impl Iter {
    #[inline]
    fn step_forward(&mut self) {
        if self.low == char::MAX {
            self.high = '\0';
        } else {
            self.low = if self.low == BEFORE_SURROGATE {
                AFTER_SURROGATE
            } else {
                #[allow(unsafe_code)]
                unsafe {
                    char::from_u32_unchecked(self.low as u32 + 1)
                }
            }
        }
    }

    #[inline]
    fn step_backward(&mut self) {
        if self.high == '\0' {
            self.low = char::MAX;
        } else {
            self.high = if self.high == AFTER_SURROGATE {
                BEFORE_SURROGATE
            } else {
                #[allow(unsafe_code)]
                unsafe {
                    char::from_u32_unchecked(self.low as u32 - 1)
                }
            }
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.low > self.high
    }
}

impl Iterator for Iter {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Avoid unstable `<&mut I as ExactSizeIterator>::is_empty`
        if (&*self).is_empty() {
            return None;
        }

        let c = self.low;
        self.step_forward();
        Some(c)
    }

    // override those default provided where we can do better

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl DoubleEndedIterator for Iter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // Avoid unstable `<&mut I as ExactSizeIterator>::is_empty`
        if (&*self).is_empty() {
            return None;
        }

        let c = self.low;
        self.step_backward();
        Some(c)
    }
}

impl ExactSizeIterator for Iter {
    // doesn't work when usize == u16 but Range<u32> is ExactSizeIterator so /shrug
    // we use said impl here so we're exactly as broken as the standard library
    fn len(&self) -> usize {
        #[allow(clippy::range_plus_one)] // for ExactSizeIterator impl
        let len = (self.low as u32..self.high as u32 + 1).len() as u32;
        ((if self.low <= BEFORE_SURROGATE && self.high >= AFTER_SURROGATE {
            len - (AFTER_SURROGATE as u32 - (BEFORE_SURROGATE as u32 + 1))
        } else {
            len
        }) as usize)
    }
}

impl FusedIterator for Iter {}

// unsafe impl TrustedLen for Iter {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn exact_size_iterator() {
        // https://github.com/rust-lang/rust/issues/34433#issuecomment-244573473
        let v: Vec<_> = CharRange::from('a'..='g')
            .iter()
            .enumerate()
            .rev()
            .collect();
        assert_eq!(
            v,
            vec![
                (6, 'g'),
                (5, 'f'),
                (4, 'e'),
                (3, 'd'),
                (2, 'c'),
                (1, 'b'),
                (0, 'a')
            ]
        );
    }
}
