//! 🔬️ Language-agnostic PersistenceDataClass fixture — Rust runner.

use super::{bindings_data_class, wire_lane_data_class, PersistenceBinding, PersistenceDataClass};

fn fixture() -> serde_json::Value {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest.join("../../🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json"),
        manifest.join("../../../🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json"),
        workspace_fixture(),
    ];
    for candidate in candidates {
        if let Ok(text) = std::fs::read_to_string(&candidate) {
            return serde_json::from_str(&text).expect("fixture json");
        }
    }
    panic!("persistence-data-class fixture not found from {manifest:?}");
}

fn workspace_fixture() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        let candidate = ancestor.join("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json");
        if candidate.is_file() {
            return candidate;
        }
    }
    manifest.join("missing-persistence-data-class-fixture.json")
}

#[test]
fn persistence_data_class_fixture_routes_bindings_and_lanes() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let expected = case["expectedDataClass"].as_str().expect("expected");
        let binding = &case["binding"];
        let kind = binding["kind"].as_str().expect("kind");
        let class = match kind {
            "folder" => PersistenceBinding::Folder { path: binding["path"].as_str().unwrap().into() }.data_class(),
            "hub" => PersistenceBinding::Hub {
                base_url: binding["baseUrl"].as_str().unwrap().into(),
                space_id: binding["spaceId"].as_str().unwrap().into(),
                surface: None,
            }
            .data_class(),
            "ephemeral" => {
                let data = binding["dataClass"].as_str().unwrap();
                if data == "ephemeralShared" {
                    PersistenceDataClass::EphemeralShared
                } else {
                    PersistenceDataClass::EphemeralLocalOnly
                }
            }
            other => panic!("unknown kind {other}"),
        };
        assert_eq!(class.as_str(), expected, "case {}", case["id"]);
        assert_eq!(class.allows_share(), case["allowsShare"].as_bool().unwrap());
        assert_eq!(class.allows_collaboration(), case["allowsCollaboration"].as_bool().unwrap());
        assert_eq!(class.is_durable(), case["durable"].as_bool().unwrap());
        if let Some(lane) = case.get("wireLane") {
            let lane_name = lane["lane"].as_str().unwrap();
            assert_eq!(wire_lane_data_class(lane_name).as_str(), lane["dataClass"].as_str().unwrap());
            assert!(!wire_lane_data_class(lane_name).is_durable(), "preview/presence must never be durable");
        }
    }
    assert_eq!(bindings_data_class(&[]), PersistenceDataClass::EphemeralLocalOnly);
    let hub = PersistenceBinding::Hub { base_url: "http://h".into(), space_id: "s".into(), surface: None };
    assert_eq!(bindings_data_class(&[hub]), PersistenceDataClass::PersistedShared);
}
