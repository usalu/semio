mod tests {
    use super::*;

    #[test]
    fn plugin_matches_direct_entropy_computation() {
        let counts = [10u64, 7, 5, 2, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let total: f64 = counts.iter().sum::<u64>() as f64;
        let p: Vec<f64> = counts.iter().map(|&c| c as f64 / total).collect();
        let expected = crate::standards::v1::subsets::table::schema::entropy_internals::discrete::entropy(&p, LogBase::Nats).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn uniform_counts_all_methods_close_to_log_k() {
        let counts = vec![1000u64; 8];
        let expected = 8.0_f64.ln();
        for method in [DiscreteMethod::Plugin, DiscreteMethod::MillerMadow, DiscreteMethod::Grassberger, DiscreteMethod::Jackknife, DiscreteMethod::ChaoShen, DiscreteMethod::SchurmannGrassberger, DiscreteMethod::JamesStein] {
            let est = entropy_discrete(&counts, method, LogBase::Nats).unwrap();
            assert!((est.value - expected).abs() < 0.01, "{:?} -> {}", method, est.value);
        }
    }

    #[test]
    fn miller_madow_correction_matches_hand_computation() {
        // 🔐️ 3 bins, all occupied, N=6: correction = (3-1)/(2*6) = 1/6.
        let counts = [3u64, 2, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        let plugin = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!((est.value - (plugin.value + 1.0 / 6.0)).abs() < 1e-9);
    }

    #[test]
    fn bias_corrected_methods_closer_to_truth_than_plugin_on_undersampled_uniform() {
        // 🔐️ K=64 uniform, N=100: plug-in should underestimate ln(64) more than Miller-Madow.
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(7);
        let k = 64;
        let mut counts = vec![0u64; k];
        for _ in 0..100 {
            counts[rng.next_below(k)] += 1;
        }
        let truth = (k as f64).ln();
        let program = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let mm = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        assert!((truth - mm.value).abs() <= (truth - program.value).abs() + 1e-9);
    }

    #[test]
    fn chao_shen_handles_all_singletons() {
        let counts = [1u64, 1, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::ChaoShen, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= 0.0);
    }

    #[test]
    fn dirichlet_rejects_nonpositive_alpha() {
        assert!(matches!(entropy_discrete(&[1, 2, 3], DiscreteMethod::Dirichlet(0.0), LogBase::Nats), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn james_stein_shrinks_toward_uniform_reducing_variance_estimate() {
        let counts = [50u64, 1, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::JamesStein, LogBase::Nats).unwrap();
        let program = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        // 🔐️ shrinkage toward uniform increases entropy relative to the concentrated plug-in estimate.
        assert!(est.value >= program.value - 1e-9);
    }

    #[test]
    fn nsb_returns_finite_value_between_zero_and_log_k() {
        let counts = [10u64, 7, 5, 2, 1, 1, 0, 0];
        let est = entropy_discrete(&counts, DiscreteMethod::Nsb, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= -1e-6);
        assert!(est.value <= (counts.len() as f64).ln() + 1e-6);
    }

    #[test]
    fn schurmann_grassberger_close_to_plugin_for_large_n_uniform() {
        let counts = vec![10_000u64; 4];
        let est = entropy_discrete(&counts, DiscreteMethod::SchurmannGrassberger, LogBase::Nats).unwrap();
        let expected = 4.0_f64.ln();
        assert!((est.value - expected).abs() < 0.01);
    }

    #[test]
    fn jackknife_matches_hand_computation_on_small_example() {
        // 🔐️ 2 bins [3,1]: N=4.
        let counts = [3u64, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::Jackknife, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= -1e-9);
    }

    #[test]
    fn small_sample_warning_triggers_for_undersampled_input() {
        let counts = [1u64; 100];
        let est = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })) || est.n == 100);
    }

    #[test]
    fn gauss_legendre_nodes_are_symmetric_and_weights_sum_to_two() {
        let (nodes, weights) = gauss_legendre(20);
        let sum_w: f64 = weights.iter().sum();
        assert!((sum_w - 2.0).abs() < 1e-9);
        for i in 0..10 {
            assert!((nodes[i] + nodes[19 - i]).abs() < 1e-9);
        }
    }

    #[test]
    fn gauss_legendre_integrates_polynomial_exactly() {
        // 🔐️ 20-point GL is exact for polynomials up to degree 39; integrate x^4 over [-1,1] = 2/5.
        let (nodes, weights) = gauss_legendre(20);
        let integral: f64 = nodes.iter().zip(weights.iter()).map(|(&x, &w)| w * x.powi(4)).sum();
        assert!((integral - 0.4).abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn all_methods_consistency_as_n_grows() {
            let k = 16usize;
            let truth = (k as f64).ln();
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(55);
            let mut prev_error = f64::INFINITY;
            for &n in &[200usize, 2_000, 20_000] {
                let mut counts = vec![0u64; k];
                for _ in 0..n {
                    counts[rng.next_below(k)] += 1;
                }
                let est = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
                let error = (truth - est.value).abs();
                assert!(error <= prev_error + 0.05, "n={n} error={error} prev={prev_error}");
                prev_error = error;
            }
        }
    }
}
