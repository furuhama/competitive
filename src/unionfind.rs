pub struct UnionFind(Vec<usize>);

#[allow(dead_code)]
impl UnionFind {
    pub fn new(len: usize) -> Self {
        Self((0..len).collect())
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.0[i] == i {
            i
        } else {
            let parent = self.0[i];
            self.0[i] = self.find(parent);
            self.0[i]
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        self.0[root_i] = root_j;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut uf = UnionFind::new(10);

        for i in 0..10 {
            assert_eq!(uf.find(i), i);
        }

        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(4, 5);
        uf.union(4, 6);
        uf.union(4, 7);

        assert_eq!(uf.find(0), 1);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 3);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 7);
        assert_eq!(uf.find(5), 7);
        assert_eq!(uf.find(6), 7);
        assert_eq!(uf.find(7), 7);
        assert_eq!(uf.find(8), 8);
        assert_eq!(uf.find(9), 9);

        uf.union(0, 2);
        uf.union(4, 8);
        uf.union(5, 9);

        assert_eq!(uf.find(0), 3);
        assert_eq!(uf.find(1), 3);
        assert_eq!(uf.find(2), 3);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 9);
        assert_eq!(uf.find(5), 9);
        assert_eq!(uf.find(6), 9);
        assert_eq!(uf.find(7), 9);
        assert_eq!(uf.find(8), 9);
        assert_eq!(uf.find(9), 9);

        // Try to union two nodes already in the same tree
        uf.union(0, 1);
        uf.union(2, 3);

        assert_eq!(uf.find(0), 3);
        assert_eq!(uf.find(1), 3);
        assert_eq!(uf.find(2), 3);
        assert_eq!(uf.find(3), 3);

        uf.union(0, 4);

        for i in 0..10 {
            assert_eq!(uf.find(i), 9);
        }
    }
}
