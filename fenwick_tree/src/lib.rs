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
            Unbounded => self.len,
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

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_random() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let len = rng.gen_range(1..100);
            let mut vec = vec![0; len];
            let mut ft = FenwickTree::<i32>::new(len);
            for _ in 0..1000 {
                let i = rng.gen_range(0..len);
                let x = rng.gen_range(-100..=100);
                vec[i] += x;
                ft.add(i, x);
                let l = rng.gen_range(0..len);
                let r = rng.gen_range(l + 1..=len);
                let sum_vec = vec[l..r].iter().sum::<i32>();
                let sum_ft = ft.sum(l..r);
                assert_eq!(sum_vec, sum_ft);
            }
        }
    }

    #[test]
    fn test_range() {
        let ft = FenwickTree::<i32>::new(10);
        assert_eq!(ft.range(..), 0..10);
        assert_eq!(ft.range(..3), 0..3);
        assert_eq!(ft.range(6..), 6..10);
        assert_eq!(ft.range(4..7), 4..7);
        assert_eq!(ft.range(4..=7), 4..8);
        assert_eq!(ft.range(..=5), 0..6);
    }
}
