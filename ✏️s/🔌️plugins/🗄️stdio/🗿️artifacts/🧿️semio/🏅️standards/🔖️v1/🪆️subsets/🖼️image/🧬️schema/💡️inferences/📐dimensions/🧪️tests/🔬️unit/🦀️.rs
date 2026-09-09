use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(width: u32, height: u32, colorspace: SemioColorspace, bit_depth: u8, frame_count: usize) -> SemioImageSnapshot {
    SemioImageSnapshot {
        schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
        width,
        height,
        colorspace,
        bit_depth,
        frames: (0..frame_count).map(|_| SemioImageFrame { delay_ms: 0, rgba8: vec![0; (width * height * 4) as usize] }).collect(),
        icc: None,
        metadata: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn derives_from_header_fields() {
    let dimensions = compute_semio_image_dimensions(&snapshot(4, 3, SemioColorspace::Rgba, 8, 2));
    assert_eq!(dimensions, SemioImageDimensions { width: 4, height: 3, bit_depth: 8, has_alpha: true, pixel_count: 12, frame_count: 2 });
}

#[semio_framework_async_macros::async_test]
async fn rgb_has_no_alpha() {
    let dimensions = compute_semio_image_dimensions(&snapshot(2, 2, SemioColorspace::Rgb, 8, 1));
    assert!(!dimensions.has_alpha);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot(5, 5, SemioColorspace::Grayscale, 8, 3);
    assert_eq!(compute_semio_image_dimensions(&snapshot), compute_semio_image_dimensions(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_image_dimensions(&SemioImageSnapshot::default()), SemioImageDimensions::default());
}
