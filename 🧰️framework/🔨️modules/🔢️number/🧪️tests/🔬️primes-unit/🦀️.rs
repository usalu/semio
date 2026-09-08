mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn is_prime_u64_hand_cases() {
        for p in [2u64, 3, 5, 7, 11, 97, 7919, 999_983] {
            assert!(is_prime_u64(p), "{p} should be prime");
        }
        for c in [1u64, 4, 6, 8, 9, 100, 561, 1105, 1729] {
            assert!(!is_prime_u64(c), "{c} should be composite");
        }
    }

    #[test]
    fn carmichael_numbers_are_rejected() {
        for &carmichael in &[561u64, 41041, 825265] {
            assert!(!is_prime_u64(carmichael), "{carmichael} is a Carmichael number, not prime");
            assert!(!is_prime(&Natural::from_u64(carmichael)));
        }
    }

    #[test]
    fn is_prime_natural_matches_u64_path_for_small_values() {
        for n in 2u64..200 {
            assert_eq!(is_prime(&Natural::from_u64(n)), is_prime_u64(n), "mismatch at {n}");
        }
    }

    #[test]
    fn is_prime_natural_handles_large_known_prime() {
        // A 128-bit-ish prime candidate; validated via BPSW.
        let p = Natural::from_str("170141183460469231731687303715884105727").unwrap(); // 2^127 - 1, a Mersenne prime
        assert!(is_prime(&p));
        let composite = p.add(&Natural::from_u64(2));
        assert!(!is_prime(&composite) || composite.to_u64().is_some());
    }

    #[test]
    fn factor_u64_reconstructs_via_multiplication() {
        for n in [1u64, 2, 97, 360, 999_983, 1_000_000] {
            let factors = factor_u64(n);
            let product: u64 = factors.iter().map(|&(p, e)| p.pow(e)).product();
            assert_eq!(product, n, "factorization of {n} = {factors:?} doesn't reconstruct");
            for (p, _) in &factors {
                assert!(is_prime_u64(*p));
            }
        }
    }

    #[test]
    fn euler_phi_hand_cases() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(9), 6);
        assert_eq!(euler_phi(97), 96);
    }

    #[test]
    fn moebius_hand_cases() {
        assert_eq!(moebius(1), 1);
        assert_eq!(moebius(6), 1); // 2*3, two distinct primes -> +1
        assert_eq!(moebius(4), 0); // 2^2 -> 0
        assert_eq!(moebius(30), -1); // 2*3*5, three distinct primes -> -1
    }

    #[test]
    fn divisors_hand_case() {
        let divs = divisors_u64(12);
        assert_eq!(divs, vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisor_count(12), 6);
        assert_eq!(divisor_sum(12), 28);
    }

    #[test]
    fn sieve_matches_is_prime_u64() {
        let sieve = Sieve::new(1000);
        for n in 0..=1000 {
            assert_eq!(sieve.is_prime(n), is_prime_u64(n as u64), "mismatch at {n}");
        }
    }

    #[test]
    fn next_prime_hand_cases() {
        assert_eq!(next_prime(&Natural::from_u64(10)), Natural::from_u64(11));
        assert_eq!(next_prime(&Natural::from_u64(1)), Natural::from_u64(2));
    }

    // #region 🔖️QuickTests
    mod quick {
        use super::*;

        #[test]
        fn factor_random_64bit_semiprimes() {
            // Products of two mid-size primes.
            for &(p, q) in &[(65537u64, 65539u64), (999_983, 999_979), (7919, 104729)] {
                let n = p * q;
                let factors = factor_u64(n);
                let product: u64 = factors.iter().map(|&(f, e)| f.pow(e)).product();
                assert_eq!(product, n);
                assert!(factors.iter().any(|&(f, _)| f == p));
                assert!(factors.iter().any(|&(f, _)| f == q));
            }
        }

        #[test]
        fn sieve_count_matches_known_pi_of_1e6() {
            let sieve = Sieve::new(1_000_000);
            assert_eq!(sieve.count(), 78498);
        }
    }
    // #endregion 🔖️QuickTests

    // #region 🔖️LongTests
    mod long {
        use super::*;

        #[test]
        fn factor_natural_random_semiprime() {
            let p = Natural::from_str("1000000000000000003").unwrap(); // prime
            assert!(is_prime(&p));
            let q = Natural::from_u64(104_729); // prime, the 10,000th prime
            let n = p.mul(&q);
            let factors = factor(&n);
            let product = factors.iter().fold(Natural::one(), |acc, (f, e)| acc.mul(&f.pow(*e as u64)));
            assert_eq!(product, n);
        }
    }
    // #endregion 🔖️LongTests
}
