//! 🎨️ One semantic fill edit with the artifact's normal undo boundary.
use crate::commands::canvas_pointer_down::DrawingSession;
use semio_s_artifact_draw_drawing_model::fill::{edit_fill, FillEdit};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "edit-fill")]
pub struct EditFill {
    pub layer_id: String,
    #[dsl(statements, block)]
    pub edit: Box<FillEdit>,
}

pub fn handle(payload: &EditFill, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if semio_s_artifact_draw_drawing_model::drawing_layer_is_locked(doc.snapshot,&payload.layer_id) { return Err(Fault::from("Unlock the layer before editing")); }
    let layer = semio_s_artifact_draw_drawing_model::find_drawing_layer(doc.snapshot,&payload.layer_id).ok_or_else(|| Fault::from("Select a layer to edit"))?;
    let source = semio_s_artifact_draw_drawing_model::layer_base(layer).attributes.fill.as_ref();
    let fill = edit_fill(source,&payload.edit).map_err(Fault::from)?;
    Ok(Emit::mutations(vec![semio_s_artifact_draw_drawing_style::replace_layer_fill::mutation::replace_layer_fill(payload.layer_id.clone().into(),fill)]))
}
