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
/// 🪡 Loop positions, surface parameters, and boundary flags.
pub type TessellationLoopUv = (Vec<Pnt3>, Vec<(f64, f64)>, Vec<bool>);

use std::collections::{HashMap, HashSet};

use crate::standards::v1::subsets::brep::schema::diff::primitives::Wire;
use crate::standards::v1::subsets::brep::schema::engine::{CurveKind, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, SurfaceKind};
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{EdgeId, FaceId, LoopId, SolidId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Coedge, Edge};
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

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
    TessellationJob::for_solid(body, solid, deflection)?.run_to_completion(body)
}

/// 🧵 Tessellates a wire into edge polylines only (no shaded triangles).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tessellate_wire(body: &Body, wire: &Wire, deflection: f64) -> Result<MeshTransfer, KernelError> {
    TessellationJob::for_wire(wire, deflection).run_to_completion(body).map(|(mesh, _)| mesh)
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
    TessellationJob::for_face(body, face, deflection)?.run_to_completion(body)
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

// #region ⏱️ResumableJob

/// ⏱️ What a [`TessellationJob`] is currently doing. Phases run in declaration order; `Complete`
/// and `Cancelled` are terminal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TessellationPhase {
    /// 🖇️ Discretizing every distinct edge exactly once (one unit per edge).
    #[default]
    SamplingEdges,
    /// 🔺 Triangulating one face per unit against the shared edge samples.
    MeshingFaces,
    /// 🗃️ Emitting one edge's polyline segments and `EdgeGroup`/`EdgeInfo` metadata per unit.
    PackingEdges,
    Complete,
    Cancelled,
}

impl TessellationPhase {
    /// 🏷️ The stable wire tag every host/extension/UI layer names this phase by.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn tag(self) -> &'static str {
        match self {
            Self::SamplingEdges => "samplingEdges",
            Self::MeshingFaces => "meshingFaces",
            Self::PackingEdges => "packingEdges",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
        }
    }
}

/// 📈 Monotone progress of one resumable tessellation: `units_done` never decreases and never
/// exceeds `units_total`, which is fixed at construction (edges + faces + edges).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TessellationProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub faces_done: usize,
    pub faces_total: usize,
    pub phase: TessellationPhase,
}

/// ⏱️ Outcome of one budgeted [`TessellationJob::step`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TessellationStep {
    /// 🔁 Budget spent, work remains — call `step` again.
    Working(TessellationProgress),
    /// ✅ Terminal: the mesh is ready, take it with [`TessellationJob::into_mesh`].
    Done(TessellationProgress),
    /// 🛑 Terminal: [`TessellationJob::cancel`] retired the job; no mesh is produced.
    Cancelled(TessellationProgress),
}

/// 🎯 What one [`TessellationJob`] covers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TessellationTarget {
    Solid,
    Face,
    Wire,
}

/// ⏱️ A tessellation split into budgetable units so a host can run it inside an interactive step
/// ceiling across many turns, report progress, and cancel it cleanly.
///
/// One unit is one edge discretization or one face triangulation — the smallest granularity the
/// crack-free, edge-first algorithm admits without recomputing shared boundary samples. A face is
/// therefore the coarsest unit; a pathological single face still costs one whole unit, which is
/// why callers pick a budget rather than a wall-clock deadline.
pub struct TessellationJob {
    target: TessellationTarget,
    deflection: f64,
    edge_order: Vec<EdgeId>,
    edge_cache: HashMap<EdgeId, Vec<(f64, Pnt3)>>,
    faces: Vec<FaceId>,
    edge_cursor: usize,
    face_cursor: usize,
    pack_cursor: usize,
    transfer: MeshTransfer,
    report: TessellationReport,
    phase: TessellationPhase,
}

impl TessellationJob {
    /// 🧩 A resumable tessellation of every face of `solid`.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn for_solid(body: &Body, solid: SolidId, deflection: f64) -> Result<Self, KernelError> {
        if body.solids.get(solid).is_none() {
            return Err(KernelError::MissingEntity(solid.to_string()));
        }
        let faces = body.solid_faces(solid);
        if faces.is_empty() {
            return Err(KernelError::InvalidInput(format!("solid {solid} has no faces")));
        }
        let edge_order = distinct_face_edges(body, &faces)?;
        Ok(Self::seeded(TessellationTarget::Solid, deflection, edge_order, faces))
    }

    /// 🧩 A resumable tessellation of one face.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn for_face(body: &Body, face: FaceId, deflection: f64) -> Result<Self, KernelError> {
        if body.faces.get(face).is_none() {
            return Err(KernelError::MissingEntity(face.to_string()));
        }
        let faces = vec![face];
        let edge_order = distinct_face_edges(body, &faces)?;
        Ok(Self::seeded(TessellationTarget::Face, deflection, edge_order, faces))
    }

    /// 🧵 A resumable discretization of a wire into edge polylines only.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn for_wire(wire: &Wire, deflection: f64) -> Self {
        let mut edge_order = Vec::with_capacity(wire.members.len());
        for (edge_id, _forward) in &wire.members {
            edge_order.push(*edge_id);
        }
        Self::seeded(TessellationTarget::Wire, deflection, edge_order, Vec::new())
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn seeded(target: TessellationTarget, deflection: f64, edge_order: Vec<EdgeId>, faces: Vec<FaceId>) -> Self {
        Self {
            target,
            deflection: deflection.max(1e-9),
            edge_cache: HashMap::with_capacity(edge_order.len()),
            edge_order,
            faces,
            edge_cursor: 0,
            face_cursor: 0,
            pack_cursor: 0,
            transfer: MeshTransfer::default(),
            report: TessellationReport::default(),
            phase: TessellationPhase::SamplingEdges,
        }
    }

    /// 📈 This job's progress right now — safe to read between steps and after termination.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn progress(&self) -> TessellationProgress {
        TessellationProgress {
            units_done: self.edge_cursor + self.face_cursor + self.pack_cursor,
            units_total: self.units_total(),
            faces_done: self.face_cursor,
            faces_total: self.faces.len(),
            phase: self.phase,
        }
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn units_total(&self) -> usize {
        self.edge_order.len() + self.faces.len() + self.edge_order.len()
    }

    /// 🛑 Retires this job at the next observable boundary — the current phase becomes `Cancelled`
    /// immediately, every partial buffer is dropped, and further steps are no-ops.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn cancel(&mut self) {
        if matches!(self.phase, TessellationPhase::Complete) {
            return;
        }
        self.phase = TessellationPhase::Cancelled;
        self.edge_cache.clear();
        self.transfer = MeshTransfer::default();
    }

    /// ✅ True once the job reached a terminal phase.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn is_terminal(&self) -> bool {
        matches!(self.phase, TessellationPhase::Complete | TessellationPhase::Cancelled)
    }

    /// ⏱️ Advances by at most `budget` units. A `budget` of zero is a legal progress probe that
    /// performs no work.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn step(&mut self, body: &Body, budget: usize) -> Result<TessellationStep, KernelError> {
        match self.phase {
            TessellationPhase::Complete => return Ok(TessellationStep::Done(self.progress())),
            TessellationPhase::Cancelled => return Ok(TessellationStep::Cancelled(self.progress())),
            _ => {}
        }
        let mut spent = 0;
        while spent < budget {
            match self.phase {
                TessellationPhase::SamplingEdges => {
                    if self.edge_cursor >= self.edge_order.len() {
                        self.phase = if self.faces.is_empty() { TessellationPhase::PackingEdges } else { TessellationPhase::MeshingFaces };
                        continue;
                    }
                    let edge = self.edge_order[self.edge_cursor];
                    if let std::collections::hash_map::Entry::Vacant(slot) = self.edge_cache.entry(edge) {
                        slot.insert(sample_edge_points(body, edge, self.deflection)?);
                    }
                    self.edge_cursor += 1;
                }
                TessellationPhase::MeshingFaces => {
                    if self.face_cursor >= self.faces.len() {
                        self.phase = TessellationPhase::PackingEdges;
                        continue;
                    }
                    let face = self.faces[self.face_cursor];
                    append_face_mesh(&mut self.transfer, &mut self.report, body, face, self.deflection, &self.edge_cache)?;
                    self.face_cursor += 1;
                }
                TessellationPhase::PackingEdges => {
                    if self.pack_cursor >= self.edge_order.len() {
                        self.phase = TessellationPhase::Complete;
                        self.edge_cache.clear();
                        return Ok(TessellationStep::Done(self.progress()));
                    }
                    let edge = self.edge_order[self.pack_cursor];
                    self.pack_edge(body, edge);
                    self.pack_cursor += 1;
                }
                TessellationPhase::Complete => return Ok(TessellationStep::Done(self.progress())),
                TessellationPhase::Cancelled => return Ok(TessellationStep::Cancelled(self.progress())),
            }
            spent += 1;
        }
        if self.edge_cursor >= self.edge_order.len() && self.face_cursor >= self.faces.len() && self.pack_cursor >= self.edge_order.len() {
            self.phase = TessellationPhase::Complete;
            self.edge_cache.clear();
            return Ok(TessellationStep::Done(self.progress()));
        }
        Ok(TessellationStep::Working(self.progress()))
    }

    /// 🗃️ One edge's polyline segments plus its `EdgeGroup`/`EdgeInfo` row. A wire target emits
    /// segments only — it carries no face topology to scope metadata to.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn pack_edge(&mut self, body: &Body, edge_id: EdgeId) {
        let Some(points) = self.edge_cache.get(&edge_id) else { return };
        let start = (self.transfer.edges.len() / 6) as u32;
        let pts: Vec<Pnt3> = points.iter().map(|&(_, p)| p).collect();
        let length = polyline_length(points);
        push_polyline_segments(&mut self.transfer.edges, &pts);
        if matches!(self.target, TessellationTarget::Wire) {
            return;
        }
        let Some(edge) = body.edges.get(edge_id) else { return };
        let count = pts.len().saturating_sub(1) as u32;
        let label = edge.label.0.to_string();
        let curve_kind = body.curves3.get(edge.curve).map_or(CurveKind::Line, curve_kind_of);
        self.transfer.edge_groups.push(EdgeGroup { start, count, entity_id: label.clone() });
        self.transfer.edge_infos.push(EdgeInfo { entity_id: label, curve_kind, length });
    }

    /// 📤 Takes the finished mesh — `None` unless the job reached `Complete`.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn into_mesh(self) -> Option<(MeshTransfer, TessellationReport)> {
        matches!(self.phase, TessellationPhase::Complete).then_some((self.transfer, self.report))
    }

    /// 🏁 Drives every remaining unit in one unbudgeted call — the one-shot façade every legacy-free
    /// caller of [`tessellate_solid`]/[`tessellate_face`]/[`tessellate_wire`] goes through.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn run_to_completion(mut self, body: &Body) -> Result<(MeshTransfer, TessellationReport), KernelError> {
        loop {
            match self.step(body, usize::MAX)? {
                TessellationStep::Done(_) => break,
                TessellationStep::Cancelled(_) => return Err(KernelError::InvalidInput("tessellation cancelled".to_string())),
                TessellationStep::Working(_) => continue,
            }
        }
        self.into_mesh().map(|(mesh, report)| (mesh, report)).ok_or_else(|| KernelError::InvalidInput("tessellation produced no mesh".to_string()))
    }
}

/// 🖇️ Every distinct edge reachable from `faces`, in deterministic face→coedge order — the single
/// ordering both the sampling and the packing phase walk, so a stepped job emits byte-identical
/// `edges`/`edge_groups`/`edge_infos` to an unbudgeted one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn distinct_face_edges(body: &Body, faces: &[FaceId]) -> Result<Vec<EdgeId>, KernelError> {
    let mut seen: HashSet<EdgeId> = HashSet::new();
    let mut order = Vec::new();
    for face in faces {
        for coedge_id in body.face_coedges(*face) {
            let edge = body.coedges.get(coedge_id).map(|c| c.edge).ok_or_else(|| KernelError::MissingEntity(coedge_id.to_string()))?;
            if seen.insert(edge) {
                order.push(edge);
            }
        }
    }
    Ok(order)
}

// #endregion ⏱️ResumableJob

// #region 🧮EdgeSample

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_edge_points(body: &Body, edge_id: EdgeId, deflection: f64) -> Result<Vec<(f64, Pnt3)>, KernelError> {
    let edge = body.edges.get(edge_id).ok_or_else(|| KernelError::MissingEntity(edge_id.to_string()))?;
    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity(edge.curve.to_string()))?;
    let v0 = body.vertices.get(edge.v0).ok_or_else(|| KernelError::MissingEntity(edge.v0.to_string()))?.position;
    let v1 = body.vertices.get(edge.v1).ok_or_else(|| KernelError::MissingEntity(edge.v1.to_string()))?.position;
    let (t0, t1) = edge.range;
    let mut points = match curve {
        Curve3::Line { .. } => vec![(t0, v0), (t1, v1)],
        Curve3::Circle { radius, .. } => {
            let n = segments_for_chord_deviation(*radius, (t1 - t0).abs(), deflection, DEFAULT_ANGULAR_TOL);
            sample_uniform(curve, t0, t1, n + 1)
        }
        Curve3::Ellipse { major_radius, minor_radius, .. } => {
            let curv_r = (*major_radius * *major_radius) / minor_radius.max(1e-12);
            let n = segments_for_chord_deviation(curv_r, (t1 - t0).abs(), deflection, DEFAULT_ANGULAR_TOL);
            sample_uniform(curve, t0, t1, n + 1)
        }
        Curve3::Nurbs { .. } => sample_curve_adaptive(curve, t0, t1, deflection, DEFAULT_ANGULAR_TOL),
    };
    if let Some(first) = points.first_mut() {
        first.1 = v0;
    }
    if let Some(last) = points.last_mut() {
        last.1 = v1;
    }
    Ok(points)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_uniform(curve: &Curve3, t0: f64, t1: f64, count: usize) -> Vec<(f64, Pnt3)> {
    let n = count.max(2);
    (0..n)
        .map(|i| {
            let t = t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64);
            (t, curve.eval(t))
        })
        .collect()
}

/// 📐 Recursive chordal + angular bisection (no fixed sample count): halves any span whose
/// midpoint deviates from the chord by more than `deflection`, or whose tangent turns by more
/// than `angular_tol`, up to [`MAX_CURVE_SUBDIV_DEPTH`] — the "exact" adaptive criterion replacing
/// the old coarse-grid-then-rescale heuristic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_curve_adaptive(curve: &Curve3, t0: f64, t1: f64, deflection: f64, angular_tol: f64) -> Vec<(f64, Pnt3)> {
    let mut out = vec![(t0, curve.eval(t0)), (t1, curve.eval(t1))];
    subdivide_curve_segment(curve, t0, t1, deflection, angular_tol, 0, &mut out);
    out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    out.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-13);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn subdivide_curve_segment(curve: &Curve3, ta: f64, tb: f64, deflection: f64, angular_tol: f64, depth: u32, out: &mut Vec<(f64, Pnt3)>) {
    if depth >= MAX_CURVE_SUBDIV_DEPTH {
        return;
    }
    let pa = curve.eval(ta);
    let pb = curve.eval(tb);
    let tm = 0.5 * (ta + tb);
    let pm = curve.eval(tm);
    let dev = pm.distance(pa.lerp(pb, 0.5));
    let ang = match (curve.tangent(ta), curve.tangent(tb)) {
        (Some(a), Some(b)) => a.angle_to(b),
        _ => 0.0,
    };
    if dev > deflection || ang > angular_tol {
        subdivide_curve_segment(curve, ta, tm, deflection, angular_tol, depth + 1, out);
        out.push((tm, pm));
        subdivide_curve_segment(curve, tm, tb, deflection, angular_tol, depth + 1, out);
    }
}

/// 📐 Exact segment count for a circular arc of `radius` spanning `arc_range` radians so that the
/// chord deviates from the arc by at most `deflection` *and* each step turns by at most
/// `angular_tol`: `n = ceil(arc_range / min(2·acos(1 − deflection/radius), angular_tol))`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segments_for_chord_deviation(radius: f64, arc_range: f64, deflection: f64, angular_tol: f64) -> usize {
    if radius <= 0.0 || arc_range <= 0.0 {
        return 1;
    }
    let d = deflection.min(radius * 1.999);
    let ratio = (1.0 - d / radius).clamp(-1.0, 1.0);
    let theta_chord = 2.0 * ratio.acos();
    let theta_step = if angular_tol > 0.0 { theta_chord.min(angular_tol) } else { theta_chord };
    if theta_step <= 1e-9 {
        return ((arc_range / 1e-9).ceil() as usize).clamp(1, 200_000);
    }
    ((arc_range / theta_step).ceil() as usize).max(1)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn polyline_length(points: &[(f64, Pnt3)]) -> f64 {
    points.windows(2).map(|w| w[0].1.distance(w[1].1)).sum()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_polyline_segments(out: &mut Vec<f32>, points: &[Pnt3]) {
    for window in points.windows(2) {
        let a = window[0];
        let b = window[1];
        out.extend([a.x as f32, a.y as f32, a.z as f32, b.x as f32, b.y as f32, b.z as f32]);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn curve_kind_of(curve: &Curve3) -> CurveKind {
    match curve {
        Curve3::Line { .. } => CurveKind::Line,
        Curve3::Circle { .. } => CurveKind::Circle,
        Curve3::Ellipse { .. } => CurveKind::Ellipse,
        Curve3::Nurbs { .. } => CurveKind::Nurbs,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surface_kind_of(surface: &Surface) -> SurfaceKind {
    match surface {
        Surface::Plane { .. } => SurfaceKind::Plane,
        Surface::Cylinder { .. } => SurfaceKind::Cylinder,
        Surface::Cone { .. } => SurfaceKind::Cone,
        Surface::Sphere { .. } => SurfaceKind::Sphere,
        Surface::Torus { .. } => SurfaceKind::Torus,
        Surface::Nurbs { .. } => SurfaceKind::Nurbs,
    }
}

// #endregion 🧮EdgeSample

// #region 🧊FaceTessellate

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn append_face_mesh(transfer: &mut MeshTransfer, report: &mut TessellationReport, body: &Body, face_id: FaceId, deflection: f64, edge_cache: &HashMap<EdgeId, Vec<(f64, Pnt3)>>) -> Result<(), KernelError> {
    let face = body.faces.get(face_id).ok_or_else(|| KernelError::MissingEntity(face_id.to_string()))?;
    let surface = body.surfaces.get(face.surface).ok_or_else(|| KernelError::MissingEntity(face.surface.to_string()))?;
    let Some(outer_id) = face.outer else {
        return Err(KernelError::InvalidInput(format!("face {face_id} has no outer loop")));
    };
    let (mut boundary_pos, mut boundary_uv, mut boundary_pole) = collect_loop_uv(body, outer_id, surface, edge_cache)?;
    remove_closing_duplicate_uv(&mut boundary_pos, &mut boundary_uv, &mut boundary_pole);
    if boundary_pos.len() < 3 {
        return Err(KernelError::Operation(format!("face {face_id} outer loop degenerated to {} points", boundary_pos.len())));
    }
    let outer_count = boundary_pos.len();
    let mut positions = boundary_pos;
    let mut uvs = boundary_uv;
    let mut poles = boundary_pole;
    let mut hole_ranges: Vec<(usize, usize)> = Vec::new();
    for &inner_id in &face.inners {
        let (mut hole_pos, mut hole_uv, mut hole_pole) = collect_loop_uv(body, inner_id, surface, edge_cache)?;
        remove_closing_duplicate_uv(&mut hole_pos, &mut hole_uv, &mut hole_pole);
        if hole_pos.len() < 3 {
            continue;
        }
        let start = positions.len();
        positions.extend(hole_pos);
        uvs.extend(hole_uv);
        poles.extend(hole_pole);
        hole_ranges.push((start, positions.len()));
    }
    let mut tris = build_constrained_triangulation(&uvs, (0, outer_count), &hole_ranges)?;
    if !surface.is_planar() {
        refine_adaptive(surface, &mut positions, &mut uvs, &mut poles, &mut tris, deflection, DEFAULT_ANGULAR_TOL);
    }
    weld_and_compact(&mut positions, &mut uvs, &poles, &mut tris, POLE_WELD_TOL);
    let mut indices = flatten_tris(&tris);
    let flipped = face.flipped;
    let desired_at = |uv: (f64, f64)| -> Vec3 {
        let mut n = surface.normal(uv.0, uv.1).unwrap_or(Vec3::Z);
        if flipped {
            n = -n;
        }
        n
    };
    fix_winding_per_triangle(&positions, &uvs, &mut indices, &desired_at);
    let (max_chordal, max_angular) = measure_report(surface, &positions, &uvs, &indices);
    report.max_chordal = report.max_chordal.max(max_chordal);
    report.max_angular = report.max_angular.max(max_angular);
    let base = (transfer.position.len() / 3) as u32;
    let tri_start = transfer.index.len() as u32;
    for (i, p) in positions.iter().enumerate() {
        transfer.position.extend([p.x as f32, p.y as f32, p.z as f32]);
        let n = vertex_normal(surface, flipped, uvs[i], &positions, &indices, i);
        transfer.normal.extend([n.x as f32, n.y as f32, n.z as f32]);
    }
    for idx in &indices {
        transfer.index.push(base + *idx);
    }
    let mut area = 0.0;
    for tri in indices.as_chunks::<3>().0 {
        let a = positions[tri[0] as usize];
        let b = positions[tri[1] as usize];
        let c = positions[tri[2] as usize];
        area += (b - a).cross(c - a).norm() * 0.5;
    }
    let label = face.label.0.to_string();
    let normal_first = desired_at(uvs.first().copied().unwrap_or((0.0, 0.0)));
    transfer.face_groups.push(FaceGroup { start: tri_start, count: indices.len() as u32, entity_id: label.clone() });
    transfer.face_infos.push(FaceInfo { entity_id: label, surface_kind: surface_kind_of(surface), area, normal: normal_first.to_array() });
    Ok(())
}

/// 🧭 Walks one loop's coedges, reusing the shared edge cache for 3D positions and resolving each
/// point's `(u, v)` from the coedge's stored p-curve when present (`Coedge::pcurve`), else via
/// surface closest-point projection — then unwraps across periodic seams (choosing the UV branch
/// nearest the previous sample) and pins the arbitrary `u` at poles/apexes to the previous branch
/// so a boundary loop that touches a singularity stays a single well-formed ring vertex there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn collect_loop_uv(body: &Body, loop_id: LoopId, surface: &Surface, edge_cache: &HashMap<EdgeId, Vec<(f64, Pnt3)>>) -> Result<TessellationLoopUv, KernelError> {
    let mut positions: Vec<Pnt3> = Vec::new();
    let mut uvs: Vec<(f64, f64)> = Vec::new();
    let mut poles: Vec<bool> = Vec::new();
    let mut prev: Option<(f64, f64)> = None;
    let mut prev_is_pole = false;
    for coedge_id in body.loop_coedges(loop_id) {
        let coedge = body.coedges.get(coedge_id).ok_or_else(|| KernelError::MissingEntity(coedge_id.to_string()))?;
        let edge = body.edges.get(coedge.edge).ok_or_else(|| KernelError::MissingEntity(coedge.edge.to_string()))?;
        let samples = edge_cache.get(&coedge.edge).ok_or_else(|| KernelError::Operation(format!("missing edge sample for {}", coedge.edge)))?;
        let ordered: Vec<(f64, Pnt3)> = if coedge.forward { samples.clone() } else { samples.iter().rev().copied().collect() };
        for (i, &(t, p)) in ordered.iter().enumerate() {
            if i == 0 {
                if let Some(&last) = positions.last() {
                    if last.distance(p) <= ENDPOINT_TOL {
                        continue;
                    }
                }
            }
            let (raw_uv, is_pole) = coedge_point_uv(body, surface, coedge, edge, t, p);
            // A pole's own `u` is meaningless and gets pinned to whatever branch we arrived from
            // (below) — but that pinned value must NOT then anchor the departing seam's own branch
            // once we leave the pole: a lune-shaped face (sphere/cone) legitimately re-enters the
            // seam at the OPPOSITE `u` branch after a pole (e.g. `0` in, `2π` out), and treating the
            // pinned pole value as still-live continuity would wrongly snap that outgoing branch
            // back to the incoming one, collapsing the whole loop onto a single meridian.
            let unwrap_ref = if prev_is_pole { None } else { prev };
            let uv = unwrap_uv(surface, raw_uv, unwrap_ref, is_pole);
            prev = Some(uv);
            prev_is_pole = is_pole;
            positions.push(p);
            uvs.push(uv);
            poles.push(is_pole);
        }
    }
    Ok((positions, uvs, poles))
}

/// 🧭 `(u, v)` for one edge sample: the coedge's stored p-curve reparametrized from the edge's own
/// `t` via `Coedge::prange`/`Edge::range` when present, else `surface_ops::closest_uv`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_point_uv(body: &Body, surface: &Surface, coedge: &Coedge, edge: &Edge, t: f64, p: Pnt3) -> ((f64, f64), bool) {
    let uv = match coedge.pcurve.and_then(|id| body.curves2.get(id)) {
        Some(pcurve) => {
            let (er0, er1) = edge.range;
            let (pr0, pr1) = coedge.prange;
            let frac = if (er1 - er0).abs() > 1e-14 { (t - er0) / (er1 - er0) } else { 0.0 };
            let pt = pr0 + frac * (pr1 - pr0);
            let uv2 = pcurve.eval(pt);
            (uv2.x, uv2.y)
        }
        None => {
            let closest = surface_ops::closest_uv(surface, surface.domain(), p, 1e-9);
            (closest.u, closest.v)
        }
    };
    let is_pole = surface.is_degenerate_uv(uv.0, uv.1);
    (uv, is_pole)
}

/// 🧭 Picks the branch `uv ± k·period` nearest `prev` for each periodic coordinate (so a boundary
/// loop crossing a seam stays continuous in UV instead of jumping); at a pole/apex `u` is
/// meaningless, so it's pinned to `prev.0` instead of being (arbitrarily) unwrapped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_uv(surface: &Surface, mut uv: (f64, f64), prev: Option<(f64, f64)>, is_pole: bool) -> (f64, f64) {
    if let Some(prev) = prev {
        if is_pole {
            uv.0 = prev.0;
            return uv;
        }
        if surface.is_u_periodic() {
            uv.0 = nearest_branch(uv.0, prev.0, std::f64::consts::TAU);
        }
        if surface.is_v_periodic() {
            uv.1 = nearest_branch(uv.1, prev.1, std::f64::consts::TAU);
        }
    }
    uv
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nearest_branch(x: f64, reference: f64, period: f64) -> f64 {
    let k = ((reference - x) / period).round();
    x + k * period
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remove_closing_duplicate_uv(positions: &mut Vec<Pnt3>, uvs: &mut Vec<(f64, f64)>, poles: &mut Vec<bool>) {
    if positions.len() > 2 {
        if let (Some(&first), Some(&last)) = (positions.first(), positions.last()) {
            if first.distance(last) <= ENDPOINT_TOL {
                positions.pop();
                uvs.pop();
                poles.pop();
            }
        }
    }
}

/// 📐 Sums the deviation-at-edge-midpoint (chordal) and normal-angle-between-vertices (angular)
/// across every triangle's edges — the same metric [`triangle_needs_refine`] converges against,
/// so on a converged mesh this reports (at worst) the tolerance the caller asked for.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn measure_report(surface: &Surface, positions: &[Pnt3], uvs: &[(f64, f64)], indices: &[u32]) -> (f64, f64) {
    let mut max_chordal = 0.0_f64;
    let mut max_angular = 0.0_f64;
    for tri in indices.as_chunks::<3>().0 {
        let idx = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
        let pts3 = [positions[idx[0]], positions[idx[1]], positions[idx[2]]];
        let uv3 = [uvs[idx[0]], uvs[idx[1]], uvs[idx[2]]];
        for e in 0..3 {
            let (p0, uv0) = (pts3[e], uv3[e]);
            let (p1, uv1) = (pts3[(e + 1) % 3], uv3[(e + 1) % 3]);
            let mid_uv = (0.5 * (uv0.0 + uv1.0), 0.5 * (uv0.1 + uv1.1));
            let dev = surface.eval(mid_uv.0, mid_uv.1).distance(p0.lerp(p1, 0.5));
            max_chordal = max_chordal.max(dev);
        }
        let normals: Vec<Vec3> = uv3.iter().filter_map(|&uv| surface.normal(uv.0, uv.1)).collect();
        for i in 0..normals.len() {
            for j in (i + 1)..normals.len() {
                max_angular = max_angular.max(normals[i].angle_to(normals[j]));
            }
        }
    }
    (max_chordal, max_angular)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vertex_normal(surface: &Surface, flipped: bool, uv: (f64, f64), positions: &[Pnt3], indices: &[u32], vertex: usize) -> Vec3 {
    if let Some(mut n) = surface.normal(uv.0, uv.1) {
        if flipped {
            n = -n;
        }
        return n;
    }
    let mut accum = Vec3::ZERO;
    for tri in indices.as_chunks::<3>().0 {
        if tri.iter().any(|&i| i as usize == vertex) {
            let a = positions[tri[0] as usize];
            let b = positions[tri[1] as usize];
            let c = positions[tri[2] as usize];
            accum = accum + (b - a).cross(c - a);
        }
    }
    accum.normalized().unwrap_or(Vec3::Z)
}

/// 📐 Per-triangle winding fix (no global flip heuristic): each triangle's own 3D cross product is
/// compared against the surface's own normal at that triangle's UV centroid, so triangles on
/// different parts of a curved face are each judged against their own local normal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fix_winding_per_triangle(positions: &[Pnt3], uvs: &[(f64, f64)], indices: &mut [u32], desired_at: &dyn Fn((f64, f64)) -> Vec3) {
    for tri in indices.as_chunks_mut::<3>().0 {
        let (ia, ib, ic) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
        let a = positions[ia];
        let b = positions[ib];
        let c = positions[ic];
        let uv_c = centroid3(uvs[ia], uvs[ib], uvs[ic]);
        let desired = desired_at(uv_c);
        let actual = (b - a).cross(c - a);
        if actual.dot(desired) < 0.0 {
            tri.swap(1, 2);
        }
    }
}

/// 🫧 Collapses coincident 3D vertices that are BOTH flagged [`is_pole`](coedge_point_uv) (poles,
/// cone apexes — any UV-distinct samples that evaluate to the same 3D point because the surface's
/// own parametrization is degenerate there) into one, dropping the now-degenerate triangles this
/// creates. Since every triangle that touched the ring around the singularity now shares the one
/// welded vertex, this *is* the "fan around the pole" the CDT itself has no notion of. Crucially
/// this must NOT weld two non-pole seam-duplicate vertices (e.g. a cylinder's `u=0`/`u=2π` corner,
/// which lands on the exact same 3D point by periodicity but needs its own distinct UV on each
/// side of the seam) — welding those collapses two legitimately-separate boundary rectangle
/// corners into one, letting later triangles reference the wrong UV and blow up chordal deviation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn weld_and_compact(positions: &mut Vec<Pnt3>, uvs: &mut Vec<(f64, f64)>, poles: &[bool], tris: &mut Vec<Tri>, weld_tol: f64) {
    let n = positions.len();
    let cell = weld_tol.max(1e-12);
    let key = |p: Pnt3| -> (i64, i64, i64) { ((p.x / cell).round() as i64, (p.y / cell).round() as i64, (p.z / cell).round() as i64) };
    let mut buckets: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    let mut remap: Vec<usize> = (0..n).collect();
    for i in 0..n {
        if !poles[i] {
            let (kx, ky, kz) = key(positions[i]);
            buckets.entry((kx, ky, kz)).or_default().push(i);
            continue;
        }
        let (kx, ky, kz) = key(positions[i]);
        let mut found: Option<usize> = None;
        'outer: for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if let Some(list) = buckets.get(&(kx + dx, ky + dy, kz + dz)) {
                        for &j in list {
                            if poles[j] && positions[remap[j]].distance(positions[i]) <= weld_tol {
                                found = Some(remap[j]);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        match found {
            Some(rep) => remap[i] = rep,
            None => buckets.entry((kx, ky, kz)).or_default().push(i),
        }
    }
    for t in tris.iter_mut() {
        t.a = remap[t.a];
        t.b = remap[t.b];
        t.c = remap[t.c];
    }
    tris.retain(|t| t.a != t.b && t.b != t.c && t.a != t.c);
    let mut used = vec![false; n];
    for t in tris.iter() {
        used[t.a] = true;
        used[t.b] = true;
        used[t.c] = true;
    }
    let mut new_index = vec![usize::MAX; n];
    let mut new_positions = Vec::new();
    let mut new_uvs = Vec::new();
    for i in 0..n {
        if used[i] {
            new_index[i] = new_positions.len();
            new_positions.push(positions[i]);
            new_uvs.push(uvs[i]);
        }
    }
    for t in tris.iter_mut() {
        t.a = new_index[t.a];
        t.b = new_index[t.b];
        t.c = new_index[t.c];
    }
    *positions = new_positions;
    *uvs = new_uvs;
}

// #endregion 🧊FaceTessellate

// #region ▲Triangulate

/// 📐 A CCW-oriented triangle referencing indices into a shared UV/position point array.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tri {
    a: usize,
    b: usize,
    c: usize,
}

/// 📐 Builds the outer/hole boundary rings into an unconstrained Bowyer–Watson Delaunay
/// triangulation of the whole point set, recovers every boundary/hole ring edge as an exact
/// triangulation edge via edge flips, then trims to the region inside the outer ring and outside
/// every hole ring — replacing the old ear-clip+fan-fallback+hole-bridging pipeline.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_constrained_triangulation(uvs: &[(f64, f64)], outer: (usize, usize), holes: &[(usize, usize)]) -> Result<Vec<Tri>, KernelError> {
    if outer.1 - outer.0 < 3 {
        return Err(KernelError::Operation("degenerate outer loop".to_string()));
    }
    let mut tris = bowyer_watson(uvs);
    let mut constraints: Vec<(usize, usize)> = Vec::new();
    push_ring_constraints(&mut constraints, outer.0, outer.1);
    for &(s, e) in holes {
        push_ring_constraints(&mut constraints, s, e);
    }
    for &(i, j) in &constraints {
        recover_edge(uvs, &mut tris, i, j);
    }
    let outer_poly: Vec<(f64, f64)> = uvs[outer.0..outer.1].to_vec();
    let hole_polys: Vec<Vec<(f64, f64)>> = holes.iter().map(|&(s, e)| uvs[s..e].to_vec()).collect();
    tris.retain(|t| {
        let c = centroid3(uvs[t.a], uvs[t.b], uvs[t.c]);
        point_in_polygon(&outer_poly, c) && hole_polys.iter().all(|h| !point_in_polygon(h, c))
    });
    Ok(tris)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_ring_constraints(out: &mut Vec<(usize, usize)>, start: usize, end: usize) {
    let count = end - start;
    for i in 0..count {
        out.push((start + i, start + (i + 1) % count));
    }
}

/// 📐 Incremental Bowyer–Watson: a super-triangle enclosing every point, then one-by-one point
/// insertion (remove circumcircle-violating triangles, re-fan the cavity boundary around the new
/// point), finally discarding any triangle still touching a super-triangle vertex.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bowyer_watson(pts: &[(f64, f64)]) -> Vec<Tri> {
    let n = pts.len();
    if n < 3 {
        return Vec::new();
    }
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for &(x, y) in pts {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let dx = (max_x - min_x).max(1e-6);
    let dy = (max_y - min_y).max(1e-6);
    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;
    let r = dx.max(dy) * 20.0 + 10.0;
    let mut all: Vec<(f64, f64)> = pts.to_vec();
    let s0 = all.len();
    all.push((cx - 2.0 * r, cy - r));
    let s1 = all.len();
    all.push((cx + 2.0 * r, cy - r));
    let s2 = all.len();
    all.push((cx, cy + 2.0 * r));
    let mut tris = vec![ensure_ccw(&all, Tri { a: s0, b: s1, c: s2 })];
    for i in 0..n {
        insert_point_into(&all, &mut tris, i);
    }
    tris.retain(|t| t.a < n && t.b < n && t.c < n);
    tris
}

/// 📐 Single-point Bowyer–Watson insertion into an existing triangulation. Safe to reuse for
/// interior Steiner refinement on an already-trimmed/constrained mesh: a constrained ring edge is
/// (after trimming) owned by exactly one triangle, so it can never be a *shared* cavity edge that
/// insertion removes — it always survives as a cavity-boundary edge, reattached to the new point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn insert_point_into(pts: &[(f64, f64)], tris: &mut Vec<Tri>, p_idx: usize) {
    let p = pts[p_idx];
    let mut bad = Vec::new();
    let mut good = Vec::new();
    for &t in tris.iter() {
        if in_circumcircle(pts, t, p) {
            bad.push(t);
        } else {
            good.push(t);
        }
    }
    if bad.is_empty() {
        return;
    }
    let mut directed: Vec<(usize, usize)> = Vec::new();
    for t in &bad {
        directed.push((t.a, t.b));
        directed.push((t.b, t.c));
        directed.push((t.c, t.a));
    }
    let mut boundary = Vec::new();
    for &(u, v) in &directed {
        if !directed.iter().any(|&(x, y)| x == v && y == u) {
            boundary.push((u, v));
        }
    }
    let mut new_tris = good;
    for (u, v) in boundary {
        new_tris.push(ensure_ccw(pts, Tri { a: u, b: v, c: p_idx }));
    }
    *tris = new_tris;
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ensure_ccw(pts: &[(f64, f64)], t: Tri) -> Tri {
    if orient(pts[t.a], pts[t.b], pts[t.c]) < 0.0 {
        Tri { a: t.a, b: t.c, c: t.b }
    } else {
        t
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn orient(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn orient_sign(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> i32 {
    let v = orient(a, b, c);
    if v > 1e-12 {
        1
    } else if v < -1e-12 {
        -1
    } else {
        0
    }
}

/// 📐 Standard incircle determinant test (assumes `t` is CCW, guaranteed by [`ensure_ccw`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn in_circumcircle(pts: &[(f64, f64)], t: Tri, p: (f64, f64)) -> bool {
    let a = pts[t.a];
    let b = pts[t.b];
    let c = pts[t.c];
    let adx = a.0 - p.0;
    let ady = a.1 - p.1;
    let bdx = b.0 - p.0;
    let bdy = b.1 - p.1;
    let cdx = c.0 - p.0;
    let cdy = c.1 - p.1;
    let al = adx * adx + ady * ady;
    let bl = bdx * bdx + bdy * bdy;
    let cl = cdx * cdx + cdy * cdy;
    let det = adx * (bdy * cl - bl * cdy) - ady * (bdx * cl - bl * cdx) + al * (bdx * cdy - bdy * cdx);
    let scale = al.max(bl).max(cl).max(1.0);
    det > 1e-9 * scale
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_in_triangulation(tris: &[Tri], i: usize, j: usize) -> bool {
    tris.iter().any(|t| {
        let e = [(t.a, t.b), (t.b, t.c), (t.c, t.a)];
        e.iter().any(|&(u, v)| (u == i && v == j) || (u == j && v == i))
    })
}

/// 📐 Sloan-style constraint recovery: repeatedly flip a triangulation edge that properly crosses
/// segment `(i, j)` (and whose quad is convex, so the flip is valid) until `(i, j)` itself is an
/// edge, or no more flips are found (best-effort — leaves whatever partial recovery it achieved
/// rather than failing the whole tessellation over one pathological edge).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn recover_edge(pts: &[(f64, f64)], tris: &mut [Tri], i: usize, j: usize) {
    if i == j {
        return;
    }
    let cap = tris.len() * 8 + 40;
    let mut guard = 0;
    while !edge_in_triangulation(tris, i, j) && guard < cap {
        guard += 1;
        if !try_flip_towards(pts, tris, i, j) {
            break;
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn try_flip_towards(pts: &[(f64, f64)], tris: &mut [Tri], i: usize, j: usize) -> bool {
    let pi = pts[i];
    let pj = pts[j];
    for a in 0..tris.len() {
        let ta = tris[a];
        let edges = [(ta.a, ta.b, ta.c), (ta.b, ta.c, ta.a), (ta.c, ta.a, ta.b)];
        for &(u, v, opp1) in &edges {
            if u == i || u == j || v == i || v == j {
                continue;
            }
            if !segments_properly_intersect(pts[u], pts[v], pi, pj) {
                continue;
            }
            if let Some((b, opp2)) = find_opposite(tris, a, u, v) {
                if is_convex_quad(&[pts[u], pts[opp2], pts[v], pts[opp1]]) {
                    let t1 = ensure_ccw(pts, Tri { a: u, b: opp2, c: opp1 });
                    let t2 = ensure_ccw(pts, Tri { a: opp2, b: v, c: opp1 });
                    tris[a] = t1;
                    tris[b] = t2;
                    return true;
                }
            }
        }
    }
    false
}

/// 📐 The triangle (besides `skip`) sharing directed edge `(v, u)` — i.e. the neighbor across
/// `skip`'s `(u, v)` edge — plus that neighbor's opposite (non-shared) vertex.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_opposite(tris: &[Tri], skip: usize, u: usize, v: usize) -> Option<(usize, usize)> {
    for (b, t) in tris.iter().enumerate() {
        if b == skip {
            continue;
        }
        if t.a == v && t.b == u {
            return Some((b, t.c));
        }
        if t.b == v && t.c == u {
            return Some((b, t.a));
        }
        if t.c == v && t.a == u {
            return Some((b, t.b));
        }
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_convex_quad(q: &[(f64, f64); 4]) -> bool {
    let mut sign = 0.0_f64;
    for i in 0..4 {
        let a = q[i];
        let b = q[(i + 1) % 4];
        let c = q[(i + 2) % 4];
        let cross = (b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0);
        if cross.abs() < 1e-14 {
            return false;
        }
        if sign == 0.0 {
            sign = cross.signum();
        } else if cross.signum() != sign {
            return false;
        }
    }
    true
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segments_properly_intersect(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), p4: (f64, f64)) -> bool {
    let d1 = orient_sign(p3, p4, p1);
    let d2 = orient_sign(p3, p4, p2);
    let d3 = orient_sign(p1, p2, p3);
    let d4 = orient_sign(p1, p2, p4);
    d1 != 0 && d2 != 0 && d3 != 0 && d4 != 0 && d1 != d2 && d3 != d4
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn centroid3(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> (f64, f64) {
    ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_polygon(ring: &[(f64, f64)], p: (f64, f64)) -> bool {
    if ring.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = ring.len() - 1;
    for i in 0..ring.len() {
        let (ui, vi) = ring[i];
        let (uj, vj) = ring[j];
        if ((vi > p.1) != (vj > p.1)) && (p.0 < (uj - ui) * (p.1 - vi) / (vj - vi) + ui) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn flatten_tris(tris: &[Tri]) -> Vec<u32> {
    let mut out = Vec::with_capacity(tris.len() * 3);
    for t in tris {
        out.push(t.a as u32);
        out.push(t.b as u32);
        out.push(t.c as u32);
    }
    out
}

/// 📐 Adaptive interior refinement: while any triangle's chordal or angular deviation exceeds
/// tolerance, insert a Steiner point at the surface point for that triangle's UV centroid (always
/// strictly interior, never touching a constrained boundary/hole edge) and re-insert it via
/// [`insert_point_into`], bounded by [`MAX_REFINE_ITERS`] passes and [`MAX_INTERIOR_POINTS`] total.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refine_adaptive(surface: &Surface, positions: &mut Vec<Pnt3>, uvs: &mut Vec<(f64, f64)>, poles: &mut Vec<bool>, tris: &mut Vec<Tri>, deflection: f64, angular_tol: f64) {
    for _ in 0..MAX_REFINE_ITERS {
        let mut candidates: Vec<(f64, f64)> = Vec::new();
        for &t in tris.iter() {
            if let Some(uv) = triangle_needs_refine(surface, positions, uvs, t, deflection, angular_tol) {
                candidates.push(uv);
            }
        }
        if candidates.is_empty() || positions.len() >= MAX_INTERIOR_POINTS {
            break;
        }
        let mut inserted_any = false;
        for uv in candidates {
            if positions.len() >= MAX_INTERIOR_POINTS {
                break;
            }
            if too_close(uvs, uv, 1e-6) {
                continue;
            }
            let p3 = surface.eval(uv.0, uv.1);
            let idx = positions.len();
            positions.push(p3);
            uvs.push(uv);
            poles.push(surface.is_degenerate_uv(uv.0, uv.1));
            insert_point_into(uvs, tris, idx);
            inserted_any = true;
        }
        if !inserted_any {
            break;
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn too_close(uvs: &[(f64, f64)], p: (f64, f64), tol: f64) -> bool {
    uvs.iter().any(|&q| (q.0 - p.0).abs() < tol && (q.1 - p.1).abs() < tol)
}

/// 📐 A triangle needs refining if either its worst edge-midpoint chordal deviation from the true
/// surface, or the angle between two of its vertices' surface normals, exceeds tolerance — in
/// which case the candidate Steiner point is the surface point at its UV centroid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn triangle_needs_refine(surface: &Surface, positions: &[Pnt3], uvs: &[(f64, f64)], t: Tri, deflection: f64, angular_tol: f64) -> Option<(f64, f64)> {
    let pts3 = [positions[t.a], positions[t.b], positions[t.c]];
    let uv3 = [uvs[t.a], uvs[t.b], uvs[t.c]];
    let mut worst_chordal = 0.0_f64;
    for e in 0..3 {
        let (p0, uv0) = (pts3[e], uv3[e]);
        let (p1, uv1) = (pts3[(e + 1) % 3], uv3[(e + 1) % 3]);
        let mid_uv = (0.5 * (uv0.0 + uv1.0), 0.5 * (uv0.1 + uv1.1));
        let dev = surface.eval(mid_uv.0, mid_uv.1).distance(p0.lerp(p1, 0.5));
        worst_chordal = worst_chordal.max(dev);
    }
    let normals: Vec<Vec3> = uv3.iter().filter_map(|&uv| surface.normal(uv.0, uv.1)).collect();
    let mut worst_angular = 0.0_f64;
    for i in 0..normals.len() {
        for j in (i + 1)..normals.len() {
            worst_angular = worst_angular.max(normals[i].angle_to(normals[j]));
        }
    }
    if worst_chordal > deflection || worst_angular > angular_tol {
        Some(centroid3(uv3[0], uv3[1], uv3[2]))
    } else {
        None
    }
}

// #endregion ▲Triangulate

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
