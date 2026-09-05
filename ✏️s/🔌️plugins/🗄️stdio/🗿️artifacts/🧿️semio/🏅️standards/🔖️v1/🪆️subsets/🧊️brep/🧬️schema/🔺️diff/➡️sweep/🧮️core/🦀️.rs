//! 🧮 Shared exact-sweep machinery: the multi-loop face builder, the validated "reuse the bottom
//! face, transform-copy the top face, build one analytic-or-NURBS lateral face per profile edge"
//! prism pattern (proved by hand against `🧱️primitives`' own `make_box`/`make_cylinder` coedge
//! conventions — see `📓️w2c-sweeps.md` §orientation), and the rail-edge cache it needs. Reused by
//! `extrude`/`revolve`(partial)/the general sweep-path chain so the coedge-orientation derivation
//! only has to be gotten right once.
//!
//! Moved into a submodule of `➡️sweep` in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave
//! W2-C, mounted via `#[path]` from `➡️sweep/🦀️.rs`.

use std::collections::HashMap;

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::line_edge;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::transform::transform_face;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve2Id, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::{Affine3, Frame3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};

// #region 🔖️Face

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

/// 🧮 One loop's member edges plus the p-curve `(Curve2Id, prange)` for each, in the same order.
pub(super) struct LoopSpec {
    pub members: Vec<(EdgeId, bool)>,
    pub pcurves: Vec<(Curve2Id, (f64, f64))>,
}

/// 🧮 Builds a face with an outer loop plus zero or more inner (hole) loops, stamping every
/// coedge's p-curve from the matching `LoopSpec` in the same walk order `make_loop` produces.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_face(body: &mut Body, surface: SurfaceId, loops: &[LoopSpec], flipped: bool, tol: Tol, rec: &mut OpRecorder) -> FaceId {
    let loop_ids: Vec<_> = loops.iter().map(|l| make_loop(body, placeholder_face(), &l.members)).collect();
    let outer = loop_ids.first().copied();
    let inners = loop_ids.get(1..).map(|s| s.to_vec()).unwrap_or_default();
    let face = add_face(body, surface, outer, inners, flipped, tol, rec);
    for &lid in &loop_ids {
        body.loops.get_mut(lid).unwrap().face = face;
    }
    for (spec, &lid) in loops.iter().zip(&loop_ids) {
        let coedges = body.loop_coedges(lid);
        for (&cid, &(pc, pr)) in coedges.iter().zip(&spec.pcurves) {
            let coedge = body.coedges.get_mut(cid).unwrap();
            coedge.pcurve = Some(pc);
            coedge.prange = pr;
        }
    }
    face
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn finish_solid(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> SolidId {
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

// #endregion 🔖️Face

// #region 🔖️Placement

/// 🧮 The kind of rigid placement a prism segment's "bottom → top" map represents — determines
/// which analytic lateral-surface recognizers may safely apply (see `📓️w2c-sweeps.md` §pcurve).
pub(super) enum Placement {
    /// 🧮 A pure translation: a line profile edge always sweeps a [`Surface::Plane`]; a circle
    /// edge whose plane normal is parallel to the direction sweeps a [`Surface::Cylinder`].
    Translate { offset: Vec3 },
    /// 🧮 An arbitrary rigid placement (rotation-minimizing-frame sweep station, or a partial
    /// revolve's own bottom→top rotation supplied pre-composed by the caller): only line and
    /// already-rational-free-form (`Curve3::Nurbs`) profile edges have a certified pcurve here —
    /// see `📓️w2c-sweeps.md` §pcurve for why circle/ellipse profile edges are refused instead of
    /// silently mis-parametrized.
    General { map: Affine3 },
}

impl Placement {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn affine(&self) -> Affine3 {
        match self {
            Placement::Translate { offset } => Affine3::translation(*offset),
            Placement::General { map } => *map,
        }
    }
}

/// 🧮 The current outward-facing unit normal of a planar face, honouring `flipped`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn planar_outward_normal(body: &Body, face: FaceId) -> Result<Vec3, KernelError> {
    let f = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face:?}")))?;
    let Surface::Plane { frame } = body.surfaces.get(f.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))? else {
        return Err(KernelError::InvalidInput("sweep profile face must be planar".into()));
    };
    Ok(if f.flipped { -frame.z } else { frame.z })
}

/// 🧮 The rigid map taking the identity frame's own axes to `frame` (columns = `frame.x/y/z`,
/// translation = `frame.origin`) — the standard "local→world" change-of-basis for a planar face's
/// own frame, used to compose consecutive sweep-station placements.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn frame_to_affine(frame: &Frame3) -> Affine3 {
    Affine3 { linear: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Mat3::from_columns(frame.x, frame.y, frame.z), translation: frame.origin.to_vec() }
}

// #endregion 🔖️Placement

// #region 🔖️LateralSurface

/// 🧮 One profile-edge's exact side surface plus its bottom/top-boundary u-domain (`= edge.range`
/// exactly, so `p = t` — see `📓️w2c-sweeps.md` §pcurve) and its "v" coordinate at the bottom/top
/// rail (so the caller can build straight/circular rail p-curves consistently).
pub(super) struct LateralSurface {
    pub surface: Surface,
    pub u_domain: (f64, f64),
    pub v_bottom: f64,
    pub v_top: f64,
}

/// 🧮 Classifies one profile edge under a [`Placement::Translate`] and builds its exact lateral
/// surface. `Curve3::Ellipse` and any `Curve3::Circle` not axis-parallel are refused (not silently
/// approximated) — see `📓️w2c-sweeps.md` §pcurve for why.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn translate_lateral(curve: &Curve3, range: (f64, f64), offset: Vec3) -> Result<LateralSurface, KernelError> {
    match curve {
        Curve3::Line { origin, dir } => {
            let p0 = *origin + *dir * range.0;
            let dir_hat = dir.normalized().ok_or_else(|| KernelError::InvalidInput("extrude: degenerate (zero-length) profile edge".into()))?;
            let normal = dir_hat.cross(offset).normalized();
            // 🧮 `x` MUST be the profile edge's own direction, not an arbitrary vector merely ⟂
            // `normal` (`Frame3::from_normal`'s `any_orthogonal()` pick, the prior bug here): the
            // caller's `bottom_pc`/`top_pc`/rail p-curves assume the whole profile edge traces a
            // straight line in this plane's own (u, v) — true only when `frame.x ∥ dir` (then
            // `frame.y ⟂ dir` follows from orthonormality, putting the entire edge at `v = 0`).
            let frame = normal.and_then(|n| Frame3::from_x_z(p0, dir_hat, n)).ok_or_else(|| KernelError::InvalidInput("extrude: profile edge is parallel to the extrude direction".into()))?;
            // 🧮 `u_domain` is the ACTUAL local-x span from `range.0` to `range.1` (`= |dir|·Δt`,
            // not `range` itself unless `dir` happens to be unit — `core::build_prism` now derives
            // its `bottom_pc`/`top_pc` slope from this pair instead of assuming `u = t`), and
            // `v_top` is `offset`'s real displacement along `frame.y` (not a hardcoded `0.0`, which
            // collapsed the top rail onto the bottom one).
            let u1 = dir.norm() * (range.1 - range.0);
            Ok(LateralSurface { surface: Surface::Plane { frame }, u_domain: (0.0, u1), v_bottom: 0.0, v_top: offset.dot(frame.y) })
        }
        Curve3::Circle { frame, radius } => {
            let dir = offset.normalized().ok_or_else(|| KernelError::InvalidInput("extrude direction is zero".into()))?;
            if dir.cross(frame.z).norm() > 1e-6 {
                return Err(KernelError::Operation("extrude: circle profile edge axis is not parallel to the extrude direction (unsupported non-axis-aligned circular extrusion pcurve)".into()));
            }
            let height = offset.dot(frame.z);
            let cyl_frame = if height >= 0.0 { *frame } else { Frame3 { origin: frame.origin, x: frame.x, y: frame.y, z: -frame.z } };
            Ok(LateralSurface { surface: Surface::Cylinder { frame: cyl_frame, radius: *radius }, u_domain: range, v_bottom: 0.0, v_top: height.abs() })
        }
        Curve3::Ellipse { .. } => Err(KernelError::Operation("extrude: elliptical profile edges are not yet supported (certified angle-to-NURBS-parameter pcurve inversion not implemented)".into())),
        Curve3::Nurbs { .. } => {
            let nc = curve.to_nurbs(range);
            let top: Vec<Pnt3> = nc.controls.iter().map(|&p| p + offset).collect();
            let surface = Surface::Nurbs { u_knots: nc.knots.clone(), v_knots: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 1, 2).unwrap(), controls: nc.controls.iter().cloned().zip(top).map(|(a, b)| vec![a, b]).collect(), weights: nc.weights.iter().map(|&w| vec![w, w]).collect() };
            Ok(LateralSurface { surface, u_domain: range, v_bottom: 0.0, v_top: 1.0 })
        }
    }
}

// #endregion 🔖️LateralSurface

// #region 🔖️Prism

/// 🧮 One built prism segment: the new top face plus every lateral face, ready to be appended to
/// a solid's face list (or the top face reused as the next segment's bottom).
pub(super) struct Prism {
    pub top: FaceId,
    pub laterals: Vec<FaceId>,
}

/// 🧮 Builds one prism segment: `bottom` is flipped in place to face outward (recorded modified),
/// a fresh `top = transform_face(bottom, map)`, and one lateral face per profile edge (every loop,
/// so holes get their own tube faces). The bottom/top boundary orientation follows the derivation
/// in `📓️w2c-sweeps.md` §orientation: bottom coedge `!f_i`, top coedge `f_i` (its own, unchanged,
/// copied value), rails shared bottom→top with `true`/`false` per adjacent lateral face.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_prism(body: &mut Body, bottom: FaceId, placement: &Placement, rec: &mut OpRecorder) -> Result<Prism, KernelError> {
    let map = placement.affine();
    let n0 = planar_outward_normal(body, bottom)?;
    let travel = match placement {
        Placement::Translate { offset } => *offset,
        Placement::General { .. } => {
            let bfd = body.faces.get(bottom).and_then(|f| match body.surfaces.get(f.surface) { Some(Surface::Plane { frame }) => Some(*frame), _ => None }).ok_or_else(|| KernelError::InvalidInput("sweep profile face must be planar".into()))?;
            map.apply_point(bfd.origin) - bfd.origin
        }
    };
    let want_flip_bottom = n0.dot(travel) > 0.0;
    let bottom_label = body.faces.get(bottom).unwrap().label;
    if want_flip_bottom {
        let f = body.faces.get_mut(bottom).unwrap();
        f.flipped = !f.flipped;
        rec.record_modified(bottom_label);
    }
    let top = transform_face(body, bottom, &map, rec)?;
    // 🧮 `transform_face` (`copy_face`) correctly preserves `flipped` verbatim under any
    // NON-reflecting map (`determinant > 0`, true for every rigid `Translate`/`General` placement
    // here) — that is the right general contract for a plain copy. But a prism's `top` cap is not
    // just a copy: it caps the SOLID on the opposite side from `bottom`, so its OUTWARD direction
    // must be the opposite of `bottom`'s (already `want_flip_bottom`-corrected) outward direction,
    // even though `top`'s surface shares `bottom`'s exact local frame/normal-generating convention
    // (translated or rigidly rotated, never mirrored). Toggling here is what `want_flip_bottom`
    // above already does for `bottom` itself, applied to the other cap. Previously this was never
    // done, so `top` silently inherited `bottom`'s sign — invisible whenever every face happened
    // to share the same (accidentally globally inverted) convention, since `solid_volume` takes
    // `.abs()` — but a real, provable bug once a cap's own quadrature is orientation-SENSITIVE
    // per-face (any curved-boundary `Plane` cap routed through the general `loop_uv_polygon` path
    // instead of the straight-edge `signed_tetra_sum` fast path): confirmed via
    // `sweep_circle_along_line_is_a_cylinder`, whose circular caps landed at `+5.17` instead of
    // `π·r²·h ≈ 15.71` — exactly `(lateral's correct +10.47) + (top cap's WRONG-signed −5.24)`
    // instead of `+15.71 = (lateral +10.47) + (top +5.24, bottom always ≈0 for either sign since
    // its own centroid lies in its own plane)`.
    {
        let t = body.faces.get_mut(top).unwrap();
        t.flipped = !t.flipped;
    }
    let bottom_loops = body.face_loops(bottom);
    let top_loops = body.face_loops(top);
    if bottom_loops.len() != top_loops.len() {
        return Err(KernelError::Operation("sweep: internal loop-count mismatch after transform_face".into()));
    }
    let mut rail_cache: HashMap<VertexId, EdgeId> = HashMap::new();
    let mut laterals = Vec::new();
    for (&bl, &tl) in bottom_loops.iter().zip(&top_loops) {
        let bce = body.loop_coedges(bl);
        let tce = body.loop_coedges(tl);
        let n = bce.len();
        for k in 0..n {
            let (b_edge, f_i) = { let c = body.coedges.get(bce[k]).unwrap(); (c.edge, c.forward) };
            let t_edge = body.coedges.get(tce[k]).unwrap().edge;
            let (s_bot, e_bot) = body.coedge_endpoints(bce[k]).unwrap();
            let (s_top, e_top) = body.coedge_endpoints(tce[k]).unwrap();
            let left_rail = *rail_cache.entry(s_bot).or_insert_with(|| {
                let (a, b) = (body.vertices.get(s_bot).unwrap().position, body.vertices.get(s_top).unwrap().position);
                line_edge(body, a, b, s_bot, s_top, Tol::DEFAULT, rec)
            });
            let right_rail = *rail_cache.entry(e_bot).or_insert_with(|| {
                let (a, b) = (body.vertices.get(e_bot).unwrap().position, body.vertices.get(e_top).unwrap().position);
                line_edge(body, a, b, e_bot, e_top, Tol::DEFAULT, rec)
            });
            let curve = body.curves3.get(body.edges.get(b_edge).unwrap().curve).unwrap().clone();
            let range = body.edges.get(b_edge).unwrap().range;
            let lat = match placement {
                Placement::Translate { offset } => translate_lateral(&curve, range, *offset)?,
                Placement::General { map } => general_lateral(&curve, range, map)?,
            };
            let lateral_flipped = matches!(lat.surface, Surface::Plane { .. });
            let surf_id = body.surfaces.insert(lat.surface);
            let u0 = lat.u_domain.0;
            let u1 = lat.u_domain.1;
            // 🧮 `bottom_pc`/`top_pc` must read `t` RAW (`prange` below is `range`, the edge's own
            // curve domain — the file-wide "p = t" convention, see `📓️w1e-primitives.md`), so
            // their slope is `u`'s actual per-unit-`t` rate, `(u1 - u0) / (range.1 - range.0)` —
            // hardcoding `dir = (1, 0)` (as if `u == t` exactly) silently assumed `lat.u_domain ==
            // range`, true only for the `Circle`/`Nurbs` cases (where it still is, recovering
            // identical behavior below: `u1 - u0 == range.1 - range.0` there ⇒ `slope == 1`); for
            // `Line` it is not, since `u_domain` is now the edge's true local-x span, not its raw
            // `t` domain (see `translate_lateral`'s `Curve3::Line` branch docstring).
            let u_slope = (u1 - u0) / (range.1 - range.0);
            let u_origin = u0 - u_slope * range.0;
            let bottom_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u_origin, lat.v_bottom), dir: Vec2::new(u_slope, 0.0) });
            let top_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u_origin, lat.v_top), dir: Vec2::new(u_slope, 0.0) });
            let (u_s, u_e) = if f_i { (u0, u1) } else { (u1, u0) };
            // 🧮 `left_pc`/`right_pc` trace a RAIL edge (`line_edge`'s own `(0, 1)` domain, `t = 0`
            // at the bottom vertex, `t = 1` at the top — see `🧱️primitives::line_edge`), so their
            // slope must be the surface's actual `v_top - v_bottom` span, not a hardcoded `1.0`
            // (only correct when that span happened to equal `1`, true for `general_lateral`'s
            // NURBS surfaces by explicit `v_knots` construction, but not for `Cylinder`'s `v =
            // height` or a `Line` profile edge's now-real `v_top = offset · frame.y`).
            let v_slope = lat.v_top - lat.v_bottom;
            let left_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u_s, lat.v_bottom), dir: Vec2::new(0.0, v_slope) });
            let right_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u_e, lat.v_bottom), dir: Vec2::new(0.0, v_slope) });
            let members = vec![(b_edge, !f_i), (left_rail, true), (t_edge, f_i), (right_rail, false)];
            let pcurves = vec![(bottom_pc, range), (left_pc, (0.0, 1.0)), (top_pc, range), (right_pc, (0.0, 1.0))];
            // 🧮 A `Curve3::Line` lateral's `Surface::Plane` needs `flipped = true`: unlike every
            // OTHER lateral surface kind (`Cylinder`/`Cone`/`Torus`/NURBS, all independently
            // verified correct with `flipped = false` — their `du × dv` naturally points outward
            // for the `[bottom(!f_i), left_rail(true), top(f_i), right_rail(false)]` coedge order
            // this file derives), the STRAIGHT-edge case routes through mass-properties'
            // `signed_tetra_sum` fast path instead of `du × dv`-based quadrature — a sign
            // convention that depends on the LOOP's own vertex winding, not on `frame.z`/`du × dv`
            // at all, and empirically comes out backward for this same coedge order. Confirmed via
            // `extrude_rectangle_matches_box_topology_and_volume`: with `flipped = false` every one
            // of the 4 side faces' independently-computed `frame.z` was the CORRECT physical
            // outward direction (hand-verified against the box's own geometry), yet their combined
            // `signed_tetra_sum` contribution was `-16` instead of `+16` — a uniform sign inversion
            // across all 4, not a partial one, consistent with this being the SAME systematic fix
            // needed everywhere a `Curve3::Line` lateral is built this way (also used by
            // `Placement::General`'s NURBS path is unaffected: `general_lateral` never returns
            // `Surface::Plane`).
            let face = build_face(body, surf_id, &[LoopSpec { members, pcurves }], lateral_flipped, Tol::DEFAULT, rec);
            laterals.push(face);
        }
    }
    Ok(Prism { top, laterals })
}

/// 🧮 Line/`Curve3::Nurbs` lateral surface under an arbitrary rigid placement — the only two
/// kinds with a certified pcurve outside `Translate`/`Rotate` (see `📓️w2c-sweeps.md` §pcurve).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn general_lateral(curve: &Curve3, range: (f64, f64), map: &Affine3) -> Result<LateralSurface, KernelError> {
    match curve {
        Curve3::Line { .. } | Curve3::Nurbs { .. } => {
            let nc = curve.to_nurbs(range);
            let top: Vec<Pnt3> = nc.controls.iter().map(|&p| map.apply_point(p)).collect();
            let surface = Surface::Nurbs { u_knots: nc.knots.clone(), v_knots: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 1, 2).unwrap(), controls: nc.controls.iter().cloned().zip(top).map(|(a, b)| vec![a, b]).collect(), weights: nc.weights.iter().map(|&w| vec![w, w]).collect() };
            Ok(LateralSurface { surface, u_domain: range, v_bottom: 0.0, v_top: 1.0 })
        }
        _ => Err(KernelError::Operation("sweep: only line and free-form (already-NURBS) profile edges have a certified pcurve along a general path station (circle/ellipse profile edges are refused, not mis-parametrized)".into())),
    }
}

// #endregion 🔖️Prism

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn require_positive(name: &str, value: f64) -> Result<(), KernelError> {
    if value <= Tol::DEFAULT.value() {
        Err(KernelError::InvalidInput(format!("{name} must be positive, got {value}")))
    } else {
        Ok(())
    }
}

#[allow(dead_code)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn make_seed_vertex(body: &mut Body, p: Pnt3, rec: &mut OpRecorder) -> VertexId {
    make_vertex(body, p, Tol::DEFAULT, rec)
}
