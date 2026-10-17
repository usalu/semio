//! 🔺️ Diff for `RemoveRelation`.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use crate::schema::diff::SemioModelDiff;
use crate::schema::snapshot::SemioModelSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveRelation, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    if !base.relations.iter().any(|r| r.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Relation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::RemoveRelation { id } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { relations: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
