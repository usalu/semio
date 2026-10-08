//! 🔺️ Diff for `SetChannelSamples`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetChannelSamples, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    if payload.index >= base.channels.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No channel exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::SetChannelSamples { index, samples } = payload;
    protocol::MutationOutcome::new({
        let d = SemioAudioChannelDiff { samples: Some(samples.clone()) };
        SemioAudioDiff { channels: Some(IndexedTripleDiff { modified: vec![IndexModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
    })
}
//#endregion 🔖️Diff
