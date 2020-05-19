#[derive(Clone, Copy)]
pub struct ModInt {
    value: u32,
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: ModInt) -> Self::Output {
        let mut d = self.value + rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        ModInt::new(d)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: ModInt) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: ModInt) -> Self::Output {
        let mut d = self.value + Self::MODULUS - rhs.value;
        if d >= Self::MODULUS {
            d -= Self::MODULUS;
        }
        ModInt::new(d)
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: ModInt) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: ModInt) -> Self::Output {
        let d = self.value as u64 * rhs.value as u64 % Self::MODULUS as u64;

        ModInt::new(d as u32)
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: ModInt) {
        *self = *self * rhs;
    }
}

impl std::ops::Neg for ModInt {
    type Output = ModInt;

    fn neg(self) -> Self::Output {
        let d = match self.value {
            0 => 0,
            _ => Self::MODULUS - self.value,
        };

        ModInt::new(d)
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<usize> for ModInt {
    fn from(val: usize) -> Self {
        let d = (val % Self::MODULUS as usize) as u32;

        ModInt::new(d)
    }
}

impl PartialEq<Self> for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u32> for ModInt {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}

#[allow(dead_code)]
impl ModInt {
    // to change modulus, rewrite this value
    // (modulus should be a prime number)
    pub const MODULUS: u32 = 1_000_000_007;

    pub fn new(n: u32) -> Self {
        Self {
            value: n % Self::MODULUS,
        }
    }

    pub fn zero() -> ModInt {
        Self::new(0)
    }

    pub fn one() -> ModInt {
        Self::new(1)
    }

    pub fn pow(self, mut n: u32) -> ModInt {
        let mut t = ModInt::one();
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

    pub fn inv(self) -> ModInt {
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
        let l = ModInt::new(10);
        let r = ModInt::new(15);
        let ans = l + r;
        assert_eq!(ans.value, 25);

        let l = ModInt::new(ModInt::MODULUS - 10);
        let r = ModInt::new(15);
        let ans = l + r;
        assert_eq!(ans.value, 5);
    }

    #[test]
    fn test_add_assign() {
        let mut l = ModInt::new(10);
        let r = ModInt::new(15);
        l += r;
        assert_eq!(l.value, 25);

        let mut l = ModInt::new(ModInt::MODULUS - 10);
        let r = ModInt::new(15);
        l += r;
        assert_eq!(l.value, 5);
    }

    #[test]
    fn test_sub() {
        let l = ModInt::new(15);
        let r = ModInt::new(10);
        let ans = l - r;
        assert_eq!(ans.value, 5);

        let l = ModInt::new(ModInt::MODULUS - 10);
        let r = ModInt::new(ModInt::MODULUS - 15);
        let ans = l - r;
        assert_eq!(ans.value, 5);

        let l = ModInt::new(ModInt::MODULUS - 15);
        let r = ModInt::new(ModInt::MODULUS - 10);
        let ans = l - r;
        assert_eq!(ans.value, ModInt::MODULUS - 5);
    }

    #[test]
    fn test_sub_assign() {
        let mut l = ModInt::new(15);
        let r = ModInt::new(10);
        l -= r;
        assert_eq!(l.value, 5);

        let mut l = ModInt::new(ModInt::MODULUS - 10);
        let r = ModInt::new(ModInt::MODULUS - 15);
        l -= r;
        assert_eq!(l.value, 5);

        let mut l = ModInt::new(ModInt::MODULUS - 15);
        let r = ModInt::new(ModInt::MODULUS - 10);
        l -= r;
        assert_eq!(l.value, ModInt::MODULUS - 5);
    }

    #[test]
    fn test_mul() {
        let l = ModInt::new(5);
        let r = ModInt::new(10);
        let ans = l * r;
        assert_eq!(ans.value, 50);

        let l = ModInt::new(ModInt::MODULUS - 5);
        let r = ModInt::new(ModInt::MODULUS - 10);
        let ans = l * r;
        assert_eq!(ans.value, 50);
    }

    #[test]
    fn test_mul_assign() {
        let mut l = ModInt::new(5);
        let r = ModInt::new(10);
        l *= r;
        assert_eq!(l.value, 50);

        let mut l = ModInt::new(ModInt::MODULUS - 5);
        let r = ModInt::new(ModInt::MODULUS - 10);
        l *= r;
        assert_eq!(l.value, 50);
    }

    #[test]
    fn test_neg() {
        let v = ModInt::new(0);
        let ans = -v;
        assert_eq!(ans.value, 0);

        let v = ModInt::new(100);
        let ans = -v;
        assert_eq!(ans.value, ModInt::MODULUS - 100);
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{}", ModInt::new(1)), "1");
        assert_eq!(
            format!("{}", ModInt::new(ModInt::MODULUS - 1)),
            "1000000006"
        );
    }

    #[test]
    fn test_from_usize() {
        assert_eq!(ModInt::from(500).value, 500);
        assert_eq!(ModInt::from((ModInt::MODULUS + 1) as usize).value, 1);
        assert_eq!(ModInt::from(std::u32::MAX as usize + 1).value, 294_967_268);
    }

    #[test]
    fn test_partial_eq_self() {
        assert!(ModInt::new(10) == ModInt::new(10));
        assert!(ModInt::new(10) != ModInt::new(7));
    }

    #[test]
    fn test_partial_eq_u32() {
        assert!(ModInt::new(10) == 10);
        assert!(ModInt::new(10) != 7);
    }

    #[test]
    fn test_new() {
        assert_eq!(ModInt::new(10).value, 10);
        assert_eq!(ModInt::new(std::u32::MAX).value, 294_967_267);
    }

    #[test]
    fn test_zero() {
        assert_eq!(ModInt::zero().value, 0);
    }

    #[test]
    fn test_one() {
        assert_eq!(ModInt::one().value, 1);
    }

    #[test]
    fn test_pow() {
        let l = ModInt::new(1);
        let ans = l.pow(100);
        assert_eq!(ans.value, 1);

        let l = ModInt::new(5);
        let ans = l.pow(3);
        assert_eq!(ans.value, 125);

        let l = ModInt::new(294_967_267);
        let ans = l.pow(294_967_267);
        assert_eq!(ans.value, 756_289_898);
    }

    #[test]
    fn test_inv() {
        let l = ModInt::new(1);
        let ans = l.inv();
        assert_eq!(ans.value, 1);

        let l = ModInt::new(500_000_004);
        let ans = l.inv();
        assert_eq!(ans.value, 2);
    }

    #[test]
    fn test_value() {
        let m = ModInt::new(1);
        assert_eq!(m.value(), 1);

        let m = ModInt::new(ModInt::MODULUS);
        assert_eq!(m.value(), 0);

        let m = ModInt::new(ModInt::MODULUS - 1);
        assert_eq!(m.value(), 1_000_000_006);
    }
}
