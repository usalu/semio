//! 💾️ 💾️ S Studio app command — `export-studio-dsl`.

use semio_hub_space_foundation::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::host::export_os_space_dsl;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "export-studio-dsl")]
pub struct ExportStudioDsl {}

pub fn handle(_payload: &ExportStudioDsl, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let space_id = semio_hub_space_operations::resolve_future(semio_hub_space_operations::config_space_id(cfg.snapshot));
    match semio_hub_space_operations::resolve_future(crate::resolve_studio_document(&space_id)) {
        Some(document) => match export_os_space_dsl(&document) {
            Ok(text_files) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{space_id}.os"), mime_type: "text/plain".into(), data: text_files.dsl, encoding: None })),
            Err(_) => Ok(Emit::default()),
        },
        None => Ok(Emit::default()),
    }
}
