use super::*;
use semio_framework_math_sampling_vocabulary::{SequenceId, SliceTextAdapter, StepIndex, Vocabulary};
use semio_framework_math_sampling_testing::{small_vocab, step_view};

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
