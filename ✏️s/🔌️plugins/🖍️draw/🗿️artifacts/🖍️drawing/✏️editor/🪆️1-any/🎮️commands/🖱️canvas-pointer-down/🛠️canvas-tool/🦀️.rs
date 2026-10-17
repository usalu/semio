//! 🛠️ Drawing canvas tool: the pointer records, the guards and actions of the canvas statechart, the statechart itself and the `DrawingTool` runner.

use super::*;

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

#[derive(Clone,Debug)]
pub struct DrawingDraftCommit {pub base:DrawingToolBase,pub identity:Option<semio_s_artifact_draw_drawing_model::identity::DrawingIdentity>}

/// ⬆️ One release: the pointer, the drag threshold in world units, and the base a commit yields against.
#[derive(Clone, Debug)]
pub struct DrawingRelease {
    pub identity:Option<semio_s_artifact_draw_drawing_model::identity::DrawingIdentity>,
    pub pointer: DrawingPointer,
    pub threshold: f64,
    pub base: DrawingToolBase,
}

/// 🖼️ A finished trace: the base and the image source it traces, if the pointer found one.
#[derive(Clone, Debug)]
pub struct DrawingTraced {
    pub identity:Option<semio_s_artifact_draw_drawing_model::identity::DrawingIdentity>,
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

pub(super) fn grab(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, _sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
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
pub(super) fn drag(ctx: &mut DrawingToolContext, event: Option<&canvas_tool::Event>, sink: &mut Vec<Command<canvas_tool::CanvasTool>>) {
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
    if let Some(layer) = release.identity.as_ref().and_then(|identity|shape_drag_layer(&release.base.document,&ctx.utility,ctx.start,release.pointer.world,identity.clone())) {
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
    let Some(canvas_tool::Event::CommitDraft(commit)) = event else { return };
    if let Some(layer) = commit.identity.as_ref().and_then(|identity|draft_layer(&commit.base.document,&ctx.utility,&ctx.points,identity.clone())) {
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
    if let Some(layer) = traced.identity.as_ref().and_then(|identity|trace_layer(&traced.base.document,traced.source_key.as_deref(),identity.clone())) {
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
            CommitDraft(DrawingDraftCommit),
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
        let runner = ToolMachineRunner::start(format!("{DRAWING_EDITOR_APP_ID}#{tool}"), protocol::ActorId(authoring_seed.into()), (), DrawingToolHost).expect("the canvas tool enters its initial configuration without yielding");
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
