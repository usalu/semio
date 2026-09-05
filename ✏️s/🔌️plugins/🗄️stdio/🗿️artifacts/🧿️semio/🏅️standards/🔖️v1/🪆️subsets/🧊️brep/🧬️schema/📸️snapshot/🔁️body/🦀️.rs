//! 🔁 Lossless `Body ↔ SemioBrepSnapshot` conversion (ticket 26/09/03/BREP-KERNEL-DEPENDENCY-
//! FREE-RUNTIME, wave W3-A) — closes the audit's §1.3/§7/§10.4/Phase-5 gap: the native `Body`
//! (generational arenas, coedges, p-curves, tolerances, `PersistentLabel`s) and the artifact
//! `SemioBrepSnapshot` (id-keyed, flat) used to be TWO disconnected representations; this is the
//! versioned mapping between them the audit calls for.
//!
//! ## Identity convention
//! Every entity id `to_snapshot` emits is literally `label.0.to_string()` (a bare decimal `u64`)
//! — the entity's own [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]. `from_snapshot` exploits this:
//! an id that parses as `u64` is trusted as literally that label (so a document that has been
//! through `to_snapshot` round-trips its exact labels — required so two independent mutation
//! constructions against the same document never mint colliding labels, `SemioBrepSnapshot::next_label`'s own doc comment); a
//! non-numeric id (STEP import's `"v12"`/`"e7"`, hand-authored fixtures' `"🐼️v1"`) mints a fresh
//! label instead, since such a document has no persistent-label history to preserve. Loops and
//! coedges carry no `PersistentLabel` in the native model (see [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::BrepArenaSeed`]'s own doc
//! comment) — `to_snapshot` mints ordinal ids (`"lp0"`, `"co0"`, …) for them, stable only within
//! one `to_snapshot` call, never round-tripped as identity (matching `BrepArenaSeed.loops`' own
//! index-addressed convention, generalized to also carry p-curve + ring position).
//!
//! ## Known, deliberate lossy corners (documented, not silently dropped)
//! - **In-plane rotation of analytic curves/surfaces.** Native `Circle`/`Ellipse`/`Cylinder`/
//!   `Cone`/`Sphere`/`Torus` carry a full [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3`] (origin + orthonormal x/y/z);
//!   [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepCurve`]/[`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepSurface`] only carry origin + the z-axis (matching STEP AP214's own
//!   `ref_direction`-unset gap, audit §10.2/§10.4). `from_snapshot` reconstructs the missing x/y
//!   via [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3::from_normal`]'s deterministic canonical choice — round-trips EXACTLY for
//!   any body built through the ordinary primitive constructors (`🔺️diff/🧱️primitives`, which
//!   themselves call `from_normal`/use world axes), lossy only for a hand-crafted body with a
//!   deliberately non-canonical in-plane rotation. Fixing this needs widening `BrepCurve`/
//!   `BrepSurface` themselves — out of scope here (would break every non-owned `🚪️io/**`
//!   construction site; left to a future wave, same as the audit's own STEP gap).
//! - **`BrepSurface::Cone.radius`** is informational/STEP-shaped only (AP214's "radius at the
//!   position's plane"); this bridge always treats `origin` as the native apex directly (radius
//!   there is trivially `0`) since cone isn't in this wave's required round-trip set (box/sphere/
//!   cylinder/torus).
//! - **A closed sub-arc edge whose two endpoints coincide** (same vertex, but spanning less than
//!   the curve's full period) round-trips as a FULL period — every primitive constructor's own
//!   same-vertex edges (cylinder/torus rims, sphere pole closers) genuinely ARE full periods, so
//!   this only affects a future fillet/blend-style short closed arc (not yet implemented).
//!
//! Everything else — every vertex/edge/face/shell/solid, every coedge incl. its p-curve/range,
//! every persistent label, every tolerance, the label high-water mark, exact NURBS knots/weights/
//! degree for curves/surfaces/p-curves — round-trips exactly.

use std::collections::HashMap;

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{FaceId, LoopId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::{curve_ops, Curve2, Curve3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::{Body, BrepArenaSeed, SeedEdge, SeedFace, SeedShell, SeedSolid, SeedVertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCoedge, BrepCurve, BrepCurve2, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot};

//#region 🔖️PointVectorBridge
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point3_to_pnt3(p: SemioPoint3) -> Pnt3 {
    Pnt3 { x: p.x, y: p.y, z: p.z }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pnt3_to_point3(p: Pnt3) -> SemioPoint3 {
    SemioPoint3 { x: p.x, y: p.y, z: p.z }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point3_to_vec3(p: SemioPoint3) -> Vec3 {
    Vec3 { x: p.x, y: p.y, z: p.z }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vec3_to_point3(v: Vec3) -> SemioPoint3 {
    SemioPoint3 { x: v.x, y: v.y, z: v.z }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point2_to_pnt2(p: SemioPoint2) -> Pnt2 {
    Pnt2 { x: p.x, y: p.y }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pnt2_to_point2(p: Pnt2) -> SemioPoint2 {
    SemioPoint2 { x: p.x, y: p.y }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point2_to_vec2(p: SemioPoint2) -> Vec2 {
    Vec2 { x: p.x, y: p.y }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vec2_to_point2(v: Vec2) -> SemioPoint2 {
    SemioPoint2 { x: v.x, y: v.y }
}
//#endregion 🔖️PointVectorBridge

//#region 🔖️CurveSurfaceBridge
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frame_from_origin_axis(origin: SemioPoint3, axis: SemioPoint3) -> Frame3 {
    Frame3::from_normal(point3_to_pnt3(origin), point3_to_vec3(axis)).unwrap_or(Frame3::WORLD)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn knot_vector_or_uniform(knots: &[f64], degree: u32, control_point_count: usize) -> KnotVector {
    KnotVector::new(knots.to_vec(), degree as usize, control_point_count).unwrap_or_else(|| KnotVector::clamped_uniform(control_point_count, degree as usize))
}

/// 📈️ `BrepCurve` → native `Curve3` — see this file's module doc for the frame/in-plane-rotation
/// caveat on `Circle`/`Ellipse`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn brep_curve_to_native(c: &BrepCurve) -> Curve3 {
    match c {
        BrepCurve::Line { origin, direction } => Curve3::Line { origin: point3_to_pnt3(*origin), dir: point3_to_vec3(*direction) },
        BrepCurve::Circle { center, axis, radius } => Curve3::Circle { frame: frame_from_origin_axis(*center, *axis), radius: *radius },
        BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => Curve3::Ellipse { frame: frame_from_origin_axis(*center, *axis), major_radius: *radius_major, minor_radius: *radius_minor },
        BrepCurve::Nurbs { control_points, weights, degree, knots } => {
            Curve3::Nurbs { knots: knot_vector_or_uniform(knots, *degree, control_points.len()), controls: control_points.iter().map(|p| point3_to_pnt3(*p)).collect(), weights: weights.clone() }
        }
    }
}

/// 📤️ Native `Curve3` → `BrepCurve` — the `to_snapshot` half.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn native_curve_to_brep(c: &Curve3) -> BrepCurve {
    match c {
        Curve3::Line { origin, dir } => BrepCurve::Line { origin: pnt3_to_point3(*origin), direction: vec3_to_point3(*dir) },
        Curve3::Circle { frame, radius } => BrepCurve::Circle { center: pnt3_to_point3(frame.origin), axis: vec3_to_point3(frame.z), radius: *radius },
        Curve3::Ellipse { frame, major_radius, minor_radius } => BrepCurve::Ellipse { center: pnt3_to_point3(frame.origin), axis: vec3_to_point3(frame.z), radius_major: *major_radius, radius_minor: *minor_radius },
        Curve3::Nurbs { knots, controls, weights } => BrepCurve::Nurbs { control_points: controls.iter().map(|p| pnt3_to_point3(*p)).collect(), weights: weights.clone(), degree: knots.degree as u32, knots: knots.knots.clone() },
    }
}

/// 🗺️ `BrepSurface` → native `Surface` — `Cone.radius` and the in-plane rotation of every
/// non-`Nurbs` variant are informational-only, see this file's module doc.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn brep_surface_to_native(s: &BrepSurface) -> Surface {
    match s {
        BrepSurface::Plane { origin, normal } => Surface::Plane { frame: frame_from_origin_axis(*origin, *normal) },
        BrepSurface::Cylinder { origin, axis, radius } => Surface::Cylinder { frame: frame_from_origin_axis(*origin, *axis), radius: *radius },
        BrepSurface::Cone { origin, axis, half_angle, .. } => Surface::Cone { frame: frame_from_origin_axis(*origin, *axis), half_angle: *half_angle },
        BrepSurface::Sphere { center, radius } => Surface::Sphere { frame: Frame3 { origin: point3_to_pnt3(*center), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, radius: *radius },
        BrepSurface::Torus { center, axis, major_radius, minor_radius } => Surface::Torus { frame: frame_from_origin_axis(*center, *axis), major_radius: *major_radius, minor_radius: *minor_radius },
        BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => {
            let (u, v) = (*u_count as usize, *v_count as usize);
            let mut controls = vec![vec![Pnt3::default(); v]; u];
            let mut w = vec![vec![1.0; v]; u];
            for i in 0..u {
                for j in 0..v {
                    let idx = i * v + j;
                    controls[i][j] = point3_to_pnt3(control_points[idx]);
                    w[i][j] = weights[idx];
                }
            }
            Surface::Nurbs { u_knots: knot_vector_or_uniform(knots_u, *degree_u, u), v_knots: knot_vector_or_uniform(knots_v, *degree_v, v), controls, weights: w }
        }
    }
}

/// 📤️ Native `Surface` → `BrepSurface` — the `to_snapshot` half.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn native_surface_to_brep(s: &Surface) -> BrepSurface {
    match s {
        Surface::Plane { frame } => BrepSurface::Plane { origin: pnt3_to_point3(frame.origin), normal: vec3_to_point3(frame.z) },
        Surface::Cylinder { frame, radius } => BrepSurface::Cylinder { origin: pnt3_to_point3(frame.origin), axis: vec3_to_point3(frame.z), radius: *radius },
        Surface::Cone { frame, half_angle } => BrepSurface::Cone { origin: pnt3_to_point3(frame.origin), axis: vec3_to_point3(frame.z), radius: 0.0, half_angle: *half_angle },
        Surface::Sphere { frame, radius } => BrepSurface::Sphere { center: pnt3_to_point3(frame.origin), radius: *radius },
        Surface::Torus { frame, major_radius, minor_radius } => BrepSurface::Torus { center: pnt3_to_point3(frame.origin), axis: vec3_to_point3(frame.z), major_radius: *major_radius, minor_radius: *minor_radius },
        Surface::Nurbs { u_knots, v_knots, controls, weights } => {
            let u = controls.len();
            let v = controls.first().map(|row| row.len()).unwrap_or(0);
            let mut control_points = Vec::with_capacity(u * v);
            let mut flat_weights = Vec::with_capacity(u * v);
            for row_c in controls {
                for &p in row_c {
                    control_points.push(pnt3_to_point3(p));
                }
            }
            for row_w in weights {
                for &w in row_w {
                    flat_weights.push(w);
                }
            }
            BrepSurface::Nurbs { control_points, weights: flat_weights, u_count: u as u32, v_count: v as u32, degree_u: u_knots.degree as u32, degree_v: v_knots.degree as u32, knots_u: u_knots.knots.clone(), knots_v: v_knots.knots.clone() }
        }
    }
}

/// 🗺️➰️ `BrepCurve2` (p-curve) → native `Curve2` — lossless: unlike `BrepCurve`/`BrepSurface`,
/// `Curve2` carries no `Frame3` (it's already flat 2D), so every variant round-trips exactly,
/// `x_axis` included.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn brep_curve2_to_native(c: &BrepCurve2) -> Curve2 {
    match c {
        BrepCurve2::Line { origin, direction } => Curve2::Line { origin: point2_to_pnt2(*origin), dir: point2_to_vec2(*direction) },
        BrepCurve2::Circle { center, radius } => Curve2::Circle { center: point2_to_pnt2(*center), radius: *radius },
        BrepCurve2::Ellipse { center, x_axis, radius_major, radius_minor } => Curve2::Ellipse { center: point2_to_pnt2(*center), x_axis: point2_to_vec2(*x_axis), major_radius: *radius_major, minor_radius: *radius_minor },
        BrepCurve2::Nurbs { control_points, weights, degree, knots } => {
            Curve2::Nurbs { knots: knot_vector_or_uniform(knots, *degree, control_points.len()), controls: control_points.iter().map(|p| point2_to_pnt2(*p)).collect(), weights: weights.clone() }
        }
    }
}

/// 📤️ Native `Curve2` → `BrepCurve2` — the `to_snapshot` half.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn native_curve2_to_brep(c: &Curve2) -> BrepCurve2 {
    match c {
        Curve2::Line { origin, dir } => BrepCurve2::Line { origin: pnt2_to_point2(*origin), direction: vec2_to_point2(*dir) },
        Curve2::Circle { center, radius } => BrepCurve2::Circle { center: pnt2_to_point2(*center), radius: *radius },
        Curve2::Ellipse { center, x_axis, major_radius, minor_radius } => BrepCurve2::Ellipse { center: pnt2_to_point2(*center), x_axis: vec2_to_point2(*x_axis), radius_major: *major_radius, radius_minor: *minor_radius },
        Curve2::Nurbs { knots, controls, weights } => BrepCurve2::Nurbs { control_points: controls.iter().map(|p| pnt2_to_point2(*p)).collect(), weights: weights.clone(), degree: knots.degree as u32, knots: knots.knots.clone() },
    }
}
//#endregion 🔖️CurveSurfaceBridge

//#region 🔖️EdgeRange
/// 📏 The `(t0, t1)` an edge's native `Curve3` spans between `start`/`end` — see this file's
/// module doc "closed sub-arc" caveat for the `same_vertex` branch. `same_vertex` (both ends the
/// SAME vertex, e.g. a full-circle rim) is inherently ambiguous from position alone (a zero-length
/// arc at that point is equally consistent with the two positions), so it is resolved as the
/// curve's full natural period — matching every full-circle/closed-NURBS seam edge every
/// primitive constructor (`🔺️diff/🧱️primitives`) emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn edge_range(curve: &Curve3, start: Pnt3, end: Pnt3, same_vertex: bool) -> (f64, f64) {
    let domain = curve.domain();
    if same_vertex {
        return if domain.0.is_finite() && domain.1.is_finite() { domain } else { (0.0, 1.0) };
    }
    let search_domain = if domain.0.is_finite() && domain.1.is_finite() { domain } else { (-1e6, 1e6) };
    let t0 = curve_ops::closest_parameter(curve, search_domain, start, 1e-9).t;
    let t1 = curve_ops::closest_parameter(curve, search_domain, end, 1e-9).t;
    (t0, t1)
}
//#endregion 🔖️EdgeRange

//#region 🔖️LabelResolution
/// 📜️ Resolves a snapshot's string ids into [`PersistentLabel`]s — see this file's module doc
/// "Identity convention". Numeric ids are trusted verbatim (the `to_snapshot` round-trip case);
/// non-numeric ids mint a fresh label above the document's high-water mark.
struct LabelResolver {
    map: HashMap<String, PersistentLabel>,
    next_fresh: u64,
}

impl LabelResolver {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolve(&mut self, id: &str) -> PersistentLabel {
        if let Some(&label) = self.map.get(id) {
            return label;
        }
        let label = match id.parse::<u64>() {
            Ok(n) => PersistentLabel(n),
            Err(_) => {
                let fresh = PersistentLabel(self.next_fresh);
                self.next_fresh += 1;
                fresh
            }
        };
        self.map.insert(id.to_string(), label);
        label
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn scan_numeric_high_water(snapshot: &SemioBrepSnapshot) -> u64 {
    let mut high = snapshot.next_label;
    let mut bump = |id: &str, high: &mut u64| {
        if let Ok(n) = id.parse::<u64>() {
            *high = (*high).max(n + 1);
        }
    };
    for v in &snapshot.vertices {
        bump(&v.id, &mut high);
    }
    for e in &snapshot.edges {
        bump(&e.id, &mut high);
    }
    for f in &snapshot.faces {
        bump(&f.id, &mut high);
    }
    for s in &snapshot.shells {
        bump(&s.id, &mut high);
    }
    for s in &snapshot.solids {
        bump(&s.id, &mut high);
    }
    high
}
//#endregion 🔖️LabelResolution

//#region 🔖️ToSnapshot
/// 🧱️ Emits one loop (as `BrepLoop`) plus its ring's `BrepCoedge`s into `snapshot`, returning the
/// minted loop id. `loop_ordinal`/`coedge_ordinal` are per-`to_snapshot`-call counters (see
/// module doc "Identity convention" — loop/coedge ids are ordinal, not persistent).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn emit_loop(body: &Body, loop_id: LoopId, snapshot: &mut SemioBrepSnapshot, loop_ordinal: &mut usize, coedge_ordinal: &mut usize) -> String {
    let loop_label = format!("lp{loop_ordinal}");
    *loop_ordinal += 1;
    let coedge_ids = body.loop_coedges(loop_id);
    let n = coedge_ids.len();
    let coedge_labels: Vec<String> = (0..n)
        .map(|_| {
            let label = format!("co{coedge_ordinal}");
            *coedge_ordinal += 1;
            label
        })
        .collect();
    let mut brep_edges = Vec::with_capacity(n);
    for (i, &cid) in coedge_ids.iter().enumerate() {
        let Some(coedge) = body.coedges.get(cid) else { continue };
        let Some(edge) = body.edges.get(coedge.edge) else { continue };
        let edge_label = edge.label.0.to_string();
        brep_edges.push(BrepLoopEdge { edge: edge_label.clone(), orientation: coedge.forward });
        let pcurve = coedge.pcurve.and_then(|pid| body.curves2.get(pid)).map(native_curve2_to_brep);
        snapshot.coedges.push(BrepCoedge {
            id: coedge_labels[i].clone(),
            edge: edge_label,
            forward: coedge.forward,
            pcurve,
            prange: coedge.prange,
            loop_id: loop_label.clone(),
            next: coedge_labels[(i + 1) % n].clone(),
            prev: coedge_labels[(i + n - 1) % n].clone(),
        });
    }
    snapshot.loops.push(BrepLoop { id: loop_label.clone(), edges: brep_edges });
    loop_label
}

impl Body {
    /// 🔁️ The lossless `Body → SemioBrepSnapshot` half — see this file's module doc.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioBrepSnapshot {
        let mut snapshot = SemioBrepSnapshot::default();

        for (_, v) in self.vertices.iter() {
            snapshot.vertices.push(BrepVertex { id: v.label.0.to_string(), point: pnt3_to_point3(v.position), tol: v.tol.0 });
        }
        for (_, e) in self.edges.iter() {
            let Some(curve) = self.curves3.get(e.curve) else { continue };
            let Some(v0) = self.vertices.get(e.v0) else { continue };
            let Some(v1) = self.vertices.get(e.v1) else { continue };
            snapshot.edges.push(BrepEdge { id: e.label.0.to_string(), start_vertex: v0.label.0.to_string(), end_vertex: v1.label.0.to_string(), curve: native_curve_to_brep(curve), tol: e.tol.0 });
        }

        let mut loop_ordinal = 0usize;
        let mut coedge_ordinal = 0usize;
        for (_, f) in self.faces.iter() {
            let Some(surface) = self.surfaces.get(f.surface) else { continue };
            let outer_loop = f.outer.map(|lid| emit_loop(self, lid, &mut snapshot, &mut loop_ordinal, &mut coedge_ordinal)).unwrap_or_default();
            let inner_loops = f.inners.iter().map(|&lid| emit_loop(self, lid, &mut snapshot, &mut loop_ordinal, &mut coedge_ordinal)).collect();
            snapshot.faces.push(BrepFace { id: f.label.0.to_string(), outer_loop, inner_loops, surface: native_surface_to_brep(surface), orientation: !f.flipped, tol: f.tol.0 });
        }

        for (_, s) in self.shells.iter() {
            let faces = s.faces.iter().filter_map(|&fid| self.faces.get(fid)).map(|f| BrepShellFace { face: f.label.0.to_string(), orientation: true }).collect();
            snapshot.shells.push(BrepShell { id: s.label.0.to_string(), faces });
        }
        for (_, s) in self.solids.iter() {
            let mut shells = Vec::with_capacity(1 + s.inners.len());
            if let Some(outer) = self.shells.get(s.outer) {
                shells.push(BrepSolidShell { shell: outer.label.0.to_string(), is_void: false });
            }
            for &sh in &s.inners {
                if let Some(shell) = self.shells.get(sh) {
                    shells.push(BrepSolidShell { shell: shell.label.0.to_string(), is_void: true });
                }
            }
            snapshot.solids.push(BrepSolid { id: s.label.0.to_string(), shells });
        }

        snapshot.next_label = self.labels.next();
        snapshot
    }
}
//#endregion 🔖️ToSnapshot

//#region 🔖️FromSnapshot
impl Body {
    /// 🔁️ The lossless `SemioBrepSnapshot → Body` half — see this file's module doc. Built on top
    /// of [`Body::from_seed`] (reusing its proven topology/label-preservation machinery) plus a
    /// second pass that attaches p-curves onto the reconstructed coedges from this snapshot's
    /// `coedges` collection (when present — see module doc "Identity convention").
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn from_snapshot(snapshot: &SemioBrepSnapshot) -> Result<Body, KernelError> {
        let high_water = scan_numeric_high_water(snapshot);
        let mut resolver = LabelResolver { map: HashMap::new(), next_fresh: high_water };

        let vertex_pos: HashMap<&str, Pnt3> = snapshot.vertices.iter().map(|v| (v.id.as_str(), point3_to_pnt3(v.point))).collect();

        let seed_vertices: Vec<SeedVertex> = snapshot.vertices.iter().map(|v| SeedVertex { label: resolver.resolve(&v.id), position: point3_to_pnt3(v.point), tol: Tol::new(if v.tol > 0.0 { v.tol } else { Tol::DEFAULT.0 }) }).collect();

        let mut seed_edges: Vec<SeedEdge> = Vec::with_capacity(snapshot.edges.len());
        for e in &snapshot.edges {
            let start_pos = vertex_pos.get(e.start_vertex.as_str()).copied().ok_or_else(|| KernelError::MissingEntity(e.start_vertex.clone()))?;
            let end_pos = vertex_pos.get(e.end_vertex.as_str()).copied().ok_or_else(|| KernelError::MissingEntity(e.end_vertex.clone()))?;
            let same_vertex = e.start_vertex == e.end_vertex;
            let curve = brep_curve_to_native(&e.curve);
            let range = edge_range(&curve, start_pos, end_pos, same_vertex);
            seed_edges.push(SeedEdge {
                label: resolver.resolve(&e.id),
                v0: resolver.resolve(&e.start_vertex),
                v1: resolver.resolve(&e.end_vertex),
                curve,
                range,
                tol: Tol::new(if e.tol > 0.0 { e.tol } else { Tol::DEFAULT.0 }),
            });
        }

        let mut seed_loops: Vec<Vec<(PersistentLabel, bool)>> = Vec::new();
        let mut loop_index_by_id: HashMap<String, usize> = HashMap::new();
        let mut resolve_loop = |loop_id: &str, resolver: &mut LabelResolver| -> Option<usize> {
            if loop_id.is_empty() {
                return None;
            }
            if let Some(&idx) = loop_index_by_id.get(loop_id) {
                return Some(idx);
            }
            let brep_loop = snapshot.loops.iter().find(|l| l.id == loop_id)?;
            let ring: Vec<(PersistentLabel, bool)> = brep_loop.edges.iter().map(|le| (resolver.resolve(&le.edge), le.orientation)).collect();
            let idx = seed_loops.len();
            seed_loops.push(ring);
            loop_index_by_id.insert(loop_id.to_string(), idx);
            Some(idx)
        };

        let mut seed_faces: Vec<SeedFace> = Vec::with_capacity(snapshot.faces.len());
        // 🧱️ `(face snapshot id, outer BrepLoop id, inner BrepLoop ids)` — retained so the p-curve
        // second pass (after `Body::from_seed`) can re-walk each face's rings in the SAME order
        // `from_seed` assigned them (outer first, then inners), matching them back to this
        // snapshot's own `BrepLoop.id`s to look up `coedges` by `loopId`.
        let mut face_loop_ids: Vec<(String, Vec<String>)> = Vec::with_capacity(snapshot.faces.len());
        for f in &snapshot.faces {
            let outer = resolve_loop(&f.outer_loop, &mut resolver);
            let inners: Vec<usize> = f.inner_loops.iter().filter_map(|lid| resolve_loop(lid, &mut resolver)).collect();
            seed_faces.push(SeedFace { label: resolver.resolve(&f.id), surface: brep_surface_to_native(&f.surface), outer, inners, flipped: !f.orientation, tol: Tol::new(if f.tol > 0.0 { f.tol } else { Tol::DEFAULT.0 }) });
            let mut ring_ids = Vec::with_capacity(1 + f.inner_loops.len());
            if !f.outer_loop.is_empty() {
                ring_ids.push(f.outer_loop.clone());
            }
            ring_ids.extend(f.inner_loops.iter().cloned());
            face_loop_ids.push((f.id.clone(), ring_ids));
        }

        let seed_shells: Vec<SeedShell> = snapshot.shells.iter().map(|s| SeedShell { label: resolver.resolve(&s.id), faces: s.faces.iter().map(|sf| resolver.resolve(&sf.face)).collect() }).collect();

        let mut seed_solids: Vec<SeedSolid> = Vec::with_capacity(snapshot.solids.len());
        for s in &snapshot.solids {
            let outer = s.shells.iter().find(|ss| !ss.is_void).map(|ss| resolver.resolve(&ss.shell)).ok_or_else(|| KernelError::InvalidInput(format!("solid {:?} has no outer (non-void) shell", s.id)))?;
            let inners = s.shells.iter().filter(|ss| ss.is_void).map(|ss| resolver.resolve(&ss.shell)).collect();
            seed_solids.push(SeedSolid { label: resolver.resolve(&s.id), outer, inners });
        }

        let seed = BrepArenaSeed { next_label: resolver.next_fresh.max(high_water), vertices: seed_vertices, edges: seed_edges, loops: seed_loops, faces: seed_faces, shells: seed_shells, solids: seed_solids };
        let mut body = Body::from_seed(&seed);

        // 🧱️ Second pass: attach p-curves. Rebuild the label→FaceId map from the just-built body
        // (labels are exactly what `resolver` minted above, so this is a direct lookup) and, for
        // each face, zip its native ring order (`face_loops`/`loop_coedges`, which `from_seed`
        // built in EXACTLY the `seed_faces`/`seed_loops` order above) against this snapshot's own
        // `coedges` (filtered by `loopId`, same ring order `to_snapshot` emitted them in).
        if !snapshot.coedges.is_empty() {
            let face_id_by_label: HashMap<PersistentLabel, FaceId> = body.faces.iter().map(|(id, face)| (face.label, id)).collect();
            for (face_snapshot_id, ring_ids) in &face_loop_ids {
                let Some(f) = snapshot.faces.iter().find(|f| &f.id == face_snapshot_id) else { continue };
                let face_label = resolver.resolve(&f.id);
                let Some(&face_id) = face_id_by_label.get(&face_label) else { continue };
                let native_loop_ids = body.face_loops(face_id);
                for (native_loop_id, brep_loop_id) in native_loop_ids.iter().zip(ring_ids.iter()) {
                    let native_coedge_ids = body.loop_coedges(*native_loop_id);
                    let brep_coedges: Vec<&BrepCoedge> = snapshot.coedges.iter().filter(|c| &c.loop_id == brep_loop_id).collect();
                    for (native_cid, brep_coedge) in native_coedge_ids.iter().zip(brep_coedges.iter()) {
                        let Some(pcurve) = &brep_coedge.pcurve else { continue };
                        let curve2_id = body.curves2.insert(brep_curve2_to_native(pcurve));
                        if let Some(coedge) = body.coedges.get_mut(*native_cid) {
                            coedge.pcurve = Some(curve2_id);
                            coedge.prange = brep_coedge.prange;
                        }
                    }
                }
            }
        }

        Ok(body)
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere, make_torus};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::EntityCounts;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn counts(body: &Body) -> EntityCounts {
        EntityCounts {
            vertices: body.vertices.len(),
            edges: body.edges.len(),
            coedges: body.coedges.len(),
            loops: body.loops.len(),
            faces: body.faces.len(),
            shells: body.shells.len(),
            solids: body.solids.len(),
            curves3: body.curves3.len(),
            curves2: body.curves2.len(),
            surfaces: body.surfaces.len(),
        }
    }

    /// 🔁️ Law: for every required primitive, `body.to_snapshot()` then `Body::from_snapshot()`
    /// then `.to_snapshot()` again produces the IDENTICAL snapshot (labels, geometry, topology) —
    /// the round-trip stabilizes after one hop, proving no information is lost on the way through
    /// `Body` and back (ticket goal: "snapshot → body → snapshot is identical").
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_round_trips(body: Body) {
        let snap1 = body.to_snapshot();
        let rebuilt = Body::from_snapshot(&snap1).expect("from_snapshot");
        let snap2 = rebuilt.to_snapshot();
        assert_eq!(snap1, snap2, "snapshot -> body -> snapshot must be identical");
        assert_eq!(counts(&body), counts(&rebuilt), "entity counts must match after round trip");
        let issues = crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body(&rebuilt);
        assert!(issues.is_empty(), "rebuilt body must validate cleanly: {issues:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn box_round_trips_through_snapshot() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
        assert_round_trips(body);
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_round_trips_through_snapshot() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        make_sphere(&mut body, 2.5, &mut rec).unwrap();
        assert_round_trips(body);
    }

    #[semio_framework_async_macros::async_test]
    async fn cylinder_round_trips_through_snapshot() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        make_cylinder(&mut body, 1.5, 4.0, &mut rec).unwrap();
        assert_round_trips(body);
    }

    #[semio_framework_async_macros::async_test]
    async fn torus_round_trips_through_snapshot() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        make_torus(&mut body, 3.0, 1.0, &mut rec).unwrap();
        assert_round_trips(body);
    }

    /// 🏷️ Persistent labels survive the round trip byte-for-byte (as decimal ids) — the property
    /// two independent mutation constructions against the same document depend on.
    #[semio_framework_async_macros::async_test]
    async fn labels_are_preserved_as_decimal_ids() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let snap = body.to_snapshot();
        for v in &snap.vertices {
            assert!(v.id.parse::<u64>().is_ok(), "vertex id {:?} must be a bare decimal label", v.id);
        }
        assert!(snap.next_label > 0);
        let rebuilt = Body::from_snapshot(&snap).unwrap();
        assert_eq!(rebuilt.labels.next(), snap.next_label);
    }

    /// 📐️ Non-numeric (STEP-import-shaped) ids mint fresh labels rather than erroring, and the
    /// resulting body is still internally consistent (every reference resolves).
    #[semio_framework_async_macros::async_test]
    async fn foreign_string_ids_mint_fresh_labels() {
        let mut snap = SemioBrepSnapshot::default();
        snap.vertices = vec![
            BrepVertex { id: "🐼️v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 0.0 },
            BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, tol: 0.0 },
            BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }, tol: 0.0 },
        ];
        snap.edges = vec![
            BrepEdge { id: "e1".into(), start_vertex: "🐼️v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: snap.vertices[0].point, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }, tol: 0.0 },
            BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Line { origin: snap.vertices[1].point, direction: SemioPoint3 { x: -1.0, y: 1.0, z: 0.0 } }, tol: 0.0 },
            BrepEdge { id: "e3".into(), start_vertex: "v3".into(), end_vertex: "🐼️v1".into(), curve: BrepCurve::Line { origin: snap.vertices[2].point, direction: SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 } }, tol: 0.0 },
        ];
        snap.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }] }];
        snap.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true, tol: 0.0 }];
        snap.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
        snap.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];

        let body = Body::from_snapshot(&snap).expect("from_snapshot on foreign ids");
        assert_eq!(body.vertices.len(), 3);
        assert_eq!(body.faces.len(), 1);
        let issues = crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body(&body);
        assert!(issues.is_empty(), "{issues:?}");
    }

    /// 📈️ `Curve3::Nurbs`/`Surface::Nurbs` knot vectors round-trip exactly through `BrepCurve::
    /// Nurbs`/`BrepSurface::Nurbs` — the `KnotVector { knots, degree }` shape is already isomorphic
    /// to `(knots: Vec<f64>, degree: u32)`, so this is a direct field-for-field check, not an
    /// approximation.
    #[semio_framework_async_macros::async_test]
    async fn nurbs_knots_round_trip_exactly() {
        let native = Curve3::Nurbs { knots: KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2, 5).unwrap(), controls: vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(3.0, 1.0, 0.0), Pnt3::new(4.0, 0.0, 0.0)], weights: vec![1.0, 0.8, 1.0, 0.8, 1.0] };
        let brep = native_curve_to_brep(&native);
        let back = brep_curve_to_native(&brep);
        match (&native, &back) {
            (Curve3::Nurbs { knots: k1, controls: c1, weights: w1 }, Curve3::Nurbs { knots: k2, controls: c2, weights: w2 }) => {
                assert_eq!(k1.knots, k2.knots);
                assert_eq!(k1.degree, k2.degree);
                assert_eq!(c1, c2);
                assert_eq!(w1, w2);
            }
            _ => panic!("expected Nurbs on both sides"),
        }
    }
}
//#endregion 🧪️Tests
