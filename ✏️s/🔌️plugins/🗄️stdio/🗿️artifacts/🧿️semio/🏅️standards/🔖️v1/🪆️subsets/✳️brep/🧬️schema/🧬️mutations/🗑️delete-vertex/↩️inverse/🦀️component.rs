//! ↩️ `delete-vertex` — restores the vertex AND every cascade-severed edge at their OWN indices.
//!
//! Two facts collide here. `delete-vertex` cascades: every edge naming the vertex as an endpoint is
//! removed with it, because an edge cannot dangle a scalar endpoint reference. And
//! `create-vertex`/`create-edge` can only APPEND. So the old inverse — one `CreateVertex` followed
//! by one `CreateEdge` per severed edge — put the right entities back in the wrong PLACES: deleting
//! `v2` out of `[v1,v2,v3,v4]` and undoing it produced `[v1,v3,v4,v2]`, with `[e1,e2,e3,e4]`
//! likewise re-ordered to `[e3,e4,e1,e2]`. `SemioBrepSnapshot` compares its collections in order and
//! this subset's `.dsl.semio` carrier writes them in order, so that is a different document, not a
//! restored one. `mutate-semio-brep`'s `inverse-delete-vertex` scenario caught it the first time the
//! Rust subject phase ran (ticket 26/08/23/END-TO-END-TESTING-REFACTOR).
//!
//! The remedy is the one `🧊️obj`'s `RemoveFace` and `✳️kit`'s `unbind-representation` needed: removing
//! index `i` closes the whole index space above it, so the tail is lifted off and re-declared in
//! order. Both index spaces are involved, so both are rebuilt: `first_edge` is the earliest position
//! in `base.edges` holding an edge incident to ANY vertex from `index` on, which is exactly the
//! point above which edge order can be disturbed. The steps are (1) delete the edges from
//! `first_edge` on that the cascade did NOT already take — `delete-edge` of an absent id is an Error
//! (`mutation.target-missing`), not a no-op, so the ones it did take must be left alone — then (2)
//! delete the vertices after `index`, which by then carry no edges and so cascade into nothing, then
//! (3) re-create the vertices from `index` on in order, then (4) re-create the edges from
//! `first_edge` on in order. Loop/face/shell membership names edges by id and neither verb cascades
//! into it, so the lift-off leaves every membership list untouched. Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteVertex;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_edge, create_vertex, delete_edge, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteVertex, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.vertices.iter().position(|v| v.id == payload.id) else {
        return Vec::new();
    };
    let disturbed = &base.vertices[index..];
    let touches = |start: &str, end: &str| disturbed.iter().any(|v| v.id == start || v.id == end);
    let first_edge = base.edges.iter().position(|e| touches(&e.start_vertex, &e.end_vertex)).unwrap_or(base.edges.len());
    let edge_tail = &base.edges[first_edge..];

    let mut undo: Vec<SemioBrepMutation> = edge_tail
        .iter()
        .filter(|e| !touches(&e.start_vertex, &e.end_vertex))
        .map(|e| SemioBrepMutation::DeleteEdge(delete_edge::mutation::DeleteEdge { id: e.id.clone() }))
        .collect();
    undo.extend(disturbed.iter().skip(1).map(|v| SemioBrepMutation::DeleteVertex(DeleteVertex { id: v.id.clone() })));
    undo.extend(disturbed.iter().map(|v| SemioBrepMutation::CreateVertex(create_vertex::mutation::CreateVertex { id: v.id.clone(), point: v.point })));
    undo.extend(edge_tail.iter().map(|e| {
        SemioBrepMutation::CreateEdge(create_edge::mutation::CreateEdge { id: e.id.clone(), start_vertex: e.start_vertex.clone(), end_vertex: e.end_vertex.clone(), curve: e.curve.clone() })
    }));
    undo
}
//#endregion 🔖️Inverse
