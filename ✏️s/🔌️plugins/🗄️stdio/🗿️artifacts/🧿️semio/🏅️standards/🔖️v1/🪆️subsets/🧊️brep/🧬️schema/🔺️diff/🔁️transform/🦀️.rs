//! 🔁 Exact affine transformation of B-Rep topology: `transform_solid`/`transform_face`/
//! `transform_wire` deep-copy the reachable topology graph into fresh entities (new
//! [`crate::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]s, recorded generated), transforming every
//! geometric support ([`Curve3`]/[`Surface`]) via [`Affine3`] and leaving every p-curve
//! ([`Curve2`]) byte-for-byte unchanged (it lives in the face's own parameter space, which the
//! same map leaves invariant — see `Surface::transformed`'s own docstring). `transform_solid_in_place`
//! is the same map applied destructively to the existing entities instead, recording them modified.
//! `copy_solid` is `transform_solid` with [`Affine3::IDENTITY`] — a duplicate with a fresh identity.
//!
//! Lane W1-B of ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME, replacing the former
//! tessellate→translate-mesh-vertices→`solid_from_triangle_soup` round trip (audit §6.2) with an
//! exact topology-preserving transform: face count, edge count, and analytic surface kind are all
//! preserved (or, under a non-similarity map, converted once to the equivalent exact NURBS by
//! `Curve3`/`Surface::transformed` themselves — never by resampling this module's own topology
//! walk).

use std::collections::HashMap;

use crate::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::standards::v1::subsets::brep::schema::diff::primitives::Wire;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, SurfaceId, VertexId};
use crate::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
#[cfg(test)]
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
#[cfg(test)]
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

// #region 🔖️Validate

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_some() {
        Ok(())
    } else {
        Err(KernelError::MissingEntity(format!("solid {solid}")))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_face(body: &Body, face: FaceId) -> Result<(), KernelError> {
    if body.faces.get(face).is_some() {
        Ok(())
    } else {
        Err(KernelError::MissingEntity(format!("face {face}")))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

// #endregion 🔖️Validate

// #region 🔖️DeepCopy

/// 🔁 Every id-remap cache a deep copy walk threads through, so shared geometry (two edges built
/// from the same [`Curve3Id`], two faces sharing a [`Curve2Id`] — not possible today but harmless
/// to support) is transformed exactly once and every reference to it repoints consistently.
#[derive(Default)]
struct CopyCtx {
    vertices: HashMap<VertexId, VertexId>,
    curves3: HashMap<Curve3Id, Curve3Id>,
    curves2: HashMap<Curve2Id, Curve2Id>,
    surfaces: HashMap<SurfaceId, SurfaceId>,
    edges: HashMap<EdgeId, EdgeId>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_curve3(body: &mut Body, ctx: &mut CopyCtx, old: Curve3Id, map: &Affine3) -> Curve3Id {
    if let Some(&id) = ctx.curves3.get(&old) {
        return id;
    }
    let curve = body.curves3.get(old).expect("live curve3").clone();
    let id = body.curves3.insert(curve.transformed(map));
    ctx.curves3.insert(old, id);
    id
}

/// 🔁 A p-curve's geometry stays byte-for-byte unchanged (it lives in the face's own `(u, v)`
/// parameter space, invariant under the surface's own affine transform) — only its arena slot is
/// fresh, since this walk is building an entirely new topology graph.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_curve2(body: &mut Body, ctx: &mut CopyCtx, old: Curve2Id) -> Curve2Id {
    if let Some(&id) = ctx.curves2.get(&old) {
        return id;
    }
    let curve = body.curves2.get(old).expect("live curve2").clone();
    let id = body.curves2.insert(curve);
    ctx.curves2.insert(old, id);
    id
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_surface(body: &mut Body, ctx: &mut CopyCtx, old: SurfaceId, map: &Affine3) -> SurfaceId {
    if let Some(&id) = ctx.surfaces.get(&old) {
        return id;
    }
    let surface = body.surfaces.get(old).expect("live surface").clone();
    let id = body.surfaces.insert(surface.transformed(map));
    ctx.surfaces.insert(old, id);
    id
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_vertex(body: &mut Body, ctx: &mut CopyCtx, old: VertexId, map: &Affine3, tol_scale: f64, rec: &mut OpRecorder) -> VertexId {
    if let Some(&id) = ctx.vertices.get(&old) {
        return id;
    }
    let v = body.vertices.get(old).expect("live vertex").clone();
    let id = make_vertex(body, map.apply_point(v.position), v.tol.scaled(tol_scale), rec);
    ctx.vertices.insert(old, id);
    id
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_edge(body: &mut Body, ctx: &mut CopyCtx, old: EdgeId, map: &Affine3, tol_scale: f64, rec: &mut OpRecorder) -> EdgeId {
    if let Some(&id) = ctx.edges.get(&old) {
        return id;
    }
    let e = body.edges.get(old).expect("live edge").clone();
    let curve = copy_curve3(body, ctx, e.curve, map);
    let v0 = copy_vertex(body, ctx, e.v0, map, tol_scale, rec);
    let v1 = copy_vertex(body, ctx, e.v1, map, tol_scale, rec);
    let id = make_edge(body, curve, e.range, v0, v1, e.tol.scaled(tol_scale), rec);
    ctx.edges.insert(old, id);
    id
}

/// 🔁 Deep-copies one loop's coedge ring, transforming its edges/p-curves via [`copy_edge`]/
/// [`copy_curve2`] and preserving each coedge's own `forward`/`prange`. [`make_loop`] doesn't
/// accept p-curve data, so this builds the ring first and patches `pcurve`/`prange` back onto the
/// coedges it returns — safe because `make_loop` inserts coedges in `members` order, and
/// [`Body::loop_coedges`] walks the ring starting from that same first coedge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_loop(body: &mut Body, ctx: &mut CopyCtx, old_loop: LoopId, map: &Affine3, tol_scale: f64, rec: &mut OpRecorder) -> LoopId {
    let old_coedges = body.loop_coedges(old_loop);
    let mut members = Vec::with_capacity(old_coedges.len());
    let mut pcurves = Vec::with_capacity(old_coedges.len());
    for &cid in &old_coedges {
        let c = body.coedges.get(cid).expect("live coedge").clone();
        let edge = copy_edge(body, ctx, c.edge, map, tol_scale, rec);
        members.push((edge, c.forward));
        pcurves.push((c.pcurve.map(|pc| copy_curve2(body, ctx, pc)), c.prange));
    }
    let new_loop = make_loop(body, placeholder_face(), &members);
    let new_coedges = body.loop_coedges(new_loop);
    for (&cid, (pcurve, prange)) in new_coedges.iter().zip(pcurves) {
        if let Some(coedge) = body.coedges.get_mut(cid) {
            coedge.pcurve = pcurve;
            coedge.prange = prange;
        }
    }
    new_loop
}

/// 🔁 Deep-copies one face: surface, outer/inner loops, and (per this module's header) flips
/// `flipped` under a reflection (`det(map) < 0`) — a reflected surface's `x × y` handedness flips
/// (see `Surface::transformed`/`Frame3::transformed`), so keeping the SAME outward-normal side of
/// the solid requires this compensating flip at the topology level.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_face(body: &mut Body, ctx: &mut CopyCtx, old_face: FaceId, map: &Affine3, tol_scale: f64, rec: &mut OpRecorder) -> FaceId {
    let f = body.faces.get(old_face).expect("live face").clone();
    let surface = copy_surface(body, ctx, f.surface, map);
    let outer = f.outer.map(|l| copy_loop(body, ctx, l, map, tol_scale, rec));
    let inners: Vec<LoopId> = f.inners.iter().map(|&l| copy_loop(body, ctx, l, map, tol_scale, rec)).collect();
    let flipped = if map.determinant() < 0.0 { !f.flipped } else { f.flipped };
    let new_face = add_face(body, surface, outer, inners.clone(), flipped, f.tol.scaled(tol_scale), rec);
    if let Some(o) = outer {
        body.loops.get_mut(o).expect("just inserted").face = new_face;
    }
    for l in inners {
        body.loops.get_mut(l).expect("just inserted").face = new_face;
    }
    new_face
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_shell(body: &mut Body, ctx: &mut CopyCtx, old_shell: ShellId, map: &Affine3, tol_scale: f64, rec: &mut OpRecorder) -> ShellId {
    let s = body.shells.get(old_shell).expect("live shell").clone();
    let faces: Vec<FaceId> = s.faces.iter().map(|&f| copy_face(body, ctx, f, map, tol_scale, rec)).collect();
    add_shell(body, faces, rec)
}

// #endregion 🔖️DeepCopy

// #region 🔖️Api

/// 🔁 Produces a NEW solid: every reachable vertex/edge/coedge/p-curve/loop/face/shell is deep
/// copied into fresh entities (fresh [`PersistentLabel`]s, all recorded generated in `rec`'s
/// [`crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpDelta`]), with every geometric support transformed by `map`
/// and every tolerance scaled by `map.max_singular_value()`. Face count and edge count are exactly
/// preserved; each face's surface stays its own analytic kind under a similarity `map`, else
/// converts once to the exact equivalent NURBS ([`Surface::transformed`]). The original solid is
/// untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn transform_solid(body: &mut Body, solid: SolidId, map: &Affine3, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_solid(body, solid)?;
    let tol_scale = map.max_singular_value();
    let mut ctx = CopyCtx::default();
    let s = body.solids.get(solid).expect("live solid").clone();
    let outer = copy_shell(body, &mut ctx, s.outer, map, tol_scale, rec);
    let inners: Vec<ShellId> = s.inners.iter().map(|&sh| copy_shell(body, &mut ctx, sh, map, tol_scale, rec)).collect();
    Ok(add_solid(body, outer, inners, rec))
}

/// 🔁 A duplicate of `solid` with independent identity — [`transform_solid`] under
/// [`Affine3::IDENTITY`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn copy_solid(body: &mut Body, solid: SolidId, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    transform_solid(body, solid, &Affine3::IDENTITY, rec)
}

/// 🔁 Produces a NEW, detached face (not attached to any shell/solid) — the same deep-copy-and-
/// transform as one face inside [`transform_solid`], usable standalone for a bare
/// [`crate::standards::v1::subsets::brep::schema::snapshot::topology::Face`] handle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn transform_face(body: &mut Body, face: FaceId, map: &Affine3, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    require_face(body, face)?;
    let tol_scale = map.max_singular_value();
    let mut ctx = CopyCtx::default();
    Ok(copy_face(body, &mut ctx, face, map, tol_scale, rec))
}

/// 🔁 Produces a NEW [`Wire`]: every member edge (and its endpoint vertices) deep copied and
/// transformed, preserving orientation and open/closed-ness. Wires are ephemeral (engine-level,
/// not yet bound to a face), so unlike [`transform_face`] there is no p-curve to carry.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn transform_wire(body: &mut Body, wire: &Wire, map: &Affine3, rec: &mut OpRecorder) -> Wire {
    let tol_scale = map.max_singular_value();
    let mut ctx = CopyCtx::default();
    let members: Vec<(EdgeId, bool)> = wire.members.iter().map(|&(e, forward)| (copy_edge(body, &mut ctx, e, map, tol_scale, rec), forward)).collect();
    let vertices: Vec<VertexId> = wire.vertices.iter().map(|&v| copy_vertex(body, &mut ctx, v, map, tol_scale, rec)).collect();
    Wire { members, vertices, closed: wire.closed }
}

/// 🔁 The same affine transform as [`transform_solid`], applied DESTRUCTIVELY to `solid`'s
/// existing vertices/curves/surfaces/tolerances instead of copying — every touched entity is
/// recorded modified (never generated) in `rec`. Topology (which vertex/edge/face/shell an id
/// refers to) is unchanged; only geometry and tolerance move.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn transform_solid_in_place(body: &mut Body, solid: SolidId, map: &Affine3, rec: &mut OpRecorder) -> Result<(), KernelError> {
    require_solid(body, solid)?;
    let tol_scale = map.max_singular_value();
    let reflecting = map.determinant() < 0.0;
    let solid_faces = body.solid_faces(solid);
    let mut seen_vertices = std::collections::HashSet::new();
    let mut seen_curves3 = std::collections::HashSet::new();
    let mut seen_surfaces = std::collections::HashSet::new();
    for face_id in solid_faces {
        let (surface_id, label) = {
            let f = body.faces.get(face_id).expect("live face");
            (f.surface, f.label)
        };
        if seen_surfaces.insert(surface_id) {
            let surface = body.surfaces.get(surface_id).expect("live surface").clone();
            *body.surfaces.get_mut(surface_id).expect("live surface") = surface.transformed(map);
        }
        if reflecting {
            let f = body.faces.get_mut(face_id).expect("live face");
            f.flipped = !f.flipped;
        }
        {
            let f = body.faces.get_mut(face_id).expect("live face");
            f.tol = f.tol.scaled(tol_scale);
        }
        rec.record_modified(label);
        for coedge_id in body.face_coedges(face_id) {
            let edge_id = body.coedges.get(coedge_id).expect("live coedge").edge;
            let e = body.edges.get(edge_id).expect("live edge").clone();
            if seen_curves3.insert(e.curve) {
                let curve = body.curves3.get(e.curve).expect("live curve3").clone();
                *body.curves3.get_mut(e.curve).expect("live curve3") = curve.transformed(map);
            }
            {
                let edge = body.edges.get_mut(edge_id).expect("live edge");
                edge.tol = edge.tol.scaled(tol_scale);
            }
            rec.record_modified(e.label);
            for &vertex_id in &[e.v0, e.v1] {
                if seen_vertices.insert(vertex_id) {
                    let (position, label, tol) = {
                        let v = body.vertices.get(vertex_id).expect("live vertex");
                        (v.position, v.label, v.tol)
                    };
                    let v = body.vertices.get_mut(vertex_id).expect("live vertex");
                    v.position = map.apply_point(position);
                    v.tol = tol.scaled(tol_scale);
                    rec.record_modified(label);
                }
            }
        }
    }
    Ok(())
}

// #endregion 🔖️Api

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
