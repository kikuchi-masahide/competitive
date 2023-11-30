#![allow(dead_code)]

type Int = i32;
type UnsignedInt = u32;
type LongLong = i64;
type UnsignedLongLong = u64;
type Double = f64;

fn input_line() -> String {
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();
    input_str = input_str.trim_end().to_owned();
    return input_str;
}

struct Input {}

fn input() -> Input {
    let mut input = Input {};

    return input;
}

fn main() {
    let mut input = input();
}
