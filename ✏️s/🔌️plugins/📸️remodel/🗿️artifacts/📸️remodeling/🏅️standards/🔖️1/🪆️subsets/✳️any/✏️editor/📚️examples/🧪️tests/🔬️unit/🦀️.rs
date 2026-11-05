use super::*;

#[semio_framework_async_macros::async_test]
async fn example_ids_are_unique_and_resolvable() {
    let ids: std::collections::BTreeSet<&str> = REMODELING_EXAMPLES.iter().map(|example| example.id).collect();
    assert_eq!(ids.len(), REMODELING_EXAMPLES.len(), "example ids must be unique");
    assert!(ids.contains(REMODELING_EXAMPLE_BOOT_ID), "the boot example must be registered");
}

#[semio_framework_async_macros::async_test]
async fn every_source_carries_its_committed_text() {
    let sources = example_sources();
    assert_eq!(sources.len(), REMODELING_EXAMPLES.len());
    for example in REMODELING_EXAMPLES {
        assert_eq!(example_text(example.id), Some(example.text));
    }
}

#[semio_framework_async_macros::async_test]
async fn the_boot_document_parses_the_demo_example() {
    assert_eq!(boot_snapshot(), semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(example_text(REMODELING_EXAMPLE_BOOT_ID).expect("boot example text")).expect("boot example parses"));
}
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, print_dsl};

#[semio_framework_async_macros::async_test]
async fn every_picker_snapshot_matches_the_shared_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️picker/🔣️.json")).expect("picker fixture");
    for vector in fixture["examples"].as_array().expect("examples") {
        let example_id = vector["id"].as_str().expect("example id");
        let snapshot = parse_dsl(example_text(example_id).expect("registered example")).expect("committed snapshot");
        let expected = &vector["expected"];
        assert_eq!(snapshot.id, expected["snapshotId"].as_str().expect("snapshot id"), "{example_id}");
        assert_eq!(snapshot.streams.iter().map(|stream| stream.id.as_str()).collect::<Vec<_>>(), expected["streamIds"].as_array().expect("streams").iter().map(|id| id.as_str().expect("stream id")).collect::<Vec<_>>(), "{example_id}");
        assert_eq!(snapshot.calibration.cameras.iter().map(|camera| camera.id.as_str()).collect::<Vec<_>>(), expected["cameraIds"].as_array().expect("cameras").iter().map(|id| id.as_str().expect("camera id")).collect::<Vec<_>>(), "{example_id}");
        assert_eq!(snapshot.streams.iter().map(|stream| stream.frames.len()).sum::<usize>(), expected["frames"].as_u64().expect("frame count") as usize, "{example_id}");
        assert_eq!(snapshot.results.mesh.mesh.child_id, expected["meshChildId"].as_str().expect("mesh child"), "{example_id}");
        assert_eq!(snapshot.results.mesh.source, semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MeshSource::Placeholder);
        assert_eq!(parse_dsl(&print_dsl(&snapshot)).expect("canonical snapshot"), snapshot, "{example_id}");
    }
}
