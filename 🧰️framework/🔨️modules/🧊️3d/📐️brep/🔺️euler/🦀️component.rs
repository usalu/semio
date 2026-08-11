//! ✂️ Checked topology editors — the *only* functions permitted to mutate a [`crate::brep::topo::Body`].
//! Each takes an `&mut OpRecorder` so no operation can forget to log what it created/modified/
//! deleted; assembling a body exclusively through these (never by poking a `Store` directly) is
//! what keeps "public shapes cannot exist in a partially invalid state" true by construction.

use crate::brep::arena::{ArenaId, CoedgeId, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, SurfaceId, VertexId};
use crate::brep::history::OpRecorder;
use crate::brep::tolerance::Tol;
use crate::brep::topo::{Body, Coedge, Edge, Face, Loop, Shell, Solid, Vertex};
use crate::brep::vec::Pnt3;

// #region 🔖️Make

fn dummy_coedge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}

/// ✂️ Creates a new vertex, recording it as generated.
pub fn make_vertex(body: &mut Body, position: Pnt3, tol: Tol, rec: &mut OpRecorder) -> VertexId {
    let label = body.new_label();
    rec.record_generated(label);
    body.vertices.insert(Vertex { position, tol, label })
}

/// ✂️ Creates a new edge referencing shared curve geometry, recording it as generated.
pub fn make_edge(body: &mut Body, curve: Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    let label = body.new_label();
    rec.record_generated(label);
    body.edges.insert(Edge { curve, range, v0, v1, tol, label })
}

/// ✂️ Builds a closed coedge ring from `members` (one `(edge, forward)` pair per coedge, in ring
/// order) and links it into a new [`Loop`]. Loops/coedges have no [`crate::brep::history::PersistentLabel`]
/// of their own (they are structural, not independently document-nameable), so nothing is recorded.
pub fn make_loop(body: &mut Body, face: FaceId, members: &[(EdgeId, bool)]) -> LoopId {
    let loop_id = body.loops.insert(Loop { first: dummy_coedge(), face });
    let coedge_ids: Vec<CoedgeId> = members.iter().map(|&(edge, forward)| body.coedges.insert(Coedge { edge, forward, pcurve: None, prange: (0.0, 0.0), loop_id, next: dummy_coedge(), prev: dummy_coedge() })).collect();
    let n = coedge_ids.len();
    for i in 0..n {
        let coedge = body.coedges.get_mut(coedge_ids[i]).unwrap();
        coedge.next = coedge_ids[(i + 1) % n];
        coedge.prev = coedge_ids[(i + n - 1) % n];
    }
    body.loops.get_mut(loop_id).unwrap().first = coedge_ids[0];
    loop_id
}

/// ✂️ Creates a new face, recording it as generated.
pub fn add_face(body: &mut Body, surface: SurfaceId, outer: Option<LoopId>, inners: Vec<LoopId>, flipped: bool, tol: Tol, rec: &mut OpRecorder) -> FaceId {
    let label = body.new_label();
    rec.record_generated(label);
    body.faces.insert(Face { surface, outer, inners, flipped, tol, label })
}

/// ✂️ Creates a new shell, recording it as generated.
pub fn add_shell(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> ShellId {
    let label = body.new_label();
    rec.record_generated(label);
    body.shells.insert(Shell { faces, label })
}

/// ✂️ Creates a new solid, recording it as generated.
pub fn add_solid(body: &mut Body, outer: ShellId, inners: Vec<ShellId>, rec: &mut OpRecorder) -> SolidId {
    let label = body.new_label();
    rec.record_generated(label);
    body.solids.insert(Solid { outer, inners, label })
}

// #endregion 🔖️Make

// #region 🔖️SplitJoin

/// ✂️ Splits `edge_id` at curve parameter `t` (which must lie strictly within the edge's current
/// range) into two edges sharing the same underlying curve, joined by a new vertex at `position`.
/// Every coedge that used the old edge is replaced by two coedges spliced into the same ring in
/// the correct order — including the degenerate case of a single self-referential coedge (a full
/// periodic edge, e.g. a closed circle, forming a one-coedge loop). Returns
/// `(first_half, second_half, new_vertex)`, where "first"/"second" are relative to the edge's own
/// `v0 → v1` direction (not any particular coedge's orientation).
pub fn split_edge(body: &mut Body, edge_id: EdgeId, t: f64, position: Pnt3, rec: &mut OpRecorder) -> (EdgeId, EdgeId, VertexId) {
    let old_edge = body.edges.get(edge_id).expect("split_edge requires a live edge id").clone();
    debug_assert!(t > old_edge.range.0 && t < old_edge.range.1, "split parameter must lie strictly within the edge's range");
    let new_vertex = make_vertex(body, position, old_edge.tol, rec);
    let e1 = make_edge(body, old_edge.curve, (old_edge.range.0, t), old_edge.v0, new_vertex, old_edge.tol, rec);
    let e2 = make_edge(body, old_edge.curve, (t, old_edge.range.1), new_vertex, old_edge.v1, old_edge.tol, rec);
    let affected: Vec<CoedgeId> = body.edge_coedges(edge_id);
    for coedge_id in affected {
        let coedge = body.coedges.get(coedge_id).unwrap().clone();
        let (first_edge, second_edge) = if coedge.forward { (e1, e2) } else { (e2, e1) };
        let self_loop = coedge.prev == coedge_id && coedge.next == coedge_id;
        let c1 = body.coedges.insert(Coedge { edge: first_edge, forward: coedge.forward, pcurve: None, prange: (0.0, 0.0), loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        let c2 = body.coedges.insert(Coedge { edge: second_edge, forward: coedge.forward, pcurve: None, prange: (0.0, 0.0), loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        if self_loop {
            body.coedges.get_mut(c1).unwrap().prev = c2;
            body.coedges.get_mut(c1).unwrap().next = c2;
            body.coedges.get_mut(c2).unwrap().prev = c1;
            body.coedges.get_mut(c2).unwrap().next = c1;
        } else {
            let prev_id = coedge.prev;
            let next_id = coedge.next;
            body.coedges.get_mut(c1).unwrap().prev = prev_id;
            body.coedges.get_mut(c1).unwrap().next = c2;
            body.coedges.get_mut(c2).unwrap().prev = c1;
            body.coedges.get_mut(c2).unwrap().next = next_id;
            body.coedges.get_mut(prev_id).unwrap().next = c1;
            body.coedges.get_mut(next_id).unwrap().prev = c2;
        }
        if let Some(lp) = body.loops.get_mut(coedge.loop_id) {
            if lp.first == coedge_id {
                lp.first = c1;
            }
        }
        body.coedges.remove(coedge_id);
    }
    body.edges.remove(edge_id);
    rec.record_deleted(old_edge.label);
    (e1, e2, new_vertex)
}

// #endregion 🔖️SplitJoin

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::curve::Curve3;
    use crate::brep::mat::Frame3;
    use crate::brep::surface::Surface;
    use crate::brep::vec::Vec3;

    /// ✂️ Builds the topology of a unit tetrahedron (4 vertices, 6 edges, 4 triangular faces, 1
    /// shell, 1 solid) purely through the checked editors above — the flagship "assemble a real
    /// closed solid from scratch" gate for this phase.
    fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
        let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
        let vertices: Vec<VertexId> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
        let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
        let mut edges = std::collections::HashMap::new();
        for &(a, b) in &edge_pairs {
            let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
            let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
            edges.insert((a, b), edge);
            edges.insert((b, a), edge);
        }
        // Four triangular faces of a tetrahedron with vertex indices 0,1,2,3.
        let face_defs = [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]];
        let mut faces = Vec::new();
        for tri in face_defs {
            let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
            let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
            let surface = body.surfaces.insert(Surface::Plane { frame });
            let members: Vec<(EdgeId, bool)> = (0..3)
                .map(|i| {
                    let a = tri[i];
                    let b = tri[(i + 1) % 3];
                    let edge = edges[&(a, b)];
                    let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                    (edge, forward)
                })
                .collect();
            let outer = make_loop(body, FaceId::from_raw(0, 0), &members);
            let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
            body.loops.get_mut(outer).unwrap().face = face;
            faces.push(face);
        }
        let shell = add_shell(body, faces, rec);
        add_solid(body, shell, vec![], rec)
    }

    #[test]
    fn tetrahedron_satisfies_euler_poincare_formula() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let vertex_count = body.vertices.len() as i64;
        let edge_count = body.edges.len() as i64;
        let face_count = body.solid_faces(solid).len() as i64;
        assert_eq!(vertex_count, 4);
        assert_eq!(edge_count, 6);
        assert_eq!(face_count, 4);
        assert_eq!(vertex_count - edge_count + face_count, 2, "V - E + F must equal 2 for a genus-0 closed solid");
    }

    #[test]
    fn tetrahedron_build_records_every_entity_as_generated() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        build_tetrahedron(&mut body, &mut rec);
        let delta = rec.into_delta();
        assert_eq!(delta.generated.len(), 4 + 6 + 4 + 1 + 1, "vertices + edges + faces + shell + solid");
        assert!(delta.deleted.is_empty());
    }

    #[test]
    fn each_face_loop_is_a_closed_ring_of_three_coedges() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        for face in body.solid_faces(solid) {
            let outer = body.faces.get(face).unwrap().outer.unwrap();
            assert_eq!(body.loop_coedges(outer).len(), 3);
        }
    }

    #[test]
    fn split_edge_on_a_free_edge_creates_two_edges_and_a_vertex() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let v1 = make_vertex(&mut body, Pnt3::new(4.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = make_edge(&mut body, curve, (0.0, 4.0), v0, v1, Tol::DEFAULT, &mut rec);
        let (e1, e2, new_vertex) = split_edge(&mut body, edge, 1.5, Pnt3::new(1.5, 0.0, 0.0), &mut rec);
        assert!(!body.edges.contains(edge));
        assert_eq!(body.edges.get(e1).unwrap().v0, v0);
        assert_eq!(body.edges.get(e1).unwrap().v1, new_vertex);
        assert_eq!(body.edges.get(e2).unwrap().v0, new_vertex);
        assert_eq!(body.edges.get(e2).unwrap().v1, v1);
        assert_eq!(body.edges.get(e1).unwrap().range, (0.0, 1.5));
        assert_eq!(body.edges.get(e2).unwrap().range, (1.5, 4.0));
    }

    #[test]
    fn split_edge_within_a_loop_ring_preserves_ring_validity() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let coedges_before = body.loop_coedges(outer);
        let target_coedge = coedges_before[0];
        let target_edge = body.coedges.get(target_coedge).unwrap().edge;
        let (t0, t1) = body.edges.get(target_edge).unwrap().range;
        let mid = 0.5 * (t0 + t1);
        let curve = body.edges.get(target_edge).unwrap().curve;
        let position = body.curves3.get(curve).unwrap().eval(mid);
        split_edge(&mut body, target_edge, mid, position, &mut rec);
        let coedges_after = body.loop_coedges(outer);
        assert_eq!(coedges_after.len(), coedges_before.len() + 1, "the ring gains exactly one coedge");
        // The ring must still be a single closed cycle covering every live coedge in the loop.
        let mut seen = std::collections::HashSet::new();
        for c in &coedges_after {
            assert!(seen.insert(*c), "ring must not repeat a coedge");
        }
        for c in &coedges_after {
            let co = body.coedges.get(*c).unwrap();
            assert!(coedges_after.contains(&co.next));
            assert!(coedges_after.contains(&co.prev));
        }
    }

    #[test]
    fn split_edge_on_a_self_referential_single_coedge_loop_produces_a_valid_two_coedge_ring() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let curve = body.curves3.insert(Curve3::Circle { frame, radius: 1.0 });
        let v = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let edge = make_edge(&mut body, curve, (0.0, std::f64::consts::TAU), v, v, Tol::DEFAULT, &mut rec);
        let loop_id = make_loop(&mut body, FaceId::from_raw(0, 0), &[(edge, true)]);
        let coedges_before = body.loop_coedges(loop_id);
        assert_eq!(coedges_before.len(), 1);
        let single = coedges_before[0];
        assert_eq!(body.coedges.get(single).unwrap().next, single);
        assert_eq!(body.coedges.get(single).unwrap().prev, single);
        split_edge(&mut body, edge, std::f64::consts::PI, Pnt3::new(-1.0, 0.0, 0.0), &mut rec);
        let coedges_after = body.loop_coedges(loop_id);
        assert_eq!(coedges_after.len(), 2);
        let a = body.coedges.get(coedges_after[0]).unwrap();
        let b = body.coedges.get(coedges_after[1]).unwrap();
        assert_eq!(a.next, coedges_after[1]);
        assert_eq!(b.next, coedges_after[0]);
        assert_eq!(a.prev, coedges_after[1]);
        assert_eq!(b.prev, coedges_after[0]);
    }
}
// #endregion 🔖️Tests
