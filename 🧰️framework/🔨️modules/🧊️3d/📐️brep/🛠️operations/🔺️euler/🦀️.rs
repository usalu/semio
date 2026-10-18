//! ✂️ Checked topology editors (the *only* functions permitted to mutate a `Body`) plus UV planar
//! arrangement and face split via those editors (imprint). Shares one compute subdir per the
//! `✂️intersect`-style "one compute subdir, not a 1:1 file mapping" precedent this ticket's wave
//! PEEL established — no dedicated `🖋️imprint` facet was pre-mounted, and the primitive editors
//! and the one operation built on them belong together. Framework-3d's own `🔺️euler` module
//! (`make_vertex`/`make_edge`/`make_loop`/`add_face`/`add_shell`/`add_solid`/`split_edge`) moved
//! in HERE, alongside the imprint code already resident from wave PEEL, in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3.
//!
//! Minimum viable imprint: cut a planar face with a line through two points, splitting the outer
//! loop into two faces that share a new chord edge. Lane 3-imprint of ticket
//! `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`. Moved from
//! `🧰️framework/🔨️modules/🧊️3d/📐️brep/🖋️imprint` (imprint) and
//! `🧰️framework/🔨️modules/🧊️3d/📐️brep/🔺️euler` (editors) in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, waves PEEL and PEEL3.
/// 🔗 Oriented edge with optional parameter curve and its range.
pub type ParametricEdge = (EdgeId, bool, Option<Curve2Id>, (f64, f64));

/// 🚶 Ordered vertices and oriented edges around a loop.
pub type LoopWalk = (Vec<VertexId>, Vec<(EdgeId, bool)>);

use semio_framework_3d_brep_representation::arena::{ArenaId, CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, SurfaceId, VertexId};
use semio_framework_3d_brep_curve::curve_ops::closest_parameter;
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_representation::error::KernelError;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::{dummy_coedge, make_loop, Body, Coedge, Edge, Face, Shell, Solid, Vertex};
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec3};

#[path = "🏗️topology-editors/🦀️.rs"]
mod topology_editors;
pub use topology_editors::{add_face, add_shell, add_solid, make_edge, make_vertex, retire_solid_scaffold, split_edge, split_edge_at_params, split_edge_with_vertex, split_planar_face_by_line};

#[path = "🪡️boundary-splice/🦀️.rs"]
mod boundary_splice;
use boundary_splice::{chain_endpoint_uv, member_uv, oriented_endpoints, ring_index_at_uv, splice_boundary_vertex_at_uv};
pub use boundary_splice::splice_boundary_vertex;

#[path = "⛓️face-chain-splits/🦀️.rs"]
mod face_chain_splits;
pub use face_chain_splits::{split_face_by_chain, split_face_by_interior_chain, split_face_by_interior_curve};

#[path = "🪚️seam-crossing-and-edge-merge/🦀️.rs"]
mod seam_crossing_and_edge_merge;
pub use seam_crossing_and_edge_merge::{kill_edge_merge_faces, split_face_by_seam_crossing};

#[path = "🗂️uv-arrangement/🦀️.rs"]
mod uv_arrangement;
use uv_arrangement::{BoundaryHit, edge_param_at_point, endpoint_vertex, intersect_segment_line_uv, loop_walk, loop_walk_pc, make_loop_pc, member_chain_pc, project_uv, resolve_edge_containing_param};

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
