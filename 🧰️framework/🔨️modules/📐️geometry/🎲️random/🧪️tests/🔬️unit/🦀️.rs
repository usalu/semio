
use super::*;

// #region 🔖️SplitMix64Tests
#[test]
fn split_mix64_is_deterministic_for_same_seed() {
    let mut a = SplitMix64::new(42);
    let mut b = SplitMix64::new(42);
    for _ in 0..16 {
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

#[test]
fn split_mix64_differs_across_seeds() {
    let mut a = SplitMix64::new(1);
    let mut b = SplitMix64::new(2);
    assert_ne!(a.next_u64(), b.next_u64());
}
// #endregion 🔖️SplitMix64Tests

// #region 🔖️RngDeterminismTests
#[test]
fn rng_next_u64_is_deterministic_for_same_seed() {
    let mut a = Rng::from_seed(1234);
    let mut b = Rng::from_seed(1234);
    let mut seq_a: Vec<u64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_a.push(a.next_u64());
    }
    let mut seq_b: Vec<u64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_b.push(b.next_u64());
    }
    assert_eq!(seq_a, seq_b);
}

#[test]
fn rng_next_f64_is_deterministic_for_same_seed() {
    let mut a = Rng::from_seed(9876);
    let mut b = Rng::from_seed(9876);
    let mut seq_a: Vec<f64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_a.push(a.next_f64());
    }
    let mut seq_b: Vec<f64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_b.push(b.next_f64());
    }
    assert_eq!(seq_a, seq_b);
}

#[test]
fn rng_state_round_trip_resumes_identically() {
    let mut original = Rng::from_seed(4242);
    for _ in 0..17 {
        original.next_u64();
    }
    let snapshot = original.state();
    let mut resumed = Rng::from_state(snapshot);
    let mut expected: Vec<u64> = Vec::with_capacity(32);
    for _ in 0..32 {
        expected.push(original.next_u64());
    }
    let mut actual: Vec<u64> = Vec::with_capacity(32);
    for _ in 0..32 {
        actual.push(resumed.next_u64());
    }
    assert_eq!(expected, actual);
}

#[test]
fn rng_different_seeds_diverge() {
    let mut a = Rng::from_seed(1);
    let mut b = Rng::from_seed(2);
    let mut seq_a: Vec<u64> = Vec::with_capacity(8);
    for _ in 0..8 {
        seq_a.push(a.next_u64());
    }
    let mut seq_b: Vec<u64> = Vec::with_capacity(8);
    for _ in 0..8 {
        seq_b.push(b.next_u64());
    }
    assert_ne!(seq_a, seq_b);
}

#[test]
fn rng_has_no_obvious_short_cycle() {
    for seed in [0u64, 1, 42, u64::MAX, 0xDEAD_BEEF] {
        let mut rng = Rng::from_seed(seed);
        let mut draws: Vec<u64> = Vec::with_capacity(2000);
        for _ in 0..2000 {
            draws.push(rng.next_u64());
        }
        let unique: std::collections::HashSet<u64> = draws.iter().copied().collect();
        assert!(unique.len() > 1990, "seed {seed} produced too many repeats: {} unique of 2000", unique.len());
    }
}
// #endregion 🔖️RngDeterminismTests

// #region 🔖️RngStatisticalTests
#[test]
fn next_f64_stays_within_unit_interval() {
    let mut rng = Rng::from_seed(7);
    for _ in 0..10_000 {
        let x = rng.next_f64();
        assert!((0.0..1.0).contains(&x));
    }
}

#[test]
fn next_range_stays_within_bounds_and_is_roughly_uniform() {
    let mut rng = Rng::from_seed(2024);
    let buckets = 10;
    let mut counts = vec![0u64; buckets];
    let draws = 100_000;
    for _ in 0..draws {
        let x = rng.next_range(0, buckets as u64);
        assert!(x < buckets as u64);
        counts[x as usize] += 1;
    }
    let expected = draws as f64 / buckets as f64;
    for count in counts {
        let ratio = count as f64 / expected;
        assert!((0.85..1.15).contains(&ratio), "bucket count {count} too far from expected {expected}");
    }
}

#[test]
fn next_range_degenerate_empty_range_returns_lo() {
    let mut rng = Rng::from_seed(5);
    assert_eq!(rng.next_range(3, 3), 3);
}

#[test]
fn next_bool_respects_extremes() {
    let mut rng = Rng::from_seed(11);
    for _ in 0..100 {
        assert!(!rng.next_bool(0.0));
    }
    for _ in 0..100 {
        assert!(rng.next_bool(1.0));
    }
}

#[test]
fn shuffle_preserves_multiset() {
    let mut rng = Rng::from_seed(99);
    let original: Vec<i32> = (0..50).collect();
    let mut shuffled = original.clone();
    rng.shuffle(&mut shuffled);
    let mut sorted_shuffled = shuffled.clone();
    sorted_shuffled.sort_unstable();
    assert_eq!(sorted_shuffled, original);
    assert_ne!(shuffled, original, "a 50-element shuffle landing on the identity permutation is astronomically unlikely");
}

#[test]
fn choose_returns_none_for_empty_slice() {
    let mut rng = Rng::from_seed(3);
    let empty: Vec<i32> = Vec::new();
    assert_eq!(rng.choose(&empty), None);
}

#[test]
fn choose_always_returns_an_element_from_the_slice() {
    let mut rng = Rng::from_seed(3);
    let items = [10, 20, 30, 40];
    for _ in 0..50 {
        let picked = rng.choose(&items).expect("non-empty slice");
        assert!(items.contains(picked));
    }
}

#[test]
fn sample_without_replacement_returns_k_distinct_indices_in_range() {
    let mut rng = Rng::from_seed(456);
    for (n, k) in [(10, 3), (100, 100), (1000, 1), (50, 0)] {
        let sample = rng.sample_without_replacement(n, k);
        assert_eq!(sample.len(), k);
        let unique: std::collections::HashSet<usize> = sample.iter().copied().collect();
        assert_eq!(unique.len(), k);
        assert!(sample.iter().all(|&i| i < n));
    }
}
// #endregion 🔖️RngStatisticalTests

// #region 🔖️AliasTableTests
#[test]
fn alias_table_sampling_frequency_matches_weights() {
    let weights = [1.0, 2.0, 3.0, 4.0];
    let table = AliasTable::new(&weights);
    let mut rng = Rng::from_seed(321);
    let draws = 200_000;
    let mut counts = [0u64; 4];
    for _ in 0..draws {
        counts[table.sample(&mut rng)] += 1;
    }
    let total: f64 = weights.iter().sum();
    for (i, &w) in weights.iter().enumerate() {
        let expected = draws as f64 * w / total;
        let ratio = counts[i] as f64 / expected;
        assert!((0.9..1.1).contains(&ratio), "index {i} count {} too far from expected {expected}", counts[i]);
    }
}

#[test]
fn alias_table_empty_weights_always_samples_index_zero() {
    let table = AliasTable::new(&[]);
    let mut rng = Rng::from_seed(1);
    for _ in 0..20 {
        assert_eq!(table.sample(&mut rng), 0);
    }
}

#[test]
fn alias_table_all_zero_weights_always_samples_index_zero() {
    let table = AliasTable::new(&[0.0, 0.0, 0.0]);
    let mut rng = Rng::from_seed(2);
    for _ in 0..20 {
        assert_eq!(table.sample(&mut rng), 0);
    }
}

#[test]
fn alias_table_single_weight_always_samples_it() {
    let table = AliasTable::new(&[5.0]);
    let mut rng = Rng::from_seed(3);
    for _ in 0..20 {
        assert_eq!(table.sample(&mut rng), 0);
    }
}
// #endregion 🔖️AliasTableTests

// #region 🔖️DistributionTests
#[test]
fn normal_stays_within_six_std_devs_of_mean() {
    let mut rng = Rng::from_seed(55);
    let mean = 10.0;
    let std_dev = 2.0;
    for _ in 0..10_000 {
        let x = normal(&mut rng, mean, std_dev);
        assert!((x - mean).abs() < 6.0 * std_dev, "normal draw {x} outside 6-sigma band");
    }
}

#[test]
fn geometric_is_always_at_least_one() {
    let mut rng = Rng::from_seed(66);
    for _ in 0..1000 {
        assert!(geometric(&mut rng, 0.3) >= 1);
    }
}

#[test]
fn geometric_p_one_always_returns_one() {
    let mut rng = Rng::from_seed(67);
    for _ in 0..100 {
        assert_eq!(geometric(&mut rng, 1.0), 1);
    }
}

#[test]
fn poisson_lambda_zero_always_returns_zero() {
    let mut rng = Rng::from_seed(77);
    for _ in 0..100 {
        assert_eq!(poisson(&mut rng, 0.0), 0);
    }
}

#[test]
fn poisson_mean_is_roughly_lambda() {
    let mut rng = Rng::from_seed(78);
    let lambda = 4.0;
    let draws = 20_000;
    let mut sum: u64 = 0;
    for _ in 0..draws {
        sum += poisson(&mut rng, lambda);
    }
    let mean = sum as f64 / draws as f64;
    assert!((mean - lambda).abs() < 0.2, "poisson mean {mean} too far from lambda {lambda}");
}

#[test]
fn powerlaw_sequence_values_are_positive_and_at_least_one() {
    let mut rng = Rng::from_seed(88);
    let values = powerlaw_sequence(&mut rng, 500, 2.5);
    assert_eq!(values.len(), 500);
    for v in values {
        assert!(v >= 1.0, "powerlaw value {v} below the theoretical minimum of 1.0");
        assert!(v.is_finite());
    }
}

#[test]
fn zipf_ranks_are_within_bounds() {
    let mut rng = Rng::from_seed(89);
    let n = 100;
    for _ in 0..2000 {
        let rank = zipf(&mut rng, n, 2.0);
        assert!((1..=n as u64).contains(&rank));
    }
}

#[test]
fn zipf_favors_low_ranks() {
    let mut rng = Rng::from_seed(90);
    let n = 20;
    let draws = 20_000;
    let mut low_rank_hits = 0u64;
    for _ in 0..draws {
        if zipf(&mut rng, n, 2.0) <= 2 {
            low_rank_hits += 1;
        }
    }
    assert!(low_rank_hits as f64 / draws as f64 > 0.5, "zipf should heavily favor the lowest ranks");
}

#[test]
fn discrete_sequence_draws_only_from_nonzero_weight_indices() {
    let mut rng = Rng::from_seed(91);
    let distribution = [0.0, 1.0, 0.0, 3.0];
    let draws = discrete_sequence(&mut rng, 200, &distribution);
    assert_eq!(draws.len(), 200);
    for d in draws {
        assert!(d == 1 || d == 3);
    }
}

#[test]
fn cumulative_distribution_ends_at_one_and_is_nondecreasing() {
    let cdf = cumulative_distribution(&[1.0, 2.0, 3.0, 4.0]);
    assert!((cdf.last().unwrap() - 1.0).abs() < 1e-12);
    for pair in cdf.windows(2) {
        assert!(pair[1] >= pair[0]);
    }
}

#[test]
fn cumulative_distribution_of_empty_weights_is_empty() {
    let cdf = cumulative_distribution(&[]);
    assert!(cdf.is_empty());
}
// #endregion 🔖️DistributionTests
