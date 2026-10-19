use super::*;
use semio_framework_math_sampling_vocabulary::{CounterRng, SliceTextAdapter};
use semio_framework_math_sampling_config::{ConstraintSpec, ForcedSpec, ProcessorSpec, SamplingMethod, TieBreak};
use semio_framework_math_sampling_testing::{small_vocab, step_view};

// #region 🔖️EngineTests
#[semio_framework_async_macros::async_test]
async fn stateless_step_is_deterministic_for_same_seed_and_config() {
    let vocab = small_vocab().await;
    let config = SamplingConfig::balanced().await;
    let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];

    let run = async || {
        let mut ws = LogitsWorkspace::new(8).await;
        let mut rng: RandomSources = CounterRng::from_seed(config.seed).await.into();
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).await.unwrap()
    };
    let a = run().await;
    let b = run().await;
    assert_eq!(a.token, b.token);
    assert_eq!(a.logprob, b.logprob);
}

#[semio_framework_async_macros::async_test]
async fn stateless_step_greedy_precise_always_picks_the_argmax() {
    let vocab = small_vocab().await;
    let config = SamplingConfig::precise().await;
    let logits = [1.0f32, 2.0, 9.0, 3.0, 1.5, 0.2, 0.1, -5.0];
    let mut ws = LogitsWorkspace::new(8).await;
    let mut rng: RandomSources = CounterRng::from_seed(0).await.into();
    let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).await.unwrap();
    assert_eq!(result.token, TokenId::new(2));
}

#[semio_framework_async_macros::async_test]
async fn stateless_step_reports_eos_finish_reason() {
    let vocab = small_vocab().await;
    let config = SamplingConfig::precise().await;
    let mut logits = [0.0f32; 8];
    logits[7] = 100.0;
    let mut ws = LogitsWorkspace::new(8).await;
    let mut rng: RandomSources = CounterRng::from_seed(0).await.into();
    let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).await.unwrap();
    assert_eq!(result.token, TokenId::new(7));
    assert_eq!(result.finish, Some(FinishReason::EosToken));
}

#[semio_framework_async_macros::async_test]
async fn stateless_step_reports_max_tokens_finish_reason() {
    let vocab = small_vocab().await;
    let config = SamplingConfig { max_tokens: 1, ..SamplingConfig::precise().await };
    let logits = [1.0f32, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut ws = LogitsWorkspace::new(8).await;
    let mut rng: RandomSources = CounterRng::from_seed(0).await.into();
    let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).await.unwrap();
    assert_eq!(result.finish, Some(FinishReason::MaxTokens));
}

#[semio_framework_async_macros::async_test]
async fn stateless_step_rejects_mismatched_logits_length() {
    let vocab = small_vocab().await;
    let config = SamplingConfig::precise().await;
    let logits = [0.0f32; 4];
    let mut ws = LogitsWorkspace::new(8).await;
    let mut rng: RandomSources = CounterRng::from_seed(0).await.into();
    let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    assert!(sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn stateless_step_probabilities_sum_to_approximately_one() {
    let vocab = small_vocab().await;
    let config = SamplingConfig::balanced().await;
    let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
    let mut ws = LogitsWorkspace::new(8).await;
    let mut rng: RandomSources = CounterRng::from_seed(1).await.into();
    let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).await.unwrap();
    let sum: f32 = ws.probs().await.iter().sum();
    assert!((sum - 1.0).abs() < 1e-4, "sum = {sum}");
}

// #endregion 🔖️EngineTests

// #region 🔖️SequenceStateTests
async fn make_state(config: &SamplingConfig) -> SequenceState {
    SequenceState::new(SequenceId::new(1), Vec::new(), config, CounterRng::from_seed(config.seed).await.into()).await.unwrap_or_else(|e| panic!("sequence state should build: {e}"))
}

#[semio_framework_async_macros::async_test]
async fn sequence_state_new_builds_configured_constraints() {
    let config = SamplingConfig { constraints: vec![ConstraintSpec::JsonMode], ..SamplingConfig::default() };
    let state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.expect("json mode constraint should build");
    assert_eq!(state.constraints.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn sequence_state_new_rejects_an_invalid_regex_constraint() {
    let config = SamplingConfig { constraints: vec![ConstraintSpec::Regex("(unclosed".to_string())], ..SamplingConfig::default() };
    assert!(SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sequence_state_checkpoint_restore_round_trips() {
    let config = SamplingConfig::precise().await;
    let mut state = make_state(&config).await;
    let vocab = small_vocab().await;
    let view = step_view(&vocab, &[], &[]).await;
    state.generated.push(TokenId::new(3));
    state.cumulative_logprob = -1.5;
    let checkpoint = state.checkpoint().await;
    state.generated.push(TokenId::new(4));
    state.cumulative_logprob = -3.0;
    state.restore(&checkpoint).await;
    assert_eq!(state.generated().await, &[TokenId::new(3)]);
    assert!((state.cumulative_logprob().await - (-1.5)).abs() < 1e-9);
    let _ = view;
}

#[semio_framework_async_macros::async_test]
async fn sequence_state_rollback_then_readvance_matches_direct_run() {
    let config = SamplingConfig::balanced().await;
    let vocab = small_vocab().await;
    let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];

    let mut direct = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(config.seed).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(8).await;
    let mut observer: SamplingObservers = NullObserver.into();
    for _ in 0..3 {
        sample_step(&config, &mut direct, &mut ws, &vocab, None, &logits, &mut observer).await.unwrap();
    }
    let direct_text = direct.to_text().await;

    let mut replayed = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(config.seed).await.into()).await.unwrap();
    let mut ws2 = LogitsWorkspace::new(8).await;
    for _ in 0..3 {
        sample_step(&config, &mut replayed, &mut ws2, &vocab, None, &logits, &mut observer).await.unwrap();
    }
    replayed.rollback(2).await;
    assert_eq!(replayed.generated().await.len(), 1);
    for _ in 0..2 {
        sample_step(&config, &mut replayed, &mut ws2, &vocab, None, &logits, &mut observer).await.unwrap();
    }
    assert_eq!(replayed.to_text().await, direct_text);
}

#[semio_framework_async_macros::async_test]
async fn sequence_state_fork_diverges_independently() {
    let config = SamplingConfig::balanced().await;
    let vocab = small_vocab().await;
    let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
    let mut base = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(config.seed).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(8).await;
    let mut observer: SamplingObservers = NullObserver.into();
    sample_step(&config, &mut base, &mut ws, &vocab, None, &logits, &mut observer).await.unwrap();

    let key_a = StreamKey { request: 0, sequence: 2, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
    let key_b = StreamKey { request: 0, sequence: 3, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
    let mut fork_a = base.fork(SequenceId::new(2), key_a).await;
    let mut fork_b = base.fork(SequenceId::new(3), key_b).await;
    assert_eq!(fork_a.generated().await, fork_b.generated().await);

    let mut ws_a = LogitsWorkspace::new(8).await;
    let mut ws_b = LogitsWorkspace::new(8).await;
    for _ in 0..5 {
        sample_step(&config, &mut fork_a, &mut ws_a, &vocab, None, &logits, &mut observer).await.unwrap();
        sample_step(&config, &mut fork_b, &mut ws_b, &vocab, None, &logits, &mut observer).await.unwrap();
    }
    assert_ne!(fork_a.generated().await, fork_b.generated().await, "independently split RNG streams should diverge over several draws");
}

#[semio_framework_async_macros::async_test]
async fn sequence_state_to_text_round_trips_and_rejects_fingerprint_mismatch() {
    let config = SamplingConfig::precise().await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let vocab = small_vocab().await;
    let mut ws = LogitsWorkspace::new(8).await;
    let mut observer: SamplingObservers = NullObserver.into();
    let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
    sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).await.unwrap();
    let text = state.to_text().await;
    let decoded = state.decode_text(&text).await.unwrap();
    assert_eq!(decoded, state.generated().await);

    let other_config = SamplingConfig::balanced().await;
    let other_state = SequenceState::new(SequenceId::new(1), Vec::new(), &other_config, CounterRng::from_seed(0).await.into()).await.unwrap();
    assert!(other_state.decode_text(&text).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sample_step_errors_when_sequence_already_finished() {
    let config = SamplingConfig { max_tokens: 1, ..SamplingConfig::precise().await };
    let vocab = small_vocab().await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(8).await;
    let mut observer: SamplingObservers = NullObserver.into();
    let logits = [1.0f32, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let result = sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).await.unwrap();
    assert_eq!(result.finish, Some(FinishReason::MaxTokens));
    assert!(sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sample_step_with_no_repeat_ngram_avoids_recreating_a_bigram() {
    let config = SamplingConfig { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, processors: vec![ProcessorSpec::NoRepeatNgram { n: 2 }], max_tokens: 6, ..SamplingConfig::default() };
    let vocab = small_vocab().await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(8).await;
    let mut observer: SamplingObservers = NullObserver.into();
    let mut logits = [0.0f32; 8];
    logits[1] = 10.0;
    logits[0] = 5.0;
    let mut tokens = Vec::new();
    for _ in 0..4 {
        let result = sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).await.unwrap();
        tokens.push(result.token);
        if result.finish.is_some() {
            break;
        }
    }
    for pair in tokens.windows(3) {
        assert!(!(pair[0] == pair[2] && pair[1] == TokenId::new(1) && pair[0] == TokenId::new(1)), "must not recreate the (1, 1) bigram twice");
    }
}

#[semio_framework_async_macros::async_test]
async fn sample_step_honors_forced_bos_at_step_zero() {
    let config = SamplingConfig { forced: ForcedSpec { bos: Some(TokenId::new(3)), ..ForcedSpec::default() }, ..SamplingConfig::precise().await };
    let vocab = small_vocab().await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(8).await;
    let mut observer: SamplingObservers = NullObserver.into();
    let logits = [10.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let result = sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).await.unwrap();
    assert_eq!(result.token, TokenId::new(3));
}

// #endregion 🔖️SequenceStateTests

// #region 🔖️ConstraintsTests
#[semio_framework_async_macros::async_test]
async fn sample_step_with_regex_constraint_only_ever_emits_matching_text() {
    let config = SamplingConfig { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, constraints: vec![ConstraintSpec::Regex("(a|b)".into())], max_tokens: 1, ..SamplingConfig::default() };
    let tokens: Vec<&[u8]> = vec![b"z", b"a", b"b"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let vocab = Vocabulary::new(3).await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(3).await;
    let mut observer: SamplingObservers = NullObserver.into();
    let logits = [10.0f32, 1.0, 1.0];
    let result = sample_step(&config, &mut state, &mut ws, &vocab, Some(&adapter), &logits, &mut observer).await.unwrap();
    assert_ne!(result.token, TokenId::new(0));
}

// #endregion 🔖️ConstraintsTests
