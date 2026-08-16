//! 🔺️ Diff fragment yielded by `ClearDefaultApp` — real handcrafted construction from `base`,
//! never apply-then-capture: drops the entry for `(dialect, role)` if one exists, else a no-op
//! (identity) diff.

use super::mutation::ClearDefaultApp;
use super::super::super::OpeningPreferences;

//#region 🔖️Diff
pub fn diff(payload: &ClearDefaultApp, base: &OpeningPreferences) -> OpeningPreferences {
    let defaults = base.defaults.iter().filter(|entry| !(entry.dialect == payload.dialect && entry.role == payload.role)).cloned().collect();
    OpeningPreferences { defaults }
}
//#endregion 🔖️Diff
