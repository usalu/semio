//! 🖼️ 🖼️ S Studio app command — `export-media`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use crate::engine::space::engine::{resolve_future, workflow_parameter_bindings_to_os, workflow_parameters_to_os};
use semio_framework_os::{materialize_os_app_instance_document_json, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{plugin_app_close_prelude::Value, ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-media")]
pub struct ExportMedia {
    pub node_id: String,
    pub format: String,
    pub document_json: String,
}

pub fn handle(payload: &ExportMedia, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let projection = doc.snapshot;
    match projection.graph.nodes.iter().find(|row| row.id == payload.node_id) {
        Some(node) => {
            let source = payload.document_json.parse::<Value>().map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.source"), error.to_string()))?;
            if !source.as_object().is_some_and(|object| !object.is_empty()) {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.source"), "export requires an explicit artifact document"));
            }
            let bindings = resolve_future(workflow_parameter_bindings_to_os(&projection.parameter_bindings));
            let parameters = resolve_future(workflow_parameters_to_os(&projection.parameters));
            let document_json = materialize_os_app_instance_document_json(&payload.document_json, &node.id, &bindings, &parameters);
            let document_value = document_json.parse::<Value>().map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.media.source"), error.to_string()))?;
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
