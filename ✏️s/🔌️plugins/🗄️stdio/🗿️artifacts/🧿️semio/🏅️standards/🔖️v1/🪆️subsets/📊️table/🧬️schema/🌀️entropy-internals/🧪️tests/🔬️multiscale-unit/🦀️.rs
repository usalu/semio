mod tests {
    use super::*;

    #[test]
    fn multiscale_config_rejects_zero_scales() {
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        assert!(MultiscaleConfig::new(0, Grain::Mean, inner).is_err());
    }

    #[test]
    fn coarse_grain_mean_matches_hand_computation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let coarse = coarse_grain(&x, 2, Grain::Mean);
        assert_eq!(coarse, vec![1.5, 3.5, 5.5]);
    }

    #[test]
    fn coarse_grain_scale_one_is_identity() {
        let x = vec![1.0, 2.0, 3.0];
        assert_eq!(coarse_grain(&x, 1, Grain::Mean), x);
    }

    #[test]
    fn multiscale_entropy_reports_requested_scales_for_long_series() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..3000).map(|_| rng.next_f64()).collect();
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        let cfg = MultiscaleConfig::new(5, Grain::Mean, inner).unwrap();
        let result = multiscale_entropy(&x, &cfg, LogBase::Bits).unwrap();
        assert_eq!(result.scales, vec![1, 2, 3, 4, 5]);
        assert_eq!(result.per_scale.len(), 5);
    }

    #[test]
    fn multiscale_entropy_stops_early_for_short_series() {
        let x: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        let cfg = MultiscaleConfig::new(20, Grain::Mean, inner).unwrap();
        let result = multiscale_entropy(&x, &cfg, LogBase::Bits).unwrap();
        assert!(result.scales.len() < 20);
    }

    mod quick {
        use super::*;

        #[test]
        fn white_noise_multiscale_entropy_differs_from_pink_like_noise() {
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(2);
            let n = 4000;
            let white: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
            // 🔐️ a crude 1/f-like signal via running-sum (integrated white noise).
            let mut acc = 0.0;
            let pink: Vec<f64> = std::iter::repeat_with(|| {
                acc = 0.98 * acc + rng.next_gaussian();
                acc
            })
            .take(n)
            .collect();
            let inner = MsInner::SampleEntropy(RegularityConfig::new(2, crate::standards::v1::subsets::table::schema::entropy_internals::Tolerance::Auto).unwrap());
            let cfg = MultiscaleConfig::new(4, Grain::Mean, inner).unwrap();
            let white_result = multiscale_entropy(&white, &cfg, LogBase::Nats).unwrap();
            let pink_result = multiscale_entropy(&pink, &cfg, LogBase::Nats).unwrap();
            assert!((white_result.complexity_index - pink_result.complexity_index).abs() > 1e-6);
        }
    }
}
