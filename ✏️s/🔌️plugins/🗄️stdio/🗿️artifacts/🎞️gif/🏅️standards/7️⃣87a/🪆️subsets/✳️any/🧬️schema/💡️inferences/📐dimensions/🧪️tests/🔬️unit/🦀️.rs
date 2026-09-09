use super::*;
use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifRgb};

#[semio_framework_async_macros::async_test]
async fn derives_bit_depth_from_global_color_table_size() {
    let gct = GifColorTable { sorted: false, colors: vec![GifRgb::default(); 4] };
    let snapshot = GifSnapshot { width: 3, height: 2, gct: Some(gct), ..GifSnapshot::default() };
    assert_eq!(compute_gif_dimensions(&snapshot), GifDimensions { width: 3, height: 2, bit_depth: 2, has_alpha: false, pixel_count: 6 });
}

#[semio_framework_async_macros::async_test]
async fn never_reports_alpha() {
    assert!(!compute_gif_dimensions(&GifSnapshot::default()).has_alpha);
}
