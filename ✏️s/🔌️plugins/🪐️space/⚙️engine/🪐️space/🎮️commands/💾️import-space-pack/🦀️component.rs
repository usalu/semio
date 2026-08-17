//! 💾️ 💾️ S Studio app command — `import-space-pack`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};

use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "import-space-pack")]
pub struct ImportSpacePack {}

pub fn handle(_payload: &ImportSpacePack, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".pack".into(), read_as: Some("dataUrl".into()), import_action: "importSpacePackPayload".into(), multiple: false }))
}
