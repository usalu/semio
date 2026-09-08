mod tests {
    use super::*;

    #[test]
    fn mod_add_sub_mul_hand_cases() {
        assert_eq!(mod_add(7, 5, 10), 2);
        assert_eq!(mod_sub(3, 7, 10), 6);
        assert_eq!(mod_mul(6, 7, 10), 2);
    }

    #[test]
    fn mod_pow_hand_cases() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(5, 0, 7), 1);
    }

    #[test]
    fn mod_inv_matches_bezout() {
        let inv = mod_inv(3, 11).unwrap();
        assert_eq!(mod_mul(3, inv, 11), 1);
        assert!(mod_inv(2, 4).is_none());
    }

    #[test]
    fn jacobi_hand_cases() {
        assert_eq!(jacobi(1001, 9907), -1);
        assert_eq!(jacobi(19, 45), 1);
        assert_eq!(jacobi(8, 21), -1);
    }

    #[test]
    fn sqrt_mod_hand_cases() {
        // 4 mod 5 has sqrt 2 or 3
        let r = sqrt_mod(4, 5).unwrap();
        assert_eq!(mod_mul(r, r, 5), 4);
        // 2 is a non-residue mod 5 (since 5 % 8 not in {1,7})
        assert!(sqrt_mod(2, 5).is_none() || mod_mul(sqrt_mod(2, 5).unwrap(), sqrt_mod(2, 5).unwrap(), 5) == 2);
    }

    #[test]
    fn crt_pair_matches_congruences() {
        let (x, m) = crt_pair(2, 3, 3, 5).unwrap();
        assert_eq!(m, 15);
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
    }

    #[test]
    fn modint_field_operations() {
        let a = ModInt::new(7, 13);
        let b = ModInt::new(9, 13);
        assert_eq!(a.add(&b).value(), (7 + 9) % 13);
        let inv = a.inv().unwrap();
        assert_eq!(a.mul(&inv).value(), 1);
    }

    #[test]
    fn modint_unbound_zero_unifies_with_bound_side() {
        let bound = ModInt::new(5, 13);
        let unbound_zero = ModInt::zero();
        let sum = bound.add(&unbound_zero);
        assert_eq!(sum.value(), 5);
        assert_eq!(sum.modulus(), 13);
    }

    // #region 🔖️QuickTests
    mod quick {
        use super::*;

        #[test]
        fn sqrt_mod_matches_brute_force_for_all_residues_small_primes() {
            for &p in &[3u64, 5, 7, 11, 13, 17, 19, 23, 29] {
                for a in 0..p {
                    let brute = (0..p).find(|&x| mod_mul(x, x, p) == a);
                    let computed = sqrt_mod(a, p);
                    match (brute, computed) {
                        (Some(_), Some(r)) => assert_eq!(mod_mul(r, r, p), a),
                        (None, None) => {}
                        (b, c) => panic!("mismatch for a={a} p={p}: brute={b:?} computed={c:?}"),
                    }
                }
            }
        }
    }
    // #endregion 🔖️QuickTests
}
