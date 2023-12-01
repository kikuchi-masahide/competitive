use cargo_snippet::snippet;

#[snippet("pow")]
// p >= 1
fn pow<T>(base: &T, p: LongLong) -> T
where
    T: std::ops::Mul<T, Output = T>,
    T: Copy,
{
    return if p < 1 {
        panic!("p must be >= 1");
    } else if p == 1 {
        *base
    } else {
        let mut b2 = pow(base, p / 2)?;
        b2 = b2 * b2;
        if p % 2 == 1 {
            b2 = b2 * (*base);
        }
        b2
    };
}
