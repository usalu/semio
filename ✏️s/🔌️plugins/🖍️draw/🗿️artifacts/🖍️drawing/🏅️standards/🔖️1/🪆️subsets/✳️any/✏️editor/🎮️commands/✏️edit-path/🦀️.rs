//! ✏️ Undoable edits of the selected path's anchors and handles.
use crate::editor::drawing::commands::canvas_pointer_down::{DrawingSession,point_selection_effect};
use crate::editor::drawing::interaction::points;
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

pub fn handle(payload: &EditPath, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if crate::schema::drawing_layer_is_locked(doc.snapshot, &payload.layer_id) { return Err(Fault::from("Unlock the path before editing")); }
    let Some(DrawingLayerNode::Path(path)) = crate::schema::find_drawing_layer(doc.snapshot, &payload.layer_id) else { return Err(Fault::from("Select a path to edit")); };
    let segments = edit_path(&path.segments, &payload.edit).map_err(Fault::from)?;
    if segments==path.segments {return Ok(Emit::default());}
    let rebind=matches!(payload.edit.as_ref(),PathEdit::Position {..}|PathEdit::Coordinate {..}|PathEdit::Translate {..});
    let mut selected=Vec::new();
    if rebind && !session.interaction.points.is_empty() {
        let before=points::geometry_id(&path.segments).ok_or_else(||Fault::from("Invalid path geometry"))?;
        let after=points::geometry_id(&segments).ok_or_else(||Fault::from("Invalid path geometry"))?;
        for id in &session.interaction.points {
            let Some(point)=points::parse_point_id(id) else {continue;};
            if point.layer_id!=payload.layer_id {selected.push(id.clone());continue;}
            if point.geometry==before && segments.get(point.index).is_some_and(|segment|points::point_slots(segment).contains(&point.point)) {
                if let Some(id)=points::point_id(point.layer_id,&after,point.index,point.point) {selected.push(id);}
            }
        }
    }
    let mut emit=Emit::commit(vec![crate::mutations::update_path_geometry(payload.layer_id.clone(), segments)],"Edit path");
    if rebind && !session.interaction.points.is_empty() {emit.effects.push(point_selection_effect(&selected));}
    Ok(emit)
}
