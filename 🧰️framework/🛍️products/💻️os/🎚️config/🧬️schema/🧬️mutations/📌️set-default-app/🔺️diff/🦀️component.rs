//! 🔺️ Diff fragment yielded by `SetDefaultApp` — real handcrafted construction from `base`, never
//! apply-then-capture: drops any existing entry for the same `(dialect, role)` and appends the new
//! pin.

use super::mutation::SetDefaultApp;
use super::super::super::{DefaultApp, OpeningPreferences};

//#region 🔖️Diff
pub fn diff(payload: &SetDefaultApp, base: &OpeningPreferences) -> protocol::MutationOutcome<OpeningPreferences> {
    if base.defaults.iter().any(|entry| entry.dialect == payload.dialect && entry.role == payload.role && entry.app == payload.app) {
        return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", format!("\"{}\" is already the default {} for \"{}\".", payload.app.app_id, payload.role.as_str(), payload.dialect.to_coordinate()));
    }
    let mut defaults: Vec<DefaultApp> = base
        .defaults
        .iter()
        .filter(|entry| !(entry.dialect == payload.dialect && entry.role == payload.role))
        .cloned()
        .collect();
    defaults.push(DefaultApp { dialect: payload.dialect.clone(), role: payload.role, app: payload.app.clone() });
    protocol::MutationOutcome::new(OpeningPreferences { defaults })
}
//#endregion 🔖️Diff
