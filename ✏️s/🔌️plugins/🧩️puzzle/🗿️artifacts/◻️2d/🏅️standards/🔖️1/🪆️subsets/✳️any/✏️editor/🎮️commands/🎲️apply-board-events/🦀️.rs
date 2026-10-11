//! 🎲️ `apply-board-events` command.

#![allow(unexpected_cfgs)]

use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::PUZZLE2D_EDITOR_APP_ID;
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::panels::{artifact, inspection};
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::recorder::Puzzle2dRecorder;
use crate::editor::puzzle2d::{puzzle2d_selection_write, Puzzle2dActionCtx, Puzzle2dSelectionRecord};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;

use machine::Command;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_tool_machine::{GestureChart, GesturePhase, ToolYield};
use semio_framework_pack_json::{json,Value,Object};
use std::sync::{Arc, OnceLock};

/// 🐢️ Classifies a batch of board events into the narrowest `UiDirtyScope` that covers all of them —
/// `applyBoardEvents` fires on every select and drag (a camera move is the view verb `setCamera`), so getting this right is most of the
/// perf-round-3 win. Unrecognized/empty event batches fall back to `Full` (safe default).
fn puzzle2d_board_events_scope(events: &[Value]) -> UiDirtyScope {
    if events.is_empty() {
        return UiDirtyScope::None;
    }
    let panes: Vec<String> = PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect();
    let mut window_bodies = false;
    let mut panel_layers = false;
    let mut panel_properties = false;
    let mut engagements = false;
    let mut measures = false;
    let mut recognized_all = true;
    for event in events {
        let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
            recognized_all = false;
            continue;
        };
        match name {
            "select" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
                engagements = true;
            }
            // 🎬️ A finished drag or rotate arrives as ONE record: positions (and handle angles) change and a drop
            // may land new edges, so the panes, the inspector and the layers tree repaint.
            "gesture" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
            }
            // 🎯️ A painted region adds an outliner row; a moved or resized one only changes geometry,
            // so it repaints the panes and the inspector without churning the layers tree.
            "regionCreate" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
            }
            "regionResize" => {
                window_bodies = true;
                panel_properties = true;
            }
            "brushPlace" | "edgeCreate" | "edgeDelete" | "nodeDelete" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
                engagements = true;
                measures = true;
            }
            "brushCandidates" => {
                window_bodies = true;
                engagements = true;
            }
            // 🖱️ Engine-local hover paints itself; the guest keeps no hover state to re-render for.
            "hover" | "preselectCancel" => {}
            _ => recognized_all = false,
        }
    }
    if !recognized_all {
        return UiDirtyScope::Full;
    }
    let mut panel_bodies = Vec::new();
    if panel_layers {
        panel_bodies.push(artifact::PUZZLE2D_PLAY_BODY_LAYERS.to_string());
    }
    if panel_properties {
        panel_bodies.push(inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string());
    }
    UiDirtyScope::Partial { window_bodies: if window_bodies { panes } else { Vec::new() }, panel_bodies, utilities: false, tools: false, engagements, measures, labels: false }
}

//#region 🛠️BoardTool
/// 🧰️ The board gestures whose press and stream stay engine-local — the board paints its own rubber band, ghost or wire
/// and reports only the release, as rows of ONE kind: an area painted (`regionCreate`, already grid-snapped by the
/// engine), a region grip dragged (`regionResize`, an absolute pose a locked region refuses), a brush stamped
/// (`brushPlace`), a wire dropped on a handle (`edgeCreate`), a wire cut (`edgeDelete`) and a delete of whatever the id
/// names (`nodeDelete`: a node, a handle, an edge or a target region). Each is a one-step tool: its release yields the
/// document leaves inside ONE `ToolTransaction` of `<appId>#<kind>`; a gesture the board cancels sends no row, and a
/// release that changes nothing leaves zero trace.
pub const PUZZLE2D_BOARD_TOOLS: [&str; 6] = ["regionCreate", "regionResize", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"];

/// 📦️ What a release does: the document after it and the leaves that take its base there.
#[derive(Debug)]
pub struct BoardToolOutcome {
    pub after: Arc<crate::Puzzle2dSnapshot>,
    pub yields: Vec<(String, Puzzle2dMutation)>,
}

/// 🧾️ One release as a tool event: the document it folds on and the row payloads of its ONE kind, in board order. What
/// it yields is derived once, however often the chart asks.
#[derive(Clone, Debug)]
pub struct BoardToolRelease {
    base: Arc<crate::Puzzle2dSnapshot>,
    kind: &'static str,
    rows: Arc<[Value]>,
    outcome: Arc<OnceLock<BoardToolOutcome>>,
}

impl BoardToolRelease {
    /// 🎬️ The release of `kind` whose row payloads are `rows`, on `base`.
    pub fn new(base: Arc<crate::Puzzle2dSnapshot>, kind: &'static str, rows: Vec<Value>) -> Self {
        Self { base, kind, rows: rows.into(), outcome: Arc::new(OnceLock::new()) }
    }

    /// 🧮️ The release recorded on its base: every row through [`Puzzle2dRecorder::fold_board_row`], which states it as
    /// the concrete kinds it consists of, keyed `<kind>:<index>` in recording order. A row the document refuses
    /// yields nothing, and says nothing; a release that changes nothing yields no kind.
    pub fn outcome(&self) -> &BoardToolOutcome {
        self.outcome.get_or_init(|| {
            let mut recorder = Puzzle2dRecorder::from_typed(Arc::clone(&self.base));
            for payload in self.rows.iter() {
                recorder.fold_board_row(self.kind, payload);
            }
            let (after, leaves) = recorder.into_parts();
            BoardToolOutcome { after, yields: leaves.into_iter().enumerate().map(|(index, leaf)| (format!("{}:{index}", self.kind), leaf)).collect() }
        })
    }
}

/// 🪶️ The board tool keeps nothing between dispatches: its gesture lives in the board engine until the release.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoardToolContext;

fn board_tool_context(input: BoardToolContext) -> BoardToolContext {
    input
}

fn release_applies(_context: &BoardToolContext, event: Option<&board_tool::Event>) -> bool {
    matches!(event, Some(board_tool::Event::Release(release)) if !release.outcome().yields.is_empty())
}

fn yield_release(_context: &mut BoardToolContext, event: Option<&board_tool::Event>, sink: &mut Vec<Command<board_tool::BoardTool>>) {
    let Some(board_tool::Event::Release(release)) = event else { return };
    sink.extend(release.outcome().yields.iter().cloned().map(|(key, mutation)| Command::Effect(ToolYield::upsert(key, mutation))));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine board_tool {
        context: BoardToolContext;
        event Event { Release(BoardToolRelease) }
        input: BoardToolContext;
        output: ();
        effect: ToolYield<Puzzle2dMutation>;
        context_from_input: board_tool_context;
        initial: idle;
        state idle {
            on Release if release_applies => idle do yield_release;
        }
    }
}

/// 🧷️ The board tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct BoardToolHost;

impl machine::Host<board_tool::BoardTool> for BoardToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Puzzle2dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🧭️ The board tool on the framework gesture runner: a release is a one-shot of the window's gesture slot — it ends
/// whatever gesture the window held and never holds one itself, so there is nothing to restore, and it folds on any
/// base.
impl GestureChart for board_tool::BoardTool {
    type Tick = BoardToolRelease;
    type Host = BoardToolHost;

    const BASE_BOUND: bool = false;

    fn tool(verb: &str) -> String {
        format!("{PUZZLE2D_EDITOR_APP_ID}#{verb}")
    }

    fn host() -> BoardToolHost {
        BoardToolHost
    }

    fn input() -> BoardToolContext {
        BoardToolContext
    }

    fn restore(_entries: &[(String, Puzzle2dMutation)], _context: &semio_framework_value::DslValue) -> Option<BoardToolContext> {
        None
    }

    fn event(phase: GesturePhase, _at_rest: bool, tick: Option<BoardToolRelease>) -> Option<board_tool::Event> {
        tick.filter(|_| phase == GesturePhase::Once).map(board_tool::Event::Release)
    }
}
//#endregion 🛠️BoardTool

/// 🎲️ Replays one engine event batch a guest-side host owns: a `gesture` row is a parametric input the select tool yields as
/// ONE transaction and a guest-side host never receives a pointer, so it is skipped; a `brushCandidates` row is
/// window-transient runtime state; a board-tool row is recorded as the concrete kinds it consists of
/// ([`Puzzle2dRecorder::fold_board_row`]); selection is framework-owned (see [`board_selection_write`]).
pub fn replay_board_events(events_json: &str, recorder: &mut Puzzle2dRecorder, runtime: &mut Puzzle2dPlayRuntime) {
    let Ok(events) = semio_framework_pack_json::from_json_str::<Vec<Value>>(events_json,semio_framework_pack_json::JsonMemberPolicy::Reject) else {
        return;
    };
    for event in events {
        let Some(name) = event.get("name").and_then(Value::as_str) else {
            continue;
        };
        let payload = event.get("payload").cloned().unwrap_or(Value::Null);
        match name {
            "gesture" => {}
            "brushCandidates" => fold_brush_candidates(runtime, &payload),
            name => {
                recorder.fold_board_row(name, &payload);
            }
        }
    }
}

/// 🖌️ The brush's candidate page a `brushCandidates` row carries — window-transient runtime state, never the document.
fn fold_brush_candidates(runtime: &mut Puzzle2dPlayRuntime, payload: &Value) {
    if let Some(candidates) = payload.get("candidates").and_then(Value::as_array) {
        runtime.brush_candidates = candidates.iter().map(semio_framework_pack_json::to_dsl_value).collect();
    }
    if let Some(source) = payload.get("sourceHandleId").and_then(Value::as_str) {
        runtime.brush_candidate_source_handle_id = source.to_string();
    }
    if let Some(index) = payload.get("index").and_then(Value::as_u64) {
        runtime.brush_candidate_index = index as usize;
    }
}

/// 🐢️ `UiDirtyScope.windowBodies`/`.panelBodies` are matched against `AppDefinition.windowKinds[].bodyKey`
/// on the shell side (`buildUiRefreshRequest`'s `uiRefreshWantsWindow`), so these must be the body-key
/// constants (`puzzle2d.play.overview`, …) — *not* the pane/kind-id constants (`PUZZLE2D_PANES`,
/// `2d-overview`, …), which are a different id space used to key utilities/engagements/measures.
pub const PUZZLE2D_WINDOW_BODY_KEYS: [&str; 3] = [overview::BODY_KEY, detail::BODY_KEY, selection::BODY_KEY];

/// 🕹️ The LAST `select` row of a batch is the engine's whole selection set (`ids`, replace semantics),
/// so it becomes one framework selection write — later rows supersede earlier ones inside a batch.
fn board_selection_write(events: &[Value], snapshot: &Value) -> Option<semio_framework_plugin::InteractionWrite> {
    let ids: Vec<String> = events
        .iter()
        .rev()
        .find(|event| event.get("name").and_then(Value::as_str) == Some("select"))?
        .get("payload")
        .and_then(|payload| payload.get("ids"))
        .and_then(Value::as_array)
        .map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default();
    Some(puzzle2d_selection_write(snapshot, &ids))
}

/// 🎰️ One board flush. Its last `select` row is the framework selection write; its `gesture` records commit through
/// the select tool; its board-tool rows commit through the board tool, one release per run of one kind — every document
/// change of a flush is a tool transaction, never a scene delta. A record whose every target is locked is refused at
/// the tool with one sentence; the selection write of the same batch still lands. A flush of transient rows alone (a
/// hover, a selection, a candidate page) yields no operation at all.
pub fn apply_board_events(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) else {
        return;
    };
    let events = semio_framework_pack_json::from_json_str::<Vec<Value>>(events_json,semio_framework_pack_json::JsonMemberPolicy::Reject).ok();
    let scope = events.as_deref().map_or(UiDirtyScope::Full, puzzle2d_board_events_scope);
    *ctx.ui_scope = scope.clone();
    let absent = Value::Null;
    let mut records = Vec::new();
    let mut rows = Vec::new();
    for event in events.as_deref().unwrap_or_default() {
        let payload = event.get("payload").unwrap_or(&absent);
        match event.get("name").and_then(Value::as_str) {
            Some("gesture") => records.extend(Puzzle2dSelectionRecord::from_gesture(payload)),
            Some("brushCandidates") => fold_brush_candidates(&mut ctx.scene.runtime, payload),
            Some(name) => rows.extend(PUZZLE2D_BOARD_TOOLS.iter().find(|kind| **kind == name).map(|kind| (*kind, payload.clone()))),
            None => {}
        }
    }
    let released = ctx.release_board(&rows);
    let released_view=released.as_ref().map(|snapshot|semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot.as_ref())));
    if let Some(write) = events.as_deref().and_then(|events| board_selection_write(events, released_view.as_ref().unwrap_or(&ctx.scene.board_snapshot))) {
        ctx.interaction_writes.push(write);
    }
    if !records.is_empty() || ctx.gesture.open().is_some() {
        ctx.commit_selection(overview::utilities::select::UTILITY_ID, records);
    }
    if !ctx.interaction_writes.is_empty() && matches!(*ctx.ui_scope, UiDirtyScope::None) {
        *ctx.ui_scope = scope;
    }
}
