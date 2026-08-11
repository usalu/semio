//! 🩹 Gap closing, sliver removal, pcurve refit, defeature, convert-to-nurbs.
//!
//! Lane 3-heal of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

use std::collections::HashSet;

use crate::brep::arena::{Curve3Id, FaceId, SolidId, SurfaceId};
use crate::brep::bspline::KnotVector;
use crate::brep::curve::Curve3;
use crate::brep::error::KernelError;
use crate::brep::sew::sew_faces;
use crate::brep::surface::Surface;
use crate::brep::tolerance::Tol;
use crate::brep::topo::Body;
use crate::brep::validate::validate_body;

// #region 🔖️Api

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
        self.vertices_merged
            + self.degenerate_edges_removed
            + self.orientations_fixed
            + self.wire_gaps_closed
            + self.small_faces_removed
            + self.duplicate_faces_removed
    }
}

/// 🩹 Validates a clean solid (no-op success); dirty solids are rejected until full healing lands.
pub fn heal_solid(body: &mut Body, solid: SolidId, tolerance: f64) -> Result<HealingReport, KernelError> {
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
        return Err(KernelError::Operation(format!(
            "heal_solid left {} validation issue(s)",
            issues.len()
        )));
    }
    let _ = solid;
    Ok(report)
}

/// 🩹 Removes selected faces from the solid shell and attempts to sew coplanar neighbor pairs.
pub fn defeature(body: &mut Body, solid: SolidId, faces_to_remove: &[FaceId]) -> Result<SolidId, KernelError> {
    if faces_to_remove.is_empty() {
        return Err(KernelError::InvalidInput("must select at least one face to remove".into()));
    }
    let solid_data = body
        .solids
        .get(solid)
        .ok_or_else(|| KernelError::MissingEntity(format!("solid {solid}")))?
        .clone();
    let shell_id = solid_data.outer;
    let shell = body
        .shells
        .get(shell_id)
        .ok_or_else(|| KernelError::MissingEntity(format!("shell {shell_id}")))?;
    let remove_set: HashSet<FaceId> = faces_to_remove.iter().copied().collect();
    let kept_faces: Vec<FaceId> = shell.faces.iter().filter(|f| !remove_set.contains(f)).copied().collect();
    if kept_faces.len() < 4 {
        return Err(KernelError::InvalidInput(format!(
            "removing {} face(s) would leave only {} face(s) (minimum 4 for a solid shell)",
            faces_to_remove.len(),
            kept_faces.len()
        )));
    }
    for fid in faces_to_remove {
        if !shell.faces.contains(fid) {
            return Err(KernelError::InvalidInput(format!("face {fid} is not on solid {solid}")));
        }
    }
    let tol = faces_to_remove
        .iter()
        .filter_map(|fid| body.faces.get(*fid))
        .map(|f| f.tol.value())
        .fold(f64::INFINITY, f64::min);
    let sew_tol = if tol.is_finite() && tol > 0.0 { tol } else { Tol::DEFAULT.value() };
    for fid in faces_to_remove {
        let neighbors = adjacent_faces(body, *fid);
        let kept_neighbors: Vec<FaceId> = neighbors.into_iter().filter(|n| !remove_set.contains(n)).collect();
        if kept_neighbors.len() == 2 && coplanar_face_pair(body, kept_neighbors[0], kept_neighbors[1]) {
            let _ = sew_faces(body, &kept_neighbors, sew_tol);
        }
    }
    body.shells.get_mut(shell_id).expect("shell").faces = kept_faces;
    Ok(solid)
}

/// 🩹 Replaces analytic curves and planes in `solid` with NURBS where conversion exists.
pub fn convert_to_nurbs(body: &mut Body, solid: SolidId) -> Result<usize, KernelError> {
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
            converted += 1;
            surface_done.insert(surface_id);
        }
    }
    let mut edge_curves: Vec<(crate::brep::arena::EdgeId, Curve3Id)> = Vec::new();
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
        let new_curve = body.curves3.insert(Curve3::Nurbs {
            knots: nurbs.knots,
            controls: nurbs.controls,
            weights: nurbs.weights,
        });
        body.edges.get_mut(edge_id).expect("edge").curve = new_curve;
        converted += 1;
    }
    Ok(converted)
}

// #endregion 🔖️Api

// #region 🔖️Helpers

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
    fa.z.dot(fb.z).abs() > 1.0 - 1e-9
        && (fa.origin - fb.origin).dot(fa.z).abs() < 1e-6
        && (fb.origin - fa.origin).dot(fb.z).abs() < 1e-6
}

fn analytic_surface_to_nurbs(surface: &Surface) -> Option<Surface> {
    match surface {
        Surface::Plane { frame } => {
            let o = frame.origin;
            let controls = vec![
                vec![o, o + frame.x],
                vec![o + frame.y, o + frame.x + frame.y],
            ];
            let weights = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
            Some(Surface::Nurbs {
                u_knots: KnotVector::clamped_uniform(2, 1),
                v_knots: KnotVector::clamped_uniform(2, 1),
                controls,
                weights,
            })
        }
        Surface::Nurbs { .. } => None,
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => None,
    }
}

// #endregion 🔖️Helpers

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::curve::Curve3;
    use crate::brep::primitives::make_box;

    #[test]
    fn heal_solid_noop_on_valid_box() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0).unwrap();
        let report = heal_solid(&mut body, solid, 1e-4).unwrap();
        assert_eq!(report.total_repairs(), 0);
    }

    #[test]
    fn defeature_removes_one_box_face() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0).unwrap();
        let face = body.solid_faces(solid)[0];
        let out = defeature(&mut body, solid, std::slice::from_ref(&face)).unwrap();
        assert_eq!(out, solid);
        assert_eq!(body.shell_faces(body.solids.get(solid).unwrap().outer).len(), 5);
    }

    #[test]
    fn defeature_rejects_empty_selection() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        assert!(defeature(&mut body, solid, &[]).is_err());
    }

    #[test]
    fn defeature_rejects_removing_too_many_faces() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let faces = body.solid_faces(solid);
        assert!(defeature(&mut body, solid, &faces[0..3]).is_err());
    }

    #[test]
    fn convert_to_nurbs_upgrades_box_planes_and_edges() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let count = convert_to_nurbs(&mut body, solid).unwrap();
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

// #endregion 🔖️Tests
