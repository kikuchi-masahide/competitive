use cargo_snippets::snippet;

#[snippet("bisearch_array")]
// arr[e-1] <= x < arr[e]
// なるeを返す(0 <= e <= n)
fn array_bisearch<T>(arr: &Vec<T>, x: T) -> usize
where
    T: std::cmp::PartialOrd,
{
    if x < arr[0] {
        return 0;
    }
    let mut s = 0;
    let mut e = arr.len();
    while e - s > 1 {
        let m = (s + e) / 2;
        if arr[m] <= x {
            s = m;
        } else {
            e = m;
        }
    }
    return e;
}
