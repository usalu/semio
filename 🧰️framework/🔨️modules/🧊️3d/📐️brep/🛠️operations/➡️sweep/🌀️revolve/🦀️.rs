//! 🌀 Exact revolve: per-edge classification against the rotation axis (line ∥axis → `Cylinder`,
//! line ⟂axis → planar annulus, line crossing the axis → `Cone`; circle whose plane contains the
//! axis → `Torus`/`Sphere`), a full-2π seam construction (mirrors `🧱️primitives::make_cylinder`'s
//! own single-seam pattern, generalized to N profile edges) and a partial-angle two-cap
//! construction (structurally the rotate-instead-of-translate mirror of `🧮️core::build_prism`,
//! with the profile edge itself playing the "rail" role and new rotation-arc edges playing the
//! "cap boundary" role). `Surface::Cylinder`/`Cone`/`Torus`/`Sphere` are angle-linear in `u`, so a
//! straight-line pcurve there is exact; `Surface::Plane` is Cartesian, so the annulus case's
//! rotation-arc pcurve is instead a genuine (angle-linear, exact) `Curve2::Circle` — see
//! `📓️w2c-sweeps.md` §revolve.
//!
//! Mounted as a submodule of `➡️sweep` in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave
//! W2-C via `#[path]` from `➡️sweep/🦀️.rs`.
/// 🌀 Lateral surface, four parameter curves with ranges, and orientation.
pub type LateralSurfaceCurves = (crate::brep::representation::arena::SurfaceId, [(crate::brep::representation::arena::Curve2Id, (f64, f64)); 4], bool);

use std::collections::HashMap;
use std::f64::consts::TAU;

use crate::brep::operations::euler::make_edge;
use crate::brep::operations::staged::{drive_solid, Plan, StageOutput, StageProgress, StageStep, StagedOperation};
use crate::brep::operations::transform::transform_face;
use crate::brep::representation::arena::{CoedgeId, EdgeId, FaceId, SolidId, VertexId};
use crate::brep::representation::curve::{Curve2, Curve3};
use crate::brep::representation::error::KernelError;
use crate::brep::representation::surface::Surface;
use crate::brep::representation::tolerance::Tol;
use crate::brep::representation::topology::history::OpRecorder;
use crate::brep::representation::topology::Body;
use crate::brep::representation::vector::matrix::{Affine3, Frame3};
use crate::brep::representation::vector::{Pnt2, Pnt3, Vec2, Vec3};

use super::core::{build_face, finish_solid, LoopSpec};

const ON_AXIS_TOL: f64 = 1e-7;

/// 🌀 One profile-edge's exact lateral surface for a rotation about `(origin, axis)`.
/// `AngleLinear`: `u` = rotation angle (angle-linear by construction — `Cylinder`/`Cone`/`Torus`/
/// `Sphere`), `v_at(t)` gives the other coordinate. `PlanarAnnulus`: `Surface::Plane` whose (u,v)
/// are Cartesian in-plane coordinates centred at the axis's foot in this plane; `uv_at(t)` gives
/// the edge's own trace, and the rotation-arc rail is a genuine `Curve2::Circle` of `radius_at(t)`
/// centred at the surface's local origin (angle-linear in UV, exact for any sweep angle).
enum RevSurface {
    // 🌀 `flip`: whether this surface's INTRINSIC `du × dv` (always the SAME, geometrically fixed
    // convention per branch below — e.g. a `Cylinder` here is always built with `x = radial.
    // normalized()`, i.e. "outward FROM the axis", since that is what anchors the seam edge's own
    // `u = 0` position exactly — see `lateral_pcurves`) needs negating to be the correct outward-
    // from-SOLID direction for THIS particular profile edge. Computed here (where the edge's own
    // local geometry is available), consumed by the caller's `build_face(..., flip, ...)` — the
    // same fact `🧮️core::build_prism`'s `lateral_flipped` reads off the profile loop's own winding.
    AngleLinear { surface: Surface, v_at: Box<dyn Fn(f64) -> f64>, flip: bool },
    PlanarAnnulus { surface: Surface, uv_at: Box<dyn Fn(f64) -> (f64, f64)>, radius_at: Box<dyn Fn(f64) -> f64>, flip: bool },
}

/// 🌀 Perpendicular distance from `p` to the infinite line `(origin, axis)` (`axis` unit).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dist_to_axis(p: Pnt3, origin: Pnt3, axis: Vec3) -> f64 {
    let v = p - origin;
    (v - axis * v.dot(axis)).norm()
}

/// 🌀 Classifies `curve` against the rotation axis and builds its exact lateral surface. `Ellipse`
/// and any `Nurbs`/off-axis `Circle` are refused (documented gap, see `📓️w2c-sweeps.md`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn classify(curve: &Curve3, origin: Pnt3, axis: Vec3) -> Result<RevSurface, KernelError> {
    match curve {
        Curve3::Line { origin: lo, dir } => {
            let dir_n = dir.normalized().ok_or_else(|| KernelError::InvalidInput("revolve: degenerate profile edge".into()))?;
            let (lo, dir) = (*lo, *dir);
            if dir_n.cross(axis).norm() < 1e-9 {
                let foot = origin + axis * (lo - origin).dot(axis);
                let radial = lo - foot;
                let radius = radial.norm();
                if radius < ON_AXIS_TOL {
                    return Err(KernelError::Operation("revolve: profile edge lies on the axis (degenerate pole edges are not yet supported here)".into()));
                }
                let x_hat = radial.normalized().unwrap();
                let frame = Frame3 { origin: foot, x: x_hat, y: axis.cross(x_hat), z: axis };
                let speed = dir.dot(axis);
                let v0 = (lo - foot).dot(axis);
                // 🌀 `x_hat` is ALWAYS `radial.normalized()` (outward FROM the axis) — required so
                // the seam's own `u = 0` lands exactly on this edge's real position (negating `x`
                // would shift the seam by `π`, breaking the pcurve). But which physical side of
                // this profile edge is the SOLID's material on — i.e. whether "outward from axis"
                // is also "outward from the solid" — depends on the edge's own walking direction
                // relative to a consistently-wound profile: for an axis-parallel edge, walking
                // `+axis` (`speed > 0`) means material is on the `-radial` side of it (matches the
                // OUTER bound of an annulus-like profile, correctly `x_hat`-outward already);
                // walking `-axis` (`speed < 0`) means material is on the `+radial` side (the INNER
                // bound — needs `du × dv` negated). Confirmed via
                // `revolve_annulus_full_turn_is_analytic_and_exact_volume`'s own profile: its outer
                // edge (r=3) walks `+Z` (`speed > 0`, no flip needed) while its inner/bore edge
                // (r=2) walks `-Z` (`speed < 0`, needs `flip = true`) — without this the bore
                // surface's contribution was added instead of subtracted.
                let flip = speed < 0.0;
                return Ok(RevSurface::AngleLinear { surface: Surface::Cylinder { frame, radius }, v_at: Box::new(move |t| v0 + speed * t), flip });
            }
            if dir_n.dot(axis).abs() < 1e-9 {
                let foot = origin + axis * (lo - origin).dot(axis);
                let frame = Frame3::from_normal(foot, axis).ok_or_else(|| KernelError::InvalidInput("revolve: degenerate axis".into()))?;
                let ux0 = (lo - foot).dot(frame.x);
                let uy0 = (lo - foot).dot(frame.y);
                let dx = dir.dot(frame.x);
                let dy = dir.dot(frame.y);
                return Ok(RevSurface::PlanarAnnulus { surface: Surface::Plane { frame }, uv_at: Box::new(move |t| (ux0 + dx * t, uy0 + dy * t)), radius_at: Box::new(move |t| ((ux0 + dx * t).powi(2) + (uy0 + dy * t).powi(2)).sqrt()), flip: false });
            }
            // Closest-point-between-two-lines (standard formula, specialized to unit directions):
            // finds `tc` along the axis nearest `lo + dir_n·s` — exact for genuinely intersecting
            // (coplanar, non-parallel) lines, which this branch assumes (dir_n × axis ≠ 0 already
            // checked by the parallel/perpendicular branches above having returned).
            let b = dir_n.dot(axis);
            let w0 = lo - origin;
            let d = dir_n.dot(w0);
            let e = axis.dot(w0);
            let denom = 1.0 - b * b;
            let tc = (e - b * d) / denom;
            let apex = origin + axis * tc;
            let sample_radial = dist_to_axis(lo, apex, axis);
            let sample_along = (lo - apex).dot(axis);
            let half_angle = sample_radial.atan2(sample_along.abs());
            let z_toward = if sample_along >= 0.0 { axis } else { -axis };
            let foot0 = origin + axis * (lo - origin).dot(axis);
            let radial0 = lo - foot0;
            let x_hat = radial0.normalized().ok_or_else(|| KernelError::InvalidInput("revolve: profile edge starts on the axis (apex-touching cone edges are not yet supported here)".into()))?;
            let frame = Frame3 { origin: apex, x: x_hat, y: z_toward.cross(x_hat), z: z_toward };
            let v0 = (lo - apex).dot(z_toward);
            let speed = dir.dot(z_toward);
            Ok(RevSurface::AngleLinear { surface: Surface::Cone { frame, half_angle }, v_at: Box::new(move |t| v0 + speed * t), flip: false })
        }
        Curve3::Circle { frame, radius } => {
            // 🌀 The circle's plane contains the axis iff the axis direction is PERPENDICULAR to
            // the plane's own normal (`frame.z`) — not parallel to it (a `cross`-based test here
            // would instead accept only the degenerate case where the plane's normal IS the axis,
            // i.e. the circle's plane is perpendicular to the axis, the opposite of what "contains
            // the axis" means). `dot ≈ 0` is the correct perpendicularity test.
            if frame.z.dot(axis).abs() > 1e-6 || (origin - frame.origin).dot(frame.z).abs() > 1e-6 {
                return Err(KernelError::Operation("revolve: circle profile edge's plane does not contain the axis".into()));
            }
            let axis_foot = origin + axis * (frame.origin - origin).dot(axis);
            let major = dist_to_axis(frame.origin, origin, axis);
            if major < ON_AXIS_TOL {
                let x_hat = frame.x.normalized().unwrap_or(Vec3::X);
                return Ok(RevSurface::AngleLinear { surface: Surface::Sphere { frame: Frame3 { origin: axis_foot, x: x_hat, y: axis.cross(x_hat), z: axis }, radius: *radius }, v_at: Box::new(|t| t), flip: false });
            }
            let x_hat = (frame.origin - axis_foot).normalized().unwrap();
            let rframe = Frame3 { origin: axis_foot, x: x_hat, y: axis.cross(x_hat), z: axis };
            Ok(RevSurface::AngleLinear { surface: Surface::Torus { frame: rframe, major_radius: major, minor_radius: *radius }, v_at: Box::new(|t| t), flip: false })
        }
        Curve3::Ellipse { .. } => Err(KernelError::Operation("revolve: elliptical profile edges are not yet supported".into())),
        Curve3::Nurbs { .. } => Err(KernelError::Operation("revolve: free-form (NURBS) profile edges are not yet supported (exact rational surface-of-revolution construction not implemented in this pass — see 📓️w2c-sweeps.md)".into())),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn axis_orbit_edge(body: &mut Body, axis_origin: Pnt3, axis: Vec3, position: Pnt3, angle: f64, (va, vb): (VertexId, VertexId), rec: &mut OpRecorder) -> EdgeId {
    let foot = axis_origin + axis * (position - axis_origin).dot(axis);
    let radial = position - foot;
    let r = radial.norm();
    if r < ON_AXIS_TOL {
        let c = body.curves3.insert(Curve3::Line { origin: position, dir: Vec3::ZERO });
        make_edge(body, c, (0.0, angle), va, vb, Tol::DEFAULT, rec)
    } else {
        let frame = Frame3 { origin: foot, x: radial.normalized().unwrap(), y: axis.cross(radial.normalized().unwrap()), z: axis };
        let c = body.curves3.insert(Curve3::Circle { frame, radius: r });
        make_edge(body, c, (0.0, angle), va, vb, Tol::DEFAULT, rec)
    }
}

/// 🌀 Builds one lateral face's surface + its 4 pcurves for `[start_edge(!f_i), left_rail(true),
/// end_edge(f_i), right_rail(false)]`, dispatching on [`RevSurface`]'s two conventions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn lateral_pcurves(body: &mut Body, rev: RevSurface, range: (f64, f64), angle: f64) -> LateralSurfaceCurves {
    match rev {
        RevSurface::AngleLinear { surface, v_at, flip } => {
            let surf_id = body.surfaces.insert(surface);
            let (v0, v1) = (v_at(range.0), v_at(range.1));
            // 🌀 `Curve2::Line::eval(t) = origin + dir·t` reads `t` RAW (never normalized to
            // `[0,1]` — `prange` below is `range`, the edge's own raw domain, matching every other
            // pcurve in this file's "p = t" convention). `dir` must therefore be `v_at`'s actual
            // per-unit-`t` slope, not the endpoint-to-endpoint delta `v1 - v0` (that delta is only
            // the correct slope when `range` itself happens to be `(0, 1)` — for any other domain
            // it silently over/under-scales `v`, e.g. `t ∈ [0, 2π]` turned `v` into `t²` instead of
            // `v_at(t)`, the bug this docstring replaced). `v_at` is affine by construction (every
            // `RevSurface` builder here composes a `Line`/`Circle` curve param linearly), so
            // sampling it at the two reference points `t=0`/`t=1` recovers its exact slope.
            let slope = v_at(1.0) - v_at(0.0);
            let origin_v = v_at(0.0);
            let start_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(0.0, origin_v), dir: Vec2::new(0.0, slope) });
            let end_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(angle, origin_v), dir: Vec2::new(0.0, slope) });
            let left_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(0.0, v0), dir: Vec2::new(1.0, 0.0) });
            let right_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(0.0, v1), dir: Vec2::new(1.0, 0.0) });
            (surf_id, [(start_pc, range), (left_pc, (0.0, angle)), (end_pc, range), (right_pc, (0.0, angle))], flip)
        }
        RevSurface::PlanarAnnulus { surface, uv_at, radius_at, flip } => {
            let surf_id = body.surfaces.insert(surface);
            let (u0, v0) = uv_at(range.0);
            let (u1, v1) = uv_at(range.1);
            // 🌀 Same raw-`t` pcurve convention as the `AngleLinear` branch above: `uv_at` is
            // affine in `t`, so its slope (and `end_map∘uv_at`'s — rotation by a fixed `angle` is
            // linear, so the composition stays affine in `t`) is recovered from samples at `t=0`/
            // `t=1`, not from the `range`-endpoint delta.
            let (su0, sv0) = uv_at(0.0);
            let (su1, sv1) = uv_at(1.0);
            let start_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(su0, sv0), dir: Vec2::new(su1 - su0, sv1 - sv0) });
            let end_map = |u: f64, v: f64, a: f64| {
                let r = (u * u + v * v).sqrt();
                let base = v.atan2(u);
                (r * (base + a).cos(), r * (base + a).sin())
            };
            let (seu0, sev0) = end_map(su0, sv0, angle);
            let (seu1, sev1) = end_map(su1, sv1, angle);
            let end_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(seu0, sev0), dir: Vec2::new(seu1 - seu0, sev1 - sev0) });
            let left_pc = body.curves2.insert(Curve2::Circle { center: Pnt2::new(0.0, 0.0), radius: radius_at(range.0) });
            let right_pc = body.curves2.insert(Curve2::Circle { center: Pnt2::new(0.0, 0.0), radius: radius_at(range.1) });
            let base0 = v0.atan2(u0);
            let base1 = v1.atan2(u1);
            // 🌀 `left_pc`/`right_pc` (the inner/outer rotation-arc rails, a genuine
            // `Curve2::Circle` each — see this file's own module doc) must trace OPPOSITE angular
            // senses from each other, matching the standard "outer boundary CCW, inner (hole)
            // boundary CW" convention ear-clipping needs to correctly net `outer_area −
            // inner_area` for a single simple polygon connected by a bridge (the zero-width
            // `edge_id` "radial" seam used twice, `📓️w2c-sweeps.md`'s own single-seam pattern
            // applied to a Plane surface's Cartesian, not angular, `(u, v)`). The coedge's own
            // `forward` flag is topology-load-bearing (ring closure, shared via `orbit_cache` with
            // adjacent lateral faces — never safe to touch here), so the sense flip is done purely
            // in this pcurve's OWN `prange` ordering (`(base+angle, base)` instead of `(base, base+
            // angle)` — reversed, sampled un-reversed by a `forward = true` coedge, or reversed
            // again by `forward = false`, either way tracing backward from the "natural" CCW
            // `Curve2::Circle` parametrization). Confirmed via
            // `revolve_annulus_full_turn_is_analytic_and_exact_volume`: before this reversal BOTH
            // rails came out the SAME sense (inner CCW, outer CW — backward, not opposite), and
            // each annulus cap's own `face_area` came out ~10.1/10.4 instead of `π·(3²−2²) ≈
            // 15.71`; reversing both here (so they trace OPPOSITE senses) fixed the area to ~15.6
            // (matching within adaptive-quadrature tolerance) and the whole solid's volume from
            // `~69.6` to the exact `78.54`.
            (surf_id, [(start_pc, range), (left_pc, (base0 + angle, base0)), (end_pc, range), (right_pc, (base1 + angle, base1))], flip)
        }
    }
}

/// 🌀 Resumable revolve. A partial angle (`|angle| < 2π - ε`) builds two planar caps — the profile
/// at the start and end angle, built exactly like `🧮️core::PrismBuilder`'s bottom/top — plus one
/// lateral face per profile edge, bounded by two new rotation-arc "rails" (one per profile vertex,
/// shared between adjacent lateral faces) instead of straight lines: one setup unit, one unit per
/// lateral, one closing unit. A full 2π revolve has no caps — one rotation-orbit circle (or
/// degenerate on-axis edge) per profile vertex, shared by its two adjacent lateral faces, and each
/// profile edge reused TWICE as its own lateral face's seam (`u=0` forward, `u=2π` reverse), the
/// direct N-edge generalization of `🧱️primitives::make_cylinder`'s own single-seam pattern — so it
/// is one unit per lateral and one closing unit.
pub struct RevolveJob {
    profile: FaceId,
    axis_origin: Pnt3,
    axis: Vec3,
    angle: f64,
    full: bool,
    end_face: Option<FaceId>,
    items: Vec<(CoedgeId, Option<CoedgeId>)>,
    rail_cache: HashMap<VertexId, EdgeId>,
    laterals: Vec<FaceId>,
    plan: Plan,
}

impl RevolveJob {
    /// 🌀 Plans a revolve of `face` about `(axis_origin, axis_direction)` by `angle` radians
    /// (`|angle| ≥ 2π` clamps to a full closed revolve).
    pub fn new(body: &Body, face: FaceId, axis_origin: Pnt3, axis_direction: Vec3, angle: f64) -> Result<Self, KernelError> {
        let axis = axis_direction.normalized().ok_or_else(|| KernelError::InvalidInput("revolve axis is zero-length".into()))?;
        if !angle.is_finite() || angle.abs() <= 1e-12 {
            return Err(KernelError::InvalidInput("revolve angle must be non-zero".into()));
        }
        let full = angle.abs() >= TAU - 1e-9;
        let profile_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face:?}")))?;
        if !full && !matches!(body.surfaces.get(profile_data.surface), Some(Surface::Plane { .. })) {
            return Err(KernelError::InvalidInput("revolve profile face must be planar".into()));
        }
        let items: Vec<(CoedgeId, Option<CoedgeId>)> = body.face_loops(face).into_iter().flat_map(|lp| body.loop_coedges(lp)).map(|coedge| (coedge, None)).collect();
        let plan = Plan::new(&[("setup", usize::from(!full)), ("laterals", items.len()), ("close", 1)]);
        Ok(Self { profile: face, axis_origin, axis, angle, full, end_face: None, items, rail_cache: HashMap::new(), laterals: Vec::new(), plan })
    }

    /// 🌀 Flips the start cap, copies it to the end angle and pairs every start coedge with its twin.
    fn setup_partial(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<(), KernelError> {
        let map = Affine3::rotation_about(self.axis_origin, self.axis, self.angle);
        let n0 = super::core::planar_outward_normal(body, self.profile)?;
        let start_frame = {
            let f = body.faces.get(self.profile).unwrap();
            match body.surfaces.get(f.surface).unwrap() {
                Surface::Plane { frame } => *frame,
                _ => return Err(KernelError::InvalidInput("revolve profile face must be planar".into())),
            }
        };
        let travel = map.apply_point(start_frame.origin) - start_frame.origin;
        let flip_start = n0.dot(travel) > 0.0;
        let start_label = body.faces.get(self.profile).unwrap().label;
        if flip_start {
            let f = body.faces.get_mut(self.profile).unwrap();
            f.flipped = !f.flipped;
            rec.record_modified(start_label);
        }
        let end_face = transform_face(body, self.profile, &map, rec)?;
        let start_loops = body.face_loops(self.profile);
        let end_loops = body.face_loops(end_face);
        if start_loops.len() != end_loops.len() {
            return Err(KernelError::Operation("revolve: internal loop-count mismatch after transform_face".into()));
        }
        self.items.clear();
        for (&sl, &el) in start_loops.iter().zip(&end_loops) {
            let sce = body.loop_coedges(sl);
            let ece = body.loop_coedges(el);
            self.items.extend((0..sce.len()).map(|k| (sce[k], Some(ece[k]))));
        }
        self.end_face = Some(end_face);
        Ok(())
    }

    /// 🌀 The lateral face of one partial-angle profile coedge.
    fn lateral_partial(&mut self, body: &mut Body, rec: &mut OpRecorder, (start, end): (CoedgeId, CoedgeId)) -> Result<(), KernelError> {
        let (edge_id, f_i) = {
            let c = body.coedges.get(start).unwrap();
            (c.edge, c.forward)
        };
        let (s_v0, s_v1) = body.coedge_endpoints(start).unwrap();
        let (e_v0, e_v1) = body.coedge_endpoints(end).unwrap();
        let curve = body.curves3.get(body.edges.get(edge_id).unwrap().curve).unwrap().clone();
        let range = body.edges.get(edge_id).unwrap().range;
        let rev = classify(&curve, self.axis_origin, self.axis)?;
        let (surf_id, [pc0, pc_left, pc1, pc_right], flip) = lateral_pcurves(body, rev, range, self.angle);
        let (axis_origin, axis, angle) = (self.axis_origin, self.axis, self.angle);
        let left_rail = *self.rail_cache.entry(s_v0).or_insert_with(|| {
            let p = body.vertices.get(s_v0).unwrap().position;
            axis_orbit_edge(body, axis_origin, axis, p, angle, (s_v0, e_v0), rec)
        });
        let right_rail = *self.rail_cache.entry(s_v1).or_insert_with(|| {
            let p = body.vertices.get(s_v1).unwrap().position;
            axis_orbit_edge(body, axis_origin, axis, p, angle, (s_v1, e_v1), rec)
        });
        let members = vec![(edge_id, !f_i), (left_rail, true), (edge_id, f_i), (right_rail, false)];
        let pcurves = vec![pc0, pc_left, pc1, pc_right];
        self.laterals.push(build_face(body, surf_id, &[LoopSpec { members, pcurves }], flip, Tol::DEFAULT, rec));
        Ok(())
    }

    /// 🌀 The lateral face of one full-turn profile coedge.
    fn lateral_full(&mut self, body: &mut Body, rec: &mut OpRecorder, coedge: CoedgeId) -> Result<(), KernelError> {
        let edge_id = body.coedges.get(coedge).unwrap().edge;
        let (v0, v1) = (body.edges.get(edge_id).unwrap().v0, body.edges.get(edge_id).unwrap().v1);
        let curve = body.curves3.get(body.edges.get(edge_id).unwrap().curve).unwrap().clone();
        let range = body.edges.get(edge_id).unwrap().range;
        let rev = classify(&curve, self.axis_origin, self.axis)?;
        let (surf_id, [pc0, pc_left, pc1, pc_right], flip) = lateral_pcurves(body, rev, range, TAU);
        let (axis_origin, axis) = (self.axis_origin, self.axis);
        let c_start = *self.rail_cache.entry(v0).or_insert_with(|| {
            let p = body.vertices.get(v0).unwrap().position;
            axis_orbit_edge(body, axis_origin, axis, p, TAU, (v0, v0), rec)
        });
        let c_end = *self.rail_cache.entry(v1).or_insert_with(|| {
            let p = body.vertices.get(v1).unwrap().position;
            axis_orbit_edge(body, axis_origin, axis, p, TAU, (v1, v1), rec)
        });
        let members = vec![(c_start, true), (edge_id, true), (c_end, false), (edge_id, false)];
        let pcurves = vec![pc_left, (pc0.0, range), pc_right, (pc1.0, range)];
        self.laterals.push(build_face(body, surf_id, &[LoopSpec { members, pcurves }], flip, Tol::DEFAULT, rec));
        Ok(())
    }
}

impl StagedOperation for RevolveJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("revolve job already finished".into()))?;
        match unit.phase {
            0 => self.setup_partial(body, rec)?,
            1 => {
                let (start, end) = self.items[unit.index];
                match end {
                    Some(end) => self.lateral_partial(body, rec, (start, end))?,
                    None => self.lateral_full(body, rec, start)?,
                }
            }
            _ => {
                if self.full {
                    let profile_label = body.faces.get(self.profile).unwrap().label;
                    rec.record_deleted(profile_label);
                    return Ok(StageStep::Done(StageOutput::Solid(finish_solid(body, std::mem::take(&mut self.laterals), rec))));
                }
                let mut faces = vec![self.profile, self.end_face.expect("end cap built in setup")];
                faces.extend(std::mem::take(&mut self.laterals));
                return Ok(StageStep::Done(StageOutput::Solid(finish_solid(body, faces, rec))));
            }
        }
        Ok(StageStep::Working)
    }
}

/// 🌀 Revolves `face` about `(axis_origin, axis_direction)` by `angle` (radians; `|angle| ≥ 2π`
/// clamps to a full closed revolve) — [`RevolveJob`] driven to completion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn revolve_face(body: &mut Body, face: FaceId, axis_origin: Pnt3, axis_direction: Vec3, angle: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    drive_solid(&mut RevolveJob::new(body, face, axis_origin, axis_direction, angle)?, body, rec)
}
