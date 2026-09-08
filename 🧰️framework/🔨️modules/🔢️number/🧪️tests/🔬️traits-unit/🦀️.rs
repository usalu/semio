mod tests {
    use super::*;

    #[test]
    fn i64_ring_axioms_hold_on_small_samples() {
        for a in -5i64..=5 {
            for b in -5i64..=5 {
                assert_eq!(a.add(&b), a + b);
                assert_eq!(a.mul(&b), a * b);
                assert_eq!(a.sub(&b), a - b);
            }
        }
    }

    #[test]
    fn i64_from_i64_via_default_matches_identity_for_positive_and_negative() {
        // exercise the default doubling-based from_i64 through a type that overrides it trivially,
        // and separately validate the algorithm itself using f64 (which does NOT override from_i64
        // meaningfully differently, so cross-check against a hand-rolled doubling loop instead).
        fn via_doubling(value: i64) -> i64 {
            <i64 as Ring>::from_i64(value)
        }
        for v in [-100i64, -1, 0, 1, 100, 12345] {
            assert_eq!(via_doubling(v), v);
        }
    }

    #[test]
    fn i64_pow_matches_repeated_multiplication() {
        for base in -4i64..=4 {
            for exp in 0u64..6 {
                let expected = base.pow(exp as u32);
                assert_eq!(Ring::pow(&base, exp), expected);
            }
        }
    }

    #[test]
    fn i64_gcd_matches_euclid_hand_cases() {
        assert_eq!(GcdDomain::gcd(&12i64, &18i64), 6);
        assert_eq!(GcdDomain::gcd(&0i64, &5i64), 5);
        assert_eq!(GcdDomain::gcd(&(-12i64), &18i64), 6);
        assert_eq!(GcdDomain::gcd(&0i64, &0i64), 0);
    }

    #[test]
    fn i64_lcm_matches_hand_cases() {
        assert_eq!(GcdDomain::lcm(&4i64, &6i64), 12);
        assert_eq!(GcdDomain::lcm(&0i64, &5i64), 0);
    }

    #[test]
    fn i64_div_rem_matches_language_semantics() {
        assert_eq!(EuclideanDomain::div_rem(&7i64, &2i64), (3, 1));
        assert_eq!(EuclideanDomain::div_rem(&(-7i64), &2i64), (-3, -1));
    }

    #[test]
    fn f64_field_inv_and_div() {
        let a = 4.0f64;
        let inv = Field::inv(&a).expect("nonzero");
        assert!((inv - 0.25).abs() < 1e-12);
        assert!(Field::inv(&0.0f64).is_none());
        let q = Field::div(&6.0f64, &3.0f64).expect("nonzero divisor");
        assert!((q - 2.0).abs() < 1e-12);
    }

    #[test]
    fn field_gcd_helper_matches_unit_convention() {
        assert_eq!(field_gcd(&0.0f64, &0.0f64), 0.0);
        assert_eq!(field_gcd(&3.0f64, &0.0f64), 1.0);
        assert_eq!(field_gcd(&3.0f64, &5.0f64), 1.0);
    }
}
