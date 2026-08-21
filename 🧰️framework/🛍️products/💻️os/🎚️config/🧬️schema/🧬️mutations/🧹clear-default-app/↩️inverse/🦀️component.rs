//! ↩️ Inverse for `ClearDefaultApp` — reads the BASE entry for `(dialect, role)`, never the diff:
//! a prior pin restores via `SetDefaultApp`; nothing to restore ⇒ no-op (`Vec::new()`, never a
//! `NoMutation` sentinel).

use super::super::super::OpeningPreferences;
use super::super::set_default_app::mutation::SetDefaultApp;
use super::super::OpeningConfigMutation;
use super::mutation::ClearDefaultApp;

//#region 🔖️Inverse
pub fn inverse(payload: &ClearDefaultApp, base: &OpeningPreferences) -> Vec<OpeningConfigMutation> {
    base.defaults
        .iter()
        .find(|entry| entry.dialect == payload.dialect && entry.role == payload.role)
        .map(|prior| vec![OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: payload.dialect.clone(), role: payload.role, app: prior.app.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
