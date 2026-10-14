mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mean(data: &[f64]) -> f64 {
        data.iter().sum::<f64>() / data.len() as f64
    }

    #[test]
    fn bootstrap_ci_contains_true_mean_for_fixed_seed() {
        let mut rng = Xorshift64::new(11);
        let true_mean = 5.0;
        let data: Vec<f64> = (0..500).map(|_| true_mean + rng.next_gaussian()).collect();
        let ci = bootstrap_ci(&data, mean, 2000, 0.95, 123).unwrap();
        assert!(ci.lower <= true_mean && true_mean <= ci.upper, "ci={ci:?}");
    }

    #[test]
    fn bootstrap_ci_rejects_insufficient_data_and_bad_level() {
        assert!(matches!(bootstrap_ci(&[], mean, 100, 0.95, 1), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(bootstrap_ci(&[1.0], mean, 100, 0.95, 1), Err(EntropyError::InsufficientData { .. })));
        assert!(matches!(bootstrap_ci(&[1.0, 2.0], mean, 100, 1.5, 1), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn jackknife_ci_matches_classical_mean_ci_for_large_n() {
        let mut rng = Xorshift64::new(22);
        let n = 1000;
        let data: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let sample_mean = mean(&data);
        let sd = (data.iter().map(|&v| (v - sample_mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt();
        let z = inverse_normal_cdf(0.975);
        let classical_lower = sample_mean - z * sd / (n as f64).sqrt();
        let classical_upper = sample_mean + z * sd / (n as f64).sqrt();

        let ci = jackknife_ci(&data, mean, 0.95).unwrap();
        assert!((ci.lower - classical_lower).abs() < 0.05, "lower {} vs {}", ci.lower, classical_lower);
        assert!((ci.upper - classical_upper).abs() < 0.05, "upper {} vs {}", ci.upper, classical_upper);
    }

    #[test]
    fn jackknife_ci_rejects_insufficient_data_and_bad_level() {
        assert!(matches!(jackknife_ci(&[], mean, 0.95), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(jackknife_ci(&[1.0, 2.0], mean, 0.0), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn permutation_test_p_value_high_under_null_low_under_effect() {
        let mut rng = Xorshift64::new(33);
        let x: Vec<f64> = (0..200).map(|_| rng.next_gaussian()).collect();
        let y: Vec<f64> = (0..200).map(|_| rng.next_gaussian()).collect();
        let diff_means = |a: &[f64], b: &[f64]| mean(a) - mean(b);

        // 🔬️ under the true null, permutation p-values are approximately uniform(0,1); a fixed
        // seed can land anywhere in that range, so assert only "not significant" (p > 0.05)
        // rather than a specific large value.
        let p_null = permutation_test(&x, &y, diff_means, 1000, 44).unwrap();
        assert!(p_null > 0.05, "p_null={p_null}");

        let y_shifted: Vec<f64> = x.iter().map(|&v| v + 5.0).collect();
        let p_effect = permutation_test(&x, &y_shifted, diff_means, 1000, 55).unwrap();
        assert!(p_effect < 0.05, "p_effect={p_effect}");
    }

    #[test]
    fn permutation_test_rejects_empty_groups_and_zero_permutations() {
        let diff_means = |a: &[f64], b: &[f64]| mean(a) - mean(b);
        assert!(matches!(permutation_test(&[], &[1.0], diff_means, 10, 1), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(permutation_test(&[1.0], &[], diff_means, 10, 1), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(permutation_test(&[1.0], &[2.0], diff_means, 0, 1), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn surrogate_config_rejects_zero_count_and_block_size() {
        assert!(matches!(SurrogateConfig::new(SurrogateKind::CircularShift, 0, 1), Err(EntropyError::InvalidConfig { .. })));
        assert!(matches!(SurrogateConfig::new(SurrogateKind::BlockShuffle { block_size: 0 }, 5, 1), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn circular_shift_and_block_shuffle_preserve_value_multiset() {
        let x: Vec<f64> = (0..40).map(|i| (i as f64).sin() * 3.0 + i as f64 * 0.1).collect();
        let mut sorted_x = x.clone();
        sorted_x.sort_by(f64::total_cmp);

        for kind in [SurrogateKind::CircularShift, SurrogateKind::BlockShuffle { block_size: 5 }] {
            let cfg = SurrogateConfig::new(kind, 10, 66).unwrap();
            let surrogates = surrogate_series(&x, &cfg).unwrap();
            assert_eq!(surrogates.len(), 10);
            for s in &surrogates {
                let mut sorted_s = s.clone();
                sorted_s.sort_by(f64::total_cmp);
                for (a, b) in sorted_s.iter().zip(sorted_x.iter()) {
                    assert!((a - b).abs() < 1e-9);
                }
            }
        }
    }

    #[test]
    fn phase_randomized_and_iaaft_preserve_power_spectrum() {
        let n = 64;
        let mut rng = Xorshift64::new(77);
        let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let fft = Fft::new(n);
        let original_spectrum = fft.forward(&x.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
        let original_magnitude: Vec<f64> = original_spectrum.iter().map(|c| c.abs()).collect();

        let phase_cfg = SurrogateConfig::new(SurrogateKind::PhaseRandomized, 1, 88).unwrap();
        let phase_surrogate = surrogate_series(&x, &phase_cfg).unwrap().pop().unwrap();
        let phase_spectrum = fft.forward(&phase_surrogate.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
        for (a, &b) in phase_spectrum.iter().zip(original_magnitude.iter()) {
            assert!((a.abs() - b).abs() < 1e-6, "phase-randomized spectrum magnitude drifted");
        }

        let iaaft_cfg = SurrogateConfig::new(SurrogateKind::Iaaft { iterations: 30 }, 1, 99).unwrap();
        let iaaft_surrogate = surrogate_series(&x, &iaaft_cfg).unwrap().pop().unwrap();
        let iaaft_spectrum = fft.forward(&iaaft_surrogate.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
        let mut sq_err = 0.0;
        let mut sq_total = 0.0;
        for (a, &b) in iaaft_spectrum.iter().zip(original_magnitude.iter()) {
            sq_err += (a.abs() - b).powi(2);
            sq_total += b * b;
        }
        assert!((sq_err / sq_total).sqrt() < 0.15, "iaaft relative spectral error too high");
    }

    #[test]
    fn iaaft_preserves_value_distribution_phase_randomized_generally_does_not() {
        let n = 50;
        let mut rng = Xorshift64::new(111);
        let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian().powi(2)).collect();
        let mut sorted_x = x.clone();
        sorted_x.sort_by(f64::total_cmp);

        let iaaft_cfg = SurrogateConfig::new(SurrogateKind::Iaaft { iterations: 20 }, 1, 222).unwrap();
        let mut sorted_iaaft = surrogate_series(&x, &iaaft_cfg).unwrap().pop().unwrap();
        sorted_iaaft.sort_by(f64::total_cmp);
        for (a, b) in sorted_iaaft.iter().zip(sorted_x.iter()) {
            assert!((a - b).abs() < 1e-9);
        }

        let phase_cfg = SurrogateConfig::new(SurrogateKind::PhaseRandomized, 1, 333).unwrap();
        let mut sorted_phase = surrogate_series(&x, &phase_cfg).unwrap().pop().unwrap();
        sorted_phase.sort_by(f64::total_cmp);
        let differs = sorted_phase.iter().zip(sorted_x.iter()).any(|(a, b)| (a - b).abs() > 1e-6);
        assert!(differs, "phase-randomized surrogate unexpectedly preserved the exact distribution");
    }

    #[test]
    fn fdr_bh_separates_tiny_and_large_p_values() {
        let p_values = vec![0.001, 0.002, 0.5, 0.7, 0.9];
        let rejected = fdr_bh(&p_values, 0.05).unwrap();
        assert_eq!(rejected, vec![true, true, false, false, false]);
    }

    #[test]
    fn fdr_bh_rejects_empty_input_and_bad_alpha() {
        assert!(matches!(fdr_bh(&[], 0.05), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(fdr_bh(&[0.1, 0.2], 1.0), Err(EntropyError::InvalidConfig { .. })));
    }

    mod quick {
        use super::*;

        #[test]
        fn fdr_bh_is_stricter_than_uncorrected_threshold_under_known_null_proportion() {
            let mut rng = Xorshift64::new(444);
            let m = 200;
            let n_null = (m as f64 * 0.8) as usize;
            let mut p_values = Vec::with_capacity(m);
            for _ in 0..n_null {
                p_values.push(rng.next_f64());
            }
            for _ in n_null..m {
                p_values.push(rng.next_f64().powi(8));
            }
            let alpha = 0.05;
            let rejected = fdr_bh(&p_values, alpha).unwrap();
            let bh_rejections = rejected.iter().filter(|&&r| r).count();
            let uncorrected_rejections = p_values.iter().filter(|&&p| p <= alpha).count();
            assert!(bh_rejections <= uncorrected_rejections, "bh={bh_rejections} uncorrected={uncorrected_rejections}");
        }

        #[test]
        fn surrogate_series_is_reproducible_from_the_same_seed() {
            let x: Vec<f64> = (0..30).map(|i| (i as f64 * 0.3).sin()).collect();
            let cfg = SurrogateConfig::new(SurrogateKind::Iaaft { iterations: 5 }, 4, 555).unwrap();
            let a = surrogate_series(&x, &cfg).unwrap();
            let b = surrogate_series(&x, &cfg).unwrap();
            assert_eq!(a, b);
        }
    }
}
