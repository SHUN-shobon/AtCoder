#![allow(dead_code)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::cmp::Ordering;
use std::collections::*;
use std::iter::FromIterator;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }

    let mut takahashi = 0;
    let mut aoki = 0;

    for i in 0..x {
        if i % (a + c) < a {
            takahashi += b;
        }
        if i % (d + f) < d {
            aoki += e;
        }
    }

    let ans = match takahashi.cmp(&aoki) {
        Ordering::Greater => "Takahashi",
        Ordering::Less => "Aoki",
        Ordering::Equal => "Draw",
    };

    println!("{}", ans);
}
