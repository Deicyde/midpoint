use crate::Midpoint;
use std::ops::*;

fn unsigned_int_midpoint<T: Add + Sub + Cmp>(a: T, b: T) {
    if a > b {
        a + (a - b) / 2
    } else {
        b + (b - a) / 2
    }
}

impl Midpoint for u8 {
    
}