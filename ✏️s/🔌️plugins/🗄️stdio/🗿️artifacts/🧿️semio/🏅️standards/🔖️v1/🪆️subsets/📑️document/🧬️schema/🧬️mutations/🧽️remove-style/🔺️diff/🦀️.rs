//! 🔺️ Diff for `RemoveStyle`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveStyle, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::RemoveStyle { id } = payload;
    protocol::MutationOutcome::new({
        SemioDocumentDiff { styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff { removed: vec![id.clone()], ..Default::default() }), images: None, blocks: None }
    })
}
//#endregion 🔖️Diff
