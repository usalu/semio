use super::*;
use crate::{NoteBlockNode, NoteImageAsset};
use protocol::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
use protocol::SemanticMutation;

fn sample_snapshot() -> NoteSnapshot {
    let mut snapshot = crate::schema::empty_note_snapshot();
    snapshot.blocks.push(NoteBlockNode::Text {
        id: "b1".into(),
        name: "Text".into(),
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 40.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        content: crate::note_text_child_record("b1", &[]),
        font_size: 18.0,
        font_weight: "normal".into(),
        align: "left".into(),
    });
    snapshot.blocks.push(NoteBlockNode::Ink { id: "b2".into(), name: "Ink".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0, visible: true, locked: false, points: vec![[0.0, 0.0]], stroke_width: 3.0, color: [0.0, 0.0, 0.0, 1.0] });
    snapshot.blocks.push(NoteBlockNode::Table {
        id: "b3".into(),
        name: "Table".into(),
        x: 0.0,
        y: 0.0,
        width: 200.0,
        height: 100.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        columns: vec!["A".into(), "B".into()],
        rows: vec![vec![crate::NoteTableCell { content: String::new() }, crate::NoteTableCell { content: String::new() }]],
    });
    snapshot.blocks.push(NoteBlockNode::Math { id: "b4".into(), name: "Math".into(), x: 0.0, y: 0.0, width: 100.0, height: 40.0, rotation: 0.0, visible: true, locked: false, tex: "x".into(), display_mode: true });
    snapshot.assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "d".into(), width: None, height: None });
    snapshot
}

fn round_trip(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> NoteSnapshot {
    let forward = apply_note_mutation(snapshot, mutation).expect("valid mutation diff");
    let mut restored = forward.clone();
    for back in mutation.inverse(snapshot) {
        restored = apply_note_mutation(&restored, &back).expect("valid inverse mutation diff");
    }
    assert_eq!(&restored, snapshot, "inverse must restore the pre-mutation snapshot for {mutation:?}");
    forward
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_note_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in NoteMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(NoteMutation::kinds().len(), 33);
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn root_scalar_inverse_and_absorb_laws() {
    let base = sample_snapshot();
    for mutation in [
        rename_note(Some("Renamed".into())),
        change_grid_visible(Some(false)),
        change_grid_spacing(Some(16.0)),
        change_grid_subdivisions(Some(8.0)),
        change_grid_opacity(Some(0.6)),
        change_snap_enabled(Some(true)),
        change_snap_grid_spacing(Some(4.0)),
        change_pencil_width(Some(5.0)),
        change_eraser_radius(Some(20.0)),
    ] {
        assert_mutation_inverse_law(&base, &mutation).await;
    }
    let d1 = change_grid_spacing(Some(10.0)).diff(&base).into_parts().0;
    let mid = MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = change_grid_spacing(Some(20.0)).diff(&mid).into_parts().0;
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn asset_inverse_law_create_replace_delete() {
    let base = sample_snapshot();
    let asset = NoteImageAsset { mime: "image/jpeg".into(), data: "e".into(), width: None, height: None };
    assert_mutation_inverse_law(&base, &create_asset("asset-2".into(), asset.clone())).await;
    assert_mutation_inverse_law(&base, &replace_asset_payload("asset-1".into(), asset.clone())).await;
    assert_mutation_inverse_law(&base, &delete_asset("asset-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn block_lifecycle_inverse_law_create_delete_duplicate() {
    let base = sample_snapshot();
    let new_block = NoteBlockNode::Text {
        id: "b99".into(),
        name: "New".into(),
        x: 5.0,
        y: 6.0,
        width: 80.0,
        height: 30.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        content: crate::note_text_child_record("b99", &[]),
        font_size: 18.0,
        font_weight: "normal".into(),
        align: "left".into(),
    };
    assert_mutation_inverse_law(&base, &create_block(new_block.clone(), None, None)).await;
    assert_mutation_inverse_law(&base, &delete_block("b1".into())).await;
    assert_mutation_inverse_law(&base, &delete_blocks(vec!["b1".into(), "b3".into()])).await;
    let dup = crate::schema::clone_block(&mut crate::schema::NoteIdOwner::new("mutation-test", 0), base.blocks.iter().find(|b| crate::schema::block_id(b) == "b1").unwrap());
    assert_mutation_inverse_law(&base, &duplicate_block("b1".into(), dup)).await;
}

#[semio_framework_async_macros::async_test]
async fn block_reparent_and_drag_inverse_law() {
    let mut base = sample_snapshot();
    base.blocks.push(NoteBlockNode::Group { id: "g1".into(), name: "Group".into(), x: 0.0, y: 0.0, width: 200.0, height: 200.0, rotation: 0.0, visible: true, locked: false, children: Vec::new() });
    assert_mutation_inverse_law(&base, &move_block_to_container("b1".into(), Some("g1".into()), 0)).await;
    assert_mutation_inverse_law(&base, &drag_blocks(vec!["b1".into(), "b2".into()], 5.0, -3.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn block_field_inverse_laws() {
    let base = sample_snapshot();
    assert_mutation_inverse_law(&base, &rename_block("b1".into(), "Renamed".into())).await;
    assert_mutation_inverse_law(&base, &change_block_visible("b1".into(), false)).await;
    assert_mutation_inverse_law(&base, &change_block_locked("b1".into(), true)).await;
    assert_mutation_inverse_law(&base, &move_block("b1".into(), 42.0, -8.0)).await;
    assert_mutation_inverse_law(&base, &resize_block("b1".into(), 120.0, 60.0)).await;
    assert_mutation_inverse_law(&base, &change_block_font_size("b1".into(), 24.0)).await;
    assert_mutation_inverse_law(&base, &edit_block_text("b1".into(), vec![crate::NoteTextParagraph { runs: Vec::new() }])).await;
    assert_mutation_inverse_law(&base, &edit_block_math("b4".into(), "y = mx + b".into())).await;
    assert_mutation_inverse_law(&base, &change_block_ink_width("b2".into(), 6.0)).await;
    assert_mutation_inverse_law(&base, &edit_block_ink_stroke("b2".into(), vec![[0.0, 0.0], [1.0, 1.0]], 1.0, 2.0, 10.0, 10.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn table_row_column_inverse_laws() {
    let base = sample_snapshot();
    assert_mutation_inverse_law(&base, &insert_table_row("b3".into())).await;
    assert_mutation_inverse_law(&base, &remove_table_row("b3".into())).await;
    assert_mutation_inverse_law(&base, &insert_table_column("b3".into())).await;
    assert_mutation_inverse_law(&base, &remove_table_column("b3".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn create_delete_block_round_trip_grows_and_shrinks_projection() {
    let base = sample_snapshot();
    let new_block = NoteBlockNode::Text {
        id: "b100".into(),
        name: "New".into(),
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        content: crate::note_text_child_record("b100", &[]),
        font_size: 18.0,
        font_weight: "normal".into(),
        align: "left".into(),
    };
    let added = round_trip(&base, &create_block(new_block, None, None));
    assert_eq!(added.blocks.len(), base.blocks.len() + 1);
    let removed = round_trip(&added, &delete_block("b100".into()));
    assert_eq!(removed.blocks.len(), base.blocks.len());
}

#[semio_framework_async_macros::async_test]
async fn delete_block_at_a_non_last_index_restores_exact_position_on_undo() {
    let base = sample_snapshot();
    // b1 is index 0 of 4; deleting then undoing must restore it there, not append it at the end.
    round_trip(&base, &delete_block("b1".into()));
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
/// (create/delete(s)/rename/change/move/resize/drag/duplicate/insert/remove/edit/replace).
#[semio_framework_async_macros::async_test]
async fn create_block_duplicate_id_is_fatal() {
    let base = sample_snapshot();
    let existing = NoteBlockNode::Text {
        id: "b1".into(),
        name: "Dup".into(),
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        content: crate::note_text_child_record("b1", &[]),
        font_size: 18.0,
        font_weight: "normal".into(),
        align: "left".into(),
    };
    let outcome = create_block(existing, None, None).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn delete_block_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &delete_block("ghost".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_blocks_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &delete_blocks(vec!["ghost".into()])).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_block_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &rename_block("ghost".into(), "x".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn change_block_locked_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &change_block_locked("ghost".into(), true)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_block_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &move_block("ghost".into(), 1.0, 1.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_block_non_finite_is_fatal() {
    let base = sample_snapshot();
    let outcome = move_block("b1".into(), f64::NAN, 0.0).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn resize_block_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &resize_block("ghost".into(), 10.0, 10.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn drag_blocks_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &drag_blocks(vec!["ghost".into()], 1.0, 1.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn duplicate_block_missing_source_is_error() {
    let base = sample_snapshot();
    let block = NoteBlockNode::Text {
        id: "b101".into(),
        name: "New".into(),
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        content: crate::note_text_child_record("b101", &[]),
        font_size: 18.0,
        font_weight: "normal".into(),
        align: "left".into(),
    };
    assert_missing_target_is_error(&base, &duplicate_block("ghost".into(), block)).await;
}

#[semio_framework_async_macros::async_test]
async fn insert_table_row_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &insert_table_row("ghost".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_table_row_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &remove_table_row("ghost".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn edit_block_text_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &edit_block_text("ghost".into(), Vec::new())).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_asset_payload_missing_target_is_error() {
    let base = sample_snapshot();
    let asset = NoteImageAsset { mime: "image/jpeg".into(), data: "e".into(), width: None, height: None };
    assert_missing_target_is_error(&base, &replace_asset_payload("ghost".into(), asset)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_asset_duplicate_id_is_fatal() {
    let base = sample_snapshot();
    let asset = NoteImageAsset { mime: "image/png".into(), data: "d".into(), width: None, height: None };
    let outcome = create_asset("asset-1".into(), asset).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn delete_asset_missing_target_is_error() {
    let base = sample_snapshot();
    assert_missing_target_is_error(&base, &delete_asset("ghost".into())).await;
}
//#endregion 🔖️OutcomeLaws

//#region ⏪️TimeTravel
/// ⏪️ Opens a standalone Note store over `base`, commits `log` one edit per leaf, supersedes the leaf at `index` with
/// `edited` and drives the Report replay to its end: the preview base is the fold of the prefix, the replay is the
/// fresh fold of the edited log (Error/Fatal leaves fold as no-ops), and a clean report overwrites to exactly that state.
async fn replay_history_edit(base: &NoteSnapshot, log: &[NoteMutation], index: usize, edited: &NoteMutation) -> protocol::ReplayReport {
    use protocol::OpBinary;
    let mut store = crate::standards::v1::subsets::any::io::snapshot::binary::new_note_store(store::create_document_envelope::<NoteSnapshot, NoteMutation>(crate::NOTE_DOCUMENT_SCHEMA, "drag-time-travel", base.clone(), None)).await.expect("the store opens");
    for mutation in log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: None }).await.expect("a block edit applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[index].clone(), protocol::InputReplacement::Input { schema: crate::NOTE_DOCUMENT_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    let prefix = log[..index].iter().fold(base.clone(), |snapshot, mutation| apply_note_mutation(&snapshot, mutation).expect("the prefix folds"));
    assert_eq!(store.state_before(&ids[index], &drafts).expect("the preview base folds").as_ref(), &prefix, "the preview base is the state right before the edited leaf, nothing downstream");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[index])).expect("the replay begins at the edited leaf");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    let report = store.replay_report(&result).expect("report");
    let fresh = log.iter().enumerate().fold(base.clone(), |snapshot, (position, mutation)| {
        let mutation = if position == index { edited } else { mutation };
        if mutation.diff(&snapshot).messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal)) { snapshot } else { apply_note_mutation(&snapshot, mutation).expect("the edited log folds") }
    });
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    if !report.blocks_finalize() {
        store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
        assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    }
    store.close();
    report
}

/// ⏪️ Time travel edits a block drag's inputs, never the ink tool: re-offsetting the first drag replays the downstream
/// move and drag onto the edited offsets, never blocks finalizing, and reports the edited leaf plus every downstream one.
#[semio_framework_async_macros::async_test]
async fn a_block_drag_edited_in_history_replays_its_downstream() {
    let base = sample_snapshot();
    let log = [drag_blocks(vec!["b1".into(), "b2".into()], 10.0, 5.0), move_block("b3".into(), 40.0, 40.0), drag_blocks(vec!["b1".into(), "b3".into()], 1.0, -2.0)];
    let report = replay_history_edit(&base, &log, 0, &drag_blocks(vec!["b1".into(), "b2".into()], -4.0, 2.5)).await;
    assert!(!report.blocks_finalize(), "{report:?}");
    assert_eq!(report.outcomes.len(), log.len(), "the replay reports the edited drag and every downstream leaf");
}

/// 🚨️ Retargeting a block drag onto a block that does not exist reports the edited leaf's own
/// `mutation.target-missing` Error, which blocks finalizing until it is edited again or withdrawn.
#[semio_framework_async_macros::async_test]
async fn a_block_drag_retargeted_onto_a_missing_block_blocks_finalizing() {
    let base = sample_snapshot();
    let log = [drag_blocks(vec!["b1".into()], 10.0, 5.0), drag_blocks(vec!["b1".into()], 1.0, 1.0)];
    let report = replay_history_edit(&base, &log, 0, &drag_blocks(vec!["ghost".into()], 10.0, 5.0)).await;
    assert!(report.blocks_finalize(), "{report:?}");
    assert!(report.outcomes[0].messages.iter().any(|message| message.code.0 == "mutation.target-missing"), "{report:?}");
}

/// 🗣️ The drag's history row label reads the block count and the offset in English and German.
#[test]
fn a_block_drag_label_reads_the_count_and_offset() {
    let label = |mutation: NoteMutation| { let label = mutation.label(); (label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_owned(), label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_owned()) };
    assert_eq!(label(drag_blocks(vec!["b1".into()], 2.5, -10.0)), ("Drag 1 block by (2.5, -10)".to_string(), "1 Block um (2,5; -10) ziehen".to_string()));
    assert_eq!(label(drag_blocks(vec!["b1".into(), "b2".into()], 0.0, 3.0)), ("Drag 2 blocks by (0, 3)".to_string(), "2 Blöcke um (0; 3) ziehen".to_string()));
}

/// 🎚️ Document-setting rows read the value the way the inspector shows it (opacity in percent, German decimal comma, switches
/// as verbs), an unset value reads as a reset, and no label leaks a Rust `Option` debug form.
#[test]
fn document_setting_labels_read_the_value_not_a_debug_option() {
    let label = |mutation: NoteMutation| { let label = mutation.label(); (label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_owned(), label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_owned()) };
    let pair = |en: &str, de: &str| (en.to_string(), de.to_string());
    assert_eq!(label(change_grid_opacity(Some(0.76))), pair("Change grid opacity to 76%", "Rasterdeckkraft auf 76 % ändern"));
    assert_eq!(label(change_grid_opacity(None)), pair("Reset grid opacity", "Rasterdeckkraft zurücksetzen"));
    assert_eq!(label(change_grid_spacing(Some(12.5))), pair("Change grid spacing to 12.5", "Rasterabstand auf 12,5 ändern"));
    assert_eq!(label(change_grid_visible(Some(false))), pair("Hide grid", "Raster ausblenden"));
    assert_eq!(label(change_snap_enabled(Some(true))), pair("Enable snapping", "Fangfunktion einschalten"));
    assert_eq!(label(rename_note(Some("Plan".into()))), pair("Rename note to \"Plan\"", "Notiz in \"Plan\" umbenennen"));
    assert_eq!(label(rename_note(None)), pair("Remove note title", "Notiztitel entfernen"));
    let settings = [change_grid_opacity(Some(1.0)), change_grid_spacing(None), change_grid_subdivisions(Some(4.0)), change_grid_subdivisions(None), change_grid_visible(Some(true)), change_grid_visible(None), change_snap_enabled(Some(false)), change_snap_enabled(None), change_snap_grid_spacing(Some(8.0)), change_snap_grid_spacing(None), change_pencil_width(Some(2.25)), change_pencil_width(None), change_eraser_radius(Some(16.0)), change_eraser_radius(None)];
    for mutation in settings {
        let (en, de) = label(mutation.clone());
        assert!(![&en, &de].iter().any(|text| text.contains("Some(") || text.contains("None") || text.contains("true") || text.contains("false")), "{mutation:?}: {en} / {de}");
    }
}

/// 🪧️ A history-edit reference chip reads a block by its own name in the shown document (the generic default, gap N3), and
/// the drag's block reference takes "Use selection" from the composite window's block selection domain.
#[test]
fn block_references_read_their_name_and_take_the_block_selection() {
    let snapshot = sample_snapshot();
    let names = semio_framework_plugin::app::time_travel::time_travel_entity_names(&semio_framework_value::ToValue::to_value(&snapshot), &["b1"].into_iter().collect());
    assert_eq!(names.get("b1").map(|label| label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_owned()).as_deref(), Some("Text"));
    let schema: serde_json::Value = serde_json::from_str(<DragBlocks as protocol::MutationLeaf>::PAYLOAD_SCHEMA).expect("leaf schema");
    let reference = &schema["properties"]["ids"]["x-semio-ui"]["ref"];
    assert_eq!((reference["domain"].as_str(), reference["granularity"].as_str()), (Some(crate::editor::note::NOTE_INTERACTION_BLOCKS), Some(crate::editor::note::NOTE_INTERACTION_GRANULARITY)), "{reference}");
}
//#endregion ⏪️TimeTravel
