//! 🕸️ Surface extraction and texturing: marching cubes, quadric simplification, hole filling, UV atlas and multi-view texture blending.
//!
//! This crate's one non-negotiable contract: [`mesh_pipeline_step`] always terminates in either
//! `Done` with [`WatertightReport::is_watertight`] `true`, or an explicit `Failed`. There is no
//! silent "good enough" mesh — [`close_voxel`] is a by-construction closed-2-manifold fallback
//! that the pipeline falls back to whenever repair/hole-filling leaves defects behind.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::SimplifyParams;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::TextureView;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::bake_texture;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::segment_charts;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::simplify_qem;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::unwrap_mesh;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedCleanPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedEdgePreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedHoleFillPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedOrientPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedRepairPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::HoleFillParams;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::WatertightReport;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::clean_mesh;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::close_voxel;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::fill_holes;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::orient_consistently;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::orient_outward;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::repair_non_manifold;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::taubin_smooth;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::triangles_intersect;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::validate_watertight;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TSDF_EXTRACTION_POINTS_PER_DISPATCH;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TriMesh;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TsdfExtractionPreparation;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::add3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::cross3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::dot3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::extract_tsdf;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::norm3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::normalize3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::scale3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sub3;

#[path = "📤️interchange-preparations/🦀️.rs"]
mod interchange_preparations;
use interchange_preparations::*;

#[path = "✅️bounded-validation/🦀️.rs"]
mod bounded_validation;
use bounded_validation::*;

#[path = "🪄️bounded-postprocess/🦀️.rs"]
mod bounded_postprocess;
use bounded_postprocess::*;

#[path = "🏭️pipeline/🦀️.rs"]
mod pipeline;
pub use pipeline::*;

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
