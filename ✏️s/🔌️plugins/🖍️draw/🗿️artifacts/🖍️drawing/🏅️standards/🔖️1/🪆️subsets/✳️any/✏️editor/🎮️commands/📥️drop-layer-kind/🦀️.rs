//! 🗂️ 🗂️ Drawing play app commands command — `drop-layer-kind`.

use crate::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "drop-layer-kind")]
pub struct DropLayerKind {
    pub kind: String,
    pub target_row_id: String,
    pub drop_position: String,
}

pub fn handle(payload: &DropLayerKind, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    if payload.kind=="image" {let (parent,index)=super::move_layer::resolve_reorder_target(document,&payload.target_row_id,&payload.drop_position)?;return Ok(super::import_image::request_at(parent.map(|parent|parent.to_string_owner()),Some(index)));}
    let operation=doc.operation()?;
    let layer = session.with_identity_control(|control|super::add_layer::build_layer(document,&payload.kind,operation,control))?;
    let (parent_id, index) = super::move_layer::resolve_reorder_target(document, &payload.target_row_id, &payload.drop_position)?;
    Ok(Emit::mutations(vec![semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(parent_id, Some(index), layer)]))
}
