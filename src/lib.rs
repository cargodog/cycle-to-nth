//! A fork of the [`Cycle`] iterator adapter, to cycle until reaching the `n`th iteration.
use core::iter::Iterator;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CycleToNth<I> {
    orig: I,
    iter: I,
    nth: usize,
    n: usize,
}

impl<I: Clone> CycleToNth<I> {
    pub fn new(iter: I, nth: usize) -> CycleToNth<I> {
        CycleToNth {
            orig: iter.clone(),
            iter,
            nth,
            n: 0,
        }
    }
}

impl<I> Iterator for CycleToNth<I>
where
    I: Clone + Iterator,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if self.n == self.nth {
            // Terminate on the nth iteration
            None
        } else {
            self.n += 1;
            match self.iter.next() {
                None => {
                    self.iter = self.orig.clone();
                    self.iter.next()
                }
                y => y,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // the CycleToNth iterator is either empty of infinite
        match self.orig.size_hint() {
            sz @ (0, Some(0)) => sz,
            (0, _) => (0, None),
            _ => (usize::MAX, None),
        }
    }
}

pub trait CycleToNthAdapter
where
    Self: Sized + Clone,
{
    fn cycle_to_nth(self, nth: usize) -> CycleToNth<Self>;
}

impl<I> CycleToNthAdapter for I
where
    Self: Sized + Clone,
    I: Iterator,
{
    fn cycle_to_nth(self, nth: usize) -> CycleToNth<Self> {
        CycleToNth::new(self, nth)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn extend_with_cycles() {
        let v = vec![1u32];
        assert_eq!(v.iter().cycle_to_nth(5).sum::<u32>(), 5u32);
    }

    #[test]
    fn truncate_to_nth() {
        let v = vec![1u32; 10];
        assert_eq!(v.iter().cycle_to_nth(5).sum::<u32>(), 5u32);
    }
}
