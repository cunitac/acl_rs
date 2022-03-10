pub struct Dsu {
    len: usize,
    /// - `i` が値でないとき、`p[i]` は `i` の親
    /// - `i` が根のとき、`p[i].wrapping_neg()` は木の頂点数
    p: Vec<usize>,
}

impl Dsu {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            p: vec![1usize.wrapping_neg(); len],
        }
    }
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        let mut x = self.leader(a);
        let mut y = self.leader(b);
        if x != y {
            if self.p[x].wrapping_neg() < self.p[y].wrapping_neg() {
                std::mem::swap(&mut x, &mut y);
            }
            self.p[x] = self.p[x].wrapping_add(self.p[y]);
            self.p[y] = x;
        }
        x
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }
    pub fn leader(&mut self, a: usize) -> usize {
        if self.p[a] < self.len {
            self.p[a] = self.leader(a);
            self.p[a]
        } else {
            self.p[a].wrapping_neg()
        }
    }
    pub fn size(&mut self, a: usize) -> usize {
        let x = self.leader(a);
        self.p[x].wrapping_neg()
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let leader_buf = (0..self.len).map(|a| self.leader(a)).collect::<Vec<_>>();
        let mut group_size = vec![0; self.len];
        for &l in &leader_buf {
            group_size[l] += 1;
        }
        let mut result = group_size
            .into_iter()
            .map(Vec::with_capacity)
            .collect::<Vec<_>>();
        for (i, &l) in leader_buf.iter().enumerate() {
            result[l].push(i);
        }
        result.retain(|v| !v.is_empty());
        result
    }
}
