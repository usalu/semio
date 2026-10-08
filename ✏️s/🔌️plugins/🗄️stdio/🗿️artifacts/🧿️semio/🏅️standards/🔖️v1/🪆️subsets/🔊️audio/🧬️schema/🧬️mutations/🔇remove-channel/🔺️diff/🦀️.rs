//! 🔺️ Diff for `RemoveChannel`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveChannel, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    let super::RemoveChannel { index } = payload;
    protocol::MutationOutcome::new(SemioAudioDiff { channels: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
