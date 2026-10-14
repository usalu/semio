use super::*;

#[semio_framework_async_macros::async_test]
async fn the_view_layout_lists_both_viewer_windows() {
    let json = semio_framework_pack_json::to_json_string(&layout());
    assert!(json.contains(crate::modes::view::windows::composite::RASTER_VIEW_WINDOW_COMPOSITE) && json.contains(crate::modes::view::windows::navigator::RASTER_VIEW_WINDOW_NAVIGATOR), "layout must reference both window kinds: {json}");
}
