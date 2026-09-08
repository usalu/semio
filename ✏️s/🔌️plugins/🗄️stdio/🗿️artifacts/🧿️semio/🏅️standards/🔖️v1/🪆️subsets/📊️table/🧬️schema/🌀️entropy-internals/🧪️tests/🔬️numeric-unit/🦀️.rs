mod tests {
    use super::*;

    #[test]
    fn ln_gamma_matches_known_factorials() {
        for n in 1..10u32 {
            let expected = (1..n).map(|k| k as f64).product::<f64>().ln();
            assert!((ln_gamma(n as f64) - expected).abs() < 1e-9, "n={n}");
        }
    }

    #[test]
    fn ln_gamma_half_matches_sqrt_pi() {
        let expected = core::f64::consts::PI.sqrt().ln();
        assert!((ln_gamma(0.5) - expected).abs() < 1e-9);
    }

    #[test]
    fn digamma_matches_known_value_at_one() {
        // 🔬️ psi(1) = -gamma (Euler-Mascheroni constant).
        let euler_mascheroni = 0.577_215_664_901_532_9;
        assert!((digamma(1.0) - (-euler_mascheroni)).abs() < 1e-9);
    }

    #[test]
    fn digamma_recurrence_holds() {
        let x = 3.7;
        assert!((digamma(x + 1.0) - (digamma(x) + 1.0 / x)).abs() < 1e-9);
    }

    #[test]
    fn trigamma_matches_pi_squared_over_six_at_one() {
        assert!((trigamma(1.0) - core::f64::consts::PI.powi(2) / 6.0).abs() < 1e-8);
    }

    #[test]
    fn erf_endpoints() {
        // 🔬️ A&S 7.1.26 has a stated max absolute error of ~1.5e-7; it is not exact at x=0.
        assert!((erf(0.0)).abs() < 1e-6);
        assert!((erf(10.0) - 1.0).abs() < 1e-6);
        assert!((erf(-10.0) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn normal_cdf_at_zero_is_half() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn inverse_normal_cdf_roundtrips_normal_cdf() {
        for p in [0.001, 0.01, 0.1, 0.3, 0.5, 0.7, 0.9, 0.99, 0.999] {
            let x = inverse_normal_cdf(p);
            let back = normal_cdf(x);
            assert!((back - p).abs() < 1e-8, "p={p} back={back}");
        }
    }

    #[test]
    fn incomplete_gamma_full_integral_is_one() {
        assert!((regularized_lower_incomplete_gamma(2.5, 1e6) - 1.0).abs() < 1e-9);
        assert!(regularized_upper_incomplete_gamma(2.5, 1e6) < 1e-9);
    }

    #[test]
    fn log_factorial_cache_matches_ln_gamma() {
        let cache = LogFactorialCache::new(20);
        for n in 0..20 {
            assert!((cache.get(n) - ln_gamma(n as f64 + 1.0)).abs() < 1e-9);
        }
    }

    #[test]
    fn neumaier_sum_exact_on_simple_case() {
        assert_eq!(neumaier_sum([1.0, 2.0, 3.0]), 6.0);
    }

    #[test]
    fn neumaier_more_accurate_than_naive_for_ill_conditioned_sum() {
        let mut values = vec![1e16, 1.0, -1e16];
        let naive: f64 = values.iter().sum();
        let stable = neumaier_sum(values.iter().copied());
        assert_eq!(naive, 0.0); // 🔬️ naive loses the 1.0 entirely
        assert!((stable - 1.0).abs() < 1e-9);
        values.reverse();
        assert!((neumaier_sum(values) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn pairwise_sum_matches_neumaier_on_large_array() {
        let values: Vec<f64> = (0..10_000).map(|i| (i as f64).sin()).collect();
        let a = pairwise_sum(&values);
        let b = neumaier_sum(values.iter().copied());
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn x_ln_x_zero_at_zero() {
        assert_eq!(x_ln_x(0.0), 0.0);
        assert!((x_ln_x(1.0) - 0.0).abs() < 1e-12);
        assert!(x_ln_x(core::f64::consts::E) > 0.0);
    }

    #[test]
    fn log_sum_exp_matches_naive_for_moderate_values() {
        let values: [f64; 4] = [0.1, 0.5, -0.3, 0.2];
        let naive = values.iter().map(|v: &f64| v.exp()).sum::<f64>().ln();
        assert!((log_sum_exp(&values) - naive).abs() < 1e-9);
    }

    #[test]
    fn log_sum_exp_avoids_overflow() {
        let values = [1000.0, 1000.5, 999.0];
        let result = log_sum_exp(&values);
        assert!(result.is_finite());
        assert!(result > 1000.0 && result < 1002.0);
    }

    #[test]
    fn xorshift_is_deterministic_for_fixed_seed() {
        let mut a = Xorshift64::new(42);
        let mut b = Xorshift64::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn xorshift_uniform_range_respected() {
        let mut rng = Xorshift64::new(1234);
        for _ in 0..1000 {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v));
            let b = rng.next_below(7);
            assert!(b < 7);
        }
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut rng = Xorshift64::new(99);
        let mut data: Vec<u32> = (0..50).collect();
        rng.shuffle(&mut data);
        let mut sorted = data.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..50).collect::<Vec<u32>>());
    }

    #[test]
    fn checked_state_count_detects_overflow() {
        assert_eq!(checked_state_count(&[2, 3, 4]), Some(24));
        // 🔬️ usize::MAX * 2 (~3.6e19) fits comfortably inside u128 (~3.4e38); an overflow needs
        // enough factors that their product exceeds u128::MAX.
        assert_eq!(checked_state_count(&[usize::MAX; 8]), None);
    }

    #[test]
    fn clamp_near_zero_behavior() {
        assert_eq!(clamp_near_zero(-1e-14, 1e-12), 0.0);
        assert_eq!(clamp_near_zero(-1.0, 1e-12), -1.0);
        assert_eq!(clamp_near_zero(5.0, 1e-12), 5.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn xorshift_gaussian_mean_and_variance_converge() {
            let mut rng = Xorshift64::new(7);
            let n = 20_000;
            let samples: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
            let mean = samples.iter().sum::<f64>() / n as f64;
            let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
            assert!(mean.abs() < 0.05, "mean={mean}");
            assert!((var - 1.0).abs() < 0.05, "var={var}");
        }

        #[test]
        fn digamma_asymptotic_region_matches_recurrence_shifted_from_small_x() {
            for x in [0.1, 0.5, 1.5, 5.9, 6.1, 50.0, 500.0] {
                let direct = digamma(x);
                let via_recurrence = digamma(x + 1.0) - 1.0 / x;
                assert!((direct - via_recurrence).abs() < 1e-8, "x={x}");
            }
        }
    }
}
