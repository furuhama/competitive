use crate::mint::Mint;

// Calculate combination and its elements as mod value
// (mod is defined as Mint::MODULUS const value)
pub struct Precalc {
    inv: Vec<Mint>,
    fact: Vec<Mint>,
    ifact: Vec<Mint>,
}

#[allow(dead_code)]
impl Precalc {
    pub fn new(n: usize) -> Self {
        let mut inv = vec![Mint::new(1); n + 1];
        let mut fact = vec![Mint::new(1); n + 1];
        let mut ifact = vec![Mint::new(1); n + 1];

        for i in 2..=n {
            fact[i] = fact[i - 1] * Mint::new(i as u32);
        }

        ifact[n] = fact[n].inv();

        if n > 0 {
            inv[n] = ifact[n] * fact[n - 1];
        }

        for i in (1..n).rev() {
            ifact[i] = ifact[i + 1] * Mint::new((i + 1) as u32);
            inv[i] = ifact[i] * fact[i - 1];
        }

        Self { inv, fact, ifact }
    }
    pub fn inv(&self, n: usize) -> Mint {
        assert!(n > 0);
        self.inv[n]
    }
    pub fn fact(&self, n: usize) -> Mint {
        self.fact[n]
    }
    pub fn ifact(&self, n: usize) -> Mint {
        self.ifact[n]
    }
    pub fn comb(&self, n: usize, k: usize) -> Mint {
        if k > n {
            return Mint::new(0);
        }
        self.fact[n] * self.ifact[k] * self.ifact[n - k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact() {
        let pc = Precalc::new(20);
        assert_eq!(pc.fact(0), 1);
        assert_eq!(pc.fact(1), 1);
        assert_eq!(pc.fact(2), 2);
        assert_eq!(pc.fact(3), 6);
        assert_eq!(pc.fact(4), 24);
        assert_eq!(pc.fact(20), 146_326_063)
    }

    #[test]
    fn test_inv() {
        let pc = Precalc::new(20);
        assert_eq!(pc.inv(1), 1);
        assert_eq!(pc.inv(2), 500_000_004);
        assert_eq!(pc.inv(3), 333_333_336);
        assert_eq!(pc.inv(20), 850_000_006)
    }

    #[test]
    fn test_ifact() {
        let pc = Precalc::new(20);
        assert_eq!(pc.ifact(0), 1);
        assert_eq!(pc.ifact(1), 1);
        assert_eq!(pc.ifact(2), 500_000_004);
        assert_eq!(pc.ifact(3), 166_666_668);
        assert_eq!(pc.ifact(20), 120_104_394)
    }

    #[test]
    fn test_comb() {
        let pc = Precalc::new(20);
        assert_eq!(pc.comb(0, 0), 1);
        assert_eq!(pc.comb(1, 0), 1);
        assert_eq!(pc.comb(1, 1), 1);
        assert_eq!(pc.comb(2, 1), 2);
        assert_eq!(pc.comb(2, 2), 1);
        assert_eq!(pc.comb(5, 1), 5);
        assert_eq!(pc.comb(5, 2), 10);
        assert_eq!(pc.comb(5, 3), 10);
        assert_eq!(pc.comb(20, 10), 184_756);

        // n < k
        assert_eq!(pc.comb(2, 5), 0);
    }
}
