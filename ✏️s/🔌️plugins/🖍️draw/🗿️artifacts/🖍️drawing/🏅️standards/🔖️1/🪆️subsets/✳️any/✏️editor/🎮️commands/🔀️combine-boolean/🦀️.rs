//! 🗂️ 🗂️ Drawing play app commands command — `combine-boolean`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::{create_drawing_boolean_layer,find_drawing_layer,find_drawing_layer_location,layer_base,layer_base_mut};
use crate::{DrawingSnapshot,DrawingLayerNode};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "combine-boolean")]
pub struct CombineBoolean {
    pub operation: String,
    pub ids: Vec<String>,
}

pub fn plan(document:&DrawingSnapshot,ids:&[String],operation:&str,admission:&semio_framework_plugin::AppOperationContext,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(Vec<DrawingMutation>,String),Fault> {
    if !crate::DRAWING_BOOLEAN_OPERATIONS.contains(&operation) {return Err(Fault::from("Choose Union, Difference, Intersection or Exclusive Or / Vereinigung, Differenz, Schnittmenge oder Exklusives Oder wählen"));}
    let unique=ids.iter().collect::<std::collections::BTreeSet<_>>();
    if ids.len()<2 || unique.len()!=ids.len() {return Err(Fault::from("Select at least two distinct geometric layers / Mindestens zwei verschiedene geometrische Ebenen auswählen"));}
    let selected=ids.iter().map(|id|find_drawing_layer(document,id).ok_or_else(||Fault::from("A selected layer no longer exists / Eine ausgewählte Ebene existiert nicht mehr"))).collect::<Result<Vec<_>,_>>()?;
    if selected.iter().any(|layer|!matches!(layer,DrawingLayerNode::Shape(_)|DrawingLayerNode::Path(_)|DrawingLayerNode::Boolean(_)|DrawingLayerNode::Trace(_))) {return Err(Fault::from("Select geometric layers to combine / Geometrische Ebenen zum Kombinieren auswählen"));}
    let locations=ids.iter().map(|id|find_drawing_layer_location(document,id).ok_or_else(||Fault::from("A selected layer no longer exists / Eine ausgewählte Ebene existiert nicht mehr"))).collect::<Result<Vec<_>,_>>()?;
    let parent=locations[0].parent_id.clone();
    if locations.iter().any(|location|location.parent_id!=parent) {return Err(Fault::from("Select layers in the same group / Ebenen in derselben Gruppe auswählen"));}
    for layer in &selected {
        let mut current=Some(layer_base(layer).id.clone());
        while let Some(id)=current {
            let node=find_drawing_layer(document,&id).ok_or_else(||Fault::from("A selected layer no longer exists"))?;
            if layer_base(node).locked || !layer_base(node).visible {return Err(Fault::from("Unlock and show selected layers and their groups / Ausgewählte Ebenen und ihre Gruppen entsperren und anzeigen"));}
            current=find_drawing_layer_location(document,&id).and_then(|location|location.parent_id);
        }
    }
    let identity=super::add_layer::prepare_identity(document,"boolean",admission,control)?;
    let id=identity.key().to_string_owner();
    let mut layer=create_drawing_boolean_layer(identity,"Boolean",operation,ids.iter().map(|id|id.as_str().into()).collect());
    layer_base_mut(&mut layer).attributes=layer_base(selected[0]).attributes.clone();
    let index=locations.iter().map(|location|location.index).max().unwrap()+1;
    Ok((vec![crate::mutations::create_layer(parent,Some(index),layer)],id))
}

pub fn handle(payload:&CombineBoolean,doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,session:&mut DrawingSession)->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
    let ids=if payload.ids.is_empty(){session.interaction.ids.clone()}else{payload.ids.clone()};
    let admission=doc.operation()?;
    let (mutations,id)=session.with_identity_control(|control|plan(doc.snapshot,&ids,&payload.operation,admission,control))?;
    let mut emit=Emit::mutations(mutations);
    emit.effects.push(crate::editor::drawing::commands::canvas_pointer_down::interaction_select_effect(&[id],"replace"));
    Ok(emit)
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
