use crate::editor::puzzle2d::config::Puzzle2dConfig;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::unit_tests::context::*;
use semio_framework_plugin::{PluginApp, ViewModel, WindowMeasure};
use serde_json::json;

fn rendered_fill_count(app: &mut Puzzle2dApp) -> f64 {
    let measures = semio_framework::io::resolve_ready(app.tool_measures(&ViewModel::default()));
    let Some([WindowMeasure::Group { children, .. }]) = measures.get(fill::TOOL_ID).map(Vec::as_slice) else { panic!("fill tool measure group") };
    let Some(WindowMeasure::Number { value, .. }) = children.first() else { panic!("fill count number") };
    *value
}

/// 🧮️ `setFillCount` publishes the count into the shared config the tool measure renders — a count far past
/// any former pin is carried verbatim, the document stays untouched, and malformed counts change nothing.
#[test]
fn set_fill_count_publishes_the_config_count_and_never_the_document() {
    let mut app = app_with_registry();
    assert_eq!(rendered_fill_count(&mut app), 100.0);
    assert_eq!(Puzzle2dConfig::default().fill_count, 100);
    let before = fixture_of(&app);
    let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 5000 })), None).expect("set fill count");
    assert!(result.mutations.is_empty(), "the count is config, never a document mutation");
    assert_eq!(fixture_of(&app), before);
    assert_eq!(rendered_fill_count(&mut app), 5000.0);
    for malformed in [json!({ "value": -1 }), json!({ "count": "NaN" }), json!({}), json!({ "value": 4_294_967_296_u64 })] {
        dispatch(&mut app, "setFillCount", Some(&malformed), None).expect("malformed count is a no-op");
        assert_eq!(rendered_fill_count(&mut app), 5000.0, "{malformed} must not change the count");
    }
    dispatch(&mut app, "setFillCount", Some(&json!({ "count": 0 })), None).expect("zero count");
    assert_eq!(rendered_fill_count(&mut app), 0.0);
    close_app(&mut app);
}
