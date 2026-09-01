//! ↩️ Inverse for `DeleteVertex`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, create_edge, create_vertex, delete_edge};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteVertex, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
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
        .map(|e| SemioBrepMutation::DeleteEdge(delete_edge::DeleteEdge { id: e.id.clone() }))
        .collect();
    undo.extend(disturbed.iter().skip(1).map(|v| SemioBrepMutation::DeleteVertex(super::DeleteVertex { id: v.id.clone() })));
    undo.extend(disturbed.iter().map(|v| SemioBrepMutation::CreateVertex(create_vertex::CreateVertex { id: v.id.clone(), point: v.point })));
    undo.extend(edge_tail.iter().map(|e| {
        SemioBrepMutation::CreateEdge(create_edge::CreateEdge { id: e.id.clone(), start_vertex: e.start_vertex.clone(), end_vertex: e.end_vertex.clone(), curve: e.curve.clone() })
    }));
    undo
}
//#endregion 🔖️Inverse
