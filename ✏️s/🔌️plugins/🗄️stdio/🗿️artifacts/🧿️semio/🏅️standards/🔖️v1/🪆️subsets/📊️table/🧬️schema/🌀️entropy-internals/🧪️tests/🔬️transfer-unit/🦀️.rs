mod tests {
    use super::*;

    #[test]
    fn transfer_config_rejects_zero_history() {
        assert!(TransferConfig::new(0, 1, TeBackend::Discrete { bins: 3 }).is_err());
    }

    #[test]
    fn te_of_independent_series_discrete_is_near_zero() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let n = 3000;
        let source: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let target: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 3 }).unwrap();
        let est = transfer_entropy(&source, &target, cfg).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn te_detects_coupling_discrete() {
        // 🔐️ target[i] = source[i-1] (with some noise mixed via binning): TE(source->target)
        // should be clearly larger than TE(target->source).
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(2);
        let n = 4000;
        let source: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let mut target = vec![0.0; n];
        target[1..n].copy_from_slice(&source[..n - 1]);
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 4 }).unwrap();
        let forward = transfer_entropy(&source, &target, cfg).unwrap();
        let backward = transfer_entropy(&target, &source, cfg).unwrap();
        assert!(forward.value > backward.value, "forward={} backward={}", forward.value, backward.value);
        assert!(forward.value > 0.1, "forward={}", forward.value);
    }

    #[test]
    fn ais_of_white_noise_is_near_zero() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_f64()).collect();
        let est = active_information_storage(&x, 1, TeBackend::Discrete { bins: 3 }, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn ais_of_highly_predictable_series_is_positive() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(4);
        let n = 2000;
        let mut x = vec![0.0; n];
        for i in 1..n {
            x[i] = 0.9 * x[i - 1] + 0.1 * rng.next_gaussian();
        }
        let cfg_backend = TeBackend::Discrete { bins: 4 };
        let est = active_information_storage(&x, 1, cfg_backend, LogBase::Nats).unwrap();
        assert!(est.value > 0.1, "got {}", est.value);
    }

    #[test]
    fn te_rejects_length_mismatch() {
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 3 }).unwrap();
        assert!(matches!(transfer_entropy(&[1.0, 2.0], &[1.0], cfg), Err(EntropyError::LengthMismatch { .. })));
    }

    mod quick {
        use super::*;

        #[test]
        fn te_knn_detects_coupling_on_coupled_logistic_maps() {
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(5);
            let n = 800;
            let mut x = vec![0.4 + 0.1 * rng.next_f64(); n];
            let mut y = vec![0.4 + 0.1 * rng.next_f64(); n];
            let r = 3.7;
            let coupling = 0.3;
            for i in 1..n {
                x[i] = r * x[i - 1] * (1.0 - x[i - 1]);
                let driven = (1.0 - coupling) * y[i - 1] + coupling * x[i - 1];
                y[i] = r * driven * (1.0 - driven);
            }
            let cfg = TransferConfig::new(1, 1, TeBackend::Knn { k: 4 }).unwrap();
            let forward = transfer_entropy(&x, &y, cfg).unwrap();
            let backward = transfer_entropy(&y, &x, cfg).unwrap();
            assert!(forward.value > backward.value, "forward={} backward={}", forward.value, backward.value);
        }
    }
}
