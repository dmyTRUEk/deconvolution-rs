//! rust nightly at 2023.09.02
//! compile options: -C opt-level=3 -C target-cpu=native

pub fn index_from_start(vec: Vec<i32>, target: i32) -> usize {
    vec
        .iter()
        .position(|&v| v == target)
        .unwrap()
}

pub fn index_from_end_by_hands(vec: Vec<i32>, target: i32) -> usize {
    vec.len() - 1 - vec
        .iter()
        .rev()
        .position(|&v| v == target)
        .unwrap()
}

pub fn index_from_end_by_enumerate(vec: Vec<i32>, target: i32) -> usize {
    vec
        .iter()
        .enumerate()
        .rev()
        .find(|(_i, &v)| v == target)
        .unwrap()
        .0
}

