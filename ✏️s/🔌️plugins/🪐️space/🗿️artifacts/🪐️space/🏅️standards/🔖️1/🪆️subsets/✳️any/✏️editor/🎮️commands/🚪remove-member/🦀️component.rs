//! 🚪️ SpaceIndexEditor commands command — `remove-member`. Members panel's remove-member action
//! (worker-brief task 3): relays `os.directory.remove-member` (contract §C6).

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-member")]
pub struct RemoveMember {
    pub user_id: String,
}

pub async fn handle(payload: &RemoveMember, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.remove-member".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "spaceId": doc.snapshot.space_id, "userId": payload.user_id }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};
    

    #[semio_framework_async_macros::async_test]
    async fn remove_member_relays_remove_member() {
        let mut app = testkit::new_app();
        let result = app.dispatch_typed(SpaceIndexCommand::RemoveMember(RemoveMember { user_id: "u-1".into() }), &semio_framework_plugin::testkit::meta("local")).expect("remove");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.directory.remove-member");
                let args = semio_framework_os_kernel::pack_rt::dsl_value_to_json(args.clone().unwrap());
                assert_eq!(args.get("userId").and_then(|v| v.as_str()), Some("u-1"));
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
