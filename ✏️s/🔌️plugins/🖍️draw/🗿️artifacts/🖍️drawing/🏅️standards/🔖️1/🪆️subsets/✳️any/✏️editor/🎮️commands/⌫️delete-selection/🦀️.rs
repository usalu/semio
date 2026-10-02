//! ⌫️ Atomic keyboard deletion of selected layers, anchors and handles.
use crate::editor::drawing::commands::canvas_pointer_down::{DrawingSession,point_selection_effect,interaction_select_effect};
use crate::editor::drawing::interaction::points;
use crate::schema::geometry::editing::{edit_path,PathEdit,PathPointRef};
use crate::{DrawingLayerNode,DrawingSnapshot};
use crate::mutations::DrawingMutation;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault,NoConfig,NoConfigMutation};
use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone,Debug,PartialEq,dsl::ToValue,dsl::FromValue,dsl::DslRecord)]
#[dsl(keyword="delete-selection")]
pub struct DeleteSelection {}

pub(crate) fn plan(document:&DrawingSnapshot,utility:&str,ids:&[String],point_ids:&[String])->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
    let nodes=utility=="editNodes";
    if !nodes && !matches!(utility,"selectDirect"|"selectMarquee"|"selectLasso"|"transformMove") {return Ok(Emit::default());}
    if ids.is_empty() || (nodes && point_ids.is_empty()) {return Ok(Emit::default());}
    if ids.len()>4096 || point_ids.len()>4096 {return Err(Fault::from("Selection exceeds deletion capacity"));}
    let selected:BTreeSet<_>=ids.iter().map(String::as_str).collect();
    let mut references:BTreeMap<&str,Vec<points::PointSelectionRef<'_>>>=BTreeMap::new();
    if nodes {for id in point_ids {
        let point=points::parse_point_id(id).ok_or_else(||Fault::from("Invalid selected point"))?;
        if selected.contains(point.layer_id) {references.entry(point.layer_id).or_default().push(point);}
    }}
    let wanted:BTreeSet<_>=if nodes {references.keys().copied().collect()}else {selected};
    if wanted.is_empty() {return Ok(Emit::default());}
    let mut stack=vec![(document.layers.as_slice(),0usize,true,false)];
    let mut found=BTreeSet::new();
    let mut deleted=BTreeSet::new();
    let mut mutations=Vec::new();
    let mut remaining=4096usize;
    while let Some((layers,index,editable,ancestor_selected))=stack.last_mut() {
        let Some(layer)=layers.get(*index) else {stack.pop();continue;};
        *index+=1;
        remaining=remaining.checked_sub(1).ok_or_else(||Fault::from("Drawing exceeds deletion capacity"))?;
        let base=crate::schema::layer_base(layer);
        let editable=*editable && base.visible && !base.locked;
        let ancestor_selected=*ancestor_selected;
        let chosen=wanted.contains(base.id.as_str());
        if chosen {
            found.insert(base.id.as_str());
            if !editable {return Err(Fault::from("Unlock and show selected layers before deleting"));}
            if nodes {
                let DrawingLayerNode::Path(path)=layer else {return Err(Fault::from("Selected points no longer belong to a path"));};
                remaining=remaining.checked_sub(path.segments.len()).ok_or_else(||Fault::from("Path exceeds deletion capacity"))?;
                let geometry=points::geometry_id(&path.segments).ok_or_else(||Fault::from("Invalid path geometry"))?;
                let targets=&references[base.id.as_str()];
                if targets.iter().any(|point|point.geometry!=geometry) {return Err(Fault::from("Selected path points changed"));}
                let refs=targets.iter().map(|point|PathPointRef {index:point.index,point:point.point}).collect();
                let segments=edit_path(&path.segments,&PathEdit::DeletePoints {points:refs}).map_err(Fault::from)?;
                if segments.is_empty() {mutations.push(crate::mutations::delete_layer(base.id.clone()));deleted.insert(base.id.as_str());}
                else if segments!=path.segments {mutations.push(crate::mutations::update_path_geometry(base.id.clone(),segments));}
            } else if !ancestor_selected {mutations.push(crate::mutations::delete_layer(base.id.clone()));}
        }
        if let DrawingLayerNode::Group(group)=layer {stack.push((group.children.as_slice(),0,editable,ancestor_selected || chosen));}
    }
    if found!=wanted {return Err(Fault::from("A selected layer no longer exists"));}
    let mut emit=if mutations.is_empty() {Emit::default()}else {Emit::mutations(mutations)};
    emit.effects.push(point_selection_effect(&[]));
    let selection=if nodes {ids.iter().filter(|id|!deleted.contains(id.as_str())).cloned().collect::<Vec<_>>()}else {Vec::new()};
    emit.effects.push(interaction_select_effect(&selection,"replace"));
    Ok(emit)
}

pub fn handle(_payload:&DeleteSelection,doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,session:&mut DrawingSession)->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
    plan(doc.snapshot,&session.active_utility_id,&session.interaction.ids,&session.interaction.points)
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
