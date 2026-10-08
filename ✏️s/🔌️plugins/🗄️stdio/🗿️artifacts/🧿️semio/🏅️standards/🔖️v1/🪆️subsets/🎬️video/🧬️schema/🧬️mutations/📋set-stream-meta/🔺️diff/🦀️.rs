//! 🔺️ Diff for `SetStreamMeta`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetStreamMeta, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    let super::SetStreamMeta { index, kind, codec, width, height, rate } = payload;
    protocol::MutationOutcome::new(match stream_at(base, *index) {
        Some(old) => diff_set_stream_meta(old, *index, *kind, codec, *width, *height, *rate),
        None => SemioVideoDiff::default(),
    })
}
//#endregion 🔖️Diff
