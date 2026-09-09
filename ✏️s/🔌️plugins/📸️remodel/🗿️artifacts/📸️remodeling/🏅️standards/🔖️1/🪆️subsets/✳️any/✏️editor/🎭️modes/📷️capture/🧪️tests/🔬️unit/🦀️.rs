use super::*;

#[semio_framework_async_macros::async_test]
async fn the_capture_layout_puts_the_filmstrip_first() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(REMODELING_PLAY_LAYOUT_CAPTURE));
    assert!(json.contains(frames::REMODELING_PLAY_WINDOW_FRAMES));
}
