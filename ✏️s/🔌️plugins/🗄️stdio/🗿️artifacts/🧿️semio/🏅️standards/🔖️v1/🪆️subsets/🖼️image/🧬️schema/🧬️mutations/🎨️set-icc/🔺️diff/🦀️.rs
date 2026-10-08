//! 🔺️ Diff for `SetIcc`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetIcc, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::SetIcc { icc } = payload;
    if &base.icc == icc {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "ICC profile is already this value.".to_string());
    }
    protocol::MutationOutcome::new(SemioImageDiff { icc: (base.icc != *icc).then_some(icc.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
