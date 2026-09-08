
use super::*;
use crate::editor::puzzle2d::testkit::*;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn document_panel_lists_nodes_section() {
    let mut app = concrete_forest_app();
    let json = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(json.contains("puzzle2d-play-document.nodes"));
    assert!(json.contains("seed-left-001"));
}

/// 🗣️ B1: locale/terminology are now real VCS'd `Puzzle2dConfig` state (was a per-call `ViewModel`
/// override) — dispatch `setLocale`/`setTerminology` to change them, then render.
#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_and_reuse() {
    let mut app = concrete_forest_app();
    let english = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(english.contains("\"Nodes\"") && english.contains("\"Edges\""));
    dispatch(&mut app, "setLocale", Some(&json!({ "value": "de" })), None).expect("setLocale");
    let german = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(german.contains("\"Knoten\"") && german.contains("\"Kanten\""));
    dispatch(&mut app, "setLocale", Some(&json!({ "value": "en" })), None).expect("setLocale");
    dispatch(&mut app, "setTerminology", Some(&json!({ "value": "reuse" })), None).expect("setTerminology");
    let reuse = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(reuse.contains("Building components"));
}
