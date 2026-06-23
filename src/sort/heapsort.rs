use crate::data_structures::MinHeap;

pub fn sort<T: PartialOrd>(v: Vec<T>, rev: Option<bool>) -> Vec<T> {
    let mut h = MinHeap::new();
    for x in v {
        h.push(x);
    }

    let mut res = Vec::with_capacity(h.size());
    while let Some(x) = h.pop() {
        res.push(x);
    }

    if rev.unwrap_or(false) {
        res.reverse();
    }

    res
}