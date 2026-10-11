//! 🗂️ 🗂️ Drawing play app commands command — `add-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::create_layer_by_kind;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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

pub(crate) fn build_layer(document: &DrawingSnapshot, kind: &str, operation: &semio_framework_plugin::AppOperationContext, control:&mut semio_framework_value::NativeEncodeControl<'_>) -> Result<crate::DrawingLayerNode, Fault> {
    if !matches!(kind, "shape:rect" | "shape:ellipse" | "shape:line" | "shape:polygon" | "path" | "text" | "image" | "group" | "boolean" | "trace") { return Err(Fault::from("Unknown layer kind")); }
    let identity=prepare_identity(document,kind,operation,control)?;
    let mut layer = create_layer_by_kind(identity, kind);
    initialize_appearance(&mut layer);
    Ok(layer)
}

/// 🪪️ Admits a fresh identity at the explicit command IO boundary.
pub(crate) fn prepare_identity(document:&DrawingSnapshot,kind:&str,operation:&semio_framework_plugin::AppOperationContext,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<crate::schema::identity::DrawingIdentity,Fault>{
 crate::standards::v1::subsets::any::io::text::identity::creation::layer_identity(document,kind,operation,control).map_err(|error|Fault::from(error.to_string()))
}

pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if payload.kind=="image" {return Ok(super::import_image::request());}
    let operation=doc.operation()?;
    let layer = session.with_identity_control(|control|build_layer(doc.snapshot,&payload.kind,operation,control))?;
    let id=crate::schema::layer_id(&layer).to_string();
    let mut emit=Emit::mutations(vec![crate::mutations::create_layer(None, Some(doc.snapshot.layers.len()), layer)]);
    emit.effects.push(crate::editor::drawing::commands::canvas_pointer_down::interaction_select_effect(&[id],"replace"));
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
