mod tests {
    use super::*;

    fn r(n: i64, d: i64) -> Rational {
        Rational::from_i64(n, d).unwrap()
    }

    #[test]
    fn normalization_reduces_to_lowest_terms() {
        assert_eq!(r(4, 8), r(1, 2));
        assert_eq!(r(-4, 8), r(-1, 2));
        assert_eq!(r(4, -8), r(-1, 2));
    }

    #[test]
    fn zero_denominator_is_rejected() {
        assert!(Rational::from_i64(1, 0).is_none());
    }

    #[test]
    fn field_ops_hand_cases() {
        assert_eq!(r(1, 2).add(&r(1, 3)), r(5, 6));
        assert_eq!(r(1, 2).mul(&r(2, 3)), r(1, 3));
        assert_eq!(r(1, 2).sub(&r(1, 3)), r(1, 6));
        assert_eq!(r(2, 3).div(&r(4, 9)).unwrap(), r(3, 2));
    }

    #[test]
    fn inv_of_zero_is_none() {
        assert!(Rational::zero().inv().is_none());
    }

    #[test]
    fn floor_ceil_trunc_hand_cases() {
        assert_eq!(r(7, 2).floor(), Integer::from_i64(3));
        assert_eq!(r(-7, 2).floor(), Integer::from_i64(-4));
        assert_eq!(r(7, 2).ceil(), Integer::from_i64(4));
        assert_eq!(r(-7, 2).ceil(), Integer::from_i64(-3));
        assert_eq!(r(-7, 2).trunc(), Integer::from_i64(-3));
    }

    #[test]
    fn ordering_via_cross_multiplication() {
        assert!(r(1, 3) < r(1, 2));
        assert!(r(-1, 2) < r(1, 3));
        assert_eq!(r(2, 4), r(1, 2));
    }

    #[test]
    fn to_f64_matches_expected_for_simple_fractions() {
        assert!((r(1, 2).to_f64() - 0.5).abs() < 1e-15);
        assert!((r(1, 3).to_f64() - (1.0 / 3.0)).abs() < 1e-15);
        assert!((r(-22, 7).to_f64() - (-22.0 / 7.0)).abs() < 1e-12);
    }

    #[test]
    fn from_f64_roundtrips_exactly() {
        for v in [0.5, 0.25, 1.0 / 3.0, 2.0, -7.5, 123456.0] {
            let rat = Rational::from_f64(v).unwrap();
            assert!((rat.to_f64() - v).abs() < 1e-12, "roundtrip mismatch for {v}");
        }
    }

    #[test]
    fn continued_fraction_reconstructs_convergents() {
        let x = r(355, 113); // close approximation of pi
        let convergents = x.convergents();
        assert_eq!(*convergents.last().unwrap().numer(), *x.numer());
        assert_eq!(*convergents.last().unwrap().denom(), *x.denom());
    }

    #[test]
    fn best_approximation_respects_denominator_bound() {
        let pi_ish = Rational::from_f64(std::f64::consts::PI).unwrap();
        let approx = pi_ish.best_approximation(&Natural::from_u64(1000));
        assert!(*approx.denom() <= Natural::from_u64(1000));
        assert!((approx.to_f64() - std::f64::consts::PI).abs() < 0.01);
    }

    #[test]
    fn field_trait_impl_matches_inherent_methods() {
        let a = r(3, 4);
        let b = r(1, 2);
        assert_eq!(Ring::add(&a, &b), a.add(&b));
        assert_eq!(Field::inv(&a).unwrap(), a.inv().unwrap());
    }
}
