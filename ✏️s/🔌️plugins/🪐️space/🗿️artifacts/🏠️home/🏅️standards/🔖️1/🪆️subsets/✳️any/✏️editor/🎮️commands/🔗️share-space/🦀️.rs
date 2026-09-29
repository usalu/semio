//! 🔗️ S Home launcher app command — `share-space` (member email + role → `os.directory.upsert-
//! member`, contract §C6). Companion "copy invite link" action lives at its own command leaf,
//! `🎮️commands/📋copy-invite-link` (one struct per `app_commands!` module, mirroring every other leaf
//! in this directory). An empty `email` (a raw row click) opens the declared `shareSpace` dialog; a
//! non-empty `email` (the dialog's own submit) relays the membership upsert to the hub — no optimistic
//! local mutation.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultOrigin};

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
/// 🚫️ The direct lane cannot tell a hub space from a local one: sharing runs only as the retained job.
pub fn handle(_payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Err(Fault::new(FaultOrigin::App, "s.home.share-space.requires-retained-job", "sharing a space runs only as the retained job"))
}

/// 🔗️ The retained route: `hub_row` is the space's one folded directory row from the job's captured projection — a
/// space without one is local-only and cannot be shared until it is promoted.
pub fn handle_with_row(payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>, hub_row: Option<&store::os_directory::DirectorySpace>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if hub_row.is_none() {
        let args = Some(pack::json_to_dsl_value(&pack::json!({
            "spaceId": payload.space_id.clone(),
            "dataClass": "ephemeralLocalOnly",
            "reason": "ephemeral-local-only"
        })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(128), dialog_id: "ephemeralShareBlocked".into(), args }));
    }
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
