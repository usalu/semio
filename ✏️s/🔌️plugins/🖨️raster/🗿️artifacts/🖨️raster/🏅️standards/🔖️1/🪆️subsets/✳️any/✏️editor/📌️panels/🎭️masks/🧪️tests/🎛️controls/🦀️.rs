//! 🎭️ Every attached mask remains controllable, including disabled masks nested in groups.
use super::*;

#[test]
fn mask_panel_keeps_disabled_masks_and_authors_semantic_controls() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️controls/🔣️.json")).unwrap();
    let document: RasterDocument = semio_framework_pack_json::from_json_str(&fixture["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for labels in [&RasterPlayLabels::NATIVE_EN, &RasterPlayLabels::NATIVE_DE] {
        let view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        let windows = TreeWindows::for_body(&view, RASTER_PLAY_BODY_MASKS);
        let tree = render(&document, &RasterConfig::default(), labels, &windows).unwrap();
        let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        let projection: serde_json::Value = serde_json::from_str(&text).unwrap();
        let mut nodes = Vec::new();
        let mut pending = vec![&projection];
        while let Some(node) = pending.pop() {
            nodes.push(node);
            pending.extend(node["children"].as_array().unwrap());
        }
        for expected in fixture["expected"].as_array().unwrap() {
            let id = expected["id"].as_str().unwrap();
            let mask_row = nodes.iter().find(|node| node["key"] == mask_row_id(id)).unwrap();
            assert_eq!(mask_row["children"].as_array().unwrap().len(), 2);
            assert!(mask_row["children"].as_array().unwrap().iter().all(|row| row["component"]["type"] == "treeItem"));
            for (field, property, label) in [("maskEnabled", "enabled", labels.mask_enabled), ("maskInvert", "invert", labels.mask_invert)] {
                let key = format!("{}.{}", mask_row_id(id), field);
                let node = nodes.iter().find(|node| node["key"] == key).unwrap_or_else(|| panic!("missing {key}"));
                assert_eq!(node["component"]["type"], "toggle");
                assert_eq!(node["component"]["on"], expected[property]);
                assert_eq!(node["bindings"][0]["trigger"], fixture["trigger"]);
                assert!(node["bindings"].to_string().contains(fixture["action"].as_str().unwrap()));
                assert!(node["bindings"].to_string().contains(id));
                assert!(node["bindings"].to_string().contains(field));
                assert_eq!(node["accessibility"]["label"], format!("{}: {}", expected["name"].as_str().unwrap(), label.as_str()));
            }
        }
        assert!(!nodes.iter().any(|node| node["key"] == mask_row_id("plain")));
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}
