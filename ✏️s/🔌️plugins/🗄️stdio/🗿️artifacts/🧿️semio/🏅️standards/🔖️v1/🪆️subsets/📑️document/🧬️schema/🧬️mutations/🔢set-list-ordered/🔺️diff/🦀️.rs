//! 🔺️ Diff for `SetListOrdered`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetListOrdered, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if block_at(base, &payload.path).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No block exists at {:?}.", payload.path.segments), [payload.path.index.to_string()]);
    }
    let super::SetListOrdered { path, ordered } = payload;
    protocol::MutationOutcome::new(match block_at(base, path) {
        Some(DocBlock::List { ordered: old, .. }) if old != ordered => wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::List(crate::standards::v1::subsets::document::schema::diff::DocListDiff { ordered: Some(*ordered), items: None }))),
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
