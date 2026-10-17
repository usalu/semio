//! 🩹️ Mesh repair: cleaning, non-manifold repair, consistent orientation, hole filling, the closed voxel fallback and watertightness validation.

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::DenseField;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::DisjointSet;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::EdgeMap;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::HalfedgeTopology;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TriMesh;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::add3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::cross3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::dot3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::extract_dense_grid;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::lerp3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::norm3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::normalize3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::scale3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sorted_edge;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sub3;

#[path = "⏱️bounded-preparations/🦀️.rs"]
mod bounded_preparations;
pub use bounded_preparations::*;

#[path = "🧼️clean-and-repair/🦀️.rs"]
mod clean_and_repair;
pub use clean_and_repair::*;

#[path = "🕳️hole-filling/🦀️.rs"]
mod hole_filling;
pub use hole_filling::*;

#[path = "🧊️voxel-close-and-validate/🦀️.rs"]
mod voxel_close_and_validate;
pub use voxel_close_and_validate::*;
