//! 🧮️ Net of one snapshot edit as brep domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `brep` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::net_keyed;
use crate::standards::v1::subsets::brep::schema::mutations::{create_edge, create_face, create_shell, create_solid, create_vertex, delete_edge, delete_face, delete_shell, delete_solid, delete_vertex, move_vertex, replace_curve, replace_surface, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioBrepSnapshot, next: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let vertices = net_keyed(&base.vertices, &next.vertices, |vertex| vertex.id.clone());
    let edges = net_keyed(&base.edges, &next.edges, |edge| edge.id.clone());
    let faces = net_keyed(&base.faces, &next.faces, |face| face.id.clone());
    let shells = net_keyed(&base.shells, &next.shells, |shell| shell.id.clone());
    let solids = net_keyed(&base.solids, &next.solids, |solid| solid.id.clone());
    let mut out = Vec::new();
    out.extend(solids.removed.iter().map(|solid| SemioBrepMutation::DeleteSolid(delete_solid::DeleteSolid { id: solid.id.clone() })));
    out.extend(shells.removed.iter().map(|shell| SemioBrepMutation::DeleteShell(delete_shell::DeleteShell { id: shell.id.clone() })));
    out.extend(faces.removed.iter().map(|face| SemioBrepMutation::DeleteFace(delete_face::DeleteFace { id: face.id.clone() })));
    out.extend(edges.removed.iter().map(|edge| SemioBrepMutation::DeleteEdge(delete_edge::DeleteEdge { id: edge.id.clone() })));
    out.extend(vertices.removed.iter().map(|vertex| SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: vertex.id.clone() })));
    out.extend(vertices.added.iter().map(|vertex| SemioBrepMutation::CreateVertex(create_vertex::CreateVertex { id: vertex.id.clone(), point: vertex.point, tol: vertex.tol, at: None })));
    out.extend(vertices.modified.iter().filter(|(before, after)| before.point != after.point).map(|(_, after)| SemioBrepMutation::MoveVertex(move_vertex::MoveVertex { vertex_id: after.id.clone(), new_point: after.point })));
    out.extend(edges.added.iter().map(|edge| SemioBrepMutation::CreateEdge(create_edge::CreateEdge { id: edge.id.clone(), start_vertex: edge.start_vertex.clone(), end_vertex: edge.end_vertex.clone(), curve: edge.curve.clone(), tol: edge.tol, at: None })));
    out.extend(edges.modified.iter().filter(|(before, after)| before.curve != after.curve).map(|(_, after)| SemioBrepMutation::ReplaceCurve(replace_curve::ReplaceCurve { edge_id: after.id.clone(), new_curve: after.curve.clone() })));
    out.extend(faces.added.iter().map(|face| {
        SemioBrepMutation::CreateFace(create_face::CreateFace { id: face.id.clone(), outer_loop: face.outer_loop.clone(), inner_loops: face.inner_loops.clone(), surface: face.surface.clone(), orientation: face.orientation, tol: face.tol, at: None })
    }));
    out.extend(faces.modified.iter().filter(|(before, after)| before.surface != after.surface).map(|(_, after)| SemioBrepMutation::ReplaceSurface(replace_surface::ReplaceSurface { face_id: after.id.clone(), new_surface: after.surface.clone() })));
    out.extend(shells.added.iter().map(|shell| SemioBrepMutation::CreateShell(create_shell::CreateShell { id: shell.id.clone(), faces: shell.faces.clone(), at: None })));
    out.extend(solids.added.iter().map(|solid| SemioBrepMutation::CreateSolid(create_solid::CreateSolid { id: solid.id.clone(), shells: solid.shells.clone(), at: None })));
    out
}
