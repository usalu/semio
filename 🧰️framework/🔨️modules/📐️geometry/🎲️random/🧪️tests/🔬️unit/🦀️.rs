
use super::*;

// #region 🔖️SplitMix64Tests
#[semio_framework_async_macros::async_test]
async fn split_mix64_is_deterministic_for_same_seed() {
    let mut a = SplitMix64::new(42).await;
    let mut b = SplitMix64::new(42).await;
    for _ in 0..16 {
        assert_eq!(a.next_u64().await, b.next_u64().await);
    }
}

#[semio_framework_async_macros::async_test]
async fn split_mix64_differs_across_seeds() {
    let mut a = SplitMix64::new(1).await;
    let mut b = SplitMix64::new(2).await;
    assert_ne!(a.next_u64().await, b.next_u64().await);
}
// #endregion 🔖️SplitMix64Tests

// #region 🔖️RngDeterminismTests
#[semio_framework_async_macros::async_test]
async fn rng_next_u64_is_deterministic_for_same_seed() {
    let mut a = Rng::from_seed(1234).await;
    let mut b = Rng::from_seed(1234).await;
    let mut seq_a: Vec<u64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_a.push(a.next_u64().await);
    }
    let mut seq_b: Vec<u64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_b.push(b.next_u64().await);
    }
    assert_eq!(seq_a, seq_b);
}

#[semio_framework_async_macros::async_test]
async fn rng_next_f64_is_deterministic_for_same_seed() {
    let mut a = Rng::from_seed(9876).await;
    let mut b = Rng::from_seed(9876).await;
    let mut seq_a: Vec<f64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_a.push(a.next_f64().await);
    }
    let mut seq_b: Vec<f64> = Vec::with_capacity(64);
    for _ in 0..64 {
        seq_b.push(b.next_f64().await);
    }
    assert_eq!(seq_a, seq_b);
}

#[semio_framework_async_macros::async_test]
async fn rng_state_round_trip_resumes_identically() {
    let mut original = Rng::from_seed(4242).await;
    for _ in 0..17 {
        original.next_u64().await;
    }
    let snapshot = original.state().await;
    let mut resumed = Rng::from_state(snapshot).await;
    let mut expected: Vec<u64> = Vec::with_capacity(32);
    for _ in 0..32 {
        expected.push(original.next_u64().await);
    }
    let mut actual: Vec<u64> = Vec::with_capacity(32);
    for _ in 0..32 {
        actual.push(resumed.next_u64().await);
    }
    assert_eq!(expected, actual);
}

#[semio_framework_async_macros::async_test]
async fn rng_different_seeds_diverge() {
    let mut a = Rng::from_seed(1).await;
    let mut b = Rng::from_seed(2).await;
    let mut seq_a: Vec<u64> = Vec::with_capacity(8);
    for _ in 0..8 {
        seq_a.push(a.next_u64().await);
    }
    let mut seq_b: Vec<u64> = Vec::with_capacity(8);
    for _ in 0..8 {
        seq_b.push(b.next_u64().await);
    }
    assert_ne!(seq_a, seq_b);
}

#[semio_framework_async_macros::async_test]
async fn rng_has_no_obvious_short_cycle() {
    for seed in [0u64, 1, 42, u64::MAX, 0xDEAD_BEEF] {
        let mut rng = Rng::from_seed(seed).await;
        let mut draws: Vec<u64> = Vec::with_capacity(2000);
        for _ in 0..2000 {
            draws.push(rng.next_u64().await);
        }
        let unique: std::collections::HashSet<u64> = draws.iter().copied().collect();
        assert!(unique.len() > 1990, "seed {seed} produced too many repeats: {} unique of 2000", unique.len());
    }
}
// #endregion 🔖️RngDeterminismTests

// #region 🔖️RngStatisticalTests
#[semio_framework_async_macros::async_test]
async fn next_f64_stays_within_unit_interval() {
    let mut rng = Rng::from_seed(7).await;
    for _ in 0..10_000 {
        let x = rng.next_f64().await;
        assert!((0.0..1.0).contains(&x));
    }
}

#[semio_framework_async_macros::async_test]
async fn next_range_stays_within_bounds_and_is_roughly_uniform() {
    let mut rng = Rng::from_seed(2024).await;
    let buckets = 10;
    let mut counts = vec![0u64; buckets];
    let draws = 100_000;
    for _ in 0..draws {
        let x = rng.next_range(0, buckets as u64).await;
        assert!(x < buckets as u64);
        counts[x as usize] += 1;
    }
    let expected = draws as f64 / buckets as f64;
    for count in counts {
        let ratio = count as f64 / expected;
        assert!((0.85..1.15).contains(&ratio), "bucket count {count} too far from expected {expected}");
    }
}

#[semio_framework_async_macros::async_test]
async fn next_range_degenerate_empty_range_returns_lo() {
    let mut rng = Rng::from_seed(5).await;
    assert_eq!(rng.next_range(3, 3).await, 3);
}

#[semio_framework_async_macros::async_test]
async fn next_bool_respects_extremes() {
    let mut rng = Rng::from_seed(11).await;
    for _ in 0..100 {
        assert!(!rng.next_bool(0.0).await);
    }
    for _ in 0..100 {
        assert!(rng.next_bool(1.0).await);
    }
}

#[semio_framework_async_macros::async_test]
async fn shuffle_preserves_multiset() {
    let mut rng = Rng::from_seed(99).await;
    let original: Vec<i32> = (0..50).collect();
    let mut shuffled = original.clone();
    rng.shuffle(&mut shuffled).await;
    let mut sorted_shuffled = shuffled.clone();
    sorted_shuffled.sort_unstable();
    assert_eq!(sorted_shuffled, original);
    assert_ne!(shuffled, original, "a 50-element shuffle landing on the identity permutation is astronomically unlikely");
}

#[semio_framework_async_macros::async_test]
async fn choose_returns_none_for_empty_slice() {
    let mut rng = Rng::from_seed(3).await;
    let empty: Vec<i32> = Vec::new();
    assert_eq!(rng.choose(&empty).await, None);
}

#[semio_framework_async_macros::async_test]
async fn choose_always_returns_an_element_from_the_slice() {
    let mut rng = Rng::from_seed(3).await;
    let items = [10, 20, 30, 40];
    for _ in 0..50 {
        let picked = rng.choose(&items).await.expect("non-empty slice");
        assert!(items.contains(picked));
    }
}

#[semio_framework_async_macros::async_test]
async fn sample_without_replacement_returns_k_distinct_indices_in_range() {
    let mut rng = Rng::from_seed(456).await;
    for (n, k) in [(10, 3), (100, 100), (1000, 1), (50, 0)] {
        let sample = rng.sample_without_replacement(n, k).await;
        assert_eq!(sample.len(), k);
        let unique: std::collections::HashSet<usize> = sample.iter().copied().collect();
        assert_eq!(unique.len(), k);
        assert!(sample.iter().all(|&i| i < n));
    }
}
// #endregion 🔖️RngStatisticalTests

// #region 🔖️AliasTableTests
#[semio_framework_async_macros::async_test]
async fn alias_table_sampling_frequency_matches_weights() {
    let weights = [1.0, 2.0, 3.0, 4.0];
    let table = AliasTable::new(&weights).await;
    let mut rng = Rng::from_seed(321).await;
    let draws = 200_000;
    let mut counts = [0u64; 4];
    for _ in 0..draws {
        counts[table.sample(&mut rng).await] += 1;
    }
    let total: f64 = weights.iter().sum();
    for (i, &w) in weights.iter().enumerate() {
        let expected = draws as f64 * w / total;
        let ratio = counts[i] as f64 / expected;
        assert!((0.9..1.1).contains(&ratio), "index {i} count {} too far from expected {expected}", counts[i]);
    }
}

#[semio_framework_async_macros::async_test]
async fn alias_table_empty_weights_always_samples_index_zero() {
    let table = AliasTable::new(&[]).await;
    let mut rng = Rng::from_seed(1).await;
    for _ in 0..20 {
        assert_eq!(table.sample(&mut rng).await, 0);
    }
}

#[semio_framework_async_macros::async_test]
async fn alias_table_all_zero_weights_always_samples_index_zero() {
    let table = AliasTable::new(&[0.0, 0.0, 0.0]).await;
    let mut rng = Rng::from_seed(2).await;
    for _ in 0..20 {
        assert_eq!(table.sample(&mut rng).await, 0);
    }
}

#[semio_framework_async_macros::async_test]
async fn alias_table_single_weight_always_samples_it() {
    let table = AliasTable::new(&[5.0]).await;
    let mut rng = Rng::from_seed(3).await;
    for _ in 0..20 {
        assert_eq!(table.sample(&mut rng).await, 0);
    }
}
// #endregion 🔖️AliasTableTests

// #region 🔖️DistributionTests
#[semio_framework_async_macros::async_test]
async fn normal_stays_within_six_std_devs_of_mean() {
    let mut rng = Rng::from_seed(55).await;
    let mean = 10.0;
    let std_dev = 2.0;
    for _ in 0..10_000 {
        let x = normal(&mut rng, mean, std_dev).await;
        assert!((x - mean).abs() < 6.0 * std_dev, "normal draw {x} outside 6-sigma band");
    }
}

#[semio_framework_async_macros::async_test]
async fn geometric_is_always_at_least_one() {
    let mut rng = Rng::from_seed(66).await;
    for _ in 0..1000 {
        assert!(geometric(&mut rng, 0.3).await >= 1);
    }
}

#[semio_framework_async_macros::async_test]
async fn geometric_p_one_always_returns_one() {
    let mut rng = Rng::from_seed(67).await;
    for _ in 0..100 {
        assert_eq!(geometric(&mut rng, 1.0).await, 1);
    }
}

#[semio_framework_async_macros::async_test]
async fn poisson_lambda_zero_always_returns_zero() {
    let mut rng = Rng::from_seed(77).await;
    for _ in 0..100 {
        assert_eq!(poisson(&mut rng, 0.0).await, 0);
    }
}

#[semio_framework_async_macros::async_test]
async fn poisson_mean_is_roughly_lambda() {
    let mut rng = Rng::from_seed(78).await;
    let lambda = 4.0;
    let draws = 20_000;
    let mut sum: u64 = 0;
    for _ in 0..draws {
        sum += poisson(&mut rng, lambda).await;
    }
    let mean = sum as f64 / draws as f64;
    assert!((mean - lambda).abs() < 0.2, "poisson mean {mean} too far from lambda {lambda}");
}

#[semio_framework_async_macros::async_test]
async fn powerlaw_sequence_values_are_positive_and_at_least_one() {
    let mut rng = Rng::from_seed(88).await;
    let values = powerlaw_sequence(&mut rng, 500, 2.5).await;
    assert_eq!(values.len(), 500);
    for v in values {
        assert!(v >= 1.0, "powerlaw value {v} below the theoretical minimum of 1.0");
        assert!(v.is_finite());
    }
}

#[semio_framework_async_macros::async_test]
async fn zipf_ranks_are_within_bounds() {
    let mut rng = Rng::from_seed(89).await;
    let n = 100;
    for _ in 0..2000 {
        let rank = zipf(&mut rng, n, 2.0).await;
        assert!((1..=n as u64).contains(&rank));
    }
}

#[semio_framework_async_macros::async_test]
async fn zipf_favors_low_ranks() {
    let mut rng = Rng::from_seed(90).await;
    let n = 20;
    let draws = 20_000;
    let mut low_rank_hits = 0u64;
    for _ in 0..draws {
        if zipf(&mut rng, n, 2.0).await <= 2 {
            low_rank_hits += 1;
        }
    }
    assert!(low_rank_hits as f64 / draws as f64 > 0.5, "zipf should heavily favor the lowest ranks");
}

#[semio_framework_async_macros::async_test]
async fn discrete_sequence_draws_only_from_nonzero_weight_indices() {
    let mut rng = Rng::from_seed(91).await;
    let distribution = [0.0, 1.0, 0.0, 3.0];
    let draws = discrete_sequence(&mut rng, 200, &distribution).await;
    assert_eq!(draws.len(), 200);
    for d in draws {
        assert!(d == 1 || d == 3);
    }
}

#[semio_framework_async_macros::async_test]
async fn cumulative_distribution_ends_at_one_and_is_nondecreasing() {
    let cdf = cumulative_distribution(&[1.0, 2.0, 3.0, 4.0]).await;
    assert!((cdf.last().unwrap() - 1.0).abs() < 1e-12);
    for pair in cdf.windows(2) {
        assert!(pair[1] >= pair[0]);
    }
}

#[semio_framework_async_macros::async_test]
async fn cumulative_distribution_of_empty_weights_is_empty() {
    let cdf = cumulative_distribution(&[]).await;
    assert!(cdf.is_empty());
}
// #endregion 🔖️DistributionTests
