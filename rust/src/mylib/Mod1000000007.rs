use cargo_snippet::snippet;

#[snippet("mod_1000000007")]
const MOD_1000000007: LongLong = 1000000007;

#[snippet("mod_1000000007")]
#[derive(Copy, Clone, Debug)]
struct Mod1000000007 {
    v: LongLong,
}

#[snippet("mod_1000000007")]
impl Mod1000000007 {
    pub fn new(v: LongLong) -> Self {
        let vv = v % MOD_1000000007;
        return Self {
            v: if vv >= 0 { vv } else { vv + MOD_1000000007 },
        };
    }
}

#[snippet("mod_1000000007")]
impl From<Int> for Mod1000000007 {
    fn from(v: Int) -> Self {
        return Self::new(v as LongLong);
    }
}

#[snippet("mod_1000000007")]
impl From<UnsignedInt> for Mod1000000007 {
    fn from(v: UnsignedInt) -> Self {
        return Self::new(v as LongLong);
    }
}

#[snippet("mod_1000000007")]
impl From<LongLong> for Mod1000000007 {
    fn from(v: LongLong) -> Self {
        return Self::new(v);
    }
}

#[snippet("mod_1000000007")]
impl From<UnsignedLongLong> for Mod1000000007 {
    fn from(v: UnsignedLongLong) -> Self {
        let v = v % (MOD_1000000007 as UnsignedLongLong);
        let v = v as LongLong;
        return Self::new(v);
    }
}

#[snippet("mod_1000000007")]
impl std::fmt::Display for Mod1000000007 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.v)?;
        Ok(())
    }
}

#[snippet("mod_1000000007")]
impl std::ops::Add for Mod1000000007 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self {
            v: if self.v + other.v >= MOD_1000000007 {
                self.v + other.v - MOD_1000000007
            } else {
                self.v + other.v
            },
        };
    }
}

#[snippet("mod_1000000007")]
impl std::ops::AddAssign for Mod1000000007 {
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v;
        if self.v >= MOD_1000000007 {
            self.v -= MOD_1000000007;
        }
    }
}

#[snippet("mod_1000000007")]
impl std::ops::Sub for Mod1000000007 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self {
            v: if self.v - other.v < 0 {
                self.v - other.v + MOD_1000000007
            } else {
                self.v - other.v
            },
        };
    }
}

#[snippet("mod_1000000007")]
impl std::ops::SubAssign for Mod1000000007 {
    fn sub_assign(&mut self, rhs: Self) {
        self.v -= rhs.v;
        if self.v < 0 {
            self.v += MOD_1000000007;
        }
    }
}

#[snippet("mod_1000000007")]
impl std::ops::Mul for Mod1000000007 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        return Self {
            v: (self.v * other.v) % MOD_1000000007,
        };
    }
}

#[snippet("mod_1000000007")]
impl std::ops::MulAssign for Mod1000000007 {
    fn mul_assign(&mut self, rhs: Self) {
        self.v *= rhs.v;
        self.v %= MOD_1000000007;
    }
}

#[snippet("mod_1000000007")]
#[snippet(include = "pow")]
impl std::ops::Div for Mod1000000007 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        return self * pow(&rhs, MOD_1000000007 - 2).unwrap();
    }
}

#[snippet("mod_1000000007")]
#[snippet(include = "pow")]
impl std::ops::DivAssign for Mod1000000007 {
    fn div_assign(&mut self, rhs: Self) {
        let inv = pow(&rhs, MOD_1000000007 - 2).unwrap();
        *self *= inv;
    }
}

#[snippet("mod_1000000007")]
#[derive(Debug, PartialEq, Eq)]
struct ParseMod1000000007Error;

#[snippet("mod_1000000007")]
impl std::str::FromStr for Mod1000000007 {
    type Err = ParseMod1000000007Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse::<LongLong>().map_err(|_| ParseMod1000000007Error)?;
        return Ok(Self {
            v: v % MOD_1000000007,
        });
    }
}
