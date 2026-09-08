mod tests {
    use super::*;

    #[test]
    fn from_rational_contains_the_exact_value() {
        let r = Rational::from_i64(1, 3).unwrap();
        let iv = Interval::from_rational(&r);
        assert!(iv.lo <= r.to_f64() && r.to_f64() <= iv.hi);
        // Certified containment check via exact rational comparison against the bounds.
        let lo_r = Rational::from_f64(iv.lo).unwrap();
        let hi_r = Rational::from_f64(iv.hi).unwrap();
        assert!(lo_r <= r && r <= hi_r);
    }

    #[test]
    fn arithmetic_hand_cases() {
        let a = Interval::point(2.0);
        let b = Interval::point(3.0);
        let sum = a.add(&b);
        assert!(sum.contains(5.0));
        let prod = a.mul(&b);
        assert!(prod.contains(6.0));
    }

    #[test]
    fn division_by_zero_straddling_interval_is_none() {
        let a = Interval::new(-1.0, 1.0);
        let b = Interval::point(1.0);
        assert!(b.div(&a).is_none());
    }

    #[test]
    fn sqrt_of_negative_is_none() {
        assert!(Interval::point(-1.0).sqrt().is_none());
        let s = Interval::point(4.0).sqrt().unwrap();
        assert!(s.contains(2.0));
    }

    #[test]
    fn sign_detection() {
        assert_eq!(Interval::new(1.0, 2.0).sign(), Some(std::cmp::Ordering::Greater));
        assert_eq!(Interval::new(-2.0, -1.0).sign(), Some(std::cmp::Ordering::Less));
        assert_eq!(Interval::new(-1.0, 1.0).sign(), None);
    }

    #[test]
    fn hull_and_intersect() {
        let a = Interval::new(0.0, 2.0);
        let b = Interval::new(1.0, 3.0);
        assert_eq!(a.hull(&b), Interval::new(0.0, 3.0));
        let inter = a.intersect(&b).unwrap();
        assert_eq!(inter, Interval::new(1.0, 2.0));
        assert!(Interval::new(0.0, 1.0).intersect(&Interval::new(2.0, 3.0)).is_none());
    }

    #[test]
    fn powi_matches_repeated_multiplication() {
        let a = Interval::point(2.0);
        let p = a.powi(10);
        assert!(p.contains(1024.0));
    }
}
