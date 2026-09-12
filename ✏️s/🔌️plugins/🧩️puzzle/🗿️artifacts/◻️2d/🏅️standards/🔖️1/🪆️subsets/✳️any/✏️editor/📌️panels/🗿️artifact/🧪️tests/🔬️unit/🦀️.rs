use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;

#[semio_framework_async_macros::async_test]
async fn document_panel_lists_nodes_section() {
    let mut app = concrete_forest_app();
    let json = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(json.contains("puzzle2d-play-document.nodes"));
    assert!(json.contains("seed-left-001"));
}

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_and_reuse() {
    let mut app = concrete_forest_app();
    let english = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(english.contains("\"Nodes\"") && english.contains("\"Edges\""));
    let german_view = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let german = render_body_with_view(&mut app, PUZZLE2D_PLAY_BODY_LAYERS, &german_view);
    assert!(german.contains("\"Knoten\"") && german.contains("\"Kanten\""));
    let reuse_view = semio_framework_plugin::ViewModel { terminology: semio_framework_plugin::Terminology::Reuse, ..Default::default() };
    let reuse = render_body_with_view(&mut app, PUZZLE2D_PLAY_BODY_LAYERS, &reuse_view);
    assert!(reuse.contains("Building components"));
}
