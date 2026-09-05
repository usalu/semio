//! ↔️ Exact offset surfaces, offset solids (Sharp/Round corners), thicken, shell, and draft.
//!
//! Every operation routes through analytic offset formulas (plane→plane, cylinder r→r±d, cone
//! same-apex-line shifted along the normal, sphere r±d, torus minor r±d) or a control-point NURBS
//! offset refined by knot insertion until the sampled deviation from the true offset is within
//! tolerance. `offset_solid` recomputes every edge as the exact intersection of its two adjacent
//! offset surfaces ([`intersect_surface_surface`], W2-A) and every vertex from the averaged
//! surface normal at that vertex — no tessellate/hull/soup anywhere on this path.
//!
//! Ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave W2-D — see `📓️w2d-blends-offsets-draft.md`.

use std::collections::{HashMap, HashSet};

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::blend::fillet_edges;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_shell, add_solid, make_loop, make_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect::{intersect_surface_surface, IntCurve};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{attach_face, finish_solid, line_edge};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{CoedgeId, EdgeId, FaceId, LoopId, SolidId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::closest_parameter;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve2;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Vec2};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::{IsoDirection, Surface};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::{Affine3, Frame3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

/// ↔️ Default working tolerance for offset topology surgery (edge/vertex recomputation and NURBS
/// offset refinement).
const OFFSET_TOL: f64 = 1e-6;

// #region 🔖️Corner

/// ↔️ Corner treatment for [`offset_solid`]. `Sharp` recomputes every edge/vertex as an exact
/// intersection of the adjacent offset surfaces (correct for planar-only solids and for the
/// concave-trim case, since the SSI naturally produces the trimmed corner). `Round` runs `Sharp`
/// first, then inserts a rolling-ball [`fillet_edges`] strip of radius `|distance|` at every edge
/// — the standard "offset = Minkowski sum with a ball" construction for a convex positive offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetCorner {
    Sharp,
    Round,
}

// #endregion 🔖️Corner

// #region 🔖️SurfaceOffset

/// ↔️ The exact (or, for NURBS, error-bounded) offset of `surface` by `distance` along its own
/// `du × dv` normal convention (independent of any face's `flipped` bit — callers negate
/// `distance` themselves when the face is flipped, matching [`offset_face`]'s existing
/// plane-only convention). Plane/Cylinder/Sphere/Torus/Cone are closed-form; `Nurbs` offsets each
/// control point along the surface normal at its Greville abscissa, then refines by inserting
/// knots (Boehm's algorithm on the homogeneous coordinates, [`KnotVector`]) until the sampled
/// deviation from the true point-wise offset is within `tol`, or errors after a bounded number of
/// refinement rounds rather than silently returning an under-converged surface.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn offset_surface(surface: &Surface, distance: f64, tol: f64) -> Result<Surface, KernelError> {
    if !distance.is_finite() {
        return Err(KernelError::InvalidInput("offset distance must be finite".into()));
    }
    match surface {
        Surface::Plane { frame } => Ok(Surface::Plane { frame: Frame3 { origin: frame.origin + frame.z * distance, x: frame.x, y: frame.y, z: frame.z } }),
        Surface::Cylinder { frame, radius } => {
            let r = radius + distance;
            if r <= tol {
                return Err(KernelError::Operation("cylinder offset collapsed the radius".into()));
            }
            Ok(Surface::Cylinder { frame: *frame, radius: r })
        }
        Surface::Sphere { frame, radius } => {
            let r = radius + distance;
            if r <= tol {
                return Err(KernelError::Operation("sphere offset collapsed the radius".into()));
            }
            Ok(Surface::Sphere { frame: *frame, radius: r })
        }
        Surface::Torus { frame, major_radius, minor_radius } => {
            let r = minor_radius + distance;
            if r <= tol {
                return Err(KernelError::Operation("torus offset collapsed the minor radius".into()));
            }
            Ok(Surface::Torus { frame: *frame, major_radius: *major_radius, minor_radius: r })
        }
        Surface::Cone { frame, half_angle } => {
            let sin_a = half_angle.sin();
            if sin_a.abs() <= 1e-12 {
                return Err(KernelError::Operation("cone offset undefined at zero half-angle".into()));
            }
            let shift = distance / sin_a;
            Ok(Surface::Cone { frame: Frame3 { origin: frame.origin - frame.z * shift, x: frame.x, y: frame.y, z: frame.z }, half_angle: *half_angle })
        }
        Surface::Nurbs { u_knots, v_knots, controls, weights } => offset_nurbs_surface(u_knots, v_knots, controls, weights, distance, tol),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn greville(knots: &KnotVector, i: usize) -> f64 {
    let p = knots.degree;
    let sum: f64 = (i + 1..=i + p).map(|k| knots.knots[k]).sum();
    sum / p as f64
}

/// ↔️ Offsets a NURBS control net along the normal evaluated at each control point's Greville
/// abscissa, refining (inserting a knot at the widest interior span, both directions, per round)
/// until an `8×8` sample grid's deviation from the true point-wise offset is within `tol`, or
/// erroring after 6 rounds rather than returning a silently under-converged surface.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn offset_nurbs_surface(u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], distance: f64, tol: f64) -> Result<Surface, KernelError> {
    let mut u_knots = u_knots.clone();
    let mut v_knots = v_knots.clone();
    let mut controls = controls.to_vec();
    let mut weights = weights.to_vec();
    for _round in 0..8 {
        let original = Surface::Nurbs { u_knots: u_knots.clone(), v_knots: v_knots.clone(), controls: controls.clone(), weights: weights.clone() };
        let mut offset_controls = controls.clone();
        for (i, row) in controls.iter().enumerate() {
            let u = greville(&u_knots, i).clamp(u_knots.domain().0, u_knots.domain().1);
            for (j, &p) in row.iter().enumerate() {
                let v = greville(&v_knots, j).clamp(v_knots.domain().0, v_knots.domain().1);
                let n = original.normal(u, v).unwrap_or(Vec3::Z);
                offset_controls[i][j] = p + n * distance;
            }
        }
        let candidate = Surface::Nurbs { u_knots: u_knots.clone(), v_knots: v_knots.clone(), controls: offset_controls.clone(), weights: weights.clone() };
        let max_err = max_offset_deviation(&original, &candidate, distance, 8);
        if max_err <= tol {
            return Ok(candidate);
        }
        // Bisects EVERY interior span (both directions), not just the single widest one — a
        // widest-span-only strategy stalls (repeatedly refines wherever the FIRST tie sits while
        // the actual worst-deviation region elsewhere never gets finer), confirmed by a direct
        // convergence-trend probe: it plateaued above `tol` for 3 consecutive rounds even given
        // 10 rounds. Uniform h-refinement doubles the span count every round instead, which is
        // guaranteed to shrink the sampled deviation everywhere (a first-order method: halving
        // knot spacing roughly halves the interpolation error at every sample, not just one span).
        for um in all_span_midpoints(&u_knots) {
            let (nu, c1, w1) = insert_u_knot_surface(&u_knots, &controls, &weights, um);
            u_knots = nu;
            controls = c1;
            weights = w1;
        }
        for vm in all_span_midpoints(&v_knots) {
            let (nv, c2, w2) = insert_v_knot_surface(&v_knots, &controls, &weights, vm);
            v_knots = nv;
            controls = c2;
            weights = w2;
        }
    }
    Err(KernelError::Operation("nurbs offset did not converge within tolerance".into()))
}

/// ↔️ Every interior knot span's midpoint (collected up front, from the UNREFINED knot vector) —
/// [`offset_nurbs_surface`]'s per-round uniform refinement target set.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn all_span_midpoints(knots: &KnotVector) -> Vec<f64> {
    knots.knots.windows(2).filter(|w| w[1] - w[0] > 1e-12).map(|w| 0.5 * (w[0] + w[1])).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn max_offset_deviation(original: &Surface, candidate: &Surface, distance: f64, samples: usize) -> f64 {
    let (u0, u1) = original.domain().0;
    let (v0, v1) = original.domain().1;
    let mut max_err = 0.0f64;
    for i in 0..samples {
        for j in 0..samples {
            let u = u0 + (u1 - u0) * (i as f64 + 0.5) / samples as f64;
            let v = v0 + (v1 - v0) * (j as f64 + 0.5) / samples as f64;
            let n = original.normal(u, v).unwrap_or(Vec3::Z);
            let truth = original.eval(u, v) + n * distance;
            let got = candidate.eval(u, v);
            max_err = max_err.max(truth.distance(got));
        }
    }
    max_err
}

/// ↔️ Inserts one `u`-knot across every `v`-row of a control/weight grid via Boehm's algorithm
/// applied to the homogeneous (weighted) coordinates — the standard "insert per channel, divide
/// by the refined weight" technique ([`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::insert_knot`]'s own docstring).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn insert_u_knot_surface(u_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], t: f64) -> (KnotVector, Vec<Vec<Pnt3>>, Vec<Vec<f64>>) {
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::insert_knot;
    let rows = controls.len();
    let cols = if rows > 0 { controls[0].len() } else { 0 };
    let mut new_knots = u_knots.clone();
    let mut new_controls: Vec<Vec<Pnt3>> = Vec::new();
    let mut new_weights: Vec<Vec<f64>> = Vec::new();
    for _ in 0..(rows + 1) {
        new_controls.push(Vec::with_capacity(cols));
        new_weights.push(Vec::with_capacity(cols));
    }
    for j in 0..cols {
        let wx: Vec<f64> = (0..rows).map(|i| controls[i][j].x * weights[i][j]).collect();
        let wy: Vec<f64> = (0..rows).map(|i| controls[i][j].y * weights[i][j]).collect();
        let wz: Vec<f64> = (0..rows).map(|i| controls[i][j].z * weights[i][j]).collect();
        let ww: Vec<f64> = (0..rows).map(|i| weights[i][j]).collect();
        let (kn, nwx) = insert_knot(u_knots, &wx, t);
        let (_, nwy) = insert_knot(u_knots, &wy, t);
        let (_, nwz) = insert_knot(u_knots, &wz, t);
        let (_, nww) = insert_knot(u_knots, &ww, t);
        new_knots = kn;
        for i in 0..nww.len() {
            let w = if nww[i].abs() > 1e-300 { nww[i] } else { 1.0 };
            new_controls[i].push(Pnt3::new(nwx[i] / w, nwy[i] / w, nwz[i] / w));
            new_weights[i].push(nww[i]);
        }
    }
    (new_knots, new_controls, new_weights)
}

/// ↔️ [`insert_u_knot_surface`]'s `v`-direction twin (transposes, reuses the same per-row logic).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn insert_v_knot_surface(v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], t: f64) -> (KnotVector, Vec<Vec<Pnt3>>, Vec<Vec<f64>>) {
    let rows = controls.len();
    let t_controls: Vec<Vec<Pnt3>> = if rows == 0 { Vec::new() } else { (0..controls[0].len()).map(|j| (0..rows).map(|i| controls[i][j]).collect()).collect() };
    let t_weights: Vec<Vec<f64>> = if rows == 0 { Vec::new() } else { (0..weights[0].len()).map(|j| (0..rows).map(|i| weights[i][j]).collect()).collect() };
    let (new_knots, new_t_controls, new_t_weights) = insert_u_knot_surface(v_knots, &t_controls, &t_weights, t);
    let new_cols = new_t_controls.len();
    let new_rows = if new_cols > 0 { new_t_controls[0].len() } else { 0 };
    let controls = (0..new_rows).map(|i| (0..new_cols).map(|j| new_t_controls[j][i]).collect()).collect();
    let weights = (0..new_rows).map(|i| (0..new_cols).map(|j| new_t_weights[j][i]).collect()).collect();
    (new_knots, controls, weights)
}

// #endregion 🔖️SurfaceOffset

// #region 🔖️Face

/// ↔️ Offsets `face`'s surface by `distance` along its outward normal (accounting for
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Face::flipped`]) and rebuilds the SAME loop/p-curve topology on the new
/// surface — exact for every analytic kind since the offset surface shares the original's frame
/// and `(u, v)` domain; for `Nurbs` the trim curves are kept in the same parameter domain
/// (documented approximation — the boundary no longer lies exactly on the offset surface beyond
/// [`offset_surface`]'s own certified deviation bound).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn offset_face(body: &mut Body, face: FaceId, distance: f64, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    if !distance.is_finite() {
        return Err(KernelError::InvalidInput("offset distance must be finite".into()));
    }
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
    let surface = body.surfaces.get(face_data.surface).ok_or_else(|| KernelError::MissingEntity(format!("surface {}", face_data.surface)))?.clone();
    let signed = if face_data.flipped { -distance } else { distance };
    let new_surface = offset_surface(&surface, signed, OFFSET_TOL)?;
    let new_surface_id = body.surfaces.insert(new_surface.clone());

    let mut loops: Vec<LoopId> = Vec::new();
    if let Some(o) = face_data.outer {
        loops.push(o);
    }
    loops.extend(face_data.inners.iter().copied());
    if loops.is_empty() {
        return Err(KernelError::InvalidInput("face has no loops".into()));
    }
    let mut member_lists: Vec<Vec<(EdgeId, bool)>> = Vec::new();
    for lp in &loops {
        let mut members = Vec::new();
        for cid in body.loop_coedges(*lp) {
            let c = body.coedges.get(cid).ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid:?}")))?;
            members.push((c.edge, c.forward));
        }
        member_lists.push(members);
    }
    let outer_members = member_lists[0].clone();
    let inner_members = member_lists[1..].to_vec();
    let new_face = attach_face(body, new_surface_id, &outer_members, face_data.flipped, face_data.tol, rec);
    for members in &inner_members {
        let lp = make_loop(body, new_face, members);
        body.faces.get_mut(new_face).unwrap().inners.push(lp);
    }
    let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
    for lp in &loops {
        for cid in body.loop_coedges(*lp) {
            let c = body.coedges.get(cid).unwrap();
            if edge_geom.contains_key(&c.edge) {
                continue;
            }
            let e = body.edges.get(c.edge).ok_or_else(|| KernelError::MissingEntity(format!("edge {:?}", c.edge)))?;
            let curve = body.curves3.get(e.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve {:?}", e.curve)))?.clone();
            edge_geom.insert(c.edge, (curve, e.range));
        }
    }
    set_face_pcurves(body, new_face, &new_surface, &edge_geom, OFFSET_TOL);
    Ok(new_face)
}

/// ↔️ Sets `pcurve`/`prange` on every coedge of `face`'s loops from `edge_geom` (keyed by the
/// coedge's *own* edge — one 3D curve/range shared by every coedge that uses it) by fitting via
/// [`Surface::project_curve`] (exact shortcut for Plane/Line/Circle, certified fit otherwise).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
/// ↔️ Exact p-curve for a `Curve3::Circle` lying in a `Surface::Plane`, EVEN when the circle's
/// own frame is rotated relative to the plane's `x`/`y` axes (a rebuilt cap boundary, e.g. an
/// offset cylinder's cap circle, is coplanar with its cap plane but virtually never axis-aligned
/// with it — `analytic_pcurve_shortcut`'s own axis-aligned shortcut, `➰️⁄curve/🦀️.rs`, does not
/// cover this). Represented as a `Curve2::Ellipse` with `major_radius == minor_radius` (an exact
/// circle) and `x_axis` the circle's own `x` mapped into the plane's local frame —
/// `Curve2::Ellipse::eval` computes its `y` term via `x_axis.perp()`, a FIXED right-handed
/// rotation in the plane's own local frame; when the circle's frame is left-handed relative to the
/// plane's (its `z` anti-parallel to the plane's `z`), negating `minor_radius` flips that term's
/// sign to match — exact for either handedness, no numeric fit. Bypasses `Surface::project_curve`'s
/// general fit entirely, which measured multiple orders of magnitude too unstable for a full
/// closed loop in this exact scenario (confirmed by a direct debug run: a rebuilt cylinder cap's
/// sampled area/volume via the general fit came out ~10⁵× too large; this exact construction
/// fixes it outright).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn exact_pcurve_for_circle_on_plane(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    let Surface::Plane { frame } = surface else { return None };
    let Curve3::Circle { frame: cf, radius } = curve else { return None };
    if cf.z.cross(frame.z).norm() > 1e-9 {
        return None;
    }
    let lo = frame.to_local(cf.origin);
    if lo.z.abs() > 1e-6 {
        return None;
    }
    let ax = frame.to_local_vector(cf.x);
    let sign = if cf.z.dot(frame.z) >= 0.0 { 1.0 } else { -1.0 };
    Some(Curve2::Ellipse { center: Pnt2::new(lo.x, lo.y), x_axis: Vec2::new(ax.x, ax.y), major_radius: *radius, minor_radius: sign * *radius })
}

/// ↔️ [`exact_pcurve_for_circle_on_plane`]'s twin for a native azimuthal `Curve3::Circle` on a
/// coaxial `Surface::Cylinder` (a rebuilt lateral face's own cap-boundary seam edge): the circle
/// maps to a straight LINE in `(u, v)` — constant `v` (its height along the axis), `u` affine in
/// the circle's own parameter (slope `±1` depending on the two frames' relative handedness) — the
/// same affine relationship the intersect module's own `linear_pcurve_on_axisymmetric` exploits
/// internally for rulings, applied here to the azimuthal direction instead, for the general
/// `Surface::project_curve` call path (which has no such shortcut of its own for a
/// non-plane target). `None` when the circle is not exactly coaxial/co-radius with the cylinder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn exact_pcurve_for_circle_on_cylinder(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    let Surface::Cylinder { frame, radius } = surface else { return None };
    let Curve3::Circle { frame: cf, radius: cr } = curve else { return None };
    if (cr - radius).abs() > 1e-9 {
        return None;
    }
    if cf.z.cross(frame.z).norm() > 1e-9 {
        return None;
    }
    let rel = cf.origin - frame.origin;
    let v0 = rel.dot(frame.z);
    if (rel - frame.z * v0).norm() > 1e-6 {
        return None;
    }
    let cx = frame.x.dot(cf.x);
    let cy = frame.y.dot(cf.x);
    let phi = cy.atan2(cx);
    let sign = if cf.z.dot(frame.z) >= 0.0 { 1.0 } else { -1.0 };
    Some(Curve2::Line { origin: Pnt2::new(phi, v0), dir: Vec2::new(sign, 0.0) })
}

/// ↔️ [`exact_pcurve_for_circle_on_cylinder`]'s ruling-direction twin: a `Curve3::Line` parallel to
/// a coaxial `Surface::Cylinder`'s own axis (a rebuilt lateral face's own seam edge) maps to a
/// straight `(u, v)` line too — constant `u`, `v` affine in the line's own parameter.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn exact_pcurve_for_line_on_cylinder(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    let Surface::Cylinder { frame, radius } = surface else { return None };
    let Curve3::Line { origin, dir } = curve else { return None };
    if dir.cross(frame.z).norm() > 1e-9 * dir.norm().max(1.0) {
        return None;
    }
    let rel = *origin - frame.origin;
    let v0 = rel.dot(frame.z);
    let radial = rel - frame.z * v0;
    if (radial.norm() - radius).abs() > 1e-6 {
        return None;
    }
    let u0 = (frame.y.dot(radial) / radius).atan2(frame.x.dot(radial) / radius);
    Some(Curve2::Line { origin: Pnt2::new(u0, v0), dir: Vec2::new(0.0, dir.dot(frame.z)) })
}

/// ↔️ Tries every exact (non-fitted) p-curve shortcut this file knows for a rebuilt face's own
/// surface/curve pair before falling back to [`Surface::project_curve`]'s general numeric fit —
/// see [`exact_pcurve_for_circle_on_plane`], [`exact_pcurve_for_circle_on_cylinder`] and
/// [`exact_pcurve_for_line_on_cylinder`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn exact_pcurve(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    exact_pcurve_for_circle_on_plane(surface, curve).or_else(|| exact_pcurve_for_circle_on_cylinder(surface, curve)).or_else(|| exact_pcurve_for_line_on_cylinder(surface, curve))
}

pub(crate) fn set_face_pcurves(body: &mut Body, face: FaceId, surface: &Surface, edge_geom: &HashMap<EdgeId, (Curve3, (f64, f64))>, tol: f64) {
    let face_data = body.faces.get(face).unwrap();
    let mut loops = Vec::new();
    if let Some(o) = face_data.outer {
        loops.push(o);
    }
    loops.extend(face_data.inners.iter().copied());
    for lp in loops {
        for cid in body.loop_coedges(lp) {
            let edge = body.coedges.get(cid).unwrap().edge;
            let Some((curve, range)) = edge_geom.get(&edge) else { continue };
            let pcurve = exact_pcurve(surface, curve).unwrap_or_else(|| surface.project_curve(curve, *range, tol));
            let pcurve_id = body.curves2.insert(pcurve);
            let c = body.coedges.get_mut(cid).unwrap();
            c.pcurve = Some(pcurve_id);
            c.prange = *range;
        }
    }
}

// #endregion 🔖️Face

// #region 🔖️Topology

/// ↔️ Result of rebuilding a solid's boundary against a per-face surface substitution: original
/// face/edge/vertex ids mapped to their newly-built counterparts (only for entities that actually
/// needed rebuilding — see [`rebuild_topology`]).
struct RebuiltTopology {
    face_new: HashMap<FaceId, FaceId>,
    edge_new: HashMap<EdgeId, (EdgeId, Curve3, (f64, f64))>,
    vertex_new: HashMap<VertexId, VertexId>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_edges(body: &Body, solid_faces: &HashSet<FaceId>) -> HashSet<EdgeId> {
    let mut edges = HashSet::new();
    for &f in solid_faces {
        for cid in body.face_coedges(f) {
            if let Some(c) = body.coedges.get(cid) {
                edges.insert(c.edge);
            }
        }
    }
    edges
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_vertices(body: &Body, edges: &HashSet<EdgeId>) -> HashSet<VertexId> {
    let mut verts = HashSet::new();
    for &e in edges {
        if let Some(edge) = body.edges.get(e) {
            verts.insert(edge.v0);
            verts.insert(edge.v1);
        }
    }
    verts
}

/// ↔️ Unique faces (within `solid_faces`) touching `edge` — length 1 for a self-adjacent seam
/// edge (both coedges land on the same face), length 2 for a real dihedral edge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_unique_faces(body: &Body, solid_faces: &HashSet<FaceId>, edge: EdgeId) -> Vec<FaceId> {
    let mut faces = Vec::new();
    for cid in body.edge_coedges(edge) {
        if let Some(c) = body.coedges.get(cid) {
            if let Some(lp) = body.loops.get(c.loop_id) {
                if solid_faces.contains(&lp.face) && !faces.contains(&lp.face) {
                    faces.push(lp.face);
                }
            }
        }
    }
    faces
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_on_face(body: &Body, edge: EdgeId, face: FaceId) -> Option<CoedgeId> {
    body.edge_coedges(edge).into_iter().find(|&cid| body.coedges.get(cid).and_then(|c| body.loops.get(c.loop_id)).map(|lp| lp.face) == Some(face))
}

/// ↔️ Original surface normal at `face`, evaluated at whichever end of `coedge`'s pcurve names
/// `vertex` (`prange.0` if `vertex == edge.v0`, else `prange.1` — the p-curve convention never
/// reverses for `forward`), or at the pcurve's midpoint when `vertex` is `None` (edge-midpoint
/// anchor use). Accounts for [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Face::flipped`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_normal_at(body: &Body, face: FaceId, coedge: CoedgeId, vertex: Option<VertexId>) -> Result<Vec3, KernelError> {
    let c = body.coedges.get(coedge).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
    let e = body.edges.get(c.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surf = body.surfaces.get(face_data.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let mut n = if let Surface::Plane { frame } = surf {
        // A planar face's normal is position-independent — no pcurve needed (planar faces don't
        // always carry one, W1-E's own documented convention).
        frame.z
    } else if let Some(pid) = c.pcurve {
        let param = match vertex {
            Some(v) if v == e.v0 => c.prange.0,
            Some(v) if v == e.v1 => c.prange.1,
            _ => 0.5 * (c.prange.0 + c.prange.1),
        };
        let pc = body.curves2.get(pid).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?;
        let uv = pc.eval(param);
        normal_with_pole_fallback(surf, uv.x, uv.y)?
    } else {
        let t_param = match vertex {
            Some(v) if v == e.v0 => e.range.0,
            Some(v) if v == e.v1 => e.range.1,
            _ => 0.5 * (e.range.0 + e.range.1),
        };
        let curve = body.curves3.get(e.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
        let point = curve.eval(t_param);
        let cu = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv(surf, surf.domain(), point, OFFSET_TOL);
        normal_with_pole_fallback(surf, cu.u, cu.v)?
    };
    if face_data.flipped {
        n = -n;
    }
    Ok(n)
}

/// ↔️ [`Surface::normal`] degenerates (`du × dv = 0`) exactly at a pole/apex singularity (e.g. a
/// sphere's `v = ±π/2`); nudges `v` toward the domain interior by a small epsilon and retries once
/// rather than failing outright on a station that legitimately sits at the singularity.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normal_with_pole_fallback(surf: &Surface, u: f64, v: f64) -> Result<Vec3, KernelError> {
    if let Some(n) = surf.normal(u, v) {
        return Ok(n);
    }
    let (v0, v1) = surf.domain().1;
    let eps = 1e-6 * (v1 - v0).abs().max(1.0);
    let v_try = if (v - v0).abs() < (v1 - v).abs() { v + eps } else { v - eps };
    surf.normal(u, v_try).ok_or_else(|| KernelError::Operation("degenerate surface normal".into()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn average_normal(normals: &[Vec3]) -> Result<Vec3, KernelError> {
    let mut sum = Vec3::ZERO;
    for &n in normals {
        sum = sum + n;
    }
    sum.normalized().ok_or_else(|| KernelError::Operation("averaged normal is degenerate".into()))
}

/// ↔️ Gaussian elimination with partial pivoting for the symmetric `3×3` system `a·x = b`; `None`
/// when `a` is numerically singular (touched normals collapse to fewer than 3 independent
/// directions — callers fall back to a lower-dimensional-safe approximation in that case).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solve3(mut a: [[f64; 3]; 3], mut b: [f64; 3]) -> Option<Vec3> {
    for col in 0..3 {
        let mut piv = col;
        for r in (col + 1)..3 {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        if a[piv][col].abs() < 1e-12 {
            return None;
        }
        a.swap(col, piv);
        b.swap(col, piv);
        for r in (col + 1)..3 {
            let f = a[r][col] / a[col][col];
            for c in col..3 {
                a[r][c] -= f * a[col][c];
            }
            b[r] -= f * b[col];
        }
    }
    let mut x = [0.0_f64; 3];
    for i in (0..3).rev() {
        let mut s = b[i];
        for j in (i + 1)..3 {
            s -= a[i][j] * x[j];
        }
        x[i] = s / a[i][i];
    }
    Some(Vec3::new(x[0], x[1], x[2]))
}

/// ↔️ The exact vertex displacement `Δ` satisfying `n_i · Δ = distance` for every touched face's
/// (outward-corrected) normal `n_i` — the real constraint a uniform-`distance` offset places on a
/// vertex shared by `k` faces (each face's plane/surface genuinely shifts by `distance` along its
/// own normal, W1-E/W2-D's own offset convention): solved via the `3×3` normal equations
/// `(NᵀN)·Δ = distanceΣn_i` (exact for the common `k ≤ 3` independent-normal case — a box corner's
/// three mutually orthogonal planes reduce this to `Δ = distance·(n0+n1+n2)`, NOT the naive
/// normalized-average-direction guess a single-face vertex would use). Falls back to the
/// normalized-average-direction approximation only when the touched normals are numerically
/// collinear (a smooth, non-corner vertex — `k` directions but 1 independent one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solve_vertex_displacement(normals: &[Vec3], distance: f64) -> Vec3 {
    let mut a = [[0.0_f64; 3]; 3];
    let mut b = [0.0_f64; 3];
    for n in normals {
        let v = [n.x, n.y, n.z];
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] += v[i] * v[j];
            }
            b[i] += v[i] * distance;
        }
    }
    a[0][0] += 1e-12;
    a[1][1] += 1e-12;
    a[2][2] += 1e-12;
    solve3(a, b).unwrap_or_else(|| {
        let sum = normals.iter().fold(Vec3::ZERO, |acc, &n| acc + n);
        sum.normalized().map(|u| u * distance).unwrap_or(Vec3::ZERO)
    })
}

/// ↔️ The point common to every plane in `planes` (unit normal, signed offset `n·x = c` pairs) —
/// the exact multi-face vertex position for [`draft_angle`], where each touched face's plane
/// equation is already fully known (rotated for a drafted face, unchanged otherwise), unlike
/// [`solve_vertex_displacement`]'s uniform-`distance` delta form. Solved via the same `3×3` normal
/// equations `(NᵀN)·x = Nᵀc`; `None` when fewer than 3 independent planes are touched (falls back to
/// the caller's own anchor, e.g. the original vertex position).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solve_plane_point(planes: &[(Vec3, f64)]) -> Option<Pnt3> {
    let mut a = [[0.0_f64; 3]; 3];
    let mut b = [0.0_f64; 3];
    for &(n, c) in planes {
        let v = [n.x, n.y, n.z];
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] += v[i] * v[j];
            }
            b[i] += v[i] * c;
        }
    }
    a[0][0] += 1e-12;
    a[1][1] += 1e-12;
    a[2][2] += 1e-12;
    solve3(a, b).map(|v| Pnt3::new(v.x, v.y, v.z))
}

/// ↔️ Reparametrizes an isocurve `curve3(s) = surface.eval(const, s)` (or `eval(s, const)`) so it
/// is directly evaluable at the *original* edge parameter `t` where `s = offset + scale·t` —
/// exact for `Line`/`Circle`/`Ellipse` when `scale` is `±1` (the unit-speed convention every
/// primitive in this kernel uses for its own seam p-curves, W1-E); `Nurbs` and any non-unit-speed
/// input pass through unreparametrized (documented limitation).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_surface(body: &Body, f: FaceId, new_surface_map: &HashMap<FaceId, Surface>) -> Result<Surface, KernelError> {
    if let Some(s) = new_surface_map.get(&f) {
        return Ok(s.clone());
    }
    let fd = body.faces.get(f).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    Ok(body.surfaces.get(fd.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?.clone())
}

/// ↔️ Rebuilds `solid`'s boundary against `new_surface_map` (a face present here gets that new
/// surface; a face absent keeps its original surface/edges/vertices verbatim). Every edge touching
/// at least one changed face is recomputed: a self-adjacent seam edge via the changed face's own
/// isocurve trimmed to its two already-repositioned vertex targets (exact, no per-surface-kind
/// special case); a degenerate pole edge by re-evaluating its (already relocated) vertex; a
/// real dihedral edge as the exact intersection of the two (possibly one unchanged) adjacent
/// surfaces via [`intersect_surface_surface`], with the branch and trim range selected by
/// proximity to `vertex_target`/`edge_target`. Only faces in `materialize` are actually rebuilt
/// into new [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Face`]s (used by [`shell_solid_with_open_faces`] to
/// skip the removed open faces while still using their offset surface to trim the kept faces).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rebuild_topology<FV, FE, FF>(
    body: &mut Body,
    solid: SolidId,
    new_surface_map: &HashMap<FaceId, Surface>,
    materialize: &HashSet<FaceId>,
    flip_new: FF,
    vertex_target: FV,
    edge_target: FE,
    tol: f64,
    rec: &mut OpRecorder,
) -> Result<RebuiltTopology, KernelError>
where
    FV: Fn(&Body, VertexId, &[(FaceId, Vec3)]) -> Pnt3,
    FE: Fn(&Body, EdgeId, Vec3) -> Pnt3,
    FF: Fn(FaceId) -> bool,
{
    let solid_faces_vec = body.solid_faces(solid);
    let solid_faces: HashSet<FaceId> = solid_faces_vec.iter().copied().collect();
    let edges = solid_edges(body, &solid_faces);
    let vertices = solid_vertices(body, &edges);

    // Pass 1: vertex positions for every vertex touching at least one changed face — every
    // touched face's own (deduplicated) outward normal is handed to `vertex_target` so it can
    // solve the exact multi-face intersection ([`solve_vertex_displacement`]/[`solve_plane_point`])
    // rather than a single naively-averaged direction (wrong at any real corner, see both helpers'
    // docstrings).
    let mut vertex_pos: HashMap<VertexId, Pnt3> = HashMap::new();
    for &v in &vertices {
        let mut touched = false;
        let mut touched_faces: Vec<(FaceId, Vec3)> = Vec::new();
        for e in body.vertex_edges(v) {
            if !edges.contains(&e) {
                continue;
            }
            for f in edge_unique_faces(body, &solid_faces, e) {
                if let Some(cid) = coedge_on_face(body, e, f) {
                    if new_surface_map.contains_key(&f) {
                        touched = true;
                    }
                    if !touched_faces.iter().any(|&(ef, _)| ef == f) {
                        touched_faces.push((f, face_normal_at(body, f, cid, Some(v))?));
                    }
                }
            }
        }
        if !touched {
            continue;
        }
        vertex_pos.insert(v, vertex_target(body, v, &touched_faces));
    }
    let mut vertex_new: HashMap<VertexId, VertexId> = HashMap::new();
    for (&v, &pos) in &vertex_pos {
        let tol_v = body.vertices.get(v).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.tol;
        vertex_new.insert(v, make_vertex(body, pos, tol_v, rec));
    }

    // Pass 2: edges touching at least one changed face.
    let mut edge_new: HashMap<EdgeId, (EdgeId, Curve3, (f64, f64))> = HashMap::new();
    for &e in &edges {
        let faces_here = edge_unique_faces(body, &solid_faces, e);
        if !faces_here.iter().any(|f| new_surface_map.contains_key(f)) {
            continue;
        }
        let edge_ent = body.edges.get(e).ok_or_else(|| KernelError::MissingEntity("edge".into()))?.clone();
        let orig_curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?.clone();
        let is_degenerate = edge_ent.v0 == edge_ent.v1 && matches!(&orig_curve, Curve3::Line { dir, .. } if dir.norm() < 1e-12);
        let nv0 = vertex_new.get(&edge_ent.v0).copied().unwrap_or(edge_ent.v0);
        let nv1 = vertex_new.get(&edge_ent.v1).copied().unwrap_or(edge_ent.v1);

        if is_degenerate {
            let pos = vertex_pos.get(&edge_ent.v0).copied().unwrap_or(body.vertices.get(edge_ent.v0).unwrap().position);
            let new_curve = Curve3::Line { origin: pos, dir: Vec3::ZERO };
            let cid = body.curves3.insert(new_curve.clone());
            let ne = make_edge_entry(body, cid, edge_ent.range, nv0, nv0, edge_ent.tol, rec);
            edge_new.insert(e, (ne, new_curve, edge_ent.range));
            continue;
        }

        if faces_here.len() == 1 {
            let f = faces_here[0];
            let ns = face_surface(body, f, new_surface_map)?;
            let cid_coedge = coedge_on_face(body, e, f).ok_or_else(|| KernelError::Operation("seam edge missing coedge".into()))?;
            let pid = body.coedges.get(cid_coedge).unwrap().pcurve.ok_or_else(|| KernelError::Operation("seam edge missing pcurve".into()))?;
            let pc = body.curves2.get(pid).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?.clone();
            let (dir, konst) = match pc {
                Curve2::Line { origin, dir } if dir.x.abs() < 1e-9 => (IsoDirection::U, origin.x),
                Curve2::Line { origin, dir } if dir.y.abs() < 1e-9 => (IsoDirection::V, origin.y),
                _ => return Err(KernelError::Operation("seam edge pcurve is not axis-aligned".into())),
            };
            // The isocurve at the seam's own (unchanged) `u`/`v` constant is exact and already
            // correctly positioned on the NEW surface (e.g. the new radius) — but its own native
            // parametrization has no reason to still line up with `edge_ent.range` (growing/
            // shrinking a solid moves the CAPS a self-adjacent lateral seam spans between, even
            // though the lateral surface's own frame — hence its isocurve's own v=0 origin — does
            // not move at all). Trimming it to the two already-correctly-repositioned vertex
            // targets (same technique the real-dihedral-edge branch below uses) is exact for any
            // op (offset, shell, draft) and needs no per-surface-kind "seam shift" special case —
            // this replaced a previous version that reused the SEAM'S OLD p-curve's `offset`/
            // `scale` verbatim (silently wrong the moment a cap moves, confirmed by a direct debug
            // run: a rebuilt offset cylinder's seam spanned its OLD z-range, not the new one).
            let iso = ns.isocurve(dir, konst);
            let v0_target = vertex_pos.get(&edge_ent.v0).copied().ok_or_else(|| KernelError::Operation("offset/draft: vertex not repositioned".into()))?;
            let v1_target = vertex_pos.get(&edge_ent.v1).copied().ok_or_else(|| KernelError::Operation("offset/draft: vertex not repositioned".into()))?;
            let search_domain = if matches!(iso, Curve3::Line { .. }) { (-1.0e6, 1.0e6) } else { iso.domain() };
            let t0 = closest_parameter(&iso, search_domain, v0_target, tol).t;
            let t1 = closest_parameter(&iso, search_domain, v1_target, tol).t;
            let new_curve = iso;
            let cid = body.curves3.insert(new_curve.clone());
            let ne = make_edge_entry(body, cid, (t0, t1), nv0, nv1, edge_ent.tol, rec);
            edge_new.insert(e, (ne, new_curve, (t0, t1)));
            continue;
        }

        if faces_here.len() != 2 {
            return Err(KernelError::Operation("offset/draft edge has an unexpected number of adjacent faces".into()));
        }
        let (fa, fb) = (faces_here[0], faces_here[1]);
        let sa = face_surface(body, fa, new_surface_map)?;
        let sb = face_surface(body, fb, new_surface_map)?;
        let candidates: Vec<IntCurve> = intersect_surface_surface(&sa, &sb, tol)?;
        if candidates.is_empty() {
            return Err(KernelError::Operation("offset/draft: adjacent offset surfaces do not intersect".into()));
        }
        let mut mid_normals = Vec::new();
        for f in [fa, fb] {
            if let Some(cid) = coedge_on_face(body, e, f) {
                mid_normals.push(face_normal_at(body, f, cid, None)?);
            }
        }
        let mid_n = average_normal(&mid_normals).unwrap_or(Vec3::Z);
        let anchor = edge_target(body, e, mid_n);
        let mut best: Option<(&IntCurve, f64)> = None;
        for cand in &candidates {
            let cp = closest_parameter(&cand.curve3, (cand.domain.min, cand.domain.max), anchor, tol);
            if best.as_ref().map(|(_, d)| cp.distance < *d).unwrap_or(true) {
                best = Some((cand, cp.distance));
            }
        }
        let (chosen, _) = best.unwrap();
        let new_curve = chosen.curve3.clone();
        let domain = (chosen.domain.min, chosen.domain.max);
        // A CLOSED edge (`v0 == v1`, e.g. a cylinder cap's own full-circle boundary) has no
        // second vertex to independently project onto the curve, but it still needs `t0` (hence
        // `t1 = t0 + period`) to land at the SHARED VERTEX's own position, not merely anywhere
        // on the curve — this edge's neighbour (a self-adjacent lateral seam, say) starts exactly
        // where this one's `curve.eval(t0)` sits, and `loop_uv_polygon` stitches consecutive
        // coedges by CONTINUITY, not by re-deriving positions; landing `t0` anywhere else (e.g.
        // the SSI candidate's own arbitrary domain start) silently opens a gap in the (u, v)
        // boundary polygon there, corrupting the sampled area/volume (confirmed directly: with
        // `t0 = domain.0` verbatim the rebuilt cylinder's lateral area came out ~15% too high).
        let (t0, t1) = if edge_ent.v0 == edge_ent.v1 {
            let v0_target = vertex_pos.get(&edge_ent.v0).copied().ok_or_else(|| KernelError::Operation("offset/draft: vertex not repositioned".into()))?;
            let t0 = closest_parameter(&new_curve, domain, v0_target, tol).t;
            (t0, t0 + (domain.1 - domain.0))
        } else {
            let v0_target = vertex_pos.get(&edge_ent.v0).copied().ok_or_else(|| KernelError::Operation("offset/draft: vertex not repositioned".into()))?;
            let v1_target = vertex_pos.get(&edge_ent.v1).copied().ok_or_else(|| KernelError::Operation("offset/draft: vertex not repositioned".into()))?;
            (closest_parameter(&new_curve, domain, v0_target, tol).t, closest_parameter(&new_curve, domain, v1_target, tol).t)
        };
        let cid = body.curves3.insert(new_curve.clone());
        let ne = make_edge_entry(body, cid, (t0, t1), nv0, nv1, edge_ent.tol, rec);
        edge_new.insert(e, (ne, new_curve, (t0, t1)));
    }

    // Pass 3: materialized faces.
    let mut face_new: HashMap<FaceId, FaceId> = HashMap::new();
    for &f in &solid_faces {
        if !materialize.contains(&f) {
            continue;
        }
        let face_data = body.faces.get(f).ok_or_else(|| KernelError::MissingEntity("face".into()))?.clone();
        let ns = face_surface(body, f, new_surface_map)?;
        let ns_id = body.surfaces.insert(ns.clone());
        let mut loops = Vec::new();
        if let Some(o) = face_data.outer {
            loops.push(o);
        }
        loops.extend(face_data.inners.iter().copied());
        if loops.is_empty() {
            return Err(KernelError::Operation("face has no loops".into()));
        }
        let mut member_lists: Vec<Vec<(EdgeId, bool)>> = Vec::new();
        for lp in &loops {
            let mut members = Vec::new();
            for cid in body.loop_coedges(*lp) {
                let c = body.coedges.get(cid).unwrap();
                let new_edge = edge_new.get(&c.edge).map(|(ne, _, _)| *ne).unwrap_or(c.edge);
                members.push((new_edge, c.forward));
            }
            member_lists.push(members);
        }
        let flipped = flip_new(f);
        let new_face = attach_face(body, ns_id, &member_lists[0], flipped, face_data.tol, rec);
        for members in &member_lists[1..] {
            let lp = make_loop(body, new_face, members);
            body.faces.get_mut(new_face).unwrap().inners.push(lp);
        }
        let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
        for lp in &loops {
            for cid in body.loop_coedges(*lp) {
                let c = body.coedges.get(cid).unwrap();
                if let Some((ne, curve, range)) = edge_new.get(&c.edge) {
                    edge_geom.insert(*ne, (curve.clone(), *range));
                } else {
                    let e = body.edges.get(c.edge).unwrap();
                    let curve = body.curves3.get(e.curve).unwrap().clone();
                    edge_geom.insert(c.edge, (curve, e.range));
                }
            }
        }
        set_face_pcurves(body, new_face, &ns, &edge_geom, tol);
        face_new.insert(f, new_face);
    }

    Ok(RebuiltTopology { face_new, edge_new, vertex_new })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn make_edge_entry(body: &mut Body, curve: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::make_edge(body, curve, range, v0, v1, tol, rec)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_planar_only(body: &Body, solid: SolidId) -> bool {
    body.solid_faces(solid).iter().all(|&f| body.faces.get(f).and_then(|fd| body.surfaces.get(fd.surface)).map(|s| matches!(s, Surface::Plane { .. })).unwrap_or(false))
}

// #endregion 🔖️Topology

// #region 🔖️OffsetSolid

/// ↔️ Uniform solid offset (positive expands, negative shrinks) with an explicit corner policy —
/// see [`OffsetCorner`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn offset_solid_with_corner(body: &mut Body, solid: SolidId, distance: f64, corner: OffsetCorner, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if !distance.is_finite() {
        return Err(KernelError::InvalidInput("offset distance must be finite".into()));
    }
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity(format!("solid {solid}")));
    }
    if distance.abs() <= 1e-15 {
        return Err(KernelError::Operation("offset distance must be non-zero".into()));
    }
    let tol = OFFSET_TOL;
    let faces_vec = body.solid_faces(solid);
    let solid_faces: HashSet<FaceId> = faces_vec.iter().copied().collect();
    let mut new_surface_map = HashMap::new();
    for &f in &solid_faces {
        let fd = body.faces.get(f).unwrap().clone();
        let s = body.surfaces.get(fd.surface).unwrap().clone();
        let signed = if fd.flipped { -distance } else { distance };
        new_surface_map.insert(f, offset_surface(&s, signed, tol)?);
    }
    let vertex_target = |b: &Body, v: VertexId, touched: &[(FaceId, Vec3)]| -> Pnt3 {
        let normals: Vec<Vec3> = touched.iter().map(|&(_, n)| n).collect();
        b.vertices.get(v).unwrap().position + solve_vertex_displacement(&normals, distance)
    };
    let edge_target = |b: &Body, e: EdgeId, n: Vec3| -> Pnt3 {
        let edge = b.edges.get(e).unwrap();
        let curve = b.curves3.get(edge.curve).unwrap();
        curve.eval(0.5 * (edge.range.0 + edge.range.1)) + n * distance
    };
    let flip_new = |_f: FaceId| false;
    let materialize = solid_faces.clone();
    let rebuilt = rebuild_topology(body, solid, &new_surface_map, &materialize, flip_new, vertex_target, edge_target, tol, rec)?;
    let mut faces: Vec<FaceId> = Vec::with_capacity(faces_vec.len());
    for &f in &faces_vec {
        faces.push(*rebuilt.face_new.get(&f).ok_or_else(|| KernelError::Operation("offset_solid: face was not rebuilt".into()))?);
    }
    let sharp_solid = finish_solid(body, faces, rec);
    match corner {
        OffsetCorner::Sharp => Ok(sharp_solid),
        OffsetCorner::Round => {
            let round_faces: HashSet<FaceId> = body.solid_faces(sharp_solid).into_iter().collect();
            // Only real dihedral edges (shared by two *distinct* faces) are meaningful fillet
            // targets — a self-adjacent seam edge (e.g. a cylinder's own lateral seam) has no
            // second face to blend against and is skipped.
            let edges: Vec<EdgeId> = solid_edges(body, &round_faces).into_iter().filter(|&e| edge_unique_faces(body, &round_faces, e).len() == 2).collect();
            if edges.is_empty() {
                return Ok(sharp_solid);
            }
            fillet_edges(body, sharp_solid, &edges, distance.abs(), rec)
        }
    }
}

/// ↔️ [`offset_solid_with_corner`] with the default corner policy: `Sharp` for a planar-only solid
/// (a box), `Round` otherwise (a rolling-ball offset for solids that already carry curved faces).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn offset_solid(body: &mut Body, solid: SolidId, distance: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let corner = if is_planar_only(body, solid) { OffsetCorner::Sharp } else { OffsetCorner::Round };
    offset_solid_with_corner(body, solid, distance, corner, rec)
}

// #endregion 🔖️OffsetSolid

// #region 🔖️Thicken

/// ↔️ [`ruled_surface_from_curves`], `pub(crate)` so [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::blend`]'s chamfer can reuse the same
/// straight-ruling construction between its two tangent-line boundaries.
/// ↔️ Builds a ruled surface between two boundary curves sharing the same analytic curve kind and
/// parameter range (the case every caller in this file produces, since an offset/rim edge is
/// always derived from its counterpart with the same `range`): fits both to NURBS over their own
/// range and lofts a degree-1 `v`-direction between the two matching control nets. Errors — rather
/// than silently approximating — when the two `to_nurbs` fits are not control-point compatible.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn ruled_surface_from_curves(c0: &Curve3, r0: (f64, f64), c1: &Curve3, r1: (f64, f64)) -> Result<Surface, KernelError> {
    let n0 = c0.to_nurbs(r0);
    let n1 = c1.to_nurbs(r1);
    if n0.controls.len() != n1.controls.len() || n0.knots.knots.len() != n1.knots.knots.len() {
        return Err(KernelError::Operation("thicken/shell: ruling curves are not control-point compatible".into()));
    }
    let v_knots = KnotVector { knots: vec![0.0, 0.0, 1.0, 1.0], degree: 1 };
    let mut controls = Vec::with_capacity(n0.controls.len());
    let mut weights = Vec::with_capacity(n0.controls.len());
    for i in 0..n0.controls.len() {
        controls.push(vec![n0.controls[i], n1.controls[i]]);
        weights.push(vec![n0.weights[i], n1.weights[i]]);
    }
    Ok(Surface::Nurbs { u_knots: n0.knots, v_knots, controls, weights })
}

/// ↔️ Builds one ruled side face per boundary coedge of `cap0`'s outer loop, connecting it to the
/// corresponding coedge of `cap1` (same index, same count — guaranteed since `cap1` was built by
/// [`offset_face`] from `cap0`'s own loop structure).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_ruled_sides(body: &mut Body, cap0: FaceId, cap1: FaceId, tol: f64, rec: &mut OpRecorder) -> Result<Vec<FaceId>, KernelError> {
    let outer0 = body.faces.get(cap0).unwrap().outer.ok_or_else(|| KernelError::Operation("thicken: cap has no outer loop".into()))?;
    let outer1 = body.faces.get(cap1).unwrap().outer.ok_or_else(|| KernelError::Operation("thicken: offset cap has no outer loop".into()))?;
    let ce0 = body.loop_coedges(outer0);
    let ce1 = body.loop_coedges(outer1);
    if ce0.len() != ce1.len() {
        return Err(KernelError::Operation("thicken: cap loop structures diverged".into()));
    }
    let mut sides = Vec::with_capacity(ce0.len());
    for i in 0..ce0.len() {
        let c0 = body.coedges.get(ce0[i]).unwrap().clone();
        let c1 = body.coedges.get(ce1[i]).unwrap().clone();
        let e0 = body.edges.get(c0.edge).unwrap().clone();
        let e1 = body.edges.get(c1.edge).unwrap().clone();
        let curve0 = body.curves3.get(e0.curve).unwrap().clone();
        let curve1 = body.curves3.get(e1.curve).unwrap().clone();
        let ruled = ruled_surface_from_curves(&curve0, e0.range, &curve1, e1.range)?;
        let ruled_id = body.surfaces.insert(ruled.clone());
        let p00 = body.vertices.get(e0.v0).unwrap().position;
        let p01 = body.vertices.get(e0.v1).unwrap().position;
        let p10 = body.vertices.get(e1.v0).unwrap().position;
        let p11 = body.vertices.get(e1.v1).unwrap().position;
        let vert_a = line_edge(body, p00, p10, e0.v0, e1.v0, Tol::DEFAULT, rec);
        let vert_b = line_edge(body, p01, p11, e0.v1, e1.v1, Tol::DEFAULT, rec);
        let members = [(c0.edge, true), (vert_b, true), (c1.edge, false), (vert_a, false)];
        let face = attach_face(body, ruled_id, &members, false, Tol::DEFAULT, rec);
        let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
        edge_geom.insert(c0.edge, (curve0, e0.range));
        edge_geom.insert(c1.edge, (curve1, e1.range));
        edge_geom.insert(vert_a, (Curve3::Line { origin: p00, dir: p10 - p00 }, (0.0, 1.0)));
        edge_geom.insert(vert_b, (Curve3::Line { origin: p01, dir: p11 - p01 }, (0.0, 1.0)));
        set_face_pcurves(body, face, &ruled, &edge_geom, tol);
        sides.push(face);
    }
    Ok(sides)
}

/// ↔️ Thickens a face into a solid of thickness `distance` — every surface kind, planar included,
/// builds an [`offset_face`] cap plus ruled side faces ([`build_ruled_sides`]) between the two
/// caps' boundaries (exact for a plane: the offset cap is a rigid translation and each ruled side
/// is a true planar quad between two parallel line edges, equivalent to a prism but staying on
/// this file's own analytic-offset machinery rather than the sweep module's shared prism builder).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn thicken_face(body: &mut Body, face: FaceId, distance: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if !distance.is_finite() || distance.abs() <= 1e-15 {
        return Err(KernelError::InvalidInput("thicken distance must be non-zero".into()));
    }
    let cap1 = offset_face(body, face, distance, rec)?;
    if let Some(fd) = body.faces.get_mut(cap1) {
        fd.flipped = !fd.flipped;
    }
    let sides = build_ruled_sides(body, face, cap1, OFFSET_TOL, rec)?;
    let mut faces = vec![face, cap1];
    faces.extend(sides);
    Ok(finish_solid(body, faces, rec))
}

// #endregion 🔖️Thicken

// #region 🔖️Shell

/// ↔️ Hollow shell of `solid` with wall thickness `thickness` (fully closed — no open faces): the
/// outer shell reuses the original faces, the inner shell is `offset_solid`'s `-thickness` result
/// with every face's orientation flipped, and the two nest as `outer`/`inners` on one [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Solid`]
/// — exact, no boolean cut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shell_solid(body: &mut Body, solid: SolidId, thickness: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if !thickness.is_finite() || thickness <= 1e-15 {
        return Err(KernelError::InvalidInput("shell thickness must be positive".into()));
    }
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity(format!("solid {solid}")));
    }
    let outer_faces = body.solid_faces(solid);
    let corner = if is_planar_only(body, solid) { OffsetCorner::Sharp } else { OffsetCorner::Round };
    let inner_solid = offset_solid_with_corner(body, solid, -thickness, corner, rec)?;
    let inner_faces = body.solid_faces(inner_solid);
    for &f in &inner_faces {
        if let Some(fd) = body.faces.get_mut(f) {
            fd.flipped = !fd.flipped;
        }
    }
    let outer_shell = add_shell(body, outer_faces, rec);
    let inner_shell = add_shell(body, inner_faces, rec);
    Ok(add_solid(body, outer_shell, vec![inner_shell], rec))
}

/// ↔️ Shells `solid` and leaves `open_faces` open: every non-open face gets its exact `-thickness`
/// offset counterpart (kept faces materialize both the original and the inner face; `open_faces`
/// materialize neither, but their offset surface still trims the neighbouring inner faces exactly
/// like a full shell would); a ruled rim face closes the gap between each open face's original
/// boundary edge and its neighbour's inner offset edge. One connected shell, no boolean cut, no
/// convex hull, `Err` on any construction failure (no silent `continue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shell_solid_with_open_faces(body: &mut Body, solid: SolidId, thickness: f64, open_faces: &[FaceId], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if open_faces.is_empty() {
        return shell_solid(body, solid, thickness, rec);
    }
    if !thickness.is_finite() || thickness <= 1e-15 {
        return Err(KernelError::InvalidInput("shell thickness must be positive".into()));
    }
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity(format!("solid {solid}")));
    }
    let tol = OFFSET_TOL;
    let distance = -thickness;
    let faces_vec = body.solid_faces(solid);
    let solid_faces: HashSet<FaceId> = faces_vec.iter().copied().collect();
    let open_set: HashSet<FaceId> = open_faces.iter().copied().collect();
    for f in &open_set {
        if !solid_faces.contains(f) {
            return Err(KernelError::MissingEntity("open face is not on the solid".into()));
        }
    }
    let mut new_surface_map = HashMap::new();
    for &f in &solid_faces {
        let fd = body.faces.get(f).unwrap().clone();
        let s = body.surfaces.get(fd.surface).unwrap().clone();
        let signed = if fd.flipped { -distance } else { distance };
        new_surface_map.insert(f, offset_surface(&s, signed, tol)?);
    }
    let materialize: HashSet<FaceId> = solid_faces.difference(&open_set).copied().collect();
    let vertex_target = |b: &Body, v: VertexId, touched: &[(FaceId, Vec3)]| -> Pnt3 {
        let normals: Vec<Vec3> = touched.iter().map(|&(_, n)| n).collect();
        b.vertices.get(v).unwrap().position + solve_vertex_displacement(&normals, distance)
    };
    let edge_target = |b: &Body, e: EdgeId, n: Vec3| -> Pnt3 {
        let edge = b.edges.get(e).unwrap();
        let curve = b.curves3.get(edge.curve).unwrap();
        curve.eval(0.5 * (edge.range.0 + edge.range.1)) + n * distance
    };
    let flip_new = |_f: FaceId| true;
    let rebuilt = rebuild_topology(body, solid, &new_surface_map, &materialize, flip_new, vertex_target, edge_target, tol, rec)?;

    let mut shell_faces: Vec<FaceId> = Vec::new();
    for &f in &faces_vec {
        if open_set.contains(&f) {
            continue;
        }
        shell_faces.push(f);
        shell_faces.push(*rebuilt.face_new.get(&f).ok_or_else(|| KernelError::Operation("shell: inner face was not built".into()))?);
    }

    for &open_f in &open_set {
        let face_data = body.faces.get(open_f).unwrap().clone();
        let mut loops = Vec::new();
        if let Some(o) = face_data.outer {
            loops.push(o);
        }
        loops.extend(face_data.inners.iter().copied());
        for lp in loops {
            for cid in body.loop_coedges(lp) {
                let c = body.coedges.get(cid).unwrap().clone();
                let e = c.edge;
                let neighbours = edge_unique_faces(body, &solid_faces, e);
                let Some(&neighbour) = neighbours.iter().find(|&&f| f != open_f) else {
                    continue;
                };
                if open_set.contains(&neighbour) {
                    continue;
                }
                let (new_edge, new_curve, new_range) = rebuilt.edge_new.get(&e).cloned().ok_or_else(|| KernelError::Operation("shell: rim edge was not built".into()))?;
                let orig_edge = body.edges.get(e).unwrap().clone();
                let orig_curve = body.curves3.get(orig_edge.curve).unwrap().clone();
                let rim_surf = ruled_surface_from_curves(&orig_curve, orig_edge.range, &new_curve, new_range)?;
                let rim_id = body.surfaces.insert(rim_surf.clone());
                let nv0 = rebuilt.vertex_new.get(&orig_edge.v0).copied().unwrap_or(orig_edge.v0);
                let nv1 = rebuilt.vertex_new.get(&orig_edge.v1).copied().unwrap_or(orig_edge.v1);
                let p00 = body.vertices.get(orig_edge.v0).unwrap().position;
                let p01 = body.vertices.get(orig_edge.v1).unwrap().position;
                let p10 = body.vertices.get(nv0).unwrap().position;
                let p11 = body.vertices.get(nv1).unwrap().position;
                let vert_a = line_edge(body, p00, p10, orig_edge.v0, nv0, Tol::DEFAULT, rec);
                let vert_b = line_edge(body, p01, p11, orig_edge.v1, nv1, Tol::DEFAULT, rec);
                let members = if c.forward { [(e, true), (vert_b, true), (new_edge, false), (vert_a, false)] } else { [(e, false), (vert_a, true), (new_edge, true), (vert_b, false)] };
                let rim_face = attach_face(body, rim_id, &members, false, Tol::DEFAULT, rec);
                let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
                edge_geom.insert(e, (orig_curve, orig_edge.range));
                edge_geom.insert(new_edge, (new_curve, new_range));
                edge_geom.insert(vert_a, (Curve3::Line { origin: p00, dir: p10 - p00 }, (0.0, 1.0)));
                edge_geom.insert(vert_b, (Curve3::Line { origin: p01, dir: p11 - p01 }, (0.0, 1.0)));
                set_face_pcurves(body, rim_face, &rim_surf, &edge_geom, tol);
                shell_faces.push(rim_face);
            }
        }
    }
    Ok(finish_solid(body, shell_faces, rec))
}

// #endregion 🔖️Shell

// #region 🔖️Draft

/// ↔️ Rotates `surface` about the line where it meets `neutral_plane` by `angle` — exact for
/// `Plane` (rigid rotation, stays a plane) and `Cylinder` (becomes a `Cone`: same-radius circle at
/// the neutral-plane crossing, apex on the axis at `radius / tan(angle)` beyond it, opening toward
/// `pull`); every other kind rotates its control net/frame rigidly about the same line as a
/// documented best-effort (a true taper needs a per-point radial reprojection this pass does not
/// implement).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn draft_one_surface(surface: &Surface, neutral_plane: &Surface, pull: Vec3, angle: f64, tol: f64) -> Result<Surface, KernelError> {
    match surface {
        Surface::Cylinder { frame, radius } => {
            let denom = frame.z.dot(match neutral_plane {
                Surface::Plane { frame: nf } => nf.z,
                _ => return Err(KernelError::Operation("draft neutral plane must be a plane".into())),
            });
            if denom.abs() <= 1e-9 {
                return Err(KernelError::Operation("draft: cylinder axis is parallel to the neutral plane".into()));
            }
            let Surface::Plane { frame: nf } = neutral_plane else { unreachable!() };
            let t = (nf.origin - frame.origin).dot(nf.z) / denom;
            let crossing = frame.origin + frame.z * t;
            let tan_a = angle.abs().tan();
            if tan_a.abs() <= 1e-12 {
                return Err(KernelError::Operation("draft angle too small for a cylinder-to-cone taper".into()));
            }
            let apex_offset = radius / tan_a;
            let sign = if pull.dot(frame.z) >= 0.0 { 1.0 } else { -1.0 };
            let apex = crossing - frame.z * (apex_offset * sign);
            Ok(Surface::Cone { frame: Frame3 { origin: apex, x: frame.x, y: frame.y, z: frame.z }, half_angle: angle.abs() })
        }
        _ => {
            let branches = intersect_surface_surface(surface, neutral_plane, tol)?;
            let (origin, dir) = branches
                .into_iter()
                .find_map(|c| if let Curve3::Line { origin, dir } = c.curve3 { Some((origin, dir)) } else { None })
                .ok_or_else(|| KernelError::Operation("draft: face does not meet the neutral plane in a line".into()))?;
            let axis = dir.normalized().ok_or_else(|| KernelError::Operation("degenerate draft rotation axis".into()))?;
            let map = Affine3::rotation_about(origin, axis, angle);
            Ok(surface.transformed(&map))
        }
    }
}

/// ↔️ Applies `angle` of draft to `faces` of `solid` about the plane through `neutral_origin` with
/// normal `neutral_normal` (per-face rotated about that plane's own intersection with the face —
/// [`draft_one_surface`]), then recomputes every adjacent edge/vertex as the exact intersection of
/// the (possibly drafted-on-both-sides) neighbouring surfaces, propagating through any chain of
/// adjacent drafted faces automatically (both surfaces of a shared edge are looked up from the
/// same substitution map).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn draft_angle(body: &mut Body, solid: SolidId, faces: &[FaceId], pull_dir: Vec3, neutral_origin: Pnt3, neutral_normal: Vec3, angle_rad: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity(format!("solid {solid}")));
    }
    if !angle_rad.is_finite() || angle_rad.abs() <= 1e-15 {
        return Err(KernelError::Operation("draft angle must be non-zero".into()));
    }
    if faces.is_empty() {
        return Err(KernelError::InvalidInput("draft requires at least one face".into()));
    }
    let pull = pull_dir.normalized().ok_or_else(|| KernelError::InvalidInput("pull direction must be non-zero".into()))?;
    let nn = neutral_normal.normalized().ok_or_else(|| KernelError::InvalidInput("neutral plane normal must be non-zero".into()))?;
    let solid_faces_vec = body.solid_faces(solid);
    let solid_faces: HashSet<FaceId> = solid_faces_vec.iter().copied().collect();
    for f in faces {
        if !solid_faces.contains(f) {
            return Err(KernelError::MissingEntity("draft face is not on the solid".into()));
        }
    }
    let neutral_plane = Surface::Plane { frame: Frame3::from_normal(neutral_origin, nn).ok_or_else(|| KernelError::InvalidInput("degenerate neutral plane".into()))? };
    let tol = OFFSET_TOL;
    let mut new_surface_map = HashMap::new();
    for &f in faces {
        let fd = body.faces.get(f).unwrap();
        let s = body.surfaces.get(fd.surface).unwrap().clone();
        new_surface_map.insert(f, draft_one_surface(&s, &neutral_plane, pull, angle_rad, tol)?);
    }
    // A drafted (or untouched, still-adjacent) planar face's ABSOLUTE plane equation is already
    // fully known post-rotation — solving the exact multi-plane intersection (see
    // [`solve_plane_point`]) is what a rotated corner actually needs (unlike offset's uniform
    // `distance`, there is no single displacement scalar for a rotation); a touched face that
    // isn't planar (e.g. a general non-Cylinder surface's rigidly-rotated control net) can't
    // contribute a linear constraint, so it's skipped here and the corner falls back to whichever
    // planar constraints remain (or the original position if none do).
    let vertex_target = |b: &Body, v: VertexId, touched: &[(FaceId, Vec3)]| -> Pnt3 {
        let mut planes: Vec<(Vec3, f64)> = Vec::new();
        for &(f, _) in touched {
            let fd = b.faces.get(f).unwrap();
            let final_surface = new_surface_map.get(&f).cloned().unwrap_or_else(|| b.surfaces.get(fd.surface).unwrap().clone());
            if let Surface::Plane { frame } = final_surface {
                planes.push((frame.z, frame.z.dot(frame.origin.to_vec())));
            }
        }
        solve_plane_point(&planes).unwrap_or_else(|| b.vertices.get(v).unwrap().position)
    };
    let edge_target = |b: &Body, e: EdgeId, _n: Vec3| -> Pnt3 {
        let edge = b.edges.get(e).unwrap();
        let curve = b.curves3.get(edge.curve).unwrap();
        curve.eval(0.5 * (edge.range.0 + edge.range.1))
    };
    let flip_new = |_f: FaceId| false;
    let materialize = solid_faces.clone();
    let rebuilt = rebuild_topology(body, solid, &new_surface_map, &materialize, flip_new, vertex_target, edge_target, tol, rec)?;
    let mut out_faces = Vec::with_capacity(solid_faces_vec.len());
    for &f in &solid_faces_vec {
        out_faces.push(*rebuilt.face_new.get(&f).ok_or_else(|| KernelError::Operation("draft: face was not rebuilt".into()))?);
    }
    Ok(finish_solid(body, out_faces, rec))
}

// #endregion 🔖️Draft

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_planar_face_from_points, make_sphere};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
    use std::f64::consts::PI;

    #[semio_framework_async_macros::async_test]
    async fn debug_plain_cylinder_volume() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).unwrap();
        let v = solid_volume(&body, solid, 1e-4).unwrap();
        let expected = PI * 1.0 * 1.0 * 2.0;
        println!("[DEBUG] plain cylinder v={v} expected={expected}");
    }

    #[semio_framework_async_macros::async_test]
    async fn offset_solid_box_sharp_matches_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (a, b, c, d) = (2.0, 3.0, 1.5, 0.3);
        let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
        let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
        let v = solid_volume(&body, grown, 1e-6).unwrap();
        let closed_form = (a + 2.0 * d) * (b + 2.0 * d) * (c + 2.0 * d);
        assert!((v - closed_form).abs() < 1e-6, "v={v} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn offset_solid_box_round_matches_minkowski_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (a, b, c, r) = (2.0, 3.0, 1.5, 0.3);
        let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
        let grown = offset_solid_with_corner(&mut body, solid, r, OffsetCorner::Round, &mut rec).unwrap();
        let v = solid_volume(&body, grown, 1e-4).unwrap();
        let closed_form = a * b * c + 2.0 * r * (a * b + b * c + c * a) + PI * r * r * (a + b + c) + (4.0 / 3.0) * PI * r * r * r;
        assert!((v - closed_form).abs() < 1e-2 * closed_form, "v={v} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn offset_cylinder_matches_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (r, h, d) = (1.0, 2.0, 0.25);
        let solid = make_cylinder(&mut body, r, h, &mut rec).unwrap();
        let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
        let v = solid_volume(&body, grown, 1e-4).unwrap();
        let closed_form = PI * (r + d) * (r + d) * h;
        assert!((v - closed_form).abs() < 1e-2 * closed_form, "v={v} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn offset_sphere_matches_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (r, d) = (1.0, 0.4);
        let solid = make_sphere(&mut body, r, &mut rec).unwrap();
        let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
        let v = solid_volume(&body, grown, 1e-4).unwrap();
        let closed_form = (4.0 / 3.0) * PI * (r + d).powi(3);
        assert!((v - closed_form).abs() < 1e-2 * closed_form, "v={v} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn offset_nurbs_surface_within_bound() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
        let u_knots = KnotVector::clamped_uniform(3, 2);
        let v_knots = KnotVector::clamped_uniform(3, 2);
        let controls = vec![
            vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.1), Pnt3::new(0.0, 2.0, 0.0)],
            vec![Pnt3::new(1.0, 0.0, 0.1), Pnt3::new(1.0, 1.0, 0.4), Pnt3::new(1.0, 2.0, 0.1)],
            vec![Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.1), Pnt3::new(2.0, 2.0, 0.0)],
        ];
        let weights = vec![vec![1.0; 3]; 3];
        let surface = Surface::Nurbs { u_knots, v_knots, controls, weights };
        let tol = 1e-4;
        let offset = offset_surface(&surface, 0.15, tol).unwrap();
        let (u0, u1) = surface.domain().0;
        let (v0, v1) = surface.domain().1;
        for i in 0..6 {
            for j in 0..6 {
                let u = u0 + (u1 - u0) * (i as f64 + 0.5) / 6.0;
                let v = v0 + (v1 - v0) * (j as f64 + 0.5) / 6.0;
                let n = surface.normal(u, v).unwrap();
                let truth = surface.eval(u, v) + n * 0.15;
                let got = offset.eval(u, v);
                assert!(truth.distance(got) <= tol * 4.0, "deviation {} at ({u},{v})", truth.distance(got));
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn thicken_planar_face_matches_box_volume() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let face = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], &mut rec).unwrap();
        let solid = thicken_face(&mut body, face, 0.5, &mut rec).unwrap();
        let v = solid_volume(&body, solid, 1e-6).unwrap();
        assert!((v - 1.0).abs() < 1e-6, "volume {v}");
    }

    #[semio_framework_async_macros::async_test]
    async fn shell_box_one_open_face_matches_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (a, b, c, t) = (2.0, 2.0, 2.0, 0.2);
        let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
        let top = *body
            .solid_faces(solid)
            .iter()
            .find(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.z - c).abs() < 1e-9))
            .unwrap();
        let shelled = shell_solid_with_open_faces(&mut body, solid, t, &[top], &mut rec).unwrap();
        let v = solid_volume(&body, shelled, 1e-6).unwrap();
        let outer = a * b * c;
        let inner = (a - 2.0 * t) * (b - 2.0 * t) * (c - t);
        let closed_form = outer - inner;
        assert!((v - closed_form).abs() < 1e-3 * closed_form.max(1.0), "v={v} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn shell_box_fully_closed_matches_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (a, b, c, t) = (2.0, 3.0, 1.5, 0.2);
        let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
        let shelled = shell_solid(&mut body, solid, t, &mut rec).unwrap();
        let v = solid_volume(&body, shelled, 1e-6).unwrap();
        let closed_form = a * b * c - (a - 2.0 * t) * (b - 2.0 * t) * (c - 2.0 * t);
        assert!((v - closed_form).abs() < 1e-6, "v={v} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn draft_box_side_face_matches_trapezoid_magnitude() {
        let (a, b, c, angle) = (1.0, 1.0, 1.0, 0.2_f64);
        let mut body_plus = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body_plus, a, b, c, &mut rec).unwrap();
        let right = *body_plus
            .solid_faces(solid)
            .iter()
            .find(|&&f| matches!(body_plus.surfaces.get(body_plus.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.x - a).abs() < 1e-9))
            .unwrap();
        let v0 = solid_volume(&body_plus, solid, 1e-6).unwrap();
        let drafted_plus = draft_angle(&mut body_plus, solid, &[right], Vec3::Z, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, angle, &mut rec).unwrap();
        let v_plus = solid_volume(&body_plus, drafted_plus, 1e-6).unwrap();

        let mut body_minus = Body::new();
        let mut rec2 = OpRecorder::new();
        let solid2 = make_box(&mut body_minus, a, b, c, &mut rec2).unwrap();
        let right2 = *body_minus
            .solid_faces(solid2)
            .iter()
            .find(|&&f| matches!(body_minus.surfaces.get(body_minus.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.x - a).abs() < 1e-9))
            .unwrap();
        let drafted_minus = draft_angle(&mut body_minus, solid2, &[right2], Vec3::Z, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, -angle, &mut rec2).unwrap();
        let v_minus = solid_volume(&body_minus, drafted_minus, 1e-6).unwrap();

        let expected_delta = b * c * c * angle.tan() / 2.0;
        assert!(v_plus != v_minus, "draft should change volume asymmetrically: {v_plus} vs {v_minus}");
        assert!(((v_plus - v0).abs() - expected_delta).abs() < 1e-4, "v_plus={v_plus} v0={v0} expected_delta={expected_delta}");
        assert!(((v_minus - v0).abs() - expected_delta).abs() < 1e-4, "v_minus={v_minus} v0={v0} expected_delta={expected_delta}");
        assert!((v_plus + v_minus - 2.0 * v0).abs() < 1e-6, "draft should be symmetric around the undrafted volume");
    }

    #[semio_framework_async_macros::async_test]
    async fn draft_zero_angle_errors() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let face = body.solid_faces(solid)[0];
        let err = draft_angle(&mut body, solid, &[face], Vec3::Z, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, 0.0, &mut rec).unwrap_err();
        assert!(matches!(err, KernelError::Operation(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn debug_offset_cylinder2() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (r, h, d) = (1.0, 2.0, 0.25);
        let solid = make_cylinder(&mut body, r, h, &mut rec).unwrap();
        let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::face_area;
        for f in body.solid_faces(grown) {
            let fd = body.faces.get(f).unwrap();
            println!("[DEBUG] face {f:?} flipped={} area={:?} surface={:?}", fd.flipped, face_area(&body, f, 1e-4), body.surfaces.get(fd.surface));
            if let Some(o) = fd.outer {
                for cid in body.loop_coedges(o) {
                    let c = body.coedges.get(cid).unwrap();
                    let e = body.edges.get(c.edge).unwrap();
                    println!("[DEBUG]   coedge edge={:?} forward={} range={:?} v0={:?} v1={:?} curve={:?}", c.edge, c.forward, e.range, e.v0, e.v1, body.curves3.get(e.curve));
                }
            }
        }
        let v = solid_volume(&body, grown, 1e-4);
        println!("[DEBUG] volume={v:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn offset_determinism_face_count_and_volume() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let a = offset_solid(&mut body, solid, 0.1, &mut rec).unwrap();
        let b = offset_solid(&mut body, solid, 0.1, &mut rec).unwrap();
        assert_eq!(body.solid_faces(a).len(), body.solid_faces(b).len());
        let va = solid_volume(&body, a, 1e-6).unwrap();
        let vb = solid_volume(&body, b, 1e-6).unwrap();
        assert!((va - vb).abs() < 1e-9, "va={va} vb={vb}");
    }
}

// #endregion 🔖️Tests
