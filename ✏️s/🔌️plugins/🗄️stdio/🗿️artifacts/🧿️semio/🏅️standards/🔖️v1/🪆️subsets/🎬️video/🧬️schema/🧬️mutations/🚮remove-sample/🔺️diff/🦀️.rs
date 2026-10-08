//! 🔺️ Diff for `RemoveSample`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveSample, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    if base.streams.get(payload.stream_index).and_then(|s| s.samples.get(payload.index)).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No sample exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::RemoveSample { stream_index, index } = payload;
    protocol::MutationOutcome::new(diff_remove_sample(*stream_index, *index))
}
//#endregion 🔖️Diff
