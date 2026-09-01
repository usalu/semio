//! 💾️ 💾️ S Studio app command — `import-space-pack`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};

use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-space-pack")]
pub struct ImportSpacePack {}

pub async fn handle(_payload: &ImportSpacePack, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(121), accept: ".pack".into(), read_as: Some("dataUrl".into()), import_action: "importSpacePackPayload".into(), multiple: false }))
}
