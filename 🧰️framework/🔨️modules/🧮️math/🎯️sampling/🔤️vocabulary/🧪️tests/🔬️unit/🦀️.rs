use super::*;

// #region 🔖️IdsTests
#[semio_framework_async_macros::async_test]
async fn step_index_checked_next_overflows_to_none() {
    assert_eq!(StepIndex::new(u32::MAX).checked_next(), None);
    assert_eq!(StepIndex::new(0).checked_next(), Some(StepIndex::new(1)));
}

#[semio_framework_async_macros::async_test]
async fn ids_display_show_raw_value() {
    assert_eq!(format!("{}", TokenId::new(42)), "42");
    assert_eq!(format!("{}", SequenceId::new(7)), "7");
    assert_eq!(format!("{}", StepIndex::new(3)), "3");
}

// #endregion 🔖️IdsTests

// #region 🔖️ErrorsTests
#[semio_framework_async_macros::async_test]
async fn every_error_variant_has_nonempty_display() {
    let errors = [
        SamplingError::InvalidConfig { field: "temperature", reason: "must be >= 0" },
        SamplingError::VocabMismatch { expected: 10, actual: 5 },
        SamplingError::NonFiniteLogits { index: 3 },
        SamplingError::EmptyDistribution,
        SamplingError::ConstraintDead { constraint: "regex" },
        SamplingError::LimitExceeded { limit: "max_beam_width" },
        SamplingError::GrammarParse { offset: 0, reason: "unexpected token" },
        SamplingError::RegexParse { offset: 0, reason: "unbalanced paren" },
        SamplingError::AutomatonBudget { budget: "max_automaton_states" },
        SamplingError::SerializationVersion { expected: 1, actual: 2 },
        SamplingError::FingerprintMismatch,
        SamplingError::Corrupted { reason: "bad header" },
        SamplingError::Collective { reason: "timeout" },
        SamplingError::Callback { reason: "reranker panicked" },
        SamplingError::Cancelled,
    ];
    for error in &errors {
        assert!(!format!("{error}").is_empty());
        let _: &dyn std::error::Error = error;
    }
}

#[semio_framework_async_macros::async_test]
async fn fallback_ladder_prefers_forced_over_eos_over_argmax_over_error() {
    let forced = Some(TokenId::new(1));
    let eos = Some(TokenId::new(2));
    let argmax = Some(TokenId::new(3));
    assert_eq!(resolve_fallback(forced, eos, argmax).await, (FallbackAction::ForcedToken, forced));
    assert_eq!(resolve_fallback(None, eos, argmax).await, (FallbackAction::Eos, eos));
    assert_eq!(resolve_fallback(None, None, argmax).await, (FallbackAction::ArgmaxRaw, argmax));
    assert_eq!(resolve_fallback(None, None, None).await, (FallbackAction::Error, None));
}

// #endregion 🔖️ErrorsTests

// #region 🔖️LimitsTests
#[semio_framework_async_macros::async_test]
async fn default_limits_validate() {
    assert!(SamplingLimits::default().validate().await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn zero_limit_fails_validation() {
    let limits = SamplingLimits { max_beam_width: 0, ..SamplingLimits::default() };
    assert!(limits.validate().await.is_err());
}

// #endregion 🔖️LimitsTests

// #region 🔖️JsonTests
#[semio_framework_async_macros::async_test]
async fn json_round_trips_basic_values() {
    let text = r#"{"a":1,"b":[true,false,null,"x\ny"],"c":{"d":-2.5}}"#;
    let value = parse_json(text, 64).await.expect("valid json");
    let a = match value.get("a").await {
        Some(v) => v.as_f64().await,
        None => None,
    };
    assert_eq!(a, Some(1.0));
    let b_field = match value.get("b").await {
        Some(v) => v.as_array().await,
        None => None,
    };
    let b = b_field.expect("array");
    assert_eq!(b[0], JsonValue::Bool(true));
    assert_eq!(b[3], JsonValue::Str("x\ny".to_string()));
    let c_d = match value.get("c").await {
        Some(c) => c.get("d").await,
        None => None,
    };
    let c_d_num = match c_d {
        Some(v) => v.as_f64().await,
        None => None,
    };
    assert_eq!(c_d_num, Some(-2.5));

    let written = write_json(&value).await;
    let reparsed = parse_json(&written, 64).await.expect("valid round-trip json");
    assert_eq!(value, reparsed);
}

#[semio_framework_async_macros::async_test]
async fn json_depth_cap_is_enforced() {
    let nested = "[".repeat(10) + &"]".repeat(10);
    assert!(parse_json(&nested, 5).await.is_err());
    assert!(parse_json(&nested, 20).await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn json_rejects_trailing_garbage() {
    assert!(parse_json("1 2", 8).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn json_rejects_malformed_literals_strings_and_numbers() {
    assert!(parse_json("", 8).await.is_err());
    assert!(parse_json("nul", 8).await.is_err());
    assert!(parse_json("truX", 8).await.is_err());
    assert!(parse_json("falsy", 8).await.is_err());
    assert!(parse_json("?", 8).await.is_err());
    assert!(parse_json("\"unterminated", 8).await.is_err());
    assert!(parse_json("\"bad\\x\"", 8).await.is_err());
    assert!(parse_json("\"\\", 8).await.is_err());
    assert!(parse_json("\"\\u12\"", 8).await.is_err());
    assert!(parse_json("--1", 8).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn json_rejects_malformed_arrays_and_objects() {
    assert!(parse_json("[1 2]", 8).await.is_err());
    assert!(parse_json("[1,]", 8).await.is_err());
    assert!(parse_json("{1:2}", 8).await.is_err());
    assert!(parse_json("{\"a\" 1}", 8).await.is_err());
    assert!(parse_json("{\"a\":1 \"b\":2}", 8).await.is_err());
    assert!(parse_json("[", 8).await.is_err());
    assert!(parse_json("{", 8).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn json_parses_empty_array_and_object_and_unicode_escape() {
    assert_eq!(parse_json("[]", 8).await.unwrap(), JsonValue::Array(Vec::new()));
    assert_eq!(parse_json("{}", 8).await.unwrap(), JsonValue::Object(Vec::new()));
    let value = parse_json("\"\\u0041\"", 8).await.unwrap();
    assert_eq!(value, JsonValue::Str("A".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn json_value_accessors_return_none_for_mismatched_variants() {
    let value = JsonValue::Str("x".to_string());
    assert_eq!(value.as_f64().await, None);
    assert_eq!(value.as_bool().await, None);
    assert_eq!(value.as_array().await, None);
    assert_eq!(value.get("k").await, None);
    assert_eq!(JsonValue::Bool(true).as_str().await, None);
}

#[semio_framework_async_macros::async_test]
async fn write_json_escapes_control_characters_and_formats_non_integral_numbers() {
    let value = JsonValue::Object(vec![("s".to_string(), JsonValue::Str("a\u{1}b".to_string())), ("n".to_string(), JsonValue::Num(1.5))]);
    let written = write_json(&value).await;
    assert!(written.contains("\\u0001"));
    assert!(written.contains("1.5"));
    assert_eq!(write_json(&JsonValue::Num(3.0)).await, "3");
}

// #endregion 🔖️JsonTests

// #region 🔖️Utf8Tests
#[semio_framework_async_macros::async_test]
async fn utf8_status_classifies_complete_partial_invalid() {
    assert_eq!(utf8_status(b"hello").await, Utf8Status::Complete);
    assert_eq!(utf8_status("héllo".as_bytes()).await, Utf8Status::Complete);
    let full = "é".as_bytes();
    assert_eq!(utf8_status(&full[..1]).await, Utf8Status::Partial { more: 1 });
    assert_eq!(utf8_status(&[0xFF]).await, Utf8Status::Invalid);
    assert_eq!(utf8_status(b"").await, Utf8Status::Complete);
}

// #endregion 🔖️Utf8Tests

// #region 🔖️NumericsTests
#[semio_framework_async_macros::async_test]
async fn softmax_live_sums_to_one_and_matches_hand_computed() {
    let logits = [1.0f32, 2.0, 3.0];
    let live = [0u32, 1, 2];
    let mut probs = [0.0f32; 3];
    softmax_live(&logits, &live, &mut probs, Accum::F64).await;
    let sum: f32 = probs.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6);
    let expected = [0.09003057f32, 0.24472847, 0.66524096];
    for (p, e) in probs.iter().zip(expected.iter()) {
        assert!((p - e).abs() < 1e-5, "{p} vs {e}");
    }
}

#[semio_framework_async_macros::async_test]
async fn softmax_live_f32_and_f64_accum_agree_closely() {
    let logits: Vec<f32> = (0..50).map(|i| (i as f32) * 0.37 - 5.0).collect();
    let live: Vec<u32> = (0..50).collect();
    let mut probs_32 = vec![0.0f32; 50];
    let mut probs_64 = vec![0.0f32; 50];
    softmax_live(&logits, &live, &mut probs_32, Accum::F32).await;
    softmax_live(&logits, &live, &mut probs_64, Accum::F64).await;
    for (a, b) in probs_32.iter().zip(probs_64.iter()) {
        assert!((a - b).abs() < 1e-3, "{a} vs {b}");
    }
}

#[semio_framework_async_macros::async_test]
async fn softmax_live_is_invariant_to_constant_shift() {
    let base = [1.0f32, 2.0, 3.0];
    let shifted = [1001.0f32, 1002.0, 1003.0];
    let live = [0u32, 1, 2];
    let mut probs_base = [0.0f32; 3];
    let mut probs_shifted = [0.0f32; 3];
    softmax_live(&base, &live, &mut probs_base, Accum::F64).await;
    softmax_live(&shifted, &live, &mut probs_shifted, Accum::F64).await;
    for (a, b) in probs_base.iter().zip(probs_shifted.iter()) {
        assert!((a - b).abs() < 1e-5);
    }
}

#[semio_framework_async_macros::async_test]
async fn logsumexp_matches_naive_for_moderate_values() {
    let values: [f64; 4] = [0.1, 0.2, 0.3, -0.5];
    let naive = values.iter().map(|v| v.exp()).sum::<f64>().ln();
    assert!((logsumexp_f64(&values).await - naive).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn logsumexp_handles_all_neg_infinity() {
    assert_eq!(logsumexp_f64(&[f64::NEG_INFINITY, f64::NEG_INFINITY]).await, f64::NEG_INFINITY);
}

#[semio_framework_async_macros::async_test]
async fn entropy_of_uniform_distribution_is_ln_n() {
    let probs = [0.25f32; 4];
    assert!((entropy_nats(&probs).await - (4.0f64).ln()).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn entropy_of_deterministic_distribution_is_zero() {
    let probs = [1.0f32, 0.0, 0.0];
    assert!(entropy_nats(&probs).await.abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn effective_candidate_count_matches_perplexity_of_uniform() {
    let probs = [0.125f32; 8];
    assert!((effective_candidate_count(&probs).await - 8.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn kahan_sum_matches_naive_sum_for_short_sequences() {
    let mut sum = KahanSum::new().await;
    let values = [1.0, 2.0, 3.0, 4.5];
    for v in values {
        sum.add(v).await;
    }
    assert!((sum.value().await - 10.5).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn partial_select_top_k_selects_highest_k_by_logit_with_tie_break() {
    let logits = [5.0f32, 1.0, 5.0, 3.0, 2.0];
    let mut live: Vec<u32> = (0..5).collect();
    let mut scratch = vec![0.0f32; 5];
    partial_select_top_k(&logits, &mut live, 3, &mut scratch).await;
    let mut top3 = live[..3].to_vec();
    top3.sort_unstable();
    assert_eq!(top3, vec![0, 2, 3]);
}

#[semio_framework_async_macros::async_test]
async fn partial_select_top_k_is_noop_when_k_covers_everything() {
    let logits = [1.0f32, 2.0, 3.0];
    let mut live: Vec<u32> = (0..3).collect();
    let mut scratch = vec![0.0f32; 3];
    let before = live.clone();
    partial_select_top_k(&logits, &mut live, 3, &mut scratch).await;
    assert_eq!(live, before);
}

#[semio_framework_async_macros::async_test]
async fn cdf_binary_search_finds_first_index_at_or_above_u() {
    let cdf = [0.2, 0.5, 0.5, 0.9, 1.0];
    assert_eq!(cdf_binary_search(&cdf, 0.0).await, 0);
    assert_eq!(cdf_binary_search(&cdf, 0.2).await, 0);
    assert_eq!(cdf_binary_search(&cdf, 0.21).await, 1);
    assert_eq!(cdf_binary_search(&cdf, 0.5).await, 1);
    assert_eq!(cdf_binary_search(&cdf, 0.95).await, 4);
    assert_eq!(cdf_binary_search(&cdf, 1.0).await, 4);
}

#[semio_framework_async_macros::async_test]
async fn sanitize_logits_neg_inf_nan_policy_masks_nan_and_rejects_pos_inf() {
    let mut logits = [1.0f32, f32::NAN, f32::NEG_INFINITY];
    let altered = sanitize_logits(&mut logits, SanitizePolicy::NegInfNan).await.expect("no +inf present");
    assert_eq!(altered, 1);
    assert_eq!(logits[1], f32::NEG_INFINITY);

    let mut with_pos_inf = [1.0f32, f32::INFINITY];
    assert!(sanitize_logits(&mut with_pos_inf, SanitizePolicy::NegInfNan).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sanitize_logits_error_policy_rejects_any_non_finite() {
    let mut logits = [1.0f32, f32::NAN];
    assert!(sanitize_logits(&mut logits, SanitizePolicy::Error).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sanitize_logits_clamp_inf_policy_clamps_positive_infinity() {
    let mut logits = [1.0f32, f32::INFINITY];
    let altered = sanitize_logits(&mut logits, SanitizePolicy::ClampInf).await.expect("clamp policy never errors on inf");
    assert_eq!(altered, 1);
    assert_eq!(logits[1], f32::MAX);
}

// #endregion 🔖️NumericsTests

// #region 🔖️BitsetTests
#[semio_framework_async_macros::async_test]
async fn bitset_new_full_has_exactly_len_bits_set() {
    let set = TokenBitset::new_full(70).await;
    assert_eq!(set.count_ones().await, 70);
    for i in 0..70 {
        assert!(set.get(TokenId::new(i)).await);
    }
}

#[semio_framework_async_macros::async_test]
async fn bitset_set_get_round_trip() {
    let mut set = TokenBitset::new_empty(10).await;
    set.set(TokenId::new(3), true).await;
    set.set(TokenId::new(7), true).await;
    assert!(set.get(TokenId::new(3)).await);
    assert!(set.get(TokenId::new(7)).await);
    assert!(!set.get(TokenId::new(4)).await);
    assert_eq!(set.count_ones().await, 2);
    set.set(TokenId::new(3), false).await;
    assert!(!set.get(TokenId::new(3)).await);
}

#[semio_framework_async_macros::async_test]
async fn bitset_and_or_and_not_operations() {
    let mut a = TokenBitset::new_empty(8).await;
    let mut b = TokenBitset::new_empty(8).await;
    a.set(TokenId::new(0), true).await;
    a.set(TokenId::new(1), true).await;
    b.set(TokenId::new(1), true).await;
    b.set(TokenId::new(2), true).await;

    let mut and = a.clone();
    and.and_with(&b).await;
    assert_eq!(and.count_ones().await, 1);
    assert!(and.get(TokenId::new(1)).await);

    let mut or = a.clone();
    or.or_with(&b).await;
    assert_eq!(or.count_ones().await, 3);

    let mut and_not = a.clone();
    and_not.and_not_with(&b).await;
    assert_eq!(and_not.count_ones().await, 1);
    assert!(and_not.get(TokenId::new(0)).await);
}

#[semio_framework_async_macros::async_test]
async fn bitset_iter_ones_skips_zero_words_across_boundaries() {
    let mut set = TokenBitset::new_empty(200).await;
    set.set(TokenId::new(5), true).await;
    set.set(TokenId::new(130), true).await;
    set.set(TokenId::new(199), true).await;
    let ones: Vec<u32> = set.iter_ones().await.map(TokenId::get).collect();
    assert_eq!(ones, vec![5, 130, 199]);
}

#[semio_framework_async_macros::async_test]
async fn bitset_first_set_and_is_all_zero() {
    let mut set = TokenBitset::new_empty(64).await;
    assert!(set.is_all_zero().await);
    assert_eq!(set.first_set().await, None);
    set.set(TokenId::new(40), true).await;
    assert!(!set.is_all_zero().await);
    assert_eq!(set.first_set().await, Some(TokenId::new(40)));
}

// #endregion 🔖️BitsetTests

// #region 🔖️RngTests
#[semio_framework_async_macros::async_test]
async fn counter_rng_is_deterministic_for_same_seed() {
    let mut a = CounterRng::from_seed(123).await;
    let mut b = CounterRng::from_seed(123).await;
    let mut seq_a: Vec<u64> = Vec::new();
    for _ in 0..32 {
        seq_a.push(a.next_u64().await);
    }
    let mut seq_b: Vec<u64> = Vec::new();
    for _ in 0..32 {
        seq_b.push(b.next_u64().await);
    }
    assert_eq!(seq_a, seq_b);
}

#[semio_framework_async_macros::async_test]
async fn counter_rng_different_seeds_diverge() {
    let mut a = CounterRng::from_seed(1).await;
    let mut b = CounterRng::from_seed(2).await;
    let mut seq_a: Vec<u64> = Vec::new();
    for _ in 0..8 {
        seq_a.push(a.next_u64().await);
    }
    let mut seq_b: Vec<u64> = Vec::new();
    for _ in 0..8 {
        seq_b.push(b.next_u64().await);
    }
    assert_ne!(seq_a, seq_b);
}

#[semio_framework_async_macros::async_test]
async fn counter_rng_split_is_independent_of_call_order() {
    let parent = CounterRng::from_seed(999).await;
    let key_a = StreamKey { request: 1, sequence: 2, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
    let key_b = StreamKey { request: 1, sequence: 3, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };

    let mut a_first = parent.split(key_a).await;
    let mut b_first = parent.split(key_b).await;
    let mut a_vals_1: Vec<u64> = Vec::new();
    for _ in 0..4 {
        a_vals_1.push(a_first.next_u64().await);
    }
    let mut b_vals_1: Vec<u64> = Vec::new();
    for _ in 0..4 {
        b_vals_1.push(b_first.next_u64().await);
    }

    let mut b_second = parent.split(key_b).await;
    let mut a_second = parent.split(key_a).await;
    let mut b_vals_2: Vec<u64> = Vec::new();
    for _ in 0..4 {
        b_vals_2.push(b_second.next_u64().await);
    }
    let mut a_vals_2: Vec<u64> = Vec::new();
    for _ in 0..4 {
        a_vals_2.push(a_second.next_u64().await);
    }

    assert_eq!(a_vals_1, a_vals_2);
    assert_eq!(b_vals_1, b_vals_2);
    assert_ne!(a_vals_1, b_vals_1);
}

#[semio_framework_async_macros::async_test]
async fn counter_rng_split_differs_by_purpose() {
    let parent = CounterRng::from_seed(42).await;
    let base = StreamKey { request: 1, sequence: 1, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
    let gumbel = StreamKey { purpose: StreamPurpose::Gumbel, ..base };
    let mut a = parent.split(base).await;
    let mut b = parent.split(gumbel).await;
    assert_ne!(a.next_u64().await, b.next_u64().await);
}

#[semio_framework_async_macros::async_test]
async fn counter_rng_snapshot_restore_resumes_identically() {
    let mut original = CounterRng::from_seed(77).await;
    for _ in 0..9 {
        original.next_u64().await;
    }
    let snapshot = original.snapshot().await;
    let mut resumed = CounterRng::from_seed(0).await;
    resumed.restore(&snapshot).await.expect("matching kind restores cleanly");
    let mut expected: Vec<u64> = Vec::new();
    for _ in 0..16 {
        expected.push(original.next_u64().await);
    }
    let mut actual: Vec<u64> = Vec::new();
    for _ in 0..16 {
        actual.push(resumed.next_u64().await);
    }
    assert_eq!(expected, actual);
}

#[semio_framework_async_macros::async_test]
async fn rng_snapshot_rejects_kind_mismatch_on_restore() {
    let snapshot = CounterRng::from_seed(1).await.snapshot().await;
    let mut xoshiro = XoshiroSource::from_seed(1).await;
    assert!(xoshiro.restore(&snapshot).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn rng_snapshot_text_round_trips() {
    let snapshot = RngSnapshot { kind: RngKind::Counter, words: [1, 2, 3, 4] };
    let text = snapshot.to_text().await;
    let parsed = RngSnapshot::from_text(&text).await.expect("valid snapshot text");
    assert_eq!(snapshot, parsed);
}

#[semio_framework_async_macros::async_test]
async fn xoshiro_source_matches_underlying_rng_sequence() {
    let mut source = XoshiroSource::from_seed(4242).await;
    let mut reference = semio_framework_geometry::random::Rng::from_seed(4242);
    for _ in 0..16 {
        assert_eq!(source.next_u64().await, reference.next_u64());
    }
}

#[semio_framework_async_macros::async_test]
async fn next_f64_open01_is_never_zero() {
    let mut rng: RandomSources = CounterRng::from_seed(0).await.into();
    for _ in 0..1000 {
        let u = rng.next_f64_open01().await;
        assert!(u > 0.0 && u <= 1.0, "u = {u} out of (0, 1]");
    }
}

#[semio_framework_async_macros::async_test]
async fn next_range_stays_within_bounds() {
    let mut rng: RandomSources = CounterRng::from_seed(5).await.into();
    for _ in 0..1000 {
        let x = rng.next_range(3, 9).await;
        assert!((3..9).contains(&x));
    }
    assert_eq!(rng.next_range(4, 4).await, 4);
}

// #endregion 🔖️RngTests

// #region 🔖️VocabularyTests
#[semio_framework_async_macros::async_test]
async fn vocabulary_validates_logits_length() {
    let vocab = Vocabulary::new(10).await;
    assert!(vocab.validate_logits_len(10).await.is_ok());
    assert!(vocab.validate_logits_len(9).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn vocabulary_is_eos_reflects_configured_set() {
    let vocab = Vocabulary::new(10).await.with_eos(vec![TokenId::new(0), TokenId::new(1)]).await;
    assert!(vocab.is_eos(TokenId::new(0)).await);
    assert!(!vocab.is_eos(TokenId::new(2)).await);
}

#[semio_framework_async_macros::async_test]
async fn slice_text_adapter_returns_bytes_and_stable_fingerprint() {
    let tokens: Vec<&[u8]> = vec![b"ab", b"c"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    assert_eq!(adapter.vocab_size().await, 2);
    assert_eq!(adapter.token_bytes(TokenId::new(0)).await, Some(b"ab".as_slice()));
    assert_eq!(adapter.token_bytes(TokenId::new(5)).await, None);
    let fp1 = adapter.fingerprint().await;
    let fp2 = SliceTextAdapter::new(&tokens).await.fingerprint().await;
    assert_eq!(fp1, fp2);

    let different: Vec<&[u8]> = vec![b"a", b"bc"];
    let fp3 = SliceTextAdapter::new(&different).await.fingerprint().await;
    assert_ne!(fp1, fp3, "separator byte must prevent boundary-shift collisions");
}

// #endregion 🔖️VocabularyTests
