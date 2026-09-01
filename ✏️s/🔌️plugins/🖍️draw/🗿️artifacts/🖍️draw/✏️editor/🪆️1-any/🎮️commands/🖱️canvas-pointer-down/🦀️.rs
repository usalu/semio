//! 🖱️ 🖱️ Draw play app commands command — `canvas-pointer-down`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::{create_draw_path_layer, create_draw_trace_layer, layer_id};
use crate::artifacts::draw::{DrawCamera, DrawLayerNode, DrawSnapshot, PathSegment};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::editor::draw::{DRAW_INTERACTION_DOMAIN, DRAW_INTERACTION_GRANULARITY};
use semio_framework_plugin::{kernel::Effect, ArtifactView, ConfigView, Emit, Fault, RequestId, UiFixedList};
use std::sync::atomic::{AtomicU64, Ordering};

//#region 🔖️GestureContext
pub const DRAW_GESTURE_PREVIEW_POINT_CAPACITY: usize = 256;

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
    pub(crate) points: UiFixedList<[f64; 2], DRAW_GESTURE_PREVIEW_POINT_CAPACITY>,
    #[serde(skip)]
    pub(crate) points_overflowed: bool,
    active: bool,
}

/// 🎇️ Document-touching side effects the gesture machine requests but never executes — `fsm`'s
/// guards/actions only ever see `(&Context, Option<&Event>)`, never the `DrawSnapshot` tree, so every
/// hit-test/commit that needs the document is deferred to `DrawSession::step_gesture` as an effect.
#[derive(Clone, Debug)]
pub enum GestureEffect {
    CommitMarquee { start: [f64; 2], end: [f64; 2], active: bool, merge: String, shift: bool, ctrl: bool, meta: bool },
    CommitShape { utility: String, start: [f64; 2], end: [f64; 2] },
    CommitDraft { utility: String, points: UiFixedList<[f64; 2], DRAW_GESTURE_PREVIEW_POINT_CAPACITY> },
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
/// 🕹️ Requests the shell to redispatch a framework-owned interaction verb (`interactionSelect`/
/// `interactionHover`) through its normal action funnel — the only way an `ArtifactApp::handle`
/// (or its gesture machine) can drive selection/hover now that both are framework-owned state,
/// never a `DrawConfigMutation` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub(crate) fn request_interaction_action(action_id: &str, args: serde_json::Value) -> Effect {
    Effect::ReplayShellCommand { action_id: action_id.into(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

pub(crate) fn interaction_select_effect_from_targets(targets: String, merge: &str) -> Effect {
    request_interaction_action(semio_framework::INTERACTION_SELECT_ACTION_ID, serde_json::json!({ "domainId": DRAW_INTERACTION_DOMAIN, "targets": targets, "merge": merge, "method": "pick" }))
}

pub(crate) fn interaction_hover_effect_from_targets(targets: String) -> Effect {
    request_interaction_action(semio_framework::INTERACTION_HOVER_ACTION_ID, serde_json::json!({ "domainId": DRAW_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets }))
}

#[cfg(test)]
pub(crate) fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    let targets = serde_json::to_string(&ids.iter().map(|id| serde_json::json!({ "granularity": DRAW_INTERACTION_GRANULARITY, "id": id })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
    interaction_select_effect_from_targets(targets, merge)
}

pub(crate) const DRAW_MARQUEE_THRESHOLD_PX: f64 = 4.0;
pub(crate) const DRAW_PICK_TOLERANCE_PX: f64 = 8.0;

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

pub(crate) fn draft_preview_segments(utility: &str, points: &UiFixedList<[f64; 2], DRAW_GESTURE_PREVIEW_POINT_CAPACITY>, cursor: [f64; 2]) -> UiFixedList<PathSegment, { DRAW_GESTURE_PREVIEW_POINT_CAPACITY + 2 }> {
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
        ctx.points = UiFixedList::default();
        if ctx.points.try_push(*world).is_err() {
            ctx.points_overflowed = true;
        }
        ctx.cursor = *world;
    }
}

fn gesture_append_draft_point(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut Vec<fsm::Command<draw_gesture::DrawGesture>>) {
    if let Some(draw_gesture::Event::PointerDown { world, .. }) = event {
        if ctx.points.len() < MAX_GESTURE_POINTS {
            if ctx.points.try_push(*world).is_err() {
                ctx.points_overflowed = true;
            }
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
const TRACE_POINTER_WORK_CAPACITY: usize = TRACE_POINTER_MAX_DEPTH * 3 + 4;
const DRAW_QUERY_HIT_CAPACITY: usize = 256;
const DRAW_QUERY_TARGET_BYTES: usize = 8_192;
const MAX_GESTURE_POINTS: usize = 48;
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
    PathBounds { path: TracePath, next: usize, min: [f64; 2], max: [f64; 2], control_hit: bool },
    PolygonBounds { path: TracePath, next: usize, min: [f64; 2], max: [f64; 2] },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TracePickCandidate {
    generality: i32,
    pub(crate) layer_id: String,
    image_key: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    work: UiFixedList<TracePointerWork, TRACE_POINTER_WORK_CAPACITY>,
    pub(crate) best: Option<TracePickCandidate>,
    pub(crate) hits: UiFixedList<String, DRAW_QUERY_HIT_CAPACITY>,
    completed_work: usize,
    pub(crate) overflowed: bool,
}

impl TracePointerJob {
    fn new(generation: u64, document: &DrawSnapshot, world: [f64; 2]) -> Self {
        Self::new_bound(generation, document, world, format!("unbound:{}", document.id))
    }

    fn new_bound(generation: u64, document: &DrawSnapshot, world: [f64; 2], base_revision: String) -> Self {
        let mut work = UiFixedList::default();
        let _ = work.try_push(TracePointerWork::Roots { next: document.layers.len() });
        Self { app_instance_id: 0, document_id: document.id.clone(), operation_id: 0, generation, base_revision, world, tolerance: 0.0, include_control_points: false, marquee: None, work, best: None, hits: UiFixedList::default(), completed_work: 0, overflowed: false }
    }

    fn new_operation(operation: &semio_framework_plugin::AppOperationContext, document: &DrawSnapshot, world: [f64; 2]) -> Self {
        let mut job = Self::new_bound(operation.generation, document, world, operation.canonical_base_revision_hex());
        job.app_instance_id = operation.app_instance_id;
        job.document_id = operation.parent_document_id.clone();
        job.operation_id = operation.operation_id;
        job
    }

    pub(crate) fn new_query(document: &DrawSnapshot, world: [f64; 2], tolerance: f64, include_control_points: bool) -> Self {
        let mut job = Self::new(0, document, world);
        job.tolerance = tolerance.max(0.0);
        job.include_control_points = include_control_points;
        job
    }

    pub(crate) fn new_marquee(document: &DrawSnapshot, start: [f64; 2], end: [f64; 2], crossing: bool) -> Self {
        let mut job = Self::new(0, document, end);
        job.marquee = Some((start, end, crossing));
        job
    }

    fn push_work(&mut self, work: TracePointerWork) {
        if self.work.try_push(work).is_err() {
            self.overflowed = true;
        }
    }

    pub(crate) fn advance(&mut self, document: &DrawSnapshot) -> bool {
        if self.overflowed {
            return true;
        }
        for _ in 0..TRACE_POINTER_WORK_PER_STEP {
            let Some(work) = self.work.pop() else { return true };
            self.completed_work += 1;
            match work {
                TracePointerWork::Roots { next } => {
                    if next == 0 {
                        continue;
                    }
                    self.push_work(TracePointerWork::Roots { next: next - 1 });
                    if let Some(path) = TracePath::root(next - 1) {
                        self.push_work(TracePointerWork::Enter(path));
                    }
                }
                TracePointerWork::Enter(path) => {
                    let Some(layer) = draw_layer_at_path(&document.layers, &path) else { continue };
                    if let DrawLayerNode::Group(group) = layer {
                        self.push_work(TracePointerWork::Visit(path));
                        self.push_work(TracePointerWork::GroupChildren { path, next: group.children.len() });
                    } else {
                        self.push_work(TracePointerWork::Visit(path));
                    }
                }
                TracePointerWork::GroupChildren { path, next } => {
                    if next == 0 {
                        continue;
                    }
                    self.push_work(TracePointerWork::GroupChildren { path, next: next - 1 });
                    if let Some(child_path) = path.child(next - 1) {
                        self.push_work(TracePointerWork::Enter(child_path));
                    }
                }
                TracePointerWork::Visit(path) => {
                    let Some(layer) = draw_layer_at_path(&document.layers, &path) else { continue };
                    match layer {
                        DrawLayerNode::Path(path_layer) if !path_layer.segments.is_empty() => self.push_work(TracePointerWork::PathBounds { path, next: 0, min: [f64::INFINITY; 2], max: [f64::NEG_INFINITY; 2], control_hit: false }),
                        DrawLayerNode::Shape(shape) if shape.shape_kind == "polygon" && shape.polygon.as_ref().is_some_and(|polygon| !polygon.points.is_empty()) => {
                            self.push_work(TracePointerWork::PolygonBounds { path, next: 0, min: [f64::INFINITY; 2], max: [f64::NEG_INFINITY; 2] })
                        }
                        _ => consider_trace_candidate(self, layer, trace_layer_world_bounds(layer), false),
                    }
                }
                TracePointerWork::PathBounds { path, next, mut min, mut max, mut control_hit } => {
                    let Some(DrawLayerNode::Path(path_layer)) = draw_layer_at_path(&document.layers, &path) else { continue };
                    if let Some(segment) = path_layer.segments.get(next) {
                        if let Some(point) = trace_segment_point(segment) {
                            extend_trace_bounds(&mut min, &mut max, point);
                        }
                        if self.include_control_points && trace_segment_control_hit(segment, &path_layer.base.transform, self.world, self.tolerance) {
                            control_hit = true;
                        }
                        self.push_work(TracePointerWork::PathBounds { path, next: next + 1, min, max, control_hit });
                    } else if min[0].is_finite() {
                        consider_trace_candidate(self, draw_layer_at_path(&document.layers, &path).expect("path work retains its layer"), trace_world_bounds(&path_layer.base.transform, min, max), control_hit);
                    }
                }
                TracePointerWork::PolygonBounds { path, next, mut min, mut max } => {
                    let Some(DrawLayerNode::Shape(shape)) = draw_layer_at_path(&document.layers, &path) else { continue };
                    let Some(polygon) = &shape.polygon else { continue };
                    if let Some(point) = polygon.points.get(next) {
                        extend_trace_bounds(&mut min, &mut max, *point);
                        self.push_work(TracePointerWork::PolygonBounds { path, next: next + 1, min, max });
                    } else if min[0].is_finite() {
                        consider_trace_candidate(self, draw_layer_at_path(&document.layers, &path).expect("polygon work retains its layer"), trace_world_bounds(&shape.base.transform, min, max), false);
                    }
                }
            }
        }
        self.work.is_empty()
    }
}

fn consider_trace_candidate(job: &mut TracePointerJob, layer: &DrawLayerNode, bounds: (f64, f64, f64, f64), control_hit: bool) {
    let base = trace_layer_base(layer);
    if !base.visible || base.locked {
        return;
    }
    if let Some((start, end, crossing)) = job.marquee {
        if matches!(layer, DrawLayerNode::Group(_)) {
            return;
        }
        let rect_x = start[0].min(end[0]);
        let rect_y = start[1].min(end[1]);
        let rect_w = (end[0] - start[0]).abs();
        let rect_h = (end[1] - start[1]).abs();
        let intersects = rect_x <= bounds.0 + bounds.2 && rect_x + rect_w >= bounds.0 && rect_y <= bounds.1 + bounds.3 && rect_y + rect_h >= bounds.1;
        let contains = bounds.0 >= rect_x && bounds.1 >= rect_y && bounds.0 + bounds.2 <= rect_x + rect_w && bounds.1 + bounds.3 <= rect_y + rect_h;
        if (if crossing { intersects } else { contains }) && job.hits.try_push(layer_id(layer).to_string()).is_err() {
            job.overflowed = true;
        }
        return;
    }
    if !trace_point_in_bounds(job.world, bounds, job.tolerance) {
        return;
    }
    let candidate = TracePickCandidate {
        generality: if control_hit {
            4
        } else {
            match layer {
            DrawLayerNode::Group(_) => 0,
            DrawLayerNode::Boolean(_) | DrawLayerNode::Trace(_) => 1,
            _ => 2,
            }
        },
        layer_id: layer_id(layer).to_string(),
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

fn trace_segment_control_hit(segment: &PathSegment, transform: &crate::artifacts::draw::DrawTransform, world: [f64; 2], tolerance: f64) -> bool {
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    let matrix = [transform.scale_x * cos, transform.scale_x * sin, -transform.scale_y * sin, transform.scale_y * cos, transform.x, transform.y];
    let points = match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Arc { to, .. } => [Some(*to), None, None],
        PathSegment::Quad { ctrl, to } => [Some(*ctrl), Some(*to), None],
        PathSegment::Cubic { ctrl1, ctrl2, to } => [Some(*ctrl1), Some(*ctrl2), Some(*to)],
        PathSegment::Close => [None, None, None],
    };
    points.into_iter().flatten().any(|point| {
        let point = trace_transform_point(matrix, point);
        let dx = world[0] - point[0];
        let dy = world[1] - point[1];
        dx * dx + dy * dy <= tolerance * tolerance
    })
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

fn trace_point_in_bounds(point: [f64; 2], bounds: (f64, f64, f64, f64), tolerance: f64) -> bool {
    point[0] >= bounds.0 - tolerance && point[0] <= bounds.0 + bounds.2 + tolerance && point[1] >= bounds.1 - tolerance && point[1] <= bounds.1 + bounds.3 + tolerance
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
    pub(crate) trace_pointer: Option<TracePointerJob>,
    pub(crate) point_query: Option<DrawPointQuery>,
    pub(crate) draft_query: Option<DrawDraftQuery>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawGesturePreviewPhase {
    Marquee,
    Shape,
    Draft,
    #[default]
    Idle,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawGesturePreview {
    pub sequence: u64,
    pub phase: DrawGesturePreviewPhase,
    pub context: GestureContext,
}

pub(crate) struct DrawPointQuery {
    pub(crate) command_id: &'static str,
    pub(crate) cursor: TracePointerJob,
    pub(crate) hover: bool,
    pub(crate) merge: String,
    pub(crate) marquee: bool,
    pub(crate) traversal_complete: bool,
    target_cursor: usize,
    targets: String,
}

pub(crate) enum DrawQueryPublication {
    Pending,
    Complete(String),
    Fault,
}

impl DrawPointQuery {
    pub(crate) fn new(command_id: &'static str, cursor: TracePointerJob, hover: bool, merge: String, marquee: bool) -> Self {
        Self { command_id, cursor, hover, merge, marquee, traversal_complete: false, target_cursor: 0, targets: String::with_capacity(DRAW_QUERY_TARGET_BYTES) }
    }

    pub(crate) fn publication_step(&mut self) -> DrawQueryPublication {
        if self.targets.is_empty() {
            self.targets.push('[');
        }
        let id = if self.marquee {
            self.cursor.hits.get(self.target_cursor)
        } else if self.target_cursor == 0 {
            self.cursor.best.as_ref().map(|candidate| &candidate.layer_id)
        } else {
            None
        };
        if let Some(id) = id {
            let Ok(id) = serde_json::to_string(id) else { return DrawQueryPublication::Fault };
            let prefix = if self.target_cursor == 0 { "" } else { "," };
            let item = format!("{prefix}{{\"granularity\":\"{DRAW_INTERACTION_GRANULARITY}\",\"id\":{id}}}");
            if self.targets.len().checked_add(item.len()).is_none_or(|bytes| bytes >= DRAW_QUERY_TARGET_BYTES) {
                return DrawQueryPublication::Fault;
            }
            self.targets.push_str(&item);
            self.target_cursor += 1;
            return DrawQueryPublication::Pending;
        }
        if self.targets.len().checked_add(1).is_none_or(|bytes| bytes > DRAW_QUERY_TARGET_BYTES) {
            return DrawQueryPublication::Fault;
        }
        self.targets.push(']');
        DrawQueryPublication::Complete(std::mem::take(&mut self.targets))
    }
}

pub(crate) struct DrawDraftQuery {
    pub(crate) command_id: &'static str,
    utility: String,
    points: UiFixedList<[f64; 2], DRAW_GESTURE_PREVIEW_POINT_CAPACITY>,
    cursor: usize,
    path_segments: Vec<PathSegment>,
    polygon_points: Vec<[f64; 2]>,
}

impl DrawDraftQuery {
    fn new(command_id: &'static str, utility: String, points: UiFixedList<[f64; 2], DRAW_GESTURE_PREVIEW_POINT_CAPACITY>) -> Self {
        let capacity = points.len().checked_add(1).map_or(DRAW_GESTURE_PREVIEW_POINT_CAPACITY + 1, |value| value.min(DRAW_GESTURE_PREVIEW_POINT_CAPACITY + 1));
        Self { command_id, utility, points, cursor: 0, path_segments: Vec::with_capacity(capacity), polygon_points: Vec::with_capacity(capacity) }
    }

    pub(crate) fn advance(&mut self, document: &DrawSnapshot) -> Option<Emit<DrawMutation, DrawConfigMutation>> {
        if self.points.len() < 2 {
            return Some(Emit::default());
        }
        if let Some(point) = self.points.get(self.cursor).copied() {
            if self.utility == "pen" {
                self.path_segments.push(if self.cursor == 0 { PathSegment::Move { to: point } } else { PathSegment::Line { to: point } });
            } else {
                self.polygon_points.push(point);
            }
            self.cursor += 1;
            return None;
        }
        let layer = if self.utility == "pen" {
            create_draw_path_layer("Path", std::mem::take(&mut self.path_segments))
        } else {
            DrawLayerNode::Shape(crate::artifacts::draw::DrawShapeBody {
                base: crate::artifacts::draw::schema::default_layer_base("Polygon"),
                shape_kind: "polygon".into(),
                rect: None,
                ellipse: None,
                circle: None,
                line: None,
                polygon: Some(crate::artifacts::draw::DrawPolygon { points: std::mem::take(&mut self.polygon_points) }),
            })
        };
        Some(commit_with_utility_reset(vec![crate::artifacts::draw::mutations::create_layer(None, Some(document.layers.len()), layer)], "Commit draft"))
    }
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
        Self { gesture: fsm::init::<draw_gesture::DrawGesture>((), &mut sink), preview_seq: 0, interaction: DrawInteractionSnapshot::default(), trace_pointer: None, point_query: None, draft_query: None }
    }
}

impl DrawSession {
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
        let matches = self
            .trace_pointer
            .as_ref()
            .is_some_and(|job| job.app_instance_id == app_instance_id && job.document_id == document_id && generation != 0 && job.generation == generation);
        if matches {
            self.trace_pointer = None;
        }
        matches
    }

    pub(crate) fn preview(&self) -> DrawGesturePreview {
        let phase = if self.gesture.matches("marqueeing") {
            DrawGesturePreviewPhase::Marquee
        } else if self.gesture.matches("shape_dragging") {
            DrawGesturePreviewPhase::Shape
        } else if self.gesture.matches("drafting") {
            DrawGesturePreviewPhase::Draft
        } else {
            DrawGesturePreviewPhase::Idle
        };
        DrawGesturePreview { sequence: self.preview_seq, phase, context: self.gesture.context.clone() }
    }

    pub(crate) fn from_checkpoint(checkpoint: &DrawGestureCheckpoint) -> Self {
        let mut session = Self::default();
        let event = match checkpoint.state.as_str() {
            "marqueeing" => Some(draw_gesture::Event::PointerDown { utility: if checkpoint.context.method == "lasso" { "selectLasso".into() } else { "selectMarquee".into() }, world: checkpoint.context.start, shift: false, ctrl: false, meta: false }),
            "shape_dragging" => Some(draw_gesture::Event::PointerDown { utility: checkpoint.context.utility.clone(), world: checkpoint.context.start, shift: false, ctrl: false, meta: false }),
            "drafting" => Some(draw_gesture::Event::PointerDown { utility: checkpoint.context.utility.clone(), world: checkpoint.context.points.get(0).copied().unwrap_or(checkpoint.context.start), shift: false, ctrl: false, meta: false }),
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

    pub(crate) fn step_gesture_retained(
        &mut self,
        command_id: &'static str,
        event: draw_gesture::Event,
        document: &DrawSnapshot,
        config: &DrawConfig,
    ) -> Option<Emit<DrawMutation, DrawConfigMutation>> {
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        fsm::macrostep(&mut self.gesture, event, &mut sink, &mut fsm::NullInspector);
        self.preview_seq = self.preview_seq.wrapping_add(1);
        let mut operations = Vec::new();
        let mut commit_description: Option<&'static str> = None;
        for command in sink {
            let fsm::Command::Effect(effect) = command else { continue };
            match effect {
                GestureEffect::CommitMarquee { start, end, active, merge, shift, ctrl, meta } => {
                    if active {
                        self.point_query = Some(DrawPointQuery::new(command_id, TracePointerJob::new_marquee(document, start, end, end[0] < start[0]), false, merge, true));
                    } else {
                        let tolerance = DRAW_PICK_TOLERANCE_PX / config.camera.zoom.max(1e-6);
                        self.point_query = Some(DrawPointQuery::new(command_id, TracePointerJob::new_query(document, end, tolerance, false), false, selection_merge_mode(shift, ctrl, meta).into(), false));
                    }
                    return None;
                }
                GestureEffect::PickPoint { world, shift, ctrl, meta } => {
                    let tolerance = DRAW_PICK_TOLERANCE_PX / config.camera.zoom.max(1e-6);
                    self.point_query = Some(DrawPointQuery::new(command_id, TracePointerJob::new_query(document, world, tolerance, true), false, selection_merge_mode(shift, ctrl, meta).into(), false));
                    return None;
                }
                GestureEffect::CommitShape { utility, start, end } => {
                    operations.extend(commit_shape_drag(document, &utility, start, end));
                    commit_description = Some("Add shape");
                }
                GestureEffect::CommitDraft { utility, points } => {
                    self.draft_query = Some(DrawDraftQuery::new(command_id, utility, points));
                    return None;
                }
                GestureEffect::CommitTrace { .. } => return Some(Emit::default()),
            }
        }
        Some(match commit_description {
            Some(description) => commit_with_utility_reset(operations, description),
            None => Emit::default(),
        })
    }

    /// 🎭️ Feeds one gesture event through the shared `fsm` statechart, then drains and executes any
    /// requested `GestureEffect`s against the live document — the only place gesture control-flow
    /// (owned by `fsm`) meets document-mutating logic (owned by `draw`). `config` is read-only (camera
    /// zoom for hit-test tolerance); a pick/marquee hit becomes an `interactionSelect` request riding
    /// as a `Effect` on the returned `Emit` — selection itself is framework-owned now, never
    /// written back into `config`.
    pub(crate) fn step_gesture(&mut self, event: draw_gesture::Event, document: &DrawSnapshot, _config: &DrawConfig) -> Emit<DrawMutation, DrawConfigMutation> {
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        fsm::macrostep(&mut self.gesture, event, &mut sink, &mut fsm::NullInspector);
        self.preview_seq = self.preview_seq.wrapping_add(1);
        let mut operations = Vec::new();
        let mut commit_description: Option<&'static str> = None;
        for command in sink {
            let fsm::Command::Effect(effect) = command else { continue };
            match effect {
                GestureEffect::CommitShape { utility, start, end } => {
                    operations.extend(commit_shape_drag(document, &utility, start, end));
                    commit_description = Some("Add shape");
                }
                GestureEffect::CommitMarquee { .. } | GestureEffect::CommitDraft { .. } | GestureEffect::CommitTrace { .. } | GestureEffect::PickPoint { .. } => {}
            }
        }
        match commit_description {
            Some(description) => commit_with_utility_reset(operations, description),
            None => Emit::default(),
        }
    }

}
//#endregion 🔖️DrawSession

use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧵️TracePointerContinuation
fn trace_progress(job: &TracePointerJob) -> DrawConfigMutation {
    DrawConfigMutation::SetTracePointerProgress { generation: job.generation, completed_work: job.completed_work as u64, pending_work: job.work.len() as u64 }
}

fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> Option<Effect> {
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
    let args = serde_json::to_value(continuation).ok().and_then(|value| semio_framework::optional_json_to_dsl(Some(value)));
    Some(Effect::DispatchAction { req: RequestId(NEXT_TRACE_POINTER_REQUEST.fetch_add(1, Ordering::Relaxed)), action: "canvasPointerDown".into(), args, delay_ms: 0 })
}

fn advance_trace_pointer(session: &mut DrawSession, mut job: TracePointerJob, payload: &CanvasPointerDown, document: &DrawSnapshot) -> Emit<DrawMutation, DrawConfigMutation> {
    if !job.advance(document) {
        let Some(effect) = queue_trace_pointer(payload, &job) else { return Emit::default() };
        let progress = trace_progress(&job);
        let _ = session.retain_trace_pointer(job);
        return Emit { config_mutations: vec![progress], effects: vec![effect], ..Default::default() };
    }
    let source_key = job.best.and_then(|candidate| candidate.image_key).or_else(|| document.assets.keys().next().cloned());
    let mut emit = commit_with_utility_reset(commit_trace_source(document, source_key), "Trace image");
    emit.config_mutations.push(DrawConfigMutation::SetTracePointerProgress { generation: 0, completed_work: 0, pending_work: 0 });
    emit
}
//#endregion 🧵️TracePointerContinuation

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
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

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
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
        let job = session.take_trace_pointer(operation.app_instance_id, &operation.parent_document_id, operation_id, generation, base_revision);
        let Some(job) = job else { return Ok(Emit::default()) };
        if payload.checkpoint_completed_work != Some(job.completed_work as u64) || payload.checkpoint_pending_work != Some(job.work.len() as u64) {
            let _ = session.retain_trace_pointer(job);
            return Ok(Emit::default());
        }
        return Ok(advance_trace_pointer(session, job, payload, document));
    }
    let config = cfg.snapshot;
    let (world_x, world_y) = canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
    let active_utility = config.active_utility_id.clone();
    if active_utility == "trace" {
        session.cancel_trace_pointer(operation.app_instance_id, &operation.parent_document_id, config.trace_pointer_generation);
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        fsm::macrostep(&mut session.gesture, draw_gesture::Event::PointerDown { utility: "trace".into(), world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta }, &mut sink, &mut fsm::NullInspector);
        session.preview_seq = session.preview_seq.wrapping_add(1);
        return Ok(advance_trace_pointer(session, TracePointerJob::new_operation(operation, document, [world_x, world_y]), payload, document));
    }
    let emit = session.step_gesture(draw_gesture::Event::PointerDown { utility: active_utility, world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta }, document, config);
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn stale_generation_and_wrong_owner_cannot_take_retained_trace() {
        let document = crate::artifacts::draw::schema::default_draw_document("fresh-trace", None);
        let mut session = DrawSession::default();
        assert!(session.retain_trace_pointer(TracePointerJob::new(11, &document, [0.0, 0.0])).is_ok());
        let base = format!("unbound:{}", document.id);
        assert!(session.take_trace_pointer(0, &document.id, 0, 12, &base).is_none());
        assert!(session.take_trace_pointer(1, &document.id, 0, 11, &base).is_none());
        assert!(session.take_trace_pointer(0, "foreign", 0, 11, &base).is_none());
        assert!(session.take_trace_pointer(0, &document.id, 0, 11, &base).is_some());
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
    fn retained_trace_interruption_cancel_and_repeated_cancel_are_exact() {
        let document = crate::artifacts::draw::schema::default_draw_document("retained", None);
        let mut session = DrawSession::default();
        assert!(session.retain_trace_pointer(TracePointerJob::new(91, &document, [4.0, 5.0])).is_ok());
        assert!(!session.cancel_trace_pointer(0, &document.id, 90));
        assert!(session.cancel_trace_pointer(0, &document.id, 91));
        assert!(!session.cancel_trace_pointer(0, &document.id, 91));
    }

    #[test]
    fn marquee_maximum_plus_one_faults_without_unbounded_growth() {
        let mut document = crate::artifacts::draw::schema::default_draw_document("marquee-max", None);
        document.layers = (0..=DRAW_QUERY_HIT_CAPACITY)
            .map(|index| create_draw_path_layer(format!("hit-{index}"), vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }]))
            .collect();
        let mut query = TracePointerJob::new_marquee(&document, [-128.0, -128.0], [128.0, 128.0], true);
        let mut turns = 0;
        while !query.advance(&document) {
            turns += 1;
            assert!(turns < 10_000);
        }
        assert!(turns > 1, "the adversarial tree must yield across turns");
        assert!(query.overflowed, "maximum plus one must fault rather than publish a partial selection");
        assert_eq!(query.hits.len(), DRAW_QUERY_HIT_CAPACITY);
    }

    #[test]
    fn draft_cursor_advances_one_point_per_turn_and_hands_back_one_commit() {
        let document = crate::artifacts::draw::schema::default_draw_document("draft-cursor", None);
        let mut points = UiFixedList::default();
        for index in 0..DRAW_GESTURE_PREVIEW_POINT_CAPACITY {
            assert!(points.try_push([index as f64, index as f64]).is_ok());
        }
        let mut query = DrawDraftQuery::new("canvasCommitDraft", "pen".into(), points);
        assert!(query.advance(&document).is_none());
        assert_eq!(query.cursor, 1);
        for _ in 1..DRAW_GESTURE_PREVIEW_POINT_CAPACITY {
            assert!(query.advance(&document).is_none());
        }
        let emit = query.advance(&document).expect("the bounded final turn hands back one commit");
        assert_eq!(emit.artifact_mutations.len(), 1);
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
