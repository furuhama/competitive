// MOD should be a prime number
const MOD: u32 = 1_000_000_007;

#[derive(Clone, Copy)]
pub struct ModInt {
    pub value: u32,
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: ModInt) -> Self::Output {
        let mut d = self.value + rhs.value;
        if d >= MOD {
            d -= MOD;
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
        let mut d = self.value + MOD - rhs.value;
        if d >= MOD {
            d -= MOD;
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
        let d = self.value as u64 * rhs.value as u64 % MOD as u64;

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
            _ => MOD - self.value,
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
    fn from(val: usize) -> ModInt {
        let d = (val % MOD as usize) as u32;

        ModInt::new(d)
    }
}

#[allow(dead_code)]
impl ModInt {
    pub fn new(n: u32) -> Self {
        Self { value: n % MOD }
    }

    pub fn zero() -> ModInt {
        Self { value: 0 }
    }

    pub fn one() -> ModInt {
        Self { value: 1 }
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
        self.pow(MOD - 2)
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

        let l = ModInt::new(MOD - 10);
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

        let mut l = ModInt::new(MOD - 10);
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

        let l = ModInt::new(MOD - 10);
        let r = ModInt::new(MOD - 15);
        let ans = l - r;
        assert_eq!(ans.value, 5);

        let l = ModInt::new(MOD - 15);
        let r = ModInt::new(MOD - 10);
        let ans = l - r;
        assert_eq!(ans.value, MOD - 5);
    }

    #[test]
    fn test_sub_assign() {
        let mut l = ModInt::new(15);
        let r = ModInt::new(10);
        l -= r;
        assert_eq!(l.value, 5);

        let mut l = ModInt::new(MOD - 10);
        let r = ModInt::new(MOD - 15);
        l -= r;
        assert_eq!(l.value, 5);

        let mut l = ModInt::new(MOD - 15);
        let r = ModInt::new(MOD - 10);
        l -= r;
        assert_eq!(l.value, MOD - 5);
    }

    #[test]
    fn test_mul() {
        let l = ModInt::new(5);
        let r = ModInt::new(10);
        let ans = l * r;
        assert_eq!(ans.value, 50);

        let l = ModInt::new(MOD - 5);
        let r = ModInt::new(MOD - 10);
        let ans = l * r;
        assert_eq!(ans.value, 50);
    }

    #[test]
    fn test_mul_assign() {
        let mut l = ModInt::new(5);
        let r = ModInt::new(10);
        l *= r;
        assert_eq!(l.value, 50);

        let mut l = ModInt::new(MOD - 5);
        let r = ModInt::new(MOD - 10);
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
        assert_eq!(ans.value, MOD - 100);
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{}", ModInt::new(1)), "1");
        assert_eq!(format!("{}", ModInt::new(MOD - 1)), "1000000006");
    }

    #[test]
    fn test_from_usize() {
        assert_eq!(ModInt::from(500).value, 500);
        assert_eq!(ModInt::from((MOD + 1) as usize).value, 1);
        assert_eq!(ModInt::from(std::u32::MAX as usize + 1).value, 294_967_268);
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
}
