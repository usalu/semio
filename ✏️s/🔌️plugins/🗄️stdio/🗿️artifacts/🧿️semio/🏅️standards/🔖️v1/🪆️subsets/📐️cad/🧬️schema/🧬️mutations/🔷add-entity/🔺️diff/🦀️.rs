//! 🔺️ Diff for `AddEntity`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddEntity, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_entity(base, &payload.entity.handle).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Entity \"{}\" already exists.", payload.entity.handle), [payload.entity.handle.clone()]);
    }
    let super::AddEntity { entity, at } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: None, entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.entities.len(), |at| at.min(base.entities.len())), item: entity.clone() }] }) })
}
//#endregion 🔖️Diff
