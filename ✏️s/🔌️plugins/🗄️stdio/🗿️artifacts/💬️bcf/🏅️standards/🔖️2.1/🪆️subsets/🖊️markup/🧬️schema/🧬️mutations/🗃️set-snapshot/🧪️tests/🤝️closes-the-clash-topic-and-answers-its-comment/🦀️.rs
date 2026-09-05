//! 🧪️ `set-snapshot` fixture — `🤝️closes-the-clash-topic-and-answers-its-comment`.
//!
//! BCF's whole tree is guid-keyed, twice over: `BcfDiff::topics` is a `NamedTripleDiff` keyed
//! by topic guid, and the modified topic's own `comments` is a second `NamedTripleDiff` keyed
//! by comment guid. So editing a topic's status AND one of its comments must produce a single
//! nested `key`/`diff` chain — never a re-stated topic, and never anything under `parts`,
//! which is this artifact's verbatim raw-retention lane for files the typed model doesn't
//! cover (`project.bcfp` and friends).
//! `BcfCommentDiff::viewpointRef` is a tri-state `Option<Option<String>>` whose `Some(None)`
//! 'reference cleared' state cannot survive a JSON round trip; this payload never clears it.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::diff::BcfDiff;
use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::mutations::{apply_bcf_mutation, BcfMutation};
use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> BcfSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> BcfSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> BcfMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` BcfSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_bcf_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.topics[0].status, "Closed", "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the topic's TopicStatus attribute must flip to Closed");
    assert_eq!(snapshot.topics[0].comments[0].text, "Rerouted the duct above the beam.", "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the comment body must be replaced");
    assert_eq!(snapshot.topics[0].title, before().topics[0].title, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the topic title is equal on both sides and must survive untouched");
    assert_eq!(snapshot.topics[0].comments[0].author, "ueli", "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the comment's author is not touched by an edit to its text");
    assert_eq!(snapshot.version, "2.1", "set-snapshot/closes-the-clash-topic-and-answers-its-comment: bcf.version's VersionId does not move");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state BcfSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <BcfMutation as protocol::Mutation<BcfSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], BcfMutation::SetSnapshot(_)), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_bcf_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_bcf_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed BcfSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: BcfSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <BcfMutation as protocol::Mutation<BcfSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_bcf_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/closes-the-clash-topic-and-answers-its-comment: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in BcfDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <BcfMutation as protocol::Mutation<BcfSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().version.is_none(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the container version is equal on both sides");
    assert!(raised.diff().parts.is_none(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the verbatim raw-retention lane is empty on both sides and must not produce a triple");
    let topics = raised.diff().topics.as_ref().expect("set-snapshot/closes-the-clash-topic-and-answers-its-comment: the topics triple must be present");
    assert!(topics.removed.is_empty() && topics.added.is_empty(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the topic is patched in place, keyed by its guid");
    let topic = &topics.modified[0].diff;
    assert_eq!(topic.status.as_deref(), Some("Closed"), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: status is the one topic scalar this payload moves");
    assert!(topic.title.is_none() && topic.description.is_none() && topic.labels.is_none(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the topic's other scalars and its whole-value labels list must stay absent");
    assert!(topic.viewpoints.is_none(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the topic has no viewpoints on either side");
    let comments = topic.comments.as_ref().expect("set-snapshot/closes-the-clash-topic-and-answers-its-comment: the comments triple must be present");
    assert_eq!(comments.modified[0].diff.text.as_deref(), Some("Rerouted the duct above the beam."), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: the nested comment patch names text and is keyed by the comment's own guid");
}

/// 🔣️ The committed diff is itself canonical and decodes to BcfDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: BcfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/closes-the-clash-topic-and-answers-its-comment: committed diff JSON is not canonical");
    assert!(
        decoded.topics.as_ref().expect("topics triple").modified[0].diff.comments.as_ref().expect("comments triple").modified[0].diff.viewpoint_ref.is_none(),
        "set-snapshot/closes-the-clash-topic-and-answers-its-comment: viewpointRef must round-trip as absent — a committed null would collapse the Some(None) 'reference cleared' state that Option<Option<String>> cannot express in JSON"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: BcfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <BcfDiff as protocol::MutationDiff<BcfSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/closes-the-clash-topic-and-answers-its-comment: committed diff did not carry before to after");
}
