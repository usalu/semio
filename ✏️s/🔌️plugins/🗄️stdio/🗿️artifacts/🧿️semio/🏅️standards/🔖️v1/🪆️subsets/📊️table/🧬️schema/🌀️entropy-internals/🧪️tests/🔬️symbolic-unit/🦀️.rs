mod tests {
    use super::*;

    // #region 🔖️EmbedTests
    #[test]
    fn embed_produces_expected_windows_and_values() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let windows = embed(&x, 3, 1).unwrap();
        assert_eq!(windows, vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
    }

    #[test]
    fn embed_respects_tau() {
        let x = [0.0, 1.0, 2.0, 3.0, 4.0];
        let windows = embed(&x, 2, 2).unwrap();
        assert_eq!(windows, vec![vec![0.0, 2.0], vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn embed_rejects_zero_dim_and_zero_tau() {
        assert!(matches!(embed(&[1.0, 2.0], 0, 1), Err(EntropyError::InvalidConfig { field: "dim", .. })));
        assert!(matches!(embed(&[1.0, 2.0], 1, 0), Err(EntropyError::InvalidConfig { field: "tau", .. })));
    }

    #[test]
    fn embed_rejects_insufficient_data() {
        let x = [1.0, 2.0];
        assert!(matches!(embed(&x, 5, 1), Err(EntropyError::InsufficientData { needed: 5, actual: 2, .. })));
    }
    // #endregion 🔖️EmbedTests

    // #region 🔖️OrdinalTests
    #[test]
    fn ordinal_config_rejects_dim_less_than_two() {
        assert!(matches!(OrdinalConfig::new(1, 1), Err(EntropyError::InvalidConfig { field: "dim", .. })));
    }

    #[test]
    fn ordinal_config_rejects_zero_tau() {
        assert!(matches!(OrdinalConfig::new(3, 0), Err(EntropyError::InvalidConfig { field: "tau", .. })));
    }

    #[test]
    fn ordinal_config_default_is_dim3_tau1() {
        let cfg = OrdinalConfig::default();
        assert_eq!(cfg.dim, 3);
        assert_eq!(cfg.tau, 1);
        assert_eq!(cfg.ties, TiePolicy::StableRank);
    }

    #[test]
    fn ordinal_config_with_ties_overrides_default() {
        let cfg = OrdinalConfig::new(3, 1).unwrap().with_ties(TiePolicy::Error);
        assert_eq!(cfg.ties, TiePolicy::Error);
    }

    #[test]
    fn ordinal_pattern_symbol_hand_computed_example() {
        // 🔤️ Window [3, 1, 4]: ascending order is index1(1) < index0(3) < index2(4), whose
        // Lehmer code (base 3!) is [1, 0, 0] -> 1*2! + 0*1! + 0*0! = 2.
        let symbol = ordinal_pattern_symbol(&[3.0, 1.0, 4.0], TiePolicy::StableRank).unwrap();
        assert_eq!(symbol, 2);
    }

    #[test]
    fn ordinal_symbolizer_all_six_dim3_patterns_are_distinct() {
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let symbolizer = OrdinalSymbolizer::new(cfg);
        assert_eq!(symbolizer.alphabet_size(), 6);
        let base = [10.0, 20.0, 30.0];
        let mut symbols = Vec::new();
        // 🔤️ Enumerate all 3! permutations of a 3-element index array.
        for a in 0..3 {
            for b in 0..3 {
                for c in 0..3 {
                    if a == b || b == c || a == c {
                        continue;
                    }
                    let window: Vec<f64> = [a, b, c].iter().map(|&i| base[i]).collect();
                    let symbol = symbolizer.symbolize(&window).unwrap();
                    assert_eq!(symbol.len(), 1);
                    symbols.push(symbol[0]);
                }
            }
        }
        assert_eq!(symbols.len(), 6);
        let mut unique = symbols.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 6);
        assert!(symbols.iter().all(|&s| s < 6));
    }

    #[test]
    fn ordinal_symbolizer_monotone_series_yields_constant_pattern() {
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let symbolizer = OrdinalSymbolizer::new(cfg);
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let symbols = symbolizer.symbolize(&x).unwrap();
        assert_eq!(symbols.len(), 8);
        assert!(symbols.iter().all(|&s| s == symbols[0]));
        assert_eq!(symbols[0], 0); // 🔤️ strictly ascending window == identity permutation
    }

    #[test]
    fn ordinal_symbolizer_alphabet_size_is_factorial() {
        assert_eq!(OrdinalSymbolizer::new(OrdinalConfig::new(4, 1).unwrap()).alphabet_size(), 24);
        assert_eq!(OrdinalSymbolizer::new(OrdinalConfig::new(5, 1).unwrap()).alphabet_size(), 120);
    }

    #[test]
    fn ordinal_ties_error_policy_rejects_equal_values() {
        let cfg = OrdinalConfig::new(3, 1).unwrap().with_ties(TiePolicy::Error);
        let symbolizer = OrdinalSymbolizer::new(cfg);
        let result = symbolizer.symbolize(&[1.0, 1.0, 2.0]);
        assert!(matches!(result, Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn ordinal_ties_stable_rank_breaks_by_original_index() {
        // 🔤️ [2, 2, 1]: ascending stable order is index2(1) < index0(2) < index1(2), whose
        // Lehmer code (base 3!) is [2, 0, 0] -> 2*2! + 0*1! + 0*0! = 4.
        let symbol = ordinal_pattern_symbol(&[2.0, 2.0, 1.0], TiePolicy::StableRank).unwrap();
        assert_eq!(symbol, 4);
    }
    // #endregion 🔖️OrdinalTests

    // #region 🔖️DispersionTests
    #[test]
    fn dispersion_symbolizer_rejects_invalid_config() {
        assert!(matches!(DispersionSymbolizer::new(1, 2, 1), Err(EntropyError::InvalidConfig { field: "classes", .. })));
        assert!(matches!(DispersionSymbolizer::new(3, 0, 1), Err(EntropyError::InvalidConfig { field: "dim", .. })));
        assert!(matches!(DispersionSymbolizer::new(3, 2, 0), Err(EntropyError::InvalidConfig { field: "tau", .. })));
    }

    #[test]
    fn dispersion_symbolizer_alphabet_size_is_classes_pow_dim() {
        let symbolizer = DispersionSymbolizer::new(3, 2, 1).unwrap();
        assert_eq!(symbolizer.alphabet_size(), 9);
    }

    #[test]
    fn dispersion_symbolizer_rejects_constant_series() {
        let symbolizer = DispersionSymbolizer::new(3, 2, 1).unwrap();
        let x = vec![5.0; 20];
        assert!(matches!(symbolizer.symbolize(&x), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn dispersion_symbolizer_symbol_count_matches_embedding() {
        let symbolizer = DispersionSymbolizer::new(4, 3, 1).unwrap();
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(11);
        let x: Vec<f64> = (0..30).map(|_| rng.next_gaussian()).collect();
        let symbols = symbolizer.symbolize(&x).unwrap();
        assert_eq!(symbols.len(), 30 - (3 - 1));
        assert!(symbols.iter().all(|&s| (s as usize) < symbolizer.alphabet_size()));
    }
    // #endregion 🔖️DispersionTests

    // #region 🔖️QuantileTests
    #[test]
    fn quantile_symbolizer_rejects_bins_less_than_two() {
        assert!(matches!(QuantileSymbolizer::new(1), Err(EntropyError::InvalidConfig { field: "bins", .. })));
    }

    #[test]
    fn quantile_symbolizer_alphabet_size_equals_bins() {
        assert_eq!(QuantileSymbolizer::new(5).unwrap().alphabet_size(), 5);
    }

    #[test]
    fn quantile_symbolizer_splits_small_example_in_half() {
        let symbolizer = QuantileSymbolizer::new(2).unwrap();
        let symbols = symbolizer.symbolize(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(symbols, vec![0, 0, 1, 1]);
    }
    // #endregion 🔖️QuantileTests

    // #region 🔖️ThresholdTests
    #[test]
    fn threshold_symbolizer_rejects_empty_edges() {
        assert!(matches!(ThresholdSymbolizer::new(vec![]), Err(EntropyError::InvalidConfig { field: "edges", .. })));
    }

    #[test]
    fn threshold_symbolizer_rejects_unsorted_edges() {
        assert!(matches!(ThresholdSymbolizer::new(vec![1.0, 0.0, 2.0]), Err(EntropyError::InvalidConfig { field: "edges", .. })));
    }

    #[test]
    fn threshold_symbolizer_rejects_non_finite_edges() {
        assert!(matches!(ThresholdSymbolizer::new(vec![0.0, f64::NAN]), Err(EntropyError::InvalidConfig { field: "edges", .. })));
    }

    #[test]
    fn threshold_symbolizer_classifies_against_edges() {
        let symbolizer = ThresholdSymbolizer::new(vec![0.0, 10.0]).unwrap();
        assert_eq!(symbolizer.alphabet_size(), 3);
        let symbols = symbolizer.symbolize(&[-5.0, 0.0, 5.0, 10.0, 15.0]).unwrap();
        assert_eq!(symbols, vec![0, 1, 1, 2, 2]);
    }
    // #endregion 🔖️ThresholdTests

    mod quick {
        use super::*;

        #[test]
        fn ordinal_symbols_always_within_alphabet_for_random_series() {
            let cfg = OrdinalConfig::new(4, 2).unwrap();
            let symbolizer = OrdinalSymbolizer::new(cfg);
            let alphabet = symbolizer.alphabet_size();
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(4242);
            for _ in 0..50 {
                let n = 20 + rng.next_below(50);
                let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
                let symbols = symbolizer.symbolize(&x).unwrap();
                assert!(symbols.iter().all(|&s| (s as usize) < alphabet));
            }
        }

        #[test]
        fn quantile_symbolizer_bins_are_roughly_balanced() {
            let symbolizer = QuantileSymbolizer::new(4).unwrap();
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(777);
            let x: Vec<f64> = (0..4000).map(|_| rng.next_gaussian()).collect();
            let symbols = symbolizer.symbolize(&x).unwrap();
            let mut counts = [0usize; 4];
            for &s in &symbols {
                counts[s as usize] += 1;
            }
            for &c in &counts {
                let frac = c as f64 / symbols.len() as f64;
                assert!((frac - 0.25).abs() < 0.03, "counts={counts:?}");
            }
        }

        #[test]
        fn dispersion_symbols_always_within_alphabet_for_random_series() {
            let symbolizer = DispersionSymbolizer::new(5, 3, 1).unwrap();
            let alphabet = symbolizer.alphabet_size();
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(909);
            for _ in 0..50 {
                let n = 30 + rng.next_below(40);
                let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
                let symbols = symbolizer.symbolize(&x).unwrap();
                assert!(symbols.iter().all(|&s| (s as usize) < alphabet));
            }
        }
    }
}
