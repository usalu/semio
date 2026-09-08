//! 💌️ SpaceIndexEditor commands command — `invite-member`. Members panel's invite-by-email
//! affordance (worker-brief task 3): relays `os.directory.upsert-member` (contract §C6) — the guest
//! never talks to the network directly, the shell's command funnel calls `DirectoryClient.command`.

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "invite-member")]
pub struct InviteMember {
    pub email: String,
    pub role: String,
}

pub fn handle(payload: &InviteMember, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.upsert-member".into(), args: Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": doc.snapshot.space_id.clone(), "email": payload.email.clone(), "role": payload.role.clone() }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
