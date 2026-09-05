//! 🧪️ `📸️set-snapshot` fixture — `⏱️retimes-the-track-and-promotes-a-sample-to-a-keyframe`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎞️ The case retimes the h264 stream from 24/1 to 25/1 and promotes its second sample to a
//! keyframe with a fresh opaque payload. The stream's `kind`/`codec`/`width`/`height`, its first
//! sample, and the whole second (subtitle) stream are untouched — so `SemioVideoDiff` must nest two
//! index-keyed triples (`streams.modified[0] → samples.modified[1]`) and set nothing else.

use crate::artifacts::semio::standards::v1::subsets::video::schema::diff::SemioVideoDiff;
use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::{apply_semio_video_mutation, SemioVideoMutation};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioVideoSnapshot {
    serde_json::from_str(BEFORE).expect("before video snapshot decodes")
}
fn expected_after() -> SemioVideoSnapshot {
    serde_json::from_str(AFTER).expect("after video snapshot decodes")
}
fn mutation() -> SemioVideoMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the container to exactly the committed `after`: a 25/1 video track
/// whose second sample is a sync point.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_video_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-video/set-snapshot: a genuinely changed container must not raise any message");
    assert_eq!(snapshot.streams[0].rate, SemioRational { num: 25, den: 1 }, "semio-video/set-snapshot: the frame rate must be retimed as an exact fraction");
    assert!(snapshot.streams[0].samples[1].key, "semio-video/set-snapshot: the second sample must become a keyframe");
    assert_eq!(snapshot.streams[1], before().streams[1], "semio-video/set-snapshot: the subtitle stream must be carried over untouched");
    assert_eq!(snapshot, expected_after(), "semio-video/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore 24/1 and demote the
/// sample back to a delta frame with its original payload.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioVideoMutation as protocol::Mutation<SemioVideoSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_video_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_video_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-video/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed containers and the mutation are already canonical: `SemioRational` is a named
/// two-field struct (never a bare tuple), `SemioVideoStreamKind` is a unit-variant enum written as
/// `"video"`/`"subtitle"`, and a sample's opaque `data` is a plain array of byte numbers.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioVideoSnapshot = serde_json::from_str(text).expect("video snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("video snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("video snapshot reparses");
        assert_eq!(reencoded, original, "semio-video/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-video/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the container really moves, so the `mutation.no-op`
/// warning an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-video/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_video_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-video/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-video/set-snapshot: an applied set-snapshot must actually move the container");
}

/// 🔺️ The sparse `SemioVideoDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: the subtitle stream must not appear at all, sample 0 must not appear in
/// `samples.modified`, and `codec`/`width`/`height`/`kind` must stay absent on the patched stream.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioVideoMutation as protocol::Mutation<SemioVideoSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced video diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed video diff decodes");
    assert_eq!(produced, committed, "semio-video/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioVideoDiff`: one patched stream
/// carrying one patched sample, no removals and no additions at either level.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioVideoDiff = serde_json::from_str(DIFF).expect("committed video diff decodes");
    let streams = decoded.streams.as_ref().expect("the committed diff carries a streams triple");
    assert!(streams.removed.is_empty() && streams.added.is_empty() && streams.modified.len() == 1 && streams.modified[0].index == 0, "semio-video/set-snapshot: exactly the first stream may be patched in place");
    let stream = &streams.modified[0].diff;
    assert!(stream.kind.is_none() && stream.codec.is_none() && stream.width.is_none() && stream.height.is_none(), "semio-video/set-snapshot: retiming must not restate the stream's identity fields");
    let samples = stream.samples.as_ref().expect("the patched stream carries a samples triple");
    assert!(samples.removed.is_empty() && samples.added.is_empty() && samples.modified.len() == 1 && samples.modified[0].index == 1, "semio-video/set-snapshot: exactly the second sample may be patched in place");
    assert!(samples.modified[0].diff.pts.is_none(), "semio-video/set-snapshot: the sample's presentation timestamp did not move and must stay absent");
    let reencoded = serde_json::to_value(&decoded).expect("video diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed video diff reparses");
    assert_eq!(reencoded, original, "semio-video/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the nested
/// stream + sample delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioVideoDiff = serde_json::from_str(DIFF).expect("committed video diff decodes");
    let produced = <SemioVideoDiff as protocol::MutationDiff<SemioVideoSnapshot>>::apply(&decoded, &before()).expect("committed video diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-video/set-snapshot: committed diff did not carry before to after");
}
