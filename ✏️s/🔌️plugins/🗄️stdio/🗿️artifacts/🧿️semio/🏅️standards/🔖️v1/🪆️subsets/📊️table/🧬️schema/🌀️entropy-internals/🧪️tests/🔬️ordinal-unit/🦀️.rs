mod tests {
    use super::*;

    #[test]
    fn permutation_entropy_of_monotone_series_is_zero() {
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let est = permutation_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn permutation_entropy_of_noise_approaches_max() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..3000).map(|_| rng.next_f64()).collect();
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let est = permutation_entropy(&x, cfg, LogBase::Bits).unwrap();
        let max = 6.0_f64.log2(); // 3! = 6 patterns
        assert!(est.value > 0.9 * max, "got {} max {}", est.value, max);
    }

    #[test]
    fn dispersion_config_rejects_small_classes() {
        assert!(DispersionConfig::new(1, 2, 1).is_err());
    }

    #[test]
    fn dispersion_entropy_of_noise_is_positive() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let cfg = DispersionConfig::new(4, 2, 1).unwrap();
        let est = dispersion_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    #[test]
    fn increment_entropy_of_constant_series_is_zero() {
        let x = vec![5.0; 100];
        // 🔐️ all increments are exactly zero -> single symbol -> zero entropy.
        let est = increment_entropy(&x, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn increment_entropy_of_noise_is_positive() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let est = increment_entropy(&x, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    #[test]
    fn increment_entropy_rejects_zero_word_length() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        assert!(increment_entropy(&x, 0, 3, LogBase::Bits).is_err());
    }

    #[test]
    fn slope_entropy_rejects_bad_thresholds() {
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        assert!(slope_entropy(&x, (0.5, 0.3), 2, LogBase::Bits).is_err());
        assert!(slope_entropy(&x, (-0.1, 0.5), 2, LogBase::Bits).is_err());
    }

    #[test]
    fn slope_entropy_of_straight_line_is_zero() {
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let est = slope_entropy(&x, (0.2, 0.8), 2, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn slope_entropy_of_noise_is_positive() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(4);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let est = slope_entropy(&x, (0.2, 0.8), 2, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn permutation_entropy_orders_regularity_correctly() {
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(5);
            let n = 2000;
            let sine: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin()).collect();
            let noise: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
            let cfg = OrdinalConfig::new(4, 1).unwrap();
            let h_sine = permutation_entropy(&sine, cfg, LogBase::Bits).unwrap().value;
            let h_noise = permutation_entropy(&noise, cfg, LogBase::Bits).unwrap().value;
            assert!(h_sine < h_noise, "sine={h_sine} noise={h_noise}");
        }
    }
}
