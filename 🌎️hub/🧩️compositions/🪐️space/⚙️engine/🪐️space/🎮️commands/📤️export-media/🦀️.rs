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
            semio_framework_plugin::resolve_ready(crate::ensure_space_fixtures_registered());
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
