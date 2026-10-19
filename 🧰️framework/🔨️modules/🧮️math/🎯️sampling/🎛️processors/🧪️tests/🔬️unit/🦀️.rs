use super::*;
use semio_framework_math_sampling_vocabulary::{CounterRng, SanitizePolicy, StepIndex, Vocabulary, cumulative_from_probs};
use semio_framework_math_sampling_testing::{small_vocab, step_view};

// #region 🔖️WarperTests
#[semio_framework_async_macros::async_test]
async fn temperature_zero_collapses_to_greedy() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 5.0, 2.0, 0.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = small_vocab().await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut temp = Temperature { value: Schedule::Constant(0.0) };
    temp.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.live().await, &[1]);
}

#[semio_framework_async_macros::async_test]
async fn temperature_scales_processed_logits() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[2.0, 4.0, 6.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut temp = Temperature { value: Schedule::Constant(2.0) };
    temp.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.processed().await, &[1.0, 2.0, 3.0]);
}

#[semio_framework_async_macros::async_test]
async fn top_k_keeps_exactly_k_highest_probability_tokens() {
    let mut ws = LogitsWorkspace::new(5).await;
    ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(5).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut top_k = TopK { k: Schedule::Constant(2.0), min_keep: 1 };
    top_k.process(&view, &mut ws).await.unwrap();
    let mut kept = ws.live().await.to_vec();
    kept.sort_unstable();
    assert_eq!(kept, vec![0, 1]);
}

#[semio_framework_async_macros::async_test]
async fn top_p_retains_smallest_prefix_covering_cumulative_mass() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut top_p = TopP { p: Schedule::Constant(0.5), min_keep: 1 };
    top_p.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.live().await, &[0]);
}

#[semio_framework_async_macros::async_test]
async fn top_p_min_keep_overrides_a_too_small_cutoff() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut top_p = TopP { p: Schedule::Constant(0.01), min_keep: 3 };
    top_p.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.live().await.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn min_p_drops_tokens_far_below_the_maximum() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[10.0, 0.0, -10.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut min_p = MinP { p: Schedule::Constant(0.1), min_keep: 1 };
    min_p.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.live().await, &[0]);
}

#[semio_framework_async_macros::async_test]
async fn typical_and_locally_typical_agree_on_the_same_input() {
    let logits = [3.0f32, 1.0, 0.5, 0.1];
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;

    let mut ws_a = LogitsWorkspace::new(4).await;
    ws_a.reset_for_step(&logits, SanitizePolicy::NegInfNan).await.unwrap();
    let mut typical = Typical { mass: Schedule::Constant(0.8), min_keep: 1 };
    typical.process(&view, &mut ws_a).await.unwrap();

    let mut ws_b = LogitsWorkspace::new(4).await;
    ws_b.reset_for_step(&logits, SanitizePolicy::NegInfNan).await.unwrap();
    let mut locally = LocallyTypical { mass: Schedule::Constant(0.8), min_keep: 1 };
    locally.process(&view, &mut ws_b).await.unwrap();

    let mut a = ws_a.live().await.to_vec();
    let mut b = ws_b.live().await.to_vec();
    a.sort_unstable();
    b.sort_unstable();
    assert_eq!(a, b);
    assert!(!a.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn tail_free_keeps_at_least_min_keep_on_short_live_sets() {
    let mut ws = LogitsWorkspace::new(2).await;
    ws.reset_for_step(&[1.0, 2.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(2).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut tail_free = TailFree { z: Schedule::Constant(0.9), min_keep: 1 };
    tail_free.process(&view, &mut ws).await.unwrap();
    assert!(!ws.live().await.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn epsilon_cutoff_drops_near_zero_probability_tokens() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[20.0, -20.0, -20.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut epsilon = EpsilonCutoff { cutoff: Schedule::Constant(0.01), min_keep: 1 };
    epsilon.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.live().await, &[0]);
}

#[semio_framework_async_macros::async_test]
async fn eta_cutoff_never_empties_live_set() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 2.0, 3.0, 4.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut eta = EtaCutoff { cutoff: Schedule::Constant(0.1), min_keep: 1 };
    eta.process(&view, &mut ws).await.unwrap();
    assert!(!ws.live().await.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn top_a_drops_low_probability_tokens_relative_to_max() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[10.0, -10.0, -10.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut top_a = TopA { power: Schedule::Constant(0.5), min_keep: 1 };
    top_a.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.live().await, &[0]);
}

#[semio_framework_async_macros::async_test]
async fn rank_truncation_keeps_exactly_max_rank_entries() {
    let mut ws = LogitsWorkspace::new(5).await;
    ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(5).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut rank = RankTruncation { max_rank: 2 };
    rank.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.live().await.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn adaptive_truncation_targeting_effective_count_shrinks_a_near_uniform_distribution() {
    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&[1.0; 6], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(6).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut adaptive = AdaptiveTruncation { target_entropy: None, target_effective_count: Some(2.0) };
    adaptive.process(&view, &mut ws).await.unwrap();
    assert!(ws.live().await.len() < 6);
    assert!(ws.live().await.len() >= 2);
}

#[semio_framework_async_macros::async_test]
async fn min_keep_guarantee_is_honored_by_every_truncation_warper() {
    let vocab = Vocabulary::new(6).await;
    let view = step_view(&vocab, &[], &[]).await;
    let logits = [1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0];
    let min_keep = 4;

    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).await.unwrap();
    TopK { k: Schedule::Constant(1.0), min_keep }.process(&view, &mut ws).await.unwrap();
    assert!(ws.live().await.len() >= min_keep);

    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).await.unwrap();
    TopP { p: Schedule::Constant(0.001), min_keep }.process(&view, &mut ws).await.unwrap();
    assert!(ws.live().await.len() >= min_keep);

    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).await.unwrap();
    MinP { p: Schedule::Constant(0.999), min_keep }.process(&view, &mut ws).await.unwrap();
    assert!(ws.live().await.len() >= min_keep);
}

// #endregion 🔖️WarperTests

// #region 🔖️SelectionTests
async fn distribution_fixture<'a>(tokens: &'a [TokenId], probs: &'a [f32], logprobs: &'a [f32], cdf: &'a [f64]) -> Distribution<'a> {
    Distribution { tokens, probs, logprobs, cdf, entropy: entropy_nats(probs).await }
}

#[semio_framework_async_macros::async_test]
async fn greedy_sampler_lowest_token_id_picks_first_of_a_tie() {
    let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
    let probs = [0.5f32, 0.5, 0.0];
    let logprobs = [probs[0].ln(), probs[1].ln(), f32::NEG_INFINITY];
    let cdf = [0.5, 1.0, 1.0];
    let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf).await;
    let mut sampler = GreedySampler { tie_break: TieBreak::LowestTokenId };
    let mut out = SelectionBuffer::default();
    let mut rng: RandomSources = CounterRng::from_seed(1).await.into();
    sampler.sample(&step_view(&small_vocab().await, &[], &[]).await, &dist, &mut rng, &mut out).await.unwrap();
    assert_eq!(out.chosen[0].token, TokenId::new(0));
}

#[semio_framework_async_macros::async_test]
async fn greedy_sampler_highest_token_id_picks_last_of_a_tie() {
    let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
    let probs = [0.5f32, 0.5, 0.0];
    let logprobs = [probs[0].ln(), probs[1].ln(), f32::NEG_INFINITY];
    let cdf = [0.5, 1.0, 1.0];
    let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf).await;
    let mut sampler = GreedySampler { tie_break: TieBreak::HighestTokenId };
    let mut out = SelectionBuffer::default();
    let mut rng: RandomSources = CounterRng::from_seed(1).await.into();
    sampler.sample(&step_view(&small_vocab().await, &[], &[]).await, &dist, &mut rng, &mut out).await.unwrap();
    assert_eq!(out.chosen[0].token, TokenId::new(1));
}

#[semio_framework_async_macros::async_test]
async fn greedy_sampler_errors_on_empty_distribution() {
    let dist = distribution_fixture(&[], &[], &[], &[]).await;
    let mut sampler = GreedySampler { tie_break: TieBreak::LowestTokenId };
    let mut out = SelectionBuffer::default();
    let mut rng: RandomSources = CounterRng::from_seed(1).await.into();
    assert!(sampler.sample(&step_view(&small_vocab().await, &[], &[]).await, &dist, &mut rng, &mut out).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn multinomial_cdf_binary_search_matches_expected_frequencies() {
    let tokens = [TokenId::new(0), TokenId::new(1)];
    let probs = [0.2f32, 0.8];
    let logprobs = [probs[0].ln(), probs[1].ln()];
    let cdf = [0.2, 1.0];
    let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf).await;
    let mut sampler = MultinomialSampler { strategy: MultinomialStrategy::CdfBinarySearch };
    let mut rng: RandomSources = CounterRng::from_seed(2024).await.into();
    let vocab = small_vocab().await;
    let view = step_view(&vocab, &[], &[]).await;
    let draws = 20_000;
    let mut count_1 = 0u32;
    for _ in 0..draws {
        let mut out = SelectionBuffer::default();
        sampler.sample(&view, &dist, &mut rng, &mut out).await.unwrap();
        if out.chosen[0].token == TokenId::new(1) {
            count_1 += 1;
        }
    }
    let ratio = count_1 as f64 / draws as f64;
    assert!((ratio - 0.8).abs() < 0.02, "ratio {ratio} too far from 0.8");
}

#[semio_framework_async_macros::async_test]
async fn multinomial_strategies_agree_statistically() {
    let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
    let probs = [0.1f32, 0.3, 0.6];
    let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln()];
    let cdf = cumulative_from_probs(&probs).await;
    let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf).await;
    let vocab = small_vocab().await;
    let view = step_view(&vocab, &[], &[]).await;
    let draws = 20_000;

    for strategy in [MultinomialStrategy::CdfBinarySearch, MultinomialStrategy::LinearScan, MultinomialStrategy::Alias] {
        let mut sampler = MultinomialSampler { strategy };
        let mut rng: RandomSources = CounterRng::from_seed(555).await.into();
        let mut counts = [0u32; 3];
        for _ in 0..draws {
            let mut out = SelectionBuffer::default();
            sampler.sample(&view, &dist, &mut rng, &mut out).await.unwrap();
            counts[out.chosen[0].token.get() as usize] += 1;
        }
        for (i, &expected) in probs.iter().enumerate() {
            let ratio = counts[i] as f64 / draws as f64;
            assert!((ratio - expected as f64).abs() < 0.03, "strategy {strategy:?} index {i}: ratio {ratio} vs expected {expected}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn gumbel_max_sampler_matches_multinomial_marginals() {
    let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
    let probs = [0.2f32, 0.3, 0.5];
    let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln()];
    let cdf = cumulative_from_probs(&probs).await;
    let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf).await;
    let vocab = small_vocab().await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut sampler = GumbelMaxSampler;
    let mut rng: RandomSources = CounterRng::from_seed(321).await.into();
    let draws = 20_000;
    let mut counts = [0u32; 3];
    for _ in 0..draws {
        let mut out = SelectionBuffer::default();
        sampler.sample(&view, &dist, &mut rng, &mut out).await.unwrap();
        counts[out.chosen[0].token.get() as usize] += 1;
    }
    for (i, &expected) in probs.iter().enumerate() {
        let ratio = counts[i] as f64 / draws as f64;
        assert!((ratio - expected as f64).abs() < 0.03, "index {i}: ratio {ratio} vs expected {expected}");
    }
}

#[semio_framework_async_macros::async_test]
async fn gumbel_top_k_returns_k_distinct_tokens() {
    let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2), TokenId::new(3)];
    let probs = [0.4f32, 0.3, 0.2, 0.1];
    let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln(), probs[3].ln()];
    let cdf = cumulative_from_probs(&probs).await;
    let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf).await;
    let vocab = small_vocab().await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut sampler = GumbelTopKSampler { k: 2 };
    let mut rng: RandomSources = CounterRng::from_seed(7).await.into();
    let mut out = SelectionBuffer::default();
    sampler.sample(&view, &dist, &mut rng, &mut out).await.unwrap();
    assert_eq!(out.chosen.len(), 2);
    assert_ne!(out.chosen[0].token, out.chosen[1].token);
}

// #endregion 🔖️SelectionTests

// #region 🔖️PenaltiesTests
#[semio_framework_async_macros::async_test]
async fn repetition_penalty_pushes_down_a_seen_positive_logit() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[4.0, 2.0, 2.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let generated = [TokenId::new(0)];
    let view = step_view(&vocab, &[], &generated).await;
    let mut penalty = RepetitionPenalty::new(2.0, PenaltyScope::GeneratedOnly).await;
    penalty.commit(&view, TokenId::new(0)).await;
    penalty.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.processed().await[0], 2.0);
    assert_eq!(ws.processed().await[1], 2.0);
}

#[semio_framework_async_macros::async_test]
async fn repetition_penalty_rollback_restores_exact_prior_state() {
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut penalty = RepetitionPenalty::new(2.0, PenaltyScope::GeneratedOnly).await;
    let mark_before = penalty.save().await;
    penalty.commit(&view, TokenId::new(0)).await;
    penalty.commit(&view, TokenId::new(1)).await;
    assert_eq!(penalty.counts.count(TokenId::new(0)).await, 1);
    penalty.rollback_to(mark_before).await;
    assert_eq!(penalty.counts.count(TokenId::new(0)).await, 0);
    assert_eq!(penalty.counts.count(TokenId::new(1)).await, 0);
}

#[semio_framework_async_macros::async_test]
async fn presence_penalty_applies_flat_penalty_regardless_of_count() {
    let mut ws = LogitsWorkspace::new(2).await;
    ws.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(2).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut penalty = PresencePenalty::new(0.5, PenaltyScope::GeneratedOnly).await;
    penalty.commit(&view, TokenId::new(0)).await;
    penalty.commit(&view, TokenId::new(0)).await;
    penalty.process(&view, &mut ws).await.unwrap();
    assert!((ws.processed().await[0] - 0.5).abs() < 1e-6);
    assert!((ws.processed().await[1] - 1.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn frequency_penalty_scales_with_occurrence_count() {
    let mut ws = LogitsWorkspace::new(2).await;
    ws.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(2).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut penalty = FrequencyPenalty::new(0.5, PenaltyScope::GeneratedOnly).await;
    penalty.commit(&view, TokenId::new(0)).await;
    penalty.commit(&view, TokenId::new(0)).await;
    penalty.process(&view, &mut ws).await.unwrap();
    assert!((ws.processed().await[0] - 0.0).abs() < 1e-6);
    assert!((ws.processed().await[1] - 1.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn decaying_penalty_weighs_recent_occurrences_more_than_distant_ones() {
    let mut ws_recent = LogitsWorkspace::new(2).await;
    ws_recent.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(2).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut recent_penalty = DecayingPenalty::new(1.0, 8, 1.0, PenaltyScope::GeneratedOnly).await;
    recent_penalty.commit(&view, TokenId::new(0)).await;
    recent_penalty.process(&view, &mut ws_recent).await.unwrap();

    let mut ws_distant = LogitsWorkspace::new(2).await;
    ws_distant.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let mut distant_penalty = DecayingPenalty::new(1.0, 8, 1.0, PenaltyScope::GeneratedOnly).await;
    distant_penalty.commit(&view, TokenId::new(0)).await;
    for _ in 0..5 {
        distant_penalty.commit(&view, TokenId::new(1)).await;
    }
    distant_penalty.process(&view, &mut ws_distant).await.unwrap();

    assert!(ws_recent.processed().await[0] < ws_distant.processed().await[0], "a more recent occurrence must be penalized harder");
}

#[semio_framework_async_macros::async_test]
async fn token_class_penalty_scales_only_classified_tokens() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[10.0, 10.0, 10.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut penalty = TokenClassPenalty::new(vec![vec![TokenId::new(0), TokenId::new(1)]], vec![0.5]).await;
    penalty.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.processed().await[0], 5.0);
    assert_eq!(ws.processed().await[1], 5.0);
    assert_eq!(ws.processed().await[2], 10.0);
}

#[semio_framework_async_macros::async_test]
async fn no_repeat_ngram_forbids_recreating_a_seen_bigram() {
    let mut ngram = NoRepeatNgram::new(2).await;
    let vocab = Vocabulary::new(5).await;
    let generated_first = [TokenId::new(0)];
    let view_after_first = step_view(&vocab, &[], &generated_first).await;
    ngram.commit(&view_after_first, TokenId::new(1)).await;
    let mut ws = LogitsWorkspace::new(5).await;
    ws.reset_for_step(&[0.0; 5], SanitizePolicy::NegInfNan).await.unwrap();
    let generated_third = [TokenId::new(0), TokenId::new(1), TokenId::new(0)];
    let view_after_third = step_view(&vocab, &[], &generated_third).await;
    ngram.process(&view_after_third, &mut ws).await.unwrap();
    assert!(!ws.mask().await.get(TokenId::new(1)).await);
    assert!(ws.mask().await.get(TokenId::new(2)).await);
}

#[semio_framework_async_macros::async_test]
async fn no_repeat_ngram_rollback_un_forbids() {
    let mut ngram = NoRepeatNgram::new(2).await;
    let vocab = Vocabulary::new(5).await;
    let generated = [TokenId::new(0)];
    let view = step_view(&vocab, &[], &generated).await;
    let mark = ngram.save().await;
    ngram.commit(&view, TokenId::new(1)).await;
    ngram.rollback_to(mark).await;
    let mut ws = LogitsWorkspace::new(5).await;
    ws.reset_for_step(&[0.0; 5], SanitizePolicy::NegInfNan).await.unwrap();
    ngram.process(&view, &mut ws).await.unwrap();
    assert!(ws.mask().await.get(TokenId::new(1)).await, "rollback must undo the forbidden-next entry");
}

#[semio_framework_async_macros::async_test]
async fn phrase_penalty_penalizes_only_after_the_proper_prefix() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let generated = [TokenId::new(0)];
    let view = step_view(&vocab, &[], &generated).await;
    let mut penalty = PhrasePenalty { phrases: vec![vec![TokenId::new(0), TokenId::new(1)]], penalty: 0.5 };
    penalty.process(&view, &mut ws).await.unwrap();
    assert!((ws.processed().await[1] - 0.5).abs() < 1e-6);
    assert_eq!(ws.processed().await[2], 1.0);
}

// #endregion 🔖️PenaltiesTests

// #region 🔖️BiasesTests
#[semio_framework_async_macros::async_test]
async fn logit_bias_sparse_and_dense_add_expected_deltas() {
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;

    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let mut sparse = LogitBiasSparse { entries: vec![(TokenId::new(1), 5.0)] };
    sparse.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.processed().await, &[1.0, 6.0, 1.0]);

    let mut ws2 = LogitsWorkspace::new(3).await;
    ws2.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let mut dense = LogitBiasDense { values: vec![0.0, -2.0, 3.0] };
    dense.process(&view, &mut ws2).await.unwrap();
    assert_eq!(ws2.processed().await, &[1.0, -1.0, 4.0]);
}

#[semio_framework_async_macros::async_test]
async fn allow_tokens_restricts_mask_to_exactly_the_allowed_set() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut allow = AllowTokens { tokens: vec![TokenId::new(1), TokenId::new(3)] };
    allow.process(&view, &mut ws).await.unwrap();
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[1, 3]);
}

#[semio_framework_async_macros::async_test]
async fn forbid_tokens_removes_exactly_the_forbidden_set() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut forbid = ForbidTokens { tokens: vec![TokenId::new(2)] };
    forbid.process(&view, &mut ws).await.unwrap();
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[0, 1, 3]);
}

#[semio_framework_async_macros::async_test]
async fn suppress_special_removes_flagged_tokens() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await.with_special(&[TokenId::new(0), TokenId::new(3)]).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut suppress = SuppressSpecial;
    suppress.process(&view, &mut ws).await.unwrap();
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[1, 2]);
}

#[semio_framework_async_macros::async_test]
async fn bad_words_masks_the_completion_token_after_its_prefix() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let generated = [TokenId::new(0)];
    let view = step_view(&vocab, &[], &generated).await;
    let mut bad = BadWords { phrases: vec![vec![TokenId::new(0), TokenId::new(1)]] };
    bad.process(&view, &mut ws).await.unwrap();
    assert!(!ws.mask().await.get(TokenId::new(1)).await);
    assert!(ws.mask().await.get(TokenId::new(2)).await);
}

#[semio_framework_async_macros::async_test]
async fn sequence_encouragement_biases_the_completion_token_after_its_prefix() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let generated = [TokenId::new(0)];
    let view = step_view(&vocab, &[], &generated).await;
    let mut encourage = SequenceEncouragement { phrases: vec![vec![TokenId::new(0), TokenId::new(1)]], bonus: 3.0 };
    encourage.process(&view, &mut ws).await.unwrap();
    assert_eq!(ws.processed().await[1], 4.0);
    assert_eq!(ws.processed().await[2], 1.0);
}

// #endregion 🔖️BiasesTests

// #region 🔖️LengthTests
#[semio_framework_async_macros::async_test]
async fn min_length_eos_suppression_masks_eos_before_the_floor() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await.with_eos(vec![TokenId::new(2)]).await;
    let generated = [TokenId::new(0)];
    let view = step_view(&vocab, &[], &generated).await;
    let mut min_len = MinLengthEosSuppression { min_tokens: 5 };
    min_len.process(&view, &mut ws).await.unwrap();
    assert!(!ws.mask().await.get(TokenId::new(2)).await);
}

#[semio_framework_async_macros::async_test]
async fn min_length_eos_suppression_allows_eos_once_floor_reached() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await.with_eos(vec![TokenId::new(2)]).await;
    let generated = vec![TokenId::new(0); 5];
    let view = step_view(&vocab, &[], &generated).await;
    let mut min_len = MinLengthEosSuppression { min_tokens: 5 };
    min_len.process(&view, &mut ws).await.unwrap();
    assert!(ws.mask().await.get(TokenId::new(2)).await);
}

#[semio_framework_async_macros::async_test]
async fn max_length_force_eos_restricts_to_eos_at_the_cap() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await.with_eos(vec![TokenId::new(2)]).await;
    let generated = vec![TokenId::new(0); 4];
    let view = step_view(&vocab, &[], &generated).await;
    let mut force = MaxLengthForceEos { max_tokens: 5 };
    force.process(&view, &mut ws).await.unwrap();
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[2]);
}

#[semio_framework_async_macros::async_test]
async fn forced_tokens_forces_bos_then_prefix_then_at_position() {
    let vocab = Vocabulary::new(6).await;
    let spec = ForcedSpec { bos: Some(TokenId::new(0)), prefix: vec![TokenId::new(1), TokenId::new(2)], at_position: vec![(StepIndex::new(5), TokenId::new(4))] };
    let mut forced = ForcedTokens { spec };

    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&[0.0; 6], SanitizePolicy::NegInfNan).await.unwrap();
    forced.process(&step_view(&vocab, &[], &[]).await, &mut ws).await.unwrap();
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[0]);

    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&[0.0; 6], SanitizePolicy::NegInfNan).await.unwrap();
    forced.process(&step_view(&vocab, &[], &[TokenId::new(0)]).await, &mut ws).await.unwrap();
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[1]);

    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&[0.0; 6], SanitizePolicy::NegInfNan).await.unwrap();
    let generated5 = vec![TokenId::new(0); 5];
    forced.process(&step_view(&vocab, &[], &generated5).await, &mut ws).await.unwrap();
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[4]);
}

// #endregion 🔖️LengthTests

// #region 🔖️AdaptiveTests
#[semio_framework_async_macros::async_test]
async fn mirostat_v2_truncates_to_tokens_within_the_surprise_budget() {
    let mut ws = LogitsWorkspace::new(6).await;
    ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(6).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut mirostat = Mirostat::new(MirostatVersion::V2, 3.0, 0.1).await;
    mirostat.process(&view, &mut ws).await.unwrap();
    assert!(ws.live().await.len() < 6);
}

#[semio_framework_async_macros::async_test]
async fn mirostat_commit_updates_mu_toward_target() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut mirostat = Mirostat::new(MirostatVersion::V2, 3.0, 0.5).await;
    mirostat.process(&view, &mut ws).await.unwrap();
    let mu_before = mirostat.mu;
    mirostat.commit(&view, TokenId::new(0)).await;
    assert_ne!(mirostat.mu, mu_before);
}

#[semio_framework_async_macros::async_test]
async fn mirostat_rollback_restores_mu() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut mirostat = Mirostat::new(MirostatVersion::V2, 3.0, 0.5).await;
    let mark = mirostat.save().await;
    mirostat.process(&view, &mut ws).await.unwrap();
    mirostat.commit(&view, TokenId::new(0)).await;
    mirostat.rollback_to(mark).await;
    assert_eq!(mirostat.mu, 6.0);
}

#[semio_framework_async_macros::async_test]
async fn entropy_pid_sharpens_the_distribution_when_entropy_exceeds_target() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(4).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut pid = EntropyPid::new(0.1, 1.0, 0.0, 0.0).await;
    pid.process(&view, &mut ws).await.unwrap();
    assert!(ws.processed().await[0] > 1.0);
}

#[semio_framework_async_macros::async_test]
async fn repetition_controller_flattens_after_crossing_the_threshold() {
    let vocab = Vocabulary::new(2).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut controller = RepetitionController::new(4, 0.5, 1.0).await;
    for _ in 0..4 {
        controller.commit(&view, TokenId::new(0)).await;
    }
    let mut ws = LogitsWorkspace::new(2).await;
    ws.reset_for_step(&[2.0, 2.0], SanitizePolicy::NegInfNan).await.unwrap();
    controller.process(&view, &mut ws).await.unwrap();
    assert!(ws.processed().await[0] < 2.0);
}

#[semio_framework_async_macros::async_test]
async fn confidence_controller_uses_low_temp_under_low_entropy() {
    let mut ws = LogitsWorkspace::new(3).await;
    ws.reset_for_step(&[10.0, 0.0, 0.0], SanitizePolicy::NegInfNan).await.unwrap();
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut controller = ConfidenceController { low_entropy: 0.0, high_entropy: 2.0, low_temp: 0.2, high_temp: 1.5 };
    controller.process(&view, &mut ws).await.unwrap();
    assert!(ws.processed().await[0] > 40.0);
}

// #endregion 🔖️AdaptiveTests
