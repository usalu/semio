//! 🧱 Analytic solid primitives: box/sphere/cylinder/cone/torus + wires/planar faces/convex hull.
//!
//! Builds closed [`Body`](crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body) solids exclusively through
//! [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler`] editors, attaching shared [`Curve3`](crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3) /
//! [`Surface`](crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface) geometry from the body's pools, with a stored [`Curve2`] p-curve on
//! every coedge. Topology: box (V=8 E=12 F=6), sphere/cylinder/cone/torus as exact analytic
//! surfaces with seam/degenerate edges (no faceting, no `segments`), convex hull as
//! coplanar-merged polygon faces (Quickhull + boundary-walk merge).

//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧱️primitives` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3. Rewritten to
//! exact analytic primitives (no `segments`, no triangle-soup topology) in ticket
//! 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave W1-E.
//!

use std::collections::HashMap;
use std::f64::consts::{FRAC_PI_2, TAU};

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve2Id, EdgeId, FaceId, SolidId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Edge};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};

// #region 🔖️Wire

/// 🧱 An ordered chain of oriented edges produced by a wire constructor (not yet bound to a face).
#[derive(Clone, Debug, PartialEq)]
pub struct Wire {
    pub members: Vec<(EdgeId, bool)>,
    pub vertices: Vec<VertexId>,
    pub closed: bool,
}

// #endregion 🔖️Wire

// #region 🔖️Helpers

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_positive(name: &str, value: f64) -> Result<(), KernelError> {
    if value <= Tol::DEFAULT.value() {
        Err(KernelError::InvalidInput(format!("{name} must be positive, got {value}")))
    } else {
        Ok(())
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn attach_face(body: &mut Body, surface_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::SurfaceId, members: &[(EdgeId, bool)], flipped: bool, tol: Tol, rec: &mut OpRecorder) -> FaceId {
    let outer = make_loop(body, placeholder_face(), members);
    let face = add_face(body, surface_id, Some(outer), vec![], flipped, tol, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    face
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn line_edge(body: &mut Body, a: Pnt3, b: Pnt3, va: VertexId, vb: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    let curve = body.curves3.insert(Curve3::Line { origin: a, dir: b - a });
    make_edge(body, curve, (0.0, 1.0), va, vb, tol, rec)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plane_at(origin: Pnt3, normal: Vec3) -> Surface {
    Surface::Plane { frame: Frame3::from_normal(origin, normal).expect("plane frame") }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn finish_solid(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> SolidId {
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newell_normal(points: &[Pnt3]) -> Option<Vec3> {
    if points.len() < 3 {
        return None;
    }
    let mut n = Vec3::ZERO;
    for i in 0..points.len() {
        let p = points[i];
        let q = points[(i + 1) % points.len()];
        n.x += (p.y - q.y) * (p.z + q.z);
        n.y += (p.z - q.z) * (p.x + q.x);
        n.z += (p.x - q.x) * (p.y + q.y);
    }
    n.normalized()
}

/// 🧱 Inserts a `Curve2::Line` p-curve (`origin + dir·t`) — the common shape for every straight
/// (non-seam-circle) p-curve segment below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn line2(body: &mut Body, origin: (f64, f64), dir: (f64, f64)) -> Curve2Id {
    body.curves2.insert(Curve2::Line { origin: Pnt2::new(origin.0, origin.1), dir: Vec2::new(dir.0, dir.1) })
}

/// 🧱 Inserts a `Curve2::Circle` p-curve — used where a 3D circle edge lands on a *planar* cap
/// whose local frame is a reflection of the circle's own frame (see `w1e-primitives.md` §caps).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn circle2(body: &mut Body, center: (f64, f64), radius: f64) -> Curve2Id {
    body.curves2.insert(Curve2::Circle { center: Pnt2::new(center.0, center.1), radius })
}

/// 🧱 Attaches an exact planar p-curve to every coedge of `face`'s outer loop, via
/// [`Surface::project_curve`] (exact for `Line`/`Circle`/`Ellipse` edges on a `Plane` through its
/// `analytic_pcurve_shortcut`, tolerance-checked fit otherwise) — the general-purpose counterpart
/// to the hand-derived `line2`/`circle2` + [`set_outer_pcurves`] calls the analytic primitives
/// (sphere/cylinder/cone/torus) use for their own known-simple edges. `make_box`/
/// `make_convex_hull`/[`make_planar_face_from_wire`] call this so their planar faces' coedges
/// carry p-curves too — every coedge must, per `check_missing_pcurves`'s ERROR-level rule
/// (`✅validation-report/🧪️body/🦀️.rs`), and these builders had no p-curve step at all before
/// (a wave-1/wave-1 integration gap between this file and validation's stricter check, not a
/// pre-existing bug in either alone). `prange` is always the edge's own `(0.0, 1.0)` order, per
/// [`set_outer_pcurves`]'s own documented convention — never reversed for `forward`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attach_planar_face_pcurves(body: &mut Body, face: FaceId, frame: Frame3, members: &[(EdgeId, bool)], tol: f64) {
    let surface = Surface::Plane { frame };
    let pcurves: Vec<(Curve2Id, (f64, f64))> = members
        .iter()
        .map(|&(edge_id, _forward)| {
            let edge = body.edges.get(edge_id).expect("member edge exists");
            let curve3 = body.curves3.get(edge.curve).expect("edge curve exists").clone();
            let pcurve = surface.project_curve(&curve3, edge.range, tol);
            (body.curves2.insert(pcurve), (0.0, 1.0))
        })
        .collect();
    set_outer_pcurves(body, face, &pcurves);
}

/// 🧱 Stamps `pcurves` (one `(curve2, prange)` pair per coedge, in `attach_face`'s own `members`
/// order) onto `face`'s outer loop. `prange` always shares its interpolating parameter `s` with
/// the coedge's underlying [`Edge::range`] — i.e. it is *never* reversed to account for
/// `Coedge::forward` (see [`crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::check_same_parameter`], which samples both ranges with
/// the same `s`) — only reparametrized (phase/sign/offset) when the p-curve targets a different
/// surface frame than the edge's own curve.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn set_outer_pcurves(body: &mut Body, face: FaceId, pcurves: &[(Curve2Id, (f64, f64))]) {
    let outer = body.faces.get(face).and_then(|f| f.outer).expect("face has an outer loop");
    let coedges = body.loop_coedges(outer);
    debug_assert_eq!(coedges.len(), pcurves.len(), "one p-curve per coedge, in member order");
    for (&coedge_id, &(pcurve, prange)) in coedges.iter().zip(pcurves) {
        let coedge = body.coedges.get_mut(coedge_id).expect("just-created coedge");
        coedge.pcurve = Some(pcurve);
        coedge.prange = prange;
    }
}

/// 🧱 A "degenerate" edge (OCCT convention): both endpoints are the same vertex *and* the 3D
/// curve is a zero-length line — the standard closing device for a pole where the whole `u`
/// range of a periodic surface collapses to one point. Used by the Euler-characteristic tests
/// below to exclude poles from the edge count (see `w1e-primitives.md` §sphere for why the naive
/// `V−E+F` would otherwise read 0, not 2, for the sphere).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn is_degenerate_edge(edge: &Edge, curve: &Curve3) -> bool {
    edge.v0 == edge.v1 && matches!(curve, Curve3::Line { dir, .. } if dir.norm() < 1e-12)
}

// #endregion 🔖️Helpers

// #region 🔖️Solids

/// 🧱 Axis-aligned box from the origin to `(w, d, h)` with six planar faces (V=8, E=12, F=6).
/// Threads the caller-owned `rec` through every euler call so the whole box's [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpDelta`]
/// is observable after this call returns, instead of being discarded at the function boundary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_box(body: &mut Body, w: f64, d: f64, h: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_positive("box width", w)?;
    require_positive("box depth", d)?;
    require_positive("box height", h)?;
    let tol = Tol::DEFAULT;
    let corners = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(w, 0.0, 0.0), Pnt3::new(w, d, 0.0), Pnt3::new(0.0, d, 0.0), Pnt3::new(0.0, 0.0, h), Pnt3::new(w, 0.0, h), Pnt3::new(w, d, h), Pnt3::new(0.0, d, h)];
    let v: Vec<VertexId> = corners.iter().map(|&p| make_vertex(body, p, tol, rec)).collect();
    let eb0 = line_edge(body, corners[0], corners[1], v[0], v[1], tol, rec);
    let eb1 = line_edge(body, corners[1], corners[2], v[1], v[2], tol, rec);
    let eb2 = line_edge(body, corners[2], corners[3], v[2], v[3], tol, rec);
    let eb3 = line_edge(body, corners[3], corners[0], v[3], v[0], tol, rec);
    let et0 = line_edge(body, corners[4], corners[5], v[4], v[5], tol, rec);
    let et1 = line_edge(body, corners[5], corners[6], v[5], v[6], tol, rec);
    let et2 = line_edge(body, corners[6], corners[7], v[6], v[7], tol, rec);
    let et3 = line_edge(body, corners[7], corners[4], v[7], v[4], tol, rec);
    let ev0 = line_edge(body, corners[0], corners[4], v[0], v[4], tol, rec);
    let ev1 = line_edge(body, corners[1], corners[5], v[1], v[5], tol, rec);
    let ev2 = line_edge(body, corners[2], corners[6], v[2], v[6], tol, rec);
    let ev3 = line_edge(body, corners[3], corners[7], v[3], v[7], tol, rec);

    let frame_bottom = Frame3::from_normal(corners[0], -Vec3::Z).expect("plane frame");
    let frame_top = Frame3::from_normal(corners[4], Vec3::Z).expect("plane frame");
    let frame_front = Frame3::from_normal(corners[0], -Vec3::Y).expect("plane frame");
    let frame_back = Frame3::from_normal(corners[3], Vec3::Y).expect("plane frame");
    let frame_left = Frame3::from_normal(corners[0], -Vec3::X).expect("plane frame");
    let frame_right = Frame3::from_normal(corners[1], Vec3::X).expect("plane frame");
    let s_bottom = body.surfaces.insert(Surface::Plane { frame: frame_bottom });
    let s_top = body.surfaces.insert(Surface::Plane { frame: frame_top });
    let s_front = body.surfaces.insert(Surface::Plane { frame: frame_front });
    let s_back = body.surfaces.insert(Surface::Plane { frame: frame_back });
    let s_left = body.surfaces.insert(Surface::Plane { frame: frame_left });
    let s_right = body.surfaces.insert(Surface::Plane { frame: frame_right });
    let bottom_members = [(eb0, false), (eb3, false), (eb2, false), (eb1, false)];
    let top_members = [(et0, true), (et1, true), (et2, true), (et3, true)];
    let front_members = [(eb0, true), (ev1, true), (et0, false), (ev0, false)];
    let back_members = [(eb2, true), (ev3, true), (et2, false), (ev2, false)];
    let left_members = [(eb3, true), (ev0, true), (et3, false), (ev3, false)];
    let right_members = [(eb1, true), (ev2, true), (et1, false), (ev1, false)];
    let bottom = attach_face(body, s_bottom, &bottom_members, false, tol, rec);
    let top = attach_face(body, s_top, &top_members, false, tol, rec);
    let front = attach_face(body, s_front, &front_members, false, tol, rec);
    let back = attach_face(body, s_back, &back_members, false, tol, rec);
    let left = attach_face(body, s_left, &left_members, false, tol, rec);
    let right = attach_face(body, s_right, &right_members, false, tol, rec);
    let tolv = tol.value();
    attach_planar_face_pcurves(body, bottom, frame_bottom, &bottom_members, tolv);
    attach_planar_face_pcurves(body, top, frame_top, &top_members, tolv);
    attach_planar_face_pcurves(body, front, frame_front, &front_members, tolv);
    attach_planar_face_pcurves(body, back, frame_back, &back_members, tolv);
    attach_planar_face_pcurves(body, left, frame_left, &left_members, tolv);
    attach_planar_face_pcurves(body, right, frame_right, &right_members, tolv);
    Ok(finish_solid(body, vec![bottom, top, front, back, left, right], rec))
}

/// 🧱 Sphere centered at the origin: ONE analytic [`Surface::Sphere`] face, bounded by a single
/// seam edge (the u=0 half-meridian great circle from south to north pole, curve domain
/// `[-π/2, π/2]`) used twice — once at u=0, once at u=2π — plus two degenerate edges (3D curve
/// collapsed to the pole point, see [`is_degenerate_edge`]) that close the (u,v) parameter
/// rectangle along `v=±π/2`. This is the standard OCCT-style sphere topology (chosen over the old
/// two-hemisphere-plus-equator split so there is exactly one seam, matching cylinder/cone/torus'
/// own single-seam convention) — see `w1e-primitives.md` for the coordinate-with-W1-F note.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_sphere(body: &mut Body, radius: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_positive("sphere radius", radius)?;
    let tol = Tol::DEFAULT;
    let frame = Frame3::WORLD;
    let south_pt = Pnt3::new(0.0, 0.0, -radius);
    let north_pt = Pnt3::new(0.0, 0.0, radius);
    let v_south = make_vertex(body, south_pt, tol, rec);
    let v_north = make_vertex(body, north_pt, tol, rec);

    // Meridian frame: local x=world X, local y=world Z ⇒ eval(t) = (r·cos t, 0, r·sin t), which
    // is exactly `sphere.eval(0, t)` (frame=WORLD) — t IS v, no reparametrization needed.
    let meridian_frame = Frame3 { origin: Pnt3::new(0.0, 0.0, 0.0), x: Vec3::X, y: Vec3::Z, z: -Vec3::Y };
    let meridian = body.curves3.insert(Curve3::Circle { frame: meridian_frame, radius });
    let e_seam = make_edge(body, meridian, (-FRAC_PI_2, FRAC_PI_2), v_south, v_north, tol, rec);

    let south_curve = body.curves3.insert(Curve3::Line { origin: south_pt, dir: Vec3::ZERO });
    let e_south = make_edge(body, south_curve, (0.0, TAU), v_south, v_south, tol, rec);
    let north_curve = body.curves3.insert(Curve3::Line { origin: north_pt, dir: Vec3::ZERO });
    let e_north = make_edge(body, north_curve, (0.0, TAU), v_north, v_north, tol, rec);

    let surface = body.surfaces.insert(Surface::Sphere { frame, radius });
    let face = attach_face(body, surface, &[(e_seam, true), (e_north, true), (e_seam, false), (e_south, false)], false, tol, rec);

    let p_seam_u0 = line2(body, (0.0, 0.0), (0.0, 1.0));
    let p_north = line2(body, (0.0, FRAC_PI_2), (1.0, 0.0));
    let p_seam_u_tau = line2(body, (TAU, 0.0), (0.0, 1.0));
    let p_south = line2(body, (0.0, -FRAC_PI_2), (1.0, 0.0));
    set_outer_pcurves(body, face, &[(p_seam_u0, (-FRAC_PI_2, FRAC_PI_2)), (p_north, (0.0, TAU)), (p_seam_u_tau, (-FRAC_PI_2, FRAC_PI_2)), (p_south, (0.0, TAU))]);

    Ok(finish_solid(body, vec![face], rec))
}

/// 🧱 Cylinder along +Z from `z=0` to `z=height`: one analytic [`Surface::Cylinder`] lateral face
/// (single seam at u=0) plus two planar caps, all four faces' coedges carrying exact p-curves.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_cylinder(body: &mut Body, radius: f64, height: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_positive("cylinder radius", radius)?;
    require_positive("cylinder height", height)?;
    let tol = Tol::DEFAULT;
    let bottom_frame = Frame3::WORLD;
    let top_frame = Frame3 { origin: Pnt3::new(0.0, 0.0, height), x: Vec3::X, y: Vec3::Y, z: Vec3::Z };
    let bot_pt = bottom_frame.to_world(Pnt3::new(radius, 0.0, 0.0));
    let top_pt = top_frame.to_world(Pnt3::new(radius, 0.0, 0.0));
    let v_bot = make_vertex(body, bot_pt, tol, rec);
    let v_top = make_vertex(body, top_pt, tol, rec);

    let bot_circle = body.curves3.insert(Curve3::Circle { frame: bottom_frame, radius });
    let e_bot = make_edge(body, bot_circle, (0.0, TAU), v_bot, v_bot, tol, rec);
    let top_circle = body.curves3.insert(Curve3::Circle { frame: top_frame, radius });
    let e_top = make_edge(body, top_circle, (0.0, TAU), v_top, v_top, tol, rec);
    let e_seam = line_edge(body, bot_pt, top_pt, v_bot, v_top, tol, rec);

    let cyl = body.surfaces.insert(Surface::Cylinder { frame: Frame3::WORLD, radius });
    let lateral = attach_face(body, cyl, &[(e_bot, true), (e_seam, true), (e_top, false), (e_seam, false)], false, tol, rec);
    let p_bot_lat = line2(body, (0.0, 0.0), (1.0, 0.0));
    let p_seam_up = line2(body, (0.0, 0.0), (0.0, height));
    let p_top_lat = line2(body, (0.0, height), (1.0, 0.0));
    set_outer_pcurves(body, lateral, &[(p_bot_lat, (0.0, TAU)), (p_seam_up, (0.0, 1.0)), (p_top_lat, (0.0, TAU)), (p_seam_up, (0.0, 1.0))]);

    // Cap frames are reflections of the lateral circles' own frame (x kept, y/z negated) so each
    // circle's world trace maps onto the cap's local (u,v) as the SAME angle mirrored: p = -t.
    let bottom_cap_frame = Frame3 { origin: Pnt3::new(0.0, 0.0, 0.0), x: Vec3::X, y: -Vec3::Y, z: -Vec3::Z };
    let s_bottom = body.surfaces.insert(Surface::Plane { frame: bottom_cap_frame });
    let bottom = attach_face(body, s_bottom, &[(e_bot, false)], false, tol, rec);
    let p_bot_cap = circle2(body, (0.0, 0.0), radius);
    set_outer_pcurves(body, bottom, &[(p_bot_cap, (0.0, -TAU))]);

    // The top cap frame is IDENTICAL to the top circle's own frame (both x=X,y=Y,z=Z), so its
    // p-curve is the direct, unreflected angle: p = t.
    let s_top = body.surfaces.insert(Surface::Plane { frame: top_frame });
    let top = attach_face(body, s_top, &[(e_top, true)], false, tol, rec);
    let p_top_cap = circle2(body, (0.0, 0.0), radius);
    set_outer_pcurves(body, top, &[(p_top_cap, (0.0, TAU))]);

    Ok(finish_solid(body, vec![lateral, bottom, top], rec))
}

/// 🧱 Pointed cone with base radius at `z=0` and apex at `(0,0,height)`: one analytic
/// [`Surface::Cone`] lateral face plus a planar base cap. No separate degenerate apex edge — the
/// single seam edge (base→apex) is used TWICE in the lateral loop (up, then down), both
/// traversals terminating at the shared apex vertex. This is the standard representation for a
/// full (untrimmed) cone: since the whole `u` range already collapses to the apex point at v=0
/// (radius `v·tan(half_angle)` → 0), an extra explicit degenerate edge there would just duplicate
/// what the seam's own two endpoints already express — unlike the sphere, whose TWO poles are not
/// both endpoints of the same seam edge and so each needs its own degenerate closer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_cone(body: &mut Body, radius: f64, height: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_positive("cone radius", radius)?;
    require_positive("cone height", height)?;
    let tol = Tol::DEFAULT;
    let half_angle = radius.atan2(height);
    if half_angle <= Tol::DEFAULT.value() || half_angle >= FRAC_PI_2 {
        return Err(KernelError::InvalidInput(format!("cone half-angle out of range: {half_angle}")));
    }
    let apex = Pnt3::new(0.0, 0.0, height);
    let base_frame = Frame3::WORLD;
    let base_pt = base_frame.to_world(Pnt3::new(radius, 0.0, 0.0));
    let v_apex = make_vertex(body, apex, tol, rec);
    let v_base = make_vertex(body, base_pt, tol, rec);

    let base_circle = body.curves3.insert(Curve3::Circle { frame: base_frame, radius });
    let e_circle = make_edge(body, base_circle, (0.0, TAU), v_base, v_base, tol, rec);
    let e_seam = line_edge(body, base_pt, apex, v_base, v_apex, tol, rec);

    let cone_frame = Frame3 { origin: apex, x: Vec3::X, y: Vec3::Y, z: -Vec3::Z };
    let cone_surf = body.surfaces.insert(Surface::Cone { frame: cone_frame, half_angle });
    let lateral = attach_face(body, cone_surf, &[(e_circle, true), (e_seam, true), (e_seam, false)], false, tol, rec);
    let p_base_lat = line2(body, (0.0, height), (1.0, 0.0));
    let p_seam = line2(body, (0.0, height), (0.0, -height));
    set_outer_pcurves(body, lateral, &[(p_base_lat, (0.0, TAU)), (p_seam, (0.0, 1.0)), (p_seam, (0.0, 1.0))]);

    let base_cap_frame = Frame3 { origin: Pnt3::new(0.0, 0.0, 0.0), x: Vec3::X, y: -Vec3::Y, z: -Vec3::Z };
    let s_base = body.surfaces.insert(Surface::Plane { frame: base_cap_frame });
    let base = attach_face(body, s_base, &[(e_circle, false)], false, tol, rec);
    let p_base_cap = circle2(body, (0.0, 0.0), radius);
    set_outer_pcurves(body, base, &[(p_base_cap, (0.0, -TAU))]);

    Ok(finish_solid(body, vec![lateral, base], rec))
}

/// 🧱 Torus in the XY plane: ONE analytic [`Surface::Torus`] face, the classic "fundamental
/// polygon" identification of a torus as a square with opposite sides glued — two full-circle
/// seam edges (the u=0 meridian and the v=0 equatorial circle), each used TWICE (once per glued
/// side pair), all four coedges meeting at the single vertex `(u,v)=(0,0)`. No degenerate edges:
/// genus 1 ⇒ χ = V−E+F = 1−2+1 = 0 (the existing test's own expectation, unlike the χ=2 solids).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_torus(body: &mut Body, major: f64, minor: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_positive("torus major radius", major)?;
    require_positive("torus minor radius", minor)?;
    if minor >= major {
        return Err(KernelError::InvalidInput(format!("torus minor radius ({minor}) must be less than major radius ({major})")));
    }
    let tol = Tol::DEFAULT;
    let shared_pt = Pnt3::new(major + minor, 0.0, 0.0);
    let v_shared = make_vertex(body, shared_pt, tol, rec);

    // Meridian (u=0 tube cross-section): local x=world X, local y=world Z, centered on the main
    // circle at (major,0,0) ⇒ eval(t) = (major+minor·cos t, 0, minor·sin t) = `torus.eval(0, t)`.
    let meridian_frame = Frame3 { origin: Pnt3::new(major, 0.0, 0.0), x: Vec3::X, y: Vec3::Z, z: -Vec3::Y };
    let meridian = body.curves3.insert(Curve3::Circle { frame: meridian_frame, radius: minor });
    let e_meridian = make_edge(body, meridian, (0.0, TAU), v_shared, v_shared, tol, rec);

    // Equatorial (v=0 main circle): eval(t) = ((major+minor)·cos t, (major+minor)·sin t, 0) =
    // `torus.eval(t, 0)`.
    let equatorial = body.curves3.insert(Curve3::Circle { frame: Frame3::WORLD, radius: major + minor });
    let e_equatorial = make_edge(body, equatorial, (0.0, TAU), v_shared, v_shared, tol, rec);

    let surface = body.surfaces.insert(Surface::Torus { frame: Frame3::WORLD, major_radius: major, minor_radius: minor });
    let face = attach_face(body, surface, &[(e_meridian, true), (e_equatorial, true), (e_meridian, false), (e_equatorial, false)], false, tol, rec);

    // u=2π ≡ u=0 and v=2π ≡ v=0 exactly (trig periodicity), so BOTH occurrences of each seam use
    // the same direct `p = t` shape — only the constant offset along the other axis differs.
    let p_meridian_u0 = line2(body, (0.0, 0.0), (0.0, 1.0));
    let p_equatorial_v_tau = line2(body, (0.0, TAU), (1.0, 0.0));
    let p_meridian_u_tau = line2(body, (TAU, 0.0), (0.0, 1.0));
    let p_equatorial_v0 = line2(body, (0.0, 0.0), (1.0, 0.0));
    set_outer_pcurves(body, face, &[(p_meridian_u0, (0.0, TAU)), (p_equatorial_v_tau, (0.0, TAU)), (p_meridian_u_tau, (0.0, TAU)), (p_equatorial_v0, (0.0, TAU))]);

    Ok(finish_solid(body, vec![face], rec))
}

/// 🧱 Builds a (possibly non-convex) solid from a triangle soup — used ONLY for mesh import and
/// as [`make_convex_hull`]'s last-resort fallback (see [`merge_coplanar_triangles`], which is
/// preferred whenever the hull's planar clusters walk to a clean boundary).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_from_triangle_soup(body: &mut Body, triangles: &[[Pnt3; 3]], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if triangles.is_empty() {
        return Err(KernelError::InvalidInput("triangle soup is empty".into()));
    }
    let tol = Tol::DEFAULT;
    let quant = |v: f64| -> i64 { (v * 1e6).round() as i64 };
    let mut key_to_idx: HashMap<(i64, i64, i64), usize> = HashMap::new();
    let mut positions: Vec<Pnt3> = Vec::new();
    let mut verts: Vec<VertexId> = Vec::new();
    for tri in triangles {
        for &p in tri {
            let key = (quant(p.x), quant(p.y), quant(p.z));
            if key_to_idx.contains_key(&key) {
                continue;
            }
            key_to_idx.insert(key, positions.len());
            positions.push(p);
            verts.push(make_vertex(body, p, tol, rec));
        }
    }
    let mut edge_map: HashMap<(usize, usize), EdgeId> = HashMap::new();
    let mut faces = Vec::with_capacity(triangles.len());
    for tri in triangles {
        let idxs = [
            *key_to_idx.get(&(quant(tri[0].x), quant(tri[0].y), quant(tri[0].z))).unwrap(),
            *key_to_idx.get(&(quant(tri[1].x), quant(tri[1].y), quant(tri[1].z))).unwrap(),
            *key_to_idx.get(&(quant(tri[2].x), quant(tri[2].y), quant(tri[2].z))).unwrap(),
        ];
        let mut members = Vec::with_capacity(3);
        for (ia, ib) in [(idxs[0], idxs[1]), (idxs[1], idxs[2]), (idxs[2], idxs[0])] {
            let key = (ia.min(ib), ia.max(ib));
            let (eid, forward) = if let Some(&existing) = edge_map.get(&key) {
                let edge = body.edges.get(existing).unwrap();
                (existing, edge.v0 == verts[ia])
            } else {
                let eid = line_edge(body, positions[ia], positions[ib], verts[ia], verts[ib], tol, rec);
                edge_map.insert(key, eid);
                (eid, true)
            };
            members.push((eid, forward));
        }
        let normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]).normalized().unwrap_or(Vec3::Z);
        let surface = body.surfaces.insert(plane_at(tri[0], normal));
        faces.push(attach_face(body, surface, &members, false, tol, rec));
    }
    Ok(finish_solid(body, faces, rec))
}

/// 🧱 Convex hull of a point cloud as a closed solid whose faces are the coplanar-MERGED polygons
/// of the underlying Quickhull triangulation (e.g. 8 box corners → 6 quad faces, not 12
/// triangles) — see [`merge_coplanar_triangles`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_convex_hull(body: &mut Body, points: &[Pnt3], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let hull = convex_hull_3d(points).ok_or_else(|| KernelError::InvalidInput("points are coplanar or degenerate — cannot form a 3D convex hull".into()))?;
    let tol = Tol::DEFAULT;
    let vertex_ids: Vec<VertexId> = hull.vertices.iter().map(|&p| make_vertex(body, p, tol, rec)).collect();
    let groups = merge_coplanar_triangles(&hull);
    let mut edge_map: HashMap<(usize, usize), EdgeId> = HashMap::new();
    let mut faces = Vec::with_capacity(groups.len());
    for group in &groups {
        let boundary = &group.boundary;
        let mut members = Vec::with_capacity(boundary.len());
        for i in 0..boundary.len() {
            let ia = boundary[i];
            let ib = boundary[(i + 1) % boundary.len()];
            let key = (ia.min(ib), ia.max(ib));
            let (eid, forward) = if let Some(&existing) = edge_map.get(&key) {
                let edge = body.edges.get(existing).unwrap();
                (existing, edge.v0 == vertex_ids[ia])
            } else {
                let eid = line_edge(body, hull.vertices[ia], hull.vertices[ib], vertex_ids[ia], vertex_ids[ib], tol, rec);
                edge_map.insert(key, eid);
                (eid, true)
            };
            members.push((eid, forward));
        }
        let frame = Frame3::from_normal(hull.vertices[boundary[0]], group.normal).expect("plane frame");
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let face = attach_face(body, surface, &members, false, tol, rec);
        attach_planar_face_pcurves(body, face, frame, &members, tol.value());
        faces.push(face);
    }
    Ok(finish_solid(body, faces, rec))
}

// #endregion 🔖️Solids

// #region 🔖️WiresFaces

/// 🧱 Open or closed polyline wire through `points` (closed requires ≥ 3 points).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_polyline_wire(body: &mut Body, points: &[Pnt3], closed: bool, rec: &mut OpRecorder) -> Result<Wire, KernelError> {
    if points.len() < 2 {
        return Err(KernelError::InvalidInput("polyline needs at least 2 points".into()));
    }
    if closed && points.len() < 3 {
        return Err(KernelError::InvalidInput("closed polyline needs at least 3 points".into()));
    }
    let tol = Tol::DEFAULT;
    let vertices: Vec<VertexId> = points.iter().map(|&p| make_vertex(body, p, tol, rec)).collect();
    let mut members = Vec::new();
    let n_edges = if closed { points.len() } else { points.len() - 1 };
    for i in 0..n_edges {
        let j = (i + 1) % points.len();
        let eid = line_edge(body, points[i], points[j], vertices[i], vertices[j], tol, rec);
        members.push((eid, true));
    }
    Ok(Wire { members, vertices, closed })
}

/// 🧱 Axis-aligned rectangle wire in the XY plane from the origin to `(width, height)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_rectangle_wire(body: &mut Body, width: f64, height: f64, rec: &mut OpRecorder) -> Result<Wire, KernelError> {
    require_positive("rectangle width", width)?;
    require_positive("rectangle height", height)?;
    make_polyline_wire(body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(width, 0.0, 0.0), Pnt3::new(width, height, 0.0), Pnt3::new(0.0, height, 0.0)], true, rec)
}

/// 🧱 Regular `sides`-gon wire of given `radius` in the XY plane, centered at the origin.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_regular_polygon_wire(body: &mut Body, radius: f64, sides: usize, rec: &mut OpRecorder) -> Result<Wire, KernelError> {
    require_positive("polygon radius", radius)?;
    if sides < 3 {
        return Err(KernelError::InvalidInput(format!("polygon needs at least 3 sides, got {sides}")));
    }
    let points: Vec<Pnt3> = (0..sides)
        .map(|i| {
            let a = TAU * i as f64 / sides as f64;
            Pnt3::new(radius * a.cos(), radius * a.sin(), 0.0)
        })
        .collect();
    make_polyline_wire(body, &points, true, rec)
}

/// 🧱 Planar face from a closed point loop (Newell normal); points must be non-collinear.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_planar_face_from_points(body: &mut Body, points: &[Pnt3], rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    if points.len() < 3 {
        return Err(KernelError::InvalidInput("planar face needs at least 3 points".into()));
    }
    let normal = newell_normal(points).ok_or_else(|| KernelError::InvalidInput("points are collinear".into()))?;
    let wire = make_polyline_wire(body, points, true, rec)?;
    make_planar_face_from_wire(body, &wire, points[0], normal, rec)
}

/// 🧱 Planar face whose outer loop is an existing closed [`Wire`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_planar_face_from_wire(body: &mut Body, wire: &Wire, origin: Pnt3, normal: Vec3, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    if !wire.closed {
        return Err(KernelError::InvalidInput("planar face requires a closed wire".into()));
    }
    if wire.members.is_empty() {
        return Err(KernelError::InvalidInput("planar face wire is empty".into()));
    }
    let frame = Frame3::from_normal(origin, normal).expect("plane frame");
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let face = attach_face(body, surface, &wire.members, false, Tol::DEFAULT, rec);
    attach_planar_face_pcurves(body, face, frame, &wire.members, Tol::DEFAULT.value());
    Ok(face)
}

// #endregion 🔖️WiresFaces

// #region 🔖️ConvexHull

#[derive(Clone)]
struct HullFace {
    verts: [usize; 3],
    normal: Vec3,
    d: f64,
    alive: bool,
}

struct ConvexHull {
    vertices: Vec<Pnt3>,
    faces: Vec<[usize; 3]>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_normal(pts: &[Pnt3], a: usize, b: usize, c: usize) -> Vec3 {
    (pts[b] - pts[a]).cross(pts[c] - pts[a]).normalized().unwrap_or(Vec3::Z)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn signed_distance(face: &HullFace, p: Pnt3) -> f64 {
    face.normal.dot(p.to_vec()) + face.d
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_initial_tetrahedron(pts: &[Pnt3]) -> Option<[usize; 4]> {
    let mut i0 = 0usize;
    for (i, p) in pts.iter().enumerate() {
        if p.x < pts[i0].x {
            i0 = i;
        }
    }
    let mut i1 = None;
    let mut best = 0.0;
    for (i, p) in pts.iter().enumerate() {
        if i == i0 {
            continue;
        }
        let dist = p.distance(pts[i0]);
        if dist > best {
            best = dist;
            i1 = Some(i);
        }
    }
    let i1 = i1?;
    let mut i2 = None;
    best = 0.0;
    let edge = pts[i1] - pts[i0];
    for (i, p) in pts.iter().enumerate() {
        if i == i0 || i == i1 {
            continue;
        }
        let area = edge.cross(*p - pts[i0]).norm();
        if area > best {
            best = area;
            i2 = Some(i);
        }
    }
    let i2 = i2?;
    if best <= 1e-12 {
        return None;
    }
    let n = face_normal(pts, i0, i1, i2);
    let mut i3 = None;
    best = 0.0;
    for (i, p) in pts.iter().enumerate() {
        if i == i0 || i == i1 || i == i2 {
            continue;
        }
        let dist = n.dot(*p - pts[i0]).abs();
        if dist > best {
            best = dist;
            i3 = Some(i);
        }
    }
    let i3 = i3?;
    if best <= 1e-12 {
        return None;
    }
    Some([i0, i1, i2, i3])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn convex_hull_3d(points: &[Pnt3]) -> Option<ConvexHull> {
    if points.len() < 4 {
        return None;
    }
    let tol = 1e-10;
    let mut pts: Vec<Pnt3> = Vec::with_capacity(points.len());
    for &p in points {
        if pts.iter().all(|q| q.distance(p) >= tol) {
            pts.push(p);
        }
    }
    if pts.len() < 4 {
        return None;
    }
    let tet = find_initial_tetrahedron(&pts)?;
    let mut faces: Vec<HullFace> = Vec::new();
    let tet_faces = [[tet[0], tet[1], tet[2]], [tet[0], tet[2], tet[3]], [tet[0], tet[3], tet[1]], [tet[1], tet[3], tet[2]]];
    for &[a, b, c] in &tet_faces {
        let normal = face_normal(&pts, a, b, c);
        let d = -normal.dot(pts[a].to_vec());
        faces.push(HullFace { verts: [a, b, c], normal, d, alive: true });
    }
    let centroid = Pnt3::new((pts[tet[0]].x + pts[tet[1]].x + pts[tet[2]].x + pts[tet[3]].x) / 4.0, (pts[tet[0]].y + pts[tet[1]].y + pts[tet[2]].y + pts[tet[3]].y) / 4.0, (pts[tet[0]].z + pts[tet[1]].z + pts[tet[2]].z + pts[tet[3]].z) / 4.0);
    for face in &mut faces {
        if signed_distance(face, centroid) > 0.0 {
            face.normal = -face.normal;
            face.d = -face.d;
            face.verts.swap(1, 2);
        }
    }
    let tet_set: std::collections::HashSet<usize> = tet.iter().copied().collect();
    for (pi, &point) in pts.iter().enumerate() {
        if tet_set.contains(&pi) {
            continue;
        }
        let visible: Vec<usize> = faces.iter().enumerate().filter(|(_, f)| f.alive && signed_distance(f, point) > tol).map(|(i, _)| i).collect();
        if visible.is_empty() {
            continue;
        }
        let mut horizon: Vec<[usize; 2]> = Vec::new();
        for &fi in &visible {
            let verts = faces[fi].verts;
            for edge_idx in 0..3 {
                let e = [verts[edge_idx], verts[(edge_idx + 1) % 3]];
                let twin_visible = visible.iter().any(|&fj| {
                    fj != fi && {
                        let w = faces[fj].verts;
                        (0..3).any(|k| w[k] == e[1] && w[(k + 1) % 3] == e[0])
                    }
                });
                if !twin_visible {
                    horizon.push(e);
                }
            }
        }
        for &fi in &visible {
            faces[fi].alive = false;
        }
        for edge in horizon {
            let a = edge[0];
            let b = edge[1];
            let normal = face_normal(&pts, a, b, pi);
            let d = -normal.dot(pts[a].to_vec());
            let mut face = HullFace { verts: [a, b, pi], normal, d, alive: true };
            if signed_distance(&face, centroid) > 0.0 {
                face.normal = -face.normal;
                face.d = -face.d;
                face.verts.swap(1, 2);
            }
            faces.push(face);
        }
    }
    let out_faces: Vec<[usize; 3]> = faces.into_iter().filter(|f| f.alive).map(|f| f.verts).collect();
    if out_faces.len() < 4 {
        return None;
    }
    Some(ConvexHull { vertices: pts, faces: out_faces })
}

// #endregion 🔖️ConvexHull

// #region 🔖️CoplanarMerge

/// 🧱 One merged planar face of a convex hull: all Quickhull triangles sharing (within tolerance)
/// the same supporting plane, reduced to their outer boundary loop.
struct FaceGroup {
    normal: Vec3,
    boundary: Vec<usize>,
}

/// 🧱 Quantizes a plane `(normal, d)` into a hashable bucket key — Quickhull triangles that are
/// genuinely coplanar (same supporting plane of the convex body) compute bit-close normal/offset
/// values from their own 3 vertices, so a `1e-7`-scale round is enough to bucket them together
/// without merging two triangles that merely happen to be nearly-but-not-exactly coplanar.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn plane_key(normal: Vec3, d: f64) -> (i64, i64, i64, i64) {
    let q = |x: f64| (x * 1e7).round() as i64;
    (q(normal.x), q(normal.y), q(normal.z), q(d))
}

/// 🧱 Groups `hull`'s triangles by supporting plane, then reduces each group to its outer
/// boundary loop by cancelling every directed edge against its opposite-direction twin (present
/// exactly when two triangles in the group share that edge — the standard consistent-winding
/// invariant a convex-hull triangulation maintains) and walking what remains. A singleton group
/// (no coplanar neighbor) degenerates to its one triangle's own three edges, so non-merged faces
/// (e.g. every face of a tetrahedron) round-trip unchanged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn merge_coplanar_triangles(hull: &ConvexHull) -> Vec<FaceGroup> {
    let mut clusters: HashMap<(i64, i64, i64, i64), (Vec3, Vec<[usize; 3]>)> = HashMap::new();
    for &[a, b, c] in &hull.faces {
        let normal = face_normal(&hull.vertices, a, b, c);
        let d = -normal.dot(hull.vertices[a].to_vec());
        let key = plane_key(normal, d);
        clusters.entry(key).or_insert_with(|| (normal, Vec::new())).1.push([a, b, c]);
    }
    let mut groups = Vec::with_capacity(clusters.len());
    for (normal, tris) in clusters.into_values() {
        let mut directed: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        for tri in &tris {
            for i in 0..3 {
                directed.insert((tri[i], tri[(i + 1) % 3]));
            }
        }
        let mut next: HashMap<usize, usize> = HashMap::new();
        for &(a, b) in &directed {
            if !directed.contains(&(b, a)) {
                next.insert(a, b);
            }
        }
        let Some((&start, _)) = next.iter().next() else { continue };
        let mut boundary = vec![start];
        let mut current = start;
        loop {
            let Some(&n) = next.get(&current) else { break };
            if n == start {
                break;
            }
            boundary.push(n);
            current = n;
            if boundary.len() > next.len() {
                break; // malformed boundary guard: never loop forever on corrupt data
            }
        }
        groups.push(FaceGroup { normal, boundary });
    }
    groups
}

// #endregion 🔖️CoplanarMerge

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn solid_counts(body: &Body, solid: SolidId) -> (usize, usize, usize) {
        let faces = body.solid_faces(solid);
        let mut edge_ids = std::collections::HashSet::new();
        let mut vertex_ids = std::collections::HashSet::new();
        for face in &faces {
            for coedge in body.face_coedges(*face) {
                let edge = body.coedges.get(coedge).unwrap().edge;
                edge_ids.insert(edge);
                let e = body.edges.get(edge).unwrap();
                vertex_ids.insert(e.v0);
                vertex_ids.insert(e.v1);
            }
        }
        (vertex_ids.len(), edge_ids.len(), faces.len())
    }

    /// 🧱 `(V, E_real, F, χ)` with degenerate edges (see [`is_degenerate_edge`]) excluded from the
    /// edge count — the "count degenerate edges consistently" convention the ticket asks for, so
    /// a pole-bearing sphere reads χ=2 like every other genus-0 solid instead of 0.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn euler_excluding_degenerate(body: &Body, solid: SolidId) -> (usize, usize, usize, i64) {
        let faces = body.solid_faces(solid);
        let mut edge_ids = std::collections::HashSet::new();
        let mut vertex_ids = std::collections::HashSet::new();
        for face in &faces {
            for coedge in body.face_coedges(*face) {
                let edge = body.coedges.get(coedge).unwrap().edge;
                edge_ids.insert(edge);
                let e = body.edges.get(edge).unwrap();
                vertex_ids.insert(e.v0);
                vertex_ids.insert(e.v1);
            }
        }
        let e_real = edge_ids.iter().filter(|&&eid| { let e = body.edges.get(eid).unwrap(); let c = body.curves3.get(e.curve).unwrap(); !is_degenerate_edge(e, c) }).count();
        let v = vertex_ids.len();
        let f = faces.len();
        (v, e_real, f, v as i64 - e_real as i64 + f as i64)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_rings_ok(body: &Body) {
        let issues = validate_body(body);
        let ring_issues: Vec<_> = issues.iter().filter(|i| matches!(i.code, "empty-loop" | "broken-ring" | "loop-not-closed" | "next-prev-mismatch")).collect();
        assert!(ring_issues.is_empty(), "ring integrity failed: {ring_issues:?}");
    }

    /// 🧱 Every coedge on `solid` must carry a p-curve, and every p-curve must agree with its
    /// edge's 3D curve at matching parameters (the same "same-parameter" check `validate_body`
    /// runs, asserted here directly so a failure names the offending primitive test, not just
    /// "some validation issue").
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_pcurves_present_and_consistent(body: &Body, solid: SolidId) {
        for face in body.solid_faces(solid) {
            for loop_id in body.face_loops(face) {
                for coedge in body.loop_coedges(loop_id) {
                    let co = body.coedges.get(coedge).unwrap();
                    assert!(co.pcurve.is_some(), "coedge {coedge:?} on face {face:?} has no p-curve");
                }
            }
        }
        let issues = validate_body(body);
        let bad: Vec<_> = issues.iter().filter(|i| i.code == "same-parameter-violated").collect();
        assert!(bad.is_empty(), "p-curve/3D-curve mismatch: {bad:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn make_box_euler_and_validate() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!((v, e, f), (8, 12, 6));
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "box should validate clean: {issues:?}");
    }

    /// 🧱 The whole box's provenance escapes the call — this is Phase 1's real deliverable, tested.
    #[semio_framework_async_macros::async_test]
    async fn make_box_surfaces_its_op_delta_to_the_caller() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
        let delta = rec.into_delta();
        assert_eq!(delta.generated.len(), 8 + 12 + 6 + 1 + 1, "vertices + edges + faces + shell + solid");
        assert!(delta.deleted.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn make_box_rejects_non_positive() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        assert!(make_box(&mut body, 0.0, 1.0, 1.0, &mut rec).is_err());
        assert!(make_box(&mut body, 1.0, -1.0, 1.0, &mut rec).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn make_sphere_one_face_with_seam_and_poles() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let (v, _e, f) = solid_counts(&body, solid);
        assert_eq!(f, 1, "one analytic spherical face, no faceting");
        assert_eq!(v, 2, "two poles");
        let (_, _, _, chi) = euler_excluding_degenerate(&body, solid);
        assert_eq!(chi, 2, "χ must read 2 once degenerate pole edges are excluded from E");
        assert_rings_ok(&body);
        assert_pcurves_present_and_consistent(&body, solid);
        assert!(make_sphere(&mut body, -1.0, &mut rec).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn make_cylinder_three_analytic_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!(f, 3);
        assert_eq!((v, e), (2, 3));
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        assert_pcurves_present_and_consistent(&body, solid);
    }

    #[semio_framework_async_macros::async_test]
    async fn make_cone_pointed_two_analytic_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_cone(&mut body, 1.0, 2.0, &mut rec).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!(f, 2);
        assert_eq!((v, e), (2, 2));
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        assert_pcurves_present_and_consistent(&body, solid);
    }

    #[semio_framework_async_macros::async_test]
    async fn make_torus_genus_one_analytic_single_face() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_torus(&mut body, 3.0, 1.0, &mut rec).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!((v, e, f), (1, 2, 1), "fundamental-polygon torus: 1 vertex, 2 seam edges, 1 face");
        assert_eq!(v as i64 - e as i64 + f as i64, 0, "torus χ must be 0 (genus 1)");
        assert_rings_ok(&body);
        assert_pcurves_present_and_consistent(&body, solid);
        assert!(make_torus(&mut body, 1.0, 1.0, &mut rec).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn make_convex_hull_tetrahedron() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let pts = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
        let solid = make_convex_hull(&mut body, &pts, &mut rec).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!((v, e, f), (4, 6, 4));
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "{issues:?}");
    }

    /// 🧱 The coplanar-merge deliverable: 8 box corners must yield 6 quad faces, not 12 triangles.
    #[semio_framework_async_macros::async_test]
    async fn make_convex_hull_box_merges_coplanar_triangles_into_six_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let pts = [
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(2.0, 0.0, 0.0),
            Pnt3::new(2.0, 3.0, 0.0),
            Pnt3::new(0.0, 3.0, 0.0),
            Pnt3::new(0.0, 0.0, 4.0),
            Pnt3::new(2.0, 0.0, 4.0),
            Pnt3::new(2.0, 3.0, 4.0),
            Pnt3::new(0.0, 3.0, 4.0),
        ];
        let solid = make_convex_hull(&mut body, &pts, &mut rec).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!((v, e, f), (8, 12, 6), "merged hull of a box must look like a box");
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        assert_rings_ok(&body);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn make_convex_hull_rejects_coplanar() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let pts = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 0.0)];
        assert!(make_convex_hull(&mut body, &pts, &mut rec).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn wires_and_planar_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let rect = make_rectangle_wire(&mut body, 2.0, 3.0, &mut rec).unwrap();
        assert!(rect.closed);
        assert_eq!(rect.members.len(), 4);
        let face = make_planar_face_from_wire(&mut body, &rect, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).unwrap();
        assert_eq!(body.loop_coedges(body.faces.get(face).unwrap().outer.unwrap()).len(), 4);
        let poly = make_regular_polygon_wire(&mut body, 1.0, 6, &mut rec).unwrap();
        assert_eq!(poly.members.len(), 6);
        let face2 = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(0.0, 1.0, 1.0)], &mut rec).unwrap();
        assert!(body.faces.get(face2).unwrap().outer.is_some());
        assert_rings_ok(&body);
    }

    #[semio_framework_async_macros::async_test]
    async fn open_polyline_wire() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let wire = make_polyline_wire(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0)], false, &mut rec).unwrap();
        assert!(!wire.closed);
        assert_eq!(wire.members.len(), 2);
        assert!(make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).is_err());
    }

    /// 🧱 Volume/area against closed forms, via the existing (not-yet-W1-F-updated) mass-properties
    /// quadrature — see `w1e-primitives.md` for the honest pass/fail report on each shape.
    #[semio_framework_async_macros::async_test]
    async fn closed_form_volumes_via_mass_properties() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
        let tol = 1e-3;

        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let sphere = make_sphere(&mut body, 2.0, &mut rec).unwrap();
        let expected = 4.0 / 3.0 * std::f64::consts::PI * 8.0;
        let got = solid_volume(&body, sphere, tol).unwrap();
        assert!((got - expected).abs() / expected < 1e-6, "sphere volume: got {got}, expected {expected}");

        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let cyl = make_cylinder(&mut body, 1.5, 4.0, &mut rec).unwrap();
        let expected = std::f64::consts::PI * 1.5 * 1.5 * 4.0;
        let got = solid_volume(&body, cyl, tol).unwrap();
        assert!((got - expected).abs() / expected < 1e-2, "cylinder volume: got {got}, expected {expected}");

        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let cone = make_cone(&mut body, 1.0, 3.0, &mut rec).unwrap();
        let expected = std::f64::consts::PI * 1.0 * 1.0 * 3.0 / 3.0;
        let got = solid_volume(&body, cone, tol).unwrap();
        assert!((got - expected).abs() / expected < 1e-2, "cone volume: got {got}, expected {expected}");

        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let torus = make_torus(&mut body, 3.0, 1.0, &mut rec).unwrap();
        let expected = 2.0 * std::f64::consts::PI * std::f64::consts::PI * 3.0 * 1.0 * 1.0;
        let got = solid_volume(&body, torus, tol).unwrap();
        assert!((got - expected).abs() / expected < 1e-2, "torus volume: got {got}, expected {expected}");
    }
}
// #endregion 🔖️Tests
