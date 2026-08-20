//! 🔺️ Diff fragment yielded by `ClearDefaultApp` — real handcrafted construction from `base`,
//! never apply-then-capture: drops the entry for `(dialect, role)` if one exists, else a no-op
//! (identity) diff.

use super::mutation::ClearDefaultApp;
use super::super::super::OpeningPreferences;

//#region 🔖️Diff
pub async fn diff(payload: &ClearDefaultApp, base: &OpeningPreferences) -> protocol::MutationOutcome<OpeningPreferences> {
    if !base.defaults.iter().any(|entry| entry.dialect == payload.dialect && entry.role == payload.role) {
        return protocol::MutationOutcome::new(base.clone()).await.warn("mutation.no-op", format!("\"{}\" has no pinned default {} to clear.", payload.dialect.to_coordinate(), payload.role.as_str().await)).await;
    }
    let defaults = base.defaults.iter().filter(|entry| !(entry.dialect == payload.dialect && entry.role == payload.role)).cloned().collect();
    protocol::MutationOutcome::new(OpeningPreferences { defaults }).await
}
//#endregion 🔖️Diff
