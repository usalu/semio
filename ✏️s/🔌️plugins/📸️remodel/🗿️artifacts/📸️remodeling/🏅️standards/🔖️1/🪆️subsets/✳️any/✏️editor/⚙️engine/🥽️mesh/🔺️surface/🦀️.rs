//! 🔺️ Mesh surface: the indexed triangle mesh, its half-edge topology and iso-surface extraction from a TSDF volume or a dense field.

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[path = "📐️trimesh-and-vectors/🦀️.rs"]
mod trimesh_and_vectors;
pub use trimesh_and_vectors::*;

#[path = "🕸️halfedge-topology/🦀️.rs"]
mod halfedge_topology;
pub use halfedge_topology::*;

#[path = "🧊️iso-surface-extraction/🦀️.rs"]
mod iso_surface_extraction;
pub use iso_surface_extraction::*;
