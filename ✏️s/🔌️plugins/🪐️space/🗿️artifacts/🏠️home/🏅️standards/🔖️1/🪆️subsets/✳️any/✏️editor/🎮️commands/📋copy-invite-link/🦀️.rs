//! 📋️ S Home launcher app command — `copy-invite-link`, the `share-space` window's companion action
//! (contract §C6: `os.directory.share-link`, sugar for `create-invite`). The actual "copy to
//! clipboard" UI feedback for the minted invite token is shell-owned once the round trip completes —
//! the token does not exist until the hub mints it, so no `Effect` here can construct it
//! synchronously.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "copy-invite-link")]
pub struct CopyInviteLink {
    pub space_id: String,
    pub role: String,
    #[value(default)]
    pub ttl_secs: u64,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &CopyInviteLink, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let role = if payload.role.trim().is_empty() { "spectator".to_string() } else { payload.role.clone() };
    let ttl_secs = if payload.ttl_secs == 0 { 3600 } else { payload.ttl_secs };
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "role": role, "ttlSecs": ttl_secs })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.share-link".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
