//! 🧪️ `📸️set-snapshot` fixture — `🔃️reverses-the-slide-order`.
//!
//! `set-snapshot` means "the document becomes this snapshot", so a replacement deck whose slides are
//! the base's own two slides in REVERSE order must land each slide's identity at its new index. The
//! slide collection is index-keyed, so the diff carries every changed identity in `SlideDiff::id`
//! next to the content that moved with it; without it the base's identifiers stayed behind the new
//! content (slide 0 carried `s-2`'s agenda under `s-1`'s id). Source of truth is the committed JSON
//! quintet beside this file.

use crate::standards::v1::subsets::presentation::schema::diff::SemioPresentationDiff;
use crate::standards::v1::subsets::presentation::schema::mutations::{apply_semio_presentation_mutation, SemioPresentationMutation};
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔃️reverses/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔃️reverses/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔃️reverses/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔃️reverses/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔃️reverses/🎯️outcome/🔣️.json");

fn before() -> SemioPresentationSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before presentation snapshot decodes")
}
fn expected_after() -> SemioPresentationSnapshot {
    dsl::json::from_json_str(AFTER).expect("after presentation snapshot decodes")
}
fn mutation() -> SemioPresentationMutation {
    dsl::json::from_json_str(MUTATION).expect("set-snapshot mutation decodes")
}
fn slide_ids(snapshot: &SemioPresentationSnapshot) -> Vec<&str> {
    snapshot.slides.iter().map(|slide| slide.id.as_str()).collect()
}

/// ▶️ The deck becomes exactly the replacement: both slides swap places WITH their identities.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_presentation_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-presentation/set-snapshot: a reordered deck must not raise any message");
    assert_eq!(slide_ids(&snapshot), ["s-2", "s-1"], "semio-presentation/set-snapshot: every slide identity must travel to its new index");
    assert_eq!(snapshot, expected_after(), "semio-presentation/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse (`set-snapshot(base)`) restores the original order and identities.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_presentation_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_presentation_mutation(&mut snapshot, step);
    }
    assert_eq!(slide_ids(&snapshot), ["s-1", "s-2"], "semio-presentation/set-snapshot: the inverse must restore the original slide order");
    assert_eq!(snapshot, base, "semio-presentation/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔺️ The produced sparse diff is exactly the committed one: both slots carry their new `id`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced presentation diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed presentation diff decodes");
    assert_eq!(produced, committed, "semio-presentation/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff alone carries `before` to `after`, and the declared outcome is `applied`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioPresentationDiff = dsl::json::from_json_str(DIFF).expect("committed presentation diff decodes");
    let produced = <SemioPresentationDiff as protocol::MutationDiff<SemioPresentationSnapshot>>::apply(&decoded, &before()).expect("committed presentation diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-presentation/set-snapshot: committed diff did not carry before to after");
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "semio-presentation/set-snapshot: this fixture declares an applied outcome");
}

/// 🔤️ The identity slot survives the diff codec: `slide-diff` leads with its `option-hex` id.
#[semio_framework_async_macros::async_test]
async fn diff_codec_round_trips_the_identity_slot() {
    let decoded: SemioPresentationDiff = dsl::json::from_json_str(DIFF).expect("committed presentation diff decodes");
    let bytes = <SemioPresentationDiff as protocol::DiffCodec>::encode_diff(&decoded).expect("presentation diff encodes");
    let reread = <SemioPresentationDiff as protocol::DiffCodec>::decode_diff(&bytes).expect("presentation diff decodes");
    assert_eq!(reread, decoded, "semio-presentation/set-snapshot: the slide identity slot did not survive the diff codec");
}
