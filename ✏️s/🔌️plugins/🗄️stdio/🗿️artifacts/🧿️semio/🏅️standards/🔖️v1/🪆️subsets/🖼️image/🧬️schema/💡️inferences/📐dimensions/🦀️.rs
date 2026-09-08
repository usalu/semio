//! 📐 `dimensions` — one named inference: the semio image's header geometry, a pure O(frames)
//! read of already-decoded snapshot fields — nothing here is per-entity/incremental (`frameCount`
//! is a single length read, not a fold over per-frame content), so this holds only the value type
//! + its pure `compute` fn (no `InferredField`).

use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};

//#region 🔖️Dimensions
/// 📐️ Semio image header-derived raster geometry. `has_alpha` reads the explicit `colorspace`
/// enum (`Rgba`/`GrayscaleAlpha` carry alpha, `Rgb`/`Grayscale`/`Indexed` do not) — no heuristic
/// needed, `SemioColorspace` already names it.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
