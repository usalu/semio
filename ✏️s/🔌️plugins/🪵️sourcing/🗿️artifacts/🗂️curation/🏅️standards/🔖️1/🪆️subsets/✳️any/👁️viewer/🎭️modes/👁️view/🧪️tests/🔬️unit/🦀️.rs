
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_layout_references_the_pool_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(pool::WINDOW_KIND_ID), "layout must reference window kind {}: {json}", pool::WINDOW_KIND_ID);
}
