use competitive::prime::*;

fn main() {
    let pf = prime_factors(24);
    let mut ans = std::collections::HashMap::new();
    ans.insert(2, 3);
    ans.insert(3, 1);
    assert_eq!(pf, ans);
}
