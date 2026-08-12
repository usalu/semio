//! 🔺️ `delete-edge` — sparse diff construction; removes the edge with this `id` from
//! `edges`. No cascade into any other collection (see the payload leaf's doc comment for
//! why). Absent `id` in `base` yields `SemioBrepDiff::default()` — a genuinely empty diff, not merely a
//! harmless-to-apply one (id-keyed collections are unordered sets; a spurious `removed` entry for
//! an absent id would make `SemioBrepDiff::is_empty()` lie).

use super::mutation::DeleteEdge;
use crate::artifacts::semio::standards::v1::engine::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteEdge, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if !base.edges.iter().any(|x| x.id == payload.id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        edges: Some(NamedTripleDiff { removed: vec![payload.id.clone()], modified: vec![], added: vec![] }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
