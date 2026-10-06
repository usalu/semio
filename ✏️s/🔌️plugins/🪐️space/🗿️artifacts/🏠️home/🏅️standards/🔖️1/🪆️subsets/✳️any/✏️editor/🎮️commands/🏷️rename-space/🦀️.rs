//! 🏷️ S Home launcher app command — `rename-space`. An empty `name` (a raw row click) opens the
//! declared `renameSpace` dialog pre-seeded with the space's CURRENT name (read from the one folded directory row the
//! retained job's captured transient projection holds for it); a non-empty `name` (the dialog's own submit) relays
//! the rename to the hub (contract §C6) — no optimistic local rename.

use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultOrigin};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "rename-space")]
pub struct RenameSpace {
    pub space_id: String,
    pub name: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
/// 🚫️ The direct lane has no directory projection to seed the dialog from: renaming runs only as the retained job.
pub fn handle(_payload: &RenameSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Err(Fault::new(FaultOrigin::App, "s.home.rename-space.requires-retained-job", "renaming a space runs only as the retained job"))
}

/// 🏷️ The retained route: `row` is the space's one folded directory row from the job's captured projection, if listed.
pub fn handle_with_row(payload: &RenameSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>, row: Option<&store::os_directory::DirectorySpace>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.name.trim().is_empty() {
        let current_name = row.map(|space| space.view.name.clone()).unwrap_or_default();
        let args = Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "spaceId": payload.space_id.clone(), "name": current_name })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(125), dialog_id: "renameSpace".into(), args }));
    }
    let args = Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "spaceId": payload.space_id.clone(), "name": payload.name.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.rename-space".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
