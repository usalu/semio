mod tests {
    use super::*;

    #[test]
    fn streaming_counts_update_matches_batch_entropy() {
        let mut sc = StreamingCounts::new(4, LogBase::Bits);
        for &x in &[0u32, 1, 1, 2, 2, 2, 3] {
            sc.update(x);
        }
        let est = sc.estimate().unwrap();
        let counts = crate::standards::v1::subsets::table::schema::entropy_internals::counts::Counts::from_symbols(&[0, 1, 1, 2, 2, 2, 3], 4).unwrap();
        let expected = crate::standards::v1::subsets::table::schema::entropy_internals::discrete::entropy(&counts.probabilities(), LogBase::Bits).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn streaming_counts_remove_undoes_update() {
        let mut sc = StreamingCounts::new(3, LogBase::Nats);
        sc.update(0);
        sc.update(1);
        sc.remove(0).unwrap();
        assert_eq!(sc.n_raw, 1);
    }

    #[test]
    fn streaming_counts_remove_rejects_unobserved_symbol() {
        let mut sc = StreamingCounts::new(3, LogBase::Nats);
        sc.update(0);
        assert!(sc.remove(1).is_err());
    }

    #[test]
    fn streaming_counts_merge_matches_combined_batch() {
        let mut a = StreamingCounts::new(3, LogBase::Nats);
        let mut b = StreamingCounts::new(3, LogBase::Nats);
        for &x in &[0u32, 1, 1] {
            a.update(x);
        }
        for &x in &[2u32, 2, 0] {
            b.update(x);
        }
        a.merge(&b).unwrap();
        let est = a.estimate().unwrap();
        let counts = crate::standards::v1::subsets::table::schema::entropy_internals::counts::Counts::from_symbols(&[0, 1, 1, 2, 2, 0], 3).unwrap();
        let expected = crate::standards::v1::subsets::table::schema::entropy_internals::discrete::entropy(&counts.probabilities(), LogBase::Nats).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn streaming_counts_snapshot_restore_roundtrips() {
        let mut sc = StreamingCounts::new(3, LogBase::Bits);
        for &x in &[0u32, 1, 2, 2] {
            sc.update(x);
        }
        let snap = sc.snapshot();
        let restored = StreamingCounts::restore(&snap).unwrap();
        assert_eq!(sc.estimate().unwrap().value, restored.estimate().unwrap().value);
    }

    #[test]
    fn sliding_window_matches_batch_recomputed_at_every_step() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let capacity = 20;
        let mut sw = SlidingWindowEntropy::new(4, capacity, LogBase::Nats).unwrap();
        let mut history: Vec<u32> = Vec::new();
        for _ in 0..200 {
            let x = rng.next_below(4) as u32;
            sw.update(x);
            history.push(x);
            let window_start = history.len().saturating_sub(capacity);
            let window = &history[window_start..];
            let counts = crate::standards::v1::subsets::table::schema::entropy_internals::counts::Counts::from_symbols(window, 4).unwrap();
            let expected = crate::standards::v1::subsets::table::schema::entropy_internals::discrete::entropy(&counts.probabilities(), LogBase::Nats).unwrap();
            let got = sw.estimate().unwrap().value;
            assert!((got - expected).abs() < 1e-9, "mismatch at len {}", history.len());
        }
    }

    #[test]
    fn sliding_window_remove_and_merge_are_unsupported() {
        let mut sw = SlidingWindowEntropy::new(3, 5, LogBase::Nats).unwrap();
        sw.update(0);
        assert!(sw.remove(0).is_err());
        let other = SlidingWindowEntropy::new(3, 5, LogBase::Nats).unwrap();
        assert!(sw.merge(&other).is_err());
    }

    #[test]
    fn decayed_entropy_rejects_bad_decay() {
        assert!(DecayedEntropy::new(3, 0.0, LogBase::Nats).is_err());
        assert!(DecayedEntropy::new(3, 1.5, LogBase::Nats).is_err());
    }

    #[test]
    fn decayed_entropy_remove_is_unsupported() {
        let mut de = DecayedEntropy::new(3, 0.9, LogBase::Nats).unwrap();
        de.update(0);
        assert!(de.remove(0).is_err());
    }

    #[test]
    fn decayed_entropy_forgets_old_symbols() {
        let mut de = DecayedEntropy::new(2, 0.5, LogBase::Bits).unwrap();
        for _ in 0..50 {
            de.update(0);
        }
        // 🔐️ after many decayed updates of the same symbol, entropy should be near zero
        // (essentially deterministic), then adding a burst of the other symbol should raise it.
        let before = de.estimate().unwrap().value;
        assert!(before < 0.1, "got {before}");
        for _ in 0..50 {
            de.update(1);
        }
        let after = de.estimate().unwrap().value;
        assert!(after < 0.5, "got {after}"); // 🔐️ decay erased symbol-0 history; now near-deterministic on symbol 1
    }

    #[test]
    fn decayed_entropy_snapshot_restore_roundtrips() {
        let mut de = DecayedEntropy::new(3, 0.8, LogBase::Nats).unwrap();
        de.update(0);
        de.update(1);
        let snap = de.snapshot();
        let restored = DecayedEntropy::restore(&snap).unwrap();
        assert!((de.estimate().unwrap().value - restored.estimate().unwrap().value).abs() < 1e-12);
    }
}
