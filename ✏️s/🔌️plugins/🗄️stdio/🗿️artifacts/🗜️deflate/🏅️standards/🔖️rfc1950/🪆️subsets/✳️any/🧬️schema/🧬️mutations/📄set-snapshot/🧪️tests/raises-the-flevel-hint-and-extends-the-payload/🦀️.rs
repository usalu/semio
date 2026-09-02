//! 🧪️ `📄set-snapshot` fixture — `raises-the-flevel-hint-and-extends-the-payload`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎚️ The case raises RFC1950 FLG.FLEVEL from `Default` to `Maximum` and appends one byte to the
//! decompressed payload, while CMF's CM/CINFO nibbles stay at deflate/`2^15` — so `DeflateDiff`
//! must emit exactly `compressionLevelHint` and `payload`.
//!
//! 🪆️ Deliberately NOT exercised here: `DeflateDiff::dict_id` is a tri-state
//! `Option<Option<u32>>`. A "preset dictionary cleared" delta is `Some(None)`, which serde writes
//! as bare `null` and reads back as `None` (= unchanged) — that shape cannot survive a JSON round
//! trip, so no committed fixture may express it. This fixture keeps FDICT clear on both sides so
//! `dict_id` stays `None` and is omitted from the diff entirely.

use crate::artifacts::deflate::schema::diff::DeflateDiff;
use crate::artifacts::deflate::schema::mutations::{apply_deflate_mutation, DeflateMutation};
use crate::artifacts::deflate::schema::snapshot::{DeflateLevelHint, DeflateSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> DeflateSnapshot {
    serde_json::from_str(BEFORE).expect("before zlib snapshot decodes")
}
fn expected_after() -> DeflateSnapshot {
    serde_json::from_str(AFTER).expect("after zlib snapshot decodes")
}
fn mutation() -> DeflateMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the zlib stream to exactly the committed `after`: FLEVEL `Maximum`
/// and a three-byte payload.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_deflate_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "deflate/set-snapshot: a genuinely changed stream must not raise any message");
    assert_eq!(snapshot.compression_level_hint, DeflateLevelHint::Maximum, "deflate/set-snapshot: FLG.FLEVEL must be raised");
    assert_eq!(snapshot.compression_method, 8, "deflate/set-snapshot: CMF's CM nibble must stay the RFC1950 deflate method");
    assert_eq!(snapshot.dict_id, None, "deflate/set-snapshot: FDICT must stay clear on both sides of this fixture");
    assert_eq!(snapshot, expected_after(), "deflate/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must drop FLEVEL back to
/// `Default` and truncate the payload again.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <DeflateMutation as protocol::Mutation<DeflateSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_deflate_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_deflate_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "deflate/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed zlib snapshots and the mutation are already canonical: `dict_id` is skipped
/// entirely while FDICT is clear, `DeflateLevelHint` is a unit-variant enum written as
/// `"default"`/`"maximum"`, and `payload` is a plain array of byte numbers even though the Rust
/// field carries a `#[dsl(base64)]` hint for the SEPARATE `ArtifactDsl` grammar.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DeflateSnapshot = serde_json::from_str(text).expect("zlib snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("zlib snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("zlib snapshot reparses");
        assert_eq!(reencoded, original, "deflate/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "deflate/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the stream really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "deflate/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_deflate_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "deflate/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "deflate/set-snapshot: an applied set-snapshot must actually move the stream");
}

/// 🔺️ The sparse `DeflateDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: the two CMF nibbles must stay absent, and the tri-state `dict_id` slot
/// must stay absent rather than being written as an unchanged-looking `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <DeflateMutation as protocol::Mutation<DeflateSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced zlib diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed zlib diff decodes");
    assert_eq!(produced, committed, "deflate/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `DeflateDiff` with `dict_id` absent —
/// the one shape this artifact's tri-state field can round-trip.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: DeflateDiff = serde_json::from_str(DIFF).expect("committed zlib diff decodes");
    assert!(decoded.compression_method.is_none() && decoded.window_bits.is_none(), "deflate/set-snapshot: the CMF header nibbles must not be re-emitted");
    assert!(decoded.dict_id.is_none(), "deflate/set-snapshot: the tri-state dict_id slot must be absent, not a round-trip-lossy null");
    assert_eq!(decoded.compression_level_hint, Some(DeflateLevelHint::Maximum), "deflate/set-snapshot: the committed diff must raise FLEVEL");
    let reencoded = serde_json::to_value(&decoded).expect("zlib diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed zlib diff reparses");
    assert_eq!(reencoded, original, "deflate/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the hint
/// plus payload is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: DeflateDiff = serde_json::from_str(DIFF).expect("committed zlib diff decodes");
    let produced = <DeflateDiff as protocol::MutationDiff<DeflateSnapshot>>::apply(&decoded, &before()).expect("committed zlib diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "deflate/set-snapshot: committed diff did not carry before to after");
}
