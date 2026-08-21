//! 🔺️ Diff fragment yielded by `ClearDefaultApp` — real handcrafted construction from `base`,
//! never apply-then-capture: drops the entry for `(dialect, role)` if one exists, else a no-op
//! (identity) diff.

use super::super::super::OpeningPreferences;
use super::mutation::ClearDefaultApp;

//#region 🔖️Diff
pub fn diff(payload: &ClearDefaultApp, base: &OpeningPreferences) -> protocol::MutationOutcome<OpeningPreferences> {
    if !base.defaults.iter().any(|entry| entry.dialect == payload.dialect && entry.role == payload.role) {
        let role = match payload.role {
            semio_framework::AppRole::Viewer => "viewer",
            semio_framework::AppRole::Editor => "editor",
        };
        return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", format!("\"{}\" has no pinned default {} to clear.", payload.dialect.to_coordinate(), role));
    }
    let defaults = base.defaults.iter().filter(|entry| !(entry.dialect == payload.dialect && entry.role == payload.role)).cloned().collect();
    protocol::MutationOutcome::new(OpeningPreferences { defaults })
}
//#endregion 🔖️Diff
