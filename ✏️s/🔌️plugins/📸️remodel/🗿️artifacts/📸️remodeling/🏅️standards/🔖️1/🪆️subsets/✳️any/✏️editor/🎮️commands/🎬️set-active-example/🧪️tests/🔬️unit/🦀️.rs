use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::set_active_example::*;
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::{example_text, REMODELING_EXAMPLES};
use crate::editor::remodeling::unit_tests::context::{app_with_registry, dispatch};
use semio_s_artifact_remodel_remodeling_editor::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn every_registered_example_id_resolves_to_committed_text() {
    assert!(!REMODELING_EXAMPLES.is_empty(), "the example registry must not be empty");
    for example in REMODELING_EXAMPLES {
        assert!(example_text(example.id).is_some(), "example '{}' must resolve", example.id);
        assert!(!example.text.is_empty(), "example '{}' must carry committed text", example.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_example_id_is_a_no_op() {
    let mut app = app_with_registry().await;
    let before = app.snapshot().expect("snapshot before");
    let dispatched = dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: "nonsense".into() })).await;
    assert!(!dispatched.edited_document(), "an unknown id writes nothing: {:?}", dispatched.lanes);
    assert_eq!(before, app.snapshot().expect("snapshot after"));
}

#[semio_framework_async_macros::async_test]
async fn re_selecting_the_boot_example_writes_no_edit() {
    let mut app = app_with_registry().await;
    let before = app.snapshot().expect("snapshot before");
    let dispatched = dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::REMODELING_EXAMPLE_BOOT_ID.into() })).await;
    assert!(!dispatched.edited_document(), "the boot document already equals the boot example: {:?}", dispatched.lanes);
    assert_eq!(before, app.snapshot().expect("snapshot after"));
}

#[semio_framework_async_macros::async_test]
async fn selecting_another_example_loads_it_without_an_edit() {
    let mut app = app_with_registry().await;
    let dispatched = dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::ID.into() })).await;
    assert!(!dispatched.edited_document(), "an example switch is a load, never an edit: {:?}", dispatched.lanes);
    let loaded = app.snapshot().expect("snapshot after");
    assert_eq!(loaded.assets.len(), semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::frames().count(), "every committed frame loads as asset content");
    assert!(!loaded.streams.is_empty(), "the example's streams load with it");
    for &(asset_id, committed) in semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::frames() {
        let source = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_asset_chunk_source(&loaded, asset_id).expect("loaded durable frame");
        assert_eq!(source.mime, "image/png");
        let bytes: Vec<u8> = source.leaves.iter().flat_map(|leaf| leaf.iter().copied()).collect();
        assert_eq!(bytes.as_slice(), committed, "{asset_id}: durable frame matches committed bytes");
        let mut reader = png::Decoder::new(std::io::Cursor::new(committed)).read_info().expect("independent PNG header");
        let mut pixels = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pixels).expect("independent PNG pixels");
        assert_eq!(info.color_type, png::ColorType::Rgba);
        assert_eq!(info.bit_depth, png::BitDepth::Eight);
        assert!(info.width > 0 && info.height > 0);
        assert_eq!((source.width, source.height), (info.width, info.height), "{asset_id}: durable dimensions");
        let decoded = semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::decode_still_image(&source.mime, &bytes).expect("native loaded frame decode");
        assert_eq!((decoded.width, decoded.height), (info.width, info.height), "{asset_id}: decoded dimensions");
        assert_eq!(decoded.data.as_slice(), &pixels[..info.buffer_size()], "{asset_id}: independently decoded pixels");
    }
}

#[semio_framework_async_macros::async_test]
async fn shared_picker_switches_load_distinct_snapshots_without_history_edits() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📚️examples/🧪️tests/🧫️picker/🔣️.json")).expect("picker fixture");
    for vector in fixture["switches"].as_array().expect("switches") {
        let mut app = app_with_registry().await;
        let from = vector["from"].as_str().expect("from");
        let to = vector["to"].as_str().expect("to");
        dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: from.into() })).await;
        let before = app.snapshot().expect("source snapshot");
        let source_history = app.history_snapshot().await.expect("source history");
        assert_eq!(source_history.edit_count, 0, "loaded example starts without edits");
        let dispatched = dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: to.into() })).await;
        assert!(!dispatched.edited_document(), "picker switch {from} → {to}: {:?}", dispatched.lanes);
        let history = app.history_snapshot().await.expect("selected history");
        assert_eq!(u64::from(history.edit_count), vector["historyEdits"].as_u64().expect("expected history edits"));
        assert_eq!(history.edit_count, source_history.edit_count, "picker creates no edit");
        assert!(!history.can_undo && !history.can_redo, "loaded example has no edit steps");
        let loaded = app.snapshot().expect("selected snapshot");
        assert_ne!(loaded, before, "picker must load {to}");
        let expected = fixture["examples"].as_array().expect("examples").iter().find(|example| example["id"].as_str() == Some(to)).expect("selected contract");
        assert_eq!(loaded.streams.iter().map(|stream| stream.id.as_str()).collect::<Vec<_>>(), expected["expected"]["streamIds"].as_array().expect("stream ids").iter().map(|id| id.as_str().expect("stream id")).collect::<Vec<_>>());
        assert_eq!(loaded.calibration.cameras.len(), expected["expected"]["cameraIds"].as_array().expect("camera ids").len());
        if to != "synthetic-orbit" {
            assert!(loaded.assets.is_empty(), "prior example media must be replaced");
            assert!(loaded.durable_artifacts.is_empty(), "prior example content must be replaced");
        }
    }
}
