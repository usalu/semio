mod tests {
    use super::*;
    use crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64;

    // #region 🔖️TwoSourceLogicGates
    #[test]
    fn copy_gate_shows_dominant_unique1_and_near_zero_synergy() {
        // 🔐️ T = S1 exactly, S2 independent noise: all info about T is uniquely S1's.
        let mut rng = Xorshift64::new(101);
        let n = 3000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target = s1.clone();
        let atoms = pid_two_sources(&s1, &s2, &target, (2, 2, 2), LogBase::Nats).unwrap();
        assert!(atoms.unique_1 > 0.5, "unique_1={}", atoms.unique_1);
        assert!(atoms.redundancy < 0.1, "redundancy={}", atoms.redundancy);
        assert!(atoms.unique_2 < 0.1, "unique_2={}", atoms.unique_2);
        assert!(atoms.synergy.abs() < 0.1, "synergy={}", atoms.synergy);
    }

    #[test]
    fn xor_gate_shows_dominant_synergy_near_one_bit() {
        // 🔐️ T = S1 XOR S2, S1/S2 independent fair coins: the classic pure-synergy example.
        let mut rng = Xorshift64::new(202);
        let n = 4000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s2.iter()).map(|(&a, &b)| a ^ b).collect();
        let atoms = pid_two_sources(&s1, &s2, &target, (2, 2, 2), LogBase::Nats).unwrap();
        assert!((atoms.synergy - core::f64::consts::LN_2).abs() < 0.1, "synergy={}", atoms.synergy);
        assert!(atoms.redundancy < 0.1, "redundancy={}", atoms.redundancy);
        assert!(atoms.unique_1 < 0.1, "unique_1={}", atoms.unique_1);
        assert!(atoms.unique_2 < 0.1, "unique_2={}", atoms.unique_2);
    }
    // #endregion 🔖️TwoSourceLogicGates

    // #region 🔖️TwoSourceValidation
    #[test]
    fn pid_two_sources_rejects_length_mismatch() {
        assert!(matches!(pid_two_sources(&[0, 1], &[0], &[0, 1], (2, 1, 2), LogBase::Nats), Err(EntropyError::LengthMismatch { .. })));
    }

    #[test]
    fn pid_two_sources_rejects_empty_input() {
        assert!(matches!(pid_two_sources(&[], &[], &[], (2, 2, 2), LogBase::Nats), Err(EntropyError::EmptyInput { .. })));
    }
    // #endregion 🔖️TwoSourceValidation

    // #region 🔖️Lattice
    #[test]
    fn lattice_has_exactly_eighteen_nodes() {
        let mut rng = Xorshift64::new(303);
        let n = 1000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        assert_eq!(lattice.node_count(), 18);
    }

    #[test]
    fn lattice_rejects_source_counts_other_than_three() {
        let s1 = [0u32, 1, 0, 1];
        let target = [0u32, 1, 1, 0];
        assert!(matches!(PidLattice::compute(&[&s1, &s1], &target, &[2, 2], 2, LogBase::Nats), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn full_set_node_i_min_equals_total_joint_mi_and_singletons_node_is_smaller() {
        let mut rng = Xorshift64::new(404);
        let n = 2000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s2.iter()).map(|(&a, &b)| a ^ b).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        let full_set_idx = lattice.nodes.iter().position(|node| node == &vec![7u32]).unwrap();
        let singletons_idx = lattice.nodes.iter().position(|node| node == &vec![1u32, 2, 4]).unwrap();
        assert!((lattice.i_min_nats[full_set_idx] - lattice.total_mi_nats).abs() < 1e-9);
        assert!(lattice.i_min_nats[singletons_idx] <= lattice.i_min_nats[full_set_idx] + 1e-9);
    }

    #[test]
    fn sum_of_all_partial_information_equals_total_mutual_information() {
        // 🔐️ the critical Mobius/zeta consistency check: sum(Pi) over all 18 nodes must equal the
        // total joint MI exactly (an algebraic identity of the inversion, not a statistical
        // convergence property), on several independently seeded random datasets.
        for seed in [11u64, 22u64, 33u64] {
            let mut rng = Xorshift64::new(seed);
            let n = 1500;
            let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
            let s2: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
            let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
            let target: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
            let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 3, 2], 3, LogBase::Nats).unwrap();
            let sum_pi: f64 = lattice.partial_info_nats.iter().sum();
            let total_mi = lattice.total_mutual_information(LogBase::Nats);
            assert!((sum_pi - total_mi).abs() < 1e-6, "seed={seed} sum_pi={sum_pi} total_mi={total_mi}");
        }
    }

    #[test]
    fn partial_information_lookup_matches_internal_full_set_node() {
        let mut rng = Xorshift64::new(505);
        let n = 1200;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s3.iter()).map(|(&a, &b)| a ^ b).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        let looked_up = lattice.partial_information(&[vec![0, 1, 2]]).unwrap();
        let full_set_idx = lattice.nodes.iter().position(|node| node == &vec![7u32]).unwrap();
        assert!((looked_up - lattice.partial_info_nats[full_set_idx]).abs() < 1e-12);
        // 🔐️ an antichain referencing a source index that doesn't correspond to any lattice node
        // (99 is out of range for a 3-source lattice) must return `None`, not panic.
        assert!(lattice.partial_information(&[vec![0], vec![1], vec![99]]).is_none());
    }
    // #endregion 🔖️Lattice
}
