//! 🗑️ S Home launcher app command — `delete-space`. Two-phase, entirely from ONE command id: the
//! first dispatch (`confirmed: false`, the normal row-click default) emits the declared `deleteSpace`
//! confirm dialog and NEVER touches the network; only a SECOND dispatch with `confirmed: true` (the
//! dialog's own submit, re-carrying `spaceId`/`confirmed` from `OpenDialog`'s pre-seeded args — an
//! empty-`.args()` dialog degenerates to a plain confirm/cancel per `DialogDefinition`'s own doc)
//! emits the real `Effect::ReplayShellCommand` (contract §C6).

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-space")]
pub struct DeleteSpace {
    pub space_id: String,
    #[value(default)]
    pub confirmed: bool,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &DeleteSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if !payload.confirmed {
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "confirmed": true })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(127), dialog_id: "deleteSpace".into(), args }));
    }
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.delete-space".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
