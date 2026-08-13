//! 🖼️ 🖼️ S Studio app command — `import-media`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};

use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "import-media")]
pub struct ImportMedia {
    pub node_id: String,
    pub format: String,
}

pub fn handle(payload: &ImportMedia, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let format_kind = semio_framework::format_descriptor(&payload.format).map(|d| d.short_id).unwrap_or_else(|| payload.format.clone());
    let accept = semio_framework_os::media_accept_filter_kinds(&[format_kind.as_str()]);
    let accept = if accept.is_empty() { format!(".{format_kind}") } else { accept };
    Ok(Emit {
        config_mutations: vec![SpaceConfigMutation::SetPendingImport { node_id: Some(payload.node_id.clone()), format: Some(format_kind.clone()) }],
        effects: vec![HostEffect::RequestFileOpen { accept, read_as: Some("dataUrl".into()), import_action: "importMediaPayload".into(), multiple: false }],
        ..Default::default()
    })
}
