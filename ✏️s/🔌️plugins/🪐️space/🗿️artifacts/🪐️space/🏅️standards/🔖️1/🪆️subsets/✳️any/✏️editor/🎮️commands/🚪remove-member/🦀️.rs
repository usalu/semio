//! 🚪️ SpaceIndexEditor commands command — `remove-member`. Members panel's remove-member action
//! (worker-brief task 3): relays `os.directory.remove-member` (contract §C6).

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-member")]
pub struct RemoveMember {
    pub user_id: String,
}

pub fn handle(payload: &RemoveMember, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.remove-member".into(), args: Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": doc.snapshot.space_id.clone(), "userId": payload.user_id.clone() }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
