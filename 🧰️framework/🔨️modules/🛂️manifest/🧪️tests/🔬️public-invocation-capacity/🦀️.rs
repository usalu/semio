//! 📏️ The public invocation envelope's capacities, pinned against the language-neutral schema the
//! TypeScript mirror (`PUBLIC_INVOCATION_*` in `🛂️manifest/🟦️.ts`, used by the shell's
//! `publicInvocationStringPages`) reads too — neither side may drift from
//! `🎛️public-invocation/🧬️schema/🔣️.json`, and neither may drift from
//! `validate_public_json_envelope`, which is the runtime that enforces them.

use super::{public_invocation_char_cost, public_invocation_string_pages, PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_DEPTH, PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR, PUBLIC_INVOCATION_STRING_BYTES};

fn schema() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🎛️public-invocation/🧬️schema/🔣️.json")).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn capacities_match_the_neutral_schema() {
    let schema = schema();
    let properties = &schema["properties"];
    assert_eq!(properties["maxBodyBytes"]["const"].as_u64().unwrap() as usize, PUBLIC_INVOCATION_BODY_BYTES);
    assert_eq!(properties["maxStringBytes"]["const"].as_u64().unwrap() as usize, PUBLIC_INVOCATION_STRING_BYTES);
    assert_eq!(properties["maxDepth"]["const"].as_u64().unwrap() as usize, PUBLIC_INVOCATION_DEPTH);
    assert_eq!(properties["escapePairWireFactor"]["const"].as_u64().unwrap() as usize, PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR);
}

/// 📐️ The escape-pair factor is the WORST case, not an average: every JSON escape the encoder can
/// emit occupies at most twice what the envelope counts for it.
#[semio_framework_async_macros::async_test]
async fn the_escape_pair_factor_bounds_every_json_escape() {
    for (escape, counted) in [("\\\"", 1usize), ("\\\\", 1), ("\\n", 1), ("\\r", 1), ("\\t", 1), ("\\b", 1), ("\\f", 1), ("\\u0000", 5), ("\\u001f", 5)] {
        let occupied = escape.len();
        assert!(occupied <= counted * PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR, "escape {escape} occupies {occupied} wire bytes for {counted} counted");
    }
    assert!(PUBLIC_INVOCATION_STRING_BYTES * PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR < PUBLIC_INVOCATION_BODY_BYTES, "a single maximal string must still leave room for its own envelope");
}

/// ⚖️ LAW: the pager fills every page to the bound and never past it, splits only on character
/// boundaries, loses nothing and reorders nothing.
#[semio_framework_async_macros::async_test]
async fn the_pager_fills_every_page_without_exceeding_the_bound() {
    let payload: String = std::iter::repeat_n(r#"{"id":"brep.extrude","name":"Extrudieren ⛰️"},"#, 4_096).collect();
    let pages = public_invocation_string_pages(&payload);
    assert!(pages.len() > 1, "a payload larger than the bound must be cut into a run");
    for page in &pages {
        let cost: usize = page.chars().map(public_invocation_char_cost).sum();
        assert!(cost <= PUBLIC_INVOCATION_STRING_BYTES, "page cost {cost} exceeds {PUBLIC_INVOCATION_STRING_BYTES}");
    }
    for page in pages.iter().take(pages.len() - 1) {
        let cost: usize = page.chars().map(public_invocation_char_cost).sum();
        assert!(cost + 10 > PUBLIC_INVOCATION_STRING_BYTES, "page cost {cost} leaves room for another character and is therefore not filled");
    }
    assert_eq!(pages.concat(), payload, "a page run must reassemble into its exact payload");
}

/// ⚖️ LAW: an empty payload still produces one addressed page, so a producer never sends a run of
/// zero pages the guest would refuse.
#[semio_framework_async_macros::async_test]
async fn an_empty_payload_is_one_empty_page() {
    assert_eq!(public_invocation_string_pages(""), vec![String::new()]);
}

/// ⚖️ LAW: the cost function charges every character at least what the encoder can write for it, so
/// a page cut by it is admitted by `validate_public_json_envelope` under either escaping policy.
#[semio_framework_async_macros::async_test]
async fn the_cost_function_never_undercharges_a_character() {
    for character in ['"', '\\', '\n', '\u{1}', 'a', 'ä', '⛰', '🌀'] {
        let raw = character.len_utf8();
        let escaped = character.len_utf16() * 5;
        let cost = public_invocation_char_cost(character);
        assert!(cost >= raw.min(escaped) && cost <= escaped.max(1), "character {character:?} costs {cost} against raw {raw} / escaped {escaped}");
    }
    assert_eq!(public_invocation_char_cost('"'), 1);
    assert_eq!(public_invocation_char_cost('\u{1}'), 5);
    assert_eq!(public_invocation_char_cost('🌀'), 10);
}
