use super::*;

// #region 🔖️ScheduleTests
#[semio_framework_async_macros::async_test]
async fn constant_schedule_ignores_input() {
    let schedule = Schedule::Constant(0.7);
    let input = ScheduleInput { step: StepIndex::new(50), generated_len: 50, last_entropy: None };
    assert_eq!(schedule.eval(input).await, 0.7);
}

#[semio_framework_async_macros::async_test]
async fn linear_schedule_interpolates_and_clamps_at_bound() {
    let schedule = Schedule::Linear { from: 0.0, to: 1.0, over_steps: 10 };
    let at = async |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None }).await;
    assert!((at(0).await - 0.0).abs() < 1e-9);
    assert!((at(5).await - 0.5).abs() < 1e-9);
    assert!((at(10).await - 1.0).abs() < 1e-9);
    assert!((at(20).await - 1.0).abs() < 1e-9, "must clamp past over_steps");
}

#[semio_framework_async_macros::async_test]
async fn cosine_schedule_starts_and_ends_at_bounds() {
    let schedule = Schedule::Cosine { from: 1.0, to: 0.0, over_steps: 8 };
    let at = async |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None }).await;
    assert!((at(0).await - 1.0).abs() < 1e-9);
    assert!((at(8).await - 0.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn piecewise_schedule_holds_at_last_breakpoint() {
    let schedule = Schedule::Piecewise(vec![(StepIndex::new(0), 1.0), (StepIndex::new(5), 2.0), (StepIndex::new(10), 3.0)]);
    let at = async |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None }).await;
    assert_eq!(at(0).await, 1.0);
    assert_eq!(at(3).await, 1.0);
    assert_eq!(at(5).await, 2.0);
    assert_eq!(at(7).await, 2.0);
    assert_eq!(at(100).await, 3.0);
}

#[semio_framework_async_macros::async_test]
async fn by_position_schedule_clamps_past_its_length() {
    let schedule = Schedule::ByPosition(vec![0.1, 0.2, 0.3]);
    let at = async |len: usize| schedule.eval(ScheduleInput { step: StepIndex::new(0), generated_len: len, last_entropy: None }).await;
    assert_eq!(at(0).await, 0.1);
    assert_eq!(at(2).await, 0.3);
    assert_eq!(at(50).await, 0.3);
}

#[semio_framework_async_macros::async_test]
async fn entropy_scaled_schedule_clamps_to_range() {
    let schedule = Schedule::EntropyScaled { base: 0.5, gain: 1.0, min: 0.0, max: 1.0 };
    let at = async |entropy: f64| schedule.eval(ScheduleInput { step: StepIndex::new(0), generated_len: 0, last_entropy: Some(entropy) }).await;
    assert_eq!(at(-10.0).await, 0.0);
    assert_eq!(at(10.0).await, 1.0);
    assert!((at(0.0).await - 0.5).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn schedule_json_round_trips_every_non_callback_variant() {
    let schedules = vec![
        Schedule::Constant(0.5),
        Schedule::Linear { from: 0.0, to: 1.0, over_steps: 10 },
        Schedule::Exponential { from: 0.1, to: 1.0, over_steps: 5 },
        Schedule::Cosine { from: 1.0, to: 0.0, over_steps: 8 },
        Schedule::Piecewise(vec![(StepIndex::new(0), 1.0), (StepIndex::new(4), 2.0)]),
        Schedule::ByPosition(vec![0.1, 0.2]),
        Schedule::EntropyScaled { base: 0.5, gain: 1.0, min: 0.0, max: 2.0 },
    ];
    for schedule in schedules {
        let json = schedule.to_json().await;
        let parsed = Schedule::from_json(&json).await.expect("round trip should succeed");
        assert_eq!(schedule, parsed);
    }
}

#[semio_framework_async_macros::async_test]
async fn callback_schedule_serializes_but_refuses_to_deserialize() {
    // 🚫️async: E4 fn-pointer slot — `Schedule::Callback` holds a plain `fn(ScheduleInput) ->
    // f64` (an `async fn`'s pointer type is unnameable), so this helper must stay sync too.
    fn double(input: ScheduleInput) -> f64 {
        input.step.get() as f64 * 2.0
    }
    let schedule = Schedule::Callback(double);
    let json = schedule.to_json().await;
    assert!(Schedule::from_json(&json).await.is_err());
}

// #endregion 🔖️ScheduleTests

// #region 🔖️ConfigTests
#[semio_framework_async_macros::async_test]
async fn default_config_validates() {
    assert!(SamplingConfig::default().validate().await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn all_presets_validate() {
    assert!(SamplingConfig::precise().await.validate().await.is_ok());
    assert!(SamplingConfig::balanced().await.validate().await.is_ok());
    assert!(SamplingConfig::creative().await.validate().await.is_ok());
    assert!(SamplingConfig::deterministic_test().await.validate().await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn builder_produces_a_validated_config() {
    let config = SamplingConfigBuilder::new()
        .await
        .method(SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch })
        .await
        .processor(ProcessorSpec::Temperature { value: Schedule::Constant(0.8) })
        .await
        .seed(7)
        .await
        .max_tokens(128)
        .await
        .build()
        .await
        .expect("valid config");
    assert_eq!(config.seed, 7);
    assert_eq!(config.max_tokens, 128);
    assert_eq!(config.processors.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_min_tokens_above_max_tokens() {
    let config = SamplingConfig { min_tokens: 10, max_tokens: 5, ..SamplingConfig::default() };
    assert!(config.validate().await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_zero_candidate_count() {
    let config = SamplingConfig { candidate_count: 0, ..SamplingConfig::default() };
    assert!(config.validate().await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_candidate_count_above_limit() {
    let limits = SamplingLimits { max_candidates: 4, ..SamplingLimits::default() };
    let config = SamplingConfig { candidate_count: 5, limits, ..SamplingConfig::default() };
    assert!(config.validate().await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_too_many_stop_sequences() {
    let limits = SamplingLimits { max_stop_sequences: 1, ..SamplingLimits::default() };
    let config = SamplingConfig { limits, stops: StopSpec { sequences: vec![b"a".to_vec(), b"b".to_vec()], ..StopSpec::default() }, ..SamplingConfig::default() };
    assert!(config.validate().await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_zero_no_repeat_ngram_order() {
    let config = SamplingConfig { processors: vec![ProcessorSpec::NoRepeatNgram { n: 0 }], ..SamplingConfig::default() };
    assert!(config.validate().await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn validate_rejects_mismatched_token_class_penalty_lengths() {
    let config = SamplingConfig { processors: vec![ProcessorSpec::TokenClassPenalty { class_tokens: vec![vec![TokenId::new(0)], vec![TokenId::new(1)]], factors: vec![0.5] }], ..SamplingConfig::default() };
    assert!(config.validate().await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn config_fingerprint_is_stable_and_sensitive_to_changes() {
    let a = SamplingConfig::balanced().await;
    let b = SamplingConfig::balanced().await;
    assert_eq!(a.fingerprint().await, b.fingerprint().await);
    let c = SamplingConfig { seed: a.seed + 1, ..a.clone() };
    assert_ne!(a.fingerprint().await, c.fingerprint().await);
}

#[semio_framework_async_macros::async_test]
async fn config_json_round_trips_core_fields() {
    let config = SamplingConfigBuilder::new()
        .await
        .method(SamplingMethod::GumbelTopK { k: 3 })
        .await
        .processor(ProcessorSpec::TopK { k: Schedule::Constant(40.0), min_keep: 1 })
        .await
        .processor(ProcessorSpec::RepetitionPenalty { penalty: 1.2, scope: PenaltyScope::GeneratedOnly })
        .await
        .seed(99)
        .await
        .candidate_count(2)
        .await
        .min_tokens(1)
        .await
        .max_tokens(200)
        .await
        .build()
        .await
        .expect("valid config");
    let json = config.to_json().await;
    let parsed = SamplingConfig::from_json(&json).await.expect("valid round trip");
    assert_eq!(parsed.method, config.method);
    assert_eq!(parsed.processors, config.processors);
    assert_eq!(parsed.seed, config.seed);
    assert_eq!(parsed.candidate_count, config.candidate_count);
    assert_eq!(parsed.min_tokens, config.min_tokens);
    assert_eq!(parsed.max_tokens, config.max_tokens);
}

#[semio_framework_async_macros::async_test]
async fn config_json_rejects_unknown_version() {
    let json = JsonValue::Object(vec![("version".into(), JsonValue::Num(2.0))]);
    assert!(matches!(SamplingConfig::from_json(&json).await, Err(SamplingError::SerializationVersion { expected: 1, actual: 2 })));
}

#[semio_framework_async_macros::async_test]
async fn all_processor_spec_variants_round_trip_through_json() {
    let specs = vec![
        ProcessorSpec::Temperature { value: Schedule::Constant(0.7) },
        ProcessorSpec::DynamicTemperature { base: Schedule::Constant(0.5), entropy_gain: 0.1, min: 0.0, max: 2.0 },
        ProcessorSpec::TopK { k: Schedule::Constant(40.0), min_keep: 1 },
        ProcessorSpec::TopP { p: Schedule::Constant(0.9), min_keep: 1 },
        ProcessorSpec::MinP { p: Schedule::Constant(0.05), min_keep: 1 },
        ProcessorSpec::Typical { mass: Schedule::Constant(0.9), min_keep: 1 },
        ProcessorSpec::LocallyTypical { mass: Schedule::Constant(0.9), min_keep: 1 },
        ProcessorSpec::TailFree { z: Schedule::Constant(0.95), min_keep: 1 },
        ProcessorSpec::Epsilon { cutoff: Schedule::Constant(0.001), min_keep: 1 },
        ProcessorSpec::Eta { cutoff: Schedule::Constant(0.001), min_keep: 1 },
        ProcessorSpec::TopA { power: Schedule::Constant(2.0), min_keep: 1 },
        ProcessorSpec::RankTruncation { max_rank: 50 },
        ProcessorSpec::AdaptiveTruncation { target_entropy: Some(2.0), target_effective_count: None },
        ProcessorSpec::RepetitionPenalty { penalty: 1.1, scope: PenaltyScope::GeneratedOnly },
        ProcessorSpec::PresencePenalty { penalty: 0.5, scope: PenaltyScope::PromptAndGenerated },
        ProcessorSpec::FrequencyPenalty { penalty: 0.3, scope: PenaltyScope::PromptOnly },
        ProcessorSpec::DecayingPenalty { penalty: 0.5, window: 32, half_life: 4.0, scope: PenaltyScope::GeneratedOnly },
        ProcessorSpec::TokenClassPenalty { class_tokens: vec![vec![TokenId::new(0)], vec![TokenId::new(1), TokenId::new(2)]], factors: vec![0.5, 0.9] },
        ProcessorSpec::NoRepeatNgram { n: 3 },
        ProcessorSpec::PhrasePenalty { phrases: vec![vec![TokenId::new(1), TokenId::new(2)]], penalty: 0.4 },
        ProcessorSpec::LogitBiasSparse { entries: vec![(TokenId::new(5), 2.0)] },
        ProcessorSpec::LogitBiasDense { values: vec![0.0, 1.0, -1.0] },
        ProcessorSpec::AllowTokens { tokens: vec![TokenId::new(1)] },
        ProcessorSpec::ForbidTokens { tokens: vec![TokenId::new(2)] },
        ProcessorSpec::SuppressSpecial,
        ProcessorSpec::BadWords { phrases: vec![vec![TokenId::new(3)]] },
        ProcessorSpec::SequenceEncouragement { phrases: vec![vec![TokenId::new(4)]], bonus: 1.5 },
        ProcessorSpec::Mirostat { version: MirostatVersion::V2, target_surprise: 5.0, learning_rate: 0.1 },
        ProcessorSpec::EntropyPid { target: 2.0, kp: 0.1, ki: 0.01, kd: 0.0 },
        ProcessorSpec::RepetitionController { window: 16, threshold: 0.5, boost: 0.2 },
        ProcessorSpec::ConfidenceController { low_entropy: 0.5, high_entropy: 3.0, low_temp: 0.5, high_temp: 1.2 },
    ];
    for spec in specs {
        let json = processor_spec_to_json(&spec).await;
        let parsed = processor_spec_from_json(&json).await.expect("round trip should succeed");
        assert_eq!(spec, parsed);
    }
}

// #endregion 🔖️ConfigTests
