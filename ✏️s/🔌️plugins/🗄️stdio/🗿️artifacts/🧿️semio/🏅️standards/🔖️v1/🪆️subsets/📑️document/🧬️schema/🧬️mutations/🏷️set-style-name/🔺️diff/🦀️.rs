//! 🔺️ Diff for `SetStyleName`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetStyleName, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if style_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::SetStyleName { id, name } = payload;
    protocol::MutationOutcome::new(match style_at(base, id) {
        Some(old) if &old.name != name => SemioDocumentDiff {
            styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff {
                modified: vec![crate::standards::v1::subsets::base::schema::triples::NamedModified { key: id.clone(), diff: crate::standards::v1::subsets::document::schema::diff::DocStyleDiff { name: Some(name.clone()), based_on: None } }],
                ..Default::default()
            }),
            images: None,
            blocks: None,
        },
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
