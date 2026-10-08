//! 🔺️ Diff for `SetFormat`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetFormat, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    let super::SetFormat { format } = payload;
    protocol::MutationOutcome::new(SemioAudioDiff { format: (*format != base.format).then_some(*format), ..Default::default() })
}
//#endregion 🔖️Diff
