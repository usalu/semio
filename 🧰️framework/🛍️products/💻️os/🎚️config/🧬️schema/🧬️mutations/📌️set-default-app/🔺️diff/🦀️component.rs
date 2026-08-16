//! 🔺️ Diff fragment yielded by `SetDefaultApp` — real handcrafted construction from `base`, never
//! apply-then-capture: drops any existing entry for the same `(dialect, role)` and appends the new
//! pin.

use super::mutation::SetDefaultApp;
use super::super::super::{DefaultApp, OpeningPreferences};

//#region 🔖️Diff
pub fn diff(payload: &SetDefaultApp, base: &OpeningPreferences) -> OpeningPreferences {
    let mut defaults: Vec<DefaultApp> = base
        .defaults
        .iter()
        .filter(|entry| !(entry.dialect == payload.dialect && entry.role == payload.role))
        .cloned()
        .collect();
    defaults.push(DefaultApp { dialect: payload.dialect.clone(), role: payload.role, app: payload.app.clone() });
    OpeningPreferences { defaults }
}
//#endregion 🔖️Diff
