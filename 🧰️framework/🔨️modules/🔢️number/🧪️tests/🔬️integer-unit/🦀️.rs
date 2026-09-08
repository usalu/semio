mod tests {
    use super::*;
    use std::str::FromStr;

    fn i(s: &str) -> Integer {
        Integer::from_str(s).unwrap()
    }

    #[test]
    fn string_roundtrip_positive_and_negative() {
        assert_eq!(i("12345").to_decimal(), "12345");
        assert_eq!(i("-12345").to_decimal(), "-12345");
        assert_eq!(i("0").to_decimal(), "0");
        assert_eq!(i("-0").to_decimal(), "0");
    }

    #[test]
    fn add_sub_sign_handling() {
        assert_eq!(i("5").add(&i("-3")), i("2"));
        assert_eq!(i("-5").add(&i("3")), i("-2"));
        assert_eq!(i("-5").add(&i("-3")), i("-8"));
        assert_eq!(i("5").sub(&i("8")), i("-3"));
    }

    #[test]
    fn mul_sign_handling() {
        assert_eq!(i("-3").mul(&i("4")), i("-12"));
        assert_eq!(i("-3").mul(&i("-4")), i("12"));
        assert_eq!(i("0").mul(&i("-4")), i("0"));
    }

    #[test]
    fn div_rem_truncated_matches_rust_semantics() {
        assert_eq!(i("7").div_rem(&i("2")), (i("3"), i("1")));
        assert_eq!(i("-7").div_rem(&i("2")), (i("-3"), i("-1")));
        assert_eq!(i("7").div_rem(&i("-2")), (i("-3"), i("1")));
        assert_eq!(i("-7").div_rem(&i("-2")), (i("3"), i("-1")));
    }

    #[test]
    fn div_rem_floor_matches_math_floor_division() {
        assert_eq!(i("-7").div_rem_floor(&i("2")), (i("-4"), i("1")));
        assert_eq!(i("7").div_rem_floor(&i("-2")), (i("-4"), i("-1")));
    }

    #[test]
    fn div_rem_euclid_remainder_always_nonnegative() {
        for (a, b) in [("-7", "2"), ("7", "-2"), ("-7", "-2"), ("7", "2")] {
            let (q, r) = i(a).div_rem_euclid(&i(b));
            assert!(!r.is_negative());
            assert_eq!(q.mul(&i(b)).add(&r), i(a));
        }
    }

    #[test]
    fn extended_gcd_satisfies_bezout_identity() {
        for (a, b) in [(240, 46), (-240, 46), (240, -46), (17, 5), (0, 5)] {
            let ia = Integer::from_i64(a);
            let ib = Integer::from_i64(b);
            let (g, x, y) = ia.extended_gcd(&ib);
            assert_eq!(x.mul(&ia).add(&y.mul(&ib)), g);
            assert!(!g.is_negative());
        }
    }

    #[test]
    fn ordering_across_signs() {
        assert!(i("-5") < i("-3"));
        assert!(i("-1") < i("0"));
        assert!(i("0") < i("1"));
        assert!(i("-100") < i("1"));
    }

    #[test]
    fn checked_isqrt_rejects_negative() {
        assert!(i("-4").checked_isqrt().is_none());
        assert_eq!(i("4").checked_isqrt().unwrap(), i("2"));
    }

    #[test]
    fn nth_root_odd_accepts_negative() {
        assert_eq!(i("-27").nth_root(3).unwrap(), i("-3"));
        assert!(i("-4").nth_root(2).is_none());
    }

    #[test]
    fn ring_trait_impl_matches_inherent_methods() {
        let a = i("7");
        let b = i("3");
        assert_eq!(Ring::add(&a, &b), a.add(&b));
        assert_eq!(Ring::mul(&a, &b), a.mul(&b));
        assert_eq!(IntegralDomain::exact_div(&i("12"), &i("4")), Some(i("3")));
        assert_eq!(IntegralDomain::exact_div(&i("12"), &i("5")), None);
    }
}
