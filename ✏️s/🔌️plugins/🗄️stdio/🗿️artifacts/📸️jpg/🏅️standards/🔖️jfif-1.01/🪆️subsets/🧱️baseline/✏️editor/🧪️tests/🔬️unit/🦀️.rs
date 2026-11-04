use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::JpgMutation;
use crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot;
use crate::JPG_BASELINE_DIALECT;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_stdio_contract::editing;
use crate::editor::jpg_baseline::*;

/// 🧬️ Registers the document schema jpg's declaration contributes — the contract every snapshot edit validates against;
/// a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v_jfif_1_01::subsets::document::schema::jpg_artifact_schema_descriptor()]).expect("the jpg document schema registers");
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_jpg_baseline_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, JPG_BASELINE_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<JpgBaselineEditor as ArtifactEditor>::DIALECT, JPG_BASELINE_DIALECT);
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    register_document_schema();
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut snapshot = JpgSnapshot::default();
    snapshot.image.pixels = vec![7, 9];
    let base: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/image/pixels{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") {
            *from = format!("/image/pixels{}", from.as_str().unwrap()).into();
        }
        let event: editing::SnapshotEditEvent = semio_framework_pack_json::from_json_str(&event.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let emitted = <JpgBaselineEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::apply_diff_cold(<JpgMutation as protocol::ColdMutation<JpgSnapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/image/pixels").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}


fn paint_command(args: Vec<(&str, u64)>) -> JpgBaselineEditCommand {
    let number = |value| semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value));
    super::jpg_baseline_editor_command_from_action(paint_region::ACTION_ID, Some(&semio_framework_value::DslValue::object(args.into_iter().map(|(key, value)| (key.into(), number(value))).collect::<Vec<_>>()))).expect("typed paint-region action")
}

fn drive_paint(command: &JpgBaselineEditCommand, snapshot: &crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot) -> Result<Vec<crate::standards::v_jfif_1_01::subsets::document::schema::mutations::JpgMutation>, semio_framework_plugin::Fault> {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let config = semio_framework_plugin::NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "paint-region-test".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "paint-region-test".into(), retained: semio_s_artifact_stdio_contract::editing::ample_close_grant() };
    let mut work = paint_region::PaintRegionWork::new();
    assert!(work.extent(command, snapshot, &interaction, None).is_some());
    for _ in 0..4096 {
        let mut sequence = 0;
        let mut step_progress = semio_framework_value::retained_clone::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, semio_s_artifact_stdio_contract::testing::unbounded_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut step_progress);
        let step = work.step(&ArtifactCommandInputs { snapshot_owner: None, command, snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }, &mut cx);
        match step {
            Err(failure) => return Err(semio_framework_plugin::Fault::from(format!("{failure:?}"))),
            Ok(ArtifactCommandWorkStep::Progress { preview, .. }) => assert!(std::str::from_utf8(preview).unwrap().contains("\"de\"")),
            Ok(ArtifactCommandWorkStep::Complete(emit)) => return Ok(emit.artifact_mutations),
            Ok(_) => panic!("unexpected paint region work step"),
        }
    }
    panic!("paint region work did not complete")
}

fn raster(width: u32, height: u32) -> crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot {
    let mut snapshot = crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot::default();
    snapshot.image.width = width;
    snapshot.image.height = height;
    snapshot.image.pixels = [1u8, 2, 3, 255].repeat((width * height) as usize);
    snapshot
}

#[test]
fn paint_region_fills_one_rectangle_of_the_decoded_raster_and_undo_restores_it() {
    use protocol::{ColdMutation, Mutation, MutationDiff};
    let base = raster(4, 3);
    let command = paint_command(vec![("x", 1), ("y", 1), ("width", 2), ("height", 1), ("red", 10), ("green", 20), ("blue", 30), ("alpha", 128)]);
    let mutations = drive_paint(&command, &base).expect("region paint completes");
    assert_eq!(mutations.len(), 1);
    let edited = protocol::apply_diff_cold(mutations[0].diff(&base).diff(), &base).expect("paint applies");
    assert_eq!(&edited.image.pixels[20..28], &[10, 20, 30, 128, 10, 20, 30, 128]);
    assert_eq!(&edited.image.pixels[..20], &base.image.pixels[..20]);
    assert_eq!(&edited.image.pixels[28..], &base.image.pixels[28..]);
    let inverse = mutations[0].inverse(&base).expect("paint inverse");
    let restored = inverse.iter().fold(edited, |current, mutation| protocol::apply_diff_cold(mutation.diff(&current).diff(), &current).expect("undo applies"));
    assert_eq!(restored, base);
}

#[test]
fn paint_region_refuses_an_out_of_bounds_rectangle_without_mutating() {
    let base = raster(4, 3);
    for (x, y, width, height) in [(3, 0, 2, 1), (0, 2, 1, 2), (4, 0, 1, 1), (u32::MAX as u64, 0, 1, 1)] {
        let command = paint_command(vec![("x", x), ("y", y), ("width", width), ("height", height), ("red", 0), ("green", 0), ("blue", 0), ("alpha", 255)]);
        assert!(drive_paint(&command, &base).is_err(), "{x},{y} {width}x{height}");
    }
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", crate::editor::jpg_baseline::JpgBaselineEditor, || semio_framework_plugin::App { definition: crate::editor::jpg_baseline::create_jpg_baseline_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_framework_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let source = include_bytes!("../../../../🧾️document/📚️examples/🎬️demo/🖼️assets/🖼️.jpg");
    let codec = <JpgBaselineEditor as ArtifactEditor>::natural_file_codec().expect("paired natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.jpg@jfif-1.01", ".jpg", "image/jpeg", true));
    let artifact = MediaArtifact {
        descriptor: MediaArtifactDescriptor {
            edge_id: None,
            port_id: Some(NATURAL_FILE_PORT.into()),
            kind_id: Some(codec.format_kind.into()),
            media_type: None,
            wire: MediaWireFormat::Binary { format_kind: codec.format_kind.into() },
            blob_hash: None,
        },
        data: source.to_vec(),
    };
    let initial = <JpgBaselineEditor as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<JpgBaselineEditor>, _>(async { semio_framework_plugin::App { definition: create_jpg_baseline_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let mut outside = artifact.clone();
    outside.data.push(0x7f);
    app.consume_media(NATURAL_FILE_PORT, artifact, scope.cx()).await.expect("registered natural import");
    semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).expect("natural import publishes");
    let opened = app.snapshot().expect("opened snapshot").clone();
    let refused = match app.consume_media(NATURAL_FILE_PORT, outside, scope.cx()).await {
        Err(_) => true,
        Ok(_) => semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).is_err(),
    };
    assert!(refused);
    assert_eq!(app.snapshot().expect("refused baseline JPEG preserves the owner"), opened);
    let saved = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |native_owner| semio_framework_plugin::artifact_app_laws::produce_fixture_media(&mut app, NATURAL_FILE_PORT, native_owner, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_identity_round_trip(&saved.data).expect("image independently reopens and writes the baseline JPEG export");
    let observed = semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::project_jpg_mutation(&saved.data).expect("project baseline JPEG export");
    let expected = semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::project_jpg_mutation(&oracle).expect("project independent baseline JPEG output");
    assert_eq!(observed.get("width"), expected.get("width"));
    assert_eq!(observed.get("height"), expected.get("height"));
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut reopened = semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<JpgBaselineEditor>, _>(async { semio_framework_plugin::App { definition: create_jpg_baseline_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    reopened.bind_instance_id(2).await;
    reopened.consume_media(NATURAL_FILE_PORT, saved, scope.cx()).await.expect("fresh owner imports exported bytes");
    semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut reopened, 2, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).expect("fresh owner import publishes");
    let reopened_snapshot = reopened.snapshot().expect("reopened snapshot").clone();
    assert_eq!(reopened_snapshot, opened);
    semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "undo", 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().expect("source undo snapshot"), initial);
    assert_eq!(reopened.snapshot().expect("reopened snapshot remains isolated"), reopened_snapshot);
    semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "redo", 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().expect("source redo snapshot"), opened);
    semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut reopened, "undo", 2, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(reopened.snapshot().expect("reopened undo snapshot"), initial);
    assert_eq!(app.snapshot().expect("source remains isolated"), opened);
    semio_framework_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut reopened, "redo", 2, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(reopened.snapshot().expect("reopened redo snapshot"), reopened_snapshot);
    artifact_app_laws::close_registered_fixture_app(&mut reopened, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}
