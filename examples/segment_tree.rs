use competitive::segment_tree::*;

#[derive(Clone)]
struct Min {
    pub value: usize,
}

// Range Minimum Query
impl Monoid for Min {
    fn mempty() -> Self {
        Self {
            value: usize::max_value(),
        }
    }

    fn mappend(l: &Self, r: &Self) -> Self {
        Self {
            value: std::cmp::min(l.value, r.value),
        }
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6]
        .iter()
        .map(|e| Min { value: *e })
        .collect::<Vec<_>>();
    let mut sg = SegmentTree::from_slice(&v);

    assert_eq!(sg.query(0, 6).value, 1);
    assert_eq!(sg.query(1, 6).value, 2);

    sg.update(0, Min { value: 10 });
    assert_eq!(sg.query(0, 6).value, 2);
    assert_eq!(sg.query(1, 6).value, 2);
}
