//! 🔺️ Diff for `SetHeadingLevel`.

#[allow(unused_imports)]
use crate::schema::snapshot::DocBlock;
#[allow(unused_imports)]
use crate::schema::diff::DocBlockDiff;
#[allow(unused_imports)]
use crate::schema::mutations::support::DocBlockLeaf;
#[allow(unused_imports)]
use crate::schema::diff::DocHeadingDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioDocumentDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::support::wrap_body_diff;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetHeadingLevel, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if block_at(base, &payload.path).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No block exists at {:?}.", payload.path.segments), [payload.path.index.to_string()]);
    }
    let super::SetHeadingLevel { path, level } = payload;
    protocol::MutationOutcome::new(match block_at(base, path) {
        Some(DocBlock::Heading { level: old, .. }) if old != level => wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Heading(DocHeadingDiff { level: Some(*level), style_id: None, runs: None }))),
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
