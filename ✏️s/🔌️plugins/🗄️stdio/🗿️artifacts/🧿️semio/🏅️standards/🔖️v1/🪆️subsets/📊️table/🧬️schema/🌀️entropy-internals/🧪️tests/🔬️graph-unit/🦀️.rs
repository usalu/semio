mod tests {
    use super::*;

    #[test]
    fn degree_entropy_of_regular_graph_is_zero() {
        // 🔐️ a 4-cycle: every node has degree 2.
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
        let est = degree_distribution_entropy(&edges, 4, false, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn degree_entropy_rejects_out_of_range_endpoint() {
        let edges = [(0, 5)];
        assert!(matches!(degree_distribution_entropy(&edges, 3, false, LogBase::Bits), Err(EntropyError::ShapeMismatch { .. })));
    }

    #[test]
    fn random_walk_entropy_rate_of_complete_graph_matches_uniform_row_entropy() {
        // 🔐️ K4: every node connects to every other node; each row is uniform over 3 neighbors.
        let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let est = random_walk_entropy_rate(&edges, 4, None, LogBase::Bits).unwrap();
        let expected = 3.0_f64.log2();
        assert!((est.value - expected).abs() < 1e-6, "got {}", est.value);
    }

    #[test]
    fn random_walk_entropy_rate_of_cycle_matches_binary_entropy() {
        // 🔐️ 4-cycle: every row is [0.5, 0.5] over its two neighbors -> 1 bit.
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
        let est = random_walk_entropy_rate(&edges, 4, None, LogBase::Bits).unwrap();
        assert!((est.value - 1.0).abs() < 1e-6, "got {}", est.value);
    }

    #[test]
    fn random_walk_handles_isolated_node() {
        let edges = [(0, 1)];
        let est = random_walk_entropy_rate(&edges, 3, None, LogBase::Bits).unwrap();
        assert!(est.value.is_finite());
    }

    #[test]
    fn random_walk_rejects_negative_weight() {
        let edges = [(0, 1)];
        assert!(random_walk_entropy_rate(&edges, 2, Some(&[-1.0]), LogBase::Bits).is_err());
    }
}
