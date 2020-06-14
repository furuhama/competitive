use competitive::multiset::MultiSet;

fn main() {
    let mut ms = MultiSet::new();
    ms.push(1);
    assert_eq!(ms.min(), Some(&1));
    assert_eq!(ms.max(), Some(&1));
    ms.push(1);
    assert_eq!(ms.min(), Some(&1));
    assert_eq!(ms.max(), Some(&1));
    ms.push(10);
    assert_eq!(ms.min(), Some(&1));
    assert_eq!(ms.max(), Some(&10));
}
