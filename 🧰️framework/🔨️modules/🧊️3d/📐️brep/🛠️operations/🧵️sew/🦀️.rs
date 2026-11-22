//! 🧵🩹 Free-face sewing (tolerance edge matching, coedge pairing) plus solid healing (gap
//! closing, sliver removal, defeature, convert-to-nurbs). Two Lane-3/5 algorithms from ticket
//! `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT` share this compute subdir because
//! `heal_solid`'s repair pass calls `sew_faces` directly and no dedicated `🩹️heal` facet was
//! pre-mounted — folded here per the `✂️intersect`-style "one compute subdir, not a 1:1 file
//! mapping" precedent this ticket's wave PEEL established. Moved from
//! `🧰️framework/🔨️modules/🧊️3d/📐️brep/{🧵️sew,🩹️heal}` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL.

use std::collections::HashMap;
use std::collections::HashSet;

use {crate::euler::{add_face, add_shell, add_solid, make_edge, make_vertex}, semio_framework_3d_brep_representation::topology::make_loop};
use crate::primitives::{SolidPrimitive, make_analytic_solid};
use crate::staged::{Plan, StageOutput, StageProgress, StageStep, StagedOperation};
use semio_framework_3d_brep_validation::{issue_reaches, validate_body};
use semio_framework_3d_brep_representation::arena::{ArenaId, Curve3Id, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use semio_framework_3d_brep_curve::bspline::KnotVector;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::error::KernelError;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::{Body, EntityRef};
use semio_framework_3d_brep_vector::Pnt3;
#[cfg(test)]
use semio_framework_3d_brep_vector::Vec3;

// #region 🔖️SewApi

/// 🧵 Sew loose faces into one solid by merging coincident boundary edges within `tolerance`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sew_faces(body: &mut Body, faces: &[FaceId], tolerance: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if faces.len() < 2 {
        return Err(KernelError::InvalidInput("sewing requires at least 2 faces".into()));
    }
    let tol = if tolerance > 0.0 && tolerance.is_finite() { Tol::new(tolerance) } else { Tol::DEFAULT };
    let linear = tol.value();
    let snapshots = snapshot_faces(body, faces)?;
    let resolution = 1.0 / linear;
    let mut vertex_map: HashMap<(i64, i64, i64), VertexId> = HashMap::new();
    let mut edge_map: HashMap<(VertexId, VertexId), EdgeId> = HashMap::new();
    let mut new_faces = Vec::with_capacity(snapshots.len());
    for snap in &snapshots {
        let mut members = Vec::with_capacity(snap.edge_endpoints.len());
        for &(start_pt, end_pt) in &snap.edge_endpoints {
            let v_start = get_or_create_vertex(body, start_pt, resolution, tol, &mut vertex_map, rec);
            let v_end = get_or_create_vertex(body, end_pt, resolution, tol, &mut vertex_map, rec);
            let (v_lo, v_hi) = if v_start <= v_end { (v_start, v_end) } else { (v_end, v_start) };
            let forward = v_start == v_lo;
            let edge = *edge_map.entry((v_lo, v_hi)).or_insert_with(|| {
                let p0 = body.vertices.get(v_lo).expect("vertex").position;
                let p1 = body.vertices.get(v_hi).expect("vertex").position;
                let curve = body.curves3.insert(Curve3::Line { origin: p0, dir: p1 - p0 });
                make_edge(body, curve, (0.0, 1.0), v_lo, v_hi, tol, rec)
            });
            members.push((edge, forward));
        }
        let placeholder = FaceId::from_raw(0, 0);
        let outer = make_loop(body, placeholder, &members);
        let face = add_face(body, snap.surface, Some(outer), vec![], snap.flipped, snap.tol, rec);
        body.loops.get_mut(outer).expect("loop").face = face;
        new_faces.push(face);
    }
    let shell = add_shell(body, new_faces, rec);
    Ok(add_solid(body, shell, vec![], rec))
}

// #endregion 🔖️SewApi

// #region 🔖️SewSnapshot

struct FaceSnapshot {
    surface: SurfaceId,
    flipped: bool,
    tol: Tol,
    edge_endpoints: Vec<(Pnt3, Pnt3)>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn snapshot_faces(body: &Body, faces: &[FaceId]) -> Result<Vec<FaceSnapshot>, KernelError> {
    let mut out = Vec::with_capacity(faces.len());
    for &fid in faces {
        let face = body.faces.get(fid).ok_or_else(|| KernelError::MissingEntity(format!("face {fid}")))?;
        let outer = face.outer.ok_or_else(|| KernelError::Operation(format!("face {fid} has no outer loop")))?;
        let mut edge_endpoints = Vec::new();
        for coedge_id in body.loop_coedges(outer) {
            let coedge = body.coedges.get(coedge_id).ok_or_else(|| KernelError::MissingEntity(format!("coedge {coedge_id}")))?;
            let edge = body.edges.get(coedge.edge).ok_or_else(|| KernelError::MissingEntity(format!("edge {}", coedge.edge)))?;
            let p0 = body.vertices.get(edge.v0).expect("v0").position;
            let p1 = body.vertices.get(edge.v1).expect("v1").position;
            let (start_pt, end_pt) = if coedge.forward { (p0, p1) } else { (p1, p0) };
            edge_endpoints.push((start_pt, end_pt));
        }
        out.push(FaceSnapshot { surface: face.surface, flipped: face.flipped, tol: face.tol, edge_endpoints });
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn get_or_create_vertex(body: &mut Body, p: Pnt3, resolution: f64, tol: Tol, map: &mut HashMap<(i64, i64, i64), VertexId>, rec: &mut OpRecorder) -> VertexId {
    let key = ((p.x * resolution).round() as i64, (p.y * resolution).round() as i64, (p.z * resolution).round() as i64);
    *map.entry(key).or_insert_with(|| make_vertex(body, p, tol, rec))
}

// #endregion 🔖️SewSnapshot

// #region 🔖️HealApi

/// 🩹 Summary of repairs performed by [`heal_solid`].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HealingReport {
    pub vertices_merged: usize,
    pub degenerate_edges_removed: usize,
    pub orientations_fixed: usize,
    pub wire_gaps_closed: usize,
    pub small_faces_removed: usize,
    pub duplicate_faces_removed: usize,
}

impl HealingReport {
    pub fn total_repairs(&self) -> usize {
        self.vertices_merged + self.degenerate_edges_removed + self.orientations_fixed + self.wire_gaps_closed + self.small_faces_removed + self.duplicate_faces_removed
    }
}

/// 🩹 Snaps nearby vertices of the selected solid and commits only a validated repair.
pub fn heal_solid(body: &mut Body, solid: SolidId, tolerance: f64, rec: &mut OpRecorder) -> Result<HealingReport, KernelError> {
    solid_exists(body, solid)?;
    let tolerance = if tolerance.is_finite() && tolerance > 0.0 { tolerance } else { 1e-6 };
    let reach = body.reachable_from(&[EntityRef::Solid(solid)]);
    let mut points = reach.vertices.iter().map(|&id| {
        let point = body.vertices.get(id).expect("reachable vertex is live").position;
        (id, point, point)
    }).collect::<Vec<_>>();
    points.sort_unstable_by_key(|(id, _, _)| id.raw_index());
    for i in 0..points.len() {
        let (start, remaining) = points.split_at_mut(i + 1);
        let position = start[i].2;
        for (_, _, next) in remaining {
            if (*next - position).norm() <= tolerance { *next = position; }
        }
    }
    points.retain(|(_, before, after)| before != after);
    if !points.is_empty() {
        let other_solids = body.solids.iter().filter(|(id, _)| *id != solid)
            .map(|(id, _)| EntityRef::Solid(id)).collect::<Vec<_>>();
        let other = body.reachable_from(&other_solids);
        if points.iter().any(|(id, _, _)| other.vertices.contains(id)) {
            return Err(KernelError::Operation("healing would move a vertex owned by another solid".into()));
        }
        for &(id, _, position) in &points {
            body.vertices.get_mut(id).expect("selected repair vertex is live").position = position;
        }
    }
    let issues = validate_body(body).into_iter().filter(|issue| issue_reaches(&reach, &issue.entity)).count();
    if issues != 0 {
        for &(id, position, _) in &points {
            body.vertices.get_mut(id).expect("rejected repair vertex is live").position = position;
        }
        return Err(KernelError::Operation(format!("heal_solid left {issues} validation issue(s)")));
    }
    for &(id, _, _) in &points {
        rec.record_modified(body.vertices.get(id).expect("validated repair vertex is live").label);
    }
    Ok(HealingReport { vertices_merged: points.len(), ..HealingReport::default() })
}

/// 🧹️ Rebuilds a closed boundary from the stock certified by feature selection.
pub struct DefeatureJob {
    stock: SolidPrimitive,
    plan: Plan,
}

impl DefeatureJob {
    /// 📦️ Stages the restoration of the certified box.
    pub fn restoring_stock(stock: SolidPrimitive) -> Result<Self, KernelError> {
        if !matches!(stock, SolidPrimitive::Box { .. }) {
            return Err(KernelError::InvalidInput("defeature restoration requires box stock".into()));
        }
        Ok(Self { stock, plan: Plan::new(&[("stock", 1)]) })
    }
}

impl StagedOperation for DefeatureJob {
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        self.plan.take().ok_or_else(|| KernelError::Operation("defeature job already finished".into()))?;
        let solid = make_analytic_solid(body, self.stock, rec)?;
        Ok(StageStep::Done(StageOutput::Solid(solid)))
    }
}

/// 🩹 Replaces analytic curves and planes in `solid` with NURBS where conversion exists. `rec`
/// records every face/edge whose geometry pool entry this swaps as modified — the entities
/// themselves keep their labels, only what they point to changes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn convert_to_nurbs(body: &mut Body, solid: SolidId, rec: &mut OpRecorder) -> Result<usize, KernelError> {
    solid_exists(body, solid)?;
    let face_ids = body.solid_faces(solid);
    let mut converted = 0usize;
    let mut surface_done: HashSet<SurfaceId> = HashSet::new();
    for fid in &face_ids {
        let surface_id = body.faces.get(*fid).expect("face").surface;
        if surface_done.contains(&surface_id) {
            continue;
        }
        let Some(surface) = body.surfaces.get(surface_id).cloned() else {
            continue;
        };
        if let Some(nurbs) = analytic_surface_to_nurbs(&surface) {
            *body.surfaces.get_mut(surface_id).expect("surface") = nurbs;
            rec.record_modified(body.faces.get(*fid).expect("face").label);
            converted += 1;
            surface_done.insert(surface_id);
        }
    }
    let mut edge_curves: Vec<(EdgeId, Curve3Id)> = Vec::new();
    for fid in &face_ids {
        for coedge_id in body.face_coedges(*fid) {
            let edge_id = body.coedges.get(coedge_id).expect("coedge").edge;
            if edge_curves.iter().any(|(e, _)| *e == edge_id) {
                continue;
            }
            let edge = body.edges.get(edge_id).expect("edge");
            edge_curves.push((edge_id, edge.curve));
        }
    }
    for (edge_id, curve_id) in edge_curves {
        let Some(curve) = body.curves3.get(curve_id).cloned() else {
            continue;
        };
        if matches!(curve, Curve3::Nurbs { .. }) {
            continue;
        }
        let range = body.edges.get(edge_id).expect("edge").range;
        let nurbs = curve.to_nurbs(range);
        let new_curve = body.curves3.insert(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights });
        let edge = body.edges.get_mut(edge_id).expect("edge");
        edge.curve = new_curve;
        rec.record_modified(edge.label);
        converted += 1;
    }
    Ok(converted)
}

// #endregion 🔖️HealApi

// #region 🔖️HealHelpers

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_exists(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_some() {
        Ok(())
    } else {
        Err(KernelError::MissingEntity(format!("solid {solid}")))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn analytic_surface_to_nurbs(surface: &Surface) -> Option<Surface> {
    match surface {
        Surface::Plane { frame } => {
            let o = frame.origin;
            let controls = vec![vec![o, o + frame.x], vec![o + frame.y, o + frame.x + frame.y]];
            let weights = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
            Some(Surface::Nurbs { u_knots: KnotVector::clamped_uniform(2, 1), v_knots: KnotVector::clamped_uniform(2, 1), controls, weights })
        }
        Surface::Nurbs { .. } => None,
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => None,
    }
}

// #endregion 🔖️HealHelpers

// #region 🧪️SewTests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🧪️SewTests

// #region 🧪️HealTests


// #endregion 🧪️HealTests
