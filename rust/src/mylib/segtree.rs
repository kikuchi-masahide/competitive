use cargo_snippet::snippet;

#[snippet("segtree")]
struct SegmentTree<T>
where
    T: Copy + core::ops::Mul<Output = T> + num_traits::One,
{
    data: Vec<T>,
}

#[snippet("segtree")]
impl<T> SegmentTree<T>
where
    T: Copy + core::ops::Mul<Output = T> + num_traits::One,
{
    pub fn new(n: usize) -> SegmentTree<T> {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        return SegmentTree {
            data: vec![T::one(); 2 * m],
        };
    }
    pub fn from(vec: &Vec<T>) -> SegmentTree<T> {
        let mut m = 1;
        while m < vec.len() {
            m *= 2;
        }
        return SegmentTree {
            data: {
                let mut v = vec![T::one(); 2 * m];
                for i in 0..vec.len() {
                    v[i + m] = vec[i];
                }
                for i in (0..m).rev() {
                    v[i] = v[2 * i] * v[2 * i + 1];
                }
                v
            },
        };
    }
    pub fn update(self: &mut Self, n: usize, x: T) {
        let mut n = n + self.data.len() / 2;
        self.data[n] = x;
        n /= 2;
        while n > 0 {
            self.data[n] = self.data[2 * n] * self.data[2 * n + 1];
            n /= 2;
        }
    }
    pub fn query(self: &Self, mut l: usize, mut r: usize) -> T {
        let mut ansl = T::one();
        let mut ansr = T::one();
        r += 1;
        l += self.data.len() / 2;
        r += self.data.len() / 2;
        while l < r {
            if l & 1 == 1 {
                ansl = ansl * self.data[l];
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                ansr = self.data[r] * ansr;
            }
            l >>= 1;
            r >>= 1;
        }
        return ansl * ansr;
    }
}
