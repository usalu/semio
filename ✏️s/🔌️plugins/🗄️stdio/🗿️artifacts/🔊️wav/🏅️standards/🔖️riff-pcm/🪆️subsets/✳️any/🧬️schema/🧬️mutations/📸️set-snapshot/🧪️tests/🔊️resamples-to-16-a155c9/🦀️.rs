//! 🧪️ `📸️set-snapshot` fixture — `🔊️resamples-to-16-khz-and-doubles-the-pcm16-amplitude`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🔊️ The case re-rates the `fmt ` chunk (8 kHz/16000 B/s → 16 kHz/32000 B/s) and doubles the four
//! `Pcm16` samples in the `data` chunk, while the verbatim-retained `fact` RIFF chunk is left
//! alone — so `WavDiff::between` must emit a whole-value `fmt` plus a whole-value `data` and must
//! leave `other_chunks` absent.

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::WavDiff;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::{apply_wav_mutation, WavMutation};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{WavData, WavSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> WavSnapshot {
    serde_json::from_str(BEFORE).expect("before WAV snapshot decodes")
}
fn expected_after() -> WavSnapshot {
    serde_json::from_str(AFTER).expect("after WAV snapshot decodes")
}
fn mutation() -> WavMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the one-channel clip to exactly the committed `after`: 16 kHz `fmt `
/// fields and the louder `Pcm16` payload.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_wav_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "wav/set-snapshot: a genuinely changed clip must not raise any message");
    assert_eq!((snapshot.fmt.sample_rate, snapshot.fmt.byte_rate), (16_000, 32_000), "wav/set-snapshot: the fmt chunk must carry the new sample and byte rate together");
    assert_eq!(snapshot.data, WavData::Pcm16(vec![0, 2000, -2000, 0]), "wav/set-snapshot: the data chunk must keep its Pcm16 typing and take the doubled samples");
    assert_eq!(snapshot, expected_after(), "wav/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must put the 8 kHz `fmt ` fields
/// and the quieter samples back, `fact` chunk included.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <WavMutation as protocol::Mutation<WavSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_wav_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_wav_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "wav/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed WAV snapshots and the mutation are already canonical: `WavFmt` omits its
/// absent extensible `ext` tail, and `WavData` is ADJACENTLY tagged (`kind` + `value`) because
/// serde cannot internally tag a newtype variant wrapping a `Vec`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WavSnapshot = serde_json::from_str(text).expect("WAV snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("WAV snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("WAV snapshot reparses");
        assert_eq!(reencoded, original, "wav/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "wav/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the clip really moves, so the `mutation.no-op` warning
/// an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "wav/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_wav_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "wav/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "wav/set-snapshot: an applied set-snapshot must actually move the clip");
}

/// 🔺️ The sparse `WavDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: `fmt` and `data` are whole-value LWW slots, and `other_chunks` must stay absent
/// rather than being re-listed just because the snapshot was replaced wholesale.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <WavMutation as protocol::Mutation<WavSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced WAV diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed WAV diff decodes");
    assert_eq!(produced, committed, "wav/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `WavDiff` with `other_chunks` absent.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WavDiff = serde_json::from_str(DIFF).expect("committed WAV diff decodes");
    assert!(decoded.other_chunks.is_none(), "wav/set-snapshot: the committed diff must leave the verbatim RIFF chunk list untouched");
    assert!(matches!(decoded.data, Some(WavData::Pcm16(_))), "wav/set-snapshot: the committed diff must keep the data chunk typed as Pcm16");
    let reencoded = serde_json::to_value(&decoded).expect("WAV diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed WAV diff reparses");
    assert_eq!(reencoded, original, "wav/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the delta is
/// a complete description of the resample, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WavDiff = serde_json::from_str(DIFF).expect("committed WAV diff decodes");
    let produced = <WavDiff as protocol::MutationDiff<WavSnapshot>>::apply(&decoded, &before()).expect("committed WAV diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "wav/set-snapshot: committed diff did not carry before to after");
}
