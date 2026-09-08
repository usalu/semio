//! 🏛️ S Home launcher app command — `manage-space`. Opens the Shell-owned Administration pane for
//! exactly one space. It carries no role, no capability, and no page: the pane the host effect opens
//! renders solely from the hub's own canonical `DirectorySpaceAdministrationPageV1`, so a client that
//! reached this action without authority still gets a server denial rather than an administration UI.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "manage-space")]
pub struct ManageSpace {
    pub space_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &ManageSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.space_id.trim().is_empty() {
        return Err(Fault::from("s.home.manage-space-requires-a-space"));
    }
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.open-administration".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
