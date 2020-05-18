use competitive::bitvec::*;

fn main() {
    let mut bv = BitVec::new();
    bv.push(true);
    let num: usize = bv.read_as();
    assert_eq!(num, 1);

    let num = 127_u8;
    let bv = BitVec::from(num);
    assert_eq!(
        bv.data,
        vec![true, true, true, true, true, true, true, false]
    )
}
