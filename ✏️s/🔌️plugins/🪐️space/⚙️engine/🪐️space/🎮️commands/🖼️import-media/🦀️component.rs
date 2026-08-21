//! 🖼️ 🖼️ S Studio app command — `import-media`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};

use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "import-media")]
pub struct ImportMedia {
    pub node_id: String,
    pub format: String,
}

pub async fn handle(payload: &ImportMedia, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let format_kind = semio_framework::format_descriptor(&payload.format)
        .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.format"), error.to_string()))?
        .map(|descriptor| descriptor.short_id)
        .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.format"), format!("unknown media format `{}`", payload.format)))?;
    let accept = semio_framework_os::media_accept_filter_kinds(&[format_kind.as_str()]).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.format"), error.to_string()))?;
    Ok(Emit {
        config_mutations: vec![SpaceConfigMutation::SetPendingImport { node_id: Some(payload.node_id.clone()), format: Some(format_kind.clone()) }],
        effects: vec![Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(122), accept, read_as: Some("dataUrl".into()), import_action: "importMediaPayload".into(), multiple: false }],
        ..Default::default()
    })
}
