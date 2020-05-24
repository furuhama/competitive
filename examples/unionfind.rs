use competitive::unionfind::*;

fn main() {
    let mut uf = UnionFind::new(5);

    assert_eq!(uf.find(0), 0);
    assert_eq!(uf.find(1), 1);

    uf.union(0, 1);

    assert_eq!(uf.find(0), 1);
    assert_eq!(uf.find(1), 1);
}
