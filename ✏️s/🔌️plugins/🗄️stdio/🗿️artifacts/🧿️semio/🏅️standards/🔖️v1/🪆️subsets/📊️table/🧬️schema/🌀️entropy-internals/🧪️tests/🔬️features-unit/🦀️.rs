mod tests {
    use super::*;

    #[test]
    fn standard_registry_computes_all_features_in_order() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let registry = FeatureRegistry::standard();
        let features = registry.compute(&x).unwrap();
        let names: Vec<&str> = features.iter().map(|f| f.name).collect();
        assert_eq!(names, vec!["histogram_entropy", "sample_entropy", "permutation_entropy", "spectral_entropy", "lempel_ziv_complexity"]);
        for f in &features {
            assert!(f.estimate.value.is_finite(), "{} produced non-finite value", f.name);
        }
    }

    #[test]
    fn with_feature_appends_after_standard_entries() {
        let registry = FeatureRegistry::standard().with_feature("custom", feature_histogram_entropy as FeatureFn);
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..1500).map(|_| rng.next_gaussian()).collect();
        let features = registry.compute(&x).unwrap();
        assert_eq!(features.last().unwrap().name, "custom");
    }

    #[test]
    fn compute_propagates_first_error() {
        let registry = FeatureRegistry::standard();
        let constant = vec![1.0; 500];
        assert!(registry.compute(&constant).is_err());
    }

    #[test]
    fn suggest_bins_prefers_freedman_diaconis_for_spread_data() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..500).map(|_| rng.next_gaussian()).collect();
        assert!(matches!(suggest_bins(&x), BinsSpec::FreedmanDiaconis));
    }

    #[test]
    fn suggest_bins_falls_back_to_sturges_for_degenerate_iqr() {
        let mut x = vec![0.0; 100];
        x[0] = 1000.0; // 🔐️ a single outlier keeps the IQR at zero
        assert!(matches!(suggest_bins(&x), BinsSpec::Sturges));
    }

    #[test]
    fn suggest_knn_k_scales_with_sample_size_and_stays_bounded() {
        assert_eq!(suggest_knn_k(0), 3);
        assert!(suggest_knn_k(16) <= 4);
        let k_large = suggest_knn_k(1_000_000);
        assert!((3..=20).contains(&k_large));
    }
}
