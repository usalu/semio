use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_bmp_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, BMP_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<BmpEditor as ArtifactEditor>::DIALECT, BMP_DIALECT);
}

fn number(value: u64) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value))
}

fn direct_command() -> BmpEditCommand {
    bmpEditor_command_from_action(
        paint_region::DIRECT_ACTION_ID,
        Some(&semio_framework_value::DslValue::object([
            ("x".into(), number(1)),
            ("y".into(), number(0)),
            ("width".into(), number(1)),
            ("height".into(), number(2)),
            ("red".into(), number(9)),
            ("green".into(), number(8)),
            ("blue".into(), number(7)),
            ("alpha".into(), number(6)),
        ])),
    )
    .expect("typed direct BMP paint action")
}

fn direct_snapshot() -> BmpSnapshot {
    crate::standards::v_v3::subsets::any::io::decode_bmp(include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")).unwrap()
}

fn drive(command: &BmpEditCommand, snapshot: &BmpSnapshot, tool_id: &'static str) -> Vec<BmpMutation> {
    let reader=std::sync::Arc::new(snapshot.clone());let snapshot=reader.as_ref();
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "bmp-paint-test".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "bmp-paint-test".into() };
    let mut work = paint_region::PaintRegionWork::new(tool_id);
    assert_eq!(work.extent(command, snapshot, &interaction, None), paint_region::CAPACITY.rows_for_items(1));
    let mut sequence = 0;
    for _ in 0..paint_region::MAXIMUM_INTERACTIVE_ROWS as usize + 2 {
        let mut cx =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(3), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match work.step(&ArtifactCommandInputs { command, snapshot, snapshot_owner: Some(&reader), config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }, &mut cx).expect("BMP paint work step") {
            ArtifactCommandWorkStep::Progress { preview, .. } => assert!(std::str::from_utf8(preview).expect("localized progress").contains("de")),
            ArtifactCommandWorkStep::Complete(emit) => {work.begin_close();for _ in 0..10000{if work.close_step(1,4096)==semio_framework_job::InteractiveJobCloseStep::Complete{assert!(work.terminal_is_empty());return emit.artifact_mutations;}}panic!("BMP paint retained result did not close");},
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::CompleteWithEphemeral { .. } | ArtifactCommandWorkStep::CompleteDownload { .. } => panic!("unexpected BMP paint work step"),
        }
    }
    panic!("BMP paint work did not complete")
}

#[test]
fn initial_bitmap_is_immediately_paintable_and_reopens() {
    let snapshot = <BmpEditor as ArtifactEditor>::initial_snapshot();
    let layout = crate::standards::v_v3::subsets::any::io::bmp_layout(&snapshot).expect("initial BMP layout");
    assert_eq!((layout.width, layout.height, layout.profile), (1, 1, crate::standards::v_v3::subsets::any::schema::snapshot::BmpProfile::DirectRgb24));
    assert_eq!(crate::standards::v_v3::subsets::any::schema::operations::bmp_rgba8_preview(&snapshot).unwrap(), [255, 255, 255, 255]);
    assert_eq!(crate::standards::v_v3::subsets::any::io::decode_bmp(&crate::standards::v_v3::subsets::any::io::encode_bmp(&snapshot).unwrap()).unwrap(), snapshot);
}

#[test]
fn natural_file_route_preserves_bmp_v3_bytes_and_opens_a_fresh_snapshot_event() {
    let codec = <BmpEditor as ArtifactEditor>::natural_file_codec().expect("BMP natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.bmp@v3", ".bmp", "image/bmp", true));
    let source = include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp");
    let current = <BmpEditor as ArtifactEditor>::initial_snapshot();
    let imported = <BmpEditor as ArtifactEditor>::decode_natural_file(source).expect("BMP natural import");
    let exported = <BmpEditor as ArtifactEditor>::encode_natural_file(&imported).expect("BMP natural export");
    assert_eq!(crate::standards::v_v3::subsets::any::io::decode_bmp(&exported).unwrap(),imported,"precise BMP samples and metadata remain owned exactly");
    let independent = semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::oracle_identity_round_trip(&exported).expect("image crate reopens BMP export");
    let independent = crate::standards::v_v3::subsets::any::io::decode_bmp(&independent).expect("independent BMP output reopens");
    assert_eq!(crate::standards::v_v3::subsets::any::schema::operations::bmp_rgba8_preview(&independent).unwrap(), crate::standards::v_v3::subsets::any::schema::operations::bmp_rgba8_preview(&imported).unwrap());
    let Some(BmpMutation::ReplaceImage(set)) = <BmpEditor as ArtifactEditor>::whole_document_operation(imported.clone()) else { panic!("natural BMP opens through one event-sourced snapshot mutation") };
    assert_eq!(set.image, imported.image);
    assert_eq!(current, <BmpEditor as ArtifactEditor>::initial_snapshot(), "opening does not replace the selected owner before publication");
}

#[semio_framework_async_macros::async_test]
async fn image_window_exposes_bilingual_profile_specific_paint_controls() {
    let definition = create_bmp_editor();
    let indexed = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == paint_region::INDEXED_ACTION_ID).expect("indexed BMP action");
    let direct = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == paint_region::DIRECT_ACTION_ID).expect("direct BMP action");
    assert_eq!((indexed.args.len(), direct.args.len()), (5, 8));
    assert_eq!(indexed.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert_eq!(direct.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(indexed.args.iter().chain(&direct.args).all(|argument| argument.required && matches!(argument.schema, semio_framework_plugin::ArgSchema::Number { integer: true, .. })));
    assert_eq!(indexed.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Paint Indexed Region");
    assert_eq!(indexed.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Indexbereich malen");
    assert_eq!(direct.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Paint Direct Region");
    assert_eq!(direct.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Direktfarbbereich malen");
}

#[test]
fn retained_direct_paint_captures_revision_and_preserves_unaddressed_bytes() {
    use protocol::{Mutation, MutationDiff};
    let before = direct_snapshot();
    let mutations = drive(&direct_command(), &before, paint_region::DIRECT_ACTION_ID);
    assert_eq!(mutations.len(), 1);
    let BmpMutation::PaintDirectRegion(payload) = &mutations[0] else { panic!("typed direct paint intent publication") };
    assert_eq!(payload.revision,crate::schema::operations::bmp_revision(&before));
    let after = protocol::apply_diff(&mutations[0].diff(&before).diff(), &before).expect("apply direct paint");
    assert_eq!(after.image.opaque_gap,before.image.opaque_gap);assert_eq!(after.image.opaque_trailer,before.image.opaque_trailer);
    let layout = crate::standards::v_v3::subsets::any::io::bmp_layout(&before).unwrap();
    for row in 0..2 {
        let sample = layout.data_offset + row * layout.row_stride + 3;
        assert_eq!(&crate::standards::v_v3::subsets::any::io::encode_bmp(&after).unwrap()[sample..sample + 3], &[7, 8, 9]);
    }
    assert_eq!(&crate::standards::v_v3::subsets::any::io::encode_bmp(&after).unwrap()[54..57], &[0xde, 0xad, 0xbe]);
    assert_eq!(&crate::standards::v_v3::subsets::any::io::encode_bmp(&after).unwrap()[crate::standards::v_v3::subsets::any::io::encode_bmp(&after).unwrap().len() - 4..], &[0xfe, 0xed, 0xfa, 0xce]);
}

#[test]
fn retained_indexed_paint_keeps_duplicate_color_indices_distinct() {
    use protocol::{Mutation, MutationDiff};
    let before = crate::standards::v_v3::subsets::any::io::decode_bmp(include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb1-duplicate-palette.bmp")).unwrap();
    let command =
        bmpEditor_command_from_action(paint_region::INDEXED_ACTION_ID, Some(&semio_framework_value::DslValue::object([("x".into(), number(0)), ("y".into(), number(0)), ("width".into(), number(1)), ("height".into(), number(1)), ("paletteIndex".into(), number(1))])))
            .unwrap();
    let preview = crate::standards::v_v3::subsets::any::schema::operations::bmp_rgba8_preview(&before).unwrap();
    let mutations = drive(&command, &before, paint_region::INDEXED_ACTION_ID);
    let after = protocol::apply_diff(&mutations[0].diff(&before).diff(), &before).unwrap();
    assert_eq!(crate::standards::v_v3::subsets::any::schema::operations::bmp_rgba8_preview(&after).unwrap(), preview);
    assert_ne!(crate::standards::v_v3::subsets::any::io::encode_bmp(&after).unwrap(), crate::standards::v_v3::subsets::any::io::encode_bmp(&before).unwrap(), "the equal-color palette entry remains a distinct authored index");
}

#[test]
fn cancelled_retained_paint_discards_revision_without_publication() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let reader=std::sync::Arc::new(direct_snapshot());let snapshot=reader.as_ref();
    let command = direct_command();
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "bmp-paint-cancel".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "bmp-paint-cancel".into() };
    let mut work = paint_region::PaintRegionWork::new(paint_region::DIRECT_ACTION_ID);
    let input = ArtifactCommandInputs { command: &command, snapshot, snapshot_owner: Some(&reader), config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    let mut sequence = 0;
    let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(3), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(work.step(&input, &mut cx).unwrap(), ArtifactCommandWorkStep::Progress { stage: "bmp-paint-region-copy", .. }));
    let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(3), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(work.step(&input, &mut cx).unwrap(), ArtifactCommandWorkStep::Progress { stage: "bmp-paint-region-copy", .. }));
    work.begin_close();
    assert_eq!(work.close_step(1, 0),semio_framework_job::InteractiveJobCloseStep::Complete);
    for _ in 0..100_000 {if work.terminal_is_empty() {break;}work.close_step(1,usize::MAX);}
    assert!(work.terminal_is_empty());
    assert_eq!(snapshot, &direct_snapshot());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::BmpEditor, || semio_framework_plugin::App { definition: super::create_bmp_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️v3/🪆️subsets/✳️any");
