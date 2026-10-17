//! 🔍️ Feature detection, binary description, matching and optical flow: oriented FAST, rBRIEF, Hamming matching and pyramidal Lucas-Kanade.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;

use remodeling_image::{build_pyramid, extract_patch, gaussian_blur, scharr_gradients, zncc, GradientField, ImageGray, Pyramid};

// #region 🔖️Keypoint
/// 📍️ A detected image feature: subpixel position, source pyramid octave, dominant orientation in radians (intensity-centroid convention), and detector response/score.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Keypoint {
    pub x: f32,
    pub y: f32,
    pub octave: u8,
    pub angle: f32,
    pub response: f32,
}
// #endregion 🔖️Keypoint

#[path = "🔎️detectors/🦀️.rs"]
mod detectors;
pub use detectors::*;

#[path = "🧬️descriptors-matching-and-flow/🦀️.rs"]
mod descriptors_matching_and_flow;
pub use descriptors_matching_and_flow::*;

#[path = "🌀️akaze-scale-space/🦀️.rs"]
mod akaze_scale_space;
pub use akaze_scale_space::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
