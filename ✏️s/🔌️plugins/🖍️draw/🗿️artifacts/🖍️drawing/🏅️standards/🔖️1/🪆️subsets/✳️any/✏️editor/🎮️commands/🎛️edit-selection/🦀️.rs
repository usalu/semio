//! 🎛️ Atomic, undoable arrangement of the current layer selection.
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::schema::{drawing_layer_is_locked, drawing_layer_world_bounds, find_drawing_layer, find_drawing_layer_location, layer_base, layer_base_mut, selected_drawing_layers};
use crate::{DrawingLayerNode, DrawingSnapshot};
use crate::op::DrawingMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "edit-selection")]
pub struct EditSelection {
    pub operation: String,
    pub ids: Vec<String>,
}

pub fn plan(document: &DrawingSnapshot, ids: &[String], operation: &str) -> Result<Vec<DrawingMutation>, Fault> {
    let selected = selected_drawing_layers(document, ids);
    if selected.is_empty() { return Err(Fault::from("Select at least one layer")); }
    if selected.iter().any(|layer| drawing_layer_is_locked(document, &layer_base(layer).id)) { return Err(Fault::from("Unlock the selected layers before editing")); }
    let locations = selected.iter().map(|layer| find_drawing_layer_location(document, &layer_base(layer).id).ok_or_else(|| Fault::from("Layer no longer exists"))).collect::<Result<Vec<_>, _>>()?;
    let parent = locations[0].parent_id.clone();
    if operation != "toPath" && locations.iter().any(|location| location.parent_id != parent) { return Err(Fault::from("Select layers in the same group")); }
    let mut operations = Vec::new();
    match operation {
        "toPath" => {
            for (layer,location) in selected.iter().zip(&locations) {
                if matches!(layer,DrawingLayerNode::Path(_)) { continue; }
                if !matches!(layer,DrawingLayerNode::Shape(_)) { return Err(Fault::from("Select geometric shapes to convert")); }
                let segments=crate::schema::layer_to_path_segments(layer);
                if segments.is_empty() || !segments.iter().all(crate::schema::valid_path_segment) { return Err(Fault::from("The shape has no valid path geometry")); }
                let path=DrawingLayerNode::Path(crate::DrawingPathBody { base:layer_base(layer).clone(),segments });
                operations.push(crate::mutations::delete_layer(layer_base(layer).id.clone()));
                operations.push(crate::mutations::create_layer(location.parent_id.clone(),Some(location.index),path));
            }
        }
        "delete" => operations.extend(selected.iter().map(|layer| crate::mutations::delete_layer(layer_base(layer).id.clone()))),
        "duplicate" => {
            let mut working = document.clone();
            for layer in &selected {
                let mut ordinal = 1usize;
                let duplicate = loop {
                    let suffix = format!(" copy {ordinal}");
                    let candidate = crate::schema::clone_drawing_layer_node(layer, &suffix);
                    if find_drawing_layer(&working, &layer_base(&candidate).id).is_none() { break candidate; }
                    ordinal += 1;
                };
                let location = find_drawing_layer_location(&working, &layer_base(layer).id).ok_or_else(|| Fault::from("Layer no longer exists"))?;
                let mutation = crate::mutations::create_layer(parent.clone(), Some(location.index + 1), duplicate);
                crate::mutations::apply_drawing_mutation(&mut working, &mutation).map_err(|error| Fault::from(error.to_string()))?;
                operations.push(mutation);
            }
        }
        "group" => {
            let mut group = crate::schema::create_drawing_group_layer("Group");
            let material = format!("{}:{}", document.id, selected.iter().map(|layer| layer_base(layer).id.as_str()).collect::<Vec<_>>().join("/"));
            layer_base_mut(&mut group).id = crate::schema::create_drawing_id("group", material.as_bytes());
            let group_id = layer_base(&group).id.clone();
            if find_drawing_layer(document, &group_id).is_some() { return Err(Fault::from("This group already exists")); }
            operations.push(crate::mutations::create_layer(parent, Some(locations[0].index), group));
            for (index, layer) in selected.iter().enumerate() { operations.push(crate::mutations::reorder_layer(layer_base(layer).id.clone(), Some(group_id.clone()), index)); }
        }
        "bringToFront" | "sendToBack" => {
            let count = parent.as_deref().and_then(|id| find_drawing_layer(document, id)).and_then(|layer| if let DrawingLayerNode::Group(group) = layer { Some(group.children.len()) } else { None }).unwrap_or(document.layers.len());
            let mut ordered = selected.clone();
            if operation == "sendToBack" { ordered.reverse(); }
            for layer in ordered { operations.push(crate::mutations::reorder_layer(layer_base(layer).id.clone(), parent.clone(), if operation == "sendToBack" { 0 } else { count - 1 })); }
        }
        "alignLeft" | "alignCenter" | "alignRight" | "alignTop" | "alignMiddle" | "alignBottom" | "distributeHorizontal" | "distributeVertical" => {
            if selected.len() < 2 { return Err(Fault::from("Select at least two layers to arrange")); }
            let mut items = selected.iter().map(|layer| drawing_layer_world_bounds(layer).map(|bounds| (*layer, bounds)).ok_or_else(|| Fault::from("The layer has no editable bounds"))).collect::<Result<Vec<_>, _>>()?;
            let horizontal = matches!(operation, "alignLeft" | "alignCenter" | "alignRight" | "distributeHorizontal");
            let start = |bounds: (f64,f64,f64,f64)| if horizontal { bounds.0 } else { bounds.1 };
            let size = |bounds: (f64,f64,f64,f64)| if horizontal { bounds.2 } else { bounds.3 };
            let min = items.iter().map(|(_, bounds)| start(*bounds)).fold(f64::INFINITY, f64::min);
            let max = items.iter().map(|(_, bounds)| start(*bounds)+size(*bounds)).fold(f64::NEG_INFINITY, f64::max);
            let distributing = operation.starts_with("distribute");
            if distributing && items.len() < 3 { return Err(Fault::from("Select at least three layers to distribute")); }
            if distributing { items.sort_by(|a,b| start(a.1).total_cmp(&start(b.1))); }
            let gap = (max-min-items.iter().map(|(_, bounds)| size(*bounds)).sum::<f64>())/(items.len()-1) as f64;
            let mut cursor = min;
            for (layer, bounds) in items {
                let destination = if distributing { let position = cursor; cursor += size(bounds)+gap; position }
                    else if matches!(operation, "alignCenter" | "alignMiddle") { (min+max-size(bounds))/2.0 }
                    else if matches!(operation, "alignRight" | "alignBottom") { max-size(bounds) }
                    else { min };
                let mut transform = layer_base(layer).transform.clone();
                if horizontal { transform.x += destination-start(bounds); } else { transform.y += destination-start(bounds); }
                operations.push(crate::mutations::update_layer_transform(layer_base(layer).id.clone(), transform));
            }
        }
        _ => return Err(Fault::from("Unknown selection operation")),
    }
    Ok(operations)
}

pub fn handle(payload: &EditSelection, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let ids = if payload.ids.is_empty() { &session.interaction.ids } else { &payload.ids };
    let mutations=plan(doc.snapshot,ids,&payload.operation)?;
    Ok(if mutations.is_empty() { Emit::default() } else { Emit::commit(mutations,&payload.operation) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
