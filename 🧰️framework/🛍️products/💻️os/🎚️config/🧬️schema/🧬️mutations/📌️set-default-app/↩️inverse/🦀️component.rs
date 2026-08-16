//! ↩️ Inverse for `SetDefaultApp` — reads the BASE entry for `(dialect, role)`, never the diff: a
//! prior pin restores to `SetDefaultApp`; no prior pin restores to `ClearDefaultApp` (the
//! coordinate was unpinned before this mutation ran).

use super::mutation::SetDefaultApp;
use super::super::super::OpeningPreferences;
use super::super::OpeningConfigMutation;
use super::super::clear_default_app::mutation::ClearDefaultApp;

//#region 🔖️Inverse
pub fn inverse(payload: &SetDefaultApp, base: &OpeningPreferences) -> Vec<OpeningConfigMutation> {
    match base.defaults.iter().find(|entry| entry.dialect == payload.dialect && entry.role == payload.role) {
        Some(prior) => vec![OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: payload.dialect.clone(), role: payload.role, app: prior.app.clone() })],
        None => vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect: payload.dialect.clone(), role: payload.role })],
    }
}
//#endregion 🔖️Inverse
