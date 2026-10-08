use super::*;

/// 🧬️ Registers the document schema tiff's declaration contributes — the contract every snapshot edit validates against;
/// a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor()]).expect("the tiff document schema registers");
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_tiff_any_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, TIFF_ANY_DIALECT.into());
    let select = def.window_kinds.iter().flat_map(|window| &window.actions).find(|action| action.id == main::SELECT_IFD_ACTION_ID).expect("TIFF page selection action");
    assert_eq!(select.kind, semio_framework_plugin::ActionKind::View);
    assert!(!select.in_palette);
    assert_eq!(select.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Select Image Page");
    assert_eq!(select.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Bildseite auswählen");
    assert_eq!(select.args.len(), 1);
    assert_eq!(select.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TiffAnyEditor as ArtifactEditor>::DIALECT, TIFF_ANY_DIALECT);
}

fn number(value: u64) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value))
}

#[test]
fn editor_page_selection_config_round_trips_and_has_an_exact_inverse() {
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    let before = TiffEditorConfig::default();
    let mutation = TiffEditorConfigMutation::SetSelectedIfd { selected_ifd: 1 };
    assert_eq!(TiffEditorConfigMutation::parse_op(&mutation.print_op()).expect("text config mutation"), mutation);
    assert_eq!(TiffEditorConfigMutation::decode_op(&mutation.encode_op().expect("binary config mutation")).expect("decoded config mutation"), mutation);
    let after = protocol::apply_diff(mutation.diff(&before).diff(), &before).expect("page selection applies");
    assert_eq!(after.selected_ifd, 1);
    let inverse = mutation.inverse(&before).expect("page selection inverse");
    assert_eq!(inverse.len(), 1);
    let restored = protocol::apply_diff(inverse[0].diff(&after).diff(), &after).expect("page selection inverse applies");
    assert_eq!(restored, before);
}

#[test]
fn retained_page_selection_publishes_only_the_addressed_config_lane() {
    let mut snapshot = tiled_editor_snapshot();
    snapshot.ifds.push(snapshot.ifds[0].clone());
    let command = TiffAnyEditCommand::SelectIfd { ifd_index: 1 };
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "tiff-page-selection".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "tiff-page-selection".into() };
    let emit = tiffAnyEditor_retained_reduce(&command, &snapshot, &TiffEditorConfig::default(), &history, &interaction, &hover, None, &operation).expect("retained page selection");
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.effects.is_empty());
    assert_eq!(emit.config_mutations, vec![TiffEditorConfigMutation::SetSelectedIfd { selected_ifd: 1 }]);
    assert!(tiffAnyEditor_retained_reduce(&TiffAnyEditCommand::SelectIfd { ifd_index: 2 }, &snapshot, &TiffEditorConfig::default(), &history, &interaction, &hover, None, &operation).is_err());
}

fn tiled_editor_snapshot() -> TiffSnapshot {
    use crate::schema::snapshot::{TiffSampleBlock, TiffWord64, TiffValues, TAG_BITS_PER_SAMPLE, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_SAMPLES_PER_PIXEL};
    let short = |tag, values| TiffTag { tag, values: TiffValues::Short(values) };
    let long = |tag, value| TiffTag { tag, values: TiffValues::Long(vec![value]) };
    TiffSnapshot {
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        ifds: vec![TiffIfd {
            entries: vec![
                long(TAG_IMAGE_WIDTH, 16),
                long(TAG_IMAGE_LENGTH, 16),
                short(TAG_BITS_PER_SAMPLE, vec![8, 8, 8]),
                short(TAG_PHOTOMETRIC, vec![2]),
                short(TAG_SAMPLES_PER_PIXEL, vec![3]),
                TiffTag { tag: 65000, values: TiffValues::Undefined(vec![7, 5, 3, 1]) },
            ],
            blocks: vec![TiffSampleBlock{x:0,y:0,width:16,height:16,channels:3,samples:vec![TiffWord64::default();16*16*3]}],
        }],
    }
}

fn paint_command() -> TiffAnyEditCommand {
    tiffAnyEditor_command_from_action(
        paint_region::ACTION_ID,
        Some(&semio_framework_value::DslValue::object([
            ("x".into(), number(15)),
            ("y".into(), number(15)),
            ("width".into(), number(1)),
            ("height".into(), number(1)),
            ("red".into(), number(9)),
            ("green".into(), number(8)),
            ("blue".into(), number(7)),
            ("alpha".into(), number(255)),
        ])),
    )
    .expect("typed TIFF paint action")
}

fn drive_paint(command: &TiffAnyEditCommand, snapshot: &TiffSnapshot, config: &TiffEditorConfig) -> Vec<TiffMutation> {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "tiff-paint-test".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "tiff-paint-test".into() };
    let mut work = paint_region::PaintRegionWork::new();
    assert_eq!(work.extent(command, snapshot, &interaction, None), paint_region::CAPACITY.rows_for_items(1));
    let mut sequence = 0;
    for _ in 0..4 {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(3), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match work.step(&ArtifactCommandInputs { snapshot_owner: None, command, snapshot, config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }, &mut cx).expect("TIFF paint work step") {
            ArtifactCommandWorkStep::Progress { preview, .. } => assert!(std::str::from_utf8(preview).expect("localized progress").contains("de")),
            ArtifactCommandWorkStep::Complete(emit) => return emit.artifact_mutations,
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::CompleteWithEphemeral { .. } | ArtifactCommandWorkStep::CompleteDownload { .. } => panic!("unexpected TIFF paint work step"),
        }
    }
    panic!("TIFF paint work did not complete")
}

#[semio_framework_async_macros::async_test]
async fn image_window_exposes_bilingual_tiled_paint_controls() {
    let definition = create_tiff_any_editor();
    let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == paint_region::ACTION_ID).expect("TIFF paint action");
    assert_eq!(action.args.len(), 8);
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(action.args.iter().all(|argument| argument.required && matches!(argument.schema, semio_framework_plugin::ArgSchema::Number { integer: true, .. })));
    assert_eq!(action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Paint Tiled Region");
    assert_eq!(action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Kachelbereich malen");
}

#[test]
fn retained_tiled_paint_captures_revision_round_trips_and_has_exact_inverse() {
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    let before = tiled_editor_snapshot();
    let mutations = drive_paint(&paint_command(), &before, &TiffEditorConfig::default());
    assert_eq!(mutations.len(), 1);
    let TiffMutation::PaintRegion(payload) = &mutations[0] else { panic!("addressed TIFF paint mutation") };
    assert_eq!(payload.revision, crate::standards::v6_0::subsets::document::schema::mutations::paint_region::samples::tiff_revision(&before));
    assert_eq!(TiffMutation::parse_op(&mutations[0].print_op()).expect("text paint round trip"), mutations[0]);
    assert_eq!(TiffMutation::decode_op(&mutations[0].encode_op().expect("binary paint encode")).expect("binary paint decode"), mutations[0]);
    let after = protocol::apply_diff(mutations[0].diff(&before).diff(), &before).expect("apply TIFF paint");
    assert_eq!(&after.ifds[0].blocks[0].samples[765..768], &[crate::schema::snapshot::TiffWord64::from_word(9),crate::schema::snapshot::TiffWord64::from_word(8),crate::schema::snapshot::TiffWord64::from_word(7)]);
    assert_eq!(after.ifds[0].entries, before.ifds[0].entries);
    let inverse = mutations[0].inverse(&before).expect("paint inverse");
    let restored = inverse.into_iter().fold(after, |current, mutation| protocol::apply_diff(mutation.diff(&current).diff(), &current).expect("apply TIFF paint inverse"));
    assert_eq!(restored, before);
}

#[test]
fn retained_tiled_paint_targets_the_selected_ifd_and_survives_artifact_undo() {
    use protocol::{Mutation, MutationDiff};
    let mut before = tiled_editor_snapshot();
    before.ifds.push(before.ifds[0].clone());
    let first_before = before.ifds[0].blocks.clone();
    let config = TiffEditorConfig { selected_ifd: 1 };
    let [mutation] = drive_paint(&paint_command(), &before, &config).try_into().expect("one selected-page paint mutation");
    let TiffMutation::PaintRegion(payload) = &mutation else { panic!("selected-page paint mutation") };
    assert_eq!(payload.ifd_index, 1);
    let after = protocol::apply_diff(mutation.diff(&before).diff(), &before).expect("selected page paint applies");
    assert_eq!(after.ifds[0].blocks, first_before);
    assert_eq!(&after.ifds[1].blocks[0].samples[765..768], &[crate::schema::snapshot::TiffWord64::from_word(9),crate::schema::snapshot::TiffWord64::from_word(8),crate::schema::snapshot::TiffWord64::from_word(7)]);
    let restored = mutation.inverse(&before).expect("selected page inverse").into_iter().fold(after, |current, inverse| protocol::apply_diff(inverse.diff(&current).diff(), &current).expect("selected page inverse applies"));
    assert_eq!(restored, before);
    assert_eq!(config.selected_ifd, 1, "artifact history never rewrites local page selection");
}

#[test]
fn cancelled_retained_tiled_paint_discards_revision_without_publication() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let snapshot = tiled_editor_snapshot();
    let command = paint_command();
    let config = TiffEditorConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "tiff-paint-cancel".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "tiff-paint-cancel".into() };
    let mut work = paint_region::PaintRegionWork::new();
    let input = ArtifactCommandInputs { snapshot_owner: None, command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    let mut sequence = 0;
    let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(3), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(work.step(&input, &mut cx).unwrap(), ArtifactCommandWorkStep::Progress { stage: "tiff-paint-region-prepare", .. }));
    let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(3), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(work.step(&input, &mut cx).unwrap(), ArtifactCommandWorkStep::Progress { stage: "tiff-paint-region-row", .. }));
    work.begin_close();
    assert!(matches!(work.close_step(1, 0), semio_framework_job::InteractiveJobCloseStep::Complete));
    assert!(work.terminal_is_empty());
    assert_eq!(snapshot, tiled_editor_snapshot());
}

#[test]
fn physical_byte_order_options_preserve_owned_large_sample_identity() {
 use crate::standards::v6_0::subsets::document::io::{TiffByteOrder,TiffNativeOptions,encode_tiff_with,decode_tiff};
 let snapshot=tiled_editor_snapshot();let before=snapshot.clone();
 for byte_order in [TiffByteOrder::LittleEndian,TiffByteOrder::BigEndian]{let native=encode_tiff_with(&snapshot,TiffNativeOptions{byte_order,..Default::default()}).unwrap();assert_eq!(&native[..2],if byte_order==TiffByteOrder::LittleEndian{b"II"}else{b"MM"});assert_eq!(decode_tiff(&native).unwrap(),snapshot);}
 assert_eq!(snapshot,before);
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    register_document_schema();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let mut snapshot = crate::schema::blank_tiff_snapshot();
    snapshot.ifds[0].entries.push(TiffTag{tag:65000,values:crate::schema::snapshot::TiffValues::Undefined(vec![7,9])});
    let payload_path=format!("/ifds/0/entries/{}/values/value",snapshot.ifds[0].entries.len()-1);
    let base: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("{payload_path}{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") { *from = format!("{payload_path}{}", from.as_str().unwrap()).into(); }
        let event: editing::SnapshotEditEvent = semio_framework_pack_json::from_json_str(&event.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let emitted = <TiffAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::apply_diff(<TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut(&payload_path).unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::TiffAnyEditor, || semio_framework_plugin::App { definition: super::create_tiff_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document");
