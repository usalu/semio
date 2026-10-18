//! 🗄️ Wire and body arenas: the wire record, the body with its arenas and retirement owners, its traversal accessors and the remap type. See [`super`].

use super::*;

// #region 🔖️Wire
/// 🧱 An ordered chain of oriented edges produced by a wire constructor (not yet bound to a face).
#[derive(Clone, Debug, PartialEq)]
pub struct Wire {
    pub members: Vec<(EdgeId, bool)>,
    pub vertices: Vec<VertexId>,
    pub closed: bool,
}

semio_framework_value::artifact_retire_struct!(Wire {members,vertices,closed});
// #endregion 🔖️Wire

// #region 🔖️Body
/// 🧱️ One B-Rep model: topology arenas + geometry pools + the label counter that stamps every
/// newly-born entity with a [`PersistentLabel`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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

semio_framework_value::artifact_retire_struct!(Vertex {position,tol,label});

semio_framework_value::artifact_retire_struct!(Edge {curve,range,v0,v1,tol,label});

semio_framework_value::artifact_retire_struct!(Coedge {edge,forward,pcurve,prange,loop_id,next,prev});

semio_framework_value::artifact_retire_struct!(Loop {first,face});

semio_framework_value::artifact_retire_struct!(Face {surface,outer,inners,flipped,tol,label});

semio_framework_value::artifact_retire_struct!(Shell {faces,label});

semio_framework_value::artifact_retire_struct!(Solid {outer,inners,label});

semio_framework_value::artifact_retire_struct!(Body {vertices,edges,coedges,loops,faces,shells,solids,curves3,curves2,surfaces,labels});

impl Body {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Body::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn face_loops(&self, face_id: FaceId) -> Vec<LoopId> {
        let Some(face) = self.faces.get(face_id) else { return Vec::new() };
        let mut result: Vec<LoopId> = face.outer.into_iter().collect();
        result.extend(face.inners.iter().copied());
        result
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn face_coedges(&self, face_id: FaceId) -> Vec<CoedgeId> {
        self.face_loops(face_id).into_iter().flat_map(|l| self.loop_coedges(l)).collect()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn shell_faces(&self, shell_id: ShellId) -> Vec<FaceId> {
        self.shells.get(shell_id).map(|s| s.faces.clone()).unwrap_or_default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn solid_shells(&self, solid_id: SolidId) -> Vec<ShellId> {
        let Some(solid) = self.solids.get(solid_id) else { return Vec::new() };
        let mut result = vec![solid.outer];
        result.extend(solid.inners.iter().copied());
        result
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn solid_faces(&self, solid_id: SolidId) -> Vec<FaceId> {
        self.solid_shells(solid_id).into_iter().flat_map(|s| self.shell_faces(s)).collect()
    }
    /// 🧱️ The edge's endpoint vertices in `(start, end)` order as seen through `coedge`'s own
    /// orientation (i.e. respecting `forward`, not the underlying edge's raw `v0`/`v1`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn coedge_endpoints(&self, coedge_id: CoedgeId) -> Option<(VertexId, VertexId)> {
        let coedge = self.coedges.get(coedge_id)?;
        let edge = self.edges.get(coedge.edge)?;
        Some(if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) })
    }
    /// 🧱️ Every vertex incident to at least one edge that references it as `v0` or `v1`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn vertex_edges(&self, vertex_id: VertexId) -> Vec<EdgeId> {
        self.edges.iter().filter(|(_, e)| e.v0 == vertex_id || e.v1 == vertex_id).map(|(id, _)| id).collect()
    }
    /// 🧱️ Every coedge that uses `edge_id` (both orientations, both faces if the edge is shared).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn edge_coedges(&self, edge_id: EdgeId) -> Vec<CoedgeId> {
        self.coedges.iter().filter(|(_, c)| coedge_uses_edge(c, edge_id)).map(|(id, _)| id).collect()
    }
}
// #endregion 🔖️Traverse

// #region 🔖️Remap
impl Body {
    /// 🧱️ A deep copy of the entire body: every arena's entries are copied into a fresh `Body`
    /// with (generally) different arena indices, but *the same* [`PersistentLabel`]s — used
    /// wherever a caller needs an independent, mutable working copy without disturbing the
    /// original (e.g. undo snapshots, before the document layer's smarter delta-based history).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn deep_copy(&self) -> Body {
        self.clone()
    }
}
// #endregion 🔖️Remap
