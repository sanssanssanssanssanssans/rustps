#![allow(dead_code)]
#![allow(unused)]
mod fastio;
mod data_structures;
mod sort;
use fastio::{FastI, FastO};

fn main() {
    let mut input = FastI::new();
    let mut output = FastO::new();

    let mut res = 0;

    for i in (1 .. 653_000u128).step_by(2) {
        res += i * i;
    }

    output.str(format!("{}", res).as_str());
}