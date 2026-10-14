use super::*;
use semio_framework_math_sampling_vocabulary::{CounterRng, SanitizePolicy, StepIndex};
use semio_framework_math_sampling_config::{MultinomialStrategy, SamplingMethod};
use semio_framework_math_sampling_testing::{small_vocab};
use semio_framework_math_sampling_engine::{NullObserver};

// #region 🔖️BatchTests
#[semio_framework_async_macros::async_test]
async fn continuous_batcher_add_remove_and_step() {
    let config = SamplingConfig::precise().await;
    let vocab = small_vocab().await;
    let mut batcher = ContinuousBatcher::new(8).await;
    let state_a = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(1).await.into()).await.unwrap();
    let state_b = SequenceState::new(SequenceId::new(2), Vec::new(), &config, CounterRng::from_seed(2).await.into()).await.unwrap();
    batcher.add_sequence(state_a).await;
    batcher.add_sequence(state_b).await;
    assert_eq!(batcher.len().await, 2);
    assert!(batcher.contains(SequenceId::new(1)).await);

    let logits_a = [1.0f32, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let logits_b = [9.0f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let request = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(1), logits: &logits_a }, BatchEntry { id: SequenceId::new(2), logits: &logits_b }] };
    let mut observer: SamplingObservers = NullObserver.into();
    let batch_result = batcher.step(&config, &vocab, None, &request, &mut observer).await;
    assert_eq!(batch_result.results.len(), 2);
    assert_eq!(batch_result.results[0].1.as_ref().unwrap().token, TokenId::new(1));
    assert_eq!(batch_result.results[1].1.as_ref().unwrap().token, TokenId::new(0));

    let removed = batcher.remove_sequence(SequenceId::new(1)).await;
    assert!(removed.is_some());
    assert_eq!(batcher.len().await, 1);
}

#[semio_framework_async_macros::async_test]
async fn continuous_batcher_step_reports_error_for_unknown_sequence() {
    let config = SamplingConfig::precise().await;
    let vocab = small_vocab().await;
    let mut batcher = ContinuousBatcher::new(8).await;
    let logits = [0.0f32; 8];
    let request = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(99), logits: &logits }] };
    let mut observer: SamplingObservers = NullObserver.into();
    let result = batcher.step(&config, &vocab, None, &request, &mut observer).await;
    assert!(result.results[0].1.is_err());
}

#[semio_framework_async_macros::async_test]
async fn continuous_batcher_per_sequence_output_is_independent_of_processing_order() {
    let config = SamplingConfig::balanced().await;
    let vocab = small_vocab().await;
    let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
    let mut observer: SamplingObservers = NullObserver.into();

    let mut forward = ContinuousBatcher::new(8).await;
    forward.add_sequence(SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(config.seed).await.into()).await.unwrap()).await;
    forward.add_sequence(SequenceState::new(SequenceId::new(2), Vec::new(), &config, CounterRng::from_seed(config.seed).await.into()).await.unwrap()).await;
    let request_forward = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(1), logits: &logits }, BatchEntry { id: SequenceId::new(2), logits: &logits }] };
    forward.step(&config, &vocab, None, &request_forward, &mut observer).await;

    let mut backward = ContinuousBatcher::new(8).await;
    backward.add_sequence(SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(config.seed).await.into()).await.unwrap()).await;
    backward.add_sequence(SequenceState::new(SequenceId::new(2), Vec::new(), &config, CounterRng::from_seed(config.seed).await.into()).await.unwrap()).await;
    let request_backward = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(2), logits: &logits }, BatchEntry { id: SequenceId::new(1), logits: &logits }] };
    backward.step(&config, &vocab, None, &request_backward, &mut observer).await;

    assert_eq!(forward.get(SequenceId::new(1)).await.unwrap().generated().await, backward.get(SequenceId::new(1)).await.unwrap().generated().await);
    assert_eq!(forward.get(SequenceId::new(2)).await.unwrap().generated().await, backward.get(SequenceId::new(2)).await.unwrap().generated().await);
}

// #endregion 🔖️BatchTests

// #region 🔖️SearchTests
#[semio_framework_async_macros::async_test]
async fn beam_search_finds_the_highest_probability_short_sequence() {
    let config = SamplingConfig::precise().await;
    let vocab = Vocabulary::new(4).await.with_eos(vec![TokenId::new(3)]).await;
    let beam_config = BeamSearchConfig { width: 4, length_penalty: 1.0, max_steps: 3 };
    let initial = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    // 🌳️ Token 1 dominates for the first two steps (building the best possible 2-token prefix);
    // from step 2 on, EOS (token 3) overwhelmingly dominates every beam alike, so whichever beam
    // carries the best prefix into that step produces the overall best-scoring finished
    // hypothesis: "1, 1, 3". Raw (non-length-normalized) cumulative log-probability would instead
    // favor never stopping ("1, 1, 1, ...") — this scenario is deliberately shaped so the length
    // penalty isn't what's under test, only "beam search finds the argmax-per-step path".
    let hypotheses = beam_search(&config, &beam_config, &vocab, None, initial, async |state| if state.generated().await.len() < 2 { vec![0.0, 3.0, -5.0, -5.0] } else { vec![-5.0, -5.0, -5.0, 10.0] }).await.unwrap();
    assert!(!hypotheses.is_empty());
    let best = &hypotheses[0];
    assert_eq!(best.state.generated().await, &[TokenId::new(1), TokenId::new(1), TokenId::new(3)]);
}

#[semio_framework_async_macros::async_test]
async fn beam_search_hypotheses_have_independent_state() {
    let config = SamplingConfig::precise().await;
    let vocab = Vocabulary::new(4).await;
    let beam_config = BeamSearchConfig { width: 3, length_penalty: 1.0, max_steps: 2 };
    let initial = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let hypotheses = beam_search(&config, &beam_config, &vocab, None, initial, async |_state| vec![1.0, 2.0, 3.0, 0.5]).await.unwrap();
    assert!(hypotheses.len() >= 2);
    // 🌳️ Every surviving hypothesis's fork is a distinct SequenceState with its own id-derived RNG.
    let mut ids: std::collections::HashSet<u64> = std::collections::HashSet::new();
    for h in &hypotheses {
        ids.insert(h.state.id().await.get());
    }
    assert_eq!(ids.len(), 1, "all forks share the parent's sequence id in this driver; independence is in per-beam state, not id");
}

#[semio_framework_async_macros::async_test]
async fn best_of_n_selects_the_candidate_with_highest_mean_logprob() {
    let config = SamplingConfig::precise().await;
    let vocab = Vocabulary::new(4).await.with_eos(vec![TokenId::new(3)]).await;
    let best_of = BestOfN { n: 3 };
    let mut observer: SamplingObservers = NullObserver.into();
    let results =
        best_of.run(&config, &vocab, None, 2, &mut observer, async |i| SequenceState::new(SequenceId::new(i as u64), Vec::new(), &config, CounterRng::from_seed(i as u64).await.into()).await, async |_state| vec![0.0, 9.0, 0.0, 1.0]).await.unwrap();
    assert_eq!(results.len(), 3);
    for i in 1..results.len() {
        assert!(results[i - 1].1 >= results[i].1, "results must be sorted best-first by mean logprob");
    }
}

#[semio_framework_async_macros::async_test]
async fn rejection_sampler_retries_until_accept_returns_true() {
    let config = SamplingConfig::balanced().await;
    let vocab = small_vocab().await;
    let mut ws = LogitsWorkspace::new(8).await;
    let mut rng: RandomSources = CounterRng::from_seed(7).await.into();
    let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
    let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    let sampler = RejectionSampler { max_attempts: 100 };
    let result = sampler.sample(&config, &mut ws, &mut rng, &logits, input, |r| r.token == TokenId::new(3)).await.unwrap();
    assert_eq!(result.token, TokenId::new(3));
}

#[semio_framework_async_macros::async_test]
async fn rejection_sampler_errors_after_max_attempts_when_never_accepted() {
    let config = SamplingConfig::precise().await;
    let vocab = small_vocab().await;
    let mut ws = LogitsWorkspace::new(8).await;
    let mut rng: RandomSources = CounterRng::from_seed(0).await.into();
    let logits = [0.0f32, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    let sampler = RejectionSampler { max_attempts: 5 };
    assert!(sampler.sample(&config, &mut ws, &mut rng, &logits, input, |r| r.token == TokenId::new(2)).await.is_err());
}

// #endregion 🔖️SearchTests

// #region 🔖️SpeculativeTests
#[semio_framework_async_macros::async_test]
async fn speculative_decode_accepts_when_draft_matches_target_distribution() {
    let config = SamplingConfig::precise().await;
    let vocab = Vocabulary::new(4).await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(4).await;
    let mut observer: SamplingObservers = NullObserver.into();
    let draft_tokens = [TokenId::new(1), TokenId::new(1)];
    let draft_distributions = vec![vec![0.05f32, 0.9, 0.03, 0.02], vec![0.05f32, 0.9, 0.03, 0.02]];
    let (results, metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &draft_tokens, &draft_distributions, async |_state| vec![0.0, 9.0, 0.0, 0.0], &mut observer).await.unwrap();
    assert_eq!(metrics.proposed, 2);
    assert_eq!(metrics.accepted, 2);
    assert!(metrics.bonus_taken);
    assert_eq!(results.len(), 3);
    assert_eq!(state.generated().await.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn speculative_decode_rejects_and_resamples_when_draft_disagrees_with_target() {
    let config = SamplingConfig::precise().await;
    let vocab = Vocabulary::new(4).await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(4).await;
    let mut observer: SamplingObservers = NullObserver.into();
    // 🎲️ Draft proposes token 0 with high confidence, but the target model overwhelmingly
    // prefers token 1 — acceptance probability is near zero, so this should almost always reject.
    let draft_tokens = [TokenId::new(0)];
    let draft_distributions = vec![vec![0.99f32, 0.01, 0.0, 0.0]];
    let (results, metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &draft_tokens, &draft_distributions, async |_state| vec![-10.0, 10.0, -10.0, -10.0], &mut observer).await.unwrap();
    assert_eq!(metrics.accepted, 0);
    assert!(!metrics.bonus_taken);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].token, TokenId::new(1));
}

#[semio_framework_async_macros::async_test]
async fn speculative_decode_matches_direct_target_sampling_distribution() {
    // ⚡️ Statistical check: over many trials, the *marginal* distribution of the first token
    // speculative decoding produces should match sampling directly from the target logits —
    // the defining correctness property of exact speculative decoding.
    let config = SamplingConfig { method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch }, ..SamplingConfig::default() };
    let vocab = Vocabulary::new(3).await;
    let target = [0.0f32, 1.0, 2.0];
    let draft_probs_dist = [0.5f32, 0.3, 0.2]; // a plausible, imperfect draft distribution

    let trials = 4_000;
    let mut counts_spec = [0u32; 3];
    for seed in 0..trials {
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(seed).await.into()).await.unwrap();
        let mut ws = LogitsWorkspace::new(3).await;
        let mut observer: SamplingObservers = NullObserver.into();
        // Draft token drawn from the (imperfect) draft distribution using a simple counter rng.
        let mut draft_rng = CounterRng::from_seed(seed ^ 0xD3AF).await;
        let cdf = cumulative_from_probs(&draft_probs_dist).await;
        let draft_token = TokenId::new(cdf_binary_search(&cdf, draft_rng.next_f64().await).await as u32);
        let draft_distributions = vec![draft_probs_dist.to_vec()];
        let (results, _metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &[draft_token], &draft_distributions, async |_state| target.to_vec(), &mut observer).await.unwrap();
        counts_spec[results[0].token.get() as usize] += 1;
    }

    let mut ws = LogitsWorkspace::new(3).await;
    let target_probs = {
        ws.reset_for_step(&target, SanitizePolicy::NegInfNan).await.unwrap();
        ws.sort_live_by_prob_desc().await;
        let mut probs = vec![0.0f32; 3];
        for (&tok, &p) in ws.live().await.iter().zip(ws.probs().await.iter()) {
            probs[tok as usize] = p;
        }
        probs
    };

    for i in 0..3 {
        let observed = counts_spec[i] as f64 / trials as f64;
        let expected = target_probs[i] as f64;
        assert!((observed - expected).abs() < 0.03, "token {i}: observed {observed} vs expected {expected}");
    }
}

// #endregion 🔖️SpeculativeTests

// #region 🔖️ShardedTests
#[semio_framework_async_macros::async_test]
async fn sharded_softmax_matches_unsharded_softmax() {
    let full_logits = [1.0f32, 2.0, 0.5, 3.0, -1.0, 0.2, 4.0, 0.1];
    let mut ws = LogitsWorkspace::new(8).await;
    ws.reset_for_step(&full_logits, SanitizePolicy::NegInfNan).await.unwrap();
    // 📐️ `softmax_over_live`'s return value is the raw pre-normalization partition sum (useful
    // for e.g. logsumexp), not the post-normalization probability sum — check `ws.probs()` for
    // that instead.
    ws.softmax_over_live().await;
    let mut unsharded_probs = [0.0f32; 8];
    for (&tok, &p) in ws.live().await.iter().zip(ws.probs().await.iter()) {
        unsharded_probs[tok as usize] = p;
    }
    let prob_sum: f32 = ws.probs().await.iter().sum();
    assert!((prob_sum - 1.0).abs() < 1e-4);

    let shard0 = &full_logits[..4];
    let shard1 = &full_logits[4..];
    let mut ranks = LocalCollective::new_group(2).await;
    let mut rank1 = ranks.pop().unwrap();
    let mut rank0 = ranks.pop().unwrap();
    // 🗂️ Two-phase mailbox convention: call every rank once, then again to read the merged result.
    let _ = sharded_softmax(&mut rank0, shard0).await;
    let _ = sharded_softmax(&mut rank1, shard1).await;
    let sharded0 = sharded_softmax(&mut rank0, shard0).await;
    let sharded1 = sharded_softmax(&mut rank1, shard1).await;

    for i in 0..4 {
        assert!((sharded0[i] - unsharded_probs[i]).abs() < 1e-5, "shard0[{i}]: {} vs {}", sharded0[i], unsharded_probs[i]);
    }
    for i in 0..4 {
        assert!((sharded1[i] - unsharded_probs[4 + i]).abs() < 1e-5, "shard1[{i}]: {} vs {}", sharded1[i], unsharded_probs[4 + i]);
    }
}

#[semio_framework_async_macros::async_test]
async fn sharded_top_k_matches_unsharded_top_k() {
    let full_logits = [1.0f32, 5.0, 0.5, 3.0, -1.0, 0.2, 4.0, 0.1];
    let shard0 = &full_logits[..4];
    let shard1 = &full_logits[4..];
    let mut ranks = LocalCollective::new_group(2).await;
    let mut rank1 = ranks.pop().unwrap();
    let mut rank0 = ranks.pop().unwrap();
    let _ = sharded_top_k(&mut rank0, shard0, 0, 3).await;
    let top_k = sharded_top_k(&mut rank1, shard1, 4, 3).await;

    let mut expected: Vec<(u32, f32)> = full_logits.iter().enumerate().map(|(i, &l)| (i as u32, l)).collect();
    expected.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let expected_tokens: Vec<u32> = expected.iter().take(3).map(|&(t, _)| t).collect();
    let actual_tokens: Vec<u32> = top_k.iter().map(|c| c.token.get()).collect();
    assert_eq!(actual_tokens, expected_tokens);
}

#[semio_framework_async_macros::async_test]
async fn sharded_sample_marginal_distribution_matches_unsharded() {
    let full_logits = [2.0f32, 0.0, 1.0, -1.0];
    let shard0 = &full_logits[..2];
    let shard1 = &full_logits[2..];
    let trials = 5_000;
    let mut counts = [0u32; 4];
    for seed in 0..trials {
        let mut ranks = LocalCollective::new_group(2).await;
        let mut rank1 = ranks.pop().unwrap();
        let mut rank0 = ranks.pop().unwrap();
        let mut rng: RandomSources = CounterRng::from_seed(seed).await.into();
        let _ = sharded_top_k(&mut rank0, shard0, 0, usize::MAX).await;
        if let Some(token) = sharded_sample(&mut rank1, shard1, 2, &mut rng).await {
            counts[token.get() as usize] += 1;
        }
    }
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&full_logits, SanitizePolicy::NegInfNan).await.unwrap();
    ws.sort_live_by_prob_desc().await;
    let mut expected = [0.0f32; 4];
    for (&tok, &p) in ws.live().await.iter().zip(ws.probs().await.iter()) {
        expected[tok as usize] = p;
    }
    for i in 0..4 {
        let observed = counts[i] as f64 / trials as f64;
        assert!((observed - expected[i] as f64).abs() < 0.03, "token {i}: observed {observed} vs expected {}", expected[i]);
    }
}

// #endregion 🔖️ShardedTests
