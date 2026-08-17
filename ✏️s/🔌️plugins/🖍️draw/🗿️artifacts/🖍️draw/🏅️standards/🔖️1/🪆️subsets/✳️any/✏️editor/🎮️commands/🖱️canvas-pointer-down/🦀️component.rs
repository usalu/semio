//! 🖱️ 🖱️ Draw play app commands command — `canvas-pointer-down`.

use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::editor::draw::{DRAW_INTERACTION_DOMAIN, DRAW_INTERACTION_GRANULARITY};
use crate::artifacts::draw::schema::{create_draw_path_layer, create_draw_trace_layer, draw_layer_world_bounds, draw_transform_to_matrix, find_draw_layer, flatten_draw_layers, layer_base, layer_id, layer_to_path_segments};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot, DrawLayerNode, PathSegment};
use semio_framework_plugin::{kernel::Effect, ConfigView, ArtifactView, Emit, Fault};

//#region 🔖️GestureContext
/// 🎛️ Per-gesture scratch geometry threaded through the shared `fsm` statechart below — one flat
/// struct (XState convention: context is machine-global, never per-state) mirroring the fields the
/// old hand-rolled `DrawDragState` enum kept per-variant.
#[derive(Clone, Debug, Default, PartialEq)]
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
    request_interaction_action(
        semio_framework::INTERACTION_SELECT_ACTION_ID,
        serde_json::json!({ "domainId": DRAW_INTERACTION_DOMAIN, "targets": interaction_targets_json(ids), "merge": merge, "method": "pick" }),
    )
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
    let source_key = hit_layer_id
        .and_then(|id| find_draw_layer(doc, &id).cloned())
        .and_then(|layer| match layer {
            DrawLayerNode::Image(image) => Some(image.image_key),
            _ => None,
        })
        .or_else(|| doc.assets.keys().next().cloned());
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
fn gesture_start_marquee(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, shift, ctrl, meta }) = event {
        ctx.method = if utility == "selectLasso" { "lasso".into() } else { "rectangle".into() };
        ctx.start = *world;
        ctx.cursor = *world;
        ctx.merge = selection_merge_mode(*shift, *ctrl, *meta).into();
        ctx.active = false;
    }
}

fn gesture_start_shape(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, .. }) = event {
        ctx.utility = utility.clone();
        ctx.start = *world;
        ctx.cursor = *world;
    }
}

fn gesture_start_draft(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, .. }) = event {
        ctx.utility = utility.clone();
        ctx.points = vec![*world];
        ctx.cursor = *world;
    }
}

fn gesture_append_draft_point(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { world, .. }) = event {
        ctx.points.push(*world);
        ctx.cursor = *world;
    }
}

fn gesture_update_marquee_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerMove { world, marquee_threshold_world }) = event {
        let distance = ((world[0] - ctx.start[0]).powi(2) + (world[1] - ctx.start[1]).powi(2)).sqrt();
        ctx.active = ctx.active || distance >= *marquee_threshold_world;
        ctx.cursor = *world;
    }
}

fn gesture_update_shape_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerMove { world, .. }) = event {
        ctx.cursor = *world;
    }
}

fn gesture_update_draft_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerMove { world, .. }) = event {
        ctx.cursor = *world;
    }
}

fn gesture_commit_marquee(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerUp { world, shift, ctrl, meta, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitMarquee { start: ctx.start, end: *world, active: ctx.active, merge: ctx.merge.clone(), shift: *shift, ctrl: *ctrl, meta: *meta }));
    }
}

fn gesture_commit_shape(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerUp { world, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitShape { utility: ctx.utility.clone(), start: ctx.start, end: *world }));
    }
}

fn gesture_commit_draft(ctx: &mut GestureContext, _event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    sink.push(fsm::Command::Effect(GestureEffect::CommitDraft { utility: ctx.utility.clone(), points: ctx.points.clone() }));
}

fn gesture_commit_trace(_ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { world, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitTrace { world: *world }));
    }
}

fn gesture_pick_point(_ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
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

impl Default for DrawSession {
    fn default() -> Self {
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        Self { gesture: fsm::init::<draw_gesture::DrawGesture>((), &mut sink), preview_seq: 0, interaction: DrawInteractionSnapshot::default() }
    }
}

impl DrawSession {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
}

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let (world_x, world_y) = canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
    let active_utility = config.active_utility_id.clone();
    let emit = session.step_gesture(draw_gesture::Event::PointerDown { utility: active_utility, world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta }, document, config);
    Ok(emit)
}
