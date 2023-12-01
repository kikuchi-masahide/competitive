use cargo_snippets::snippet;

#[snippet("bisearch_int")]
// f(arr[0]) = ... = f(arr[e-1]) = true,
// f(arr[e]) = ... = f(arr[n-1]) = false
// なるsを返す(0 <= e <= n)
fn int_bisearch(f: impl Fn(Int) -> bool, n: &Int) -> Int {
    let mut s = -1;
    let mut e = *n;
    while e - s > 1 {
        let m = (s + e) / 2;
        if f(m) {
            s = m;
        } else {
            e = m;
        }
    }
    return e;
}
