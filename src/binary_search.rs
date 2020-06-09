pub fn binary_search<
    T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
>(
    lower: T,
    upper: T,
    proc: impl Fn(T) -> bool,
) -> T {
    let mut lower = lower;
    let mut upper = upper;

    let div = T::from(2);

    loop {
        let mid = (lower + upper) / div;
        if lower == mid || mid == upper {
            break mid;
        }

        if proc(mid) {
            lower = mid;
        } else {
            upper = mid;
        }
    }
}

pub trait BinarySearchable<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

// allows binary_search for Slice
impl<T: Ord> BinarySearchable<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut lower = 0;
        let mut upper = self.len();

        while lower != upper {
            let mid = (lower + upper) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less => {
                    lower = mid + 1;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    upper = mid;
                }
            }
        }
        lower
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut lower = 0;
        let mut upper = self.len();

        while lower != upper {
            let mid = (lower + upper) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    lower = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    upper = mid;
                }
            }
        }
        lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let ans = binary_search(0, v.len(), |idx| v[idx] < 4);
        assert_eq!(ans, 2);

        let ans = binary_search(0.0, 100.0, |f| f * f < 49.0);
        assert_eq!(ans, 7.0);
    }

    #[test]
    fn test_vec_lower_bound() {
        let v = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(v.lower_bound(&3), 2);
        assert_eq!(v.lower_bound(&4), 3);
        assert_eq!(v.lower_bound(&1), 0);
        // out of ranges
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&7), 6);

        let v = vec![1, 2, 3, 3, 3, 4];
        assert_eq!(v.lower_bound(&3), 2);

        let v = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(v.lower_bound(&3), 2);
    }

    #[test]
    fn test_vec_upper_bound() {
        let v = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(v.upper_bound(&3), 3);
        assert_eq!(v.upper_bound(&4), 4);
        // out of ranges
        assert_eq!(v.upper_bound(&0), 0);
        assert_eq!(v.upper_bound(&7), 6);

        let v = vec![1, 2, 3, 3, 3, 4];
        assert_eq!(v.upper_bound(&3), 5);

        let v = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(v.upper_bound(&3), 3);
    }
}
