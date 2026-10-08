//! 🔺️ Diff for `SetFramePixels`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetFramePixels, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::SetFramePixels { index, rgba8 } = payload;
    if *index >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {index} does not exist."), [index.to_string()]);
    }
    let expected_len = base.width as usize * base.height as usize * 4;
    if rgba8.len() != expected_len {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Frame index {index} pixel buffer has {} byte(s), expected {expected_len} (width*height*4).", rgba8.len()), [index.to_string()]);
    }
    protocol::MutationOutcome::new({
        SemioImageDiff { frames: Some(SemioImageFramesDiff { modified: vec![IndexModified { index: *index, diff: SemioImageFrameDiff { delay_ms: None, rgba8: Some(rgba8.clone()) } }], ..Default::default() }), ..Default::default() }
    })
}
//#endregion 🔖️Diff
