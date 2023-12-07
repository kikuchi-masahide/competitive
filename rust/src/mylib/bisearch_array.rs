use cargo_snippet::snippet;

#[snippet("bisearch_array")]
fn lower_bound<T>(arr: &Vec<T>, x: T) -> usize
where
    T: std::cmp::PartialOrd,
{
    // arr[e-1] <= x < arr[e]
    // なるeを返す(0 <= e <= n)
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

#[snippet("bisearch_array")]
fn upper_bound<T>(arr: &Vec<T>, x: T) -> usize
where
    T: std::cmp::PartialOrd,
{
    // arr[e-1] < x <= arr[e]
    // なるeを返す(0 <= e <= n)
    if x <= arr[0] {
        return 0;
    }
    let mut s = 0;
    let mut e = arr.len();
    while e - s > 1 {
        let m = (s + e) / 2;
        if arr[m] < x {
            s = m;
        } else {
            e = m;
        }
    }
    return e;
}
