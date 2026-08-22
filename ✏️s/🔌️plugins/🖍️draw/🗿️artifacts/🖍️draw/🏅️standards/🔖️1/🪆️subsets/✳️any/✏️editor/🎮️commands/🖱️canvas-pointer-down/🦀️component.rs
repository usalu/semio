//! 🖱️ 🖱️ Draw play app commands command — `canvas-pointer-down`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::{create_draw_path_layer, create_draw_trace_layer, draw_layer_world_bounds, draw_transform_to_matrix, find_draw_layer, flatten_draw_layers, layer_base, layer_id, layer_to_path_segments};
use crate::artifacts::draw::{DrawCamera, DrawLayerNode, DrawSnapshot, PathSegment};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::editor::draw::{DRAW_INTERACTION_DOMAIN, DRAW_INTERACTION_GRANULARITY};
use semio_framework_plugin::{kernel::Effect, ArtifactView, ConfigView, Emit, Fault, RequestId};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

//#region 🔖️GestureContext
/// 🎛️ Per-gesture scratch geometry threaded through the shared `fsm` statechart below — one flat
/// struct (XState convention: context is machine-global, never per-state) mirroring the fields the
/// old hand-rolled `DrawDragState` enum kept per-variant.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GestureContext {
    method: String,
    merge: String,
    pub(crate) utility: String,
    pub(crate) start: [f64; 2],
    pub(crate) cursor: [f64; 2],
    pub(crate) points: Vec<[f64; 2]>,
    active: bool,
}

/// 🎇️ Document-touching side effects the gesture machine requests but never executes — `fsm`'s
/// guards/actions only ever see `(&Context, Option<&Event>)`, never the `DrawSnapshot` tree, so every
/// hit-test/commit that needs the document is deferred to `DrawSession::step_gesture` as an effect.
#[derive(Clone, Debug)]
pub enum GestureEffect {
    CommitMarquee { start: [f64; 2], end: [f64; 2], active: bool, merge: String, shift: bool, ctrl: bool, meta: bool },
    CommitShape { utility: String, start: [f64; 2], end: [f64; 2] },
    CommitDraft { utility: String, points: Vec<[f64; 2]> },
    CommitTrace { world: [f64; 2] },
    PickPoint { world: [f64; 2], shift: bool, ctrl: bool, meta: bool },
}

fn gesture_context_from_input(_input: ()) -> GestureContext {
    GestureContext::default()
}
//#endregion 🔖️GestureContext

//#region 🔖️DocumentHelpers
pub(crate) fn canvas_point_to_world(camera: &DrawCamera, x: f64, y: f64, viewport_w: f64, viewport_h: f64) -> (f64, f64) {
    let zoom = camera.zoom.max(0.01);
    ((x - viewport_w * 0.5) / zoom + camera.x, (y - viewport_h * 0.5) / zoom + camera.y)
}

fn matrix_transform_point(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

/// 🎯️ Maps shift/ctrl/meta modifiers to a framework `MergeMode` wire string (matches
/// `@semio-tech/ui-react`'s `marqueeModeFromModifiers`) — the actual set algebra now runs inside
/// the framework's `next_selection` machine (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM),
/// not here; this crate only ever computes WHICH ids were hit and asks the framework to apply them.
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

/// 🕹️ JSON-encodes `ids` as the `Vec<InteractionTarget>` string the framework's `interactionSelect`/
/// `interactionHover` actions require in their `targets` arg — every hit id shares the domain's one
/// granularity.
fn interaction_targets_json(ids: &[String]) -> String {
    serde_json::to_string(&ids.iter().map(|id| serde_json::json!({ "granularity": DRAW_INTERACTION_GRANULARITY, "id": id })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into())
}

/// 🕹️ Requests the shell to redispatch a framework-owned interaction verb (`interactionSelect`/
/// `interactionHover`) through its normal action funnel — the only way an `ArtifactApp::handle`
/// (or its gesture machine) can drive selection/hover now that both are framework-owned state,
/// never a `DrawConfigMutation` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub(crate) fn request_interaction_action(action_id: &str, args: serde_json::Value) -> Effect {
    Effect::ReplayShellCommand { action_id: action_id.into(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

pub(crate) fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    request_interaction_action(semio_framework::INTERACTION_SELECT_ACTION_ID, serde_json::json!({ "domainId": DRAW_INTERACTION_DOMAIN, "targets": interaction_targets_json(ids), "merge": merge, "method": "pick" }))
}

pub(crate) fn interaction_hover_effect(ids: &[String]) -> Effect {
    request_interaction_action(semio_framework::INTERACTION_HOVER_ACTION_ID, serde_json::json!({ "domainId": DRAW_INTERACTION_DOMAIN, "channel": "pointer", "targets": interaction_targets_json(ids) }))
}

pub(crate) const DRAW_MARQUEE_THRESHOLD_PX: f64 = 4.0;
pub(crate) const DRAW_PICK_TOLERANCE_PX: f64 = 8.0;

pub(crate) struct DrawPickTarget {
    generality: i32,
    layer_id: String,
}

fn draw_pick_generality(layer: &DrawLayerNode) -> i32 {
    match layer {
        DrawLayerNode::Group(_) => 0,
        DrawLayerNode::Boolean(_) | DrawLayerNode::Trace(_) => 1,
        _ => 2,
    }
}

fn ancestor_group_ids(doc: &DrawSnapshot, target_id: &str) -> Vec<String> {
    fn walk(layers: &[DrawLayerNode], target_id: &str, ancestors: &mut Vec<String>) -> bool {
        for layer in layers {
            if layer_id(layer) == target_id {
                return true;
            }
            if let DrawLayerNode::Group(group) = layer {
                ancestors.push(group.base.id.clone());
                if walk(&group.children, target_id, ancestors) {
                    return true;
                }
                ancestors.pop();
            }
        }
        false
    }
    let mut ancestors = Vec::new();
    walk(&doc.layers, target_id, &mut ancestors);
    ancestors
}

fn segment_control_points(segment: &PathSegment) -> Vec<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } => vec![*to],
        PathSegment::Quad { ctrl, to } => vec![*ctrl, *to],
        PathSegment::Cubic { ctrl1, ctrl2, to } => vec![*ctrl1, *ctrl2, *to],
        PathSegment::Arc { to, .. } => vec![*to],
        PathSegment::Close => Vec::new(),
    }
}

/// 🎯️ All pick targets under a world point (groups win by default, control points win over everything when enabled).
pub(crate) fn resolve_pick_targets_at(doc: &DrawSnapshot, world: [f64; 2], tolerance_world: f64, include_control_points: bool) -> Vec<DrawPickTarget> {
    let mut hits = Vec::new();
    for layer in flatten_draw_layers(&doc.layers).into_iter().rev() {
        let base = layer_base(layer);
        if !base.visible || base.locked {
            continue;
        }
        let Some((x, y, width, height)) = draw_layer_world_bounds(layer) else { continue };
        if !(world[0] >= x && world[0] <= x + width && world[1] >= y && world[1] <= y + height) {
            continue;
        }
        let id = layer_id(layer).to_string();
        hits.push(DrawPickTarget { generality: draw_pick_generality(layer), layer_id: id.clone() });
        if matches!(layer, DrawLayerNode::Group(_)) {
            continue;
        }
        for group_id in ancestor_group_ids(doc, &id) {
            if !hits.iter().any(|target| target.layer_id == group_id) {
                hits.push(DrawPickTarget { generality: 0, layer_id: group_id });
            }
        }
        if include_control_points && matches!(layer, DrawLayerNode::Path(_) | DrawLayerNode::Shape(_)) {
            let matrix = draw_transform_to_matrix(&base.transform);
            for segment in layer_to_path_segments(layer) {
                for local in segment_control_points(&segment) {
                    let world_point = matrix_transform_point(matrix, local);
                    let dx = world[0] - world_point[0];
                    let dy = world[1] - world_point[1];
                    if (dx * dx + dy * dy).sqrt() <= tolerance_world {
                        hits.push(DrawPickTarget { generality: 4, layer_id: id.clone() });
                    }
                }
            }
        }
    }
    hits
}

pub(crate) fn best_pick_layer_id(targets: &[DrawPickTarget]) -> Option<String> {
    targets.iter().max_by_key(|target| target.generality).map(|target| target.layer_id.clone())
}

/// 🎯️ Resolves the single best pick target under `world`, camera-zoom-scaled tolerance — a pure
/// query now (selection itself is framework-owned; the caller turns the result into an
/// `interactionSelect` request).
pub(crate) fn resolve_point_pick(doc: &DrawSnapshot, camera: &DrawCamera, world: [f64; 2], include_control_points: bool) -> Option<String> {
    let tolerance = DRAW_PICK_TOLERANCE_PX / camera.zoom.max(1e-6);
    best_pick_layer_id(&resolve_pick_targets_at(doc, world, tolerance, include_control_points))
}

/// ⬚️ Marquee/lasso layer hits — reduces the lasso gesture to its bounding box, matching the premigration behaviour.
pub(crate) fn marquee_layer_hits(doc: &DrawSnapshot, start: [f64; 2], end: [f64; 2], crossing: bool) -> Vec<String> {
    let rect_x = start[0].min(end[0]);
    let rect_y = start[1].min(end[1]);
    let rect_w = (end[0] - start[0]).abs();
    let rect_h = (end[1] - start[1]).abs();
    let mut hits = Vec::new();
    for layer in flatten_draw_layers(&doc.layers) {
        let base = layer_base(layer);
        if !base.visible || matches!(layer, DrawLayerNode::Group(_)) {
            continue;
        }
        let Some((x, y, width, height)) = draw_layer_world_bounds(layer) else { continue };
        let intersects = rect_x <= x + width && rect_x + rect_w >= x && rect_y <= y + height && rect_y + rect_h >= y;
        let contains = x >= rect_x && y >= rect_y && x + width <= rect_x + rect_w && y + height <= rect_y + rect_h;
        if if crossing { intersects } else { contains } {
            hits.push(layer_id(layer).to_string());
        }
    }
    hits
}

pub(crate) fn shape_preview_segments(utility: &str, start: [f64; 2], end: [f64; 2]) -> Vec<PathSegment> {
    if utility == "shapeLine" {
        return vec![PathSegment::Move { to: start }, PathSegment::Line { to: end }];
    }
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if utility == "shapeRect" {
        return vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close];
    }
    let cx = x + width / 2.0;
    let cy = y + height / 2.0;
    let rx = width / 2.0;
    let ry = height / 2.0;
    let k = 0.552_284_749_8;
    vec![
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + rx * k, cy - ry], ctrl2: [cx + rx, cy - ry * k], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + ry * k], ctrl2: [cx + rx * k, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - rx * k, cy + ry], ctrl2: [cx - rx, cy + ry * k], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - ry * k], ctrl2: [cx - rx * k, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ]
}

pub(crate) fn draft_preview_segments(utility: &str, points: &[[f64; 2]], cursor: [f64; 2]) -> Vec<PathSegment> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut segments = vec![PathSegment::Move { to: points[0] }];
    for point in points.iter().skip(1) {
        segments.push(PathSegment::Line { to: *point });
    }
    segments.push(PathSegment::Line { to: cursor });
    if utility == "shapePolygon" && points.len() > 1 {
        segments.push(PathSegment::Close);
    }
    segments
}

/// 🔷️ Emits the operations that commit a shape drag (add the shape layer + return to direct-select);
/// empty when the drag is too small to commit.
fn commit_shape_drag(doc: &DrawSnapshot, utility: &str, start: [f64; 2], end: [f64; 2]) -> Vec<DrawMutation> {
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if width < 1.0 && height < 1.0 {
        return Vec::new();
    }
    let layer = DrawLayerNode::Shape(crate::artifacts::draw::DrawShapeBody {
        base: crate::artifacts::draw::schema::default_layer_base(match utility {
            "shapeLine" => "Line",
            "shapeEllipse" => "Ellipse",
            _ => "Rectangle",
        }),
        shape_kind: match utility {
            "shapeLine" => "line",
            "shapeEllipse" => "ellipse",
            _ => "rect",
        }
        .into(),
        rect: if utility == "shapeRect" { Some(crate::artifacts::draw::DrawRect { x, y, width, height }) } else { None },
        ellipse: if utility == "shapeEllipse" { Some(crate::artifacts::draw::DrawEllipse { cx: x + width / 2.0, cy: y + height / 2.0, rx: width / 2.0, ry: height / 2.0 }) } else { None },
        circle: None,
        line: if utility == "shapeLine" { Some(crate::artifacts::draw::DrawLine { x1: start[0], y1: start[1], x2: end[0], y2: end[1] }) } else { None },
        polygon: None,
    });
    vec![crate::artifacts::draw::mutations::create_layer(None, Some(doc.layers.len()), layer)]
}

/// ✒️ Emits the operations that commit a freehand/polygon draft into a path or polygon layer; empty
/// when the draft has too few points to form a shape.
fn commit_draft(doc: &DrawSnapshot, utility: &str, points: &[[f64; 2]]) -> Vec<DrawMutation> {
    if points.len() < 2 {
        return Vec::new();
    }
    let layer = if utility == "pen" {
        let mut segments = vec![PathSegment::Move { to: points[0] }];
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
        create_draw_path_layer("Path", segments)
    } else {
        DrawLayerNode::Shape(crate::artifacts::draw::DrawShapeBody {
            base: crate::artifacts::draw::schema::default_layer_base("Polygon"),
            shape_kind: "polygon".into(),
            rect: None,
            ellipse: None,
            circle: None,
            line: None,
            polygon: Some(crate::artifacts::draw::DrawPolygon { points: points.to_vec() }),
        })
    };
    vec![crate::artifacts::draw::mutations::create_layer(None, Some(doc.layers.len()), layer)]
}

/// 🖍️ Emits the operations that add a trace layer over the picked image (or first asset); empty
/// when no bitmap source is available.
fn commit_trace_at(doc: &DrawSnapshot, camera: &DrawCamera, world: [f64; 2]) -> Vec<DrawMutation> {
    let hit_layer_id = resolve_point_pick(doc, camera, world, false);
    let source_key = match hit_layer_id {
        Some(id) => find_draw_layer(doc, &id).cloned(),
        None => None,
    }
    .and_then(|layer| match layer {
        DrawLayerNode::Image(image) => Some(image.image_key),
        _ => None,
    })
    .or_else(|| doc.assets.keys().next().cloned());
    commit_trace_source(doc, source_key)
}

fn commit_trace_source(doc: &DrawSnapshot, source_key: Option<String>) -> Vec<DrawMutation> {
    let Some(source_key) = source_key else { return Vec::new() };
    let layer = create_draw_trace_layer("Trace", &source_key);
    vec![crate::artifacts::draw::mutations::create_layer(None, Some(doc.layers.len()), layer)]
}

/// 🧰️ Wraps a committed gesture's `operations` as a single described edit plus the host effect that returns
/// the canvas to the default select utility (the active utility is host-owned, never a document operation).
fn commit_with_utility_reset(operations: Vec<DrawMutation>, description: &str) -> Emit<DrawMutation, DrawConfigMutation> {
    if operations.is_empty() {
        return Emit::default();
    }
    let mut emit = Emit::commit(operations, description);
    emit.effects.push(Effect::SetActiveUtility { window_id: crate::editor::draw::DRAW_PLAY_WINDOW_CANVAS.into(), utility_id: crate::editor::draw::DRAW_DEFAULT_UTILITY.into() });
    emit
}

//#endregion 🔖️DocumentHelpers

//#region 🔖️GestureGuards
fn utility_is_marquee(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if utility == "selectMarquee" || utility == "selectLasso")
}

fn utility_is_shape(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if matches!(utility.as_str(), "shapeRect" | "shapeEllipse" | "shapeLine"))
}

fn utility_is_draft(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if utility == "pen" || utility == "shapePolygon")
}

fn utility_is_trace(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if utility == "trace")
}

/// 🖊️ Drafting self-loop: the same pen/polygon utility is still active, so the pointer-down appends a point.
fn utility_is_draft_same(ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if (utility == "pen" || utility == "shapePolygon") && utility == &ctx.utility)
}

/// 🖊️ Drafting restart: a different pen/polygon utility switched in without going through `UtilityChanged` first.
fn utility_is_draft_different(ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if (utility == "pen" || utility == "shapePolygon") && utility != &ctx.utility)
}

fn utility_is_select_direct(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerUp { utility, .. }) if utility == "selectDirect")
}
//#endregion 🔖️GestureGuards

//#region 🔖️GestureActions
fn gesture_start_marquee(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, shift, ctrl, meta }) = event {
        ctx.method = if utility == "selectLasso" { "lasso".into() } else { "rectangle".into() };
        ctx.start = *world;
        ctx.cursor = *world;
        ctx.merge = selection_merge_mode(*shift, *ctrl, *meta).into();
        ctx.active = false;
    }
}

fn gesture_start_shape(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, .. }) = event {
        ctx.utility = utility.clone();
        ctx.start = *world;
        ctx.cursor = *world;
    }
}

fn gesture_start_draft(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, .. }) = event {
        ctx.utility = utility.clone();
        ctx.points = vec![*world];
        ctx.cursor = *world;
    }
}

fn gesture_append_draft_point(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerDown { world, .. }) = event {
        if ctx.points.len() < MAX_GESTURE_POINTS {
            ctx.points.push(*world);
        }
        ctx.cursor = *world;
    }
}

fn gesture_update_marquee_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerMove { world, marquee_threshold_world }) = event {
        let distance = ((world[0] - ctx.start[0]).powi(2) + (world[1] - ctx.start[1]).powi(2)).sqrt();
        ctx.active = ctx.active || distance >= *marquee_threshold_world;
        ctx.cursor = *world;
    }
}

fn gesture_update_shape_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerMove { world, .. }) = event {
        ctx.cursor = *world;
    }
}

fn gesture_update_draft_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerMove { world, .. }) = event {
        ctx.cursor = *world;
    }
}

fn gesture_commit_marquee(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerUp { world, shift, ctrl, meta, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitMarquee { start: ctx.start, end: *world, active: ctx.active, merge: ctx.merge.clone(), shift: *shift, ctrl: *ctrl, meta: *meta }));
    }
}

fn gesture_commit_shape(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerUp { world, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitShape { utility: ctx.utility.clone(), start: ctx.start, end: *world }));
    }
}

fn gesture_commit_draft(ctx: &mut GestureContext, _event: Option<&draw_gesture::Event>, sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    sink.push(fsm::Command::Effect(GestureEffect::CommitDraft { utility: ctx.utility.clone(), points: ctx.points.clone() }));
}

fn gesture_commit_trace(_ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerDown { world, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitTrace { world: *world }));
    }
}

fn gesture_pick_point(_ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerUp { world, shift, ctrl, meta, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::PickPoint { world: *world, shift: *shift, ctrl: *ctrl, meta: *meta }));
    }
}
//#endregion 🔖️GestureActions

//#region 🔖️GestureStatechart
// 🎭️ Pointer-gesture control flow — states/events/guards straight off the old hand-rolled
// `DrawDragState` match arms, compiled by `fsm`'s `statechart!` DSL into dense static tables.
// (plain comment, not a doc comment: rustdoc cannot document a macro invocation, and the resulting
// `unused_doc_comments` warning is a hard error under this crate's `-D warnings` gate. The
// `unexpected_cfgs` warning `fsm::statechart!` triggers here is silenced crate-wide in `📦️glue.rs` —
// an `#[allow]` on the macro invocation itself is ignored by rustc, see its own `unused_attributes`
// warning if you try.)
fsm::statechart! {
    machine draw_gesture {
        context: GestureContext;
        event Event {
            PointerDown { utility: String, world: [f64; 2], shift: bool, ctrl: bool, meta: bool },
            PointerMove { world: [f64; 2], marquee_threshold_world: f64 },
            PointerUp { utility: String, world: [f64; 2], shift: bool, ctrl: bool, meta: bool },
            CommitDraft,
            Escape,
            UtilityChanged,
        }
        input: ();
        output: ();
        effect: GestureEffect;
        context_from_input: gesture_context_from_input;
        initial: idle;

        state idle {
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_draft => drafting do gesture_start_draft;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on PointerUp if utility_is_select_direct => idle do gesture_pick_point;
        }
        state marqueeing {
            on PointerMove => marqueeing do gesture_update_marquee_cursor;
            on PointerUp => idle do gesture_commit_marquee;
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_draft => drafting do gesture_start_draft;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on Escape => idle;
            on UtilityChanged => idle;
        }
        state shape_dragging {
            on PointerMove => shape_dragging do gesture_update_shape_cursor;
            on PointerUp => idle do gesture_commit_shape;
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_draft => drafting do gesture_start_draft;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on Escape => idle;
            on UtilityChanged => idle;
        }
        state drafting {
            on PointerMove => drafting do gesture_update_draft_cursor;
            on PointerDown if utility_is_draft_same => drafting do gesture_append_draft_point;
            on PointerDown if utility_is_draft_different => drafting do gesture_start_draft;
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on CommitDraft => idle do gesture_commit_draft;
            on Escape => idle;
            on UtilityChanged => idle;
        }
    }
}
//#endregion 🔖️GestureStatechart

//#region 🧵️TracePointerJob
const TRACE_POINTER_WORK_PER_STEP: usize = 32;
const TRACE_POINTER_MAX_DEPTH: usize = 32;
const MAX_GESTURE_POINTS: usize = 48;
const MAX_LIVE_TRACE_POINTER_JOBS: usize = 64;
static NEXT_TRACE_POINTER_REQUEST: AtomicU64 = AtomicU64::new(20_000);

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
enum TracePointerWork {
    Roots { next: usize },
    Enter(TracePath),
    GroupChildren { path: TracePath, next: usize },
    Visit(TracePath),
    PathBounds { path: TracePath, next: usize, min: [f64; 2], max: [f64; 2] },
    PolygonBounds { path: TracePath, next: usize, min: [f64; 2], max: [f64; 2] },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TracePickCandidate {
    generality: i32,
    image_key: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TracePointerJob {
    app_instance_id: u32,
    document_id: String,
    operation_id: u64,
    generation: u64,
    base_revision: String,
    world: [f64; 2],
    work: Vec<TracePointerWork>,
    best: Option<TracePickCandidate>,
    completed_work: usize,
    replay_target: Option<(usize, usize)>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TracePointerJobKey {
    app_instance_id: u32,
    document_id: String,
    operation_id: u64,
    generation: u64,
    base_revision: String,
}

static TRACE_POINTER_JOBS: OnceLock<Mutex<BTreeMap<TracePointerJobKey, TracePointerJob>>> = OnceLock::new();
static ACTIVE_TRACE_POINTER_JOBS: OnceLock<Mutex<BTreeMap<(u32, String, u64), TracePointerJobKey>>> = OnceLock::new();

fn trace_pointer_jobs() -> &'static Mutex<BTreeMap<TracePointerJobKey, TracePointerJob>> {
    TRACE_POINTER_JOBS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn active_trace_pointer_jobs() -> &'static Mutex<BTreeMap<(u32, String, u64), TracePointerJobKey>> {
    ACTIVE_TRACE_POINTER_JOBS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn trace_job_key(job: &TracePointerJob) -> TracePointerJobKey {
    TracePointerJobKey { app_instance_id: job.app_instance_id, document_id: job.document_id.clone(), operation_id: job.operation_id, generation: job.generation, base_revision: job.base_revision.clone() }
}

fn put_trace_pointer_job(_document_id: &str, job: TracePointerJob) -> bool {
    let key = trace_job_key(&job);
    let mut jobs = trace_pointer_jobs().lock().expect("draw trace-pointer job lock");
    if !jobs.contains_key(&key) && jobs.len() >= MAX_LIVE_TRACE_POINTER_JOBS {
        return false;
    }
    active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").insert((key.app_instance_id, key.document_id.clone(), key.operation_id), key.clone());
    jobs.insert(key, job);
    true
}

fn take_trace_pointer_job(app_instance_id: u32, document_id: &str, operation_id: u64, generation: u64, base_revision: &str) -> Option<TracePointerJob> {
    let scope = (app_instance_id, document_id.to_string(), operation_id);
    let key = active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").get(&scope).cloned()?;
    if key.generation != generation || key.base_revision != base_revision {
        return None;
    }
    trace_pointer_jobs().lock().expect("draw trace-pointer job lock").remove(&key)
}

pub(crate) fn cancel_trace_pointer_job(app_instance_id: u32, document_id: &str, generation: u64) {
    if generation != 0 {
        let mut active = active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock");
        let keys: Vec<_> = active.iter().filter(|((app, parent, _), key)| *app == app_instance_id && parent == document_id && key.generation == generation).map(|(scope, _)| scope.clone()).collect();
        for scope in keys {
            if let Some(key) = active.remove(&scope) {
                trace_pointer_jobs().lock().expect("draw trace-pointer job lock").remove(&key);
            }
        }
    }
}

impl TracePointerJob {
    fn new(generation: u64, document: &DrawSnapshot, world: [f64; 2]) -> Self {
        Self::new_bound(generation, document, world, format!("unbound:{}", document.id))
    }

    fn new_bound(generation: u64, document: &DrawSnapshot, world: [f64; 2], base_revision: String) -> Self {
        Self { app_instance_id: 0, document_id: document.id.clone(), operation_id: 0, generation, base_revision, world, work: vec![TracePointerWork::Roots { next: document.layers.len() }], best: None, completed_work: 0, replay_target: None }
    }

    fn new_operation(operation: &semio_framework_plugin::AppOperationContext, document: &DrawSnapshot, world: [f64; 2]) -> Self {
        let mut job = Self::new_bound(operation.generation, document, world, operation.canonical_base_revision_hex());
        job.app_instance_id = operation.app_instance_id;
        job.document_id = operation.parent_document_id.clone();
        job.operation_id = operation.operation_id;
        job
    }

    fn advance(&mut self, document: &DrawSnapshot) -> bool {
        for _ in 0..TRACE_POINTER_WORK_PER_STEP {
            let Some(work) = self.work.pop() else { return true };
            self.completed_work += 1;
            match work {
                TracePointerWork::Roots { next } => {
                    if next == 0 {
                        continue;
                    }
                    self.work.push(TracePointerWork::Roots { next: next - 1 });
                    if let Some(path) = TracePath::root(next - 1) {
                        self.work.push(TracePointerWork::Enter(path));
                    }
                }
                TracePointerWork::Enter(path) => {
                    let Some(layer) = draw_layer_at_path(&document.layers, &path) else { continue };
                    if let DrawLayerNode::Group(group) = layer {
                        self.work.push(TracePointerWork::Visit(path));
                        self.work.push(TracePointerWork::GroupChildren { path, next: group.children.len() });
                    } else {
                        self.work.push(TracePointerWork::Visit(path));
                    }
                }
                TracePointerWork::GroupChildren { path, next } => {
                    if next == 0 {
                        continue;
                    }
                    self.work.push(TracePointerWork::GroupChildren { path, next: next - 1 });
                    if let Some(child_path) = path.child(next - 1) {
                        self.work.push(TracePointerWork::Enter(child_path));
                    }
                }
                TracePointerWork::Visit(path) => {
                    let Some(layer) = draw_layer_at_path(&document.layers, &path) else { continue };
                    match layer {
                        DrawLayerNode::Path(path_layer) if !path_layer.segments.is_empty() => self.work.push(TracePointerWork::PathBounds { path, next: 0, min: [f64::INFINITY; 2], max: [f64::NEG_INFINITY; 2] }),
                        DrawLayerNode::Shape(shape) if shape.shape_kind == "polygon" && shape.polygon.as_ref().is_some_and(|polygon| !polygon.points.is_empty()) => {
                            self.work.push(TracePointerWork::PolygonBounds { path, next: 0, min: [f64::INFINITY; 2], max: [f64::NEG_INFINITY; 2] })
                        }
                        _ => consider_trace_candidate(self, layer, trace_layer_world_bounds(layer)),
                    }
                }
                TracePointerWork::PathBounds { path, next, mut min, mut max } => {
                    let Some(DrawLayerNode::Path(path_layer)) = draw_layer_at_path(&document.layers, &path) else { continue };
                    if let Some(segment) = path_layer.segments.get(next) {
                        if let Some(point) = trace_segment_point(segment) {
                            extend_trace_bounds(&mut min, &mut max, point);
                        }
                        self.work.push(TracePointerWork::PathBounds { path, next: next + 1, min, max });
                    } else if min[0].is_finite() {
                        consider_trace_candidate(self, draw_layer_at_path(&document.layers, &path).expect("path work retains its layer"), trace_world_bounds(&path_layer.base.transform, min, max));
                    }
                }
                TracePointerWork::PolygonBounds { path, next, mut min, mut max } => {
                    let Some(DrawLayerNode::Shape(shape)) = draw_layer_at_path(&document.layers, &path) else { continue };
                    let Some(polygon) = &shape.polygon else { continue };
                    if let Some(point) = polygon.points.get(next) {
                        extend_trace_bounds(&mut min, &mut max, *point);
                        self.work.push(TracePointerWork::PolygonBounds { path, next: next + 1, min, max });
                    } else if min[0].is_finite() {
                        consider_trace_candidate(self, draw_layer_at_path(&document.layers, &path).expect("polygon work retains its layer"), trace_world_bounds(&shape.base.transform, min, max));
                    }
                }
            }
        }
        self.work.is_empty()
    }
}

fn consider_trace_candidate(job: &mut TracePointerJob, layer: &DrawLayerNode, bounds: (f64, f64, f64, f64)) {
    let base = trace_layer_base(layer);
    if !base.visible || base.locked || !trace_point_in_bounds(job.world, bounds) {
        return;
    }
    let candidate = TracePickCandidate {
        generality: match layer {
            DrawLayerNode::Group(_) => 0,
            DrawLayerNode::Boolean(_) | DrawLayerNode::Trace(_) => 1,
            _ => 2,
        },
        image_key: match layer {
            DrawLayerNode::Image(image) => Some(image.image_key.clone()),
            _ => None,
        },
    };
    if job.best.as_ref().is_none_or(|best| candidate.generality >= best.generality) {
        job.best = Some(candidate);
    }
}

fn draw_layer_at_path<'a>(roots: &'a [DrawLayerNode], path: &TracePath) -> Option<&'a DrawLayerNode> {
    let mut indices = path.indices();
    let mut layer = roots.get(indices.next()?)?;
    for index in indices {
        let DrawLayerNode::Group(group) = layer else { return None };
        layer = group.children.get(index)?;
    }
    Some(layer)
}

fn trace_segment_point(segment: &PathSegment) -> Option<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => Some(*to),
        PathSegment::Close => None,
    }
}

fn extend_trace_bounds(min: &mut [f64; 2], max: &mut [f64; 2], point: [f64; 2]) {
    min[0] = min[0].min(point[0]);
    min[1] = min[1].min(point[1]);
    max[0] = max[0].max(point[0]);
    max[1] = max[1].max(point[1]);
}

fn trace_world_bounds(transform: &crate::artifacts::draw::DrawTransform, min: [f64; 2], max: [f64; 2]) -> (f64, f64, f64, f64) {
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    let matrix = [transform.scale_x * cos, transform.scale_x * sin, -transform.scale_y * sin, transform.scale_y * cos, transform.x, transform.y];
    let corners = [min, [max[0], min[1]], max, [min[0], max[1]]];
    let mut world_min = [f64::INFINITY; 2];
    let mut world_max = [f64::NEG_INFINITY; 2];
    for corner in corners {
        extend_trace_bounds(&mut world_min, &mut world_max, trace_transform_point(matrix, corner));
    }
    (world_min[0], world_min[1], world_max[0] - world_min[0], world_max[1] - world_min[1])
}

fn trace_transform_point(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

fn trace_layer_base(layer: &DrawLayerNode) -> &crate::artifacts::draw::DrawLayerBase {
    match layer {
        DrawLayerNode::Shape(value) => &value.base,
        DrawLayerNode::Path(value) => &value.base,
        DrawLayerNode::Text(value) => &value.base,
        DrawLayerNode::Image(value) => &value.base,
        DrawLayerNode::Group(value) => &value.base,
        DrawLayerNode::Boolean(value) => &value.base,
        DrawLayerNode::Trace(value) => &value.base,
    }
}

fn trace_layer_world_bounds(layer: &DrawLayerNode) -> (f64, f64, f64, f64) {
    let local = match layer {
        DrawLayerNode::Text(value) => (value.x, value.y, (value.content.len() as f64 * value.size * 0.6).max(8.0), (value.size * 1.2).max(8.0)),
        DrawLayerNode::Image(value) => (0.0, 0.0, value.width, value.height),
        DrawLayerNode::Shape(value) => match value.shape_kind.as_str() {
            "rect" => value.rect.as_ref().map(|rect| (rect.x, rect.y, rect.width, rect.height)).unwrap_or((-64.0, -64.0, 128.0, 128.0)),
            "ellipse" => value.ellipse.as_ref().map(|ellipse| (ellipse.cx - ellipse.rx, ellipse.cy - ellipse.ry, ellipse.rx * 2.0, ellipse.ry * 2.0)).unwrap_or((-64.0, -64.0, 128.0, 128.0)),
            "circle" => value.circle.as_ref().map(|circle| (circle.cx - circle.r, circle.cy - circle.r, circle.r * 2.0, circle.r * 2.0)).unwrap_or((-64.0, -64.0, 128.0, 128.0)),
            "line" => value.line.as_ref().map(|line| (line.x1.min(line.x2), line.y1.min(line.y2), (line.x2 - line.x1).abs(), (line.y2 - line.y1).abs())).unwrap_or((-64.0, -64.0, 128.0, 128.0)),
            _ => (-64.0, -64.0, 128.0, 128.0),
        },
        DrawLayerNode::Path(_) => (-64.0, -64.0, 128.0, 128.0),
        DrawLayerNode::Group(_) | DrawLayerNode::Boolean(_) | DrawLayerNode::Trace(_) => (-64.0, -64.0, 128.0, 128.0),
    };
    trace_world_bounds(&trace_layer_base(layer).transform, [local.0, local.1], [local.0 + local.2, local.1 + local.3])
}

fn trace_point_in_bounds(point: [f64; 2], bounds: (f64, f64, f64, f64)) -> bool {
    point[0] >= bounds.0 && point[0] <= bounds.0 + bounds.2 && point[1] >= bounds.1 && point[1] <= bounds.1 + bounds.3
}
//#endregion 🧵️TracePointerJob

//#region 🔖️DrawSession
/// 🕹️ Owned snapshot of `InteractionView::selection(DRAW_INTERACTION_DOMAIN)`, read once per
/// dispatch by `ArtifactApp::handle` and threaded through `DrawSession` to every command handler —
/// decouples handlers from `semio_framework_plugin::app::InteractionView` itself (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawInteractionSnapshot {
    pub ids: Vec<String>,
}

/// 🧪️ `app_commands!` dispatch context — the live gesture snapshot, the preview tick counter, and
/// the current `"strokes"` interaction selection.
pub struct DrawSession {
    /// 🎭️ Live `fsm` snapshot driving pointer gestures.
    pub(crate) gesture: draw_gesture::Snapshot,
    /// 👻️ Per-`key` monotone counter for `gesture_preview`.
    preview_seq: u64,
    /// 🕹️ Current `"strokes"` selection — set by `ArtifactApp::handle` before every dispatch.
    pub(crate) interaction: DrawInteractionSnapshot,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DrawGestureCheckpoint {
    pub app_instance_id: u32,
    pub document_id: String,
    pub operation_id: u64,
    pub generation: u64,
    pub base_revision: String,
    state: String,
    context: GestureContext,
    preview_seq: u64,
}

impl Default for DrawSession {
    fn default() -> Self {
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        Self { gesture: fsm::init::<draw_gesture::DrawGesture>((), &mut sink), preview_seq: 0, interaction: DrawInteractionSnapshot::default() }
    }
}

impl DrawSession {
    pub(crate) fn from_checkpoint(checkpoint: &DrawGestureCheckpoint) -> Self {
        let mut session = Self::default();
        let event = match checkpoint.state.as_str() {
            "marqueeing" => Some(draw_gesture::Event::PointerDown { utility: if checkpoint.context.method == "lasso" { "selectLasso".into() } else { "selectMarquee".into() }, world: checkpoint.context.start, shift: false, ctrl: false, meta: false }),
            "shape_dragging" => Some(draw_gesture::Event::PointerDown { utility: checkpoint.context.utility.clone(), world: checkpoint.context.start, shift: false, ctrl: false, meta: false }),
            "drafting" => Some(draw_gesture::Event::PointerDown { utility: checkpoint.context.utility.clone(), world: checkpoint.context.points.first().copied().unwrap_or(checkpoint.context.start), shift: false, ctrl: false, meta: false }),
            _ => None,
        };
        if let Some(event) = event {
            let mut sink = Vec::new();
            fsm::macrostep(&mut session.gesture, event, &mut sink, &mut fsm::NullInspector);
        }
        session.gesture.context = checkpoint.context.clone();
        session.preview_seq = checkpoint.preview_seq;
        session
    }

    pub(crate) fn checkpoint(&self, app_instance_id: u32, document_id: &str, operation_id: u64, generation: u64, base_revision: &str) -> DrawGestureCheckpoint {
        let state = ["marqueeing", "shape_dragging", "drafting"].into_iter().find(|state| self.gesture.matches(state)).unwrap_or("idle");
        DrawGestureCheckpoint { app_instance_id, document_id: document_id.into(), operation_id, generation, base_revision: base_revision.into(), state: state.into(), context: self.gesture.context.clone(), preview_seq: self.preview_seq }
    }

    /// 🎭️ Feeds one gesture event through the shared `fsm` statechart, then drains and executes any
    /// requested `GestureEffect`s against the live document — the only place gesture control-flow
    /// (owned by `fsm`) meets document-mutating logic (owned by `draw`). `config` is read-only (camera
    /// zoom for hit-test tolerance); a pick/marquee hit becomes an `interactionSelect` request riding
    /// as a `Effect` on the returned `Emit` — selection itself is framework-owned now, never
    /// written back into `config`.
    pub(crate) fn step_gesture(&mut self, event: draw_gesture::Event, document: &DrawSnapshot, config: &DrawConfig) -> Emit<DrawMutation, DrawConfigMutation> {
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        fsm::macrostep(&mut self.gesture, event, &mut sink, &mut fsm::NullInspector);
        self.preview_seq = self.preview_seq.wrapping_add(1);
        let mut operations = Vec::new();
        let mut commit_description: Option<&'static str> = None;
        let mut select_request: Option<(Vec<String>, String)> = None;
        for command in sink {
            let fsm::Command::Effect(effect) = command else { continue };
            match effect {
                GestureEffect::CommitMarquee { start, end, active, merge, shift, ctrl, meta } => {
                    if active {
                        let crossing = end[0] < start[0];
                        select_request = Some((marquee_layer_hits(document, start, end, crossing), merge));
                    } else {
                        let picked = resolve_point_pick(document, &config.camera, end, false);
                        select_request = Some((picked.into_iter().collect(), selection_merge_mode(shift, ctrl, meta).to_string()));
                    }
                }
                GestureEffect::CommitShape { utility, start, end } => {
                    operations.extend(commit_shape_drag(document, &utility, start, end));
                    commit_description = Some("Add shape");
                }
                GestureEffect::CommitDraft { utility, points } => {
                    operations.extend(commit_draft(document, &utility, &points));
                    commit_description = Some("Commit draft");
                }
                GestureEffect::CommitTrace { world } => {
                    operations.extend(commit_trace_at(document, &config.camera, world));
                    commit_description = Some("Trace image");
                }
                GestureEffect::PickPoint { world, shift, ctrl, meta } => {
                    let picked = resolve_point_pick(document, &config.camera, world, true);
                    select_request = Some((picked.into_iter().collect(), selection_merge_mode(shift, ctrl, meta).to_string()));
                }
            }
        }
        let mut emit = match commit_description {
            Some(description) => commit_with_utility_reset(operations, description),
            None => Emit::default(),
        };
        if let Some((ids, merge)) = select_request {
            emit.effects.push(interaction_select_effect(&ids, &merge));
        }
        emit
    }

    /// 👻️ A `(key, seq, payload)` tuple already shaped as `SyncSession::publish_preview`'s exact
    /// argument list once a host bridge can carry it out of this sandboxed plugin. `#[allow(dead_code)]`:
    /// no caller exists inside this crate today; exercised by `🧪️Tests` only until that bridge lands.
    #[allow(dead_code)]
    pub(crate) fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let payload = draw_gesture_preview_payload(&self.gesture.context, self.gesture.matches("idle"))?;
        Some(("gesture", self.preview_seq, serde_json::to_vec(&payload).ok()?))
    }
}

/// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": a pure, JSON-serializable
/// snapshot of the gesture machine's live, uncommitted scratch geometry. `None` while `draw_gesture`
/// is `idle` (no live gesture to preview); this function only ever reads `GestureContext`, never
/// `DrawSnapshot`/`DrawMutation` — a preview can never become persistent state.
fn draw_gesture_preview_payload(ctx: &GestureContext, is_idle: bool) -> Option<serde_json::Value> {
    if is_idle {
        return None;
    }
    Some(serde_json::json!({
        "method": ctx.method,
        "utility": ctx.utility,
        "start": ctx.start,
        "cursor": ctx.cursor,
        "points": ctx.points,
        "active": ctx.active,
    }))
}
//#endregion 🔖️DrawSession

use serde::{Deserialize, Serialize};

//#region 🧵️TracePointerContinuation
fn trace_progress(job: &TracePointerJob) -> DrawConfigMutation {
    DrawConfigMutation::SetTracePointerProgress { generation: job.generation, completed_work: job.completed_work as u64, pending_work: job.work.len() as u64 }
}

fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> Option<Effect> {
    let (checkpoint_completed_work, checkpoint_pending_work) = job.replay_target.unwrap_or((job.completed_work, job.work.len()));
    let continuation = CanvasPointerDown {
        app_instance_id: Some(job.app_instance_id),
        parent_document_id: Some(job.document_id.clone()),
        operation_id: Some(job.operation_id),
        generation: Some(job.generation),
        base_revision: Some(job.base_revision.clone()),
        world_x: Some(job.world[0]),
        world_y: Some(job.world[1]),
        checkpoint_completed_work: Some(checkpoint_completed_work as u64),
        checkpoint_pending_work: Some(checkpoint_pending_work as u64),
        ..payload.clone()
    };
    let args = serde_json::to_value(continuation).ok().and_then(|value| semio_framework::optional_json_to_dsl(Some(value)));
    Some(Effect::DispatchAction { req: RequestId(NEXT_TRACE_POINTER_REQUEST.fetch_add(1, Ordering::Relaxed)), action: "canvasPointerDown".into(), args, delay_ms: 0 })
}

fn advance_trace_pointer(mut job: TracePointerJob, payload: &CanvasPointerDown, document: &DrawSnapshot) -> Emit<DrawMutation, DrawConfigMutation> {
    if let Some((target_completed, target_pending)) = job.replay_target {
        let complete = job.advance(document);
        if job.completed_work > target_completed || (job.completed_work == target_completed && job.work.len() != target_pending) || (complete && job.completed_work < target_completed) {
            return Emit::default();
        }
        if job.completed_work < target_completed {
            let Some(effect) = queue_trace_pointer(payload, &job) else { return Emit::default() };
            put_trace_pointer_job(&document.id, job.clone());
            return Emit { config_mutations: vec![trace_progress(&job)], effects: vec![effect], ..Default::default() };
        }
        job.replay_target = None;
        let Some(effect) = queue_trace_pointer(payload, &job) else { return Emit::default() };
        put_trace_pointer_job(&document.id, job.clone());
        return Emit { config_mutations: vec![trace_progress(&job)], effects: vec![effect], ..Default::default() };
    }
    if !job.advance(document) {
        let Some(effect) = queue_trace_pointer(payload, &job) else { return Emit::default() };
        put_trace_pointer_job(&document.id, job.clone());
        return Emit { config_mutations: vec![trace_progress(&job)], effects: vec![effect], ..Default::default() };
    }
    let source_key = job.best.and_then(|candidate| candidate.image_key).or_else(|| document.assets.keys().next().cloned());
    active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").remove(&(job.app_instance_id, job.document_id, job.operation_id));
    let mut emit = commit_with_utility_reset(commit_trace_source(document, source_key), "Trace image");
    emit.config_mutations.push(DrawConfigMutation::SetTracePointerProgress { generation: 0, completed_work: 0, pending_work: 0 });
    emit
}
//#endregion 🧵️TracePointerContinuation

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_instance_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub world_x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub world_y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_completed_work: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_pending_work: Option<u64>,
}

pub async fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let operation = doc.operation()?;
    let document_revision = crate::editor::draw::draw_document_revision(doc, cfg.snapshot);
    if let Some(generation) = payload.generation {
        let Some(base_revision) = payload.base_revision.as_deref() else { return Ok(Emit::default()) };
        if base_revision.len() != 64
            || payload.parent_document_id.as_ref().is_some_and(|id| id.len() > 256)
            || cfg.snapshot.active_utility_id != "trace"
            || cfg.snapshot.trace_pointer_generation != generation
            || payload.app_instance_id != Some(operation.app_instance_id)
            || payload.parent_document_id.as_deref() != Some(operation.parent_document_id.as_str())
            || base_revision != document_revision
        {
            return Ok(Emit::default());
        }
        let Some(operation_id) = payload.operation_id else { return Ok(Emit::default()) };
        let job = take_trace_pointer_job(operation.app_instance_id, &operation.parent_document_id, operation_id, generation, base_revision).or_else(|| {
            let completed = usize::try_from(payload.checkpoint_completed_work?).ok()?;
            let pending = usize::try_from(payload.checkpoint_pending_work?).ok()?;
            let mut job = TracePointerJob::new_bound(generation, document, [payload.world_x?, payload.world_y?], base_revision.into());
            job.app_instance_id = operation.app_instance_id;
            job.document_id = operation.parent_document_id.clone();
            job.operation_id = operation_id;
            job.replay_target = Some((completed, pending));
            Some(job)
        });
        let Some(job) = job else { return Ok(Emit::default()) };
        if job.replay_target.is_none() && (payload.checkpoint_completed_work != Some(job.completed_work as u64) || payload.checkpoint_pending_work != Some(job.work.len() as u64)) {
            put_trace_pointer_job(&document.id, job);
            return Ok(Emit::default());
        }
        return Ok(advance_trace_pointer(job, payload, document));
    }
    let config = cfg.snapshot;
    let (world_x, world_y) = canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
    let active_utility = config.active_utility_id.clone();
    if active_utility == "trace" {
        let scope = (operation.app_instance_id, operation.parent_document_id.clone(), operation.operation_id);
        let has_active = active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").contains_key(&scope);
        if !has_active && trace_pointer_jobs().lock().expect("draw trace-pointer job lock").len() >= MAX_LIVE_TRACE_POINTER_JOBS {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("draw.trace-pointer.busy"), "the bounded Draw trace-pointer job pool is full"));
        }
        cancel_trace_pointer_job(operation.app_instance_id, &operation.parent_document_id, config.trace_pointer_generation);
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        fsm::macrostep(&mut session.gesture, draw_gesture::Event::PointerDown { utility: "trace".into(), world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta }, &mut sink, &mut fsm::NullInspector);
        session.preview_seq = session.preview_seq.wrapping_add(1);
        return Ok(advance_trace_pointer(TracePointerJob::new_operation(operation, document, [world_x, world_y]), payload, document));
    }
    let emit = session.step_gesture(draw_gesture::Event::PointerDown { utility: active_utility, world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta }, document, config);
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::draw::testkit::{draw_app_with_registry, set_utility, DrawApp};
    use crate::editor::draw::DrawCommand;
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::PluginApp;

    fn payload(generation: Option<u64>) -> CanvasPointerDown {
        CanvasPointerDown { x: 1.0, y: 1.0, width: 100.0, height: 100.0, shift: false, ctrl: false, meta: false, generation, checkpoint_completed_work: None, checkpoint_pending_work: None, ..Default::default() }
    }

    async fn app_with_document_id(id: &str) -> DrawApp {
        let mut app = draw_app_with_registry().await;
        let mut snapshot = crate::artifacts::draw::schema::default_draw_document(id, None);
        snapshot.layers = (0..96).map(|index| create_draw_path_layer(format!("{id}-layer-{index}"), vec![PathSegment::Move { to: [index as f64, index as f64] }])).collect();
        let envelope = store::create_document_envelope::<_, DrawMutation>(crate::artifacts::draw::DRAW_DOCUMENT_SCHEMA, id, snapshot, None);
        let files = store::print_document_pack(&envelope).await.expect("Draw document pack");
        app.load_document_pack(&files).await.expect("load Draw document identity");
        app.handle_action("setActiveUtility", Some(&serde_json::json!({ "utilityId": "trace" })), &meta(id)).await.expect("set public Draw trace utility");
        app
    }

    fn continuation(result: semio_framework_plugin::InvocationResult) -> Option<serde_json::Value> {
        result.requested_effects.into_iter().find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "canvasPointerDown" => args.map(store::pack_rt::dsl_value_to_json),
            _ => None,
        })
    }

    #[semio_framework_async_macros::async_test]
    async fn trace_pointer_step_consumes_at_most_the_fixed_work_budget() {
        let mut document = crate::artifacts::draw::schema::default_draw_document("bounded-trace", None);
        let mut segments = vec![PathSegment::Move { to: [0.0, 0.0] }];
        segments.extend((0..256).map(|index| PathSegment::Line { to: [index as f64, index as f64] }));
        document.layers = vec![create_draw_path_layer("long-path", segments)];
        let mut job = TracePointerJob::new(7, &document, [4.0, 4.0]);

        assert!(!job.advance(&document));
        assert_eq!(job.completed_work, TRACE_POINTER_WORK_PER_STEP);
        assert!(!job.work.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn continuation_work_helpers_have_synchronous_compile_shape() {
        let _: fn(&mut TracePointerJob, &DrawSnapshot) -> bool = TracePointerJob::advance;
        let _: fn(Vec<DrawMutation>, &str) -> Emit<DrawMutation, DrawConfigMutation> = commit_with_utility_reset;
        let _: fn(&DrawLayerNode) -> (f64, f64, f64, f64) = trace_layer_world_bounds;
    }

    #[semio_framework_async_macros::async_test]
    async fn stale_input_generation_neither_advances_nor_commits() {
        let document = crate::artifacts::draw::schema::default_draw_document("fresh-trace", None);
        let history = semio_framework_plugin::HistoryView::empty().await;
        let view = ArtifactView::new(&document, &history).await;
        let config = DrawConfig { active_utility_id: "trace".into(), trace_pointer_generation: 12, ..Default::default() };
        put_trace_pointer_job(&document.id, TracePointerJob::new(11, &document, [0.0, 0.0]));
        let mut stale_payload = payload(Some(11));
        stale_payload.checkpoint_completed_work = Some(0);
        stale_payload.checkpoint_pending_work = Some(1);
        let emit = handle(&stale_payload, &view, &ConfigView { snapshot: &config }, &mut DrawSession::default()).await.expect("stale handler");
        assert!(emit.artifact_mutations.is_empty());
        assert!(emit.effects.is_empty());
        assert!(emit.config_mutations.is_empty());
    }

    #[test]
    fn wide_roots_and_groups_enqueue_at_most_two_items_per_work_unit() {
        let leaf = create_draw_path_layer("leaf", vec![PathSegment::Move { to: [0.0, 0.0] }]);
        let mut roots = crate::artifacts::draw::schema::default_draw_document("wide-roots", None);
        roots.layers = vec![leaf.clone(); 10_000];
        let mut roots_job = TracePointerJob::new(8, &roots, [0.0, 0.0]);
        roots_job.advance(&roots);
        assert_eq!(roots_job.completed_work, TRACE_POINTER_WORK_PER_STEP);
        assert!(roots_job.work.len() <= TRACE_POINTER_WORK_PER_STEP + 2);

        let mut group = crate::artifacts::draw::schema::create_draw_group_layer("wide");
        let DrawLayerNode::Group(body) = &mut group else { unreachable!() };
        body.children = vec![leaf; 10_000];
        let mut document = crate::artifacts::draw::schema::default_draw_document("wide-group", None);
        document.layers = vec![group];
        let mut job = TracePointerJob::new(9, &document, [0.0, 0.0]);
        job.advance(&document);
        assert_eq!(job.completed_work, TRACE_POINTER_WORK_PER_STEP);
        assert!(job.work.len() <= TRACE_POINTER_WORK_PER_STEP + 2);
    }

    #[test]
    fn production_handler_continues_across_worker_threads_with_fixed_size_checkpoint() {
        let mut document = crate::artifacts::draw::schema::default_draw_document("cross-worker", None);
        let segments = (0..256).map(|index| PathSegment::Line { to: [index as f64, index as f64] }).collect();
        document.layers = vec![create_draw_path_layer("long", segments)];
        let document = std::sync::Arc::new(document);
        let first_document = document.clone();
        let generation = std::thread::spawn(move || {
            let history = semio_framework::io::resolve_ready(semio_framework_plugin::HistoryView::empty());
            let view = semio_framework::io::resolve_ready(ArtifactView::new(first_document.as_ref(), &history));
            let config = DrawConfig { active_utility_id: "trace".into(), ..Default::default() };
            let emit = semio_framework::io::resolve_ready(handle(&payload(None), &view, &ConfigView { snapshot: &config }, &mut DrawSession::default())).expect("initial handler");
            assert_eq!(emit.effects.len(), 1);
            let DrawConfigMutation::SetTracePointerProgress { generation, .. } = emit.config_mutations[0] else { panic!("trace progress") };
            generation
        })
        .join()
        .expect("initial worker joins");
        let second_document = document.clone();
        std::thread::spawn(move || {
            let (completed, pending, base_revision, world) = {
                let jobs = trace_pointer_jobs().lock().expect("trace jobs");
                let job = jobs.values().find(|job| job.document_id == second_document.id && job.generation == generation).expect("portable job");
                (job.completed_work as u64, job.work.len() as u64, job.base_revision.clone(), job.world)
            };
            let history = semio_framework::io::resolve_ready(semio_framework_plugin::HistoryView::empty());
            let view = semio_framework::io::resolve_ready(ArtifactView::new(second_document.as_ref(), &history));
            let config = DrawConfig { active_utility_id: "trace".into(), trace_pointer_generation: generation, ..Default::default() };
            let mut continuation = payload(Some(generation));
            continuation.checkpoint_completed_work = Some(completed);
            continuation.checkpoint_pending_work = Some(pending);
            continuation.base_revision = Some(base_revision);
            continuation.world_x = Some(world[0]);
            continuation.world_y = Some(world[1]);
            let emit = semio_framework::io::resolve_ready(handle(&continuation, &view, &ConfigView { snapshot: &config }, &mut DrawSession::default())).expect("continuation handler");
            assert!(!emit.effects.is_empty() || !emit.config_mutations.is_empty());
        })
        .join()
        .expect("continuation worker joins");
    }

    #[test]
    fn adversarial_wide_tree_handler_and_checkpoint_encoding_stay_under_watchdog() {
        let leaf = create_draw_path_layer("leaf", vec![PathSegment::Move { to: [0.0, 0.0] }]);
        let mut document = crate::artifacts::draw::schema::default_draw_document("timed-wide", None);
        document.layers = vec![leaf; 20_000];
        let history = semio_framework::io::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let view = semio_framework::io::resolve_ready(ArtifactView::new(&document, &history));
        let config = DrawConfig { active_utility_id: "trace".into(), ..Default::default() };
        let started = std::time::Instant::now();
        let emit = semio_framework::io::resolve_ready(handle(&payload(None), &view, &ConfigView { snapshot: &config }, &mut DrawSession::default())).expect("wide handler");
        let elapsed = started.elapsed();
        assert_eq!(emit.effects.len(), 1);
        assert!(elapsed < std::time::Duration::from_millis(8), "wide handler plus checkpoint encoding took {elapsed:?}");
        let Effect::DispatchAction { args, .. } = &emit.effects[0] else { panic!("continuation dispatch") };
        assert!(format!("{args:?}").len() < 512, "checkpoint must remain fixed-size");
    }

    #[test]
    fn same_generation_isolated_by_document_identity() {
        let first = crate::artifacts::draw::schema::default_draw_document("registry-first", None);
        let second = crate::artifacts::draw::schema::default_draw_document("registry-second", None);
        put_trace_pointer_job(&first.id, TracePointerJob::new(77, &first, [1.0, 1.0]));
        put_trace_pointer_job(&second.id, TracePointerJob::new(77, &second, [2.0, 2.0]));
        assert_eq!(take_trace_pointer_job(0, &first.id, 0, 77, &format!("unbound:{}", first.id)).map(|job| job.world), Some([1.0, 1.0]));
        assert_eq!(take_trace_pointer_job(0, &second.id, 0, 77, &format!("unbound:{}", second.id)).map(|job| job.world), Some([2.0, 2.0]));
    }

    #[test]
    fn more_than_thirty_two_documents_keep_their_active_jobs() {
        for index in 0..40 {
            let document = crate::artifacts::draw::schema::default_draw_document(&format!("registry-many-{index}"), None);
            put_trace_pointer_job(&document.id, TracePointerJob::new(88, &document, [index as f64, 0.0]));
        }
        for index in 0..40 {
            let id = format!("registry-many-{index}");
            assert_eq!(take_trace_pointer_job(0, &id, 0, 88, &format!("unbound:{id}")).map(|job| job.world[0]), Some(index as f64));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn public_action_bus_reconstructs_after_registry_loss_and_rejects_stale_revision_under_eight_ms() {
        trace_pointer_jobs().lock().expect("draw trace-pointer job lock").clear();
        active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").clear();
        crate::editor::draw::draw_sessions().lock().expect("draw session lock").clear();
        crate::editor::draw::active_draw_sessions().lock().expect("draw active session lock").clear();
        let mut app = draw_app_with_registry().await;
        for _ in 0..96 {
            app.handle_action("addLayer", Some(&serde_json::json!({ "kind": "shape:rect" })), &meta("local")).await.expect("seed bounded trace scene");
        }
        set_utility(&mut app, "trace").await;
        let command = DrawCommand::CanvasPointerDown(CanvasPointerDown { x: 50.0, y: 50.0, width: 100.0, height: 100.0, ..Default::default() });
        let started = std::time::Instant::now();
        let wire = <DrawCommand as protocol::OpBinary>::encode_op(&command).expect("Draw command encode");
        assert_eq!(<DrawCommand as protocol::OpBinary>::decode_op(&wire).expect("Draw command decode"), command);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Draw public command codec envelope exceeded 8 ms");

        let args = serde_json::json!({ "x": 50.0, "y": 50.0, "width": 100.0, "height": 100.0, "shift": false, "ctrl": false, "meta": false });
        let started = std::time::Instant::now();
        let first = app.handle_action("canvasPointerDown", Some(&args), &meta("local")).await.expect("public Draw trace start");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Draw public start handler/job/op-codec/diff/apply envelope exceeded 8 ms");
        let checkpoint = first
            .requested_effects
            .into_iter()
            .find_map(|effect| match effect {
                Effect::DispatchAction { action, args, .. } if action == "canvasPointerDown" => args.map(store::pack_rt::dsl_value_to_json),
                _ => None,
            })
            .expect("Draw trace continuation");

        trace_pointer_jobs().lock().expect("draw trace-pointer job lock").clear();
        active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").clear();
        crate::editor::draw::draw_sessions().lock().expect("draw session lock").clear();
        crate::editor::draw::active_draw_sessions().lock().expect("draw active session lock").clear();
        let started = std::time::Instant::now();
        let resumed = app.handle_action("canvasPointerDown", Some(&checkpoint), &meta("local")).await.expect("restart-reconstructed Draw continuation");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Draw restart handler/job/op-codec/diff/apply envelope exceeded 8 ms");
        assert!(!resumed.requested_effects.is_empty(), "substantive trace state must resume after every process registry is cleared");

        app.handle_action("addLayer", Some(&serde_json::json!({ "kind": "shape:rect" })), &meta("local")).await.expect("advance document revision");
        let started = std::time::Instant::now();
        let stale = app.handle_action("canvasPointerDown", Some(&checkpoint), &meta("local")).await.expect("stale Draw continuation is a no-op");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Draw stale-revision rejection exceeded 8 ms");
        assert!(stale.mutations.is_empty() && stale.requested_effects.is_empty(), "an ABA/stale Draw continuation must neither mutate nor requeue");
        set_utility(&mut app, "selectDirect").await;
        assert!(trace_pointer_jobs().lock().expect("draw trace-pointer job lock").is_empty());
        assert!(active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn public_action_bus_isolates_two_documents_and_cancellation() {
        trace_pointer_jobs().lock().expect("draw trace-pointer job lock").clear();
        active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").clear();
        crate::editor::draw::draw_sessions().lock().expect("draw session lock").clear();
        crate::editor::draw::active_draw_sessions().lock().expect("draw active session lock").clear();
        let mut first = app_with_document_id("draw-document-a").await;
        let mut second = app_with_document_id("draw-document-b").await;
        let args = serde_json::json!({ "x": 50.0, "y": 50.0, "width": 100.0, "height": 100.0, "shift": false, "ctrl": false, "meta": false });
        let first_checkpoint = continuation(first.handle_action("canvasPointerDown", Some(&args), &meta("document-a")).await.expect("Draw document A start")).expect("Draw checkpoint A");
        let second_checkpoint = continuation(second.handle_action("canvasPointerDown", Some(&args), &meta("document-b")).await.expect("Draw document B start")).expect("Draw checkpoint B");
        let first_payload: CanvasPointerDown = serde_json::from_value(first_checkpoint.clone()).expect("Draw checkpoint A decode");
        let second_payload: CanvasPointerDown = serde_json::from_value(second_checkpoint.clone()).expect("Draw checkpoint B decode");
        assert_ne!(first_payload.generation, second_payload.generation);
        assert!(trace_pointer_jobs().lock().expect("draw trace-pointer job lock").keys().any(|key| key.document_id == "draw-document-a"));
        assert!(trace_pointer_jobs().lock().expect("draw trace-pointer job lock").keys().any(|key| key.document_id == "draw-document-b"));

        first.handle_action("setActiveUtility", Some(&serde_json::json!({ "utilityId": "selectDirect" })), &meta("document-a")).await.expect("cancel Draw document A");
        assert!(!trace_pointer_jobs().lock().expect("draw trace-pointer job lock").keys().any(|key| key.document_id == "draw-document-a"));
        assert!(trace_pointer_jobs().lock().expect("draw trace-pointer job lock").keys().any(|key| key.document_id == "draw-document-b"));
        let sibling = second.handle_action("canvasPointerDown", Some(&second_checkpoint), &meta("document-b")).await.expect("continue Draw document B");
        assert!(continuation(sibling).is_some(), "cancelling one Draw document must not cancel another");
        let foreign = second.handle_action("canvasPointerDown", Some(&first_checkpoint), &meta("document-b")).await.expect("foreign Draw continuation no-op");
        assert!(foreign.requested_effects.is_empty() && foreign.mutations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn public_action_bus_returns_busy_for_the_sixty_fifth_trace() {
        trace_pointer_jobs().lock().expect("draw trace-pointer job lock").clear();
        active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").clear();
        let args = serde_json::json!({ "x": 50.0, "y": 50.0, "width": 100.0, "height": 100.0, "shift": false, "ctrl": false, "meta": false });
        let mut admitted = Vec::new();
        for index in 0..MAX_LIVE_TRACE_POINTER_JOBS {
            let mut app = app_with_document_id(&format!("draw-admission-{index}")).await;
            let started = std::time::Instant::now();
            let result = app.handle_action("canvasPointerDown", Some(&args), &meta("admission")).await.expect("admit Draw trace");
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "Draw admitted trace envelope exceeded 8 ms");
            assert!(continuation(result).is_some());
            admitted.push(app);
        }
        let mut sixty_fifth = app_with_document_id("draw-admission-65").await;
        let started = std::time::Instant::now();
        let busy = sixty_fifth.handle_action("canvasPointerDown", Some(&args), &meta("admission")).await.expect_err("the 65th Draw trace must be Busy");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Draw 65th Busy envelope exceeded 8 ms");
        assert_eq!(busy.code.0, "draw.trace-pointer.busy");
        drop(admitted);
        trace_pointer_jobs().lock().expect("draw trace-pointer job lock").clear();
        active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").clear();
    }

    #[semio_framework_async_macros::async_test]
    async fn public_action_bus_caps_adversarial_depth_and_reconstructs_under_eight_ms() {
        trace_pointer_jobs().lock().expect("draw trace-pointer job lock").clear();
        active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").clear();
        let mut layer = create_draw_path_layer("deep-leaf", vec![PathSegment::Move { to: [0.0, 0.0] }]);
        for depth in 0..(TRACE_POINTER_MAX_DEPTH * 2) {
            let mut group = crate::artifacts::draw::schema::create_draw_group_layer(&format!("deep-group-{depth}"));
            let DrawLayerNode::Group(body) = &mut group else { unreachable!() };
            body.children.push(layer);
            layer = group;
        }
        let mut snapshot = crate::artifacts::draw::schema::default_draw_document("draw-deep", None);
        snapshot.layers = vec![layer];
        let envelope = store::create_document_envelope::<_, DrawMutation>(crate::artifacts::draw::DRAW_DOCUMENT_SCHEMA, "draw-deep", snapshot, None);
        let files = store::print_document_pack(&envelope).await.expect("deep Draw document pack");
        let mut app = draw_app_with_registry().await;
        app.load_document_pack(&files).await.expect("load deep Draw document");
        app.handle_action("setActiveUtility", Some(&serde_json::json!({ "utilityId": "trace" })), &meta("deep")).await.expect("set deep Draw trace utility");
        let args = serde_json::json!({ "x": 50.0, "y": 50.0, "width": 100.0, "height": 100.0, "shift": false, "ctrl": false, "meta": false });
        let started = std::time::Instant::now();
        let first = app.handle_action("canvasPointerDown", Some(&args), &meta("deep")).await.expect("deep Draw public start");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "deep Draw start envelope exceeded 8 ms");
        let checkpoint = continuation(first).expect("deep Draw continuation");
        trace_pointer_jobs().lock().expect("draw trace-pointer job lock").clear();
        active_trace_pointer_jobs().lock().expect("draw active trace-pointer lock").clear();
        let started = std::time::Instant::now();
        let resumed = app.handle_action("canvasPointerDown", Some(&checkpoint), &meta("deep")).await.expect("deep Draw reconstructed continuation");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "deep Draw reconstructed envelope exceeded 8 ms");
        assert!(continuation(resumed).is_some(), "capped deep traversal must remain substantively resumable");
    }

    #[semio_framework_async_macros::async_test]
    async fn pointer_down_fsm_inputs_settle_in_one_microstep() {
        let mut session = DrawSession::default();
        for utility in ["selectMarquee", "shapeRect", "pen", "shapePolygon", "trace"] {
            let mut sink = Vec::new();
            let report = fsm::macrostep(&mut session.gesture, draw_gesture::Event::PointerDown { utility: utility.into(), world: [1.0, 2.0], shift: false, ctrl: false, meta: false }, &mut sink, &mut fsm::NullInspector);
            assert!(report.microsteps <= 1, "{utility} used {} microsteps", report.microsteps);
        }
    }
}
//#endregion 🧪️Tests
