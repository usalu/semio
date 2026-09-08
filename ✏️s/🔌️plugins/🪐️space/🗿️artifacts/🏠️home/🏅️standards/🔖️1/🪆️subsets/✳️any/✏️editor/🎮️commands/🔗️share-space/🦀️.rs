//! 🔗️ S Home launcher app command — `share-space` (member email + role → `os.directory.upsert-
//! member`, contract §C6). Companion "copy invite link" action lives at its own command leaf,
//! `🎮️commands/📋copy-invite-link` (one struct per `app_commands!` module, mirroring every other leaf
//! in this directory). An empty `email` (a raw row click) opens the declared `shareSpace` dialog; a
//! non-empty `email` (the dialog's own submit) relays the membership upsert to the hub — no optimistic
//! local mutation.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "share-space")]
pub struct ShareSpace {
    pub space_id: String,
    pub email: String,
    pub role: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.email.trim().is_empty() {
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(126), dialog_id: "shareSpace".into(), args }));
    }
    let role = if payload.role.trim().is_empty() { "spectator".to_string() } else { payload.role.clone() };
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "email": payload.email.clone(), "role": role })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.upsert-member".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
