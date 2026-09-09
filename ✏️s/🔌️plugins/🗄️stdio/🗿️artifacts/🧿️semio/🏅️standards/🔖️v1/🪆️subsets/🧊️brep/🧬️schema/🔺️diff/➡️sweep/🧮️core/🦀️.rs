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

use crate::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_loop, make_vertex};
use crate::standards::v1::subsets::brep::schema::diff::primitives::line_edge;
use crate::standards::v1::subsets::brep::schema::diff::transform::transform_face;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve2Id, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3};
use crate::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::{Affine3, Frame3};
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};

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
    Affine3 { linear: crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Mat3::from_columns(frame.x, frame.y, frame.z), translation: frame.origin.to_vec() }
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
            let surface = Surface::Nurbs {
                u_knots: nc.knots.clone(),
                v_knots: crate::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 1, 2).unwrap(),
                controls: nc.controls.iter().copied().zip(top).map(|(a, b)| vec![a, b]).collect(),
                weights: nc.weights.iter().map(|&w| vec![w, w]).collect(),
            };
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

/// 🧮 Builds one prism segment: `bottom` is flipped in place to face away from the travel
/// (recorded modified), a fresh `top = transform_face(bottom, map)` whose `flipped` is toggled so
/// the two caps face opposite ways, and one lateral face per profile edge of every loop (so holes
/// get their own tube faces).
///
/// # Lateral orientation
///
/// Two independent invariants fix every lateral face, and both are derived — never patched per
/// face or per surface kind (`📓️w2c-sweeps.md` §orientation, `📓️sweep-kernel-2026-09-09.md`):
///
/// * **Its loop is written in its own surface's natural `(u, v)` sense.** Every lateral surface
///   this file builds has `u` increasing along the profile edge's own curve direction and `v`
///   increasing along the travel, so the counter-clockwise circuit is unconditionally
///   `[(profile edge, forward), (rail at the edge's END vertex, forward), (top edge, reversed),
///   (rail at the edge's START vertex, reversed)]` — independent of how the CAP happens to
///   traverse that edge. Keying the circuit off the cap's `forward` instead (the previous
///   `[(b_edge, !f_i), …]` form) wound the loop CLOCKWISE in `(u, v)` for every profile edge the
///   cap traverses forward, which is the region's complement.
/// * **Its `flipped` states whether `du × dv` is the outward normal**, and follows from the
///   profile's winding and the sweep direction alone: the profile loop is counter-clockwise about
///   its own surface frame `Z_b`, so the material-outward in-plane direction at a profile edge is
///   `d × Z_b` for the cap's traversal direction `d = ±dir`, while `du × dv = dir × travel` and
///   `Z_b = ±travel` by the cap correction above. Both signs collapse to
///   `flipped = f_i != bottom.flipped`. It reproduces `🧱️primitives::make_box`'s hand-verified
///   side-face convention exactly, and it is what keeps `forward XOR flipped` opposite on every
///   shared cap edge and rail, so `validate_body` sees a coherent shell.
///
/// Compensating a mass-property sign with `flipped` (the previous `flipped = matches!(Plane)`)
/// inverted every extruded side wall's shading normal and tessellated winding instead, which is
/// what made a watertight prism measure `−V/3` as a triangle soup.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_prism(body: &mut Body, bottom: FaceId, placement: &Placement, rec: &mut OpRecorder) -> Result<Prism, KernelError> {
    let map = placement.affine();
    let n0 = planar_outward_normal(body, bottom)?;
    let travel = match placement {
        Placement::Translate { offset } => *offset,
        Placement::General { .. } => {
            let bfd = body
                .faces
                .get(bottom)
                .and_then(|f| match body.surfaces.get(f.surface) {
                    Some(Surface::Plane { frame }) => Some(*frame),
                    _ => None,
                })
                .ok_or_else(|| KernelError::InvalidInput("sweep profile face must be planar".into()))?;
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
    let bottom_flipped = body.faces.get(bottom).unwrap().flipped;
    let top = transform_face(body, bottom, &map, rec)?;
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
            let (b_edge, f_i) = {
                let c = body.coedges.get(bce[k]).unwrap();
                (c.edge, c.forward)
            };
            let t_edge = body.coedges.get(tce[k]).unwrap().edge;
            let (s_bot, e_bot) = body.coedge_endpoints(bce[k]).unwrap();
            let (s_top, e_top) = body.coedge_endpoints(tce[k]).unwrap();
            let (start_bot, start_top, end_bot, end_top) = if f_i { (s_bot, s_top, e_bot, e_top) } else { (e_bot, e_top, s_bot, s_top) };
            let start_rail = *rail_cache.entry(start_bot).or_insert_with(|| {
                let (a, b) = (body.vertices.get(start_bot).unwrap().position, body.vertices.get(start_top).unwrap().position);
                line_edge(body, a, b, start_bot, start_top, Tol::DEFAULT, rec)
            });
            let end_rail = *rail_cache.entry(end_bot).or_insert_with(|| {
                let (a, b) = (body.vertices.get(end_bot).unwrap().position, body.vertices.get(end_top).unwrap().position);
                line_edge(body, a, b, end_bot, end_top, Tol::DEFAULT, rec)
            });
            let curve = body.curves3.get(body.edges.get(b_edge).unwrap().curve).unwrap().clone();
            let range = body.edges.get(b_edge).unwrap().range;
            let lat = match placement {
                Placement::Translate { offset } => translate_lateral(&curve, range, *offset)?,
                Placement::General { map } => general_lateral(&curve, range, map)?,
            };
            let lateral_flipped = f_i != bottom_flipped;
            let surf_id = body.surfaces.insert(lat.surface);
            let u0 = lat.u_domain.0;
            let u1 = lat.u_domain.1;
            let u_slope = (u1 - u0) / (range.1 - range.0);
            let u_origin = u0 - u_slope * range.0;
            let v_slope = lat.v_top - lat.v_bottom;
            let bottom_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u_origin, lat.v_bottom), dir: Vec2::new(u_slope, 0.0) });
            let top_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u_origin, lat.v_top), dir: Vec2::new(u_slope, 0.0) });
            let start_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u0, lat.v_bottom), dir: Vec2::new(0.0, v_slope) });
            let end_pc = body.curves2.insert(Curve2::Line { origin: Pnt2::new(u1, lat.v_bottom), dir: Vec2::new(0.0, v_slope) });
            let members = vec![(b_edge, true), (end_rail, true), (t_edge, false), (start_rail, false)];
            let pcurves = vec![(bottom_pc, range), (end_pc, (0.0, 1.0)), (top_pc, range), (start_pc, (0.0, 1.0))];
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
            let surface = Surface::Nurbs {
                u_knots: nc.knots.clone(),
                v_knots: crate::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 1, 2).unwrap(),
                controls: nc.controls.iter().copied().zip(top).map(|(a, b)| vec![a, b]).collect(),
                weights: nc.weights.iter().map(|&w| vec![w, w]).collect(),
            };
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
