use competitive::precalc::*;

fn main() {
    let pc = Precalc::new(1_000);
    assert_eq!(pc.comb(1_000, 500), 159_835_829);
}
