//! 🧪️ `📄set-snapshot` fixture — `rerates-to-48-khz-and-rewrites-the-right-channel`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🔊️ The case moves the clip from 44.1 kHz to 48 kHz and halves the right channel's three
//! samples, while the sample FORMAT, the left channel and the single `title` tag stay put — so
//! `SemioAudioDiff` must carry `sampleRate` plus an index-keyed `channels` triple touching only
//! index 1, with `format` and `tags` absent.
//!
//! Every committed sample is a dyadic fraction (0, ±0.5, ±0.25) so the `f32` values print
//! exactly through `serde_json` and the canonicality assertion is about serde shape, not float
//! formatting luck.

use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::SemioAudioDiff;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{apply_semio_audio_mutation, SemioAudioMutation};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioAudioSnapshot {
    serde_json::from_str(BEFORE).expect("before audio snapshot decodes")
}
fn expected_after() -> SemioAudioSnapshot {
    serde_json::from_str(AFTER).expect("after audio snapshot decodes")
}
fn mutation() -> SemioAudioMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the two-channel clip to exactly the committed `after`: 48 kHz and a
/// quieter right channel.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_audio_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-audio/set-snapshot: a genuinely changed clip must not raise any message");
    assert_eq!(snapshot.sample_rate, 48_000, "semio-audio/set-snapshot: the clip must be re-rated");
    assert_eq!(snapshot.format, SemioAudioFormat::Pcm16, "semio-audio/set-snapshot: re-rating must not change the sample format");
    assert_eq!(snapshot.channels[0], before().channels[0], "semio-audio/set-snapshot: the left channel must be carried over untouched");
    assert_eq!(snapshot, expected_after(), "semio-audio/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must return the clip to 44.1 kHz
/// and restore the louder right channel.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioAudioMutation as protocol::Mutation<SemioAudioSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_audio_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_audio_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-audio/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed clips and the mutation are already canonical: `SemioAudioFormat` is a plain
/// unit-variant enum written as `"pcm16"`, and each channel is an object with its own `samples`
/// array rather than a bare array of floats.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioAudioSnapshot = serde_json::from_str(text).expect("audio snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("audio snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("audio snapshot reparses");
        assert_eq!(reencoded, original, "semio-audio/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-audio/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the clip really moves, so the `mutation.no-op` warning
/// an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-audio/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_audio_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-audio/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-audio/set-snapshot: an applied set-snapshot must actually move the clip");
}

/// 🔺️ The sparse `SemioAudioDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: the unchanged left channel must not appear in `channels.modified`, and
/// `format`/`tags` must stay absent rather than being re-stated by a whole-snapshot set.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioAudioMutation as protocol::Mutation<SemioAudioSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced audio diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed audio diff decodes");
    assert_eq!(produced, committed, "semio-audio/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioAudioDiff`: a new sample rate
/// plus a one-entry channel patch, whose payload is a whole `samples` vector (channels are not
/// sample-by-sample diffed).
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioAudioDiff = serde_json::from_str(DIFF).expect("committed audio diff decodes");
    assert!(decoded.format.is_none() && decoded.tags.is_none(), "semio-audio/set-snapshot: neither the sample format nor the tag list may be re-emitted");
    let channels = decoded.channels.as_ref().expect("the committed diff carries a channels triple");
    assert!(channels.removed.is_empty() && channels.added.is_empty() && channels.modified.len() == 1 && channels.modified[0].index == 1, "semio-audio/set-snapshot: exactly the right channel may be patched in place");
    assert!(channels.modified[0].diff.samples.is_some(), "semio-audio/set-snapshot: a channel patch replaces its whole samples vector");
    let reencoded = serde_json::to_value(&decoded).expect("audio diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed audio diff reparses");
    assert_eq!(reencoded, original, "semio-audio/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the rate
/// plus one channel is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioAudioDiff = serde_json::from_str(DIFF).expect("committed audio diff decodes");
    let produced = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&decoded, &before()).expect("committed audio diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-audio/set-snapshot: committed diff did not carry before to after");
}
