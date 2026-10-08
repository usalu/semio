//! 🔺️ Diff for `InsertStream`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertStream, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    if payload.index > base.streams.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No stream slot exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::InsertStream { index, stream } = payload;
    protocol::MutationOutcome::new(diff_insert_stream(*index, stream.clone()))
}
//#endregion 🔖️Diff
