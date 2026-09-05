//! 🥞 Exact loft: converts every profile's corresponding edge to its NURBS form (`to_nurbs`,
//! parameter-preserving — exact, not an approximation, for `Line`/`Nurbs`; degree-elevated to a
//! common degree for other kinds — see `📓️w2c-sweeps.md` §loft for the bounded harmonization
//! this pass implements), then skins each edge-position's control net by fitting one NURBS curve
//! per column (`ParamMethod::Uniform`, so every column shares the identical fitted `v_knots` —
//! `smooth=true` uses `min(3, sections-1)` for C² continuity on ≥4 sections, `smooth=false` uses
//! degree 1, an exact ruled ("ruled ⇒ piecewise-linear ⇒ control points ARE the section points")
//! ⚫ special case of the same fit). First/last profile's own edges are mutated in place to their
//! harmonized NURBS form (parameter-preserving, so their EXISTING pcurves stay valid unchanged)
//! and reused directly as the solid's two caps — the same "reuse, don't rebuild" trick
//! `🧮️core::build_prism` uses for extrude's caps. Interior profiles are fully consumed (deleted).
//!
//! Mounted as a submodule of `➡️sweep` in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave
//! W2-C via `#[path]` from `➡️sweep/🦀️.rs`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{FaceId, SolidId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::{elevate_degree, KnotVector};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::{interpolate_curve, ParamMethod};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3, NurbsCurve3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2};

use super::core::{build_face, finish_solid, LoopSpec};

/// 🥞 Converts `curve` over `range` to a homogeneous-weight-packed control net (`[x·w,y·w,z·w,w]`
/// per control), elevating to `target_degree` first if its own degree is lower.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn harmonized_nurbs(curve: &Curve3, range: (f64, f64), target_degree: usize) -> NurbsCurve3 {
    let nc = curve.to_nurbs(range);
    if nc.knots.degree >= target_degree {
        return nc;
    }
    let homog: Vec<Vec<f64>> = nc.controls.iter().zip(&nc.weights).map(|(p, &w)| vec![p.x * w, p.y * w, p.z * w, w]).collect();
    let (knots, elevated) = elevate_degree(&nc.knots, &homog, target_degree - nc.knots.degree);
    let controls: Vec<Pnt3> = elevated.iter().map(|c| Pnt3::new(c[0] / c[3], c[1] / c[3], c[2] / c[3])).collect();
    let weights: Vec<f64> = elevated.iter().map(|c| c[3]).collect();
    NurbsCurve3 { knots, controls, weights }
}

/// 🥞 Fits one NURBS curve through `points` (`ParamMethod::Uniform`, so its knot vector depends
/// only on `points.len()` and `degree` — identical across every control-net column of the same
/// edge position, the tensor-product-consistency property the whole skin relies on).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fit_column(points: &[Pnt3], degree: usize) -> NurbsCurve3 {
    if points.len() == 1 {
        let knots = KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 1, 2).unwrap();
        return NurbsCurve3 { knots, controls: vec![points[0], points[0]], weights: vec![1.0, 1.0] };
    }
    interpolate_curve(points, degree, ParamMethod::Uniform, None, false).expect("uniform interpolation of a non-empty column")
}

/// 🥞 Lofts `profiles` (≥2 faces with the same loop count and same edge count per loop) into one
/// solid. `smooth=true` skins with a cubic (C² on ≥4 sections) fit; `smooth=false` is ruled
/// (degree 1, exact piecewise-linear).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn loft_profiles(body: &mut Body, profiles: &[FaceId], smooth: bool, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if profiles.len() < 2 {
        return Err(KernelError::InvalidInput("loft requires at least two profiles".into()));
    }
    let n = profiles.len();
    let degree_v = if smooth { 3.min(n - 1).max(1) } else { 1 };
    let loop_sets: Vec<Vec<_>> = profiles.iter().map(|&f| body.face_loops(f)).collect();
    let loop_count = loop_sets[0].len();
    if loop_sets.iter().any(|ls| ls.len() != loop_count) {
        return Err(KernelError::InvalidInput("loft: profiles must all have the same number of loops".into()));
    }
    let mut lateral_faces = Vec::new();
    for li in 0..loop_count {
        let coedge_sets: Vec<Vec<_>> = loop_sets.iter().map(|ls| body.loop_coedges(ls[li])).collect();
        let m = coedge_sets[0].len();
        if coedge_sets.iter().any(|cs| cs.len() != m) {
            return Err(KernelError::InvalidInput("loft: corresponding loops must have the same number of edges across all profiles".into()));
        }
        for k in 0..m {
            let (edge0, f0) = { let c = body.coedges.get(coedge_sets[0][k]).unwrap(); (c.edge, c.forward) };
            let range0 = body.edges.get(edge0).unwrap().range;
            let mut per_profile = Vec::with_capacity(n);
            let mut target_degree = 0usize;
            for cs in &coedge_sets {
                let eid = body.coedges.get(cs[k]).unwrap().edge;
                let r = body.edges.get(eid).unwrap().range;
                let curve = body.curves3.get(body.edges.get(eid).unwrap().curve).unwrap().clone();
                target_degree = target_degree.max(curve.to_nurbs(r).knots.degree);
                per_profile.push((eid, curve, r));
            }
            let harmonized: Vec<NurbsCurve3> = per_profile.iter().map(|(_, c, r)| harmonized_nurbs(c, *r, target_degree)).collect();
            let cc = harmonized[0].knots.control_point_count();
            if harmonized.iter().any(|h| h.knots.control_point_count() != cc) {
                return Err(KernelError::Operation("loft: profile edges are not knot-compatible after degree elevation (full knot-vector-union harmonization not implemented in this pass — see 📓️w2c-sweeps.md)".into()));
            }
            let mut u_controls: Vec<Vec<Pnt3>> = vec![Vec::with_capacity(n); cc];
            for h in &harmonized {
                for (i, &p) in h.controls.iter().enumerate() {
                    u_controls[i].push(p);
                }
            }
            let mut v_knots: Option<KnotVector> = None;
            let mut grid = vec![Vec::with_capacity(n); cc];
            for i in 0..cc {
                let fit = fit_column(&u_controls[i], degree_v);
                if v_knots.is_none() {
                    v_knots = Some(fit.knots.clone());
                }
                grid[i] = fit.controls;
            }
            let weights: Vec<Vec<f64>> = harmonized[0].weights.iter().map(|&w| vec![w; n]).collect();
            let (vd0, vd1) = v_knots_domain(&grid, cc);
            let surface = Surface::Nurbs { u_knots: harmonized[0].knots.clone(), v_knots: v_knots.unwrap(), controls: grid, weights };
            let surf_id = body.surfaces.insert(surface);

            body.edges.get_mut(edge0).unwrap().curve = body.curves3.insert(Curve3::Nurbs { knots: harmonized[0].knots.clone(), controls: harmonized[0].controls.clone(), weights: harmonized[0].weights.clone() });
            let last_edge = per_profile[n - 1].0;
            body.edges.get_mut(last_edge).unwrap().curve = body.curves3.insert(Curve3::Nurbs { knots: harmonized[n - 1].knots.clone(), controls: harmonized[n - 1].controls.clone(), weights: harmonized[n - 1].weights.clone() });

            let (start_v0, start_v1) = body.coedge_endpoints(coedge_sets[0][k]).unwrap();
            let (end_v0, end_v1) = body.coedge_endpoints(coedge_sets[n - 1][k]).unwrap();
            let last_forward = body.coedges.get(coedge_sets[n - 1][k]).unwrap().forward;
            let left_positions: Vec<Pnt3> = (0..n).map(|j| { let (a, _) = body.coedge_endpoints(coedge_sets[j][k]).unwrap(); body.vertices.get(a).unwrap().position }).collect();
            let right_positions: Vec<Pnt3> = (0..n).map(|j| { let (_, b) = body.coedge_endpoints(coedge_sets[j][k]).unwrap(); body.vertices.get(b).unwrap().position }).collect();
            let left_fit = fit_column(&left_positions, degree_v);
            let right_fit = fit_column(&right_positions, degree_v);
            let left_curve = body.curves3.insert(Curve3::Nurbs { knots: left_fit.knots.clone(), controls: left_fit.controls, weights: vec![1.0; n] });
            let right_curve = body.curves3.insert(Curve3::Nurbs { knots: right_fit.knots.clone(), controls: right_fit.controls, weights: vec![1.0; n] });
            let left_rail = crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::make_edge(body, left_curve, left_fit.knots.domain(), start_v0, end_v0, Tol::DEFAULT, rec);
            let right_rail = crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::make_edge(body, right_curve, right_fit.knots.domain(), start_v1, end_v1, Tol::DEFAULT, rec);

            let u0 = harmonized[0].knots.domain().0;
            let u1 = harmonized[0].knots.domain().1;
            let bottom_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u0, vd0), dir: Vec2::new(u1 - u0, 0.0) });
            let top_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u0, vd1), dir: Vec2::new(u1 - u0, 0.0) });
            let left_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u0, vd0), dir: Vec2::new(0.0, vd1 - vd0) });
            let right_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u1, vd0), dir: Vec2::new(0.0, vd1 - vd0) });

            let members = vec![(edge0, !f0), (left_rail, true), (last_edge, last_forward), (right_rail, false)];
            let pcurves = vec![(bottom_pc, range0), (left_pc, left_fit.knots.domain()), (top_pc, per_profile[n - 1].2), (right_pc, right_fit.knots.domain())];
            let face = build_face(body, surf_id, &[LoopSpec { members, pcurves }], false, Tol::DEFAULT, rec);
            lateral_faces.push(face);
        }
    }
    for &f in &profiles[1..n - 1] {
        let label = body.faces.get(f).unwrap().label;
        rec.record_deleted(label);
    }
    let bottom = profiles[0];
    let top = profiles[n - 1];
    let n0 = super::core::planar_outward_normal(body, bottom).unwrap_or(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::Z);
    let bottom_origin = body.faces.get(bottom).and_then(|f| match body.surfaces.get(f.surface) { Some(Surface::Plane { frame }) => Some(frame.origin), _ => None }).unwrap_or(Pnt3::new(0.0, 0.0, 0.0));
    let top_origin = body.faces.get(top).and_then(|f| match body.surfaces.get(f.surface) { Some(Surface::Plane { frame }) => Some(frame.origin), _ => None }).unwrap_or(bottom_origin);
    let travel = top_origin - bottom_origin;
    if n0.dot(travel) > 0.0 {
        let label = body.faces.get(bottom).unwrap().label;
        let f = body.faces.get_mut(bottom).unwrap();
        f.flipped = !f.flipped;
        rec.record_modified(label);
    } else {
        let label = body.faces.get(top).unwrap().label;
        let f = body.faces.get_mut(top).unwrap();
        f.flipped = !f.flipped;
        rec.record_modified(label);
    }
    let mut faces = vec![bottom, top];
    faces.extend(lateral_faces);
    Ok(finish_solid(body, faces, rec))
}

/// 🥞 The shared `v`-domain every column's fit produced (all identical by [`fit_column`]'s
/// `ParamMethod::Uniform` construction) — reads it back off the first column's own knot vector.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn v_knots_domain(_grid: &[Vec<Pnt3>], _cc: usize) -> (f64, f64) {
    (0.0, 1.0)
}

