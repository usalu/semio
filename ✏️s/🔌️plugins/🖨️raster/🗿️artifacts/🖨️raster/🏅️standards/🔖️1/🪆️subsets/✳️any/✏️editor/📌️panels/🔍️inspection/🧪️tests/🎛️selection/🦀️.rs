//! 🧪️ Shared selection vectors projected into native semantic controls.
use super::*;

#[test]
fn inspector_projects_selected_properties_and_foreground_in_both_languages() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let document: RasterDocument = dsl::json::from_json_str(r#"{"schema":"raster.document","id":"inspection","title":"Inspection","layers":[{"kind":"pixel","id":"paint.foreground","name":"123","mask":null,"width":32,"height":32,"imageKey":null},{"kind":"pixel","id":"paint.background","name":"Background","mask":null,"width":64,"height":64,"imageKey":null}]}"#).unwrap();
    for labels in [&RasterPlayLabels::NATIVE_EN, &RasterPlayLabels::NATIVE_DE] {
        for case in fixture["cases"].as_array().unwrap() {
            let ids: Vec<String> = serde_json::from_value(case["selection"].clone()).unwrap();
            let node = render(&document, &RasterConfig::default(), &ids, labels).unwrap();
            let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
            let _: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert!(text.contains("setBrushColor"));
            assert!(text.contains(labels.foreground.as_str()));
            assert_eq!(text.contains("patchLayers"), !ids.is_empty());
            for field in case["fields"].as_array().unwrap() {
                assert!(text.contains(&format!("raster-inspector.{}.input", field.as_str().unwrap())), "{text}");
            }
            if ids.len() == 2 { assert!(text.contains(labels.mixed.as_str())); }
        }
    }
    crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot(document);
}
