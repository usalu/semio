mod tests {
    use super::*;

    #[test]
    fn from_symbols_counts_correctly() {
        let counts = Counts::from_symbols(&[0, 1, 1, 2, 0, 0], 3).unwrap();
        assert_eq!(counts.raw(), &[3.0, 2.0, 1.0]);
        assert_eq!(counts.total(), 6.0);
        assert_eq!(counts.n_raw(), 6);
    }

    #[test]
    fn from_symbols_rejects_empty() {
        assert!(matches!(Counts::from_symbols(&[], 3), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn from_symbols_rejects_out_of_range() {
        assert!(matches!(Counts::from_symbols(&[0, 5], 3), Err(EntropyError::ShapeMismatch { .. })));
    }

    #[test]
    fn probabilities_normalize_to_one() {
        let counts = Counts::from_symbols(&[0, 1, 1, 2], 3).unwrap();
        let p = counts.probabilities();
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!((p[1] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn weighted_counts_effective_sample_size() {
        let counts = Counts::from_weighted(&[0, 1], &[1.0, 1.0], 2).unwrap();
        assert!((counts.n_effective() - 2.0).abs() < 1e-9);
        let skewed = Counts::from_weighted(&[0, 1], &[10.0, 0.001], 2).unwrap();
        assert!(skewed.n_effective() < 1.1);
    }

    #[test]
    fn support_and_singleton_diagnostics() {
        let counts = Counts::from_symbols(&[0, 0, 1, 2, 2, 2], 4).unwrap();
        assert_eq!(counts.support_size(), 3);
        assert_eq!(counts.singletons(), 1);
        assert_eq!(counts.doubletons(), 1);
    }

    #[test]
    fn laplace_smoothing_adds_pseudocount() {
        let counts = Counts::from_symbols(&[0, 0, 1], 2).unwrap();
        let p = counts.smoothed_probabilities(SmoothingPrior::Laplace);
        // (2+1)/(3+2), (1+1)/(3+2)
        assert!((p[0] - 0.6).abs() < 1e-12);
        assert!((p[1] - 0.4).abs() < 1e-12);
    }

    #[test]
    fn joint_counts_marginals_match_independent_construction() {
        let x = [0, 0, 1, 1];
        let y = [0, 1, 0, 1];
        let joint = JointCounts::from_pairs(&x, &y, 2, 2).unwrap();
        assert_eq!(joint.marginal_x(), vec![0.5, 0.5]);
        assert_eq!(joint.marginal_y(), vec![0.5, 0.5]);
        assert_eq!(joint.total(), 4.0);
    }

    #[test]
    fn validate_probabilities_renormalizes_within_tolerance() {
        let p = validate_probabilities(&[0.5, 0.5 + 1e-10], Tolerances::default()).unwrap();
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn validate_probabilities_rejects_large_deviation() {
        assert!(matches!(validate_probabilities(&[0.5, 0.2], Tolerances::default()), Err(EntropyError::NotNormalized { .. })));
    }

    #[test]
    fn validate_probabilities_rejects_nan() {
        assert!(matches!(validate_probabilities(&[0.5, f64::NAN], Tolerances::default()), Err(EntropyError::NonFinite { .. })));
    }

    #[test]
    fn encode_categories_is_first_seen_order() {
        let (symbols, k) = encode_categories(&["b", "a", "b", "c"]);
        assert_eq!(symbols, vec![0, 1, 0, 2]);
        assert_eq!(k, 3);
    }
}
