//! 🔺️ Diff for `InsertRelation`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertRelation, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    if base.relations.iter().any(|r| r.id == payload.relation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Relation \"{}\" already exists.", payload.relation.id), [payload.relation.id.clone()]);
    }
    let super::InsertRelation { relation, at } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { relations: Some(NamedTripleDiff { added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.relations.len(), |at| at.min(base.relations.len())), item: relation.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
