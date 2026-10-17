//! 📷️ Artifact-level framing shared by editor and viewer.
use super::*;
use semio_framework_test_fixtures::fixture_text;
#[test]
fn drawing_scene_framing_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️framing/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let nodes: Vec<semio_s_artifact_draw_drawing_model::DrawingSceneNode> = semio_framework_pack_json::from_json_str(&case["nodes"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let artboard: Option<semio_s_artifact_draw_drawing_model::DrawingArtboard> = semio_framework_pack_json::from_json_str(&case["artboard"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let actual = drawing_scene_bounds(artboard.as_ref(),&nodes);
        for (index,value) in actual.iter().enumerate() { assert!((value-case["expected"][index].as_f64().unwrap()).abs()<1e-8,"{}: {actual:?}",case["name"]); }
    }
}
