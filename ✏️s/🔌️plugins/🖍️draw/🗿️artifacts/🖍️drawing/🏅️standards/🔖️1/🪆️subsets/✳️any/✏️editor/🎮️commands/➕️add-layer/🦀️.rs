//! 🗂️ 🗂️ Drawing play app commands command — `add-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::create_layer_by_kind;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}

/// 🎨️ Paint new artwork without replacing an explicitly supplied appearance.
pub(crate) fn initialize_appearance(layer: &mut crate::DrawingLayerNode) {
    use crate::{DrawingLayerNode,FillStyle,StrokeStyle};
    let filled=matches!(layer,DrawingLayerNode::Shape(shape) if shape.shape_kind!="line") || matches!(layer,DrawingLayerNode::Boolean(_) | DrawingLayerNode::Trace(_));
    let outlined=matches!(layer,DrawingLayerNode::Shape(_) | DrawingLayerNode::Path(_));
    let text=matches!(layer,DrawingLayerNode::Text(_));
    let attributes=&mut crate::schema::layer_base_mut(layer).attributes;
    if attributes.fill.is_some() || attributes.stroke.is_some() { return; }
    if filled { attributes.fill=Some(FillStyle::Solid { color:[0.2,0.7,0.65,1.0] }); }
    else if text { attributes.fill=Some(FillStyle::Solid { color:[0.0,0.0,0.0,1.0] }); }
    if outlined { attributes.stroke=Some(StrokeStyle { color:[0.1,0.15,0.2,1.0],width:2.0,cap: crate::StrokeCap::Round,join: crate::StrokeJoin::Round,dash:None }); }
}

pub(crate) fn build_layer(document: &DrawingSnapshot, kind: &str, operation: Option<&semio_framework_plugin::AppOperationContext>) -> Result<crate::DrawingLayerNode, Fault> {
    if !matches!(kind, "shape:rect" | "shape:ellipse" | "shape:line" | "shape:polygon" | "path" | "text" | "image" | "group" | "boolean" | "trace") { return Err(Fault::from("Unknown layer kind")); }
    let mut layer = create_layer_by_kind(kind);
    initialize_appearance(&mut layer);
    identify_created_layer(document,&mut layer,kind,operation);
    Ok(layer)
}

/// 🪪️ Assigns a fresh layer identity within the document and retained operation scope.
pub(crate) fn identify_created_layer(document: &DrawingSnapshot, layer: &mut crate::DrawingLayerNode, kind: &str, operation: Option<&semio_framework_plugin::AppOperationContext>) {
    let mut material = document.id.bytes().collect::<Vec<_>>();
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
        let id = crate::standards::v1::subsets::any::schema::create_drawing_id("layer", &candidate);
        if crate::schema::find_drawing_layer(document, &id).is_none() { crate::schema::layer_base_mut(layer).id = id.into(); return; }
        ordinal += 1;
    }
}

pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if payload.kind=="image" {return Ok(super::import_image::request());}
    let layer = build_layer(doc.snapshot, &payload.kind, doc.operation_optional())?;
    let id=crate::schema::layer_id(&layer).to_string();
    let mut emit=Emit::mutations(vec![crate::mutations::create_layer(None, Some(doc.snapshot.layers.len()), layer)]);
    emit.effects.push(crate::editor::drawing::commands::canvas_pointer_down::interaction_select_effect(&[id],"replace"));
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
