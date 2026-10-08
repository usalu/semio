//! 🔺️ Diff for `SetSampleFlags`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetSampleFlags, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    let super::SetSampleFlags { stream_index, index, pts, key } = payload;
    protocol::MutationOutcome::new(match sample_at(base, *stream_index, *index) {
        Some(old) => diff_set_sample_flags(old, *stream_index, *index, *pts, *key),
        None => SemioVideoDiff::default(),
    })
}
//#endregion 🔖️Diff
