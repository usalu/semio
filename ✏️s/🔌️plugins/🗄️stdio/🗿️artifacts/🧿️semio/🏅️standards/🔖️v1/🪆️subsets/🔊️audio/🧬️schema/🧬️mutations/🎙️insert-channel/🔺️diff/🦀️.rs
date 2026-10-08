//! 🔺️ Diff for `InsertChannel`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertChannel, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    if payload.index > base.channels.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No channel slot exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::InsertChannel { index, channel } = payload;
    protocol::MutationOutcome::new({
        SemioAudioDiff { channels: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.channels.len()), item: channel.clone() }], ..Default::default() }), ..Default::default() }
    })
}
//#endregion 🔖️Diff
