#![allow(dead_code)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::*;
use std::iter::FromIterator;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut in_upper = false;
    let mut in_lower = false;
    let mut set = HashSet::new();

    for c in s {
        if c.is_ascii_uppercase() {
            in_upper = true;
        }
        if c.is_ascii_lowercase() {
            in_lower = true;
        }

        if set.contains(&c) {
            println!("No");
            return;
        }

        set.insert(c);
    }

    if in_upper && in_lower {
        println!("Yes");
    } else {
        println!("No");
    }
}
