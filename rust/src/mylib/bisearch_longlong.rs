use cargo_snippets::snippet;

#[snippet("bisearch_longlong")]
// f(0) = ... = f(e-1) = true,
// f(e) = ... = f(n-1) = false
// なるsを返す(0 <= e <= n)
fn longlong_bisearch(f: impl Fn(LongLong) -> bool, n: &LongLong) -> LongLong {
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
