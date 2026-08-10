//! 👥️ S Studio app — local presence heartbeat command.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowSnapshot, WorkflowMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};

//#region 🔖️PresenceHeartbeat
pub mod presence_heartbeat {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "presence-heartbeat")]
    pub struct PresenceHeartbeat {
        pub client_id: String,
        pub name: String,
    }

    /// 🐢️ A heartbeat only records this client's own identity for the presence broadcast — it must
    /// declare `None` `ui_scope` so it never triggers a full-shell `refresh-ui` for the sending client.
    pub fn handle(payload: &PresenceHeartbeat, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let config_mutations = vec![SpaceConfigMutation::SetClient { client_id: Some(payload.client_id.clone()), client_name: Some(payload.name.clone()) }];
        Ok(Emit { config_mutations, ui_scope: semio_framework::kernel::UiDirtyScope::None, ..Default::default() })
    }
}
//#endregion 🔖️PresenceHeartbeat

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        use crate::apps::space::SpaceCommand;
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat { client_id: "c1".into(), name: "Ada".into() }));
    }

    #[test]
    fn presence_heartbeat_declares_none_ui_scope() {
        use crate::apps::space::testkit::studio_emit;
        use crate::apps::space::SpaceCommand;
        use crate::demo_space_projection;
        use semio_framework::kernel::UiDirtyScope;
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat { client_id: "client-test-c".into(), name: "Cass".into() })).expect("handle");
        assert!(matches!(emit.ui_scope, UiDirtyScope::None), "presenceHeartbeat must declare None, got {:?}", emit.ui_scope);
    }
}
//#endregion 🧪️Tests
