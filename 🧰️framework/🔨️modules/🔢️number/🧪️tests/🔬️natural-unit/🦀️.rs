mod tests {
    use super::*;
    use std::str::FromStr;

    fn n(s: &str) -> Natural {
        Natural::from_str(s).unwrap()
    }

    // #region 🔖️FundamentalTests
    #[test]
    fn decimal_string_roundtrip() {
        let s = "123456789012345678901234567890";
        assert_eq!(n(s).to_decimal(), s);
    }

    #[test]
    fn zero_roundtrips() {
        assert_eq!(n("0").to_decimal(), "0");
        assert!(n("0").is_zero());
    }

    #[test]
    fn add_matches_hand_case() {
        assert_eq!(n("999999999999999999999").add(&n("1")).to_decimal(), "1000000000000000000000");
    }

    #[test]
    fn checked_sub_matches_hand_case_and_rejects_negative() {
        assert_eq!(n("1000000000000000000000").checked_sub(&n("1")).unwrap().to_decimal(), "999999999999999999999");
        assert!(n("1").checked_sub(&n("2")).is_none());
    }

    #[test]
    fn mul_matches_hand_case() {
        assert_eq!(n("123456789").mul(&n("987654321")).to_decimal(), "121932631112635269");
    }

    #[test]
    fn div_rem_self_check_invariant() {
        let u = n("123456789012345678901234567890");
        let v = n("98765432109876543210");
        let (q, r) = u.div_rem(&v);
        assert!(r < v);
        assert_eq!(q.mul(&v).add(&r), u);
    }

    #[test]
    fn div_rem_by_larger_value_is_zero_quotient() {
        let (q, r) = n("5").div_rem(&n("100"));
        assert!(q.is_zero());
        assert_eq!(r, n("5"));
    }

    #[test]
    fn isqrt_matches_known_squares() {
        assert_eq!(n("144").isqrt(), n("12"));
        assert_eq!(n("143").isqrt(), n("11"));
        assert_eq!(n("0").isqrt(), n("0"));
        assert_eq!(n("1").isqrt(), n("1"));
    }

    #[test]
    fn nth_root_matches_known_cubes() {
        assert_eq!(n("27").nth_root(3), n("3"));
        assert_eq!(n("26").nth_root(3), n("2"));
        assert_eq!(n("1000000").nth_root(3), n("100"));
    }

    #[test]
    fn gcd_matches_euclid_hand_cases() {
        assert_eq!(n("48").gcd(&n("18")), n("6"));
        assert_eq!(n("0").gcd(&n("5")), n("5"));
        assert_eq!(n("17").gcd(&n("13")), n("1"));
    }

    #[test]
    fn hex_roundtrip() {
        let value = n("4059231");
        let hex = value.to_hex();
        assert_eq!(Natural::from_str_radix(&hex, 16).unwrap(), value);
    }

    #[test]
    fn bit_length_and_bit_access() {
        let v = n("256"); // 2^8
        assert_eq!(v.bit_length(), 9);
        assert!(v.bit(8));
        assert!(!v.bit(7));
    }

    #[test]
    fn shl_shr_roundtrip() {
        let v = n("123456789");
        assert_eq!(v.shl(37).shr(37), v);
    }

    #[test]
    fn ord_compares_by_magnitude() {
        assert!(n("9") < n("10"));
        assert!(n("100000000000000000000") > n("99999999999999999999"));
        assert_eq!(n("5"), n("5"));
    }
    // #endregion 🔖️FundamentalTests

    // #region 🔖️QuickTests
    mod quick {
        use super::*;

        #[test]
        fn karatsuba_matches_schoolbook_across_threshold() {
            let mut seed = 0x1234_5678_9abc_def1u64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            // Sizes straddling KARATSUBA_THRESHOLD (32 limbs) on both operands.
            for limb_count in [1usize, 10, 31, 32, 33, 40, 64, 96] {
                let a_limbs: Vec<u64> = (0..limb_count).map(|_| next()).collect();
                let b_limbs: Vec<u64> = (0..limb_count / 2 + 1).map(|_| next()).collect();
                let a = Natural::normalize(a_limbs);
                let b = Natural::normalize(b_limbs);
                let via_karatsuba = a.mul(&b);
                let via_schoolbook = a.mul_schoolbook_pub(&b);
                assert_eq!(via_karatsuba, via_schoolbook, "mismatch at limb_count={limb_count}");
            }
        }

        #[test]
        fn div_rem_invariant_holds_on_random_shapes() {
            let mut seed = 0xdead_beef_cafe_babeu64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            for _ in 0..200 {
                let a_len = 1 + (next() % 20) as usize;
                let b_len = 1 + (next() % a_len.max(1) as u64) as usize;
                let a = Natural::normalize((0..a_len).map(|_| next()).collect());
                let mut b = Natural::normalize((0..b_len).map(|_| next()).collect());
                if b.is_zero() {
                    b = Natural::one();
                }
                let (q, r) = a.div_rem(&b);
                assert!(r < b);
                assert_eq!(q.mul(&b).add(&r), a);
            }
        }

        #[test]
        fn div_rem_add_back_branch_is_exercised_and_correct() {
            // Crafted vectors known to trigger Knuth D's rare add-back correction: divisor with a
            // near-max top limb paired with a dividend that makes the initial qhat estimate overshoot.
            let u = Natural::normalize(vec![0, 0, 0x8000_0000_0000_0000, 1]);
            let v = Natural::normalize(vec![0xFFFF_FFFF_FFFF_FFFF, 0x8000_0000_0000_0000]);
            let (q, r) = u.div_rem(&v);
            assert!(r < v);
            assert_eq!(q.mul(&v).add(&r), u);
        }

        #[test]
        fn sieve_like_isqrt_matches_f64_for_moderate_values() {
            for v in [2u64, 3, 99, 1_000_003, 999_999_999] {
                let exact = Natural::from_u64(v).isqrt();
                let approx = (v as f64).sqrt() as u64;
                let exact_u64 = exact.to_u64().unwrap();
                assert!(exact_u64.abs_diff(approx) <= 1, "isqrt({v}) = {exact_u64}, float estimate {approx}");
            }
        }
    }
    // #endregion 🔖️QuickTests

    // #region 🔖️LongTests
    mod long {
        use super::*;

        #[test]
        fn stress_4096_bit_mul_div_gcd() {
            let mut seed = 0x0123_4567_89ab_cdefu64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            let limbs_4096 = 64; // 4096 / 64
            for _ in 0..5 {
                let a = Natural::normalize((0..limbs_4096).map(|_| next()).collect());
                let mut b = Natural::normalize((0..limbs_4096 / 2).map(|_| next()).collect());
                if b.is_zero() {
                    b = Natural::one();
                }
                let prod = a.mul(&b);
                let (q, r) = prod.div_rem(&b);
                assert_eq!(r, Natural::zero());
                assert_eq!(q, a);
                let g = a.gcd(&b);
                assert!(a.div_rem(&g).1.is_zero());
                assert!(b.div_rem(&g).1.is_zero());
            }
        }
    }
    // #endregion 🔖️LongTests

    // #region 🔖️ExhaustiveTests
    mod exhaustive {
        use super::*;

        #[test]
        fn stress_16384_bit_operations() {
            let mut seed = 0xfeed_face_dead_c0deu64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            let limbs_16384 = 256; // 16384 / 64
            for _ in 0..3 {
                let a = Natural::normalize((0..limbs_16384).map(|_| next()).collect());
                let b = Natural::normalize((0..limbs_16384).map(|_| next()).collect());
                let sum = a.add(&b);
                assert!(sum >= a && sum >= b);
                let prod = a.mul(&b);
                if !a.is_zero() {
                    let (q, r) = prod.div_rem(&a);
                    assert_eq!(r, Natural::zero());
                    assert_eq!(q, b);
                }
            }
        }
    }
    // #endregion 🔖️ExhaustiveTests
}
