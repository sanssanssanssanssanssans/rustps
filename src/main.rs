#![allow(dead_code)]
#![allow(unused)]
mod fastio;
mod data_structures;
mod sort;
mod math;
use fastio::{FastI, FastO};
use math::fib;


fn main() {
    let mut input = FastI::new();
    let mut output = FastO::new();

    let mut sum = 0;
    let mut l = 0u64;
    let mut r = 200u64;

    while l < r {
        let mid = (l + r + 1) / 2;
        if fib(mid, 0) <= 4_000_000 {
            l = mid;
        } else {
            r = mid - 1;
        }
    }

    let last = l;
    for i in 0..=last {
        let x = fib(i, 0);
        if x % 2 == 0 {
            sum += x;
        }
    }
    
    output.str(format!("{}", sum).as_str());
}