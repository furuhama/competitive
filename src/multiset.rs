use crate::binary_search::BinarySearchable;

pub struct MultiSet<T> {
    data: Vec<T>,
}

impl<T: std::cmp::Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, i: T) {
        let idx = self.data.lower_bound(&i);
        self.data.insert(idx, i);
    }

    pub fn remove(&mut self, i: T) -> Result<(), ()> {
        match self.data.binary_search(&i) {
            Ok(idx) => {
                self.data.remove(idx);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn max(&self) -> Option<&T> {
        self.data.iter().rev().next()
    }

    pub fn min(&self) -> Option<&T> {
        self.data.iter().next()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let mut ms = MultiSet::<usize>::new();
        assert_eq!(ms.len(), 0);
        ms.push(1);
        assert_eq!(ms.len(), 1);
        ms.remove(1).unwrap();
        assert_eq!(ms.len(), 0);
    }

    #[test]
    fn test_max() {
        let mut ms = MultiSet::<usize>::new();
        assert_eq!(ms.max(), None);
        ms.push(1);
        assert_eq!(ms.max(), Some(&1));
        ms.push(1);
        assert_eq!(ms.max(), Some(&1));
        ms.push(10);
        assert_eq!(ms.max(), Some(&10));
        ms.push(5);
        assert_eq!(ms.max(), Some(&10));
        ms.remove(1).unwrap();
        assert_eq!(ms.max(), Some(&10));
        ms.remove(10).unwrap();
        assert_eq!(ms.max(), Some(&5));
        ms.remove(5).unwrap();
        assert_eq!(ms.max(), Some(&1));
        ms.remove(1).unwrap();
        assert_eq!(ms.max(), None);
    }

    #[test]
    fn test_min() {
        let mut ms = MultiSet::<usize>::new();
        assert_eq!(ms.min(), None);
        ms.push(5);
        assert_eq!(ms.min(), Some(&5));
        ms.push(5);
        assert_eq!(ms.min(), Some(&5));
        ms.push(1);
        assert_eq!(ms.min(), Some(&1));
        ms.push(10);
        assert_eq!(ms.min(), Some(&1));
        ms.remove(1).unwrap();
        assert_eq!(ms.min(), Some(&5));
        ms.remove(10).unwrap();
        assert_eq!(ms.min(), Some(&5));
        ms.remove(5).unwrap();
        assert_eq!(ms.min(), Some(&5));
        ms.remove(5).unwrap();
        assert_eq!(ms.min(), None);
    }
}
