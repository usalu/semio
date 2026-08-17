//! 💾️ 💾️ S Studio app command — `export-studio-pack`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::host::{export_os_space_pack};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, Effect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-studio-pack")]
pub struct ExportStudioPack {}

pub fn handle(_payload: &ExportStudioPack, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let space_id = crate::engine::space::config_space_id(cfg.snapshot);
    match crate::resolve_studio_document(&space_id) {
        Some(document) => match export_os_space_pack(&document) {
            Ok(pack_files) => {
                use base64::Engine;
                Ok(Emit {
                    effects: vec![
                        Effect::DownloadMediaExport { filename: format!("{space_id}.pack"), mime_type: "application/octet-stream".into(), data: base64::engine::general_purpose::STANDARD.encode(&pack_files.pack), encoding: Some("base64".into()) },
                        Effect::DownloadMediaExport { filename: format!("{space_id}.ops"), mime_type: "text/plain".into(), data: pack_files.ops, encoding: None },
                    ],
                    ..Default::default()
                })
            }
            Err(_) => Ok(Emit::default()),
        },
        None => Ok(Emit::default()),
    }
}
