//! 🕹️ Atomic document-axis keyboard movement for layers and path points.
use crate::editor::drawing::commands::canvas_pointer_down::{DrawingSession,point_selection_effect};
use crate::editor::drawing::interaction::points;
use crate::schema::geometry::{multiply,editing::{PathPointRef,translate_world_path_points}};
use crate::{DrawingLayerNode,DrawingSnapshot};
use crate::mutations::DrawingMutation;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault,NoConfig,NoConfigMutation};
use std::collections::{BTreeMap,BTreeSet};

pub(crate) fn plan(document:&DrawingSnapshot,session:&DrawingSession,delta:[f64;2])->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
    plan_selection(document,&session.active_utility_id,&session.interaction.ids,&session.interaction.points,delta)
}

pub(crate) fn plan_selection(document:&DrawingSnapshot,utility:&str,ids:&[String],point_ids:&[String],delta:[f64;2])->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
    let nodes=utility=="editNodes";
    if !nodes && !matches!(utility,"selectDirect"|"selectMarquee"|"selectLasso"|"transformMove") {return Ok(Emit::default());}
    if ids.is_empty() || (nodes && point_ids.is_empty()) {return Ok(Emit::default());}
    if ids.len()>4_096 || point_ids.len()>4_096 {return Err(Fault::from("Selection exceeds keyboard movement capacity"));}
    let selected: BTreeSet<_>=ids.iter().map(String::as_str).collect();
    let mut references:BTreeMap<&str,Vec<points::PointSelectionRef<'_>>>=BTreeMap::new();
    if nodes {for id in point_ids {
        let point=points::parse_point_id(id).ok_or_else(||Fault::from("Invalid selected point"))?;
        if selected.contains(point.layer_id) {references.entry(point.layer_id).or_default().push(point);}
    }}
    let wanted:BTreeSet<_>=if nodes {references.keys().copied().collect()} else {selected};
    if wanted.is_empty() {return Ok(Emit::default());}
    let mut stack=vec![(document.layers.as_slice(),0usize,[1.0,0.0,0.0,1.0,0.0,0.0],true,false)];
    let mut found=BTreeSet::new();
    let mut mutations=Vec::new();
    let mut rebound=Vec::new();
    let mut remaining=4_096usize;
    while let Some((layers,index,parent,editable,ancestor_selected))=stack.last_mut() {
        let Some(layer)=layers.get(*index) else {stack.pop();continue;};
        *index+=1;
        remaining=remaining.checked_sub(1).ok_or_else(||Fault::from("Drawing exceeds keyboard movement capacity"))?;
        let base=crate::schema::layer_base(layer);
        let editable=*editable && base.visible && !base.locked;
        let chosen=wanted.contains(base.id.as_str());
        let ancestor_selected=*ancestor_selected;
        let parent=*parent;
        let matrix=multiply(parent,crate::schema::drawing_transform_to_matrix(&base.transform));
        if chosen {
            found.insert(base.id.as_str());
            if !editable {return Err(Fault::from("Unlock and show selected layers before moving"));}
            if nodes {
                let DrawingLayerNode::Path(path)=layer else {return Err(Fault::from("Selected points no longer belong to a path"));};
                remaining=remaining.checked_sub(path.segments.len()).ok_or_else(||Fault::from("Path exceeds keyboard movement capacity"))?;
                let before=points::geometry_id(&path.segments).ok_or_else(||Fault::from("Invalid path geometry"))?;
                let targets=&references[base.id.as_str()];
                if targets.iter().any(|point|point.geometry!=before) {return Err(Fault::from("Selected path points changed"));}
                let refs=targets.iter().map(|point|PathPointRef {index:point.index,point:point.point}).collect::<Vec<_>>();
                let segments=translate_world_path_points(&path.segments,&refs,matrix,delta).map_err(Fault::from)?;
                let after=points::geometry_id(&segments).ok_or_else(||Fault::from("Invalid moved path geometry"))?;
                for point in targets {rebound.push(points::point_id(point.layer_id,&after,point.index,point.point).ok_or_else(||Fault::from("Invalid selected point"))?);}
                if segments!=path.segments {mutations.push(crate::mutations::update_path_geometry(base.id.clone(),segments));}
            } else if !ancestor_selected {
                let source=&base.transform;
                let [x,y,scale_x,scale_y,rotation]=crate::schema::geometry::translation::translate([source.x,source.y,source.scale_x,source.scale_y,source.rotation],parent,delta).ok_or_else(||Fault::from("Cannot move through a singular or nonfinite transform"))?;
                let transform=crate::DrawingTransform {x,y,scale_x,scale_y,rotation,shear:source.shear};
                if transform!=*source {mutations.push(crate::mutations::update_layer_transform(base.id.clone(),transform));}
            }
        }
        if let DrawingLayerNode::Group(group)=layer {stack.push((group.children.as_slice(),0,matrix,editable,ancestor_selected || chosen));}
    }
    if found!=wanted {return Err(Fault::from("A selected layer no longer exists"));}
    if mutations.is_empty() {return Ok(Emit::default());}
    let mut emit=Emit::commit(mutations,if nodes {"Nudge path points"} else {"Nudge layers"});
    if nodes {emit.effects.push(point_selection_effect(&rebound));}
    Ok(emit)
}

macro_rules! nudge_command {
    ($module:ident,$name:ident,$keyword:literal,$dx:expr,$dy:expr)=>{
        pub mod $module {
            use super::*;
            #[derive(Clone,Debug,PartialEq,dsl::ToValue,dsl::FromValue,dsl::DslRecord)]
            #[dsl(keyword=$keyword)]
            pub struct $name {}
            pub fn handle(_payload:&$name,doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,session:&mut DrawingSession)->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
                plan(doc.snapshot,session,[$dx,$dy])
            }
        }
    };
}

nudge_command!(nudge_selection_left,NudgeSelectionLeft,"nudge-selection-left",-1.0,0.0);
nudge_command!(nudge_selection_right,NudgeSelectionRight,"nudge-selection-right",1.0,0.0);
nudge_command!(nudge_selection_up,NudgeSelectionUp,"nudge-selection-up",0.0,-1.0);
nudge_command!(nudge_selection_down,NudgeSelectionDown,"nudge-selection-down",0.0,1.0);
nudge_command!(nudge_selection_left_fast,NudgeSelectionLeftFast,"nudge-selection-left-fast",-10.0,0.0);
nudge_command!(nudge_selection_right_fast,NudgeSelectionRightFast,"nudge-selection-right-fast",10.0,0.0);
nudge_command!(nudge_selection_up_fast,NudgeSelectionUpFast,"nudge-selection-up-fast",0.0,-10.0);
nudge_command!(nudge_selection_down_fast,NudgeSelectionDownFast,"nudge-selection-down-fast",0.0,10.0);

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
