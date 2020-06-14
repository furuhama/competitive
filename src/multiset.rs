#[derive(Clone, Debug)]
pub struct MultiSet<T> {
    data: std::collections::BTreeMap<T, usize>,
}

impl<T: std::cmp::Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self {
            data: std::collections::BTreeMap::new(),
        }
    }

    pub fn push(&mut self, v: T) {
        *self.data.entry(v).or_insert(0) += 1;
    }

    pub fn remove(&mut self, v: T) -> Result<(), ()> {
        match self.data.get_mut(&v) {
            Some(i) => {
                if *i == 1 {
                    self.data.remove(&v).unwrap();
                } else {
                    *i -= 1;
                }
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn max(&self) -> Option<&T> {
        self.data.iter().rev().next().map(|(k, _)| k)
    }

    pub fn min(&self) -> Option<&T> {
        self.data.iter().next().map(|(k, _)| k)
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
