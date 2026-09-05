//! 🖼️ 🖼️ S Studio app command — `export-media`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use crate::engine::space::engine::{resolve_future, workflow_parameter_bindings_to_os, workflow_parameters_to_os};
use pack::json;
use semio_framework_os::{materialize_os_app_instance_document_json, os_app_registration, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{plugin_app_close_prelude::Value, ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
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
            let schema_json = json::object([("schema".to_string(), json::Value::from(schema.as_str()))]).to_string();
            let bindings = resolve_future(workflow_parameter_bindings_to_os(&projection.parameter_bindings));
            let parameters = resolve_future(workflow_parameters_to_os(&projection.parameters));
            let document_json = materialize_os_app_instance_document_json(&schema_json, &node.id, &bindings, &parameters);
            // 🌉️ `export_os_app_instance_media_kind` is a legacy os-host boundary that still takes a
            // `serde_json::Value` (framework gap — no `ToValue`/`FromValue` bridge exists for it, see
            // ticket dossier). `semio_framework_plugin::plugin_app_close_prelude::Value` re-exports the
            // exact same `serde_json::Value` type without this crate taking its own runtime `serde_json`
            // dependency; `str::parse` reaches `serde_json`'s own `FromStr` impl on that type.
            let document_value: Value = document_json.parse().unwrap_or_else(|_| Value::Object(Default::default()));
            let format_kind = semio_framework::format_descriptor(&payload.format)
                .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.format"), error.to_string()))?
                .map(|descriptor| descriptor.short_id)
                .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.format"), format!("unknown media format `{}`", payload.format)))?;
            let result = semio_framework_os::export_os_app_instance_media_kind(node, &document_value, &format_kind).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.export"), error))?;
            Ok(Emit::effect(Effect::DownloadMediaExport { filename: result.file_name, mime_type: result.mime_type, data: result.data, encoding: result.encoding }))
        }
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo_space_projection;
    use crate::engine::space::testkit::{apply_config, studio_emit};
    use crate::engine::space::SpaceCommand;
    use serde_json::json;

    #[semio_framework_async_macros::async_test]
    async fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportMedia(ExportMedia { node_id: "n1".into(), format: "dwg".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMedia(crate::engine::space::commands::import_media::ImportMedia { node_id: "n1".into(), format: "dwg".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMediaPayload(crate::engine::space::commands::import_media_payload::ImportMediaPayload { payload: "data:...".into() }));
    }

    /// 🪪️ A neutral format id keeps this command law about typed effects rather than linking an
    /// unrelated full Stdio codec catalog into Space's four-family Home-I/O build.
    const DWG_FORMAT_ID: &str = "s.stdio.dwg.standard.ac1018.representation.document";

    #[semio_framework_async_macros::async_test]
    async fn export_media_emits_download_effect_and_import_requests_file_open() {
        crate::engine::space::testkit::seed_draw_plugin();
        semio_framework::register_format_descriptors([semio_framework::FormatDescriptor {
            kind_id: DWG_FORMAT_ID.into(),
            short_id: DWG_FORMAT_ID.into(),
            aliases: Vec::new(),
            mimes: vec!["image/vnd.dwg".into()],
            extensions: vec![".dwg".into()],
            name: "Drawing exchange".into(),
            full_name: "Drawing exchange test carrier".into(),
            neutral: false,
            dir_name: "dwg".into(),
            is_binary: true,
        }])
        .expect("register neutral format descriptor");
        semio_framework_os::workflow::register_os_media_export_handler_kind("2d.drawing", DWG_FORMAT_ID, |_doc| {
            Ok(semio_framework_os::OsMediaExportResult { data: base64_codec::base64_standard_encode(b"space-home-io-test"), mime_type: "image/vnd.dwg".into(), file_name: "draw.dwg".into(), encoding: Some("base64".into()) })
        });
        // 🚪️ Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1:
        // `register_dwg_import_handler` is deleted (see host `🦀️.rs`'s `media_export_raster`
        // module) -- it was a thin, always-`"dwg"` wrapper over `register_os_media_import_handler_kind`
        // (which stays; it is a domain-neutral format-agnostic primitive, not one of this wave's five
        // targeted functions). This test's `"2d.drawing"` kind is a synthetic test-only stand-in for
        // the real `🖍️draw` artifact (never a registered `ArtifactKindSpec`), so it has no real
        // `ComposerEntry` to migrate to -- inlined here rather than deleted, since the test below
        // exercises `SpaceCommand::ImportMedia`'s effect-producing behaviour, not the DWG bridge itself.
        semio_framework_os::workflow::register_os_media_import_handler_kind("2d.drawing", DWG_FORMAT_ID, |bytes| {
            if bytes != b"space-home-io-test" {
                return Err("unexpected neutral carrier bytes".into());
            }
            Ok(json!({ "schema": "draw.document", "imported": true }))
        });

        let projection = demo_space_projection().await;
        let node = projection.graph.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
        let config = SpaceConfig::default();

        let export = studio_emit(&projection, &config, &SpaceCommand::ExportMedia(ExportMedia { node_id: node.id.clone(), format: DWG_FORMAT_ID.into() })).await.expect("handle");
        let (data, encoding) = export
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::DownloadMediaExport { data, encoding, .. } => Some((data.clone(), encoding.clone())),
                _ => None,
            })
            .expect("DownloadMediaExport effect");
        assert!(!data.is_empty());
        assert_eq!(encoding.as_deref(), Some("base64"));

        let import = studio_emit(&projection, &config, &SpaceCommand::ImportMedia(crate::engine::space::commands::import_media::ImportMedia { node_id: node.id.clone(), format: DWG_FORMAT_ID.into() })).await.expect("handle");
        assert!(import.effects.iter().any(|effect| matches!(effect, Effect::RequestFileOpen { import_action, accept, .. } if import_action == "importMediaPayload" && accept.contains(".dwg"))));
        assert_eq!(import.config_mutations, vec![SpaceConfigMutation::SetPendingImport { node_id: Some(node.id), format: Some(DWG_FORMAT_ID.into()) }]);

        let pending_config = apply_config(&config, &import.config_mutations).await;
        let payload = studio_emit(&projection, &pending_config, &SpaceCommand::ImportMediaPayload(crate::engine::space::commands::import_media_payload::ImportMediaPayload { payload: format!("data:image/vnd.dwg;base64,{data}") })).await.expect("handle");
        assert!(payload.artifact_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
