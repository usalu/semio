
use super::*;
use crate::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifFrame, GifRgb};

#[semio_framework_async_macros::async_test]
async fn derives_bit_depth_from_global_color_table_size() {
    let gct = GifColorTable { sorted: false, colors: vec![GifRgb::default(); 4] };
    let snapshot = GifSnapshot { width: 3, height: 2, gct: Some(gct), ..GifSnapshot::default() };
    let dims = compute_gif_dimensions(&snapshot);
    assert_eq!(dims.bit_depth, 2);
    assert!(!dims.has_alpha);
}

#[semio_framework_async_macros::async_test]
async fn has_alpha_when_any_frame_declares_a_transparent_index() {
    let frame = GifFrame { transparent_index: Some(0), ..GifFrame::default() };
    let snapshot = GifSnapshot { frames: vec![frame], ..GifSnapshot::default() };
    assert!(compute_gif_dimensions(&snapshot).has_alpha);
}
