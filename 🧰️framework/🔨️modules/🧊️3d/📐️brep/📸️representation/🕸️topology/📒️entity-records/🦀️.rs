//! 📒️ Entity records: the vertex, edge, coedge, loop, face, shell and solid records with their retirement, the coedge use probe and the loop ring builders. See [`super`].

use super::*;

/// 🔗 Whether `coedge` is a use of `edge`: the one edge-use probe valence and incidence walks spend per coedge.
pub fn coedge_uses_edge(coedge: &Coedge, edge: EdgeId) -> bool {
    coedge.edge == edge
}

// #region 🔖️Entities
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct Vertex {
    pub position: Pnt3,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ An edge's `curve` is shared geometry; `range` is *this edge's* portion of that curve's
/// parameter domain, so two edges split from one original edge share `curve` with disjoint ranges.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct Loop {
    pub first: CoedgeId,
    pub face: FaceId,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct Shell {
    pub faces: Vec<FaceId>,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct Solid {
    pub outer: ShellId,
    pub inners: Vec<ShellId>,
    pub label: PersistentLabel,
}
// #endregion 🔖️Entities

// #region 🔖️Ring
/// 🔗 The placeholder coedge id a ring member holds until [`make_loop`] or an edge split links its neighbours.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dummy_coedge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}

/// ✂️ Builds a closed coedge ring from `members` (one `(edge, forward)` pair per coedge, in ring
/// order) and links it into a new [`Loop`]. Loops/coedges have no [`crate::topology::history::PersistentLabel`]
/// of their own (they are structural, not independently document-nameable), so nothing is recorded.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// #endregion 🔖️Ring
