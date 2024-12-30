use std::ops::{
    Neg,
    Add,
    Sub,
    Mul,
    Div,
};
use std::cmp::{
    PartialEq,
    PartialOrd,
};
use std::fmt::Display;
use num::integer::gcd;

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    num: isize,
    denom: isize,
}

impl Rational {
    pub fn new(num: isize, denom: isize) -> Rational {
        Rational {
            num: num,
            denom: denom
        }
    }

    pub fn simplify(self) -> Rational {
        let g = gcd(self.num, self.denom);
        let sign = if self.denom < 0 { -1 } else { 1 };
        Rational {
            num: self.num / g * sign,
            denom: self.denom / g * sign,
        }
    }

    pub fn to_f64(self) -> f64 {
        self.num as f64 / self.denom as f64
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.denom)
    }
}

impl Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational {
            num: -self.num,
            denom: self.denom
        }
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, rhs: Rational) -> Rational {
        Rational {
            num: self.num * rhs.denom + self.denom * rhs.num,
            denom: self.denom * rhs.denom
        }.simplify()
    }
}

impl Add<isize> for Rational {
    type Output = Rational;
    fn add(self, rhs: isize) -> Rational {
        Rational {
            num: self.num + self.denom * rhs,
            denom: self.denom
        }.simplify()
    }
}

impl Add<Rational> for isize {
    type Output = Rational;
    fn add(self, rhs: Rational) -> Rational {
        Rational {
            num: self * rhs.denom + rhs.num,
            denom: rhs.denom
        }.simplify()
    }
}

impl Sub for Rational {
    type Output = Rational;
    fn sub(self, rhs: Rational) -> Rational {
        Rational {
            num: self.num * rhs.denom - self.denom * rhs.num,
            denom: self.denom * rhs.denom
        }.simplify()
    }
}

impl Sub<isize> for Rational {
    type Output = Rational;
    fn sub(self, rhs: isize) -> Rational {
        Rational {
            num: self.num - self.denom * rhs,
            denom: self.denom
        }.simplify()
    }
}

impl Sub<Rational> for isize {
    type Output = Rational;
    fn sub(self, rhs: Rational) -> Rational {
        Rational {
            num: self * rhs.denom - rhs.num,
            denom: rhs.denom
        }.simplify()
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Rational) -> Rational {
        Rational {
            num: self.num * rhs.num,
            denom: self.denom * rhs.denom
        }.simplify()
    }
}

impl Mul<isize> for Rational {
    type Output = Rational;
    fn mul(self, rhs: isize) -> Rational {
        Rational {
            num: self.num * rhs,
            denom: self.denom
        }.simplify()
    }
}

impl Mul<Rational> for isize {
    type Output = Rational;
    fn mul(self, rhs: Rational) -> Rational {
        Rational {
            num: self * rhs.num,
            denom: rhs.denom
        }.simplify()
    }
}

impl Div for Rational {
    type Output = Rational;
    fn div(self, rhs: Rational) -> Rational {
        Rational {
            num: self.num * rhs.denom,
            denom: self.denom * rhs.num
        }.simplify()
    }
}

impl Div<isize> for Rational {
    type Output = Rational;
    fn div(self, rhs: isize) -> Rational {
        Rational {
            num: self.num,
            denom: self.denom * rhs
        }.simplify()
    }
}

impl Div<Rational> for isize {
    type Output = Rational;
    fn div(self, rhs: Rational) -> Rational {
        Rational {
            num: self * rhs.denom,
            denom: rhs.num
        }.simplify()
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        let a = self.simplify();
        let b = other.simplify();
        a.num == b.num && a.denom == b.denom
    }
}

impl PartialEq<isize> for Rational {
    fn eq(&self, other: &isize) -> bool {
        *other * self.denom == self.num
    }
}

impl PartialEq<Rational> for isize {
    fn eq(&self, other: &Rational) -> bool {
        self * other.denom == other.num
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Rational) -> Option<std::cmp::Ordering> {
        let a = *self - *other;
        if a.num * a.denom < 0 {
            Some(std::cmp::Ordering::Less)
        } else if a.num * a.denom > 0 {
            Some(std::cmp::Ordering::Greater)
        } else if a.num == 0 {
            Some(std::cmp::Ordering::Equal)       
        } else {
            None
        }
    }
}

impl PartialOrd<isize> for Rational {
    fn partial_cmp(&self, other: &isize) -> Option<std::cmp::Ordering> {
        let a = *self - *other;
        if a.num * a.denom < 0 {
            Some(std::cmp::Ordering::Less)
        } else if a.num * a.denom > 0 {
            Some(std::cmp::Ordering::Greater)
        } else if a.num == 0 {
            Some(std::cmp::Ordering::Equal)       
        } else {
            None
        }
    }
}

impl PartialOrd<Rational> for isize {
    fn partial_cmp(&self, other: &Rational) -> Option<std::cmp::Ordering> {
        let a = *self - *other;
        if a.num * a.denom < 0 {
            Some(std::cmp::Ordering::Less)
        } else if a.num * a.denom > 0 {
            Some(std::cmp::Ordering::Greater)
        } else if a.num == 0 {
            Some(std::cmp::Ordering::Equal)       
        } else {
            None
        }
    }
}