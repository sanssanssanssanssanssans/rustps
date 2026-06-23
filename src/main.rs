#![allow(dead_code)]
#![allow(unused)]
mod fastio;
mod data_structures;
mod sort;
use fastio::{FastI, FastO};
use data_structures::MinHeap;
use sort::heapsort;

fn main() {
    let mut input = FastI::new();
    let mut output = FastO::new();

    let (n, m) = (input.usize(), input.i32());
    let mut v = vec![0; n + 1];
    for i in 1 ..= n {
        v[i] = input.i32();
    }
    if m == 1 {
        v = heapsort::sort(v, Some(false));
        for i in 1 ..= n {
            output.i32(v[i]);
            output.newline();
        }
    } else {
        v = heapsort::sort(v, Some(true));
        for i in 0 .. n {
            output.i32(v[i]);
            output.newline();
        }
    }
}