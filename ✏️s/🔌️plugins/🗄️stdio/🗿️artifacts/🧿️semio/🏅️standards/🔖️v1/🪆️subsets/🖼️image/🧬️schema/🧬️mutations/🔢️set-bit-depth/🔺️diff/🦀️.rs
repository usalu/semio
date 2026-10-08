//! 🔺️ Diff for `SetBitDepth`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetBitDepth, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::SetBitDepth { bit_depth } = payload;
    if base.bit_depth == *bit_depth {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Bit depth is already this value.".to_string());
    }
    protocol::MutationOutcome::new(SemioImageDiff { bit_depth: (base.bit_depth != *bit_depth).then_some(*bit_depth), ..Default::default() })
}
//#endregion 🔖️Diff
