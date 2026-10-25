//! 🖱️ Drawing canvas pointer — `canvas-pointer-down`, the canvas TOOL and its retained session.
//!
//! 🛠️ The canvas tool is a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the `🛠️tool-machine` runner:
//! every gesture — a layer drag, a handle resize or rotation, a node drag, a shape drag, a pen or polygon draft, a trace, a
//! keyboard nudge — leaves as ONE `ToolTransaction` of relative, parametric leaves (`drag-layers`, `rotate-layers`,
//! `scale-layers`, `drag-path-points`, `create-layer`). Tool state is never history; the yielded mutations are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5). Marquee, lasso and click picks
//! are interaction, never mutations: the session derives their selection query from the tool's context at the release.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::create_layer_by_kind;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_s_artifact_draw_drawing_transform::drag_layers::mutation::drag_layers;
use semio_s_artifact_draw_drawing_transform::drag_path_points::mutation::{drag_path_points, DrawingPathPointTarget};
use semio_s_artifact_draw_drawing_geometry::multiply;
use semio_s_artifact_draw_drawing_geometry::editing::{PathPointRef, translate_world_path_points};
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_value::paged::PagedUtf8;
use semio_framework_value_persistent::list::PagedList;
use semio_s_artifact_draw_drawing_model::{DrawingImageAsset};
use semio_s_artifact_draw_drawing_model::{layer_base, layer_base_mut, find_drawing_layer, find_drawing_layer_location};
use semio_framework_plugin::{MediaType,MediaClass,MediaForm};
use semio_framework_plugin::kernel::{ClipboardFragment,ClipboardError,PastePlacement,PasteAnchor};
use semio_framework_value::paged::{Utf8Text};
use semio_s_artifact_draw_drawing_editor_interaction::{DRAWING_INTERACTION_DOMAIN, DRAWING_INTERACTION_GRANULARITY, DRAWING_POINT_DOMAIN, DRAWING_POINT_GRANULARITY};
use semio_s_artifact_draw_drawing_editor_interaction::points;
use crate::modes::edit::windows::canvas::config::DrawingCanvasWindowConfig;
use crate::modes::edit::windows::canvas::transient::DrawingCanvasWindowTransient;
use semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::rotate_layers;
use semio_s_artifact_draw_drawing_transform::scale_layers::mutation::scale_layers;
use semio_s_artifact_draw_drawing_geometry::handles::{handle_motion, HandleMotion};
use semio_s_artifact_draw_drawing_scene::paint::scene::query::{PreparedScenePickJob, PreparedScenePick, PreparedSceneIdentity};
use semio_s_artifact_draw_drawing_editor_interaction::geometry_session::MountedSceneQuery;
use semio_s_artifact_draw_drawing_model::{create_drawing_trace_layer, layer_id};
use semio_s_artifact_draw_drawing_model::create_drawing_path_layer;
use semio_s_artifact_draw_drawing_model::{PathSegment};
use semio_s_artifact_draw_drawing_geometry::editing::{PathPoint, path_point_hit};
use machine::Command;
use semio_framework_plugin::{kernel::Effect, AppOperationContext, RequestId, UiFixedList};
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolYield};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

//#region 🔖️ToolContext

#[path = "🎯️tool-context/🦀️.rs"]
mod tool_context;
pub use tool_context::*;

//#endregion 🔖️ToolContext

//#region 🔖️DocumentHelpers

//#endregion 🔖️DocumentHelpers

//#region 🛠️CanvasTool

#[path = "🛠️canvas-tool/🦀️.rs"]
mod canvas_tool_machine;
pub use canvas_tool_machine::*;

//#endregion 🛠️CanvasTool

//#region 🧵️TracePointerJob

#[path = "🧵️trace-pointer-job/🦀️.rs"]
mod trace_pointer_job;
pub use trace_pointer_job::*;

//#endregion 🧵️TracePointerJob

//#region 🔖️DrawingSession

#[path = "🖌️drawing-session/🦀️.rs"]
mod drawing_session;
pub use drawing_session::*;

//#endregion 🔖️DrawingSession

//#region 🧵️TracePointerContinuation

//#endregion 🧵️TracePointerContinuation

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
    #[value(default)]
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub app_instance_id: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_revision: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub world_x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub world_y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_completed_work: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_pending_work: Option<u64>,
}

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let operation = doc.operation()?;
    let document_revision = semio_s_artifact_draw_drawing_editor_interaction::drawing_document_revision(doc);
    if let Some(generation) = payload.generation {
        let Some(base_revision) = payload.base_revision.as_deref() else { return Ok(Emit::default()) };
        if base_revision.len() != 64
            || payload.parent_document_id.as_ref().is_some_and(|id| id.len() > 256)
            || session.active_utility_id != "trace"
            || session.window_transient.trace_pointer_generation != generation
            || payload.app_instance_id != Some(operation.app_instance_id)
            || payload.parent_document_id.as_deref() != Some(operation.parent_document_id.as_str())
            || base_revision != document_revision
        {
            return Ok(Emit::default());
        }
        let Some(operation_id) = payload.operation_id else { return Ok(Emit::default()) };
        let job = session.take_trace_pointer(operation.app_instance_id, &operation.parent_document_id, operation_id, generation, base_revision);
        let Some(job) = job else { return Ok(Emit::default()) };
        if payload.checkpoint_completed_work != Some(job.completed_work as u64) || payload.checkpoint_pending_work != Some(job.work.len() as u64) {
            let _ = session.retain_trace_pointer(job);
            return Ok(Emit::default());
        }
        let base = session.tool_base(doc);
        return advance_trace_pointer(session, job, payload, base);
    }
    let (world_x, world_y) = canvas_point_to_world(&session.window_config.viewport, payload.x, payload.y, payload.width, payload.height);
    let pointer = DrawingPointer { utility: session.active_utility_id.clone(), world: [world_x, world_y], shift: payload.shift, alt: payload.alt, ctrl: payload.ctrl, meta: payload.meta };
    if pointer.utility == "trace" {
        session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, session.window_transient.trace_pointer_generation);
        session.press(pointer)?;
        let base = session.tool_base(doc);
        return advance_trace_pointer(session, TracePointerJob::new_operation(operation, document, [world_x, world_y]), payload, base);
    }
    session.press(pointer)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️canvas-tool/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

/// 🎨️ Paint new artwork without replacing an explicitly supplied appearance.
pub fn initialize_appearance(layer: &mut semio_s_artifact_draw_drawing_model::DrawingLayerNode) {
    use semio_s_artifact_draw_drawing_model::{DrawingLayerNode, FillStyle, StrokeStyle};
    let filled=matches!(layer,DrawingLayerNode::Shape(shape) if shape.shape_kind!="line") || matches!(layer,DrawingLayerNode::Boolean(_) | DrawingLayerNode::Trace(_));
    let outlined=matches!(layer,DrawingLayerNode::Shape(_) | DrawingLayerNode::Path(_));
    let text=matches!(layer,DrawingLayerNode::Text(_));
    let attributes=&mut semio_s_artifact_draw_drawing_model::layer_base_mut(layer).attributes;
    if attributes.fill.is_some() || attributes.stroke.is_some() { return; }
    if filled { attributes.fill=Some(FillStyle::Solid { color:[0.2,0.7,0.65,1.0] }); }
    else if text { attributes.fill=Some(FillStyle::Solid { color:[0.0,0.0,0.0,1.0] }); }
    if outlined { attributes.stroke=Some(StrokeStyle { color:[0.1,0.15,0.2,1.0],width:2.0,cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round,join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Round,dash:None }); }
}

/// 🪪️ Admits a fresh identity at the explicit command IO boundary.
pub fn prepare_identity(document:&DrawingSnapshot,kind:&str,operation:&semio_framework_plugin::AppOperationContext,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_s_artifact_draw_drawing_model::identity::DrawingIdentity,Fault>{
 semio_s_artifact_draw_drawing_io::text::identity::creation::layer_identity(document,kind,operation,control).map_err(|error|Fault::from(error.to_string()))
}

/// 🎯️ What a selection transform addresses once validated against `document`: the selected layers without the ones a
/// selected ancestor already moves, or the selected points of the selected paths — `None` when nothing moves.
pub enum DrawingSelectionTargets {
    Layers(PagedList<PagedUtf8<{usize::MAX}>,{usize::MAX}>),
    Points(Vec<DrawingPathPointTarget>),
}

/// 🔎️ Validates the selection a transform by `delta` addresses — every selected layer exists, is visible and unlocked
/// through its whole ancestry and moves through a regular transform; every selected point belongs to a selected path at
/// its current geometry — and answers its targets in document order. A refusal changes nothing.
pub fn drawing_selection_targets(document:&DrawingSnapshot,nodes:bool,ids:&[String],point_ids:&[String],delta:[f64;2])->Result<Option<DrawingSelectionTargets>,Fault> {
    if ids.is_empty() || (nodes && point_ids.is_empty()) {return Ok(None);}
    if ids.len()>4_096 || point_ids.len()>4_096 {return Err(Fault::from("Selection exceeds keyboard movement capacity"));}
    let selected: BTreeSet<_>=ids.iter().map(|id|PagedUtf8::<{usize::MAX}>::from(id.as_str())).collect();
    let mut references:BTreeMap<PagedUtf8<{usize::MAX}>,Vec<points::PointSelectionRef<'_>>>=BTreeMap::new();
    if nodes {for id in point_ids {
        let point=points::parse_point_id(id).ok_or_else(||Fault::from("Invalid selected point"))?;
        if selected.contains(&PagedUtf8::from(point.layer_id)) {references.entry(point.layer_id.into()).or_default().push(point);}
    }}
    let wanted:BTreeSet<_>=if nodes {references.keys().cloned().collect()} else {selected};
    if wanted.is_empty() {return Ok(None);}
    let mut stack=vec![(&document.layers,0usize,[1.0,0.0,0.0,1.0,0.0,0.0],true,false)];
    let mut found=BTreeSet::new();
    let (mut layers,mut targets)=(PagedList::new(),Vec::new());
    let mut remaining=4_096usize;
    while let Some((children,index,parent,editable,ancestor_selected))=stack.last_mut() {
        let Some(layer)=children.get(*index) else {stack.pop();continue;};
        *index+=1;
        remaining=remaining.checked_sub(1).ok_or_else(||Fault::from("Drawing exceeds keyboard movement capacity"))?;
        let base=semio_s_artifact_draw_drawing_model::layer_base(layer);
        let editable=*editable && base.visible && !base.locked;
        let chosen=wanted.contains(&base.id);
        let ancestor_selected=*ancestor_selected;
        let parent=*parent;
        let matrix=multiply(parent,semio_s_artifact_draw_drawing_model::affine::drawing_transform_to_matrix(&base.transform));
        if chosen {
            found.insert(base.id.clone());
            if !editable {return Err(Fault::from("Unlock and show selected layers before moving"));}
            if nodes {
                let DrawingLayerNode::Path(path)=layer else {return Err(Fault::from("Selected points no longer belong to a path"));};
                remaining=remaining.checked_sub(path.segments.len()).ok_or_else(||Fault::from("Path exceeds keyboard movement capacity"))?;
                let before=points::geometry_id(&path.segments).ok_or_else(||Fault::from("Invalid path geometry"))?;
                let chosen_points=&references[&base.id];
                if chosen_points.iter().any(|point|point.geometry!=before) {return Err(Fault::from("Selected path points changed"));}
                let refs=chosen_points.iter().map(|point|PathPointRef {index:point.index,point:point.point}).collect::<Vec<_>>();
                translate_world_path_points(&path.segments,&refs,matrix,delta).map_err(Fault::from)?;
                targets.extend(chosen_points.iter().map(|point|DrawingPathPointTarget {layer_id:base.id.clone(),index:point.index,point:point.point}));
            } else if !ancestor_selected {
                let source=&base.transform;
                semio_s_artifact_draw_drawing_geometry::translation::translate([source.x,source.y,source.scale_x,source.scale_y,source.rotation],parent,delta).ok_or_else(||Fault::from("Cannot move through a singular or nonfinite transform"))?;
                layers.push(base.id.clone());
            }
        }
        if let DrawingLayerNode::Group(group)=layer {stack.push((&group.children,0,matrix,editable,ancestor_selected || chosen));}
    }
    if found!=wanted {return Err(Fault::from("A selected layer no longer exists"));}
    Ok(if nodes {(!targets.is_empty()).then_some(DrawingSelectionTargets::Points(targets))} else {(!layers.is_empty()).then_some(DrawingSelectionTargets::Layers(layers))})
}

/// 🎯️ The dragged path points `point_ids` name on the selected paths `ids`, validated as a node drag validates them.
pub fn drawing_point_targets(document:&DrawingSnapshot,ids:&[String],point_ids:&[String])->Result<Vec<DrawingPathPointTarget>,Fault> {
    match drawing_selection_targets(document,true,ids,point_ids,[0.0,0.0])? {
        Some(DrawingSelectionTargets::Points(targets))=>Ok(targets),
        _=>Err(Fault::from("Select path points to move")),
    }
}

//#region 🪢️TaxonomyMounts
#[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️canvas-commit-draft/🦀️.rs"]
pub mod canvas_commit_draft;
#[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-double-click/🦀️.rs"]
pub mod canvas_double_click;
#[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚪️canvas-escape/🦀️.rs"]
pub mod canvas_escape;
#[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
pub mod canvas_pointer_move;
#[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬆️canvas-pointer-up/🦀️.rs"]
pub mod canvas_pointer_up;
#[path = "."]
pub mod modes {
    #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
    pub mod edit;
}
//#endregion 🪢️TaxonomyMounts
