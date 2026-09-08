
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_every_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    for id in [pool::SOURCING_CURATION_WINDOW_POOL, curated::SOURCING_CURATION_WINDOW_CURATED, preview::SOURCING_CURATION_WINDOW_PREVIEW, grid::SOURCING_CURATION_WINDOW_GRID] {
        assert!(json.contains(id), "layout must reference window kind {id}: {json}");
    }
}
