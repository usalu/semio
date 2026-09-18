use super::*;
use crate::editor::note::commands::{nudge_selection_down, nudge_selection_left, nudge_selection_right, nudge_selection_right_fast};
use crate::schema::{block_bounds, block_id, create_block_by_kind, mutations::apply_note_mutation, NoteIdOwner};
use crate::NoteSnapshot;

/// 🕹️ Selection is framework-owned `InteractionState`, resolved into `NoteDispatchCtx::selected_block_ids`
/// by `ArtifactEditor::handle`/the retained tool. These laws drive the handlers with that resolved selection
/// directly: the native app harness delivers an EMPTY `InteractionState` to a migrated verb's tool job
/// (ticket 26/09/17/NOTE-PLUGIN-END-TO-END — proven with `[DEBUG]` instrumentation; the react shell delivers
/// it correctly, see the ticket's `🐍️note-interact-probe.mjs` delete/undo steps), so a dispatch-level nudge law
/// would assert the harness, not the command.
fn selected_document() -> (NoteSnapshot, String) {
    let mut ids = NoteIdOwner::new("nudge-test", 0);
    let block = create_block_by_kind(&mut ids, "text", 0.0, 0.0);
    let id = block_id(&block).to_string();
    (NoteSnapshot { blocks: vec![block], ..crate::schema::empty_note_snapshot() }, id)
}

fn moved_bounds(document: &NoteSnapshot, selected: &str, emit: &semio_framework_plugin::Emit<crate::op::NoteMutation, semio_framework_plugin::NoConfigMutation>) -> (f64, f64) {
    assert_eq!(emit.artifact_mutations.len(), 1, "a nudge emits exactly one move");
    let next = apply_note_mutation(document, &emit.artifact_mutations[0]).expect("apply nudge");
    let block = crate::schema::find_block(&next.blocks, selected).expect("nudged block");
    let (x, y, ..) = block_bounds(block);
    (x, y)
}

fn dispatch_ctx(selected: &str) -> crate::editor::note::NoteDispatchCtx {
    crate::editor::note::NoteDispatchCtx { selected_block_ids: vec![selected.to_string()], id_owner: NoteIdOwner::new("nudge-test", 1), view_state: None, window_transient: Default::default(), window_transient_owner: None }
}

#[semio_framework_async_macros::async_test]
async fn nudge_direction_actions_move_selection_without_args() {
    type Nudge = fn(&NoteSnapshot, &str) -> semio_framework_plugin::Emit<crate::op::NoteMutation, semio_framework_plugin::NoConfigMutation>;
    let up: Nudge = |document, selected| {
        let history = semio_framework_plugin::HistoryView::empty();
        handle(&NudgeSelectionUp {}, &semio_framework_plugin::ArtifactView::new(document, &history), &semio_framework_plugin::ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None }, &mut dispatch_ctx(selected)).expect("nudge up")
    };
    let down: Nudge = |document, selected| {
        let history = semio_framework_plugin::HistoryView::empty();
        nudge_selection_down::handle(&nudge_selection_down::NudgeSelectionDown {}, &semio_framework_plugin::ArtifactView::new(document, &history), &semio_framework_plugin::ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None }, &mut dispatch_ctx(selected)).expect("nudge down")
    };
    let left: Nudge = |document, selected| {
        let history = semio_framework_plugin::HistoryView::empty();
        nudge_selection_left::handle(&nudge_selection_left::NudgeSelectionLeft {}, &semio_framework_plugin::ArtifactView::new(document, &history), &semio_framework_plugin::ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None }, &mut dispatch_ctx(selected)).expect("nudge left")
    };
    let right: Nudge = |document, selected| {
        let history = semio_framework_plugin::HistoryView::empty();
        nudge_selection_right::handle(&nudge_selection_right::NudgeSelectionRight {}, &semio_framework_plugin::ArtifactView::new(document, &history), &semio_framework_plugin::ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None }, &mut dispatch_ctx(selected)).expect("nudge right")
    };
    for (nudge, expected, label) in [(up, (0.0, -1.0), "up"), (down, (0.0, 1.0), "down"), (left, (-1.0, 0.0), "left"), (right, (1.0, 0.0), "right")] {
        let (document, selected) = selected_document();
        assert_eq!(moved_bounds(&document, &selected, &nudge(&document, &selected)), expected, "nudge {label} moved the block to an unexpected position");
    }
}

#[semio_framework_async_macros::async_test]
async fn nudge_fast_actions_use_ten_pixel_step() {
    let (document, selected) = selected_document();
    let history = semio_framework_plugin::HistoryView::empty();
    let emit = nudge_selection_right_fast::handle(
        &nudge_selection_right_fast::NudgeSelectionRightFast {},
        &semio_framework_plugin::ArtifactView::new(&document, &history),
        &semio_framework_plugin::ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None },
        &mut dispatch_ctx(&selected),
    )
    .expect("nudge right fast");
    assert_eq!(moved_bounds(&document, &selected, &emit), (10.0, 0.0));
}

#[semio_framework_async_macros::async_test]
async fn a_nudge_without_a_selection_emits_nothing() {
    let (document, _) = selected_document();
    let history = semio_framework_plugin::HistoryView::empty();
    let mut ctx = crate::editor::note::NoteDispatchCtx { selected_block_ids: Vec::new(), id_owner: NoteIdOwner::new("nudge-test", 2), view_state: None, window_transient: Default::default(), window_transient_owner: None };
    let emit = handle(&NudgeSelectionUp {}, &semio_framework_plugin::ArtifactView::new(&document, &history), &semio_framework_plugin::ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None }, &mut ctx).expect("nudge up");
    assert!(emit.artifact_mutations.is_empty(), "an empty selection must not move anything");
}
