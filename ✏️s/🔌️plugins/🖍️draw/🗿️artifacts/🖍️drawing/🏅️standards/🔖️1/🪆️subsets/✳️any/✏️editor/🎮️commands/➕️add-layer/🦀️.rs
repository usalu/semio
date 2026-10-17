//! 🗂️ 🗂️ Drawing play app commands command — `add-layer`.

use semio_s_artifact_draw_drawing_editor_tool::initialize_appearance;
use semio_s_artifact_draw_drawing_editor_tool::prepare_identity;
use semio_s_artifact_draw_drawing_editor_tool::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::create_layer_by_kind;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}


pub fn build_layer(document: &DrawingSnapshot, kind: &str, operation: &semio_framework_plugin::AppOperationContext, control:&mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_s_artifact_draw_drawing_model::DrawingLayerNode, Fault> {
    if !matches!(kind, "shape:rect" | "shape:ellipse" | "shape:line" | "shape:polygon" | "path" | "text" | "image" | "group" | "boolean" | "trace") { return Err(Fault::from("Unknown layer kind")); }
    let identity=prepare_identity(document,kind,operation,control)?;
    let mut layer = create_layer_by_kind(identity, kind);
    initialize_appearance(&mut layer);
    Ok(layer)
}


pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if payload.kind=="image" {return Ok(super::import_image::request());}
    let operation=doc.operation()?;
    let layer = session.with_identity_control(|control|build_layer(doc.snapshot,&payload.kind,operation,control))?;
    let id=semio_s_artifact_draw_drawing_model::layer_id(&layer).to_string();
    let mut emit=Emit::mutations(vec![semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, Some(doc.snapshot.layers.len()), layer)]);
    emit.effects.push(semio_s_artifact_draw_drawing_editor_tool::interaction_select_effect(&[id],"replace"));
    Ok(emit)
}

