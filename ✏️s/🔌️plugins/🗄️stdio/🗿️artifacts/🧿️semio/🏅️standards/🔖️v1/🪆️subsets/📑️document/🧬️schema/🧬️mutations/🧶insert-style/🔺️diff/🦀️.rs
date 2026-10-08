//! 🔺️ Diff for `InsertStyle`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertStyle, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::InsertStyle { style, at } = payload;
    protocol::MutationOutcome::new({
        SemioDocumentDiff { styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff { added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.styles.len(), |at| at.min(base.styles.len())), item: style.clone() }], ..Default::default() }), images: None, blocks: None }
    })
}
//#endregion 🔖️Diff
