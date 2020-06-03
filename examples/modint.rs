use competitive::modint::*;

fn main() {
    let m = Mint::new(10);
    let ans = m.pow(1_000_000);
    assert_eq!(ans.value(), 907328795);
}
