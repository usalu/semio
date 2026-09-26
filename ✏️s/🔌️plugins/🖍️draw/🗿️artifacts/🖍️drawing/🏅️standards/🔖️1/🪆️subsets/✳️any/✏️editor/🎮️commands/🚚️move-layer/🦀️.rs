//! 🗂️ 🗂️ Drawing play app commands command — `move-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::find_drawing_layer_location;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️DocumentHelpers
pub(crate) fn resolve_reorder_target(document: &DrawingSnapshot, target_row_id: &str, drop_position: &str) -> Result<(Option<String>, usize), Fault> {
    if !matches!(drop_position, "before" | "after" | "inside") { return Err(Fault::from("Unknown layer drop position")); }
    if target_row_id == "drawing-play-layers" || target_row_id == "drawing-play-layers.empty" { return Ok((None, document.layers.len())); }
    let selected = crate::schema::selected_drawing_layers(document, &[target_row_id.into()]);
    let layer = selected.first().ok_or_else(|| Fault::from("Drop target no longer exists"))?;
    let id = crate::schema::layer_id(layer);
    if crate::schema::drawing_layer_is_locked(document, id) { return Err(Fault::from("Unlock the destination before moving layers")); }
    if drop_position == "inside" {
        return match layer { crate::DrawingLayerNode::Group(group) => Ok((Some(group.base.id.clone()), group.children.len())), _ => Err(Fault::from("Only groups can contain layers")) };
    }
    let location = find_drawing_layer_location(document, id).ok_or_else(|| Fault::from("Drop target no longer exists"))?;
    Ok((location.parent_id, location.index + usize::from(drop_position == "after")))
}
//#endregion 🔖️DocumentHelpers

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-layer")]
pub struct MoveLayer {
    pub layer_id: String,
    pub target_row_id: String,
    pub drop_position: String,
}

pub fn plan(document: &DrawingSnapshot, payload: &MoveLayer) -> Result<DrawingMutation, Fault> {

    let source = find_drawing_layer_location(document, &payload.layer_id).ok_or_else(|| Fault::from("Layer no longer exists"))?;
    if crate::schema::drawing_layer_is_locked(document, &payload.layer_id) { return Err(Fault::from("Unlock the layer before moving it")); }
    let (parent_id, mut index) = resolve_reorder_target(document, &payload.target_row_id, &payload.drop_position)?;
    let mut ancestor = parent_id.clone();
    while let Some(id) = ancestor {
        if id == payload.layer_id { return Err(Fault::from("A group cannot contain itself")); }
        ancestor = find_drawing_layer_location(document, &id).and_then(|location| location.parent_id);
    }
    if source.parent_id == parent_id && source.index < index { index -= 1; }
    Ok(crate::mutations::reorder_layer(payload.layer_id.clone(), parent_id, index))
}

pub fn handle(payload: &MoveLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::commit(vec![plan(doc.snapshot, payload)?], "Move layer"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
