//! 🎨️ One semantic fill edit with the artifact's normal undo boundary.
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::schema::fill::{edit_fill, FillEdit};
use crate::{DrawingSnapshot, DrawingMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "edit-fill")]
pub struct EditFill {
    pub layer_id: String,
    #[dsl(statements, block)]
    pub edit: Box<FillEdit>,
}

pub fn handle(payload: &EditFill, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if crate::schema::drawing_layer_is_locked(doc.snapshot,&payload.layer_id) { return Err(Fault::from("Unlock the layer before editing")); }
    let layer = crate::schema::find_drawing_layer(doc.snapshot,&payload.layer_id).ok_or_else(|| Fault::from("Select a layer to edit"))?;
    let source = crate::schema::layer_base(layer).attributes.fill.as_ref();
    let fill = edit_fill(source,&payload.edit).map_err(Fault::from)?;
    Ok(Emit::commit(vec![crate::mutations::replace_layer_fill(payload.layer_id.clone(),fill)],"Edit fill"))
}
