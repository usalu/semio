
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
    // 📐️ Hand-computed via exp(x - 3): [exp(-2), exp(-1), exp(0)] / sum
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

    // 🎲️ Splitting in either order from the same parent must produce identical child streams.
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
    let mut reference = geometry::random::Rng::from_seed(4242).await;
    for _ in 0..16 {
        assert_eq!(source.next_u64().await, reference.next_u64().await);
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

// #region 🔖️WorkspaceTests
async fn small_vocab() -> Vocabulary {
    Vocabulary::new(8).await.with_eos(vec![TokenId::new(7)]).await
}

async fn step_view<'a>(vocab: &'a Vocabulary, prompt: &'a [TokenId], generated: &'a [TokenId]) -> StepView<'a> {
    StepView { sequence: SequenceId::new(1), step: StepIndex::new(generated.len() as u32), prompt, generated, vocab, adapter: None, last_entropy: None }
}

#[semio_framework_async_macros::async_test]
async fn workspace_reset_for_step_initializes_live_to_full_vocab_and_argmax() {
    let mut ws = LogitsWorkspace::new(5).await;
    let logits = [1.0f32, 3.0, 2.0, 3.0, 0.0];
    ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).await.expect("finite logits never error");
    assert_eq!(ws.live().await, &[0, 1, 2, 3, 4]);
    // 📐️ Ties between indices 1 and 3 (both 3.0) break toward the lowest token id.
    assert_eq!(ws.saved_argmax().await, TokenId::new(1));
}

#[semio_framework_async_macros::async_test]
async fn workspace_sync_live_with_mask_removes_masked_entries() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).await.unwrap();
    ws.mask_mut().await.set(TokenId::new(2), false).await;
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[0, 1, 3]);
}

#[semio_framework_async_macros::async_test]
async fn workspace_sort_live_by_prob_desc_orders_by_probability_then_token_id() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 3.0, 3.0, 0.5], SanitizePolicy::NegInfNan).await.unwrap();
    ws.sort_live_by_prob_desc().await;
    assert_eq!(ws.live().await, &[1, 2, 0, 3]);
    assert!(ws.probs().await[0] >= ws.probs().await[1]);
    assert!(ws.probs().await[1] >= ws.probs().await[2]);
}

#[semio_framework_async_macros::async_test]
async fn workspace_truncate_live_to_respects_min_keep() {
    let mut ws = LogitsWorkspace::new(5).await;
    ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    ws.sort_live_by_prob_desc().await;
    ws.truncate_live_to(1, 3).await;
    assert_eq!(ws.live().await.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn workspace_collapse_live_to_argmax_leaves_single_best_entry() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 5.0, 2.0, 5.0], SanitizePolicy::NegInfNan).await.unwrap();
    ws.collapse_live_to_argmax().await;
    assert_eq!(ws.live().await, &[1]);
}

#[semio_framework_async_macros::async_test]
async fn workspace_pool_reuses_released_workspace() {
    let mut pool = WorkspacePool::new().await;
    let ws = pool.acquire(16).await;
    assert_eq!(ws.vocab_size().await, 16);
    pool.release(ws).await;
    let reused = pool.acquire(16).await;
    assert_eq!(reused.vocab_size().await, 16);
}
// #endregion 🔖️WorkspaceTests

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
    // 🌡️ Logits chosen so softmax gives one dominant token (~0.87) plus a long tail.
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
    // 📐️ A peaked distribution's *natural* effective count is already low — "targeting" a
    // higher count than that can never shrink it (there's nothing to cut). Truncation only
    // makes sense the other way: start near-uniform (effective count 6) and target lower (2).
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
    logits[7] = 100.0; // 📖️ token 7 is the configured EOS token.
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
    assert_eq!(penalty.counts.count(TokenId::new(0)), 1);
    penalty.rollback_to(mark_before).await;
    assert_eq!(penalty.counts.count(TokenId::new(0)), 0);
    assert_eq!(penalty.counts.count(TokenId::new(1)), 0);
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
    // History: [0, 1, 0]. The bigram (0 -> 1) was already seen, so after another `0` the
    // engine must forbid token `1` (it would recreate that exact bigram).
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
    // 📐️ Uniform entropy (ln 4 ≈ 1.39) is far above the 0.1 target, so `error = target - entropy`
    // is very negative, driving `temp` toward (and clamped at) `0.05` — a *low* temperature that
    // sharpens the distribution (reduces entropy) by scaling logits *up* (dividing by a small
    // temp), the correct control direction for "entropy is too high, pull it down".
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
    // 📐️ Near-zero entropy selects a temperature near `low_temp` (0.2), scaling logits up ~5x.
    assert!(ws.processed().await[0] > 40.0);
}
// #endregion 🔖️AdaptiveTests

// #region 🔖️StopsTests
// 🔀️ `MockAdapter` deleted (O1 de-dyn cleanup, math-dedyn): it duplicated `SliceTextAdapter`'s
// owned `Vec<Vec<u8>>` representation exactly, and was the only reason `TokenTextAdapters`
// would have needed a second variant — `dyn_enum_close!`'s variant list has no per-variant
// `#[cfg(test)]` slot, so a test-only second implementor cannot cleanly join a production enum.
// The one test that used it now builds a `SliceTextAdapter` instead (identical behavior for
// this purpose: no test here reads `fingerprint()`, the one place the two differed).

#[semio_framework_async_macros::async_test]
async fn aho_corasick_matches_and_reports_hold_back_on_partial_prefix() {
    let ac = AhoCorasick::build(&[b"ab".to_vec()]).await;
    let s1 = ac.step(0, b'a').await;
    assert_eq!(ac.depth(s1).await, 1);
    assert!(ac.matched_at(s1).await.is_none());
    let s2 = ac.step(s1, b'b').await;
    assert!(ac.matched_at(s2).await.is_some());
}

#[semio_framework_async_macros::async_test]
async fn aho_corasick_handles_overlapping_patterns_via_fail_links() {
    let ac = AhoCorasick::build(&[b"abc".to_vec(), b"bcd".to_vec()]).await;
    let mut state = 0u32;
    for &byte in b"abcd" {
        state = ac.step(state, byte).await;
    }
    // 🛑️ After consuming "abcd", the automaton must have matched "bcd" via the fail link.
    let (index, len) = ac.matched_at(state).await.expect("bcd must match via fail link");
    assert_eq!(len, 3);
    let _ = index;
}

#[semio_framework_async_macros::async_test]
async fn token_stop_condition_fires_on_configured_token() {
    let vocab = small_vocab().await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut stop = TokenStopCondition { tokens: vec![TokenId::new(9)] };
    assert_eq!(stop.on_token(&view, TokenId::new(9)).await, StopPoll::Finished { reason: FinishReason::StopToken, matched_bytes: 0 });
    assert_eq!(stop.on_token(&view, TokenId::new(1)).await, StopPoll::Continue);
}

#[semio_framework_async_macros::async_test]
async fn text_stop_condition_matches_a_multi_token_stop_sequence() {
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&[b"He", b"llo", b"!"]).await.into();
    let vocab = Vocabulary::new(3).await;
    let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
    let mut stop = TextStopCondition::new(&[b"Hello".to_vec()], StopTextMode::Include).await;
    assert_eq!(stop.on_token(&view, TokenId::new(0)).await, StopPoll::Hold { ambiguous_bytes: 2 });
    let result = stop.on_token(&view, TokenId::new(1)).await;
    assert_eq!(result, StopPoll::Finished { reason: FinishReason::StopSequence { index: 0 }, matched_bytes: 5 });
}

#[semio_framework_async_macros::async_test]
async fn text_stop_condition_without_adapter_never_matches() {
    let vocab = Vocabulary::new(3).await;
    let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
    let mut stop = TextStopCondition::new(&[b"Hello".to_vec()], StopTextMode::Include).await;
    assert_eq!(stop.on_token(&view, TokenId::new(0)).await, StopPoll::Continue);
}
// #endregion 🔖️StopsTests

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
    // 📐️ Token 1 always dominates except when masked, so the no-repeat-ngram guard should force
    // deviation the moment the same bigram would otherwise recur.
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
    // 🌡️ Token 1 (the argmax) can never appear twice consecutively after the same predecessor.
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

// #region 🔖️AutomataTests
#[semio_framework_async_macros::async_test]
async fn dfa_matches_a_star_b_pattern() {
    let limits = SamplingLimits::default();
    let dfa = Dfa::from_pattern("a*b", &limits).await.unwrap();
    for accepted in ["b", "ab", "aaab"] {
        let mut state = dfa.start().await;
        for &byte in accepted.as_bytes() {
            state = dfa.step(state, byte).await;
        }
        assert!(dfa.is_accept(state).await, "{accepted:?} should be accepted");
    }
    for rejected in ["a", "ba", "abc", ""] {
        let mut state = dfa.start().await;
        let mut dead = false;
        for &byte in rejected.as_bytes() {
            state = dfa.step(state, byte).await;
            if dfa.is_dead(state).await {
                dead = true;
                break;
            }
        }
        assert!(dead || !dfa.is_accept(state).await, "{rejected:?} should not be accepted");
    }
}

#[semio_framework_async_macros::async_test]
async fn dfa_handles_alternation_class_and_bounded_repeat() {
    let limits = SamplingLimits::default();
    let dfa = Dfa::from_pattern("[a-c]{2,3}", &limits).await.unwrap();
    let matches = async |s: &str| {
        let mut state = dfa.start().await;
        for &byte in s.as_bytes() {
            state = dfa.step(state, byte).await;
            if dfa.is_dead(state).await {
                return false;
            }
        }
        dfa.is_accept(state).await
    };
    assert!(matches("ab").await);
    assert!(matches("abc").await);
    assert!(!matches("a").await);
    assert!(!matches("abca").await);
    assert!(!matches("ad").await);
}

#[semio_framework_async_macros::async_test]
async fn dfa_alive_flag_marks_dead_ends_as_unreachable_to_accept() {
    let limits = SamplingLimits::default();
    let dfa = Dfa::from_pattern("ab", &limits).await.unwrap();
    let mut state = dfa.start().await;
    state = dfa.step(state, b'z').await;
    assert!(dfa.is_dead(state).await);
    assert!(!dfa.is_alive(state).await);
}

#[semio_framework_async_macros::async_test]
async fn dfa_rejects_unbalanced_parens_with_parse_error() {
    let limits = SamplingLimits::default();
    assert!(Dfa::from_pattern("(ab", &limits).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn dfa_budget_is_enforced_for_pathologically_wide_patterns() {
    let limits = SamplingLimits { max_automaton_states: 4, ..SamplingLimits::default() };
    assert!(Dfa::from_pattern("(a|b|c|d|e|f|g|h){5}", &limits).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn dfa_token_cache_computes_allowed_tokens_and_next_states() {
    let limits = SamplingLimits::default();
    let dfa = Dfa::from_pattern("ab", &limits).await.unwrap();
    let tokens: Vec<&[u8]> = vec![b"a", b"b", b"x"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let mut cache = DfaTokenMemo::new(16).await;
    let (allowed, next) = cache.get_or_compute(&dfa, dfa.start().await, &adapter).await;
    assert!(allowed.get(TokenId::new(0)).await);
    assert!(!allowed.get(TokenId::new(1)).await);
    assert!(!allowed.get(TokenId::new(2)).await);
    let after_a = next[0];
    let (allowed2, _) = cache.get_or_compute(&dfa, after_a, &adapter).await;
    assert!(allowed2.get(TokenId::new(1)).await);
}

#[semio_framework_async_macros::async_test]
async fn dfa_supports_plus_optional_and_negated_class_quantifiers() {
    let limits = SamplingLimits::default();
    let matches = async |pattern: &str, s: &str| {
        let dfa = Dfa::from_pattern(pattern, &limits).await.unwrap();
        let mut state = dfa.start().await;
        for &byte in s.as_bytes() {
            state = dfa.step(state, byte).await;
            if dfa.is_dead(state).await {
                return false;
            }
        }
        dfa.is_accept(state).await
    };
    assert!(matches("a+", "aaa").await);
    assert!(!matches("a+", "").await);
    assert!(matches("ab?c", "ac").await);
    assert!(matches("ab?c", "abc").await);
    assert!(matches("[^a-c]", "d").await);
    assert!(!matches("[^a-c]", "b").await);
}

#[semio_framework_async_macros::async_test]
async fn dfa_handles_escaped_bytes_and_unbounded_repeat() {
    let limits = SamplingLimits::default();
    let dfa = Dfa::from_pattern(r"a\n{1,}", &limits).await.unwrap();
    let mut state = dfa.start().await;
    for &byte in b"a\n\n\n" {
        state = dfa.step(state, byte).await;
    }
    assert!(dfa.is_accept(state).await);
}

#[semio_framework_async_macros::async_test]
async fn regex_parse_errors_on_unclosed_class_and_dangling_escapes() {
    let limits = SamplingLimits::default();
    assert!(Dfa::from_pattern("[abc", &limits).await.is_err());
    assert!(Dfa::from_pattern("a\\", &limits).await.is_err());
    assert!(Dfa::from_pattern("[a\\", &limits).await.is_err());
    assert!(Dfa::from_pattern("a{2", &limits).await.is_err());
    assert!(Dfa::from_pattern("a).await", &limits).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn dfa_token_cache_evicts_all_entries_once_max_entries_is_reached() {
    let limits = SamplingLimits::default();
    let dfa = Dfa::from_pattern("a*b", &limits).await.unwrap();
    let tokens: Vec<&[u8]> = vec![b"a", b"b"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let mut cache = DfaTokenMemo::new(1).await;
    let start = dfa.start().await;
    cache.get_or_compute(&dfa, start, &adapter).await;
    let after_a = dfa.step(start, b'a').await;
    // 🤖️ Filling a second, distinct state must evict the first since max_entries is 1.
    cache.get_or_compute(&dfa, after_a, &adapter).await;
    assert_eq!(cache.entries.len(), 1);
    assert!(cache.entries.contains_key(&after_a));
}
// #endregion 🔖️AutomataTests

// #region 🔖️ConstraintsTests
#[semio_framework_async_macros::async_test]
async fn regex_constraint_masks_to_only_valid_continuations() {
    let limits = SamplingLimits::default();
    let mut constraint = RegexConstraint::new("ab", &limits).await.unwrap();
    let tokens: Vec<&[u8]> = vec![b"a", b"b", b"x"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let vocab = Vocabulary::new(3).await;
    let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
    let mut mask = TokenBitset::new_full(3).await;
    constraint.fill_mask(&view, &mut mask).await.unwrap();
    assert!(mask.get(TokenId::new(0)).await);
    assert!(!mask.get(TokenId::new(1)).await);
    assert!(!mask.get(TokenId::new(2)).await);
    assert!(!constraint.is_satisfied().await);
    constraint.accept(&view, TokenId::new(0)).await.unwrap();
    let mut mask2 = TokenBitset::new_full(3).await;
    constraint.fill_mask(&view, &mut mask2).await.unwrap();
    assert!(mask2.get(TokenId::new(1)).await);
    constraint.accept(&view, TokenId::new(1)).await.unwrap();
    assert!(constraint.is_satisfied().await);
}

#[semio_framework_async_macros::async_test]
async fn regex_constraint_rollback_restores_dfa_state() {
    let limits = SamplingLimits::default();
    let mut constraint = RegexConstraint::new("ab", &limits).await.unwrap();
    let tokens: Vec<&[u8]> = vec![b"a", b"b"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let vocab = Vocabulary::new(2).await;
    let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
    let mark = constraint.save().await;
    constraint.accept(&view, TokenId::new(0)).await.unwrap();
    assert!(!constraint.is_satisfied().await);
    constraint.rollback_to(mark).await;
    let mut mask = TokenBitset::new_full(2).await;
    constraint.fill_mask(&view, &mut mask).await.unwrap();
    assert!(mask.get(TokenId::new(0)).await, "rollback must restore the pre-'a' DFA state");
}

#[semio_framework_async_macros::async_test]
async fn trie_constraint_only_allows_configured_phrase_tokens() {
    let mut constraint = TrieConstraint::new(&[vec![TokenId::new(0), TokenId::new(1)], vec![TokenId::new(2)]]).await;
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let mut mask = TokenBitset::new_full(3).await;
    constraint.fill_mask(&view, &mut mask).await.unwrap();
    assert!(mask.get(TokenId::new(0)).await);
    assert!(mask.get(TokenId::new(2)).await);
    assert!(!mask.get(TokenId::new(1)).await);
    assert!(!constraint.is_satisfied().await);
    constraint.accept(&view, TokenId::new(2)).await.unwrap();
    assert!(constraint.is_satisfied().await);
    assert!(constraint.is_finished().await);
}

#[semio_framework_async_macros::async_test]
async fn must_include_constraint_is_satisfied_once_an_alternative_appears() {
    let mut constraint = MustIncludeConstraint::new(vec![vec![TokenId::new(1), TokenId::new(2)]]).await;
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    assert!(!constraint.is_satisfied().await);
    constraint.accept(&view, TokenId::new(0)).await.unwrap();
    assert!(!constraint.is_satisfied().await);
    constraint.accept(&view, TokenId::new(1)).await.unwrap();
    constraint.accept(&view, TokenId::new(2)).await.unwrap();
    assert!(constraint.is_satisfied().await);
}

#[semio_framework_async_macros::async_test]
async fn json_mode_constraint_accepts_valid_json_and_rejects_invalid() {
    let tokens: Vec<&[u8]> = vec![b"{", b"\"a\"", b":", b"1", b"}", b"]"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let vocab = Vocabulary::new(tokens.len()).await;
    let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };

    let mut good = JsonModeConstraint::new().await;
    for &tok in &[0u32, 1, 2, 3, 4] {
        good.accept(&view, TokenId::new(tok)).await.unwrap();
    }
    assert!(good.is_satisfied().await);
    assert!(!good.is_dead().await);

    let mut bad = JsonModeConstraint::new().await;
    bad.accept(&view, TokenId::new(0)).await.unwrap();
    bad.accept(&view, TokenId::new(5)).await.unwrap(); // ']' can't close an object
    assert!(bad.is_dead().await);
}

#[semio_framework_async_macros::async_test]
async fn json_schema_constraint_flags_a_schema_violation_once_json_completes() {
    let tokens: Vec<&[u8]> = vec![b"{", b"\"a\"", b":", b"1", b"}"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let vocab = Vocabulary::new(tokens.len()).await;
    let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };

    let schema = parse_json(r#"{"type":"object","required":["b"]}"#, 16).await.unwrap();
    let mut constraint = JsonSchemaConstraint::new(schema).await;
    for &tok in &[0u32, 1, 2, 3, 4] {
        constraint.accept(&view, TokenId::new(tok)).await.unwrap();
    }
    assert!(constraint.mode.is_satisfied().await);
    assert!(constraint.is_dead().await, "missing required property 'b' should be flagged");
}

#[semio_framework_async_macros::async_test]
async fn json_schema_validator_checks_type_enum_and_bounds() {
    let schema = parse_json(r#"{"type":"number","minimum":0,"maximum":10}"#, 16).await.unwrap();
    assert!(validates_json_schema(&JsonValue::Num(5.0), &schema).await);
    assert!(!validates_json_schema(&JsonValue::Num(-1.0), &schema).await);
    assert!(!validates_json_schema(&JsonValue::Str("x".into()), &schema).await);

    let enum_schema = parse_json(r#"{"enum":["a","b"]}"#, 16).await.unwrap();
    assert!(validates_json_schema(&JsonValue::Str("a".into()), &enum_schema).await);
    assert!(!validates_json_schema(&JsonValue::Str("c".into()), &enum_schema).await);
}

#[semio_framework_async_macros::async_test]
async fn ebnf_constraint_compiles_a_simple_recursive_ish_grammar_and_masks_correctly() {
    let grammar = "greeting ::= \"hi\" | \"hello\" ;";
    let limits = SamplingLimits::default();
    let mut constraint = EbnfConstraint::new(grammar, &limits).await.unwrap();
    let tokens: Vec<&[u8]> = vec![b"hi", b"hello", b"bye"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let vocab = Vocabulary::new(3).await;
    let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
    let mut mask = TokenBitset::new_full(3).await;
    constraint.fill_mask(&view, &mut mask).await.unwrap();
    assert!(mask.get(TokenId::new(0)).await);
    assert!(mask.get(TokenId::new(1)).await);
    assert!(!mask.get(TokenId::new(2)).await);
}

#[semio_framework_async_macros::async_test]
async fn ebnf_constraint_rejects_unbounded_left_recursion() {
    let grammar = "a ::= a \"x\" ;";
    let limits = SamplingLimits { max_grammar_bytes: 50, ..SamplingLimits::default() };
    assert!(EbnfConstraint::new(grammar, &limits).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn build_constraint_covers_every_constraint_spec_variant() {
    let limits = SamplingLimits::default();
    assert!(build_constraint(&ConstraintSpec::Regex("a".into()), &limits).await.is_ok());
    assert!(build_constraint(&ConstraintSpec::Trie(vec![vec![TokenId::new(0)]]), &limits).await.is_ok());
    assert!(build_constraint(&ConstraintSpec::MustInclude(vec![vec![TokenId::new(0)]]), &limits).await.is_ok());
    assert!(build_constraint(&ConstraintSpec::JsonMode, &limits).await.is_ok());
    assert!(build_constraint(&ConstraintSpec::Ebnf("a ::= \"x\" ;".into()), &limits).await.is_ok());
    assert!(build_constraint(&ConstraintSpec::JsonSchema(JsonValue::Object(Vec::new())), &limits).await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn sample_step_with_regex_constraint_only_ever_emits_matching_text() {
    let config = SamplingConfig { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, constraints: vec![ConstraintSpec::Regex("(a|b)".into())], max_tokens: 1, ..SamplingConfig::default() };
    let tokens: Vec<&[u8]> = vec![b"z", b"a", b"b"];
    let adapter: TokenTextAdapters = SliceTextAdapter::new(&tokens).await.into();
    let vocab = Vocabulary::new(3).await;
    let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, CounterRng::from_seed(0).await.into()).await.unwrap();
    let mut ws = LogitsWorkspace::new(3).await;
    let mut observer: SamplingObservers = NullObserver.into();
    // 🧱️ Token 0 ("z") has the highest raw logit but must be masked out by the regex constraint.
    let logits = [10.0f32, 1.0, 1.0];
    let result = sample_step(&config, &mut state, &mut ws, &vocab, Some(&adapter), &logits, &mut observer).await.unwrap();
    assert_ne!(result.token, TokenId::new(0));
}

#[semio_framework_async_macros::async_test]
async fn validates_json_schema_checks_integer_string_and_array_bounds() {
    let int_schema = parse_json(r#"{"type":"integer"}"#, 16).await.unwrap();
    assert!(validates_json_schema(&JsonValue::Num(3.0), &int_schema).await);
    assert!(!validates_json_schema(&JsonValue::Num(3.5), &int_schema).await);

    let str_schema = parse_json(r#"{"minLength":2,"maxLength":4}"#, 16).await.unwrap();
    assert!(validates_json_schema(&JsonValue::Str("abc".into()), &str_schema).await);
    assert!(!validates_json_schema(&JsonValue::Str("a".into()), &str_schema).await);
    assert!(!validates_json_schema(&JsonValue::Str("abcde".into()), &str_schema).await);

    let arr_schema = parse_json(r#"{"minItems":1,"maxItems":2,"items":{"type":"number"}}"#, 16).await.unwrap();
    assert!(validates_json_schema(&JsonValue::Array(vec![JsonValue::Num(1.0)]), &arr_schema).await);
    assert!(!validates_json_schema(&JsonValue::Array(Vec::new()), &arr_schema).await);
    assert!(!validates_json_schema(&JsonValue::Array(vec![JsonValue::Num(1.0), JsonValue::Num(2.0), JsonValue::Num(3.0)]), &arr_schema).await);
    assert!(!validates_json_schema(&JsonValue::Array(vec![JsonValue::Str("x".into())]), &arr_schema).await);
}

#[semio_framework_async_macros::async_test]
async fn validates_json_schema_checks_object_required_properties_and_enum() {
    let schema = parse_json(r#"{"type":"object","required":["a"],"properties":{"a":{"type":"number"}}}"#, 16).await.unwrap();
    let ok = JsonValue::Object(vec![("a".to_string(), JsonValue::Num(1.0))]);
    assert!(validates_json_schema(&ok, &schema).await);
    let missing = JsonValue::Object(Vec::new());
    assert!(!validates_json_schema(&missing, &schema).await);
    let wrong_type = JsonValue::Object(vec![("a".to_string(), JsonValue::Str("x".into()))]);
    assert!(!validates_json_schema(&wrong_type, &schema).await);

    let enum_schema = parse_json(r#"{"enum":[1,2]}"#, 16).await.unwrap();
    assert!(!validates_json_schema(&JsonValue::Num(3.0), &enum_schema).await);
}

#[semio_framework_async_macros::async_test]
async fn must_include_constraint_is_trivially_satisfied_with_no_alternatives_and_supports_rollback() {
    let vocab = Vocabulary::new(3).await;
    let view = step_view(&vocab, &[], &[]).await;
    let empty = MustIncludeConstraint::new(Vec::new()).await;
    assert!(empty.is_satisfied().await);
    assert!(!empty.is_finished().await);

    let mut constraint = MustIncludeConstraint::new(vec![vec![TokenId::new(1)]]).await;
    let mark = constraint.save().await;
    constraint.accept(&view, TokenId::new(1)).await.unwrap();
    assert!(constraint.is_satisfied().await);
    constraint.rollback_to(mark).await;
    assert!(!constraint.is_satisfied().await);
    constraint.reset().await;
    assert!(!constraint.is_satisfied().await);
    let forked = constraint.fork().await;
    assert!(!forked.is_satisfied().await);
}
// #endregion 🔖️ConstraintsTests

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
    let hypotheses = beam_search(&config, &beam_config, &vocab, None, initial, |state| if state.generated.len() < 2 { vec![0.0, 3.0, -5.0, -5.0] } else { vec![-5.0, -5.0, -5.0, 10.0] }).await.unwrap();
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
    let hypotheses = beam_search(&config, &beam_config, &vocab, None, initial, |_state| vec![1.0, 2.0, 3.0, 0.5]).await.unwrap();
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
        best_of.run(&config, &vocab, None, 2, &mut observer, async |i| SequenceState::new(SequenceId::new(i as u64), Vec::new(), &config, CounterRng::from_seed(i as u64).await.into()).await, |_state| vec![0.0, 9.0, 0.0, 1.0]).await.unwrap();
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
    let (results, metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &draft_tokens, &draft_distributions, |_state| vec![0.0, 9.0, 0.0, 0.0], &mut observer).await.unwrap();
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
    let (results, metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &draft_tokens, &draft_distributions, |_state| vec![-10.0, 10.0, -10.0, -10.0], &mut observer).await.unwrap();
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
        let (results, _metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &[draft_token], &draft_distributions, |_state| target.to_vec(), &mut observer).await.unwrap();
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

// #region 🔖️DiffusionTests
struct ConstantDenoiser {
    target: f32,
}
impl Denoiser for ConstantDenoiser {
    async fn prediction_type(&self) -> PredictionType {
        PredictionType::Sample
    }
    async fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, _branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
        for o in out.iter_mut() {
            *o = self.target;
        }
        Ok(())
    }
}

#[semio_framework_async_macros::async_test]
async fn all_noise_schedules_are_non_increasing() {
    let schedules = vec![
        NoiseSchedule::Linear { beta_start: 0.0001, beta_end: 0.02 },
        NoiseSchedule::ScaledLinear { beta_start: 0.0001, beta_end: 0.02 },
        NoiseSchedule::Cosine { s: 0.008 },
        NoiseSchedule::Karras { sigma_min: 0.01, sigma_max: 10.0, rho: 7.0 },
        NoiseSchedule::Exponential { sigma_min: 0.01, sigma_max: 10.0 },
        NoiseSchedule::Polynomial { sigma_min: 0.01, sigma_max: 10.0, power: 2.0 },
    ];
    for schedule in schedules {
        let sigmas = schedule.sigmas(10).await;
        assert_eq!(sigmas.len(), 10);
        for w in sigmas.windows(2) {
            assert!(w[0] >= w[1] - 1e-9, "{schedule:?} sigmas not non-increasing: {sigmas:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn custom_schedule_returns_its_values_verbatim() {
    let values = vec![5.0, 3.0, 1.0, 0.0];
    let schedule = NoiseSchedule::Custom(values.clone());
    assert_eq!(schedule.sigmas(4).await, values);
}

#[semio_framework_async_macros::async_test]
async fn euler_solver_converges_exactly_to_a_constant_target_when_schedule_reaches_zero() {
    // 📐️ With a constant (x-independent) `Sample` prediction, the Euler update at the final
    // step (sigma_next == 0) reduces algebraically to `x_next = denoised` exactly — a good
    // closed-form check that doesn't depend on any specific schedule shape.
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![10.0, 5.0, 2.0, 0.5, 0.0]), solver: Solver::Euler, steps: 4, guidance: None, seed: 0 };
    let mut latent = vec![0.0f32; 4];
    let mut denoiser = ConstantDenoiser { target: 5.0 };
    run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
    for &v in &latent {
        assert!((v - 5.0).abs() < 1e-3, "v={v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn ddim_eta_zero_is_deterministic_regardless_of_seed() {
    let run = async |seed: u64| {
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.0]), solver: Solver::Ddim { eta: 0.0 }, steps: 3, guidance: None, seed };
        let mut latent = vec![1.0f32; 4];
        let mut denoiser = ConstantDenoiser { target: 3.0 };
        run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
        latent
    };
    assert_eq!(run(1).await, run(999).await);
}

#[semio_framework_async_macros::async_test]
async fn euler_ancestral_and_ddim_with_eta_differ_across_seeds() {
    let run = async |seed: u64| {
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.2]), solver: Solver::EulerAncestral, steps: 3, guidance: None, seed };
        let mut latent = vec![1.0f32; 4];
        let mut denoiser = ConstantDenoiser { target: 3.0 };
        run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
        latent
    };
    assert_ne!(run(1).await, run(2).await, "ancestral sampling's injected noise should differ across seeds");
}

#[semio_framework_async_macros::async_test]
async fn guidance_combine_extrapolates_away_from_unconditional() {
    let guidance = Guidance { scale: 2.0, rescale: 0.0 };
    let cond = [1.0f32, 2.0];
    let uncond = [0.0f32, 0.0];
    let mut out = [0.0f32; 2];
    guidance.combine(&cond, &uncond, &mut out).await;
    assert_eq!(out, [2.0, 4.0]);
}

#[semio_framework_async_macros::async_test]
async fn run_diffusion_with_guidance_evaluates_both_branches_every_step() {
    struct BranchTrackingDenoiser {
        cond_calls: usize,
        uncond_calls: usize,
    }
    impl Denoiser for BranchTrackingDenoiser {
        async fn prediction_type(&self) -> PredictionType {
            PredictionType::Sample
        }
        async fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
            match branch {
                GuidanceBranch::Conditional => {
                    self.cond_calls += 1;
                    out.fill(1.0);
                }
                GuidanceBranch::Unconditional => {
                    self.uncond_calls += 1;
                    out.fill(0.0);
                }
            }
            Ok(())
        }
    }
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![2.0, 1.0, 0.0]), solver: Solver::Euler, steps: 2, guidance: Some(Guidance { scale: 1.5, rescale: 0.0 }), seed: 0 };
    let mut latent = vec![0.0f32; 2];
    let mut denoiser = BranchTrackingDenoiser { cond_calls: 0, uncond_calls: 0 };
    run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
    assert_eq!(denoiser.cond_calls, 2);
    assert_eq!(denoiser.uncond_calls, 2);
}

#[semio_framework_async_macros::async_test]
async fn heun_solver_uses_two_evaluations_per_non_final_step() {
    struct CountingDenoiser {
        calls: usize,
    }
    impl Denoiser for CountingDenoiser {
        async fn prediction_type(&self) -> PredictionType {
            PredictionType::Sample
        }
        async fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, _branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
            self.calls += 1;
            out.fill(2.0);
            Ok(())
        }
    }
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 0.0]), solver: Solver::Heun, steps: 2, guidance: None, seed: 0 };
    let mut latent = vec![0.0f32; 2];
    let mut denoiser = CountingDenoiser { calls: 0 };
    run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
    // step 0 (sigma_next=2.0, not final): 2 evaluations; step 1 (sigma_next=0.0, final): 1 evaluation.
    assert_eq!(denoiser.calls, 3);
}

#[semio_framework_async_macros::async_test]
async fn run_diffusion_cancellation_stops_the_run_and_errors() {
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.0]), solver: Solver::Euler, steps: 3, guidance: None, seed: 0 };
    let mut latent = vec![0.0f32; 2];
    let mut denoiser = ConstantDenoiser { target: 1.0 };
    let mut calls = 0;
    let result = run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| {
        calls += 1;
        if calls == 1 { StepControlFlow::Cancel } else { StepControlFlow::Continue }
    })
    .await;
    assert!(result.is_err());
    assert_eq!(calls, 1);
}

#[semio_framework_async_macros::async_test]
async fn img2img_start_index_boundaries() {
    assert_eq!(img2img_start_index(10, 1.0).await, 0);
    assert_eq!(img2img_start_index(10, 0.0).await, 10);
    assert_eq!(img2img_start_index(10, 0.5).await, 5);
}

#[semio_framework_async_macros::async_test]
async fn apply_inpaint_mask_blends_by_mask_weight() {
    let mut x = [10.0f32, 10.0, 10.0];
    let original = [0.0f32, 0.0, 0.0];
    let mask = [1.0f32, 0.0, 0.5];
    let mut rng: RandomSources = CounterRng::from_seed(1).await.into();
    // 🌫️ sigma = 0.0 means the re-noised original equals the original exactly, isolating the
    // blend-weight arithmetic from the injected-noise term.
    apply_inpaint_mask(&mut x, &original, &mask, 0.0, &mut rng).await;
    assert!((x[0] - 0.0).abs() < 1e-6, "fully masked (1.0) must fall back to the original");
    assert_eq!(x[1], 10.0, "unmasked (0.0) must stay untouched");
    assert!((x[2] - 5.0).abs() < 1e-6, "half-masked (0.5) must blend 50/50");
}

#[semio_framework_async_macros::async_test]
async fn prediction_type_conversions_round_trip_through_denoised() {
    let x = [2.0f32, -1.0];
    let sigma = 0.5;
    let target = [3.0f32, 3.0];

    let sample_raw = target;
    assert_eq!(to_denoised(PredictionType::Sample, &x, sigma, &sample_raw).await, target.to_vec());

    // 📐️ epsilon such that `x - sigma*eps == target` exactly.
    let eps: Vec<f32> = x.iter().zip(target).map(|(&xi, ti)| (xi - ti) / sigma as f32).collect();
    let denoised = to_denoised(PredictionType::Epsilon, &x, sigma, &eps).await;
    for (d, t) in denoised.iter().zip(target) {
        assert!((d - t).abs() < 1e-4);
    }
}
// #endregion 🔖️DiffusionTests
