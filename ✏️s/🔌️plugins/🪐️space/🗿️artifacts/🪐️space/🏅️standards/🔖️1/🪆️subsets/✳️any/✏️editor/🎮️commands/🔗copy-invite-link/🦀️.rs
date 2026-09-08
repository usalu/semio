//! 🔗️ SpaceIndexEditor commands command — `copy-invite-link`. Members panel's copy-invite-link
//! action (worker-brief task 3): relays `os.directory.share-link` (contract §C6 — the shell's
//! `directoryCommandFromAction` sugars this action id into a `DirectoryCommand::CreateInvite`,
//! `📓️w2-c-report.md`'s "design decisions" #— then copies the redeemable link to the clipboard;
//! the guest never touches the clipboard or the network directly).

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "copy-invite-link")]
pub struct CopyInviteLink {
    pub role: String,
    pub ttl_secs: u64,
}

pub fn handle(payload: &CopyInviteLink, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.share-link".into(), args: Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": doc.snapshot.space_id.clone(), "role": payload.role.clone(), "ttlSecs": payload.ttl_secs }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
