fn min<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd,
{
    return if a < b { a } else { b };
}

fn max<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd,
{
    return if a > b { a } else { b };
}
