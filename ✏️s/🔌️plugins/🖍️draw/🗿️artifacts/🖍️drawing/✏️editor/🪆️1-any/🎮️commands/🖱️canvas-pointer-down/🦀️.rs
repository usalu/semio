//! 🖱️ Drawing canvas pointer — `canvas-pointer-down`, the canvas TOOL and its retained session.
//!
//! 🛠️ The canvas tool is a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the `🛠️tool-machine` runner:
//! every gesture — a layer drag, a handle resize or rotation, a node drag, a shape drag, a pen or polygon draft, a trace, a
//! keyboard nudge — leaves as ONE `ToolTransaction` of relative, parametric leaves (`drag-layers`, `rotate-layers`,
//! `scale-layers`, `drag-path-points`, `create-layer`). Tool state is never history; the yielded mutations are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5). Marquee, lasso and click picks
//! are interaction, never mutations: the session derives their selection query from the tool's context at the release.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::drawing::{DRAWING_INTERACTION_DOMAIN, DRAWING_INTERACTION_GRANULARITY, DRAWING_POINT_DOMAIN, DRAWING_POINT_GRANULARITY};
use crate::editor::drawing::interaction::points;
use crate::editor::drawing::modes::edit::windows::canvas::config::DrawingCanvasWindowConfig;
use crate::editor::drawing::modes::edit::windows::canvas::transient::DrawingCanvasWindowTransient;
use crate::mutations::{drag_layers, drag_path_points, rotate_layers, scale_layers, DrawingPathPointTarget};
use crate::op::DrawingMutation;
use crate::schema::geometry::handles::{handle_motion, HandleMotion};
use crate::schema::scene_paint::scene::query::{PreparedScenePickJob,PreparedScenePick,PreparedSceneIdentity};
use crate::editor::drawing::geometry_session::MountedSceneQuery;
use crate::schema::{create_drawing_trace_layer, layer_id};
use crate::standards::v1::subsets::any::io::text::snapshot::{create_drawing_path_layer};
use crate::{DrawingLayerNode, DrawingSnapshot, PathSegment};
use crate::schema::geometry::editing::{PathPoint,path_point_hit};
use machine::Command;
use semio_framework_plugin::{kernel::Effect, AppOperationContext, ArtifactView, ConfigView, Emit, Fault, RequestId, UiFixedList};
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolYield};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

//#region 🔖️ToolContext
pub const DRAWING_GESTURE_PREVIEW_POINT_CAPACITY: usize = 256;

/// 🪢 Bounded lasso polygon retained during incremental document traversal.
#[derive(Clone, Debug)]
pub struct LassoPolygon {
    points: [[f64;2];DRAWING_GESTURE_PREVIEW_POINT_CAPACITY],
    len: usize,
}

impl LassoPolygon {
    fn from_points(points: &UiFixedList<[f64;2],DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>, end: [f64;2]) -> Self {
        let mut polygon=Self { points: [[0.0;2];DRAWING_GESTURE_PREVIEW_POINT_CAPACITY],len: points.len() };
        for (index,point) in points.iter().enumerate() { polygon.points[index]=*point; }
        if polygon.len==0 || polygon.points[polygon.len-1]!=end {
            let index=polygon.len.min(DRAWING_GESTURE_PREVIEW_POINT_CAPACITY-1);
            polygon.points[index]=end;
            polygon.len=(index+1).min(DRAWING_GESTURE_PREVIEW_POINT_CAPACITY);
        }
        polygon
    }
    fn as_slice(&self) -> &[[f64;2]] { &self.points[..self.len] }
}

fn append_lasso_point(context: &mut DrawingToolContext, point: [f64;2], spacing: f64) {
    context.lasso_spacing=context.lasso_spacing.max(spacing).max(1e-9);
    if let Some(previous)=context.points.get(context.points.len().saturating_sub(1)) {
        if (point[0]-previous[0]).hypot(point[1]-previous[1])<context.lasso_spacing { return; }
    }
    if context.points.len()==DRAWING_GESTURE_PREVIEW_POINT_CAPACITY {
        let retained=context.points.len()/2;
        for index in 0..retained { let point=context.points[index*2]; context.points[index]=point; }
        while context.points.len()>retained { context.points.pop(); }
        context.lasso_spacing*=2.0;
    }
    if context.points.try_push(point).is_err() { context.points_overflowed=true; }
}

/// ✊️ What a press grabbed once the session resolved it against the document: layers — with the handle and the
/// selection bounds when a transform handle was hit — or path points.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawingGrab {
    Layers { targets: Vec<String>, handle: Option<(usize, [f64; 4])> },
    Points { targets: Vec<DrawingPathPointTarget> },
}

/// 🎛️ The canvas tool's context — one flat record (XState convention: context is machine-global, never per-state):
/// the press, the cursor, the marquee or draft points, and what a drag grabbed. Tool state, never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawingToolContext {
    pub(crate) method: String,
    lasso_spacing: f64,
    merge: String,
    pub(crate) utility: String,
    pub(crate) start: [f64; 2],
    pub(crate) cursor: [f64; 2],
    pub(crate) points: UiFixedList<[f64; 2], DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>,
    pub(crate) points_overflowed: bool,
    active: bool,
    pub(crate) grab: Option<DrawingGrab>,
    constrained: bool,
    centered: bool,
}

fn tool_context_from_input(_input: ()) -> DrawingToolContext {
    DrawingToolContext::default()
}
//#endregion 🔖️ToolContext

//#region 🔖️DocumentHelpers
pub(crate) fn canvas_point_to_world(viewport: &store::Viewport2d, x: f64, y: f64, viewport_w: f64, viewport_h: f64) -> (f64, f64) {
    let zoom = viewport.zoom.max(0.01);
    ((x - viewport_w * 0.5) / zoom + viewport.x, (y - viewport_h * 0.5) / zoom + viewport.y)
}

/// 🎯️ Maps shift/ctrl/meta modifiers to a framework `MergeMode` wire string (matches
/// `@semio-tech/ui-react`'s `marqueeModeFromModifiers`) — the actual set algebra runs inside the framework's
/// `next_selection` machine; this crate only ever computes WHICH ids were hit and asks the framework to apply them.
pub(crate) fn selection_merge_mode(shift: bool, ctrl: bool, meta: bool) -> &'static str {
    let ctrl_or_meta = ctrl || meta;
    if shift && ctrl_or_meta {
        "invertive"
    } else if shift {
        "additive"
    } else if ctrl_or_meta {
        "subtractive"
    } else {
        "replace"
    }
}

/// 🕹️ Requests the shell to redispatch a framework-owned interaction verb (`interactionSelect`/`interactionHover`)
/// through its normal action funnel — selection and hover are framework-owned state, never a document mutation.
pub(crate) fn request_interaction_action(action_id: &str, args: semio_framework_value::DslValue) -> Effect {
    Effect::ReplayShellCommand { action_id: action_id.into(), args: Some(args) }
}

pub(crate) fn interaction_select_effect_from_targets(targets: String, merge: &str) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_SELECT_ACTION_ID,
        semio_framework_value::DslValue::object([
            ("domainId".to_string(), semio_framework_value::DslValue::String(DRAWING_INTERACTION_DOMAIN.to_string())),
            ("targets".to_string(), semio_framework_value::DslValue::String(targets)),
            ("merge".to_string(), semio_framework_value::DslValue::String(merge.to_string())),
            ("method".to_string(), semio_framework_value::DslValue::String("pick".to_string())),
        ]),
    )
}

pub(crate) fn interaction_hover_effect_from_targets(targets: String) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_HOVER_ACTION_ID,
        semio_framework_value::DslValue::object([("domainId".to_string(), semio_framework_value::DslValue::String(DRAWING_INTERACTION_DOMAIN.to_string())), ("channel".to_string(), semio_framework_value::DslValue::String("pointer".to_string())), ("targets".to_string(), semio_framework_value::DslValue::String(targets))]),
    )
}

pub(crate) fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    let items = ids.iter().map(|id| semio_framework_value::DslValue::object([("granularity".to_string(), semio_framework_value::DslValue::String(DRAWING_INTERACTION_GRANULARITY.to_string())), ("id".to_string(), semio_framework_value::DslValue::String(id.clone()))])).collect::<Vec<_>>();
    let targets = semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(items));
    interaction_select_effect_from_targets(targets, merge)
}

pub(crate) fn point_selection_effect(ids:&[String])->Effect {
    let targets=ids.iter().map(|id|semio_framework_value::DslValue::object([("granularity".into(),semio_framework_value::DslValue::String(DRAWING_POINT_GRANULARITY.into())),("id".into(),semio_framework_value::DslValue::String(id.clone()))])).collect::<Vec<_>>();
    point_selection_effect_from_targets(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(targets)))
}

pub(crate) fn point_selection_effect_from_targets(targets:String)->Effect {
    request_interaction_action(semio_framework::INTERACTION_SELECT_ACTION_ID,semio_framework_value::DslValue::object([
        ("domainId".into(),semio_framework_value::DslValue::String(DRAWING_POINT_DOMAIN.into())),
        ("targets".into(),semio_framework_value::DslValue::String(targets)),
        ("merge".into(),semio_framework_value::DslValue::String("replace".into())),
        ("method".into(),semio_framework_value::DslValue::String("pick".into())),
    ]))
}

pub(crate) const DRAWING_MARQUEE_THRESHOLD_PX: f64 = 4.0;
pub(crate) const DRAWING_PICK_TOLERANCE_PX: f64 = 8.0;

pub(crate) fn shape_preview_segments(utility: &str, start: [f64; 2], end: [f64; 2]) -> UiFixedList<PathSegment, 7> {
    let mut segments = UiFixedList::default();
    if utility == "shapeLine" {
        let _ = segments.try_push(PathSegment::Move { to: start });
        let _ = segments.try_push(PathSegment::Line { to: end });
        return segments;
    }
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if utility == "shapeRect" {
        for segment in [PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close] {
            let _ = segments.try_push(segment);
        }
        return segments;
    }
    let cx = x + width / 2.0;
    let cy = y + height / 2.0;
    let rx = width / 2.0;
    let ry = height / 2.0;
    let k = 0.552_284_749_8;
    for segment in [
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + rx * k, cy - ry], ctrl2: [cx + rx, cy - ry * k], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + ry * k], ctrl2: [cx + rx * k, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - rx * k, cy + ry], ctrl2: [cx - rx, cy + ry * k], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - ry * k], ctrl2: [cx - rx * k, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ] {
        let _ = segments.try_push(segment);
    }
    segments
}

pub(crate) fn draft_preview_segments(utility: &str, points: &UiFixedList<[f64; 2], DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>, cursor: [f64; 2]) -> UiFixedList<PathSegment, { DRAWING_GESTURE_PREVIEW_POINT_CAPACITY + 2 }> {
    let mut segments = UiFixedList::default();
    if points.is_empty() {
        return segments;
    }
    let _ = segments.try_push(PathSegment::Move { to: points[0] });
    for point in points.iter().skip(1) {
        let _ = segments.try_push(PathSegment::Line { to: *point });
    }
    let _ = segments.try_push(PathSegment::Line { to: cursor });
    if utility == "shapePolygon" && points.len() > 1 {
        let _ = segments.try_push(PathSegment::Close);
    }
    segments
}

/// 🔷️ The replay-stable id of a dragged shape: its kind, geometry and document ordinal, scoped by the admitted operation.
fn shape_drag_id(utility: &str, geometry: [f64; 4], layer_ordinal: usize, operation: Option<&AppOperationContext>) -> String {
    let mut identity = Vec::with_capacity(1 + 4 + 8 + operation.map_or(0, |operation| operation.parent_document_id.len() + 60) + 32);
    identity.push(match utility {
        "shapeLine" => 1,
        "shapeEllipse" => 2,
        _ => 0,
    });
    if let Some(operation) = operation {
        identity.extend_from_slice(&operation.app_instance_id.to_be_bytes());
        identity.extend_from_slice(&(operation.parent_document_id.len() as u64).to_be_bytes());
        identity.extend_from_slice(operation.parent_document_id.as_bytes());
        identity.extend_from_slice(&operation.operation_id.to_be_bytes());
        identity.extend_from_slice(&operation.generation.to_be_bytes());
        identity.extend_from_slice(&operation.canonical_base_revision);
    }
    identity.extend_from_slice(&(layer_ordinal as u64).to_be_bytes());
    for value in geometry {
        identity.extend_from_slice(&value.to_bits().to_be_bytes());
    }
    crate::schema::create_drawing_id("shape", &identity)
}

/// 🔷️ The `create-layer` a shape drag yields, `None` when the drag is too small to commit.
fn shape_drag_layer(doc: &DrawingSnapshot, utility: &str, start: [f64; 2], end: [f64; 2], operation: Option<&AppOperationContext>) -> Option<DrawingMutation> {
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if width < 1.0 && height < 1.0 {
        return None;
    }
    let (name, shape_kind, geometry) = match utility {
        "shapeLine" => ("Line", "line", [start[0], start[1], end[0], end[1]]),
        "shapeEllipse" => ("Ellipse", "ellipse", [x + width / 2.0, y + height / 2.0, width / 2.0, height / 2.0]),
        _ => ("Rectangle", "rect", [x, y, width, height]),
    };
    let mut base = crate::schema::default_layer_base(name);
    base.id = shape_drag_id(utility, geometry, doc.layers.len(), operation);
    let mut layer = DrawingLayerNode::Shape(crate::DrawingShapeBody {
        base,
        shape_kind: shape_kind.into(),
        rect: if utility == "shapeRect" { Some(crate::DrawingRect { x, y, width, height }) } else { None },
        ellipse: if utility == "shapeEllipse" { Some(crate::DrawingEllipse { cx: x + width / 2.0, cy: y + height / 2.0, rx: width / 2.0, ry: height / 2.0 }) } else { None },
        circle: None,
        line: if utility == "shapeLine" { Some(crate::DrawingLine { x1: start[0], y1: start[1], x2: end[0], y2: end[1] }) } else { None },
        polygon: None,
    });
    crate::editor::drawing::commands::add_layer::initialize_appearance(&mut layer);
    Some(crate::mutations::create_layer(None, Some(doc.layers.len()), layer))
}

/// 🖊️ The `create-layer` a committed pen or polygon draft yields, `None` below two points.
fn draft_layer(doc: &DrawingSnapshot, utility: &str, points: &UiFixedList<[f64; 2], DRAWING_GESTURE_PREVIEW_POINT_CAPACITY>, operation: Option<&AppOperationContext>) -> Option<DrawingMutation> {
    if points.len() < 2 {
        return None;
    }
    let mut layer = if utility == "pen" {
        create_drawing_path_layer("Path", points.iter().enumerate().map(|(index, point)| if index == 0 { PathSegment::Move { to: *point } } else { PathSegment::Line { to: *point } }).collect())
    } else {
        DrawingLayerNode::Shape(crate::DrawingShapeBody {
            base: crate::schema::default_layer_base("Polygon"),
            shape_kind: "polygon".into(),
            rect: None,
            ellipse: None,
            circle: None,
            line: None,
            polygon: Some(crate::DrawingPolygon { points: points.iter().copied().collect() }),
        })
    };
    crate::editor::drawing::commands::add_layer::identify_created_layer(doc, &mut layer, if utility == "pen" { "path" } else { "shape:polygon" }, operation);
    crate::editor::drawing::commands::add_layer::initialize_appearance(&mut layer);
    Some(crate::mutations::create_layer(None, Some(doc.layers.len()), layer))
}

/// 🖼️ The `create-layer` a finished trace yields, `None` without an image source.
fn trace_layer(doc: &DrawingSnapshot, source_key: Option<&str>) -> Option<DrawingMutation> {
    let mut layer = create_drawing_trace_layer("Trace", source_key?);
    crate::editor::drawing::commands::add_layer::initialize_appearance(&mut layer);
    Some(crate::mutations::create_layer(None, Some(doc.layers.len()), layer))
}

/// 🧰️ The host effects a committed creation carries beside its one edit: the created layers become the selection and the
/// canvas returns to the default select utility (the active utility is host-owned, never a document operation).
fn created_layer_effects(mutations: &[DrawingMutation]) -> Vec<Effect> {
    let created = mutations.iter().filter_map(|mutation| match mutation {
        DrawingMutation::CreateLayer(create) => Some(layer_id(&create.layer).to_string()),
        _ => None,
    }).collect::<Vec<_>>();
    if created.is_empty() {
        return Vec::new();
    }
    vec![interaction_select_effect(&created, "replace"), Effect::SetActiveUtility { window_id: crate::editor::drawing::DRAWING_PLAY_WINDOW_CANVAS.into(), utility_id: crate::editor::drawing::DRAWING_DEFAULT_UTILITY.into() }]
}

/// 🎚️ The world-space motion a drag grabbed so far, as the parametric leaf it yields — `None` for the identity or a
/// motion no leaf admits (a non-finite offset, a collapsing zero scale).
pub(crate) fn drawing_grab_leaf(grab: &DrawingGrab, start: [f64; 2], cursor: [f64; 2], constrained: bool, centered: bool) -> Option<DrawingMutation> {
    match grab {
        DrawingGrab::Layers { targets, handle: None } => {
            let (dx, dy) = (cursor[0] - start[0], cursor[1] - start[1]);
            (dx.is_finite() && dy.is_finite() && (dx, dy) != (0.0, 0.0)).then(|| drag_layers(targets.clone(), dx, dy))
        }
        DrawingGrab::Layers { targets, handle: Some((handle, bounds)) } => match handle_motion(*handle, *bounds, start, cursor, constrained, centered)? {
            HandleMotion::Rotate { pivot, angle } => (angle != 0.0).then(|| rotate_layers(targets.clone(), pivot[0], pivot[1], angle)),
            HandleMotion::Scale { pivot, scale } => (scale != [1.0, 1.0] && scale[0] != 0.0 && scale[1] != 0.0).then(|| scale_layers(targets.clone(), pivot[0], pivot[1], scale[0], scale[1])),
        },
        DrawingGrab::Points { targets } => {
            let mut delta = [cursor[0] - start[0], cursor[1] - start[1]];
            if constrained {
                if delta[0].abs() >= delta[1].abs() { delta[1] = 0.0; } else { delta[0] = 0.0; }
            }
            (delta.iter().all(|value| value.is_finite()) && delta != [0.0, 0.0]).then(|| drag_path_points(targets.clone(), delta[0], delta[1]))
        }
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🛠️CanvasTool
/// 🪪️ The editor app id every canvas tool transaction's `tool` is scoped by: `<appId>#<utility or verb>`.
pub const DRAWING_EDITOR_APP_ID: &str = "s.draw.drawing@1/*#editor";

/// 🔑️ The transaction key of a gesture's one parametric leaf — every drag tick upserts it, so an open transaction always
/// holds ONE net leaf.
pub const DRAWING_TOOL_LEAF_KEY: &str = "gesture";

/// 🖱️ One press or release of the canvas pointer, in world coordinates, with its modifiers.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawingPointer {
    pub utility: String,
    pub world: [f64; 2],
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
}

/// ↔️ One pointer sample: the world position, the drag threshold in world units and the live transform modifiers.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawingPointerMove {
    pub world: [f64; 2],
    pub threshold: f64,
    pub constrained: bool,
    pub centered: bool,
}

/// 🧱️ The committed document a commit yields against, and the admission that scopes every id it mints — dispatch inputs,
/// never tool state.
#[derive(Clone, Debug)]
pub struct DrawingToolBase {
    pub document: Arc<DrawingSnapshot>,
    pub operation: Option<AppOperationContext>,
}

/// ⬆️ One release: the pointer, the drag threshold in world units, and the base a commit yields against.
#[derive(Clone, Debug)]
pub struct DrawingRelease {
    pub pointer: DrawingPointer,
    pub threshold: f64,
    pub base: DrawingToolBase,
}

/// 🖼️ A finished trace: the base and the image source it traces, if the pointer found one.
#[derive(Clone, Debug)]
pub struct DrawingTraced {
    pub base: DrawingToolBase,
    pub source_key: Option<String>,
}

fn pressed(event: Option<&canvas_tool::Event>) -> Option<&DrawingPointer> {
    match event {
        Some(canvas_tool::Event::PointerDown(pointer)) => Some(pointer),
        _ => None,
    }
}

fn press_grabs(_ctx: &DrawingToolContext, event: Option<&canvas_tool::Event>) -> bool {
    pressed(event).is_some_and(|pointer| pointer.utility == "editNodes" || (pointer.utility == "selectDirect" && !pointer.ctrl && !pointer.meta))
}

fn press_marquee(_ctx: &DrawingToolContext, event: Option<&canvas_tool::Event>) -> bool {
    pressed(event).is_some_and(|pointer| matches!(pointer.utility.as_str(), "selectMarquee" | "selectLasso"))
}

fn press_shape(_ctx: &DrawingToolContext, event: Option<&canvas_tool::Event>) -> bool {
    pressed(event).is_some_and(|pointer| matches!(pointer.utility.as_str(), "shapeRect" | "shapeEllipse" | "shapeLine"))
}

fn press_draft(_ctx: &DrawingToolContext, event: Option<&canvas_tool::Event>) -> bool {
    pressed(event).is_some_and(|pointer| matches!(pointer.utility.as_str(), "pen" | "shapePolygon"))
}

/// 🖊️ Drafting self-loop: the same pen/polygon utility is still active, so the press appends a point.
fn press_same_draft(ctx: &DrawingToolContext, event: Option<&canvas_tool::Event>) -> bool {
    press_draft(ctx, event) && pressed(event).is_some_and(|pointer| pointer.utility == ctx.utility)
}

/// 🖊️ Drafting restart: a different pen/polygon utility switched in.
fn press_other_draft(ctx: &DrawingToolContext, event: Option<&canvas_tool::Event>) -> bool {
    press_draft(ctx, event) && pressed(event).is_some_and(|pointer| pointer.utility != ctx.utility)
}

fn press_trace(_ctx: &DrawingToolContext, event: Option<&canvas_tool::Event>) -> bool {
    pressed(event).is_some_and(|pointer| pointer.utility == "trace")
}

fn start_press(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(pointer) = pressed(event) {
        ctx.utility = pointer.utility.clone();
        ctx.start = pointer.world;
        ctx.cursor = pointer.world;
        ctx.active = false;
        ctx.grab = None;
        ctx.constrained = pointer.shift;
        ctx.centered = pointer.alt;
    }
}

fn grab(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(canvas_tool::Event::Grab(grabbed)) = event {
        ctx.grab = Some(grabbed.clone());
        ctx.active = false;
    }
}

fn track(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(canvas_tool::Event::PointerMove(sample)) = event {
        ctx.cursor = sample.world;
    }
}

/// ✊️ One drag sample: the cursor and the live modifiers move, the drag activates past the threshold, and from then on
/// the net leaf is upserted under the one gesture key (retracted while the motion is the identity).
fn drag(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    let Some(canvas_tool::Event::PointerMove(sample)) = event else { return };
    if !sample.world.iter().all(|value| value.is_finite()) {
        return;
    }
    ctx.cursor = sample.world;
    ctx.constrained = sample.constrained;
    ctx.centered = sample.centered;
    ctx.active |= (sample.world[0] - ctx.start[0]).hypot(sample.world[1] - ctx.start[1]) >= sample.threshold;
    if !ctx.active {
        return;
    }
    let leaf = ctx.grab.as_ref().and_then(|grab| drawing_grab_leaf(grab, ctx.start, ctx.cursor, ctx.constrained, ctx.centered));
    sink.push(Command::Effect(match leaf {
        Some(leaf) => ToolYield::upsert(DRAWING_TOOL_LEAF_KEY, leaf),
        None => ToolYield::retract(DRAWING_TOOL_LEAF_KEY),
    }));
}

/// ⬆️ The release of a drag: the final sample settles the leaf and the transaction commits — an inactive or identity drag
/// commits empty, which publishes nothing.
fn release_drag(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    let Some(canvas_tool::Event::PointerUp(release)) = event else { return };
    if release.pointer.world.iter().all(|value| value.is_finite()) {
        ctx.cursor = release.pointer.world;
        ctx.constrained = release.pointer.shift;
        ctx.centered = release.pointer.alt;
        ctx.active |= (ctx.cursor[0] - ctx.start[0]).hypot(ctx.cursor[1] - ctx.start[1]) >= release.threshold;
    }
    let leaf = (ctx.active && ctx.cursor != ctx.start).then(|| ctx.grab.as_ref().and_then(|grab| drawing_grab_leaf(grab, ctx.start, ctx.cursor, ctx.constrained, ctx.centered))).flatten();
    sink.push(Command::Effect(match leaf {
        Some(leaf) => ToolYield::upsert(DRAWING_TOOL_LEAF_KEY, leaf),
        None => ToolYield::retract(DRAWING_TOOL_LEAF_KEY),
    }));
    sink.push(Command::Effect(ToolYield::Commit));
    ctx.grab = None;
    ctx.active = false;
}

/// 🧯️ Escape during a drag: the open transaction aborts with zero trace.
fn abort_drag(ctx: &mut DrawingToolContext, _event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    ctx.grab = None;
    ctx.active = false;
    sink.push(Command::Effect(ToolYield::Abort));
}

fn start_marquee(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    let pointer = match event {
        Some(canvas_tool::Event::PointerDown(pointer) | canvas_tool::Event::NodeMarquee(pointer)) => pointer,
        _ => return,
    };
    ctx.method = if pointer.utility == "selectLasso" { "lasso".into() } else { "rectangle".into() };
    ctx.start = pointer.world;
    ctx.cursor = pointer.world;
    ctx.merge = selection_merge_mode(pointer.shift, pointer.ctrl, pointer.meta).into();
    ctx.active = false;
    ctx.grab = None;
    ctx.lasso_spacing = 0.0;
    ctx.points = UiFixedList::default();
    if ctx.method == "lasso" && ctx.points.try_push(pointer.world).is_err() {
        ctx.points_overflowed = true;
    }
}

fn track_marquee(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(canvas_tool::Event::PointerMove(sample)) = event {
        let distance = ((sample.world[0] - ctx.start[0]).powi(2) + (sample.world[1] - ctx.start[1]).powi(2)).sqrt();
        ctx.active = ctx.active || distance >= sample.threshold;
        if ctx.method == "lasso" {
            append_lasso_point(ctx, sample.world, sample.threshold / 4.0);
        }
        ctx.cursor = sample.world;
    }
}

fn start_shape(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(pointer) = pressed(event) {
        ctx.utility = pointer.utility.clone();
        ctx.start = pointer.world;
        ctx.cursor = pointer.world;
    }
}

/// 🔷️ The release of a shape drag yields its `create-layer` and commits — a drag too small to draw yields nothing.
fn commit_shape(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    let Some(canvas_tool::Event::PointerUp(release)) = event else { return };
    ctx.cursor = release.pointer.world;
    if let Some(layer) = shape_drag_layer(&release.base.document, &ctx.utility, ctx.start, release.pointer.world, release.base.operation.as_ref()) {
        sink.push(Command::Effect(ToolYield::upsert(DRAWING_TOOL_LEAF_KEY, layer)));
        sink.push(Command::Effect(ToolYield::Commit));
    }
}

fn start_draft(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(pointer) = pressed(event) {
        ctx.utility = pointer.utility.clone();
        ctx.points = UiFixedList::default();
        if ctx.points.try_push(pointer.world).is_err() {
            ctx.points_overflowed = true;
        }
        ctx.cursor = pointer.world;
    }
}

fn append_draft_point(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(pointer) = pressed(event) {
        if ctx.points.len() < MAX_GESTURE_POINTS && ctx.points.try_push(pointer.world).is_err() {
            ctx.points_overflowed = true;
        }
        ctx.cursor = pointer.world;
    }
}

/// 🖊️ Committing a draft yields its `create-layer` and commits — fewer than two points yield nothing.
fn yield_draft(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    let Some(canvas_tool::Event::CommitDraft(base)) = event else { return };
    if let Some(layer) = draft_layer(&base.document, &ctx.utility, &ctx.points, base.operation.as_ref()) {
        sink.push(Command::Effect(ToolYield::upsert(DRAWING_TOOL_LEAF_KEY, layer)));
        sink.push(Command::Effect(ToolYield::Commit));
    }
    ctx.points = UiFixedList::default();
}

fn start_trace(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(pointer) = pressed(event) {
        ctx.utility = pointer.utility.clone();
        ctx.start = pointer.world;
        ctx.cursor = pointer.world;
    }
}

/// 🖼️ A finished trace yields its `create-layer` and commits — no image source yields nothing.
fn commit_trace(_ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    let Some(canvas_tool::Event::Traced(traced)) = event else { return };
    if let Some(layer) = trace_layer(&traced.base.document, traced.source_key.as_deref()) {
        sink.push(Command::Effect(ToolYield::upsert(DRAWING_TOOL_LEAF_KEY, layer)));
        sink.push(Command::Effect(ToolYield::Commit));
    }
}

/// ⌨️ A one-shot selection transform (a keyboard nudge) yields its planned leaf and commits in one event.
fn yield_once(_ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
    if let Some(canvas_tool::Event::Once(leaf)) = event {
        sink.push(Command::Effect(ToolYield::upsert(DRAWING_TOOL_LEAF_KEY, leaf.clone())));
        sink.push(Command::Effect(ToolYield::Commit));
    }
}

// 🎭️ The canvas tool's control flow (plain comment: rustdoc cannot document a macro invocation). `pressing` waits for the
// session to resolve what a select press grabbed; `dragging` yields the net leaf per sample and commits on release.
machine::statechart! {
    machine canvas_tool {
        context: DrawingToolContext;
        event Event {
            PointerDown(DrawingPointer),
            PointerMove(DrawingPointerMove),
            PointerUp(DrawingRelease),
            Grab(DrawingGrab),
            NodeMarquee(DrawingPointer),
            CommitDraft(DrawingToolBase),
            Traced(DrawingTraced),
            Once(DrawingMutation),
            Escape,
        }
        input: ();
        output: ();
        effect: ToolYield<DrawingMutation>;
        context_from_input: tool_context_from_input;
        initial: idle;

        state idle {
            on PointerDown if press_grabs => pressing do start_press;
            on PointerDown if press_marquee => marqueeing do start_marquee;
            on PointerDown if press_shape => shaping do start_shape;
            on PointerDown if press_draft => drafting do start_draft;
            on PointerDown if press_trace => tracing do start_trace;
            on Once => idle do yield_once;
        }
        state pressing {
            on Grab => dragging do grab;
            on NodeMarquee => marqueeing do start_marquee;
            on PointerMove => pressing do track;
            on PointerUp => idle;
            on Escape => idle;
        }
        state dragging {
            on PointerMove => dragging do drag;
            on PointerUp => idle do release_drag;
            on Escape => idle do abort_drag;
        }
        state marqueeing {
            on PointerMove => marqueeing do track_marquee;
            on PointerUp => idle;
            on PointerDown if press_marquee => marqueeing do start_marquee;
            on PointerDown if press_shape => shaping do start_shape;
            on PointerDown if press_draft => drafting do start_draft;
            on PointerDown if press_trace => tracing do start_trace;
            on Escape => idle;
        }
        state shaping {
            on PointerMove => shaping do track;
            on PointerUp => idle do commit_shape;
            on PointerDown if press_marquee => marqueeing do start_marquee;
            on PointerDown if press_shape => shaping do start_shape;
            on PointerDown if press_draft => drafting do start_draft;
            on PointerDown if press_trace => tracing do start_trace;
            on Escape => idle;
        }
        state drafting {
            on PointerMove => drafting do track;
            on PointerDown if press_same_draft => drafting do append_draft_point;
            on PointerDown if press_other_draft => drafting do start_draft;
            on PointerDown if press_marquee => marqueeing do start_marquee;
            on PointerDown if press_shape => shaping do start_shape;
            on PointerDown if press_trace => tracing do start_trace;
            on CommitDraft => idle do yield_draft;
            on Escape => idle;
        }
        state tracing {
            on Traced => idle do commit_trace;
            on PointerDown if press_trace => tracing do start_trace;
            on Escape => idle;
        }
    }
}

/// 🧷️ The canvas tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct DrawingToolHost;

impl machine::Host<canvas_tool::CanvasTool> for DrawingToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<DrawingMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

static DRAWING_TOOL_TICK: AtomicU64 = AtomicU64::new(0);

/// ⏰️ The host clock a canvas tool event runs on: the host's wall time and a process-monotone tick, so a transaction id
/// minted at an upsert is unique per author, moment and event even when two gestures share a millisecond.
pub fn drawing_tool_clock() -> protocol::HybridLogicalTimestamp {
    semio_framework_tool_machine::authoring_clock(DRAWING_TOOL_TICK.fetch_add(1, Ordering::Relaxed))
}

/// 🛠️ One canvas tool: the runner of the canvas statechart for one utility, its transactions scoped
/// `<appId>#<utility>` and minted from the admission's authoring seed. Lives in the retained gesture session (ephemeral,
/// local, one per app instance and utility; a moved base or a utility change retires it with zero trace).
pub struct DrawingTool {
    runner: ToolMachineRunner<canvas_tool::CanvasTool, DrawingToolHost>,
}

impl DrawingTool {
    /// 🚀️ The tool at rest for `utility` (or a command verb), minting from `authoring_seed`.
    pub fn start(tool: &str, authoring_seed: &str) -> Self {
        let runner = ToolMachineRunner::start(format!("{DRAWING_EDITOR_APP_ID}#{tool}"), protocol::ActorId(authoring_seed.to_string()), (), DrawingToolHost).expect("the canvas tool enters its initial configuration without yielding");
        Self { runner }
    }

    /// 🔎️ Whether the tool's configuration holds the state with this stable id.
    pub fn matches(&self, state: &str) -> bool {
        self.runner.snapshot().matches(state)
    }

    /// 🛋️ Whether the tool rests (no gesture in flight).
    pub fn at_rest(&self) -> bool {
        self.runner.at_rest()
    }

    /// 🎛️ The tool's context (ephemeral, never history).
    pub fn context(&self) -> &DrawingToolContext {
        &self.runner.snapshot().context
    }

    /// 📝️ The provisional leaf of the open transaction, if any.
    pub fn provisional(&self) -> Option<&DrawingMutation> {
        self.runner.transaction().and_then(|transaction| transaction.entries().first()).map(|(_, mutation)| mutation)
    }

    /// 📨️ Runs one event on the host clock.
    pub fn send(&mut self, event: canvas_tool::Event) -> Result<ToolStep<DrawingMutation>, ToolRefusal> {
        self.runner.send(event, drawing_tool_clock())
    }

    /// 🧯️ Host abort: the open transaction vanishes with zero trace and the tool rests.
    pub fn abort(&mut self, reason: ToolAbortReason) -> ToolStep<DrawingMutation> {
        self.runner.abort(reason)
    }
}

/// 📤️ What one tool step publishes: ONE edit stamped with the transaction for a commit, nothing otherwise; a refused event
/// is a fault (the tool broke its own transaction law) and publishes nothing.
pub fn drawing_tool_emit(step: Result<ToolStep<DrawingMutation>, ToolRefusal>) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    match step.map_err(|refusal| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(refusal.code()), "the Drawing canvas tool refused its own transaction"))? {
        ToolStep::Committed(transaction, mutations) => {
            let effects = created_layer_effects(&mutations);
            let mut emit = Emit::commit_transaction(transaction, mutations);
            emit.effects.extend(effects);
            Ok(emit)
        }
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => Ok(Emit::default()),
    }
}
//#endregion 🛠️CanvasTool

//#region 🧵️TracePointerJob
const TRACE_POINTER_WORK_PER_STEP: usize = 32;
const TRACE_POINTER_MAX_DEPTH: usize = 32;
const TRACE_POINTER_WORK_CAPACITY: usize = TRACE_POINTER_MAX_DEPTH * 3 + 4;
const DRAWING_QUERY_HIT_CAPACITY: usize = 256;
const DRAWING_QUERY_TARGET_BYTES: usize = 8_192;
const MAX_GESTURE_POINTS: usize = 48;
static NEXT_TRACE_POINTER_REQUEST: AtomicU64 = AtomicU64::new(20_000);

#[derive(Clone, Copy, Debug, Default, semio_framework_value::ToValue, semio_framework_value::FromValue)]
struct TracePath {
    indices: [u16; TRACE_POINTER_MAX_DEPTH],
    len: u8,
}

impl TracePath {
    fn root(index: usize) -> Option<Self> {
        let mut path = Self::default();
        path.indices[0] = u16::try_from(index).ok()?;
        path.len = 1;
        Some(path)
    }

    fn child(mut self, index: usize) -> Option<Self> {
        let next = usize::from(self.len);
        if next >= TRACE_POINTER_MAX_DEPTH {
            return None;
        }
        self.indices[next] = u16::try_from(index).ok()?;
        self.len += 1;
        Some(self)
    }

    fn indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.indices[..usize::from(self.len)].iter().map(|index| usize::from(*index))
    }
}

#[derive(Clone, Debug)]
enum TracePointerWork {
    SelectionPaths {next:usize},
    SelectionBounds {next:usize},
    ControlNodes {next:usize},
    ControlSegments {index:usize,path:TracePath,next:usize,matrix:[f64;6]},
    NodeArea {path:TracePath,next:usize,matrix:[f64;6]},
    PublishNodeArea {path:TracePath,next:usize,geometry:String},
    Pick,
    PublishPick {next:usize},
}

#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub(crate) struct TracePickCandidate {
    generality: i32,
    pub(crate) layer_id: String,
    image_key: Option<String>,
    path: TracePath,
}

// No `ToValue`/`FromValue`: `work`/`hits` are framework `UiFixedList` fields with no `ToValue`
// impl — never serialized in this plugin (grep-confirmed).
pub(crate) struct TracePointerJob {
    app_instance_id: u32,
    document_id: String,
    operation_id: u64,
    generation: u64,
    base_revision: String,
    world: [f64; 2],
    tolerance: f64,
    include_control_points: bool,
    marquee: Option<([f64; 2], [f64; 2], bool)>,
    lasso: Option<LassoPolygon>,
    work: UiFixedList<TracePointerWork, TRACE_POINTER_WORK_CAPACITY>,
    pub(crate) best: Option<TracePickCandidate>,
    pub(crate) hits: UiFixedList<String, DRAWING_QUERY_HIT_CAPACITY>,
    pub(crate) node_editing: bool,
    node_area:bool,
    node_hash:Option<semio_framework_hash::Hasher>,
    node_indices:Vec<usize>,
    node_bytes:usize,
    node_hit: Option<(TracePath,usize,PathPoint,f64)>,
    selected_ids: Vec<String>,
    selected_paths: Vec<TracePath>,
    selection_bounds: Option<[f64;4]>,
    completed_work: usize,
    cache_identity:Option<PreparedSceneIdentity>,
    pick:Option<PreparedScenePickJob>,
    pub(crate) failure:Option<String>,
    pub(crate) overflowed: bool,
}

impl TracePointerJob {
    fn new(generation: u64, document: &DrawingSnapshot, world: [f64; 2]) -> Self {
        Self::new_bound(generation, document, world, format!("unbound:{}", document.id))
    }

    fn new_bound(generation: u64, document: &DrawingSnapshot, world: [f64; 2], base_revision: String) -> Self {
        let mut work = UiFixedList::default();
        let _ = work.try_push(TracePointerWork::SelectionPaths {next:0});
        Self {
            app_instance_id: 0,
            document_id: document.id.clone(),
            operation_id: 0,
            generation,
            base_revision,
            world,
            tolerance: 0.0,
            include_control_points: false,
            marquee: None,
            lasso: None,
            work,
            best: None,
            hits: UiFixedList::default(),
            node_editing: false,
            node_area:false,node_hash:None,node_indices:Vec::new(),node_bytes:0,
            node_hit: None,
            selected_ids: Vec::new(),
            selected_paths: Vec::new(),
            selection_bounds: None,
            completed_work: 0,
            cache_identity:None,pick:None,failure:None,
            overflowed: false,
        }
    }

    fn new_operation(operation: &AppOperationContext, document: &DrawingSnapshot, world: [f64; 2]) -> Self {
        let mut job = Self::new_bound(operation.generation, document, world, operation.canonical_base_revision_hex());
        job.app_instance_id = operation.app_instance_id;
        job.document_id = operation.parent_document_id.clone();
        job.operation_id = operation.operation_id;
        job
    }

    pub(crate) fn new_query(document: &DrawingSnapshot, world: [f64; 2], tolerance: f64, include_control_points: bool) -> Self {
        let mut job = Self::new(0, document, world);
        job.tolerance = tolerance.max(0.0);
        job.include_control_points = include_control_points;
        job
    }

    pub(crate) fn retain_selection_bounds(&mut self,ids: &[String]) -> Result<(),Fault> {
        if ids.len()>DRAWING_QUERY_HIT_CAPACITY || ids.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Selection exceeds gesture capacity"));}
        self.selected_ids=ids.to_vec();
        Ok(())
    }

    pub(crate) fn new_marquee(document: &DrawingSnapshot, start: [f64; 2], end: [f64; 2], crossing: bool) -> Self {
        let mut job = Self::new(0, document, end);
        job.marquee = Some((start, end, crossing));
        job
    }

    pub(crate) fn new_lasso(document: &DrawingSnapshot, polygon: LassoPolygon) -> Self {
        let mut job=Self::new(0,document,[0.0;2]);
        job.lasso=Some(polygon);
        job
    }

    fn push_work(&mut self, work: TracePointerWork) {
        if self.work.try_push(work).is_err() {
            self.overflowed = true;
        }
    }

    /// 🪪️ Every granted step and eventual publication refers to the exact complete cache.
    pub(crate) fn validate_cache(&self,borrowed:&MountedSceneQuery<'_>)->Result<(),Fault>{
        let live=PreparedSceneIdentity{source:borrowed.source,build:borrowed.build,flatness:borrowed.scene.flatness};
        if self.cache_identity!=Some(live) {return Err(Fault::from("Drawing geometry changed during the pointer query"));}
        if let Some(error)=&self.failure {return Err(Fault::from(error.clone()));}
        Ok(())
    }

    pub(crate) fn advance(&mut self,document:&DrawingSnapshot,borrowed:&MountedSceneQuery<'_>)->bool {
        if self.overflowed{return true;}
        let live=PreparedSceneIdentity{source:borrowed.source,build:borrowed.build,flatness:borrowed.scene.flatness};
        if self.cache_identity.is_none(){self.cache_identity=Some(live);}
        if let Err(error)=self.validate_cache(borrowed){self.failure=Some(error.message);self.overflowed=true;return true;}
        let scene=borrowed.scene;
        for _ in 0..TRACE_POINTER_WORK_PER_STEP {
            let Some(work)=self.work.pop()else{return true};self.completed_work+=1;
            let step=(||->Result<(),Fault>{
                match work {
                    TracePointerWork::SelectionPaths{next}=>{
                        if let Some(node)=scene.plan.nodes.get(next) {
                            if self.selected_ids.contains(&node.id) {self.selected_paths.push(TracePath::from_source(&node.source_path).ok_or_else(||Fault::from("Invalid prepared layer address"))?);}
                            self.push_work(TracePointerWork::SelectionPaths{next:next+1});
                        }else{self.push_work(TracePointerWork::SelectionBounds{next:0});}
                    }
                    TracePointerWork::SelectionBounds{next}=>{
                        if let Some(node)=scene.plan.nodes.get(next) {
                            if node.visible&&node.opacity>0.0&&node.groups.iter().all(|group|group.opacity>0.0)&&crate::schema::scene_preparation::scene_selection_relation(&node.source_path,node.locked_ancestors,&self.selected_paths).map_err(|error|Fault::from(error.to_string()))?.bounds_selection.is_some() {
                                if let Some([x,y,r,b])=scene.geometry[next].bounds.or(scene.geometry[next].geometry_bounds) {let bounds=[x,y,r-x,b-y];self.selection_bounds=Some(self.selection_bounds.map_or(bounds,|old|{let(x,y)=(old[0].min(bounds[0]),old[1].min(bounds[1]));[x,y,(old[0]+old[2]).max(bounds[0]+bounds[2])-x,(old[1]+old[3]).max(bounds[1]+bounds[3])-y]}));}
                            }
                            self.push_work(TracePointerWork::SelectionBounds{next:next+1});
                        }else if self.node_area||self.node_editing||self.include_control_points{self.push_work(TracePointerWork::ControlNodes{next:scene.plan.nodes.len()});}
                        else{self.push_work(TracePointerWork::Pick);}
                    }
                    TracePointerWork::ControlNodes{next}=>{
                        if next==0 {if !self.node_area{self.push_work(TracePointerWork::Pick);}return Ok(());}
                        let index=next-1;self.push_work(TracePointerWork::ControlNodes{next:index});let node=&scene.plan.nodes[index];
                        if !node.visible||node.opacity<=0.0||node.groups.iter().any(|group|group.opacity<=0.0)||node.locked_ancestors!=0{return Ok(());}
                        let path=TracePath::from_source(&node.source_path).ok_or_else(||Fault::from("Invalid prepared layer address"))?;
                        if !matches!(drawing_layer_at_path(&document.layers,&path),Some(DrawingLayerNode::Path(_))){return Ok(());}
                        if self.node_area {if self.selected_ids.contains(&node.id){self.node_hash=Some(points::geometry_hasher());self.node_indices.clear();self.push_work(TracePointerWork::NodeArea{path,next:0,matrix:node.transform});}}
                        else if self.node_editing&&self.selected_ids.contains(&node.id)||self.include_control_points{self.push_work(TracePointerWork::ControlSegments{index,path,next:0,matrix:node.transform});}
                    }
                    TracePointerWork::ControlSegments{index,path,next,matrix}=>{
                        let Some(DrawingLayerNode::Path(layer))=drawing_layer_at_path(&document.layers,&path)else{return Err(Fault::from("Authored control path changed"));};
                        if let Some(segment)=layer.segments.get(next){
                            if let Some((point,distance))=path_point_hit(segment,matrix,self.world,self.tolerance){
                                if self.node_editing&&self.selected_ids.contains(&layer.base.id)&&self.node_hit.as_ref().is_none_or(|(previous,_,_,old)|previous.indices==path.indices&&previous.len==path.len&&distance<*old){self.node_hit=Some((path,next,point,distance));}
                                if self.include_control_points&&self.best.as_ref().is_none_or(|best|best.generality<4){self.best=Some(TracePickCandidate{generality:4,layer_id:scene.plan.nodes[index].id.clone(),image_key:None,path});}
                            }
                            self.push_work(TracePointerWork::ControlSegments{index,path,next:next+1,matrix});
                        }
                    }
                    TracePointerWork::NodeArea{path,next,matrix}=>{
                        let Some(DrawingLayerNode::Path(layer))=drawing_layer_at_path(&document.layers,&path)else{return Err(Fault::from("Authored anchor path changed"));};
                        if let Some(segment)=layer.segments.get(next){
                            points::hash_segment(self.node_hash.as_mut().expect("active node hash"),segment).ok_or_else(||Fault::from("Invalid authored anchor"))?;
                            if self.marquee.is_some_and(|(start,end,_)|points::anchor_in_marquee(segment,matrix,start,end)){if self.node_indices.len()+self.hits.len()>=DRAWING_QUERY_HIT_CAPACITY{return Err(Fault::from("Anchor selection exceeds capacity"));}self.node_indices.push(next);}
                            self.push_work(TracePointerWork::NodeArea{path,next:next+1,matrix});
                        }else{let geometry=self.node_hash.take().expect("finished node hash").finalize().to_hex();self.push_work(TracePointerWork::PublishNodeArea{path,next:0,geometry});}
                    }
                    TracePointerWork::PublishNodeArea{path,next,geometry}=>{
                        if let Some(index)=self.node_indices.get(next){
                            let layer=drawing_layer_at_path(&document.layers,&path).ok_or_else(||Fault::from("Authored anchor path changed"))?;
                            let id=points::point_id(&trace_layer_base(layer).id,&geometry,*index,PathPoint::Anchor).ok_or_else(||Fault::from("Invalid anchor selection"))?;
                            self.node_bytes+=id.len();if self.node_bytes>DRAWING_QUERY_TARGET_BYTES||self.hits.try_push(id).is_err(){return Err(Fault::from("Anchor selection exceeds capacity"));}
                            self.push_work(TracePointerWork::PublishNodeArea{path,next:next+1,geometry});
                        }
                    }
                    TracePointerWork::Pick=>{
                        if self.pick.is_none(){
                            let query=if let Some(polygon)=&self.lasso{PreparedScenePick::Lasso{points:polygon.as_slice().to_vec()}}else if let Some((start,end,crossing))=self.marquee{PreparedScenePick::Rectangle{start,end,crossing}}else{PreparedScenePick::Point{point:self.world,tolerance:self.tolerance,required_flatness:if self.tolerance==0.0{scene.flatness}else{(self.tolerance/80.0).clamp(1e-6,16.0)}}};
                            self.pick=Some(PreparedScenePickJob::new(live,query,DRAWING_QUERY_HIT_CAPACITY).map_err(Fault::from)?);
                            self.push_work(TracePointerWork::Pick);
                        }else if self.pick.as_mut().unwrap().advance(scene,live,1).map_err(Fault::from)?.done{self.push_work(TracePointerWork::PublishPick{next:0});}
                        else{self.push_work(TracePointerWork::Pick);}
                    }
                    TracePointerWork::PublishPick{next}=>{
                        if let Some(index)=self.pick.as_ref().unwrap().result(live).map_err(Fault::from)?.get(next).copied(){
                            let node=&scene.plan.nodes[index];let path=TracePath::from_source(&node.source_path).ok_or_else(||Fault::from("Invalid prepared layer address"))?;
                            if self.marquee.is_some()||self.lasso.is_some(){self.node_bytes+=node.id.len();if self.node_bytes>DRAWING_QUERY_TARGET_BYTES||self.hits.try_push(node.id.clone()).is_err(){return Err(Fault::from("Layer selection exceeds capacity"));}}
                            else if self.best.as_ref().is_none_or(|best|best.generality<4){self.best=Some(TracePickCandidate{generality:2,layer_id:node.id.clone(),image_key:match &node.content{crate::schema::scene_preparation::DocumentSceneContent::Image{asset,..}=>Some(asset.clone()),_=>None},path});}
                            self.push_work(TracePointerWork::PublishPick{next:next+1});
                        }
                    }
                }Ok(())
            })();
            if let Err(error)=step{self.failure=Some(error.message);self.overflowed=true;return true;}
        }
        self.work.is_empty()
    }
}

impl AsRef<[u16]> for TracePath{fn as_ref(&self)->&[u16]{&self.indices[..usize::from(self.len)]}}
impl TracePath{
 fn from_source(source:&[u16])->Option<Self>{if source.is_empty()||source.len()>TRACE_POINTER_MAX_DEPTH{return None;}let mut path=Self{indices:[0;TRACE_POINTER_MAX_DEPTH],len:source.len()as u8};path.indices[..source.len()].copy_from_slice(source);Some(path)}
}

fn drawing_layer_at_path<'a>(roots: &'a [DrawingLayerNode], path: &TracePath) -> Option<&'a DrawingLayerNode> {
    let mut indices = path.indices();
    let mut layer = roots.get(indices.next()?)?;
    for index in indices {
        let DrawingLayerNode::Group(group) = layer else { return None };
        layer = group.children.get(index)?;
    }
    Some(layer)
}

fn trace_path_matrix(roots: &[DrawingLayerNode], path: &TracePath) -> Option<[f64;6]> {
    let mut layers=roots;
    let mut matrix=[1.0,0.0,0.0,1.0,0.0,0.0];
    for index in path.indices() {
        let layer=layers.get(index)?;
        matrix=crate::schema::geometry::multiply(matrix,crate::schema::drawing_transform_to_matrix(&trace_layer_base(layer).transform));
        layers=if let DrawingLayerNode::Group(group)=layer { &group.children } else { &[] };
    }
    Some(matrix)
}

fn trace_layer_base(layer: &DrawingLayerNode) -> &crate::DrawingLayerBase {
    match layer {
        DrawingLayerNode::Shape(value) => &value.base,
        DrawingLayerNode::Path(value) => &value.base,
        DrawingLayerNode::Text(value) => &value.base,
        DrawingLayerNode::Image(value) => &value.base,
        DrawingLayerNode::Group(value) => &value.base,
        DrawingLayerNode::Boolean(value) => &value.base,
        DrawingLayerNode::Trace(value) => &value.base,
    }
}

//#endregion 🧵️TracePointerJob

//#region 🔖️DrawingSession
/// 🕹️ Owned snapshot of the framework's `"strokes"` and `"points"` selections, read once per dispatch and threaded through
/// `DrawingSession` to every command handler — decouples handlers from `semio_framework_plugin::app::InteractionView`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawingInteractionSnapshot {
    pub ids: Vec<String>,
    pub points: Vec<String>,
}

/// 🧪️ `app_commands!` dispatch context — the canvas tool, the preview tick counter, the current selections, the Canvas
/// window's state, the committed base of this dispatch and the retained hit-test and trace jobs.
pub struct DrawingSession {
    /// 🛠️ The canvas tool driving pointer gestures (ephemeral tool state, never history).
    pub(crate) tool: DrawingTool,
    /// 👻️ Monotone preview counter.
    preview_seq: u64,
    /// 🕹️ Current `"strokes"` / `"points"` selection — set before every dispatch.
    pub(crate) interaction: DrawingInteractionSnapshot,
    pub(crate) active_utility_id: String,
    pub(crate) window_config: DrawingCanvasWindowConfig,
    pub(crate) window_transient: DrawingCanvasWindowTransient,
    /// 🧱️ The committed document and admission of the current dispatch, shared without a copy when the retained owner has it.
    pub(crate) base: Option<DrawingToolBase>,
    pub(crate) trace_pointer: Option<TracePointerJob>,
    pub(crate) source_identity:Option<crate::schema::scene_identity::SceneIdentity>,
    pub(crate) point_query: Option<DrawingPointQuery>,
    move_sample_cursor: usize,
    pub(crate) node_marquee: Option<NodeMarquee>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawingGesturePreviewPhase {
    Move,
    Marquee,
    Shape,
    Draft,
    #[default]
    Idle,
}

/// 👁️ What the Canvas window paints of an in-flight gesture — derived from the tool's context only, never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawingGesturePreview {
    pub sequence: u64,
    pub phase: DrawingGesturePreviewPhase,
    pub context: DrawingToolContext,
    pub transformation: Option<(Vec<String>,[f64;6])>,
    pub node_translation: Option<DrawingNodeTranslation>,
}

/// 📍️ The points a node drag moves and its world offset so far.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawingNodeTranslation {
    pub targets:Vec<DrawingPathPointTarget>,
    pub delta:[f64;2],
}

pub(crate) struct NodeMarquee {
    layer_ids:Vec<String>,
    current:Vec<String>,
    mode:points::PointPickMode,
}

/// ✊️ Resolves what a select press grabs, in bounded steps: every selected layer found in document order, a layer inside
/// an already grabbed group dropped (it moves with the group), a locked, hidden or singular ancestry refused.
struct LayerGrabPreparation {
    ids: Vec<String>,
    next: Option<TracePath>,
    found: usize,
    targets: Vec<(TracePath, String)>,
    handle: Option<(usize, [f64; 4])>,
}

fn next_layer_path(document: &DrawingSnapshot, mut path: TracePath) -> Option<TracePath> {
    if matches!(drawing_layer_at_path(&document.layers,&path),Some(DrawingLayerNode::Group(group)) if !group.children.is_empty()) { return path.child(0); }
    loop {
        let depth=usize::from(path.len).checked_sub(1)?;
        let count=if depth==0 { document.layers.len() } else {
            let mut parent=path; parent.len-=1;
            match drawing_layer_at_path(&document.layers,&parent)? { DrawingLayerNode::Group(group)=>group.children.len(),_=>0 }
        };
        if usize::from(path.indices[depth])+1<count {
            path.indices[depth]=path.indices[depth].checked_add(1)?;
            return Some(path);
        }
        path.len-=1;
    }
}

impl LayerGrabPreparation {
    fn new(document: &DrawingSnapshot, ids: Vec<String>, handle: Option<(usize, [f64; 4])>) -> Self {
        Self { ids, next: (!document.layers.is_empty()).then(|| TracePath::root(0)).flatten(), found: 0, targets: Vec::new(), handle }
    }

    fn advance(&mut self, document: &DrawingSnapshot) -> Result<bool,Fault> {
        if self.found==self.ids.len() { return Ok(true); }
        let Some(path)=self.next else {
            if self.found!=self.ids.len() { return Err(Fault::from("A selected layer no longer exists")); }
            return Ok(true);
        };
        let layer=drawing_layer_at_path(&document.layers,&path).ok_or_else(||Fault::from("A selected layer no longer exists"))?;
        if matches!(layer,DrawingLayerNode::Group(group) if !group.children.is_empty()) && usize::from(path.len)>=TRACE_POINTER_MAX_DEPTH { return Err(Fault::from("Selection exceeds the gesture traversal depth")); }
        self.next=next_layer_path(document,path);
        let base=trace_layer_base(layer);
        if !self.ids.contains(&base.id) { return Ok(false); }
        self.found+=1;
        if self.targets.iter().any(|(target,_)|crate::schema::geometry::translation::path_contains(&target.indices[..usize::from(target.len)],&path.indices[..usize::from(path.len)])) { return Ok(false); }
        let mut prefix=path;
        while prefix.len>0 {
            let ancestor=trace_layer_base(drawing_layer_at_path(&document.layers,&prefix).ok_or_else(||Fault::from("Selected ancestry changed"))?);
            if ancestor.locked || !ancestor.visible { return Err(Fault::from("Unlock and show the selected layers before moving")); }
            prefix.len-=1;
        }
        let mut parent_path=path; parent_path.len-=1;
        let parent=trace_path_matrix(&document.layers,&parent_path).ok_or_else(||Fault::from("Selected parent transform is unavailable"))?;
        if crate::schema::geometry::inverse(parent).is_none() { return Err(Fault::from("Cannot move through a singular transform")); }
        self.targets.push((path,base.id.clone()));
        Ok(false)
    }

    fn grab(self) -> DrawingGrab {
        DrawingGrab::Layers { targets: self.targets.into_iter().map(|(_, id)| id).collect(), handle: self.handle }
    }
}

pub(crate) struct DrawingPointQuery {
    pub(crate) command_id: &'static str,
    pub(crate) cursor: TracePointerJob,
    pub(crate) hover: bool,
    pub(crate) merge: String,
    pub(crate) marquee: bool,
    pub(crate) traversal_complete: bool,
    pub(crate) drag_start: Option<[f64;2]>,
    pub(crate) constrained: bool,
    pub(crate) centered: bool,
    grab_preparation: Option<LayerGrabPreparation>,
    grab_prepared: bool,
    pub(crate) preserve_selection: bool,
    pub(crate) node_selection: Option<Vec<String>>,
    area_current:Vec<String>,
    pub(crate) point_pick_mode: points::PointPickMode,
    target_cursor: usize,
    targets: String,
}

pub(crate) enum DrawingQueryPublication {
    Pending,
    Complete(String),
    Fault,
}

impl DrawingPointQuery {
    pub(crate) fn new(command_id: &'static str, cursor: TracePointerJob, hover: bool, merge: String, marquee: bool) -> Self {
        Self { command_id, cursor, hover, merge, marquee, traversal_complete: false, drag_start: None, constrained:false, centered:false, grab_preparation: None, grab_prepared: false, preserve_selection: false, node_selection:None, area_current:Vec::new(), point_pick_mode:points::PointPickMode::Replace, target_cursor: 0, targets: String::with_capacity(DRAWING_QUERY_TARGET_BYTES) }
    }

    pub(crate) fn publication_step(&mut self) -> DrawingQueryPublication {
        if self.targets.is_empty() {
            self.targets.push('[');
        }
        let id = if let Some(selection)=&self.node_selection {
            selection.get(self.target_cursor)
        } else if self.marquee {
            self.cursor.hits.get(self.target_cursor)
        } else if self.target_cursor == 0 {
            self.cursor.best.as_ref().map(|candidate| &candidate.layer_id)
        } else {
            None
        };
        if let Some(id) = id {
            let id = semio_framework_pack_json::to_json_string(id);
            let prefix = if self.target_cursor == 0 { "" } else { "," };
            let granularity=if self.node_selection.is_some() {DRAWING_POINT_GRANULARITY}else {DRAWING_INTERACTION_GRANULARITY};
            let item = format!("{prefix}{{\"granularity\":\"{granularity}\",\"id\":{id}}}");
            if self.targets.len().checked_add(item.len()).is_none_or(|bytes| bytes >= DRAWING_QUERY_TARGET_BYTES) {
                return DrawingQueryPublication::Fault;
            }
            self.targets.push_str(&item);
            self.target_cursor += 1;
            return DrawingQueryPublication::Pending;
        }
        if self.targets.len().checked_add(1).is_none_or(|bytes| bytes > DRAWING_QUERY_TARGET_BYTES) {
            return DrawingQueryPublication::Fault;
        }
        self.targets.push(']');
        DrawingQueryPublication::Complete(std::mem::take(&mut self.targets))
    }
}

/// 📍️ The point ids a committed node drag leaves selected: every dragged point re-bound to its path's moved geometry.
pub(crate) fn drawing_rebound_points(base: &DrawingSnapshot, leaf: &DrawingMutation) -> Result<Vec<String>, Fault> {
    let DrawingMutation::DragPathPoints(drag) = leaf else { return Ok(Vec::new()) };
    let mut moved = base.clone();
    crate::mutations::apply_drawing_mutation(&mut moved, leaf).map_err(|error| Fault::from(error.to_string()))?;
    let mut rebound = Vec::with_capacity(drag.targets.len());
    for target in &drag.targets {
        let Some(DrawingLayerNode::Path(path)) = crate::schema::find_drawing_layer(&moved, &target.layer_id) else { continue };
        let geometry = points::geometry_id(&path.segments).ok_or_else(|| Fault::from("Invalid moved path geometry"))?;
        rebound.push(points::point_id(&target.layer_id, &geometry, target.index, target.point).ok_or_else(|| Fault::from("Invalid selected point"))?);
    }
    Ok(rebound)
}

impl Default for DrawingSession {
    fn default() -> Self {
        Self::new(crate::editor::drawing::DRAWING_DEFAULT_UTILITY, "")
    }
}

impl DrawingSession {
    /// 🚀️ A session at rest for `active_utility_id`, its canvas tool minting from `authoring_seed`.
    pub(crate) fn new(active_utility_id: &str, authoring_seed: &str) -> Self {
        Self {
            tool: DrawingTool::start(active_utility_id, authoring_seed),
            preview_seq: 0,
            interaction: DrawingInteractionSnapshot::default(),
            active_utility_id: active_utility_id.into(),
            window_config: DrawingCanvasWindowConfig::default(),
            window_transient: DrawingCanvasWindowTransient::default(),
            base: None,
            trace_pointer: None,
            source_identity:None,
            point_query: None,
            move_sample_cursor: 0,
            node_marquee: None,
        }
    }

    /// 🧱️ The committed base a tool commit yields against: the retained owner's shared document, else a copy of `doc`.
    pub(crate) fn tool_base(&self, doc: &ArtifactView<'_, DrawingSnapshot>) -> DrawingToolBase {
        self.base.clone().unwrap_or_else(|| DrawingToolBase { document: Arc::new(doc.snapshot.clone()), operation: doc.operation_optional().cloned() })
    }

    fn threshold(&self) -> f64 {
        DRAWING_MARQUEE_THRESHOLD_PX / self.window_config.viewport.zoom.max(1e-6)
    }

    fn tolerance(&self) -> f64 {
        DRAWING_PICK_TOLERANCE_PX / self.window_config.viewport.zoom.max(1e-6)
    }

    fn step(&mut self, event: canvas_tool::Event) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        let step = self.tool.send(event);
        self.preview_seq = self.preview_seq.wrapping_add(1);
        drawing_tool_emit(step)
    }

    /// 🖱️ One press on the canvas.
    pub(crate) fn press(&mut self, pointer: DrawingPointer) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        self.step(canvas_tool::Event::PointerDown(pointer))
    }

    /// ↔️ One pointer sample with the live transform modifiers (shift constrains, alt centres).
    pub(crate) fn sample(&mut self, world: [f64; 2], constrained: bool, centered: bool) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        if !world.iter().all(|value| value.is_finite()) {
            return Ok(Emit::default());
        }
        let threshold = self.threshold();
        self.step(canvas_tool::Event::PointerMove(DrawingPointerMove { world, threshold, constrained, centered }))
    }

    /// 🚪️ Escape: a drag aborts with zero trace, a marquee, shape or draft is dropped, a node marquee forgotten.
    pub(crate) fn escape(&mut self) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        self.node_marquee = None;
        self.step(canvas_tool::Event::Escape)
    }

    /// 🚫️ A cancelled release (pointer left the canvas, capture lost): the live gesture aborts with zero trace and nothing
    /// is selected, picked or committed; a click-sequenced draft is left untouched.
    pub(crate) fn cancel(&mut self) -> Emit<DrawingMutation, NoConfigMutation> {
        if !self.tool.matches("drafting") {
            self.node_marquee = None;
            let _ = self.tool.abort(ToolAbortReason::CaptureLost);
            self.preview_seq = self.preview_seq.wrapping_add(1);
        }
        Emit::default()
    }

    /// ✅️ Commits the open pen or polygon draft as one transaction.
    pub(crate) fn finish_draft(&mut self, base: DrawingToolBase) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        self.step(canvas_tool::Event::CommitDraft(base))
    }

    /// 🖼️ Commits a finished trace as one transaction.
    fn traced(&mut self, base: DrawingToolBase, source_key: Option<String>) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        self.step(canvas_tool::Event::Traced(DrawingTraced { base, source_key }))
    }

    /// ⬆️ One release. A marquee's or a resting select utility's release is interaction: it arms the bounded selection query
    /// (`None` until it publishes). Every other release goes to the tool, which commits a drag or a shape as ONE transaction;
    /// a committed node drag re-binds the point selection to the moved geometry.
    pub(crate) fn release(&mut self, command_id: &'static str, pointer: DrawingPointer, base: DrawingToolBase) -> Result<Option<Emit<DrawingMutation, NoConfigMutation>>, Fault> {
        let (threshold, tolerance) = (self.threshold(), self.tolerance());
        let document = base.document.clone();
        if self.tool.matches("marqueeing") {
            let context = self.tool.context().clone();
            self.step(canvas_tool::Event::PointerUp(DrawingRelease { pointer: pointer.clone(), threshold, base }))?;
            let end = pointer.world;
            if let Some(selection) = self.node_marquee.take() {
                if !context.active {
                    let ids = points::merge_point_selection(&selection.current, &[], selection.mode);
                    let mut emit = Emit::default();
                    emit.effects.push(point_selection_effect(&ids));
                    return Ok(Some(emit));
                }
                let mut cursor = TracePointerJob::new_marquee(&document, context.start, end, false);
                cursor.node_editing = true;
                cursor.node_area = true;
                cursor.selected_ids = selection.layer_ids;
                let mut query = DrawingPointQuery::new(command_id, cursor, false, "replace".into(), true);
                query.preserve_selection = true;
                query.area_current = selection.current;
                query.point_pick_mode = selection.mode;
                self.point_query = Some(query);
                return Ok(None);
            }
            self.point_query = Some(if context.active {
                let query = if context.method == "lasso" { TracePointerJob::new_lasso(&document, LassoPolygon::from_points(&context.points, end)) } else { TracePointerJob::new_marquee(&document, context.start, end, end[0] < context.start[0]) };
                DrawingPointQuery::new(command_id, query, false, context.merge.clone(), true)
            } else {
                DrawingPointQuery::new(command_id, TracePointerJob::new_query(&document, end, tolerance, false), false, selection_merge_mode(pointer.shift, pointer.ctrl, pointer.meta).into(), false)
            });
            return Ok(None);
        }
        if self.tool.at_rest() && matches!(pointer.utility.as_str(), "selectDirect" | "editNodes") {
            self.point_query = Some(DrawingPointQuery::new(command_id, TracePointerJob::new_query(&document, pointer.world, tolerance, true), false, selection_merge_mode(pointer.shift, pointer.ctrl, pointer.meta).into(), false));
            return Ok(None);
        }
        let mut emit = self.step(canvas_tool::Event::PointerUp(DrawingRelease { pointer, threshold, base }))?;
        if let Some(leaf @ DrawingMutation::DragPathPoints(_)) = emit.artifact_mutations.first() {
            let rebound = drawing_rebound_points(&document, leaf)?;
            emit.effects.push(point_selection_effect(&rebound));
        }
        Ok(Some(emit))
    }

    /// ✊️ Resolves what the pending select press grabbed, in bounded steps, and hands it to the tool: path points under
    /// `editNodes`, else the hit layer or the whole selection (a hit transform handle keeps the selection). An empty node
    /// press starts a node marquee instead. `Ok(false)` while more document remains to walk.
    pub(crate) fn prepare_grab(&mut self, document: &DrawingSnapshot, ids: &[String], point_ids:&[String]) -> Result<bool,Fault> {
        let Some(query)=self.point_query.as_mut() else { return Ok(true); };
        if query.cursor.node_area && !query.grab_prepared {
            let hits=query.cursor.hits.iter().cloned().collect::<Vec<_>>();
            let selected=points::merge_point_selection(&query.area_current,&hits,query.point_pick_mode);
            if selected.len()>DRAWING_QUERY_HIT_CAPACITY || selected.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Point selection exceeds gesture capacity"));}
            query.node_selection=Some(selected);query.grab_prepared=true;return Ok(true);
        }
        if query.grab_prepared || query.drag_start.is_none() { return Ok(true); }
        if query.cursor.node_editing {
            query.grab_prepared=true;
            query.preserve_selection=true;
            if point_ids.len()>DRAWING_QUERY_HIT_CAPACITY || point_ids.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Point selection exceeds gesture capacity"));}
            let current=point_ids.iter().filter(|id|points::parse_point_id(id).is_some_and(|point|ids.iter().any(|id|id==point.layer_id))).cloned().collect::<Vec<_>>();
            let hit=if let Some((path,index,point,_))=query.cursor.node_hit {
                let Some(DrawingLayerNode::Path(layer))=drawing_layer_at_path(&document.layers,&path) else {return Err(Fault::from("Missing selected path"));};
                let geometry=points::geometry_id(&layer.segments).ok_or_else(||Fault::from("Invalid path geometry"))?;
                Some(points::point_id(&layer.base.id,&geometry,index,point).ok_or_else(||Fault::from("Invalid selected point"))?)
            } else {None};
            let selected=points::pick_point_selection(&current,hit.as_deref(),query.point_pick_mode);
            if selected.len()>DRAWING_QUERY_HIT_CAPACITY || selected.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Point selection exceeds gesture capacity"));}
            let start=query.drag_start.expect("a grab preparation starts at its press");
            if hit.is_none() {
                self.node_marquee=Some(NodeMarquee {layer_ids:ids.to_vec(),current,mode:query.point_pick_mode});
                self.step(canvas_tool::Event::NodeMarquee(DrawingPointer { utility:"selectMarquee".into(),world:start,shift:false,alt:false,ctrl:false,meta:false }))?;
                return Ok(true);
            }
            query.node_selection=Some(selected.clone());
            if hit.as_ref().is_some_and(|hit|selected.contains(hit)) {
                let targets=crate::editor::drawing::commands::nudge_selection::drawing_point_targets(document,ids,&selected)?;
                self.step(canvas_tool::Event::Grab(DrawingGrab::Points { targets }))?;
            }
            return Ok(true);
        }
        if query.grab_preparation.is_none() {
            if ids.len()>DRAWING_QUERY_HIT_CAPACITY || ids.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Selection exceeds gesture capacity"));}
            let start=query.drag_start.expect("a grab preparation starts at its press");
            let handle=query.cursor.selection_bounds.and_then(|bounds|crate::schema::geometry::handles::hit_handle(bounds,start,self.window_config.viewport.zoom).map(|handle|(handle,bounds)));
            let mut selected=if handle.is_some() {
                query.preserve_selection=true;
                ids.to_vec()
            } else {
                if query.constrained {query.grab_prepared=true;query.preserve_selection=true;return Ok(true);}
                let Some(candidate)=query.cursor.best.as_ref() else {query.grab_prepared=true;return Ok(true);};
                let mut prefix=candidate.path;
                while prefix.len>0 {
                    if drawing_layer_at_path(&document.layers,&prefix).is_some_and(|layer|ids.contains(&trace_layer_base(layer).id)) {query.preserve_selection=true;break;}
                    prefix.len-=1;
                }
                if query.preserve_selection {ids.to_vec()} else {vec![candidate.layer_id.clone()]}
            };
            selected.sort();selected.dedup();
            query.grab_preparation=Some(LayerGrabPreparation::new(document,selected,handle));
            return Ok(false);
        }
        let preparation=query.grab_preparation.as_mut().expect("a started grab preparation remains retained");
        if !preparation.advance(document)? { return Ok(false); }
        let grab=query.grab_preparation.take().expect("a finished grab preparation remains retained").grab();
        query.grab_prepared=true;
        self.step(canvas_tool::Event::Grab(grab))?;
        Ok(true)
    }

    pub(crate) fn advance_lasso_move(&mut self, payload: &crate::editor::drawing::commands::canvas_pointer_move::CanvasPointerMove) -> Result<Option<Emit<DrawingMutation,NoConfigMutation>>,Fault> {
        let [x,y]=payload.samples.get(self.move_sample_cursor).copied().unwrap_or([payload.x,payload.y]);
        let (x,y)=canvas_point_to_world(&self.window_config.viewport,x,y,payload.width,payload.height);
        let emit=self.sample([x,y],payload.shift,payload.alt)?;
        self.move_sample_cursor+=1;
        if self.move_sample_cursor<payload.samples.len() { return Ok(None); }
        self.move_sample_cursor=0;
        Ok(Some(emit))
    }

    fn retain_trace_pointer(&mut self, job: TracePointerJob) -> Result<(), TracePointerJob> {
        if self.trace_pointer.is_some() {
            return Err(job);
        }
        self.trace_pointer = Some(job);
        Ok(())
    }

    fn take_trace_pointer(&mut self, app_instance_id: u32, document_id: &str, operation_id: u64, generation: u64, base_revision: &str) -> Option<TracePointerJob> {
        let job = self.trace_pointer.as_ref()?;
        if job.app_instance_id != app_instance_id || job.document_id != document_id || job.operation_id != operation_id || job.generation != generation || job.base_revision != base_revision {
            return None;
        }
        self.trace_pointer.take()
    }

    pub(crate) fn cancel_trace_pointer(&mut self, app_instance_id: u32, document_id: &str, generation: u64) -> bool {
        let matches = self.trace_pointer.as_ref().is_some_and(|job| job.app_instance_id == app_instance_id && job.document_id == document_id && generation != 0 && job.generation == generation);
        if matches {
            self.trace_pointer = None;
        }
        matches
    }

    /// 👁️ The in-flight gesture as the Canvas window paints it, derived from the tool's context only.
    pub(crate) fn preview(&self) -> DrawingGesturePreview {
        let phase = if self.tool.matches("pressing") || self.tool.matches("dragging") {
            DrawingGesturePreviewPhase::Move
        } else if self.tool.matches("marqueeing") {
            DrawingGesturePreviewPhase::Marquee
        } else if self.tool.matches("shaping") {
            DrawingGesturePreviewPhase::Shape
        } else if self.tool.matches("drafting") {
            DrawingGesturePreviewPhase::Draft
        } else {
            DrawingGesturePreviewPhase::Idle
        };
        let context = self.tool.context();
        let dragging = self.tool.matches("dragging") && context.active;
        let (transformation, node_translation) = match context.grab.as_ref().filter(|_| dragging) {
            Some(DrawingGrab::Layers { targets, handle }) => {
                let matrix = match handle {
                    None => Some([1.0, 0.0, 0.0, 1.0, context.cursor[0] - context.start[0], context.cursor[1] - context.start[1]]),
                    Some((handle, bounds)) => handle_motion(*handle, *bounds, context.start, context.cursor, context.constrained, context.centered).map(HandleMotion::matrix),
                };
                (matrix.map(|matrix| (targets.clone(), matrix)), None)
            }
            Some(grab @ DrawingGrab::Points { targets }) => match drawing_grab_leaf(grab, context.start, context.cursor, context.constrained, context.centered) {
                Some(DrawingMutation::DragPathPoints(drag)) => (None, Some(DrawingNodeTranslation { targets: targets.clone(), delta: [drag.dx, drag.dy] })),
                _ => (None, None),
            },
            None => (None, None),
        };
        DrawingGesturePreview { sequence: self.preview_seq, phase, context: context.clone(), transformation, node_translation }
    }
}
//#endregion 🔖️DrawingSession

use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

//#region 🧵️TracePointerContinuation
fn retain_trace_progress(session: &mut DrawingSession, job: &TracePointerJob) {
    session.window_transient.trace_pointer_generation = job.generation;
    session.window_transient.trace_pointer_completed_work = job.completed_work as u64;
    session.window_transient.trace_pointer_pending_work = job.work.len() as u64;
}

fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> Effect {
    let continuation = CanvasPointerDown {
        app_instance_id: Some(job.app_instance_id),
        parent_document_id: Some(job.document_id.clone()),
        operation_id: Some(job.operation_id),
        generation: Some(job.generation),
        base_revision: Some(job.base_revision.clone()),
        world_x: Some(job.world[0]),
        world_y: Some(job.world[1]),
        checkpoint_completed_work: Some(job.completed_work as u64),
        checkpoint_pending_work: Some(job.work.len() as u64),
        ..payload.clone()
    };
    let args = Some(semio_framework_value::ToValue::to_value(&continuation));
    Effect::DispatchAction { req: RequestId(NEXT_TRACE_POINTER_REQUEST.fetch_add(1, Ordering::Relaxed)), action: "canvasPointerDown".into(), args, delay_ms: 0 }
}

/// 🖼️ One bounded step of the trace pointer job: requeued while the document walk continues, then the canvas tool commits
/// the trace layer of the image under the pointer as one transaction.
fn advance_trace_pointer(session: &mut DrawingSession, mut job: TracePointerJob, payload: &CanvasPointerDown, base: DrawingToolBase) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let captured=session.source_identity.ok_or_else(||Fault::from("Drawing pointer has no source authority"))?;
    crate::editor::drawing::geometry_session::prepare_query(captured,&base.document);
    let complete=crate::editor::drawing::geometry_session::with_query(captured,|status,borrowed|match status{
        crate::schema::scene_identity::admission::SceneAdmissionStatus::Pending=>Ok(false),
        crate::schema::scene_identity::admission::SceneAdmissionStatus::Ready=>{let complete=job.advance(&base.document,&borrowed.unwrap());if job.overflowed{Err(Fault::from(job.failure.clone().unwrap_or_else(||"Drawing pointer exceeds capacity".into())))}else{Ok(complete)}},
        _=>Err(Fault::from("Drawing pointer geometry is unavailable or changed")),
    })?;
    if !complete {
        let effect = queue_trace_pointer(payload, &job);
        retain_trace_progress(session, &job);
        let _ = session.retain_trace_pointer(job);
        return Ok(Emit { effects: vec![effect], ..Default::default() });
    }
    let source_key = job.best.and_then(|candidate| candidate.image_key).or_else(|| base.document.assets.keys().next().cloned());
    session.window_transient.trace_pointer_generation = 0;
    session.window_transient.trace_pointer_completed_work = 0;
    session.window_transient.trace_pointer_pending_work = 0;
    session.traced(base, source_key)
}
//#endregion 🧵️TracePointerContinuation

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
    let document_revision = crate::editor::drawing::drawing_document_revision(doc);
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
