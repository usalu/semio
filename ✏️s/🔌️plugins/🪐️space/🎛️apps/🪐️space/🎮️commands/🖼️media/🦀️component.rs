//! 🖼️ S Studio app — per-instance media export/import commands.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use crate::apps::space::engine::{workflow_parameter_bindings_to_os, workflow_parameters_to_os};
use semio_framework_os::{materialize_os_app_instance_document_json, os_app_registration, WorkflowSnapshot, WorkflowMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};
use serde_json::{json, Value};

//#region 🔖️ExportMedia
pub mod export_media {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-media")]
    pub struct ExportMedia {
        pub node_id: String,
        pub format: String,
    }

    pub fn handle(payload: &ExportMedia, doc: &DocumentView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let projection = doc.snapshot;
        match projection.graph.nodes.iter().find(|row| row.id == payload.node_id) {
            Some(node) => {
                crate::ensure_space_fixtures_registered();
                let schema = os_app_registration(&node.plugin_id, &node.app_id).map(|row| row.source_format).unwrap_or_default();
                let document_json = materialize_os_app_instance_document_json(&json!({ "schema": schema }).to_string(), &node.id, &workflow_parameter_bindings_to_os(&projection.parameter_bindings), &workflow_parameters_to_os(&projection.parameters));
                let document_value: Value = serde_json::from_str(&document_json).unwrap_or_else(|_| json!({}));
                let format_kind = semio_framework::normalize_stdio_format_kind(&payload.format).unwrap_or(payload.format.as_str());
                match semio_framework_os::export_os_app_instance_media_kind(node, &document_value, format_kind) {
                    Ok(result) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: result.file_name, mime_type: result.mime_type, data: result.data, encoding: result.encoding })),
                    Err(_) => Ok(Emit::default()),
                }
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportMedia

//#region 🔖️ImportMedia
pub mod import_media {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-media")]
    pub struct ImportMedia {
        pub node_id: String,
        pub format: String,
    }

    pub fn handle(payload: &ImportMedia, _doc: &DocumentView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let format_kind = semio_framework::normalize_stdio_format_kind(&payload.format).unwrap_or(payload.format.as_str());
        let accept = semio_framework_os::media_accept_filter_kinds(&[format_kind]);
        let accept = if accept.is_empty() { format!(".{format_kind}") } else { accept };
        Ok(Emit {
            config_mutations: vec![SpaceConfigMutation::SetPendingImport { node_id: Some(payload.node_id.clone()), format: Some(format_kind.to_string()) }],
            effects: vec![HostEffect::RequestFileOpen { accept, read_as: Some("dataUrl".into()), import_action: "importMediaPayload".into(), multiple: false }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️ImportMedia

//#region 🔖️ImportMediaPayload
pub mod import_media_payload {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-media-payload")]
    pub struct ImportMediaPayload {
        pub payload: String,
    }

    pub fn handle(payload: &ImportMediaPayload, doc: &DocumentView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let mut config_mutations = Vec::new();
        if let (Some(node_id), Some(format_name)) = (config.pending_import_node_id.as_ref(), config.pending_import_format.as_ref()) {
            let node_id = node_id.clone();
            let format_name = format_name.clone();
            config_mutations.push(SpaceConfigMutation::SetPendingImport { node_id: None, format: None });
            let format_kind = semio_framework::normalize_stdio_format_kind(&format_name).unwrap_or(format_name.as_str());
            use base64::Engine;
            let base64_part = payload.payload.split_once(',').map_or(payload.payload.as_str(), |(_, data)| data);
            if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
                if let Some(node) = doc.snapshot.graph.nodes.iter().find(|row| row.id == node_id) {
                    // 📥️ Decoding/validation happens here; the decoded content is applied to the
                    // node's own document-ref document by the host (a cross-document operation the
                    // shell can't author from its own store), so this arm emits no studio document
                    // operation.
                    let _ = semio_framework_os::import_os_app_instance_media_kind(node, &bytes, format_kind);
                }
            }
        }
        Ok(Emit::config(config_mutations))
    }
}
//#endregion 🔖️ImportMediaPayload

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::space::testkit::{apply_config, studio_emit};
    use crate::apps::space::SpaceCommand;
    use crate::demo_space_projection;
    use semio_framework::MediaFormat;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportMedia(export_media::ExportMedia { node_id: "n1".into(), format: "dwg".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMedia(import_media::ImportMedia { node_id: "n1".into(), format: "dwg".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMediaPayload(import_media_payload::ImportMediaPayload { payload: "data:...".into() }));
    }

    #[test]
    fn export_media_emits_download_effect_and_import_requests_file_open() {
        use base64::Engine;
        crate::apps::space::testkit::seed_draw_plugin();
        semio_framework_os::register_os_media_export_handler("2d.drawing", MediaFormat::Dwg, |_doc| {
            let drawing = semio_framework_os::DwgDrawing::default();
            let bytes = semio_framework_os::dwg_to_bytes(&drawing)?;
            Ok(semio_framework_os::OsMediaExportResult { data: base64::engine::general_purpose::STANDARD.encode(bytes), mime_type: MediaFormat::Dwg.mime_type().into(), file_name: "draw.dwg".into(), encoding: Some("base64".into()) })
        });
        semio_framework_os::register_dwg_import_handler("2d.drawing", |_drawing| Ok(json!({ "schema": "draw.document", "imported": true })));

        let projection = demo_space_projection();
        let node = projection.graph.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
        let config = SpaceConfig::default();

        let export = studio_emit(&projection, &config, &SpaceCommand::ExportMedia(export_media::ExportMedia { node_id: node.id.clone(), format: "stdio.dwg".into() })).expect("handle");
        let (data, encoding) = export
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::DownloadMediaExport { data, encoding, .. } => Some((data.clone(), encoding.clone())),
                _ => None,
            })
            .expect("DownloadMediaExport effect");
        assert!(!data.is_empty());
        assert_eq!(encoding.as_deref(), Some("base64"));

        let import = studio_emit(&projection, &config, &SpaceCommand::ImportMedia(import_media::ImportMedia { node_id: node.id.clone(), format: "stdio.dwg".into() })).expect("handle");
        assert!(import.effects.iter().any(|effect| matches!(effect, HostEffect::RequestFileOpen { import_action, accept, .. } if import_action == "importMediaPayload" && accept.contains(".dwg"))));
        assert_eq!(import.config_mutations, vec![SpaceConfigMutation::SetPendingImport { node_id: Some(node.id), format: Some("dwg".into()) }]);

        let pending_config = apply_config(&config, &import.config_mutations);
        let payload = studio_emit(&projection, &pending_config, &SpaceCommand::ImportMediaPayload(import_media_payload::ImportMediaPayload { payload: format!("data:image/vnd.dwg;base64,{data}") })).expect("handle");
        assert!(payload.document_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
