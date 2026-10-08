//! 🔺️ Diff for `SetDimensions`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetDimensions, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::SetDimensions { width, height } = payload;
    if base.width == *width && base.height == *height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Dimensions are already this value.".to_string());
    }
    protocol::MutationOutcome::new(SemioImageDiff { width: (base.width != *width).then_some(*width), height: (base.height != *height).then_some(*height), ..Default::default() })
}
//#endregion 🔖️Diff
