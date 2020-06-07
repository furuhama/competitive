pub trait BinarySearchable<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearchable<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }
        left
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    left = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
