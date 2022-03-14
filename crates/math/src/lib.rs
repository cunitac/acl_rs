pub fn pow_mod(x: i64, mut n: u64, m: u32) -> u32 {
    assert!(m != 0);
    if m == 1 {
        return 0;
    }
    let bt = Barrett::new(m);
    let mut r = 1;
    let mut y = x.rem_euclid(m as i64) as u32;
    while n > 0 {
        if n & 1 == 1 {
            r = bt.mul(r, y);
        }
        y = bt.mul(y, y);
        n >>= 1;
    }
    r
}

struct Barrett {
    m: u32,
    im: u64,
}

impl Barrett {
    /// # Panics
    ///
    /// `(1..1 << 31).contains(&m)` でないとき
    fn new(m: u32) -> Self {
        assert!((1..1 << 31).contains(&m));
        let im = !0 / m as u64 + 1;
        Self { m, im }
    }
    fn mul(&self, a: u32, b: u32) -> u32 {
        let &Self { m, im } = self;
        let z = a as u64 * b as u64;
        let x = (z as u128 * im as u128) >> 64;
        let v = z.wrapping_sub(x as u64 * m as u64) as u32;
        let c = if v < m { v } else { v.wrapping_add(m) };
        c
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn test_barret() {
        let mut rng = thread_rng();
        for _ in 0..100_000 {
            let m = rng.gen_range(1..1 << 31);
            let a = rng.gen::<u32>();
            let b = rng.gen::<u32>();
            let bt = Barrett::new(m);
            assert_eq!((a as u64 * b as u64) % m as u64, bt.mul(a, b) as u64);
        }
    }
}
