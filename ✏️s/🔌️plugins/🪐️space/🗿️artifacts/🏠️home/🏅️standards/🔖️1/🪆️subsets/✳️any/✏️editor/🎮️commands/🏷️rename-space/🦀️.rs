//! 🏷️ S Home launcher app command — `rename-space`. An empty `name` (a raw row click) opens the
//! declared `renameSpace` dialog pre-seeded with the space's CURRENT name (read from the already-folded
//! directory read model, `HomeConfig::directory`); a non-empty `name` (the dialog's own submit) relays
//! the rename to the hub (contract §C6) — no optimistic local rename.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "rename-space")]
pub struct RenameSpace {
    pub space_id: String,
    pub name: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &RenameSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.name.trim().is_empty() {
        let current_name = cfg.snapshot.directory()?.spaces.get(&payload.space_id).map(|space| space.view.name.clone()).unwrap_or_default();
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "name": current_name })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(125), dialog_id: "renameSpace".into(), args }));
    }
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "name": payload.name.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.rename-space".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
