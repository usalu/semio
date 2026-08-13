//! 🖼️ 🖼️ S Studio app command — `export-media`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use crate::apps::space::engine::{workflow_parameter_bindings_to_os, workflow_parameters_to_os};
use semio_framework_os::{materialize_os_app_instance_document_json, os_app_registration, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};
use serde_json::{json, Value};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-media")]
pub struct ExportMedia {
    pub node_id: String,
    pub format: String,
}

pub fn handle(payload: &ExportMedia, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let projection = doc.snapshot;
    match projection.graph.nodes.iter().find(|row| row.id == payload.node_id) {
        Some(node) => {
            crate::ensure_space_fixtures_registered();
            let schema = os_app_registration(&node.plugin_id, &node.app_id).map(|row| row.source_format).unwrap_or_default();
            let document_json = materialize_os_app_instance_document_json(&json!({ "schema": schema }).to_string(), &node.id, &workflow_parameter_bindings_to_os(&projection.parameter_bindings), &workflow_parameters_to_os(&projection.parameters));
            let document_value: Value = serde_json::from_str(&document_json).unwrap_or_else(|_| json!({}));
            let format_kind = semio_framework::format_descriptor(&payload.format).map(|d| d.short_id).unwrap_or_else(|| payload.format.clone());
            match semio_framework_os::export_os_app_instance_media_kind(node, &document_value, &format_kind) {
                Ok(result) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: result.file_name, mime_type: result.mime_type, data: result.data, encoding: result.encoding })),
                Err(_) => Ok(Emit::default()),
            }
        }
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::space::testkit::{apply_config, studio_emit};
    use crate::apps::space::SpaceCommand;
    use crate::demo_space_projection;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportMedia(ExportMedia { node_id: "n1".into(), format: "dwg".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMedia(crate::apps::space::commands::import_media::ImportMedia { node_id: "n1".into(), format: "dwg".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMediaPayload(crate::apps::space::commands::import_media_payload::ImportMediaPayload { payload: "data:...".into() }));
    }

    #[test]
    fn export_media_emits_download_effect_and_import_requests_file_open() {
        use base64::Engine;
        crate::apps::space::testkit::seed_draw_plugin();
        semio_framework_os::workflow::register_os_media_export_handler_kind("2d.drawing", "dwg", |_doc| {
            let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
            let bytes = semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&drawing)?;
            Ok(semio_framework_os::OsMediaExportResult { data: base64::engine::general_purpose::STANDARD.encode(bytes), mime_type: "image/vnd.dwg".into(), file_name: "draw.dwg".into(), encoding: Some("base64".into()) })
        });
        // 🚪️ Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1:
        // `register_dwg_import_handler` is deleted (see host `🦀️component.rs`'s `media_export_raster`
        // module) -- it was a thin, always-`"dwg"` wrapper over `register_os_media_import_handler_kind`
        // (which stays; it is a domain-neutral format-agnostic primitive, not one of this wave's five
        // targeted functions). This test's `"2d.drawing"` kind is a synthetic test-only stand-in for
        // the real `🖍️draw` artifact (never a registered `ArtifactKindSpec`), so it has no real
        // `ComposerEntry` to migrate to -- inlined here rather than deleted, since the test below
        // exercises `SpaceCommand::ImportMedia`'s effect-producing behaviour, not the DWG bridge itself.
        semio_framework_os::workflow::register_os_media_import_handler_kind("2d.drawing", "dwg", |bytes| {
            let _drawing = semio_s_plugin_stdio::artifacts::dwg::dwg_from_bytes(bytes)?;
            Ok(json!({ "schema": "draw.document", "imported": true }))
        });

        let projection = demo_space_projection();
        let node = projection.graph.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
        let config = SpaceConfig::default();

        let export = studio_emit(&projection, &config, &SpaceCommand::ExportMedia(ExportMedia { node_id: node.id.clone(), format: "stdio.dwg".into() })).expect("handle");
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

        let import = studio_emit(&projection, &config, &SpaceCommand::ImportMedia(crate::apps::space::commands::import_media::ImportMedia { node_id: node.id.clone(), format: "stdio.dwg".into() })).expect("handle");
        assert!(import.effects.iter().any(|effect| matches!(effect, HostEffect::RequestFileOpen { import_action, accept, .. } if import_action == "importMediaPayload" && accept.contains(".dwg"))));
        assert_eq!(import.config_mutations, vec![SpaceConfigMutation::SetPendingImport { node_id: Some(node.id), format: Some("dwg".into()) }]);

        let pending_config = apply_config(&config, &import.config_mutations);
        let payload = studio_emit(&projection, &pending_config, &SpaceCommand::ImportMediaPayload(crate::apps::space::commands::import_media_payload::ImportMediaPayload { payload: format!("data:image/vnd.dwg;base64,{data}") })).expect("handle");
        assert!(payload.artifact_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
