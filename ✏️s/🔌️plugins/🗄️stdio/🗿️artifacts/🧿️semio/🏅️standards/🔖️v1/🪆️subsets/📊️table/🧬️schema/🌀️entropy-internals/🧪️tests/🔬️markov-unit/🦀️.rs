mod tests {
    use super::*;
    use crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64;

    /// ⛓️ Generates a sequence from a hand-specified 2-state chain (`transition[i][j] = P(i -> j)`)
    /// starting at state 0, using a deterministic PRNG so tests are exactly reproducible.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn generate_two_state_sequence(transition: [[f64; 2]; 2], n: usize, seed: u64) -> Vec<u32> {
        let mut rng = Xorshift64::new(seed);
        let mut seq = Vec::with_capacity(n);
        let mut state = 0u32;
        for _ in 0..n {
            seq.push(state);
            let p_stay = transition[state as usize][state as usize];
            state = if rng.next_f64() < p_stay { state } else { 1 - state };
        }
        seq
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn binary_entropy_nats(p: f64) -> f64 {
        -(x_ln_x(p) + x_ln_x(1.0 - p))
    }

    #[test]
    fn fit_rejects_too_short_sequence() {
        let seq = [0u32, 1];
        let result = MarkovChain::fit(&seq, 2, 2);
        assert!(matches!(result, Err(EntropyError::InsufficientData { .. })));
    }

    #[test]
    fn fit_rejects_order_zero() {
        let seq = [0u32, 1, 0, 1];
        assert!(matches!(MarkovChain::fit(&seq, 2, 0), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn fit_rejects_out_of_range_symbol() {
        let seq = [0u32, 1, 2, 0];
        assert!(matches!(MarkovChain::fit(&seq, 2, 1), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn order_one_matches_ordinary_transition_counts() {
        let seq = [0u32, 0, 1, 0, 1, 1, 1, 0];
        let chain = MarkovChain::fit(&seq, 2, 1).unwrap();
        let mut expected = [[0.0_f64; 2]; 2];
        for w in seq.windows(2) {
            expected[w[0] as usize][w[1] as usize] += 1.0;
        }
        for (context, row) in expected.iter().enumerate() {
            for (next, &expected_count) in row.iter().enumerate() {
                assert_eq!(chain.counts[context * 2 + next], expected_count, "context={context} next={next}");
            }
        }
    }

    #[test]
    fn periodic_two_cycle_has_near_zero_entropy_rate() {
        let seq: Vec<u32> = (0..100u32).map(|i| i % 2).collect();
        let chain = MarkovChain::fit(&seq, 2, 1).unwrap();
        let estimate = chain.entropy_rate(LogBase::Bits).unwrap();
        assert!(estimate.value.abs() < 1e-9, "value={}", estimate.value);
        let pi = chain.stationary().unwrap();
        assert!((pi[0] - 0.5).abs() < 1e-6, "pi={pi:?}");
        assert!((pi[1] - 0.5).abs() < 1e-6, "pi={pi:?}");
    }

    #[test]
    fn entropy_rate_diagnostics_report_order_and_alphabet() {
        let seq = [0u32, 1, 0, 1, 0, 1];
        let chain = MarkovChain::fit(&seq, 2, 1).unwrap();
        let estimate = chain.entropy_rate(LogBase::Nats).unwrap();
        assert_eq!(estimate.method, "markov_entropy_rate");
        assert_eq!(estimate.n, seq.len());
        assert_eq!(estimate.diagnostics, vec![("order", 1.0), ("alphabet_size", 2.0)]);
    }

    mod quick {
        use super::*;

        #[test]
        fn two_state_chain_converges_to_analytic_stationary_and_entropy_rate() {
            // ⛓️ pi P = pi for [[0.9, 0.1], [0.5, 0.5]] solves to pi = (5/6, 1/6).
            let transition = [[0.9, 0.1], [0.5, 0.5]];
            let seq = generate_two_state_sequence(transition, 50_000, 12_345);
            let chain = MarkovChain::fit(&seq, 2, 1).unwrap();

            let pi = chain.stationary().unwrap();
            let pi0_expected = 5.0 / 6.0;
            let pi1_expected = 1.0 / 6.0;
            assert!((pi[0] - pi0_expected).abs() < 0.02, "pi={pi:?}");
            assert!((pi[1] - pi1_expected).abs() < 0.02, "pi={pi:?}");

            let expected_nats = pi0_expected * binary_entropy_nats(0.9) + pi1_expected * binary_entropy_nats(0.5);
            let estimate = chain.entropy_rate(LogBase::Nats).unwrap();
            assert!((estimate.value - expected_nats).abs() < 0.02, "value={} expected={expected_nats}", estimate.value);
        }
    }
}
