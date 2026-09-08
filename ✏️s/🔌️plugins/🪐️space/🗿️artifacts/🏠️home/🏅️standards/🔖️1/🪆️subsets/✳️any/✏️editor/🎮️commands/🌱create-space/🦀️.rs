//! 🌱️ S Home launcher app command — `create-space`. Contract §C6: the plugin never touches the
//! network — a validated create request is relayed to the shell as `Effect::ReplayShellCommand`;
//! the resulting `space.created` event returns over `/directory/ws` and is folded back in by
//! `fold-directory-events`. No optimistic mutation of the read model. An empty (raw toolbar click, not
//! yet a dialog submit) `name` opens the declared `createSpace` dialog instead of relaying — the local-
//! only "create ephemeral studio" path (`create-studio`) is untouched and still works with no hub.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "create-space")]
pub struct CreateSpace {
    pub name: String,
    pub kind: String,
    pub visibility: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &CreateSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.name.trim().is_empty() {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(123), dialog_id: "createSpace".into(), args: None }));
    }
    let kind = if payload.kind.trim().is_empty() { "atelier".to_string() } else { payload.kind.clone() };
    let visibility = if payload.visibility.trim().is_empty() { "private".to_string() } else { payload.visibility.clone() };
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "name": payload.name.clone(), "spaceKind": kind, "visibility": visibility })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.create-space".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
