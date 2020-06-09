use competitive::binary_search::*;

fn main() {
    let v = vec![0, 1, 2, 3, 3, 3, 4, 5, 6];
    assert_eq!(v.lower_bound(&3), 3);

    let sqrt = binary_search(0.0, 100.0, |f| f * f < 256.0);
    assert_eq!(sqrt, 16.0);
}
