//! 🔺️ `delete-vertex` — a real cascade: `vertices` AND `edges` both change (every edge whose
//! `start_vertex`/`end_vertex` references this vertex is cascade-deleted too, since an edge cannot
//! validly dangle a scalar endpoint reference — unlike `loop.edges`/`shell.faces`/`solid.shells`,
//! which are Vec-membership fields with no modify-verb and are therefore NOT cascaded into, see the
//! payload leaf's doc comment). Absent `id` in `base` yields `SemioBrepDiff::default()` — a
//! genuinely empty diff (id-keyed collections are unordered sets; a spurious `removed` entry for an
//! absent id would make `SemioBrepDiff::is_empty()` lie).

use super::mutation::DeleteVertex;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteVertex, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if !base.vertices.iter().any(|v| v.id == payload.id) {
        return SemioBrepDiff::default();
    }
    let dependent_edges: Vec<String> = base.edges.iter().filter(|e| e.start_vertex == payload.id || e.end_vertex == payload.id).map(|e| e.id.clone()).collect();
    SemioBrepDiff {
        vertices: Some(NamedTripleDiff { removed: vec![payload.id.clone()], modified: vec![], added: vec![] }),
        edges: if dependent_edges.is_empty() { None } else { Some(NamedTripleDiff { removed: dependent_edges, modified: vec![], added: vec![] }) },
        ..Default::default()
    }
}
//#endregion 🔖️Diff
