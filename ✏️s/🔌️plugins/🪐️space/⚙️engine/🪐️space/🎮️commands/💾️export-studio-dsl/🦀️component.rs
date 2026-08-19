//! 💾️ 💾️ S Studio app command — `export-studio-dsl`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::host::{export_os_space_dsl};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, Effect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-studio-dsl")]
pub struct ExportStudioDsl {}

pub async fn handle(_payload: &ExportStudioDsl, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let space_id = crate::engine::space::config_space_id(cfg.snapshot);
    match crate::resolve_studio_document(&space_id) {
        Some(document) => match export_os_space_dsl(&document) {
            Ok(text_files) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{space_id}.os"), mime_type: "text/plain".into(), data: text_files.dsl, encoding: None })),
            Err(_) => Ok(Emit::default()),
        },
        None => Ok(Emit::default()),
    }
}
