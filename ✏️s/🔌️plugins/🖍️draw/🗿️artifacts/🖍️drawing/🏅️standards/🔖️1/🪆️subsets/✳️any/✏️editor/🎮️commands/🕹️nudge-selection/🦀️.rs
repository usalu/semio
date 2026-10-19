//! 🕹️ Document-axis keyboard movement for layers and path points — each nudge is ONE canvas tool transaction of the
//! relative leaf the drag gesture yields too (`drag-layers`, or `drag-path-points` under `editNodes`).
use semio_s_artifact_draw_drawing_editor_tool::DrawingSelectionTargets;
use semio_s_artifact_draw_drawing_editor_tool::drawing_selection_targets;
use crate::editor::drawing::commands::canvas_pointer_down::canvas_tool;
use semio_s_artifact_draw_drawing_editor_tool::{drawing_rebound_points, drawing_tool_emit, point_selection_effect, DrawingSession, DrawingTool};
use semio_s_artifact_draw_drawing_editor_interaction::points;
use semio_s_artifact_draw_drawing_transform::drag_layers::mutation::drag_layers;
use semio_s_artifact_draw_drawing_transform::drag_path_points::mutation::{drag_path_points, DrawingPathPointTarget};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_geometry::multiply;
use semio_s_artifact_draw_drawing_geometry::editing::{PathPointRef, translate_world_path_points};
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault,NoConfig,NoConfigMutation};
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_value::paged::PagedUtf8;
use semio_framework_value_persistent::list::PagedList;


/// 🧮️ The one parametric leaf a nudge of the current selection by `delta` yields — `drag-path-points` under `editNodes`,
/// `drag-layers` under a select utility — `None` when the utility does not move or nothing is selected.
pub fn plan_selection(document:&DrawingSnapshot,utility:&str,ids:&[String],point_ids:&[String],delta:[f64;2])->Result<Option<DrawingMutation>,Fault> {
    let nodes=utility=="editNodes";
    if !nodes && !matches!(utility,"selectDirect"|"selectMarquee"|"selectLasso"|"transformMove") {return Ok(None);}
    Ok(match drawing_selection_targets(document,nodes,ids,point_ids,delta)? {
        Some(DrawingSelectionTargets::Layers(targets))=>Some(drag_layers(targets,delta[0],delta[1])),
        Some(DrawingSelectionTargets::Points(targets))=>Some(drag_path_points(targets.into(),delta[0],delta[1])),
        None=>None,
    })
}

/// ⌨️ One keyboard nudge as ONE canvas tool transaction under the verb's tool id; a node nudge re-binds the point selection.
pub fn nudge(doc:&ArtifactView<'_,DrawingSnapshot>,session:&DrawingSession,verb:&str,delta:[f64;2])->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
    let Some(leaf)=plan_selection(doc.snapshot,&session.active_utility_id,&session.interaction.ids,&session.interaction.points,delta)? else {return Ok(Emit::default());};
    let mut tool=DrawingTool::start(verb,doc.operation_optional().map_or("",|operation|operation.authoring_seed.as_str()));
    let mut emit=drawing_tool_emit(tool.send(canvas_tool::Event::Once(leaf.clone())))?;
    if matches!(leaf,DrawingMutation::DragPathPoints(_)) {emit.effects.push(point_selection_effect(&drawing_rebound_points(doc.snapshot,&leaf)?));}
    Ok(emit)
}

macro_rules! nudge_command {
    ($module:ident,$name:ident,$verb:literal,$keyword:literal,$dx:expr,$dy:expr)=>{
        pub mod $module {

            #[derive(Clone,Debug,PartialEq,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned)]
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

