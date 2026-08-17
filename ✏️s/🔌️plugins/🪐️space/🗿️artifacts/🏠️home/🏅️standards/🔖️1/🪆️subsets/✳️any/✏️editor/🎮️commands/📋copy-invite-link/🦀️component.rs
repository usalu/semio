//! 📋️ S Home launcher app command — `copy-invite-link`, the `share-space` window's companion action
//! (contract §C6: `os.directory.share-link`, sugar for `create-invite`). The actual "copy to
//! clipboard" UI feedback for the minted invite token is shell-owned once the round trip completes —
//! the token does not exist until the hub mints it, so no `HostEffect` here can construct it
//! synchronously.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "copy-invite-link")]
pub struct CopyInviteLink {
    pub space_id: String,
    pub role: String,
    #[serde(default)]
    pub ttl_secs: u64,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &CopyInviteLink, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let role = if payload.role.trim().is_empty() { "spectator".to_string() } else { payload.role.clone() };
    let ttl_secs = if payload.ttl_secs == 0 { 3600 } else { payload.ttl_secs };
    let args = dsl::to_dsl_value(&json!({ "spaceId": payload.space_id, "role": role, "ttlSecs": ttl_secs })).ok();
    Ok(Emit::effect(HostEffect::ReplayShellCommand { action_id: "os.directory.share-link".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_invite_link_relays_share_link_with_default_ttl() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&CopyInviteLink { space_id: "sp-1".into(), role: "spectator".into(), ttl_secs: 0 }, &doc, &cfg).expect("handle");
        let (action_id, args) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
                _ => None,
            })
            .expect("a ReplayShellCommand effect");
        assert_eq!(action_id, "os.directory.share-link");
        let args_value: serde_json::Value = dsl::from_dsl_value(args.expect("args")).expect("json");
        assert_eq!(args_value["ttlSecs"], 3600);
        assert_eq!(args_value["spaceId"], "sp-1");
    }

    #[test]
    fn explicit_ttl_is_respected() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&CopyInviteLink { space_id: "sp-1".into(), role: "author".into(), ttl_secs: 60 }, &doc, &cfg).expect("handle");
        let args = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::ReplayShellCommand { args, .. } => args.clone(),
                _ => None,
            })
            .expect("args");
        let args_value: serde_json::Value = dsl::from_dsl_value(args).expect("json");
        assert_eq!(args_value["ttlSecs"], 60);
        assert_eq!(args_value["role"], "author");
    }
}
//#endregion 🧪️Tests
