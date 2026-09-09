use super::*;
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun};
use protocol::{Mutation, MutationDiff, SemanticMutation};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioTextSnapshot {
    SemioTextSnapshot {
        runs: vec![SemioTextRun { language: "en".into(), content: "hello".into(), marks: vec![] }, SemioTextRun { language: "en".into(), content: "world".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }] }],
        ..Default::default()
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(base: &SemioTextSnapshot, operation: &SemioTextMutation) -> SemioTextSnapshot {
    let forward = operation.diff(base).diff().apply(base).expect("apply must succeed for a well-formed fixture");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    // 🔧️ Each inverse's diff must be computed against the CURRENT (`restored`) state, not the
    // stale pre-operation `base` — `text`'s whole-list-replace diff shape (📓️taxonomy.md's
    // `SemioTextRunList` wrapper) reconstructs the entire `runs` vec from whatever base it is
    // given, so calling it against the wrong base silently discards the forward mutation's
    // effect instead of undoing it. Same standard `mutation.diff(&current); current =
    // diff.diff().apply(&current)` threading `apply_semio_image_mutation`/`apply_semio_mutation`
    // establish elsewhere in this standard.
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

#[semio_framework_async_macros::async_test]
async fn insert_remove_run_round_trips() {
    let base = fixture();
    let new_run = SemioTextRun { language: "de".into(), content: "hallo".into(), marks: vec![] };

    let insert = SemioTextMutation::InsertRun(insert_run::InsertRun { index: 1, run: new_run.clone() });
    let after_insert = round_trip(&base, &insert);
    assert_eq!(after_insert.runs.len(), base.runs.len() + 1);
    assert_eq!(after_insert.runs[1], new_run);

    let undo = insert.inverse(&base);
    assert_eq!(undo, vec![SemioTextMutation::RemoveRun(remove_run::RemoveRun { index: 1 })]);

    let remove = SemioTextMutation::RemoveRun(remove_run::RemoveRun { index: 0 });
    let after_remove = round_trip(&base, &remove);
    assert_eq!(after_remove.runs.len(), base.runs.len() - 1);
    assert_eq!(after_remove.runs[0], base.runs[1]);
}

#[semio_framework_async_macros::async_test]
async fn remove_run_of_an_out_of_range_index_has_an_empty_inverse() {
    let base = fixture();
    let remove = SemioTextMutation::RemoveRun(remove_run::RemoveRun { index: 99 });
    assert!(remove.inverse(&base).is_empty(), "removing an absent index has nothing to undo");
    assert_eq!(remove.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "an out-of-range remove is a no-op");
}

#[semio_framework_async_macros::async_test]
async fn edit_run_and_change_run_language_round_trip() {
    let base = fixture();

    let edit = SemioTextMutation::EditRun(edit_run::EditRun { index: 0, new_content: "greetings".into() });
    let after = round_trip(&base, &edit);
    assert_eq!(after.runs[0].content, "greetings");
    assert_eq!(after.runs[0].language, base.runs[0].language);

    let change_lang = SemioTextMutation::ChangeRunLanguage(change_run_language::ChangeRunLanguage { index: 0, new_language: "fr".into() });
    let after = round_trip(&base, &change_lang);
    assert_eq!(after.runs[0].language, "fr");

    let missing = SemioTextMutation::EditRun(edit_run::EditRun { index: 99, new_content: "x".into() });
    assert!(missing.inverse(&base).is_empty(), "editing an absent index has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn reorder_runs_round_trips() {
    let base = fixture();
    assert!(base.runs.len() >= 2, "fixture must have at least two runs to exercise reorder");

    let reorder = SemioTextMutation::ReorderRuns(reorder_runs::ReorderRuns { from: 0, to: 1 });
    let after = round_trip(&base, &reorder);
    assert_eq!(after.runs[0], base.runs[1]);
    assert_eq!(after.runs[1], base.runs[0]);
}

#[semio_framework_async_macros::async_test]
async fn add_remove_mark_round_trips() {
    let base = fixture();
    let mark = SemioTextMark { kind: SemioTextMarkKind::Link, href: "https://semio.tech".into() };

    let add = SemioTextMutation::AddMark(add_mark::AddMark { run_index: 0, index: 0, mark: mark.clone() });
    let after_add = round_trip(&base, &add);
    assert_eq!(after_add.runs[0].marks, vec![mark.clone()]);

    let undo = add.inverse(&base);
    assert_eq!(undo, vec![SemioTextMutation::RemoveMark(remove_mark::RemoveMark { run_index: 0, index: 0 })]);

    let remove = SemioTextMutation::RemoveMark(remove_mark::RemoveMark { run_index: 1, index: 0 });
    let after_remove = round_trip(&base, &remove);
    assert!(after_remove.runs[1].marks.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn add_remove_mark_of_an_absent_run_has_an_empty_inverse() {
    let base = fixture();
    let remove = SemioTextMutation::RemoveMark(remove_mark::RemoveMark { run_index: 99, index: 0 });
    assert!(remove.inverse(&base).is_empty());
    assert_eq!(remove.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(SemioTextMutation::kinds().len(), 7);
    let mutation = SemioTextMutation::RemoveRun(remove_run::RemoveRun { index: 2 });
    assert_eq!(mutation.semantics().kind, "remove-run");
    assert_eq!(mutation.semantics().record, "RemovedRun");
    assert_eq!(mutation.target(), vec!["2".to_string()]);
}

/// 🏷️ `KINDS` (this facet's own const, consumed by `mutate-semio-text`'s adapter) must name
/// every declared variant, in the exact order and spelling `#[derive(dsl::Mutations)]` assigns —
/// the framework never parses Rust, so this is what keeps the catalog honest.
#[semio_framework_async_macros::async_test]
async fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = SemioTextMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
