
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_analyze_layout_pairs_the_model_window_with_the_report() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(REMODELING_PLAY_LAYOUT_ANALYZE));
    assert!(json.contains(report::REMODELING_PLAY_WINDOW_REPORT));
}
