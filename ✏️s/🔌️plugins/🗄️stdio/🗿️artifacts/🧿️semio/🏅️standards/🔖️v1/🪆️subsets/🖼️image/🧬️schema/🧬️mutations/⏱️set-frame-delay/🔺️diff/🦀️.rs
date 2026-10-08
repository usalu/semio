//! 🔺️ Diff for `SetFrameDelay`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetFrameDelay, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::SetFrameDelay { index, delay_ms } = payload;
    if *index >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {index} does not exist."), [index.to_string()]);
    }
    protocol::MutationOutcome::new({
        SemioImageDiff { frames: Some(SemioImageFramesDiff { modified: vec![IndexModified { index: *index, diff: SemioImageFrameDiff { delay_ms: Some(*delay_ms), rgba8: None } }], ..Default::default() }), ..Default::default() }
    })
}
//#endregion 🔖️Diff
