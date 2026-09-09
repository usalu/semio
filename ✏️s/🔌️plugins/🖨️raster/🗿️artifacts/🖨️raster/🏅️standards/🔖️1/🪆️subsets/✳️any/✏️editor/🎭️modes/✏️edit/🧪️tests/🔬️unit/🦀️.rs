use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let json = dsl::os_pack::json::to_json_string(&layout());
    assert!(json.contains(composite::RASTER_PLAY_WINDOW_COMPOSITE) && json.contains(navigator::RASTER_PLAY_WINDOW_NAVIGATOR), "layout must reference both window kinds: {json}");
}
