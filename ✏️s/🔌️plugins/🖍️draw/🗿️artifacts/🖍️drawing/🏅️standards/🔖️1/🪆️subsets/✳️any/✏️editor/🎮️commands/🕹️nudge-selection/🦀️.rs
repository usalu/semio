//! 🕹️ Document-axis keyboard movement for layers and path points — each nudge is ONE canvas tool transaction of the
//! relative leaf the drag gesture yields too (`drag-layers`, or `drag-path-points` under `editNodes`).
use crate::editor::drawing::commands::canvas_pointer_down::{canvas_tool, drawing_rebound_points, drawing_tool_emit, point_selection_effect, DrawingSession, DrawingTool};
use crate::editor::drawing::interaction::points;
use crate::mutations::{drag_layers, drag_path_points, DrawingMutation, DrawingPathPointTarget};
use crate::schema::geometry::{multiply,editing::{PathPointRef,translate_world_path_points}};
use crate::{DrawingLayerNode,DrawingSnapshot};
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault,NoConfig,NoConfigMutation};
use std::collections::{BTreeMap,BTreeSet};

/// 🎯️ What a selection transform addresses once validated against `document`: the selected layers without the ones a
/// selected ancestor already moves, or the selected points of the selected paths — `None` when nothing moves.
pub(crate) enum DrawingSelectionTargets {
    Layers(Vec<String>),
    Points(Vec<DrawingPathPointTarget>),
}

/// 🔎️ Validates the selection a transform by `delta` addresses — every selected layer exists, is visible and unlocked
/// through its whole ancestry and moves through a regular transform; every selected point belongs to a selected path at
/// its current geometry — and answers its targets in document order. A refusal changes nothing.
pub(crate) fn drawing_selection_targets(document:&DrawingSnapshot,nodes:bool,ids:&[String],point_ids:&[String],delta:[f64;2])->Result<Option<DrawingSelectionTargets>,Fault> {
    if ids.is_empty() || (nodes && point_ids.is_empty()) {return Ok(None);}
    if ids.len()>4_096 || point_ids.len()>4_096 {return Err(Fault::from("Selection exceeds keyboard movement capacity"));}
    let selected: BTreeSet<_>=ids.iter().map(String::as_str).collect();
    let mut references:BTreeMap<&str,Vec<points::PointSelectionRef<'_>>>=BTreeMap::new();
    if nodes {for id in point_ids {
        let point=points::parse_point_id(id).ok_or_else(||Fault::from("Invalid selected point"))?;
        if selected.contains(point.layer_id) {references.entry(point.layer_id).or_default().push(point);}
    }}
    let wanted:BTreeSet<_>=if nodes {references.keys().copied().collect()} else {selected};
    if wanted.is_empty() {return Ok(None);}
    let mut stack=vec![(document.layers.as_slice(),0usize,[1.0,0.0,0.0,1.0,0.0,0.0],true,false)];
    let mut found=BTreeSet::new();
    let (mut layers,mut targets)=(Vec::new(),Vec::new());
    let mut remaining=4_096usize;
    while let Some((children,index,parent,editable,ancestor_selected))=stack.last_mut() {
        let Some(layer)=children.get(*index) else {stack.pop();continue;};
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
                let chosen_points=&references[base.id.as_str()];
                if chosen_points.iter().any(|point|point.geometry!=before) {return Err(Fault::from("Selected path points changed"));}
                let refs=chosen_points.iter().map(|point|PathPointRef {index:point.index,point:point.point}).collect::<Vec<_>>();
                translate_world_path_points(&path.segments,&refs,matrix,delta).map_err(Fault::from)?;
                targets.extend(chosen_points.iter().map(|point|DrawingPathPointTarget {layer_id:base.id.clone(),index:point.index,point:point.point}));
            } else if !ancestor_selected {
                let source=&base.transform;
                crate::schema::geometry::translation::translate([source.x,source.y,source.scale_x,source.scale_y,source.rotation],parent,delta).ok_or_else(||Fault::from("Cannot move through a singular or nonfinite transform"))?;
                layers.push(base.id.clone());
            }
        }
        if let DrawingLayerNode::Group(group)=layer {stack.push((group.children.as_slice(),0,matrix,editable,ancestor_selected || chosen));}
    }
    if found!=wanted {return Err(Fault::from("A selected layer no longer exists"));}
    Ok(if nodes {(!targets.is_empty()).then_some(DrawingSelectionTargets::Points(targets))} else {(!layers.is_empty()).then_some(DrawingSelectionTargets::Layers(layers))})
}

/// 🎯️ The dragged path points `point_ids` name on the selected paths `ids`, validated as a node drag validates them.
pub(crate) fn drawing_point_targets(document:&DrawingSnapshot,ids:&[String],point_ids:&[String])->Result<Vec<DrawingPathPointTarget>,Fault> {
    match drawing_selection_targets(document,true,ids,point_ids,[0.0,0.0])? {
        Some(DrawingSelectionTargets::Points(targets))=>Ok(targets),
        _=>Err(Fault::from("Select path points to move")),
    }
}

/// 🧮️ The one parametric leaf a nudge of the current selection by `delta` yields — `drag-path-points` under `editNodes`,
/// `drag-layers` under a select utility — `None` when the utility does not move or nothing is selected.
pub(crate) fn plan_selection(document:&DrawingSnapshot,utility:&str,ids:&[String],point_ids:&[String],delta:[f64;2])->Result<Option<DrawingMutation>,Fault> {
    let nodes=utility=="editNodes";
    if !nodes && !matches!(utility,"selectDirect"|"selectMarquee"|"selectLasso"|"transformMove") {return Ok(None);}
    Ok(match drawing_selection_targets(document,nodes,ids,point_ids,delta)? {
        Some(DrawingSelectionTargets::Layers(targets))=>Some(drag_layers(targets,delta[0],delta[1])),
        Some(DrawingSelectionTargets::Points(targets))=>Some(drag_path_points(targets,delta[0],delta[1])),
        None=>None,
    })
}

/// ⌨️ One keyboard nudge as ONE canvas tool transaction under the verb's tool id; a node nudge re-binds the point selection.
pub(crate) fn nudge(doc:&ArtifactView<'_,DrawingSnapshot>,session:&DrawingSession,verb:&str,delta:[f64;2])->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
    let Some(leaf)=plan_selection(doc.snapshot,&session.active_utility_id,&session.interaction.ids,&session.interaction.points,delta)? else {return Ok(Emit::default());};
    let mut tool=DrawingTool::start(verb,doc.operation_optional().map_or("",|operation|operation.authoring_seed.as_str()));
    let mut emit=drawing_tool_emit(tool.send(canvas_tool::Event::Once(leaf.clone())))?;
    if matches!(leaf,DrawingMutation::DragPathPoints(_)) {emit.effects.push(point_selection_effect(&drawing_rebound_points(doc.snapshot,&leaf)?));}
    Ok(emit)
}

macro_rules! nudge_command {
    ($module:ident,$name:ident,$verb:literal,$keyword:literal,$dx:expr,$dy:expr)=>{
        pub mod $module {
            use super::*;
            #[derive(Clone,Debug,PartialEq,dsl::ToValue,dsl::FromValue,dsl::DslRecord)]
            #[dsl(keyword=$keyword)]
            pub struct $name {}
            pub fn handle(_payload:&$name,doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,session:&mut DrawingSession)->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
                nudge(doc,session,$verb,[$dx,$dy])
            }
        }
    };
}

nudge_command!(nudge_selection_left,NudgeSelectionLeft,"nudgeSelectionLeft","nudge-selection-left",-1.0,0.0);
nudge_command!(nudge_selection_right,NudgeSelectionRight,"nudgeSelectionRight","nudge-selection-right",1.0,0.0);
nudge_command!(nudge_selection_up,NudgeSelectionUp,"nudgeSelectionUp","nudge-selection-up",0.0,-1.0);
nudge_command!(nudge_selection_down,NudgeSelectionDown,"nudgeSelectionDown","nudge-selection-down",0.0,1.0);
nudge_command!(nudge_selection_left_fast,NudgeSelectionLeftFast,"nudgeSelectionLeftFast","nudge-selection-left-fast",-10.0,0.0);
nudge_command!(nudge_selection_right_fast,NudgeSelectionRightFast,"nudgeSelectionRightFast","nudge-selection-right-fast",10.0,0.0);
nudge_command!(nudge_selection_up_fast,NudgeSelectionUpFast,"nudgeSelectionUpFast","nudge-selection-up-fast",0.0,-10.0);
nudge_command!(nudge_selection_down_fast,NudgeSelectionDownFast,"nudgeSelectionDownFast","nudge-selection-down-fast",0.0,10.0);

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
