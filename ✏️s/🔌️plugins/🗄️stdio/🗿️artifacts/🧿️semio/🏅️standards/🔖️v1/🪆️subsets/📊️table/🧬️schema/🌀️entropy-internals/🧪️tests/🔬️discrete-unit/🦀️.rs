mod tests {
    use super::*;

    #[test]
    fn uniform_entropy_equals_log_support_size() {
        for k in [2usize, 3, 8, 16] {
            let p = vec![1.0 / k as f64; k];
            let h = entropy(&p, LogBase::Nats).unwrap();
            assert!((h - (k as f64).ln()).abs() < 1e-9, "k={k}");
        }
    }

    #[test]
    fn fair_coin_entropy_is_one_bit() {
        let h = binary_entropy(0.5, LogBase::Bits).unwrap();
        assert!((h - 1.0).abs() < 1e-9);
    }

    #[test]
    fn deterministic_distribution_entropy_is_zero() {
        let h = entropy(&[1.0, 0.0, 0.0], LogBase::Bits).unwrap();
        assert!(h.abs() < 1e-12);
    }

    #[test]
    fn entropy_rejects_empty() {
        assert!(matches!(entropy(&[], LogBase::Bits), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn cross_entropy_of_identical_distributions_equals_entropy() {
        let p = [0.2, 0.3, 0.5];
        let h = entropy(&p, LogBase::Bits).unwrap();
        let ce = cross_entropy(&p, &p, LogBase::Bits).unwrap();
        assert!((h - ce).abs() < 1e-9);
    }

    #[test]
    fn cross_entropy_infinite_on_support_mismatch() {
        let p = [0.5, 0.5];
        let q = [1.0, 0.0];
        assert_eq!(cross_entropy(&p, &q, LogBase::Bits).unwrap(), f64::INFINITY);
    }

    #[test]
    fn chain_rule_holds_for_joint_entropy() {
        // 🔐️ X,Y independent uniform over {0,1}: H(X,Y) = H(X) + H(Y|X) = 2 bits.
        let joint = [0.25, 0.25, 0.25, 0.25];
        let h_xy = joint_entropy(&joint, LogBase::Bits).unwrap();
        let h_x = entropy(&[0.5, 0.5], LogBase::Bits).unwrap();
        let h_y_given_x = conditional_entropy(&joint, 2, 2, LogBase::Bits).unwrap();
        assert!((h_xy - (h_x + h_y_given_x)).abs() < 1e-9);
        assert!((h_xy - 2.0).abs() < 1e-9);
    }

    #[test]
    fn conditional_entropy_zero_when_y_determined_by_x() {
        // 🔐️ Y = X exactly: H(Y|X) = 0.
        let joint = [0.5, 0.0, 0.0, 0.5];
        let h = conditional_entropy(&joint, 2, 2, LogBase::Bits).unwrap();
        assert!(h.abs() < 1e-9);
    }

    #[test]
    fn hartley_entropy_matches_log_k() {
        assert!((hartley_entropy(8, LogBase::Bits).unwrap() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn renyi_rejects_alpha_exactly_one() {
        assert!(matches!(renyi_entropy(&[0.5, 0.5], 1.0, LogBase::Bits), Err(EntropyError::UndefinedResult { .. })));
    }

    #[test]
    fn renyi_alpha_zero_equals_hartley_over_support() {
        let p = [0.5, 0.5, 0.0];
        let h = renyi_entropy(&p, 0.0, LogBase::Bits).unwrap();
        assert!((h - 1.0).abs() < 1e-9); // support size 2 -> log2(2) = 1
    }

    #[test]
    fn renyi_alpha_two_equals_collision_entropy() {
        let p = [0.5, 0.25, 0.25];
        let a = renyi_entropy(&p, 2.0, LogBase::Bits).unwrap();
        let b = collision_entropy(&p, LogBase::Bits).unwrap();
        assert!((a - b).abs() < 1e-12);
    }

    #[test]
    fn renyi_continuity_near_alpha_one_approaches_shannon() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let near = renyi_entropy(&p, 1.0 + 1e-6, LogBase::Nats).unwrap();
        assert!((near - shannon).abs() < 1e-4);
    }

    #[test]
    fn min_entropy_matches_negative_log_max_probability() {
        let p = [0.6, 0.3, 0.1];
        let h = min_entropy(&p, LogBase::Bits).unwrap();
        assert!((h - (-(0.6_f64.log2()))).abs() < 1e-9);
    }

    #[test]
    fn tsallis_limit_at_q_one_equals_shannon_nats() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let tsallis = tsallis_entropy(&p, 1.0).unwrap();
        assert!((tsallis - shannon).abs() < 1e-9);
    }

    #[test]
    fn tsallis_uniform_matches_closed_form() {
        // 🔐️ S_q(uniform_k) = (1 - k^(1-q)) / (q - 1)
        let k = 4;
        let p = vec![1.0 / k as f64; k];
        let q = 2.0;
        let expected = (1.0 - (k as f64).powf(1.0 - q)) / (q - 1.0);
        assert!((tsallis_entropy(&p, q).unwrap() - expected).abs() < 1e-9);
    }

    #[test]
    fn kaniadakis_limit_at_kappa_zero_equals_shannon_nats() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let kaniadakis = kaniadakis_entropy(&p, 0.0).unwrap();
        assert!((kaniadakis - shannon).abs() < 1e-9);
    }

    #[test]
    fn sharma_mittal_reduces_to_renyi_as_beta_approaches_one() {
        let p = [0.2, 0.3, 0.5];
        let alpha = 2.0;
        let renyi = renyi_entropy(&p, alpha, LogBase::Nats).unwrap();
        let sm = sharma_mittal_entropy(&p, alpha, 1.0 + 1e-7).unwrap();
        assert!((sm - renyi).abs() < 1e-4);
    }

    #[test]
    fn normalized_entropy_of_uniform_is_one() {
        let p = [0.25, 0.25, 0.25, 0.25];
        assert!((normalized_entropy(&p, LogBase::Bits).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn normalized_entropy_of_deterministic_is_zero() {
        let p = [1.0, 0.0, 0.0, 0.0];
        assert!(normalized_entropy(&p, LogBase::Bits).unwrap().abs() < 1e-12);
    }

    #[test]
    fn entropy_non_negative_for_random_distributions() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(2024);
        for _ in 0..200 {
            let k = 2 + (rng.next_below(5));
            let mut raw: Vec<f64> = (0..k).map(|_| rng.next_f64()).collect();
            let sum: f64 = raw.iter().sum();
            for v in &mut raw {
                *v /= sum;
            }
            let h = entropy(&raw, LogBase::Bits).unwrap();
            assert!(h >= -1e-9);
            assert!(h <= (k as f64).log2() + 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn renyi_is_monotone_nonincreasing_in_alpha() {
            let p = [0.6, 0.3, 0.1];
            let alphas = [-2.0, -0.5, 0.5, 2.0, 5.0, 20.0];
            let mut prev = renyi_entropy(&p, alphas[0], LogBase::Nats).unwrap();
            for &a in &alphas[1..] {
                let h = renyi_entropy(&p, a, LogBase::Nats).unwrap();
                assert!(h <= prev + 1e-9, "alpha={a} h={h} prev={prev}");
                prev = h;
            }
        }
    }
}
