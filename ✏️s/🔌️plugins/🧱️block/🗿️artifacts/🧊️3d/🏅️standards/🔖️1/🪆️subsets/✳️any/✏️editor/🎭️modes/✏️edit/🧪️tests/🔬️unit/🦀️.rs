
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_world_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(world::BLOCK3D_WINDOW_WORLD), "layout must reference the world window kind: {json}");
}
