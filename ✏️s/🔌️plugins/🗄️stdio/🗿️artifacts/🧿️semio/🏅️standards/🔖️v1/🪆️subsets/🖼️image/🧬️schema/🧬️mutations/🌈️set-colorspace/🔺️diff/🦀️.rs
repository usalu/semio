//! 🔺️ Diff for `SetColorspace`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetColorspace, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::SetColorspace { colorspace } = payload;
    if base.colorspace == *colorspace {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Colorspace is already this value.".to_string());
    }
    protocol::MutationOutcome::new(SemioImageDiff { colorspace: (base.colorspace != *colorspace).then_some(*colorspace), ..Default::default() })
}
//#endregion 🔖️Diff
