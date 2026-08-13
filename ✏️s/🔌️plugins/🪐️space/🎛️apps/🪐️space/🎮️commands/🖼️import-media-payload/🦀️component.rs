//! 🖼️ 🖼️ S Studio app command — `import-media-payload`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};

use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "import-media-payload")]
pub struct ImportMediaPayload {
    pub payload: String,
}

pub fn handle(payload: &ImportMediaPayload, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let mut config_mutations = Vec::new();
    if let (Some(node_id), Some(format_name)) = (config.pending_import_node_id.as_ref(), config.pending_import_format.as_ref()) {
        let node_id = node_id.clone();
        let format_name = format_name.clone();
        config_mutations.push(SpaceConfigMutation::SetPendingImport { node_id: None, format: None });
        let format_kind = semio_framework::format_descriptor(&format_name).map(|d| d.short_id).unwrap_or_else(|| format_name.clone());
        use base64::Engine;
        let base64_part = payload.payload.split_once(',').map_or(payload.payload.as_str(), |(_, data)| data);
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
            if let Some(node) = doc.snapshot.graph.nodes.iter().find(|row| row.id == node_id) {
                // 📥️ Decoding/validation happens here; the decoded content is applied to the
                // node's own document-ref document by the host (a cross-document operation the
                // shell can't author from its own store), so this arm emits no studio document
                // operation.
                let _ = semio_framework_os::import_os_app_instance_media_kind(node, &bytes, &format_kind);
            }
        }
    }
    Ok(Emit::config(config_mutations))
}
