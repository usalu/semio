//! 🌫️ Dense reconstruction: PatchMatch multi-view stereo, depth fusion, TSDF volumes and point-cloud analysis.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).
use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;

use std::collections::{BTreeMap, HashMap};

use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::jacobi_eigen_symmetric;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD;
use semio_s_artifact_remodel_remodeling_spatial::standards::v1::subsets::any::schema::spatial_internals::KdTree;

const MAX_INTERACTIVE_FUSED_POINTS: usize = 512;

// #region 🔖️PointCloud
/// 🏷️ Per-point semantic classification label produced by ground/planar segmentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum PointClass {
    Unclassified,
    Ground,
    Vegetation,
    Building,
    Noise,
}

/// ☁️ 3D point cloud as struct-of-arrays: `normals`/`colors`/`confidence`/`classification` are each
/// either empty (attribute unset) or exactly `positions.len()` entries, index-aligned with
/// `positions`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct PointCloud {
    pub positions: Vec<[f64; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[u8; 3]>,
    pub confidence: Vec<f32>,
    pub classification: Vec<PointClass>,
}

/// 🔁️ Widens a stored `f32` normal to `f64` for arithmetic.
fn to_f64_3(n: [f32; 3]) -> [f64; 3] {
    [f64::from(n[0]), f64::from(n[1]), f64::from(n[2])]
}

impl PointCloud {
    /// ☁️ Empty point cloud with no optional attributes.
    pub fn new() -> Self {
        Self::default()
    }

    /// ☁️ Point cloud built from bare positions, with every optional attribute unset.
    pub fn from_positions(positions: Vec<[f64; 3]>) -> Self {
        Self { positions, ..Self::default() }
    }

    /// 🔢️ Number of points.
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// 🔢️ Whether the cloud holds no points.
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
}
// #endregion 🔖️PointCloud

// #region 🔖️DepthMap
/// 🌊️ Per-pixel depth, normal and confidence buffers for one reference view, row-major with pixel
/// `(x, y)` at `data[y * width + x]`.
///
/// Invalid-sentinel convention: a depth `<= 0.0` (including the zero-fill from [`DepthMap::new`])
/// or a non-finite value marks a pixel as having no valid estimate. [`DepthMap::get`] returns
/// `None` for such pixels as well as for out-of-bounds coordinates; `normal`/`confidence` at
/// invalid pixels carry no meaning and callers should gate on `get` first.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct DepthMap {
    pub width: u32,
    pub height: u32,
    pub depth: Vec<f32>,
    pub normal: Vec<[f32; 3]>,
    pub confidence: Vec<f32>,
}

impl DepthMap {
    /// 🌊️ Zero-filled depth map (every pixel invalid) of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        let n = (width as usize) * (height as usize);
        Self { width, height, depth: vec![0.0; n], normal: vec![[0.0; 3]; n], confidence: vec![0.0; n] }
    }

    /// 🔍️ Valid depth at `(x, y)`, or `None` if out of bounds or the pixel carries the invalid
    /// sentinel (`<= 0` or non-finite).
    pub fn get(&self, x: u32, y: u32) -> Option<f32> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let d = self.depth[(y * self.width + x) as usize];
        (d > 0.0 && d.is_finite()).then_some(d)
    }
}

fn depthmap_index(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

/// 🚫️ Resets one pixel to the invalid sentinel across all three buffers.
fn depthmap_invalidate(map: &mut DepthMap, idx: usize) {
    map.depth[idx] = 0.0;
    map.normal[idx] = [0.0; 3];
    map.confidence[idx] = 0.0;
}
// #endregion 🔖️DepthMap

#[path = "🧩️patch-match-stereo/🦀️.rs"]
mod patch_match_stereo;
pub use patch_match_stereo::*;

#[path = "🧊️fusion-and-tsdf/🦀️.rs"]
mod fusion_and_tsdf;
pub use fusion_and_tsdf::*;

#[path = "☁️point-cloud-analysis/🦀️.rs"]
mod point_cloud_analysis;
pub use point_cloud_analysis::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
