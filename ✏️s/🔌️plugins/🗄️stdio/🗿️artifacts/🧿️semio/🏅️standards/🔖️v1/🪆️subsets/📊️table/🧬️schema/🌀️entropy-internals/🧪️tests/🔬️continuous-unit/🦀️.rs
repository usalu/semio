mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn box_muller_gaussian(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(seed);
        (0..n).map(|_| rng.next_gaussian()).collect()
    }

    #[test]
    fn gaussian_mle_matches_closed_form() {
        let x = box_muller_gaussian(5000, 1);
        let est = entropy_continuous(&x, &ContinuousMethod::GaussianMle, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn knn_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(3000, 2);
        let est = entropy_continuous(&x, &ContinuousMethod::Knn { k: 5 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn kde_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(2000, 3);
        let cfg = KdeConfig { kernel: Kernel::Gaussian, bandwidth: Bandwidth::Silverman };
        let est = entropy_continuous(&x, &ContinuousMethod::Kde(cfg), LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn vasicek_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(5000, 4);
        let est = entropy_continuous(&x, &ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn correa_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(3000, 5);
        let est = entropy_continuous(&x, &ContinuousMethod::Correa { m: 0 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn uniform_entropy_near_zero() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(6);
        let x: Vec<f64> = (0..5000).map(|_| rng.next_f64()).collect();
        let est = entropy_continuous(&x, &ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn histogram_entropy_reasonable_for_uniform() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(7);
        let x: Vec<f64> = (0..5000).map(|_| rng.next_f64()).collect();
        let est = entropy_continuous(&x, &ContinuousMethod::Histogram(BinsSpec::Sturges), LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.2, "got {}", est.value);
    }

    #[test]
    fn rejects_constant_series() {
        let x = vec![1.0; 100];
        assert!(entropy_continuous(&x, &ContinuousMethod::GaussianMle, LogBase::Nats).is_err());
        assert!(entropy_continuous(&x, &ContinuousMethod::Knn { k: 3 }, LogBase::Nats).is_err());
    }

    #[test]
    fn rejects_too_few_samples() {
        let x = vec![1.0];
        assert!(matches!(entropy_continuous(&x, &ContinuousMethod::GaussianMle, LogBase::Nats), Err(EntropyError::InsufficientData { .. })));
    }

    #[test]
    fn kde_loo_differs_from_naive_resubstitution_direction() {
        // 🔐️ LOO removes the self-term's downward bias, so LOO entropy should exceed naive
        // resubstitution (which double-counts each point against itself).
        let x = box_muller_gaussian(500, 8);
        let cfg = KdeConfig { kernel: Kernel::Gaussian, bandwidth: Bandwidth::Silverman };
        let density = KdeDensity::fit(&x, cfg).unwrap();
        let loo = density.entropy(LogBase::Nats).unwrap().value;
        let resub_nats = -x.iter().map(|&xi| density.pdf(xi).max(1e-300).ln()).sum::<f64>() / x.len() as f64;
        assert!(loo > resub_nats);
    }

    mod quick {
        use super::*;

        #[test]
        fn exponential_entropy_matches_closed_form() {
            // 🔐️ differential entropy of Exp(1) is 1 nat.
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(9);
            let x: Vec<f64> = (0..5000).map(|_| -rng.next_f64().max(1e-12).ln()).collect();
            let est = entropy_continuous(&x, &ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
            assert!((est.value - 1.0).abs() < 0.05, "got {}", est.value);
        }

        #[test]
        fn all_continuous_estimators_agree_within_tolerance_on_gaussian() {
            let x = box_muller_gaussian(4000, 42);
            let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
            let methods = vec![ContinuousMethod::GaussianMle, ContinuousMethod::Knn { k: 5 }, ContinuousMethod::Vasicek { m: 0 }, ContinuousMethod::Kde(KdeConfig::default())];
            for method in methods {
                let est = entropy_continuous(&x, &method, LogBase::Nats).unwrap();
                assert!((est.value - expected).abs() < 0.15, "{:?} -> {}", method, est.value);
            }
        }
    }
}
