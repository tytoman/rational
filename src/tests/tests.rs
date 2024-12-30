#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = Rational::new(4, 6);
        assert_eq!(r.num, 4);
        assert_eq!(r.denom, 6);
    }

    #[test]
    fn test_simplify() {
        let r = Rational::new(4, 6).simplify();
        assert_eq!(r.num, 2);
        assert_eq!(r.denom, 3);
    }

    #[test]
    fn test_to_f64() {
        let r = Rational::new(1, 2);
        assert_eq!(r.to_f64(), 0.5);
    }

    #[test]
    fn test_display() {
        let r = Rational::new(4, 6).simplify();
        assert_eq!(format!("{}", r), "2/3");
    }

    #[test]
    fn test_neg() {
        let r = -Rational::new(2, 3);
        assert_eq!(r.num, -2);
        assert_eq!(r.denom, 3);
    }

    #[test]
    fn test_add() {
        let r1 = Rational::new(1, 3);
        let r2 = Rational::new(1, 6);
        assert_eq!(r1 + r2, Rational::new(1, 2));
    }

    #[test]
    fn test_sub() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 6);
        assert_eq!(r1 - r2, Rational::new(1, 3));
    }

    #[test]
    fn test_mul() {
        let r1 = Rational::new(2, 3);
        let r2 = Rational::new(3, 4);
        assert_eq!(r1 * r2, Rational::new(1, 2));
    }

    #[test]
    fn test_div() {
        let r1 = Rational::new(2, 3);
        let r2 = Rational::new(3, 4);
        assert_eq!(r1 / r2, Rational::new(8, 9));
    }

    #[test]
    fn test_partial_eq() {
        let r1 = Rational::new(2, 4);
        let r2 = Rational::new(1, 2);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_partial_ord() {
        let r1 = Rational::new(1, 3);
        let r2 = Rational::new(1, 2);
        assert!(r1 < r2);
    }
}
