//! 🧩 Crack-free, seam/pole-aware, error-controlled tessellation producing [`MeshTransfer`].
//!
//! Edges are discretized exactly once (chordal + angular deviation bounded) and keyed by
//! [`EdgeId`], so every adjacent face reuses bit-identical 3D boundary samples (crack-free).
//! Each face's UV-domain boundary (from the coedge's stored p-curve when present, else surface
//! closest-point projection with periodic-seam unwrapping) is triangulated with a constrained
//! Delaunay triangulation (Bowyer–Watson incremental insertion + edge-flip constraint recovery),
//! then adaptively refined with interior Steiner points until every triangle's chordal and
//! angular deviation from the true surface is within tolerance. Coincident 3D vertices (poles,
//! cone apexes) are welded post-triangulation, collapsing the surrounding ring into a fan.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧩️tessellate` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL, then rewritten
//! for ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave W1-G (see that ticket's
//! `📓️w1g-tessellation.md`) to consume `⚙️engine/🔖️contract`'s `MeshTransfer` (W1-A) — its
//! `edge_groups`/`face_infos`/`edge_infos` fields are now filled directly. Pure algorithm only —
//! the real `InferredField<SemioBrepSnapshot>` wrapper is future work (see
//! `💡️inferences/✅validation-report`'s doc comment for why it's not built yet).
//!
//! Crate cut of `semio-framework-3d-brep-queries` (wave E4 source admission ceiling, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).
/// 🪡 Loop positions, surface parameters, and boundary flags.
pub type TessellationLoopUv = (Vec<Pnt3>, Vec<(f64, f64)>, Vec<bool>);

use std::collections::HashMap;
use semio_framework_mesh_engine::HistoryFoldIndex;

type EdgeSampleCache = HistoryFoldIndex<EdgeId, Vec<(f64, Pnt3)>>;

use semio_framework_3d_brep_representation::topology::Wire;
use semio_framework_3d_brep_representation::contract::{CurveKind, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, SurfaceKind, VertexGroup};
use semio_framework_3d_brep_representation::arena::{CoedgeId, EdgeId, FaceId, LoopId, ShellId, SolidId, VertexId};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::error::KernelError;
use semio_framework_3d_brep_surface::surface_ops;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_representation::topology::{Body, Coedge, Edge};
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

// #region 🔖️Constants

/// 📐 Secondary sanity bound on how far a sampled step may turn (radians), independent of the
/// caller's chordal `deflection` — guards against pathological near-degenerate segment counts
/// (e.g. a huge-radius, very-loose-deflection arc reducing to 1-2 segments) rather than driving
/// resolution itself. `deflection` is meant to be the PRIMARY, caller-controlled accuracy knob
/// (`segments_for_chord_deviation`, `triangle_needs_refine`) — a tight angular floor (the
/// previous `0.35`, ~20°) silently overrode it, forcing the same ~18-segment circle regardless of
/// deflection across a wide, ordinary range (any `deflection` above ~1.5% of the radius), which is
/// what made `tighter_deflection_yields_more_triangles` observe identical triangle counts for two
/// very different requested tolerances — a real implementation bug, not a flawed assertion.
/// Deliberately NOT an exact divisor of `τ` (unlike `π/3`, which forces boundary segment count to
/// land EXACTLY on the floor with zero headroom, so `triangle_needs_refine`'s later angular check
/// re-triggers on the very ring it just accepted, cascading into needless interior refinement —
/// confirmed live via `DEBUG_TESS`, `π/3` made a *coarser* 0.2 deflection produce MORE triangles
/// than a finer 0.02 one, the opposite of `segments_for_chord_deviation`'s intent).
const DEFAULT_ANGULAR_TOL: f64 = 1.4;
const ENDPOINT_TOL: f64 = 1e-9;
const POLE_WELD_TOL: f64 = 1e-7;
const POLE_BRANCH_TOL: f64 = 1e-9;
const MAX_REFINE_ITERS: usize = 8;
const MAX_CURVE_SUBDIV_DEPTH: u32 = 12;
const MAX_INTERIOR_POINTS: usize = 20_000;

// #endregion 🔖️Constants

// #region 🔖️Report

/// 📐 Error certificate for one tessellation call: the worst chordal deviation (surface-to-chord
/// distance, model units) and worst angular deviation (surface normal variation across a
/// triangle, radians) observed across the accepted mesh.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TessellationReport {
    pub max_chordal: f64,
    pub max_angular: f64,
}

// #endregion 🔖️Report

// #region 🔖️Api

/// 🧩 Tessellates every face of `solid` with edge-first shared discretization into one
/// [`MeshTransfer`]. Thin wrapper over [`tessellate_solid_with_report`] for callers that don't
/// need the error certificate — signature kept stable since `⚙️engine`, `🔺️diff/↔️offset`,
/// `🔺️diff/🔀️boolean` and `⚙️engine/📦️mesh-io` all call this directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tessellate_solid(body: &Body, solid: SolidId, deflection: f64) -> Result<MeshTransfer, KernelError> {
    tessellate_solid_with_report(body, solid, deflection).map(|(mesh, _)| mesh)
}

/// 🧩 [`tessellate_solid`] plus the [`TessellationReport`] error certificate. Drives the resumable
/// [`TessellationJob`] to completion in one unbudgeted call — the incremental path is the ONLY
/// algorithm, so a one-shot result is byte-identical to the same job stepped in small budgets.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tessellate_solid_with_report(body: &Body, solid: SolidId, deflection: f64) -> Result<(MeshTransfer, TessellationReport), KernelError> {
    TessellationJob::new(deflection).run_to_completion(body,TessellationInput::Solid(solid))
}

/// 🧵 Tessellates a wire into edge polylines only (no shaded triangles).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tessellate_wire(body: &Body, wire: &Wire, deflection: f64) -> Result<MeshTransfer, KernelError> {
    TessellationJob::new(deflection).run_to_completion(body,TessellationInput::Wire(&wire.members)).map(|(mesh, _)| mesh)
}

/// 🧩 Tessellates a single face into a [`MeshTransfer`] with one [`FaceGroup`]. Thin wrapper over
/// [`tessellate_face_with_report`], signature kept stable for the same reason as
/// [`tessellate_solid`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tessellate_face(body: &Body, face: FaceId, deflection: f64) -> Result<MeshTransfer, KernelError> {
    tessellate_face_with_report(body, face, deflection).map(|(mesh, _)| mesh)
}

/// 🧩 [`tessellate_face`] plus the [`TessellationReport`] error certificate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tessellate_face_with_report(body: &Body, face: FaceId, deflection: f64) -> Result<(MeshTransfer, TessellationReport), KernelError> {
    TessellationJob::new(deflection).run_to_completion(body,TessellationInput::Face(face))
}

/// 🧩 Samples `edge` to a deflection-bounded polyline and returns packed xyz `f32` positions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sample_edge_polyline(body: &Body, edge: EdgeId, deflection: f64) -> Vec<f32> {
    match sample_edge_points(body, edge, deflection.max(1e-9)) {
        Ok(points) => points.iter().flat_map(|&(_, p)| [p.x as f32, p.y as f32, p.z as f32]).collect(),
        Err(_) => Vec::new(),
    }
}

// #endregion 🔖️Api

#[path = "⏱️tessellation-job/🦀️.rs"]
mod tessellation_job;
pub use tessellation_job::{TessellationInput, TessellationJob, TessellationPhase, TessellationProgress, TessellationStep};

#[path = "🧵️edge-sampling/🦀️.rs"]
mod edge_sampling;
use edge_sampling::{curve_kind_of, polyline_length, push_polyline_segments, sample_edge_points, surface_kind_of};

#[path = "🧊️face-meshing/🦀️.rs"]
mod face_meshing;
use face_meshing::append_face_mesh;
#[cfg(test)]
use face_meshing::collect_loop_uv;

#[path = "🔺️triangulation-and-refinement/🦀️.rs"]
mod triangulation_and_refinement;
use triangulation_and_refinement::{Tri, build_constrained_triangulation, centroid3, flatten_tris, refine_adaptive};

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests

//#region 🧪️TestSupport
#[cfg(test)]
pub(crate) use semio_framework_3d_testing_support as test_support;
//#endregion 🧪️TestSupport
