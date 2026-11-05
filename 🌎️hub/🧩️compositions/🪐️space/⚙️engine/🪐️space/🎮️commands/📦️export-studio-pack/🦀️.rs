//! 💾️ 💾️ S Studio app command — `export-studio-pack`.

use semio_hub_space_foundation::config::{SpaceConfig, SpaceConfigMutation};
use semio_os::host::export_os_space_pack;
use semio_os::{WorkflowMutation, WorkflowSnapshot};
use semio_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "export-studio-pack")]
pub struct ExportStudioPack {}

pub fn handle(_payload: &ExportStudioPack, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let space_id = semio_hub_space_operations::resolve_future(semio_hub_space_operations::config_space_id(cfg.snapshot));
    match semio_hub_space_operations::resolve_future(crate::resolve_studio_document(&space_id)) {
        Some(document) => match export_os_space_pack(&document) {
            Ok(export) => {
                let stem = export.file_stem(&space_id);
                let pack_files = export.files;
                Ok(Emit {
                    effects: vec![
                        Effect::DownloadMediaExport { filename: format!("{stem}.pack"), mime_type: "application/octet-stream".into(), data: base64_codec::base64_standard_encode(&pack_files.pack), encoding: Some("base64".into()) },
                        Effect::DownloadMediaExport { filename: format!("{stem}.ops"), mime_type: "text/plain".into(), data: pack_files.ops, encoding: None },
                    ],
                    ..Default::default()
                })
            }
            Err(_) => Ok(Emit::default()),
        },
        None => Ok(Emit::default()),
    }
}
