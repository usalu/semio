//! 📐 `dimensions` — one named inference: the semio image's header geometry, a pure O(frames)
//! read of already-decoded snapshot fields — nothing here is per-entity/incremental (`frameCount`
//! is a single length read, not a fold over per-frame content), so this holds only the value type
//! + its pure `compute` fn (no `InferredField`).

use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Dimensions
/// 📐️ Semio image header-derived raster geometry. `has_alpha` reads the explicit `colorspace`
/// enum (`Rgba`/`GrayscaleAlpha` carry alpha, `Rgb`/`Grayscale`/`Indexed` do not) — no heuristic
/// needed, `SemioColorspace` already names it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioImageDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub has_alpha: bool,
    pub pixel_count: u64,
    pub frame_count: u32,
}

/// 📐️ Computes [`SemioImageDimensions`] from a snapshot's header fields — pure, total,
/// O(frames) only for the length read.
pub fn compute_semio_image_dimensions(snapshot: &SemioImageSnapshot) -> SemioImageDimensions {
    SemioImageDimensions {
        width: snapshot.width,
        height: snapshot.height,
        bit_depth: snapshot.bit_depth,
        has_alpha: matches!(snapshot.colorspace, SemioColorspace::Rgba | SemioColorspace::GrayscaleAlpha),
        pixel_count: snapshot.width as u64 * snapshot.height as u64,
        frame_count: snapshot.frames.len() as u32,
    }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};

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

    #[test]
    fn derives_from_header_fields() {
        let dimensions = compute_semio_image_dimensions(&snapshot(4, 3, SemioColorspace::Rgba, 8, 2));
        assert_eq!(dimensions, SemioImageDimensions { width: 4, height: 3, bit_depth: 8, has_alpha: true, pixel_count: 12, frame_count: 2 });
    }

    #[test]
    fn rgb_has_no_alpha() {
        let dimensions = compute_semio_image_dimensions(&snapshot(2, 2, SemioColorspace::Rgb, 8, 1));
        assert!(!dimensions.has_alpha);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = snapshot(5, 5, SemioColorspace::Grayscale, 8, 3);
        assert_eq!(compute_semio_image_dimensions(&snapshot), compute_semio_image_dimensions(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_semio_image_dimensions(&SemioImageSnapshot::default()), SemioImageDimensions::default());
    }
}
//#endregion 🧪️Tests
