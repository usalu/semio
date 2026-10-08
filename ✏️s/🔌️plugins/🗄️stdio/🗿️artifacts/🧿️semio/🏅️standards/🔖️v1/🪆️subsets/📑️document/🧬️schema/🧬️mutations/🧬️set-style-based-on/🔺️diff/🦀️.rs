//! 🔺️ Diff for `SetStyleBasedOn`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetStyleBasedOn, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::SetStyleBasedOn { id, based_on } = payload;
    protocol::MutationOutcome::new(match style_at(base, id) {
        Some(old) if &old.based_on != based_on => SemioDocumentDiff {
            styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff {
                modified: vec![crate::standards::v1::subsets::base::schema::triples::NamedModified { key: id.clone(), diff: crate::standards::v1::subsets::document::schema::diff::DocStyleDiff { name: None, based_on: Some(based_on.clone()) } }],
                ..Default::default()
            }),
            images: None,
            blocks: None,
        },
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
