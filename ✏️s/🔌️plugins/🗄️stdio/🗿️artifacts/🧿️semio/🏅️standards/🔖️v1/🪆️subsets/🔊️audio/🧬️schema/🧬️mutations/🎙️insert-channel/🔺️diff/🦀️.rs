//! 🔺️ Diff for `InsertChannel`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertChannel, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    let super::InsertChannel { index, channel } = payload;
    protocol::MutationOutcome::new({
        SemioAudioDiff { channels: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.channels.len()), item: channel.clone() }], ..Default::default() }), ..Default::default() }
    })
}
//#endregion 🔖️Diff
