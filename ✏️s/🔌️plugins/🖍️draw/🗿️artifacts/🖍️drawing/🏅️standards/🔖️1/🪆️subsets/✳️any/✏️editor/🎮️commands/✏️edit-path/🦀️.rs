//! ✏️ Undoable edits of the selected path's anchors and handles.
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::schema::geometry::editing::{edit_path, PathEdit};
use crate::{DrawingLayerNode, DrawingSnapshot};
use crate::mutations::DrawingMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "edit-path")]
pub struct EditPath {
    pub layer_id: String,
    #[dsl(statements, block)]
    pub edit: Box<PathEdit>,
}

pub fn handle(payload: &EditPath, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if crate::schema::drawing_layer_is_locked(doc.snapshot, &payload.layer_id) { return Err(Fault::from("Unlock the path before editing")); }
    let Some(DrawingLayerNode::Path(path)) = crate::schema::find_drawing_layer(doc.snapshot, &payload.layer_id) else { return Err(Fault::from("Select a path to edit")); };
    let segments = edit_path(&path.segments, &payload.edit).map_err(Fault::from)?;
    Ok(Emit::commit(vec![crate::mutations::update_path_geometry(payload.layer_id.clone(), segments)], "Edit path"))
}
