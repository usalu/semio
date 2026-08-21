//! 🧪️ `📄set-snapshot` fixture — `rewrites-the-second-slides-textbox-and-adds-a-speaker-note`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎞️ The case rewrites the second slide's text box and gives that slide its first speaker note.
//! The master, the layout, the first slide and the second slide's own frame and `layoutId` are all
//! unchanged — so `SemioPresentationDiff` must fill its index-keyed `slides` slot alone, and the
//! shape patch must stay a `TextBox` patch with `frame` unset (never the `Replace` a shape-KIND
//! change would produce).
//!
//! 🪆️ `SlideDiff::layout_id` is a tri-state `Option<Option<String>>`; "detach this slide from its
//! layout" is `Some(None)`, which serde writes as bare `null` and reads back as `None`
//! (= unchanged), so no committed fixture may express it. This case keeps the layout stable.

use crate::artifacts::semio::standards::v1::subsets::presentation::schema::diff::{SemioPresentationDiff, SlideShapeDiff};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::{apply_semio_presentation_mutation, SemioPresentationMutation};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioPresentationSnapshot {
    serde_json::from_str(BEFORE).expect("before presentation snapshot decodes")
}
fn expected_after() -> SemioPresentationSnapshot {
    serde_json::from_str(AFTER).expect("after presentation snapshot decodes")
}
fn mutation() -> SemioPresentationMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the deck to exactly the committed `after`: a final agenda slide with
/// one speaker note.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_presentation_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-presentation/set-snapshot: a genuinely changed deck must not raise any message");
    assert_eq!(snapshot.slides[1].id, "s-2", "semio-presentation/set-snapshot: a slide keeps its persistent id even though the collection is index-addressed");
    assert_eq!(snapshot.slides[1].notes.len(), 1, "semio-presentation/set-snapshot: the speaker note must be added");
    assert_eq!(snapshot.slides[0], before().slides[0], "semio-presentation/set-snapshot: the title slide must be carried over untouched");
    assert_eq!((&snapshot.masters, &snapshot.layouts), (&before().masters, &before().layouts), "semio-presentation/set-snapshot: editing a slide must not touch the master or layout tables");
    assert_eq!(snapshot, expected_after(), "semio-presentation/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore the draft agenda and
/// remove the speaker note again.
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
    assert!(snapshot.slides[1].notes.is_empty(), "semio-presentation/set-snapshot: the inverse must remove the added speaker note");
    assert_eq!(snapshot, base, "semio-presentation/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed decks and the mutation are already canonical. The presentation-specific trap
/// pinned here: `SlideShape` is tagged `shapeKind`, NOT `kind`, because the `Placeholder` variant
/// carries its own `kind` field and an internally-tagged enum's tag must not collide with a
/// variant's field name. The reused `DocBlock` keeps its own `kind` tag and its snake_case
/// `style_id` variant field.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioPresentationSnapshot = serde_json::from_str(text).expect("presentation snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("presentation snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("presentation snapshot reparses");
        assert_eq!(reencoded, original, "semio-presentation/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-presentation/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the deck really moves, so the `mutation.no-op` warning
/// an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-presentation/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_presentation_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-presentation/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-presentation/set-snapshot: an applied set-snapshot must actually move the deck");
}

/// 🔺️ The sparse `SemioPresentationDiff` this mutation produces is exactly the committed diff —
/// the load-bearing assertion: `masters` and `layouts` must stay absent, slide 0 must not appear in
/// `slides.modified`, and the slide patch must leave its tri-state `layoutId` unset.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced presentation diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed presentation diff decodes");
    assert_eq!(produced, committed, "semio-presentation/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioPresentationDiff`: one slide
/// patched in place, its text box patched as a `TextBox` (frame unset), its notes purely appended.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioPresentationDiff = serde_json::from_str(DIFF).expect("committed presentation diff decodes");
    assert!(decoded.masters.is_none() && decoded.layouts.is_none(), "semio-presentation/set-snapshot: neither the master nor the layout table may be re-emitted");
    let slides = decoded.slides.as_ref().expect("the committed diff carries a slides triple");
    assert!(slides.removed.is_empty() && slides.added.is_empty() && slides.modified.len() == 1 && slides.modified[0].index == 1, "semio-presentation/set-snapshot: exactly the second slide may be patched in place");
    let slide = &slides.modified[0].diff;
    assert!(slide.layout_id.is_none(), "semio-presentation/set-snapshot: the tri-state layout reference must stay absent, not a round-trip-lossy null");
    let shapes = slide.shapes.as_ref().expect("the patched slide carries a shapes triple");
    let SlideShapeDiff::TextBox { frame, blocks } = &shapes.modified[0].diff else {
        panic!("semio-presentation/set-snapshot: the shape delta must stay a TextBox patch, never the Replace a shape-kind change would produce");
    };
    assert!(frame.is_none(), "semio-presentation/set-snapshot: the text box did not move or resize, so its frame must stay absent");
    assert!(blocks.as_ref().is_some_and(|blocks| blocks.modified.len() == 1 && blocks.added.is_empty() && blocks.removed.is_empty()), "semio-presentation/set-snapshot: the single body block is replaced whole, never removed and re-added");
    let notes = slide.notes.as_ref().expect("the patched slide carries a notes triple");
    assert!(notes.removed.is_empty() && notes.modified.is_empty() && notes.added.len() == 1 && notes.added[0].index == 0, "semio-presentation/set-snapshot: the speaker note must arrive as a pure append at final position 0");
    let reencoded = serde_json::to_value(&decoded).expect("presentation diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed presentation diff reparses");
    assert_eq!(reencoded, original, "semio-presentation/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the body
/// block plus the appended note is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioPresentationDiff = serde_json::from_str(DIFF).expect("committed presentation diff decodes");
    let produced = <SemioPresentationDiff as protocol::MutationDiff<SemioPresentationSnapshot>>::apply(&decoded, &before()).expect("committed presentation diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-presentation/set-snapshot: committed diff did not carry before to after");
}
