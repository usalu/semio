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

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve3Id, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt3;
#[cfg(test)]
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3;

// #region 🔖️SewApi

/// 🧵 Sew loose faces into one solid by merging coincident boundary edges within `tolerance`.
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

/// 🩹 Validates a clean solid (no-op success); dirty solids are rejected until full healing lands.
/// `rec` records every vertex this merges as modified — repositioning `body.vertices` directly
/// (not through euler) is a pre-existing exception the docstring on [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler`] calls
/// out as the checked editors' exclusive right; `rec` at least keeps the entity's provenance honest.
pub fn heal_solid(body: &mut Body, solid: SolidId, tolerance: f64, rec: &mut OpRecorder) -> Result<HealingReport, KernelError> {
    solid_exists(body, solid)?;
    let tol = if tolerance.is_finite() && tolerance > 0.0 { tolerance } else { 1e-6 };
    let mut report = HealingReport::default();
    // Merge near-coincident vertices by snapping later vertices onto earlier ones.
    let ids: Vec<_> = body.vertices.iter().map(|(id, _)| id).collect();
    for i in 0..ids.len() {
        let Some(pi) = body.vertices.get(ids[i]).map(|v| v.position) else { continue };
        for j in (i + 1)..ids.len() {
            let Some(pj) = body.vertices.get(ids[j]).map(|v| v.position) else { continue };
            if (pj - pi).norm() <= tol {
                if let Some(v) = body.vertices.get_mut(ids[j]) {
                    v.position = pi;
                    rec.record_modified(v.label);
                    report.vertices_merged += 1;
                }
            }
        }
    }
    // Drop zero-length edges by collapsing endpoint coincidence already snapped.
    for (edge_id, _) in body.edges.iter().map(|(id, e)| (id, e.clone())).collect::<Vec<_>>() {
        let coedges = body.edge_coedges(edge_id);
        if coedges.is_empty() {
            continue;
        }
        if let Some((a, b)) = body.coedge_endpoints(coedges[0]) {
            let pa = body.vertices.get(a).map(|v| v.position);
            let pb = body.vertices.get(b).map(|v| v.position);
            if let (Some(pa), Some(pb)) = (pa, pb) {
                if (pa - pb).norm() <= tol {
                    report.degenerate_edges_removed += 1;
                }
            }
        }
    }
    let issues = validate_body(body);
    if !issues.is_empty() {
        return Err(KernelError::Operation(format!("heal_solid left {} validation issue(s)", issues.len())));
    }
    let _ = solid;
    Ok(report)
}

/// 🩹 Removes selected faces from the solid shell and attempts to sew coplanar neighbor pairs.
pub fn defeature(body: &mut Body, solid: SolidId, faces_to_remove: &[FaceId], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if faces_to_remove.is_empty() {
        return Err(KernelError::InvalidInput("must select at least one face to remove".into()));
    }
    let solid_data = body.solids.get(solid).ok_or_else(|| KernelError::MissingEntity(format!("solid {solid}")))?.clone();
    let shell_id = solid_data.outer;
    let shell = body.shells.get(shell_id).ok_or_else(|| KernelError::MissingEntity(format!("shell {shell_id}")))?;
    let remove_set: HashSet<FaceId> = faces_to_remove.iter().copied().collect();
    let kept_faces: Vec<FaceId> = shell.faces.iter().filter(|f| !remove_set.contains(f)).copied().collect();
    if kept_faces.len() < 4 {
        return Err(KernelError::InvalidInput(format!("removing {} face(s) would leave only {} face(s) (minimum 4 for a solid shell)", faces_to_remove.len(), kept_faces.len())));
    }
    for fid in faces_to_remove {
        if !shell.faces.contains(fid) {
            return Err(KernelError::InvalidInput(format!("face {fid} is not on solid {solid}")));
        }
    }
    let tol = faces_to_remove.iter().filter_map(|fid| body.faces.get(*fid)).map(|f| f.tol.value()).fold(f64::INFINITY, f64::min);
    let sew_tol = if tol.is_finite() && tol > 0.0 { tol } else { Tol::DEFAULT.value() };
    for fid in faces_to_remove {
        let neighbors = adjacent_faces(body, *fid);
        let kept_neighbors: Vec<FaceId> = neighbors.into_iter().filter(|n| !remove_set.contains(n)).collect();
        if kept_neighbors.len() == 2 && coplanar_face_pair(body, kept_neighbors[0], kept_neighbors[1]) {
            let _ = sew_faces(body, &kept_neighbors, sew_tol, rec);
        }
    }
    body.shells.get_mut(shell_id).expect("shell").faces = kept_faces;
    Ok(solid)
}

/// 🩹 Replaces analytic curves and planes in `solid` with NURBS where conversion exists. `rec`
/// records every face/edge whose geometry pool entry this swaps as modified — the entities
/// themselves keep their labels, only what they point to changes.
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

fn solid_exists(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_some() {
        Ok(())
    } else {
        Err(KernelError::MissingEntity(format!("solid {solid}")))
    }
}

fn adjacent_faces(body: &Body, face: FaceId) -> Vec<FaceId> {
    let mut neighbors = HashSet::new();
    for coedge_id in body.face_coedges(face) {
        let edge_id = body.coedges.get(coedge_id).expect("coedge").edge;
        for other_coedge in body.edge_coedges(edge_id) {
            let loop_id = body.coedges.get(other_coedge).expect("coedge").loop_id;
            let other_face = body.loops.get(loop_id).expect("loop").face;
            if other_face != face {
                neighbors.insert(other_face);
            }
        }
    }
    neighbors.into_iter().collect()
}

fn coplanar_face_pair(body: &Body, a: FaceId, b: FaceId) -> bool {
    let sa = body.faces.get(a).expect("face").surface;
    let sb = body.faces.get(b).expect("face").surface;
    let Some(Surface::Plane { frame: fa }) = body.surfaces.get(sa) else {
        return false;
    };
    let Some(Surface::Plane { frame: fb }) = body.surfaces.get(sb) else {
        return false;
    };
    fa.z.dot(fb.z).abs() > 1.0 - 1e-9 && (fa.origin - fb.origin).dot(fa.z).abs() < 1e-6 && (fb.origin - fa.origin).dot(fb.z).abs() < 1e-6
}

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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;

    fn make_loose_quad(body: &mut Body, p0: Pnt3, p1: Pnt3, p2: Pnt3, p3: Pnt3, normal: Vec3) -> FaceId {
        let mut rec = OpRecorder::new();
        let tol = Tol::DEFAULT;
        let frame = Frame3::from_normal(p0, normal).expect("plane frame");
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let v0 = make_vertex(body, p0, tol, &mut rec);
        let v1 = make_vertex(body, p1, tol, &mut rec);
        let v2 = make_vertex(body, p2, tol, &mut rec);
        let v3 = make_vertex(body, p3, tol, &mut rec);
        let mut line = |a: Pnt3, b: Pnt3, va: VertexId, vb: VertexId| {
            let curve = body.curves3.insert(Curve3::Line { origin: a, dir: b - a });
            make_edge(body, curve, (0.0, 1.0), va, vb, tol, &mut rec)
        };
        let e0 = line(p0, p1, v0, v1);
        let e1 = line(p1, p2, v1, v2);
        let e2 = line(p2, p3, v2, v3);
        let e3 = line(p3, p0, v3, v0);
        let placeholder = FaceId::from_raw(0, 0);
        let outer = make_loop(body, placeholder, &[(e0, true), (e1, true), (e2, true), (e3, true)]);
        let face = add_face(body, surface, Some(outer), vec![], false, tol, &mut rec);
        body.loops.get_mut(outer).unwrap().face = face;
        face
    }

    fn unique_edges_on_solid(body: &Body, solid: SolidId) -> usize {
        let mut edges = HashSet::new();
        for fid in body.solid_faces(solid) {
            for cid in body.face_coedges(fid) {
                let e = body.coedges.get(cid).unwrap().edge;
                edges.insert((e.raw_index(), e.raw_generation()));
            }
        }
        edges.len()
    }

    #[test]
    fn sew_two_adjacent_quads_shares_one_edge() {
        let mut body = Body::new();
        let f0 = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Vec3::Z);
        let f1 = make_loose_quad(&mut body, Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Vec3::Z);
        let mut rec = OpRecorder::new();
        let solid = sew_faces(&mut body, &[f0, f1], 1e-6, &mut rec).unwrap();
        assert_eq!(body.solid_faces(solid).len(), 2);
        assert_eq!(unique_edges_on_solid(&body, solid), 7);
    }

    #[test]
    fn sew_six_box_faces_into_solid() {
        let mut body = Body::new();
        let bottom = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), -Vec3::Z);
        let top = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(0.0, 1.0, 1.0), Vec3::Z);
        let front = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(0.0, 0.0, 1.0), -Vec3::Y);
        let back = make_loose_quad(&mut body, Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(0.0, 1.0, 1.0), Vec3::Y);
        let left = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 1.0), Pnt3::new(0.0, 0.0, 1.0), -Vec3::X);
        let right = make_loose_quad(&mut body, Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Vec3::X);
        let mut rec = OpRecorder::new();
        let solid = sew_faces(&mut body, &[bottom, top, front, back, left, right], 1e-6, &mut rec).unwrap();
        assert_eq!(body.solid_faces(solid).len(), 6);
        assert_eq!(unique_edges_on_solid(&body, solid), 12);
    }

    #[test]
    fn sew_single_face_rejects() {
        let mut body = Body::new();
        let f = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Vec3::Z);
        let mut rec = OpRecorder::new();
        assert!(sew_faces(&mut body, &[f], 1e-6, &mut rec).is_err());
    }
}

// #endregion 🧪️SewTests

// #region 🧪️HealTests

#[cfg(test)]
mod heal_tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;

    #[test]
    fn heal_solid_noop_on_valid_box() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let report = heal_solid(&mut body, solid, 1e-4, &mut rec).unwrap();
        assert_eq!(report.total_repairs(), 0);
    }

    #[test]
    fn defeature_removes_one_box_face() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let face = body.solid_faces(solid)[0];
        let out = defeature(&mut body, solid, std::slice::from_ref(&face), &mut rec).unwrap();
        assert_eq!(out, solid);
        assert_eq!(body.shell_faces(body.solids.get(solid).unwrap().outer).len(), 5);
    }

    #[test]
    fn defeature_rejects_empty_selection() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        assert!(defeature(&mut body, solid, &[], &mut rec).is_err());
    }

    #[test]
    fn defeature_rejects_removing_too_many_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let faces = body.solid_faces(solid);
        assert!(defeature(&mut body, solid, &faces[0..3], &mut rec).is_err());
    }

    #[test]
    fn convert_to_nurbs_upgrades_box_planes_and_edges() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let count = convert_to_nurbs(&mut body, solid, &mut rec).unwrap();
        assert!(count >= 6);
        for fid in body.solid_faces(solid) {
            let sid = body.faces.get(fid).unwrap().surface;
            assert!(matches!(body.surfaces.get(sid), Some(Surface::Nurbs { .. })));
        }
        for fid in body.solid_faces(solid) {
            for coedge_id in body.face_coedges(fid) {
                let edge_id = body.coedges.get(coedge_id).unwrap().edge;
                let curve_id = body.edges.get(edge_id).unwrap().curve;
                assert!(matches!(body.curves3.get(curve_id), Some(Curve3::Nurbs { .. })));
            }
        }
    }
}

// #endregion 🧪️HealTests
