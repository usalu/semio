//! 🔺️ Diff for `InsertElement`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertElement, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::InsertElement { element, at } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { elements: Some(NamedTripleDiff { added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.elements.len(), |at| at.min(base.elements.len())), item: element.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
