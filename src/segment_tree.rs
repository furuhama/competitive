pub trait Monoid {
    fn mempty() -> Self;

    fn mappend(l: &Self, r: &Self) -> Self;
}

pub struct SegmentTree<T> {
    data: T,
    span: usize,
    l: Option<Box<SegmentTree<T>>>,
    r: Option<Box<SegmentTree<T>>>,
}

impl<T: Clone + Monoid> SegmentTree<T> {
    pub fn from_slice(s: &[T]) -> Self {
        if s.len() == 1 {
            Self {
                data: s[0].clone(),
                span: 1,
                l: None,
                r: None,
            }
        } else {
            let m = s.len() / 2;
            let l = Self::from_slice(&s[0..m]);
            let r = Self::from_slice(&s[m..]);

            Self {
                data: T::mappend(&l.data, &r.data),
                span: s.len(),
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            }
        }
    }

    pub fn update(&mut self, i: usize, value: T) {
        if self.span == 1 {
            assert!(i == 0);

            self.data = value;
        } else {
            let m = self.span / 2;
            let l = self.l.as_mut().unwrap();
            let r = self.r.as_mut().unwrap();
            if i < m {
                l.update(i, value);
            } else {
                r.update(i - m, value);
            }

            self.data = T::mappend(&l.data, &r.data)
        }
    }

    // query by [l, r) range
    pub fn query(&self, l: usize, r: usize) -> T {
        assert!(l <= r);
        assert!(r <= self.span);

        if l == r {
            T::mempty()
        } else if r - l == self.span {
            self.data.clone()
        } else {
            let m = self.span / 2;
            let l_ref = self.l.as_ref().unwrap();
            let r_ref = self.r.as_ref().unwrap();

            T::mappend(
                &l_ref.query(std::cmp::min(l, m), std::cmp::min(r, m)),
                &r_ref.query(std::cmp::max(l, m) - m, std::cmp::max(r, m) - m),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Range Minimum Query
    impl Monoid for usize {
        fn mempty() -> Self {
            Self::max_value()
        }

        fn mappend(l: &Self, r: &Self) -> Self {
            std::cmp::min(*l, *r)
        }
    }

    #[test]
    fn test_from_slice() {
        let v = vec![1, 2, 3, 4];

        let sg = SegmentTree::from_slice(&v);
        assert_eq!(sg.data, 1);

        let l = sg.l.unwrap();
        assert_eq!(l.data, 1);
        assert_eq!(l.l.unwrap().data, 1);
        assert_eq!(l.r.unwrap().data, 2);

        let r = sg.r.unwrap();
        assert_eq!(r.data, 3);
        assert_eq!(r.l.unwrap().data, 3);
        assert_eq!(r.r.unwrap().data, 4);
    }

    #[test]
    fn test_update_query() {
        let v = vec![1, 2, 3, 4];

        let mut sg = SegmentTree::from_slice(&v);
        assert_eq!(sg.query(0, 4), 1);
        assert_eq!(sg.query(1, 4), 2);
        assert_eq!(sg.query(0, 3), 1);

        sg.update(0, 10); // [10, 2, 3, 4]
        assert_eq!(sg.query(0, 4), 2);
        assert_eq!(sg.query(1, 4), 2);
        assert_eq!(sg.query(0, 3), 2);

        sg.update(0, 0); // [0, 2, 3, 4]
        assert_eq!(sg.query(0, 4), 0);
        assert_eq!(sg.query(1, 4), 2);
        assert_eq!(sg.query(0, 3), 0);
    }
}
