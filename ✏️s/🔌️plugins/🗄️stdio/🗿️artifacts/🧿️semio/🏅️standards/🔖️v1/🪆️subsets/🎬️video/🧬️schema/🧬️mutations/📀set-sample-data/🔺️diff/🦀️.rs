//! 🔺️ Diff for `SetSampleData`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetSampleData, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    if base.streams.get(payload.stream_index).and_then(|s| s.samples.get(payload.index)).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No sample exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::SetSampleData { stream_index, index, data } = payload;
    protocol::MutationOutcome::new(match sample_at(base, *stream_index, *index) {
        Some(old) => diff_set_sample_data(old, *stream_index, *index, data.clone()),
        None => SemioVideoDiff::default(),
    })
}
//#endregion 🔖️Diff
