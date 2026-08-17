//! 👁️️ SpaceIndexEditor commands command — `set-visibility`. Members panel's visibility toggle
//! (worker-brief task 3): relays `os.directory.set-visibility` (contract §C6).

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-visibility")]
pub struct SetVisibility {
    pub visibility: String,
}

pub fn handle(payload: &SetVisibility, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.set-visibility".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "spaceId": doc.snapshot.space_id, "visibility": payload.visibility }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};
    

    #[test]
    fn set_visibility_relays_the_directory_command() {
        let mut app = testkit::new_app();
        let result = app.dispatch_typed(SpaceIndexCommand::SetVisibility(SetVisibility { visibility: "public".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set visibility");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.directory.set-visibility");
                let args = semio_framework_os_kernel::pack_rt::dsl_value_to_json(args.clone().unwrap());
                assert_eq!(args.get("visibility").and_then(|v| v.as_str()), Some("public"));
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
