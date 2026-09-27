//! 📷️ Artifact-level framing shared by editor and viewer.
use super::*;
#[test]
fn drawing_scene_framing_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let nodes: Vec<crate::schema::DrawingSceneNode> = dsl::json::from_json_str(&case["nodes"].to_string()).unwrap();
        let artboard: Option<crate::DrawingArtboard> = dsl::json::from_json_str(&case["artboard"].to_string()).unwrap();
        let actual = drawing_scene_bounds(artboard.as_ref(),&nodes);
        for (index,value) in actual.iter().enumerate() { assert!((value-case["expected"][index].as_f64().unwrap()).abs()<1e-8,"{}: {actual:?}",case["name"]); }
    }
    eprintln!("[DEBUG] shared Drawing framing covers visible paths, transformed images, text, artboards and sheared strokes");
}
