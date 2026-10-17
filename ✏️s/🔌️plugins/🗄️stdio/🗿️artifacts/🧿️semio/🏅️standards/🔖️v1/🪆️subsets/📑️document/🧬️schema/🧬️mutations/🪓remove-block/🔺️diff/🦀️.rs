//! 🔺️ Diff for `RemoveBlock`.

#[allow(unused_imports)]
use crate::schema::mutations::support::DocBlockLeaf;
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
pub fn diff(payload: &super::RemoveBlock, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if block_at(base, &payload.path).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No block exists at {:?}.", payload.path.segments), [payload.path.index.to_string()]);
    }
    let super::RemoveBlock { path } = payload;
    protocol::MutationOutcome::new(wrap_body_diff(path, DocBlockLeaf::Removed))
}
//#endregion 🔖️Diff
