use crate::GIF_89A_DIALECT;
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use crate::editor::gif_89a::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_gif_89a_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, GIF_89A_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Gif89aEditor as ArtifactEditor>::DIALECT, GIF_89A_DIALECT);
}

#[test]
fn natural_file_codec_declares_gif_and_round_trips_the_demo_document() {
    let codec = <Gif89aEditor as ArtifactEditor>::natural_file_codec().expect("GIF declares its natural file codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.gif@89a", ".gif", "image/gif", true));
    let snapshot = crate::standards::v89a::subsets::any::schema::demo_gif_snapshot();
    let bytes = <Gif89aEditor as ArtifactEditor>::encode_natural_file(&snapshot).expect("GIF natural bytes");
    assert!(bytes.starts_with(b"GIF8"));
    let reopened = <Gif89aEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("natural bytes reopen");
    assert_eq!(<Gif89aEditor as ArtifactEditor>::encode_natural_file(&reopened).expect("reopened document exports"), bytes);
}

#[test]
fn natural_file_decode_refuses_truncated_and_foreign_bytes_without_panicking() {
    let bytes = <Gif89aEditor as ArtifactEditor>::encode_natural_file(&crate::standards::v89a::subsets::any::schema::demo_gif_snapshot()).expect("GIF natural bytes");
    for length in [0, 3, 6, 12, bytes.len() / 2, bytes.len() - 1] {
        assert!(<Gif89aEditor as ArtifactEditor>::decode_natural_file(&bytes[..length]).is_err(), "truncated at {length}");
    }
    assert!(<Gif89aEditor as ArtifactEditor>::decode_natural_file(b"\x89PNG\r\n\x1a\n").is_err());
}

#[test]
fn schema_declares_one_native_codec_per_standard_and_the_runtime_binds_them() {
    let factories = crate::native_codecs();
    assert_eq!(factories.iter().map(|factory| factory.id).collect::<Vec<_>>(), ["stdio.native.gif.87a.v1", "stdio.native.gif.89a.v1"]);
    crate::definition().expect("every schema-declared GIF native codec binds to its factory, schema, extension and pack hash");
}


fn paint_command(args: Vec<(&str, u64)>) -> Gif89aEditCommand {
    let number = |value| semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value));
    super::gif89a_editor_command_from_action(paint_region::ACTION_ID, Some(&semio_framework_value::DslValue::object(args.into_iter().map(|(key, value)| (key.into(), number(value))).collect::<Vec<_>>()))).expect("typed paint-region action")
}

fn drive_paint(command: &Gif89aEditCommand, snapshot: &crate::standards::v89a::subsets::any::schema::snapshot::GifSnapshot) -> Result<Vec<crate::standards::v89a::subsets::any::schema::mutations::GifMutation>, semio_plugin::Fault> {
    use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let config = semio_plugin::NoConfig::default();
    let history = semio_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_plugin::app::InteractionHoverState::default();
    let operation = semio_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "paint-region-test".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "paint-region-test".into(), retained: semio_s_artifact_stdio_contract::editing::ample_close_grant() };
    let mut work = paint_region::PaintRegionWork::new();
    assert!(work.extent(command, snapshot, &interaction, None).is_some());
    for _ in 0..4096 {
        let mut sequence = 0;
        let mut step_progress = semio_framework_value::retained_clone::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, semio_s_artifact_stdio_contract::testing::unbounded_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut step_progress);
        let step = work.step(&ArtifactCommandInputs { snapshot_owner: None, command, snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }, &mut cx);
        match step {
            Err(failure) => return Err(semio_plugin::Fault::from(format!("{failure:?}"))),
            Ok(ArtifactCommandWorkStep::Progress { preview, .. }) => assert!(std::str::from_utf8(preview).unwrap().contains("\"de\"")),
            Ok(ArtifactCommandWorkStep::Complete(emit)) => return Ok(emit.artifact_mutations),
            Ok(_) => panic!("unexpected paint region work step"),
        }
    }
    panic!("paint region work did not complete")
}

#[test]
fn paint_region_fills_one_rectangle_of_one_frame_and_leaves_every_other_index_untouched() {
    use protocol::{ColdMutation, Mutation, MutationDiff};
    let base = crate::standards::v89a::subsets::any::schema::demo_gif_snapshot();
    let item = &base.frames[0];
    let (width, height) = (item.width as u64, item.height as u64);
    let command = paint_command(vec![("item", 0), ("x", 0), ("y", 0), ("width", 1), ("height", 1), ("index", 1)]);
    let mutations = drive_paint(&command, &base).expect("region paint completes");
    assert_eq!(mutations.len(), 1);
    let edited = protocol::apply_diff_cold(mutations[0].diff(&base).diff(), &base).expect("paint applies");
    assert_eq!(edited.frames[0].indices[0], 1);
    assert_eq!(&edited.frames[0].indices[1..], &base.frames[0].indices[1..]);
    assert_eq!((edited.frames[0].width as u64, edited.frames[0].height as u64), (width, height));
    let inverse = mutations[0].inverse(&base).expect("paint inverse");
    let restored = inverse.iter().fold(edited, |current, mutation| protocol::apply_diff_cold(mutation.diff(&current).diff(), &current).expect("undo applies"));
    assert_eq!(restored, base);
}

#[test]
fn paint_region_refuses_a_missing_frame_and_an_out_of_bounds_rectangle_without_mutating() {
    let base = crate::standards::v89a::subsets::any::schema::demo_gif_snapshot();
    let missing = paint_command(vec![("item", 99), ("x", 0), ("y", 0), ("width", 1), ("height", 1), ("index", 0)]);
    assert!(drive_paint(&missing, &base).is_err());
    let outside = paint_command(vec![("item", 0), ("x", 0), ("y", 0), ("width", 100000), ("height", 1), ("index", 0)]);
    assert!(drive_paint(&outside, &base).is_err());
}

#[test]
fn window_declares_region_painting_as_a_mutation_action() {
    use crate::editor::gif_89a::modes::edit::windows::main;
    let definition = main::definition();
    let paint = definition.actions.iter().find(|action| action.id == "paint-region").expect("region painting is a window action");
    assert_eq!(paint.kind, semio_plugin::ActionKind::Mutation);
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::gif_89a::Gif89aEditor, || semio_plugin::App { definition: crate::editor::gif_89a::create_gif_89a_editor(), examples: Vec::new() }, "../../🏅️standards/9️⃣89a/🪆️subsets/🧱️base");
