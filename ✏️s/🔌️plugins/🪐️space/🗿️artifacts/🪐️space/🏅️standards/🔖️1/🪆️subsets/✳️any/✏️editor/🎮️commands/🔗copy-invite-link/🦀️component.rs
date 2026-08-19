//! 🔗️ SpaceIndexEditor commands command — `copy-invite-link`. Members panel's copy-invite-link
//! action (worker-brief task 3): relays `os.directory.share-link` (contract §C6 — the shell's
//! `directoryCommandFromAction` sugars this action id into a `DirectoryCommand::CreateInvite`,
//! `📓️w2-c-report.md`'s "design decisions" #— then copies the redeemable link to the clipboard;
//! the guest never touches the clipboard or the network directly).

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "copy-invite-link")]
pub struct CopyInviteLink {
    pub role: String,
    pub ttl_secs: u64,
}

pub async fn handle(payload: &CopyInviteLink, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.share-link".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "spaceId": doc.snapshot.space_id, "role": payload.role, "ttlSecs": payload.ttl_secs }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};
    

    #[test]
    async fn copy_invite_link_relays_share_link() {
        let mut app = testkit::new_app();
        let result = app.dispatch_typed(SpaceIndexCommand::CopyInviteLink(CopyInviteLink { role: "spectator".into(), ttl_secs: 3600 }), &semio_framework_plugin::testkit::meta("local")).expect("copy link");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.directory.share-link");
                let args = semio_framework_os_kernel::pack_rt::dsl_value_to_json(args.clone().unwrap());
                assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("spectator"));
                // 🔢️ `DslValue`'s numeric lane round-trips through f64 (confirmed empirically: a JSON
                // `u64` comes back as `3600.0`, not `3600`) — `serde_json::Number::as_u64()` only
                // succeeds for values that were themselves parsed/stored as an unsigned integer, so
                // asserting via `.as_f64()` is the honest check here, not a workaround.
                assert_eq!(args.get("ttlSecs").and_then(|v| v.as_f64()), Some(3600.0));
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
