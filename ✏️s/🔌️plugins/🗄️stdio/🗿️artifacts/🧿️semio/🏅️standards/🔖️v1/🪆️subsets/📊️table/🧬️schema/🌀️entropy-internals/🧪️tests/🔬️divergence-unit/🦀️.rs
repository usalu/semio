mod tests {
    use super::*;

    #[test]
    fn kl_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        assert!(kl_divergence(&p, &p, LogBase::Bits).unwrap().abs() < 1e-9);
    }

    #[test]
    fn kl_is_non_negative_for_random_distributions() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        for _ in 0..200 {
            let k = 2 + rng.next_below(5);
            let mut p: Vec<f64> = (0..k).map(|_| rng.next_f64() + 0.01).collect();
            let mut q: Vec<f64> = (0..k).map(|_| rng.next_f64() + 0.01).collect();
            let sp: f64 = p.iter().sum();
            let sq: f64 = q.iter().sum();
            p.iter_mut().for_each(|v| *v /= sp);
            q.iter_mut().for_each(|v| *v /= sq);
            let d = kl_divergence(&p, &q, LogBase::Nats).unwrap();
            assert!(d >= -1e-9, "d={d}");
        }
    }

    #[test]
    fn kl_infinite_on_support_mismatch() {
        let p = [0.5, 0.5];
        let q = [1.0, 0.0];
        assert_eq!(kl_divergence(&p, &q, LogBase::Bits).unwrap(), f64::INFINITY);
    }

    #[test]
    fn js_divergence_symmetric_and_bounded_by_ln2() {
        let p = [0.9, 0.1];
        let q = [0.1, 0.9];
        let a = js_divergence(&p, &q, LogBase::Nats).unwrap();
        let b = js_divergence(&q, &p, LogBase::Nats).unwrap();
        assert!((a - b).abs() < 1e-9);
        assert!(a <= core::f64::consts::LN_2 + 1e-9);
        assert!(a >= 0.0);
    }

    #[test]
    fn js_of_identical_distributions_is_zero() {
        let p = [0.3, 0.7];
        assert!(js_divergence(&p, &p, LogBase::Bits).unwrap().abs() < 1e-9);
    }

    #[test]
    fn hellinger_distance_bounds_and_identity() {
        let p = [0.5, 0.5];
        assert!(hellinger_distance(&p, &p).unwrap().abs() < 1e-9);
        let q = [1.0, 0.0];
        let d = hellinger_distance(&p, &q).unwrap();
        assert!((0.0..=1.0).contains(&d));
    }

    #[test]
    fn total_variation_bounds_and_identity() {
        let p = [0.5, 0.5];
        assert!(total_variation(&p, &p).unwrap().abs() < 1e-9);
        let q = [1.0, 0.0];
        assert!((total_variation(&p, &q).unwrap() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn chi_square_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        assert!(chi_square_divergence(&p, &p).unwrap().abs() < 1e-9);
    }

    #[test]
    fn renyi_divergence_rejects_alpha_one() {
        assert!(matches!(renyi_divergence(&[0.5, 0.5], &[0.3, 0.7], 1.0, LogBase::Bits), Err(EntropyError::UndefinedResult { .. })));
    }

    #[test]
    fn renyi_divergence_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        let d = renyi_divergence(&p, &p, 2.0, LogBase::Nats).unwrap();
        assert!(d.abs() < 1e-9);
    }

    #[test]
    fn tsallis_divergence_limit_matches_kl() {
        let p = [0.2, 0.3, 0.5];
        let q = [0.3, 0.3, 0.4];
        let kl = kl_divergence(&p, &q, LogBase::Nats).unwrap();
        let tsallis = tsallis_divergence(&p, &q, 1.0).unwrap();
        assert!((kl - tsallis).abs() < 1e-6);
    }

    #[test]
    fn wasserstein_1d_of_identical_samples_is_zero() {
        let x = [1.0, 2.0, 3.0, 4.0];
        assert!(wasserstein_1d(&x, &x).unwrap().abs() < 1e-9);
    }

    #[test]
    fn wasserstein_1d_matches_hand_computation_equal_size() {
        // 🔐️ equal-size sorted samples: W1 = mean |x_sorted - y_sorted|.
        let x = [1.0, 2.0, 3.0];
        let y = [4.0, 5.0, 6.0];
        let w = wasserstein_1d(&x, &y).unwrap();
        assert!((w - 3.0).abs() < 1e-9);
    }

    #[test]
    fn energy_distance_of_identical_distributions_is_near_zero() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(energy_distance(&x, &x).unwrap().abs() < 1e-9);
    }

    #[test]
    fn energy_distance_is_non_negative() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..30).map(|_| rng.next_gaussian()).collect();
        let y: Vec<f64> = (0..30).map(|_| rng.next_gaussian() + 2.0).collect();
        assert!(energy_distance(&x, &y).unwrap() > 0.0);
    }

    #[test]
    fn log_det_divergence_of_identical_matrices_is_zero() {
        let cov = vec![4.0, 1.0, 1.0, 3.0];
        let d = log_det_divergence(&cov, &cov, 2).unwrap();
        assert!(d.abs() < 1e-7);
    }

    #[test]
    fn bregman_squared_euclidean_matches_direct_formula() {
        let p = [1.0, 2.0, 3.0];
        let q = [0.5, 2.5, 2.0];
        let phi = |x: &[f64]| x.iter().map(|v| v * v).sum::<f64>();
        let grad = |x: &[f64]| -> Vec<f64> { x.iter().map(|v| 2.0 * v).collect() };
        let d = bregman_divergence(&p, &q, phi, grad).unwrap();
        let expected: f64 = p.iter().zip(q.iter()).map(|(a, b)| (a - b).powi(2)).sum();
        assert!((d - expected).abs() < 1e-9);
    }

    #[test]
    fn itakura_saito_of_identical_spectra_is_zero() {
        let p = [1.0, 2.0, 3.0];
        assert!(itakura_saito_divergence(&p, &p).unwrap().abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn renyi_divergence_monotone_in_alpha() {
            let p = [0.6, 0.3, 0.1];
            let q = [0.2, 0.3, 0.5];
            let alphas = [0.1, 0.5, 2.0, 5.0];
            let mut prev = renyi_divergence(&p, &q, alphas[0], LogBase::Nats).unwrap();
            for &a in &alphas[1..] {
                let d = renyi_divergence(&p, &q, a, LogBase::Nats).unwrap();
                assert!(d >= prev - 1e-9, "alpha={a}");
                prev = d;
            }
        }
    }
}
