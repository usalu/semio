//! 🗂️ 🗂️ Drawing play app commands command — `add-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::create_layer_by_kind;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}

pub(crate) fn build_layer(document: &DrawingSnapshot, kind: &str, operation: Option<&semio_framework_plugin::AppOperationContext>) -> Result<crate::DrawingLayerNode, Fault> {
    if !matches!(kind, "shape:rect" | "shape:ellipse" | "shape:line" | "shape:polygon" | "path" | "text" | "image" | "group" | "boolean" | "trace") { return Err(Fault::from("Unknown layer kind")); }
    let mut layer = create_layer_by_kind(kind);
    let mut material = document.id.as_bytes().to_vec();
    material.extend_from_slice(kind.as_bytes());
    if let Some(operation) = operation {
        material.extend_from_slice(&operation.app_instance_id.to_be_bytes());
        material.extend_from_slice(&operation.operation_id.to_be_bytes());
        material.extend_from_slice(&operation.generation.to_be_bytes());
        material.extend_from_slice(&operation.canonical_base_revision);
    }
    let mut ordinal = document.layers.len();
    loop {
        let mut candidate = material.clone();
        candidate.extend_from_slice(&(ordinal as u64).to_be_bytes());
        let id = crate::schema::create_drawing_id("layer", &candidate);
        if crate::schema::find_drawing_layer(document, &id).is_none() { crate::schema::layer_base_mut(&mut layer).id = id; return Ok(layer); }
        ordinal += 1;
    }
}

pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let layer = build_layer(doc.snapshot, &payload.kind, doc.operation_optional())?;
    Ok(Emit::commit(vec![crate::mutations::create_layer(None, Some(doc.snapshot.layers.len()), layer)], "Add layer"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
