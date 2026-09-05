//! 🧪️ `📸️set-snapshot` fixture — `🔑️promotes-the-second-movi-chunk-to-a-keyframe`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎞️ The case rewrites the payload of the second `00dc` chunk of the single `vids` stream and
//! flips its `idx1`-derived keyframe flag, leaving `avih`, the stream's `strh`/`strf` and the
//! retained `JUNK` chunk alone — so `AviDiff::between` must nest two index-keyed triples
//! (`streams.modified[0] → chunks.modified[1]`) and nothing else.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::AviDiff;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::{apply_avi_mutation, AviMutation};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> AviSnapshot {
    serde_json::from_str(BEFORE).expect("before AVI snapshot decodes")
}
fn expected_after() -> AviSnapshot {
    serde_json::from_str(AFTER).expect("after AVI snapshot decodes")
}
fn mutation() -> AviMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the two-frame clip to exactly the committed `after`: the second
/// `movi` chunk is repainted and becomes a sync point.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_avi_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "avi/set-snapshot: a genuinely changed clip must not raise any message");
    let chunk = &snapshot.streams[0].chunks[1];
    assert_eq!(chunk.fourcc, "00dc", "avi/set-snapshot: the chunk identity fourcc is never rewritten by a chunk patch");
    assert!(chunk.keyframe, "avi/set-snapshot: the second movi chunk must become a keyframe");
    assert_eq!(snapshot, expected_after(), "avi/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must demote the second chunk back
/// to a delta frame and restore its original payload.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <AviMutation as protocol::Mutation<AviSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_avi_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_avi_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "avi/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed AVI snapshots and the mutation are already canonical. Two AVI-specific traps
/// are pinned here: `AviStreamFormat` is internally tagged on `format` (`bitmapInfo`) and its
/// container-level `rename_all` renames only the VARIANT — the `BITMAPINFOHEADER` fields therefore
/// stay snake_case on the wire (`bit_count`, `size_image`, `x_pels_per_meter`, …) even though the
/// surrounding `AviStreamHeader` is camelCase; and every payload — `avih`'s `dwReserved[4]`
/// included — is a plain array of numbers, never base64.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: AviSnapshot = serde_json::from_str(text).expect("AVI snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("AVI snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("AVI snapshot reparses");
        assert_eq!(reencoded, original, "avi/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "avi/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the clip really moves, so the `mutation.no-op` warning
/// an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "avi/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_avi_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "avi/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "avi/set-snapshot: an applied set-snapshot must actually move the clip");
}

/// 🔺️ The sparse `AviDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: the whole-value `main_header`, the `idx1_present` flag and the `unknown_chunks`
/// triple must all stay absent, and the stream must be PATCHED at index 0 rather than removed and
/// re-added.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <AviMutation as protocol::Mutation<AviSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced AVI diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed AVI diff decodes");
    assert_eq!(produced, committed, "avi/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `AviDiff`: one modified stream, no
/// removals or additions at either nesting level.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: AviDiff = serde_json::from_str(DIFF).expect("committed AVI diff decodes");
    assert!(decoded.main_header.is_none() && decoded.idx1_present.is_none() && decoded.unknown_chunks.is_none() && decoded.hdrl_extra.is_none(), "avi/set-snapshot: the committed diff must touch nothing but the streams triple");
    let streams = decoded.streams.as_ref().expect("the committed diff carries a streams triple");
    assert!(streams.removed.is_empty() && streams.added.is_empty() && streams.modified.len() == 1, "avi/set-snapshot: the single stream must be patched in place, never removed and re-added");
    assert!(streams.modified[0].diff.strl_extra.is_none(), "avi/set-snapshot: this fixture never touches strl_extra");
    let reencoded = serde_json::to_value(&decoded).expect("AVI diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed AVI diff reparses");
    assert_eq!(reencoded, original, "avi/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the nested
/// chunk delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: AviDiff = serde_json::from_str(DIFF).expect("committed AVI diff decodes");
    let produced = <AviDiff as protocol::MutationDiff<AviSnapshot>>::apply(&decoded, &before()).expect("committed AVI diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "avi/set-snapshot: committed diff did not carry before to after");
}
