
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_model_and_frames_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(model::REMODELING_PLAY_WINDOW_MAIN));
    assert!(json.contains(frames::REMODELING_PLAY_WINDOW_FRAMES));
}
