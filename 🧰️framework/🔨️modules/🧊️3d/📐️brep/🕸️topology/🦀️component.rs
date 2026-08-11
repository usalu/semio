//! 🧱️ The B-Rep topology model: `Body` owns arenas of `Vertex/Edge/Coedge/Loop/Face/Shell/Solid`
//! plus geometry pools (`Curve3`/`Curve2`/`Surface`) that entities reference by id rather than
//! owning directly. **Host authority:** construct `Body` only inside an `Engine::compute` call or a
//! host-owned `EngineCache` entry — never as long-lived plugin state.

use crate::brep::arena::{CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, Store, SurfaceId, VertexId};
use crate::brep::curve::{Curve2, Curve3};
use crate::brep::history::{LabelSource, PersistentLabel};
use crate::brep::surface::Surface;
use crate::brep::tolerance::Tol;
use crate::brep::vec::Pnt3;

// #region 🔖️Entities

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vertex {
    pub position: Pnt3,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ An edge's `curve` is shared geometry; `range` is *this edge's* portion of that curve's
/// parameter domain, so two edges split from one original edge share `curve` with disjoint ranges.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Edge {
    pub curve: Curve3Id,
    pub range: (f64, f64),
    pub v0: VertexId,
    pub v1: VertexId,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ One face's use of one edge within one loop. `forward` is this use's orientation relative to
/// the edge's own `v0 → v1` direction. `pcurve`/`prange` are the edge's curve reparametrized into
/// the owning face's `(u, v)` domain — `None` only ever transiently, before a producer has filled
/// it in; a face with a missing pcurve on a non-planar surface fails validation (see `validate.rs`).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coedge {
    pub edge: EdgeId,
    pub forward: bool,
    pub pcurve: Option<Curve2Id>,
    pub prange: (f64, f64),
    pub loop_id: LoopId,
    pub next: CoedgeId,
    pub prev: CoedgeId,
}

/// 🧱️ A closed cycle of coedges bounding one region of a face (the outer boundary, or one hole).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Loop {
    pub first: CoedgeId,
    pub face: FaceId,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Face {
    pub surface: SurfaceId,
    pub outer: Option<LoopId>,
    pub inners: Vec<LoopId>,
    /// 🧱️ `true` when the face's outward normal is `-normal(surface)` (the surface's own natural
    /// normal, reversed) rather than matching it directly.
    pub flipped: bool,
    pub tol: Tol,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shell {
    pub faces: Vec<FaceId>,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Solid {
    pub outer: ShellId,
    pub inners: Vec<ShellId>,
    pub label: PersistentLabel,
}

// #endregion 🔖️Entities

// #region 🔖️Body

/// 🧱️ One B-Rep model: topology arenas + geometry pools + the label counter that stamps every
/// newly-born entity with a [`PersistentLabel`].
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Body {
    pub vertices: Store<Vertex, VertexId>,
    pub edges: Store<Edge, EdgeId>,
    pub coedges: Store<Coedge, CoedgeId>,
    pub loops: Store<Loop, LoopId>,
    pub faces: Store<Face, FaceId>,
    pub shells: Store<Shell, ShellId>,
    pub solids: Store<Solid, SolidId>,
    pub curves3: Store<Curve3, Curve3Id>,
    pub curves2: Store<Curve2, Curve2Id>,
    pub surfaces: Store<Surface, SurfaceId>,
    pub labels: LabelSource,
}

impl Body {
    pub fn new() -> Self {
        Body::default()
    }
    pub fn new_label(&mut self) -> PersistentLabel {
        self.labels.next_label()
    }
}

// #endregion 🔖️Body

// #region 🔖️Traverse

impl Body {
    /// 🧱️ Walks a loop's coedge ring starting from `Loop::first`, following `next` until it
    /// returns to the start. Panics via a debug assertion in the euler layer's invariant checks
    /// if the ring is malformed; callers here get a plain `Vec` (empty if the loop id is stale).
    pub fn loop_coedges(&self, loop_id: LoopId) -> Vec<CoedgeId> {
        let Some(lp) = self.loops.get(loop_id) else { return Vec::new() };
        let mut result = Vec::new();
        let mut current = lp.first;
        loop {
            result.push(current);
            let Some(coedge) = self.coedges.get(current) else { break };
            current = coedge.next;
            if current == lp.first {
                break;
            }
            if result.len() > self.coedges.len() {
                break; // malformed ring guard: never loop forever on corrupt data
            }
        }
        result
    }
    pub fn face_loops(&self, face_id: FaceId) -> Vec<LoopId> {
        let Some(face) = self.faces.get(face_id) else { return Vec::new() };
        let mut result: Vec<LoopId> = face.outer.into_iter().collect();
        result.extend(face.inners.iter().copied());
        result
    }
    pub fn face_coedges(&self, face_id: FaceId) -> Vec<CoedgeId> {
        self.face_loops(face_id).into_iter().flat_map(|l| self.loop_coedges(l)).collect()
    }
    pub fn shell_faces(&self, shell_id: ShellId) -> Vec<FaceId> {
        self.shells.get(shell_id).map(|s| s.faces.clone()).unwrap_or_default()
    }
    pub fn solid_shells(&self, solid_id: SolidId) -> Vec<ShellId> {
        let Some(solid) = self.solids.get(solid_id) else { return Vec::new() };
        let mut result = vec![solid.outer];
        result.extend(solid.inners.iter().copied());
        result
    }
    pub fn solid_faces(&self, solid_id: SolidId) -> Vec<FaceId> {
        self.solid_shells(solid_id).into_iter().flat_map(|s| self.shell_faces(s)).collect()
    }
    /// 🧱️ The edge's endpoint vertices in `(start, end)` order as seen through `coedge`'s own
    /// orientation (i.e. respecting `forward`, not the underlying edge's raw `v0`/`v1`).
    pub fn coedge_endpoints(&self, coedge_id: CoedgeId) -> Option<(VertexId, VertexId)> {
        let coedge = self.coedges.get(coedge_id)?;
        let edge = self.edges.get(coedge.edge)?;
        Some(if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) })
    }
    /// 🧱️ Every vertex incident to at least one edge that references it as `v0` or `v1`.
    pub fn vertex_edges(&self, vertex_id: VertexId) -> Vec<EdgeId> {
        self.edges.iter().filter(|(_, e)| e.v0 == vertex_id || e.v1 == vertex_id).map(|(id, _)| id).collect()
    }
    /// 🧱️ Every coedge that uses `edge_id` (both orientations, both faces if the edge is shared).
    pub fn edge_coedges(&self, edge_id: EdgeId) -> Vec<CoedgeId> {
        self.coedges.iter().filter(|(_, c)| c.edge == edge_id).map(|(id, _)| id).collect()
    }
}

// #endregion 🔖️Traverse

// #region 🔖️Remap

impl Body {
    /// 🧱️ A deep copy of the entire body: every arena's entries are copied into a fresh `Body`
    /// with (generally) different arena indices, but *the same* [`PersistentLabel`]s — used
    /// wherever a caller needs an independent, mutable working copy without disturbing the
    /// original (e.g. undo snapshots, before the document layer's smarter delta-based history).
    pub fn deep_copy(&self) -> Body {
        self.clone()
    }
}

// #endregion 🔖️Remap

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::arena::ArenaId;
    use crate::brep::mat::Frame3;
    use crate::brep::vec::Vec3;

    fn null_coedge() -> CoedgeId {
        ArenaId::from_raw(0, 0)
    }
    fn null_loop() -> LoopId {
        ArenaId::from_raw(0, 0)
    }
    fn null_face() -> FaceId {
        ArenaId::from_raw(0, 0)
    }

    // Small test-only builders that pre-fetch `body.new_label()` into a local before the
    // `insert(...)` call — calling `body.new_label()` inline as an argument to `body.x.insert(..)`
    // is a double mutable borrow of `body` the borrow checker rejects even though the fields are
    // disjoint (the two calls are nested, not sequential).
    fn insert_vertex(body: &mut Body, position: Pnt3) -> VertexId {
        let label = body.new_label();
        body.vertices.insert(Vertex { position, tol: Tol::DEFAULT, label })
    }
    fn insert_edge(body: &mut Body, curve: Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId) -> EdgeId {
        let label = body.new_label();
        body.edges.insert(Edge { curve, range, v0, v1, tol: Tol::DEFAULT, label })
    }
    fn insert_face(body: &mut Body, surface: SurfaceId) -> FaceId {
        let label = body.new_label();
        body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: false, tol: Tol::DEFAULT, label })
    }
    fn insert_shell(body: &mut Body, faces: Vec<FaceId>) -> ShellId {
        let label = body.new_label();
        body.shells.insert(Shell { faces, label })
    }
    fn insert_solid(body: &mut Body, outer: ShellId, inners: Vec<ShellId>) -> SolidId {
        let label = body.new_label();
        body.solids.insert(Solid { outer, inners, label })
    }

    fn make_triangle_loop(body: &mut Body, face: FaceId, positions: [Pnt3; 3]) -> LoopId {
        let vertices: Vec<VertexId> = positions.iter().map(|&p| insert_vertex(body, p)).collect();
        let curves: Vec<Curve3Id> = (0..3)
            .map(|i| {
                let a = positions[i];
                let b = positions[(i + 1) % 3];
                body.curves3.insert(Curve3::Line { origin: a, dir: b - a })
            })
            .collect();
        let edges: Vec<EdgeId> = (0..3).map(|i| insert_edge(body, curves[i], (0.0, 1.0), vertices[i], vertices[(i + 1) % 3])).collect();
        let loop_id = body.loops.insert(Loop { first: null_coedge(), face });
        let coedge_ids: Vec<CoedgeId> = edges.iter().map(|&e| body.coedges.insert(Coedge { edge: e, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() })).collect();
        for i in 0..3 {
            let coedge = body.coedges.get_mut(coedge_ids[i]).unwrap();
            coedge.next = coedge_ids[(i + 1) % 3];
            coedge.prev = coedge_ids[(i + 2) % 3];
        }
        body.loops.get_mut(loop_id).unwrap().first = coedge_ids[0];
        loop_id
    }

    #[test]
    fn loop_coedges_walks_the_full_ring_once() {
        let mut body = Body::new();
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let face = insert_face(&mut body, surface);
        let loop_id = make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)]);
        let coedges = body.loop_coedges(loop_id);
        assert_eq!(coedges.len(), 3);
        assert_eq!(coedges[0], body.loops.get(loop_id).unwrap().first);
    }

    #[test]
    fn face_loops_includes_outer_and_all_inner_loops() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let face = insert_face(&mut body, surface);
        let outer = make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(10.0, 0.0, 0.0), Pnt3::new(0.0, 10.0, 0.0)]);
        let inner = make_triangle_loop(&mut body, face, [Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(1.0, 2.0, 0.0)]);
        body.faces.get_mut(face).unwrap().outer = Some(outer);
        body.faces.get_mut(face).unwrap().inners = vec![inner];
        let loops = body.face_loops(face);
        assert_eq!(loops.len(), 2);
        assert!(loops.contains(&outer));
        assert!(loops.contains(&inner));
        assert_eq!(body.face_coedges(face).len(), 6);
    }

    #[test]
    fn shell_and_solid_traversal_returns_all_members() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let f1 = insert_face(&mut body, surface);
        let f2 = insert_face(&mut body, surface);
        let shell = insert_shell(&mut body, vec![f1, f2]);
        let inner_shell = insert_shell(&mut body, vec![]);
        let solid = insert_solid(&mut body, shell, vec![inner_shell]);
        assert_eq!(body.shell_faces(shell), vec![f1, f2]);
        assert_eq!(body.solid_shells(solid), vec![shell, inner_shell]);
        assert_eq!(body.solid_faces(solid), vec![f1, f2]);
    }

    #[test]
    fn coedge_endpoints_respects_orientation() {
        let mut body = Body::new();
        let v0 = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
        let v1 = insert_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0));
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = insert_edge(&mut body, curve, (0.0, 1.0), v0, v1);
        let loop_id = body.loops.insert(Loop { first: null_coedge(), face: null_face() });
        let fwd = body.coedges.insert(Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() });
        let rev = body.coedges.insert(Coedge { edge, forward: false, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() });
        assert_eq!(body.coedge_endpoints(fwd), Some((v0, v1)));
        assert_eq!(body.coedge_endpoints(rev), Some((v1, v0)));
    }

    #[test]
    fn vertex_edges_and_edge_coedges_find_all_incident_entries() {
        let mut body = Body::new();
        let v0 = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
        let v1 = insert_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0));
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = insert_edge(&mut body, curve, (0.0, 1.0), v0, v1);
        body.coedges.insert(Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: null_loop(), next: null_coedge(), prev: null_coedge() });
        body.coedges.insert(Coedge { edge, forward: false, pcurve: None, prange: (0.0, 1.0), loop_id: null_loop(), next: null_coedge(), prev: null_coedge() });
        assert_eq!(body.vertex_edges(v0), vec![edge]);
        assert_eq!(body.vertex_edges(v1), vec![edge]);
        assert_eq!(body.edge_coedges(edge).len(), 2);
    }

    #[test]
    fn serde_round_trips_a_whole_body() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let face = insert_face(&mut body, surface);
        make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)]);
        let json = serde_json::to_string(&body).unwrap();
        let back: Body = serde_json::from_str(&json).unwrap();
        assert_eq!(back.vertices.len(), body.vertices.len());
        assert_eq!(back.edges.len(), body.edges.len());
        assert_eq!(back.faces.len(), body.faces.len());
    }

    #[test]
    fn deep_copy_produces_an_independent_body() {
        let mut body = Body::new();
        let v = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
        let mut copy = body.deep_copy();
        copy.vertices.get_mut(v).unwrap().position = Pnt3::new(9.0, 9.0, 9.0);
        assert_ne!(body.vertices.get(v).unwrap().position, copy.vertices.get(v).unwrap().position);
    }
}
// #endregion 🔖️Tests
