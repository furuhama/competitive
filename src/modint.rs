#[derive(Clone, Copy, Debug)]
pub struct Mint {
    value: u32,
}

impl std::ops::Add for Mint {
    type Output = Mint;
    fn add(self, rhs: Mint) -> Self::Output {
        let mut d = self.value + rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        Mint::new(d)
    }
}

impl std::ops::AddAssign for Mint {
    fn add_assign(&mut self, rhs: Mint) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Mint {
    type Output = Mint;
    fn sub(self, rhs: Mint) -> Self::Output {
        let mut d = self.value + Self::MODULUS - rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        Mint::new(d)
    }
}

impl std::ops::SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Mint) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Mint {
    type Output = Mint;
    fn mul(self, rhs: Mint) -> Self::Output {
        let d = self.value as u64 * rhs.value as u64 % Self::MODULUS as u64;

        Mint::new(d as u32)
    }
}

impl std::ops::MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Mint) {
        *self = *self * rhs;
    }
}

impl std::ops::Neg for Mint {
    type Output = Mint;

    fn neg(self) -> Self::Output {
        let d = match self.value {
            0 => 0,
            _ => Self::MODULUS - self.value,
        };

        Mint::new(d)
    }
}

impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<usize> for Mint {
    fn from(val: usize) -> Self {
        let d = (val % Self::MODULUS as usize) as u32;

        Mint::new(d)
    }
}

impl PartialEq<Self> for Mint {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u32> for Mint {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}

#[allow(dead_code)]
impl Mint {
    // to change modulus, rewrite this value
    // (modulus should be a prime number)
    pub const MODULUS: u32 = 1_000_000_007;

    pub fn new(n: u32) -> Self {
        Self {
            value: n % Self::MODULUS,
        }
    }

    pub fn pow(self, mut n: u32) -> Mint {
        let mut t = Mint::new(1);
        let mut s = self;
        while n > 0 {
            if n & 1 == 1 {
                t *= s;
            }
            s *= s;
            n >>= 1;
        }
        t
    }

    pub fn inv(self) -> Mint {
        assert!(self.value > 0);
        self.pow(Self::MODULUS - 2)
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let l = Mint::new(10);
        let r = Mint::new(15);
        let ans = l + r;
        assert_eq!(ans.value, 25);

        let l = Mint::new(Mint::MODULUS - 10);
        let r = Mint::new(15);
        let ans = l + r;
        assert_eq!(ans.value, 5);
    }

    #[test]
    fn test_add_assign() {
        let mut l = Mint::new(10);
        let r = Mint::new(15);
        l += r;
        assert_eq!(l.value, 25);

        let mut l = Mint::new(Mint::MODULUS - 10);
        let r = Mint::new(15);
        l += r;
        assert_eq!(l.value, 5);
    }

    #[test]
    fn test_sub() {
        let l = Mint::new(15);
        let r = Mint::new(10);
        let ans = l - r;
        assert_eq!(ans.value, 5);

        let l = Mint::new(Mint::MODULUS - 10);
        let r = Mint::new(Mint::MODULUS - 15);
        let ans = l - r;
        assert_eq!(ans.value, 5);

        let l = Mint::new(Mint::MODULUS - 15);
        let r = Mint::new(Mint::MODULUS - 10);
        let ans = l - r;
        assert_eq!(ans.value, Mint::MODULUS - 5);
    }

    #[test]
    fn test_sub_assign() {
        let mut l = Mint::new(15);
        let r = Mint::new(10);
        l -= r;
        assert_eq!(l.value, 5);

        let mut l = Mint::new(Mint::MODULUS - 10);
        let r = Mint::new(Mint::MODULUS - 15);
        l -= r;
        assert_eq!(l.value, 5);

        let mut l = Mint::new(Mint::MODULUS - 15);
        let r = Mint::new(Mint::MODULUS - 10);
        l -= r;
        assert_eq!(l.value, Mint::MODULUS - 5);
    }

    #[test]
    fn test_mul() {
        let l = Mint::new(5);
        let r = Mint::new(10);
        let ans = l * r;
        assert_eq!(ans.value, 50);

        let l = Mint::new(Mint::MODULUS - 5);
        let r = Mint::new(Mint::MODULUS - 10);
        let ans = l * r;
        assert_eq!(ans.value, 50);
    }

    #[test]
    fn test_mul_assign() {
        let mut l = Mint::new(5);
        let r = Mint::new(10);
        l *= r;
        assert_eq!(l.value, 50);

        let mut l = Mint::new(Mint::MODULUS - 5);
        let r = Mint::new(Mint::MODULUS - 10);
        l *= r;
        assert_eq!(l.value, 50);
    }

    #[test]
    fn test_neg() {
        let v = Mint::new(0);
        let ans = -v;
        assert_eq!(ans.value, 0);

        let v = Mint::new(100);
        let ans = -v;
        assert_eq!(ans.value, Mint::MODULUS - 100);
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{}", Mint::new(1)), "1");
        assert_eq!(format!("{}", Mint::new(Mint::MODULUS - 1)), "1000000006");
    }

    #[test]
    fn test_from_usize() {
        assert_eq!(Mint::from(500).value, 500);
        assert_eq!(Mint::from((Mint::MODULUS + 1) as usize).value, 1);
        assert_eq!(Mint::from(std::u32::MAX as usize + 1).value, 294_967_268);
    }

    #[test]
    fn test_partial_eq_self() {
        assert!(Mint::new(10) == Mint::new(10));
        assert!(Mint::new(10) != Mint::new(7));
    }

    #[test]
    fn test_partial_eq_u32() {
        assert!(Mint::new(10) == 10);
        assert!(Mint::new(10) != 7);
    }

    #[test]
    fn test_new() {
        assert_eq!(Mint::new(10).value, 10);
        assert_eq!(Mint::new(std::u32::MAX).value, 294_967_267);
    }

    #[test]
    fn test_pow() {
        let l = Mint::new(1);
        let ans = l.pow(100);
        assert_eq!(ans.value, 1);

        let l = Mint::new(5);
        let ans = l.pow(3);
        assert_eq!(ans.value, 125);

        let l = Mint::new(294_967_267);
        let ans = l.pow(294_967_267);
        assert_eq!(ans.value, 756_289_898);
    }

    #[test]
    fn test_inv() {
        let l = Mint::new(1);
        let ans = l.inv();
        assert_eq!(ans.value, 1);

        let l = Mint::new(500_000_004);
        let ans = l.inv();
        assert_eq!(ans.value, 2);
    }

    #[test]
    fn test_value() {
        let m = Mint::new(1);
        assert_eq!(m.value(), 1);

        let m = Mint::new(Mint::MODULUS);
        assert_eq!(m.value(), 0);

        let m = Mint::new(Mint::MODULUS - 1);
        assert_eq!(m.value(), 1_000_000_006);
    }
}
