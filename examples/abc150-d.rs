// https://atcoder.jp/contests/abc150/tasks/abc150_d

use itertools::Itertools as _;

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt])          => { read!([$tt; read!(usize)]) };
        ([$tt:tt; $n:expr]) => { (0..$n).map(|_| read!($tt)).collect::<Vec<_>>() };
        (($($tt:tt),+))     => { ($(read!($tt)),*) };
        ($ty:ty)            => { input.next().unwrap().parse::<$ty>().unwrap() };
    }

    let (n, m) = read!((usize, usize));
    let a = read!([usize; n]);

    if !a.iter().copied().map(usize::trailing_zeros).all_equal() {
        println!("0");
        return;
    }

    let x0 = a.into_iter().fold(1, num::integer::lcm) / 2;
    let ans = (m + x0) / (2 * x0);
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
