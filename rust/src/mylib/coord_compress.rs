use cargo_snippets::snippet;

#[snippet("coord_compress")]
//座標圧縮
struct CoordCompress<T> {
    //decompress[新規座標] = 古い座標
    map: Vec<T>,
}

#[snippet("coord_compress")]
#[derive(Debug, PartialEq, Eq)]
struct CoordCompressError;

#[snippet("coord_compress")]
impl<T> CoordCompress<T>
where
    T: Eq + core::hash::Hash + Ord + Copy,
{
    pub fn new(org: &Vec<T>) -> CoordCompress<T> {
        return CoordCompress {
            map: org.iter().unique().sorted().map(|t| *t).collect_vec(),
        };
    }
    //古い座標 -> 新規座標の変換
    pub fn compress(self: &Self, x: T) -> Result<usize, CoordCompressError> {
        //decompress[s] <= x < decompress[e]
        let mut s: Int = -1;
        let mut e: Int = self.map.len() as Int;
        while e - s > 1 {
            let m = ((e + s) / 2) as usize;
            if self.map[m] <= x {
                s = m as Int;
            } else {
                e = m as Int;
            }
        }
        if s < 0 || self.map[s as usize] != x {
            return Err(CoordCompressError);
        } else {
            return Ok(s as usize);
        }
    }
    //新規座標 -> 古い座標の変換
    pub fn decompress(self: &Self, i: usize) -> Result<T, CoordCompressError> {
        if i >= self.map.len() {
            return Err(CoordCompressError);
        } else {
            return Ok(self.map[i]);
        }
    }
    pub fn len(self: &Self) -> usize {
        return self.map.len();
    }
}
