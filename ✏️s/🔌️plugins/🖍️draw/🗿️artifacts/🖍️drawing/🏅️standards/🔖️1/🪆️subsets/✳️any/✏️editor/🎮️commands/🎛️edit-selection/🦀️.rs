//! 🎛️ Atomic, undoable arrangement of the current layer selection.
#[path="🗂️stack/🦀️.rs"]
mod stack;
#[path="🧩️ungroup/🦀️.rs"]
mod ungroup;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::schema::{drawing_layer_is_locked, find_drawing_layer, find_drawing_layer_location, layer_base, layer_base_mut, selected_drawing_layers};
use crate::{DrawingLayerNode, DrawingSnapshot};
use crate::op::DrawingMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-selection")]
pub struct EditSelection {
    pub operation: String,
    pub ids: Vec<String>,
}

pub fn plan(document: &DrawingSnapshot, ids: &[String], operation: &str) -> Result<Vec<DrawingMutation>, Fault> {
    if operation=="ungroup" {return ungroup::plan(document,ids).map(|(mutations,_)|mutations);}
    let selected = selected_drawing_layers(document, ids);
    if selected.is_empty() { return Err(Fault::from("Select at least one layer")); }
    if selected.iter().any(|layer| drawing_layer_is_locked(document, &layer_base(layer).id)) { return Err(Fault::from("Unlock the selected layers before editing")); }
    if matches!(operation,"alignLeft"|"alignCenter"|"alignRight"|"alignTop"|"alignMiddle"|"alignBottom"|"distributeHorizontal"|"distributeVertical") {return plan_arrangement(document,&selected,operation);}
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
        "bringToFront" | "sendToBack" | "bringForward" | "sendBackward" => {
            let siblings=match parent.as_deref().and_then(|id|find_drawing_layer(document,id)) {
                Some(DrawingLayerNode::Group(group))=>&group.children,
                _=>&document.layers,
            };
            let order=siblings.iter().map(|layer|layer_base(layer).id.clone()).collect::<Vec<_>>();
            let ids=selected.iter().map(|layer|layer_base(layer).id.clone()).collect::<Vec<_>>();
            let moves=stack::stack_moves(&order,&ids,operation).ok_or_else(||Fault::from("Invalid layer stack selection"))?;
            operations.extend(moves.into_iter().map(|(id,index)|crate::mutations::reorder_layer(id,parent.clone(),index)));
        }
        _ => return Err(Fault::from("Unknown selection operation")),
    }
    Ok(operations)
}

fn plan_arrangement(document:&DrawingSnapshot,selected:&[&DrawingLayerNode],operation:&str)->Result<Vec<DrawingMutation>,Fault> {
    let wanted=selected.iter().map(|layer|layer_base(layer).id.as_str()).collect::<std::collections::BTreeSet<_>>();
    let mut stack=vec![(document.layers.as_slice(),0usize,[1.0,0.0,0.0,1.0,0.0,0.0],true,false)];
    let mut items=Vec::new();
    let mut remaining=4_096usize;
    while let Some((layers,index,parent,editable,ancestor_selected))=stack.last_mut() {
        let Some(layer)=layers.get(*index) else {stack.pop();continue;};
        *index+=1;
        remaining=remaining.checked_sub(1).ok_or_else(||Fault::from("Drawing exceeds arrangement capacity"))?;
        let base=layer_base(layer);
        let chosen=wanted.contains(base.id.as_str());
        let editable=*editable && base.visible && !base.locked;
        let ancestor_selected=*ancestor_selected;
        let parent=*parent;
        if chosen {
            if !editable {return Err(Fault::from("Unlock and show selected layers before arranging"));}
            if !ancestor_selected {
                let (x,y,width,height)=crate::schema::drawing_layer_bounds_with_parent(layer,parent).ok_or_else(||Fault::from("The layer has no editable bounds"))?;
                items.push((layer,parent,[x,y,width,height]));
            }
        }
        let matrix=crate::schema::geometry::multiply(parent,crate::schema::drawing_transform_to_matrix(&base.transform));
        if let DrawingLayerNode::Group(group)=layer {stack.push((group.children.as_slice(),0,matrix,editable,ancestor_selected || chosen));}
    }
    let bounds=items.iter().map(|(_,_,bounds)|*bounds).collect::<Vec<_>>();
    let deltas=crate::schema::geometry::arrangement::arrange(&bounds,operation).ok_or_else(||Fault::from(if operation.starts_with("distribute") {"Select at least three layers with finite bounds to distribute"}else {"Select at least two layers with finite bounds to align"}))?;
    let mut mutations=Vec::new();
    for ((layer,parent,_),delta) in items.into_iter().zip(deltas) {
        let source=&layer_base(layer).transform;
        let [x,y,scale_x,scale_y,rotation]=crate::schema::geometry::translation::translate([source.x,source.y,source.scale_x,source.scale_y,source.rotation],[parent[0],parent[1],parent[2],parent[3],0.0,0.0],delta).ok_or_else(||Fault::from("Cannot arrange through a singular or nonfinite transform"))?;
        let transform=crate::DrawingTransform {x,y,scale_x,scale_y,rotation,shear:source.shear};
        if transform!=*source {mutations.push(crate::mutations::update_layer_transform(layer_base(layer).id.clone(),transform));}
    }
    Ok(mutations)
}

pub fn handle(payload: &EditSelection, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let ids = if payload.ids.is_empty() { &session.interaction.ids } else { &payload.ids };
    if payload.operation=="ungroup" {
        let (mutations,selection)=ungroup::plan(doc.snapshot,ids)?;
        let mut emit=Emit::mutations(mutations);
        emit.effects.push(crate::editor::drawing::commands::canvas_pointer_down::interaction_select_effect(&selection,"replace"));
        return Ok(emit);
    }
    let mutations=plan(doc.snapshot,ids,&payload.operation)?;
    Ok(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
