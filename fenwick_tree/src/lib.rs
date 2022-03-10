use std::{
    iter::Sum,
    ops::{AddAssign, Range, RangeBounds, Sub},
};

pub struct FenwickTree<T> {
    len: usize,
    val: Vec<T>,
}

impl<T: Val> FenwickTree<T> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            val: vec![T::zero(); len],
        }
    }
    pub fn add(&mut self, mut i: usize, x: T) {
        assert!((0..self.len).contains(&i));
        i += 1;
        while i <= self.len {
            self.val[i - 1] += &x;
            i += 1 << i.trailing_zeros();
        }
    }
    pub fn sum(&self, range: impl RangeBounds<usize>) -> T {
        let Range { start, end } = self.range(range);
        self.sum_prefix(end) - self.sum_prefix(start)
    }
    fn sum_prefix(&self, mut i: usize) -> T {
        let mut sum = T::zero();
        while i > 0 {
            sum += &self.val[i - 1];
            i -= 1 << i.trailing_zeros();
        }
        sum
    }
    fn range(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&s) => s + 1,
            Excluded(&s) => s,
            Unbounded => 0,
        };
        start..end
    }
}

/// ほとんどの数値型はこれを満たすはずです。
pub trait Val: Clone + for<'a> AddAssign<&'a Self> + Sub<Output = Self> + Sum<Self> {
    fn zero() -> Self {
        std::iter::empty().sum()
    }
}
impl<T: Clone + for<'a> AddAssign<&'a Self> + Sub<Output = Self> + Sum<Self>> Val for T {}
