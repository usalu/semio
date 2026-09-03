//! 🔁 Exact affine transformation of B-Rep topology: `transform_solid`/`transform_face`/
//! `transform_wire` deep-copy the reachable topology graph into fresh entities (new
//! [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]s, recorded generated), transforming every
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

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::Wire;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, SurfaceId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
#[cfg(test)]
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
#[cfg(test)]
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

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
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpDelta`]), with every geometric support transformed by `map`
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
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Face`] handle.
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

    #[semio_framework_async_macros::async_test]
    async fn transform_solid_preserves_face_and_edge_counts_for_a_box() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let box_id = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
        let faces_before = body.solid_faces(box_id).len();
        let edges_before: std::collections::HashSet<EdgeId> = body.solid_faces(box_id).into_iter().flat_map(|f| body.face_coedges(f)).filter_map(|c| body.coedges.get(c).map(|co| co.edge)).collect();
        let map = Affine3::rotation_about(Pnt3::new(1.0, 0.0, 0.0), Vec3::new(0.2, 1.0, 0.3), 0.7).compose(&Affine3::translation(Vec3::new(3.0, -1.0, 2.0)));
        let transformed = transform_solid(&mut body, box_id, &map, &mut rec).unwrap();
        let faces_after = body.solid_faces(transformed).len();
        let edges_after: std::collections::HashSet<EdgeId> = body.solid_faces(transformed).into_iter().flat_map(|f| body.face_coedges(f)).filter_map(|c| body.coedges.get(c).map(|co| co.edge)).collect();
        assert_eq!(faces_before, faces_after);
        assert_eq!(edges_before.len(), edges_after.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn transform_solid_keeps_analytic_surface_kinds_for_a_cylinder_under_similarity() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let cyl = make_cylinder(&mut body, 1.5, 4.0, &mut rec).unwrap();
        let map = Affine3::rotation_about(Pnt3::new(0.0, 0.0, 0.0), Vec3::Y, 0.5).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0)));
        let transformed = transform_solid(&mut body, cyl, &map, &mut rec).unwrap();
        for f in body.solid_faces(transformed) {
            let surface_id = body.faces.get(f).unwrap().surface;
            let surface = body.surfaces.get(surface_id).unwrap();
            assert!(matches!(surface, Surface::Cylinder { .. } | Surface::Plane { .. }), "expected an analytic surface kind, got {surface:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn transform_solid_scales_volume_by_the_determinant_magnitude() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let sphere = make_sphere(&mut body, 2.0, &mut rec).unwrap();
        let volume_before = solid_volume(&body, sphere, 1e-4).unwrap();
        let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 3.0));
        let transformed = transform_solid(&mut body, sphere, &map, &mut rec).unwrap();
        let volume_after = solid_volume(&body, transformed, 1e-4).unwrap();
        let expected = volume_before * map.determinant().abs();
        assert!((volume_after - expected).abs() / expected < 1e-3, "volume {volume_after} vs expected {expected}");
    }

    #[semio_framework_async_macros::async_test]
    async fn transform_solid_rotate_then_inverse_rotate_round_trips_vertex_positions() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let box_id = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let original_positions: Vec<Pnt3> = body.solid_faces(box_id).into_iter().flat_map(|f| body.face_coedges(f)).filter_map(|c| body.coedges.get(c).map(|co| co.edge)).filter_map(|e| body.edges.get(e).map(|edge| edge.v0)).filter_map(|v| body.vertices.get(v).map(|vertex| vertex.position)).collect();
        let map = Affine3::rotation_about(Pnt3::new(0.5, 0.5, 0.5), Vec3::new(1.0, 2.0, 3.0), 1.234);
        let inverse = map.inverse().unwrap();
        let rotated = transform_solid(&mut body, box_id, &map, &mut rec).unwrap();
        let back = transform_solid(&mut body, rotated, &inverse, &mut rec).unwrap();
        let back_positions: Vec<Pnt3> = body.solid_faces(back).into_iter().flat_map(|f| body.face_coedges(f)).filter_map(|c| body.coedges.get(c).map(|co| co.edge)).filter_map(|e| body.edges.get(e).map(|edge| edge.v0)).filter_map(|v| body.vertices.get(v).map(|vertex| vertex.position)).collect();
        assert_eq!(original_positions.len(), back_positions.len());
        for p in &back_positions {
            let closest = original_positions.iter().map(|o| o.distance(*p)).fold(f64::INFINITY, f64::min);
            assert!(closest < 1e-9, "round-tripped vertex {p:?} did not land within 1e-9 of an original vertex");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_solid_is_identical_geometry_at_a_fresh_identity() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let box_id = make_box(&mut body, 2.0, 1.0, 1.0, &mut rec).unwrap();
        let copy = copy_solid(&mut body, box_id, &mut rec).unwrap();
        assert_ne!(box_id, copy);
        let volume_original = solid_volume(&body, box_id, 1e-4).unwrap();
        let volume_copy = solid_volume(&body, copy, 1e-4).unwrap();
        assert!((volume_original - volume_copy).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn transform_solid_generated_delta_covers_every_new_face() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let box_id = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let mut transform_rec = OpRecorder::new();
        let transformed = transform_solid(&mut body, box_id, &Affine3::translation(Vec3::new(1.0, 0.0, 0.0)), &mut transform_rec).unwrap();
        let delta = transform_rec.into_delta();
        let new_labels: std::collections::HashSet<PersistentLabel> = body.solid_faces(transformed).into_iter().filter_map(|f| body.faces.get(f).map(|face| face.label)).collect();
        for label in new_labels {
            assert!(delta.generated.contains(&label), "face label {label:?} missing from generated delta");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn transform_solid_in_place_records_modified_not_generated() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let box_id = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let mut transform_rec = OpRecorder::new();
        transform_solid_in_place(&mut body, box_id, &Affine3::translation(Vec3::new(2.0, 0.0, 0.0)), &mut transform_rec).unwrap();
        let delta = transform_rec.into_delta();
        assert!(delta.generated.is_empty(), "in-place transform must never generate new entities");
        assert!(!delta.modified.is_empty());
    }
}
// #endregion 🔖️Tests
