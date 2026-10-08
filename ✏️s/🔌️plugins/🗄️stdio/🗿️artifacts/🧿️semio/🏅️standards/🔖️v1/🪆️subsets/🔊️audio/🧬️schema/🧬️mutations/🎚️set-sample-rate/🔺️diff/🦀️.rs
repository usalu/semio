//! 🔺️ Diff for `SetSampleRate`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetSampleRate, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    let super::SetSampleRate { sample_rate } = payload;
    protocol::MutationOutcome::new(SemioAudioDiff { sample_rate: (*sample_rate != base.sample_rate).then_some(*sample_rate), ..Default::default() })
}
//#endregion 🔖️Diff
