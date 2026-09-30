//! 🖱️ Overview-window utility — Select: the default pointer utility (rectangle/lasso marquee plus
//! click picking). Together with `🖌️brush` it forms this window's entire exclusive utility set, so it
//! carries `group: None` and renders as its own flat utility-bar icon, never a collapsed dropdown.
//!
//! 🛠️ It is also the select TOOL: a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the
//! `🛠️tool-machine` runner. Every selection transform — a board gesture record, `translateSelection`, the HUD
//! `move`/`rotate`/`scale`, a keyboard nudge, an inspector `delta` — enters as a [`Puzzle2dSelectionRecord`]
//! and leaves as ONE `ToolTransaction`: the parametric `drag-`/`rotate-`/`scale-selection` leaf plus the
//! `connect-handles` its drop lands, targets and edge ids literal. Tool state is never history; the yielded
//! mutations are (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §8).
#![allow(unexpected_cfgs)]

use crate::editor::puzzle2d::commands::proximity_connect::puzzle2d_proximity_pairs;
use crate::editor::puzzle2d::{fixture_nodes, puzzle2d_occupied_handles, puzzle2d_push_edge, PUZZLE2D_PROXIMITY_GESTURE_MAX};
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, connect_handles, drag_selection, rotate_selection, scale_selection, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;
use machine::Command;
use semio_framework_plugin::{LocalizedLabel, UtilityCategory, UtilityDefinition};
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};
use serde_json::{json, Value};
use std::sync::Arc;

pub const UTILITY_ID: &str = "select";

/// 🪪️ The editor app id every select-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const PUZZLE2D_EDITOR_APP_ID: &str = "s.puzzle.puzzle2d@1/*#editor";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new(UTILITY_ID, label, "mouse-pointer") }
}

//#region 🎬️Record
/// 🎬️ How one selection transform moves its targets — the parameters of the leaf it yields.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle2dSelectionMotion {
    Drag { dx: f64, dy: f64 },
    Rotate { pivot_x: f64, pivot_y: f64, angle: f64 },
    Scale { pivot_x: f64, pivot_y: f64, factor: f64 },
}

/// 🎬️ One selection transform the select tool yields: the literal target ids, the motion, the handle pairs the
/// board recorded at the release, and whether the drop runs the proximity search on the moved state.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle2dSelectionRecord {
    pub targets: Vec<String>,
    pub motion: Puzzle2dSelectionMotion,
    pub proximity: Vec<(String, String)>,
    pub connect: bool,
}

impl Puzzle2dSelectionRecord {
    /// 🧩️ Decodes one board `gesture` row payload (`🖥️Board2dHost/🧬️schema/🔣️board-event-coalescing`), its targets
    /// deduplicated in first-seen order; `None` for a malformed, non-finite, target-less or non-positive-factor
    /// record. A drag drops, so it runs the proximity search too.
    pub fn from_gesture(payload: &Value) -> Option<Self> {
        let number = |key: &str| payload.get(key).and_then(Value::as_f64).filter(|value| value.is_finite());
        let targets = puzzle2d_unique_targets(payload.get("targets")?.as_array()?.iter().filter_map(Value::as_str).map(str::to_string));
        let motion = match payload.get("kind")?.as_str()? {
            "drag" => Puzzle2dSelectionMotion::Drag { dx: number("dx")?, dy: number("dy")? },
            "rotate" => Puzzle2dSelectionMotion::Rotate { pivot_x: number("pivotX")?, pivot_y: number("pivotY")?, angle: number("angle")? },
            "scale" => Puzzle2dSelectionMotion::Scale { pivot_x: number("pivotX")?, pivot_y: number("pivotY")?, factor: number("factor").filter(|factor| *factor > 0.0)? },
            _ => return None,
        };
        let proximity = payload.get("proximity").and_then(Value::as_array).into_iter().flatten().filter_map(|pair| Some((pair.get("source")?.as_str()?.to_string(), pair.get("target")?.as_str()?.to_string()))).collect();
        (!targets.is_empty()).then(|| Self { connect: matches!(motion, Puzzle2dSelectionMotion::Drag { .. }), targets, motion, proximity })
    }

    /// 🧮️ The parametric leaf this record yields, over its targets deduplicated in first-seen order — a leaf's
    /// target set is never empty (see [`Self::applies_to`]) and never repeats an id.
    pub fn mutation(&self) -> Puzzle2dMutation {
        let targets = puzzle2d_unique_targets(self.targets.iter().cloned());
        match self.motion {
            Puzzle2dSelectionMotion::Drag { dx, dy } => drag_selection(targets, dx, dy),
            Puzzle2dSelectionMotion::Rotate { pivot_x, pivot_y, angle } => rotate_selection(targets, pivot_x, pivot_y, angle),
            Puzzle2dSelectionMotion::Scale { pivot_x, pivot_y, factor } => scale_selection(targets, pivot_x, pivot_y, factor),
        }
    }

    /// 🎚️ Whether the motion is one a leaf admits and that moves: finite numbers, a factor above zero, not the
    /// identity.
    fn moves(&self) -> bool {
        match self.motion {
            Puzzle2dSelectionMotion::Drag { dx, dy } => dx.is_finite() && dy.is_finite() && (dx, dy) != (0.0, 0.0),
            Puzzle2dSelectionMotion::Rotate { pivot_x, pivot_y, angle } => pivot_x.is_finite() && pivot_y.is_finite() && angle.is_finite() && angle != 0.0,
            Puzzle2dSelectionMotion::Scale { pivot_x, pivot_y, factor } => pivot_x.is_finite() && pivot_y.is_finite() && factor.is_finite() && factor > 0.0 && factor != 1.0,
        }
    }

    /// 🔎️ Whether this record moves anything on `base`: an admissible non-identity motion and at least one target
    /// that is an unlocked record it applies to — a rotation turns nodes only, an axis-aligned target region never.
    pub fn applies_to(&self, base: &Puzzle2dSnapshot) -> bool {
        self.moves() && self.targets.iter().any(|id| self.movable(base, id))
    }

    /// 🚚️ Whether `id` names an unlocked record of `base` this motion moves.
    fn movable(&self, base: &Puzzle2dSnapshot, id: &str) -> bool {
        let regions = !matches!(self.motion, Puzzle2dSelectionMotion::Rotate { .. });
        base.nodes.iter().any(|node| node.id == id && node.locked != Some(true)) || (regions && base.target_regions.iter().any(|region| region.id == id && !region.locked))
    }

    /// 🧬️ The record a parametric leaf states — how a resumed stream recovers its accumulated transform.
    pub fn from_leaf(leaf: &Puzzle2dMutation, connect: bool) -> Option<Self> {
        let (targets, motion) = match leaf {
            Puzzle2dMutation::DragSelection(leaf) => (leaf.targets.clone(), Puzzle2dSelectionMotion::Drag { dx: leaf.dx, dy: leaf.dy }),
            Puzzle2dMutation::RotateSelection(leaf) => (leaf.targets.clone(), Puzzle2dSelectionMotion::Rotate { pivot_x: leaf.pivot_x, pivot_y: leaf.pivot_y, angle: leaf.angle }),
            Puzzle2dMutation::ScaleSelection(leaf) => (leaf.targets.clone(), Puzzle2dSelectionMotion::Scale { pivot_x: leaf.pivot_x, pivot_y: leaf.pivot_y, factor: leaf.factor }),
            _ => return None,
        };
        Some(Self { targets, motion, proximity: Vec::new(), connect })
    }

    /// ➕️ This transform followed by `tick` as ONE net transform: offsets add, angles add, factors multiply — only
    /// for the same targets, the same kind and (for a rotation or scaling) the same pivot; `None` otherwise.
    pub fn then(&self, tick: &Self) -> Option<Self> {
        if self.targets != tick.targets {
            return None;
        }
        let motion = match (self.motion, tick.motion) {
            (Puzzle2dSelectionMotion::Drag { dx, dy }, Puzzle2dSelectionMotion::Drag { dx: next_dx, dy: next_dy }) => Puzzle2dSelectionMotion::Drag { dx: dx + next_dx, dy: dy + next_dy },
            (Puzzle2dSelectionMotion::Rotate { pivot_x, pivot_y, angle }, Puzzle2dSelectionMotion::Rotate { pivot_x: next_x, pivot_y: next_y, angle: next }) if (pivot_x, pivot_y) == (next_x, next_y) => Puzzle2dSelectionMotion::Rotate { pivot_x, pivot_y, angle: angle + next },
            (Puzzle2dSelectionMotion::Scale { pivot_x, pivot_y, factor }, Puzzle2dSelectionMotion::Scale { pivot_x: next_x, pivot_y: next_y, factor: next }) if (pivot_x, pivot_y) == (next_x, next_y) => Puzzle2dSelectionMotion::Scale { pivot_x, pivot_y, factor: factor * next },
            _ => return None,
        };
        Some(Self { targets: self.targets.clone(), motion, proximity: self.proximity.iter().chain(&tick.proximity).cloned().collect(), connect: self.connect || tick.connect })
    }

    /// 🎯️ A drag of `targets` (deduplicated in first-seen order) by `(dx, dy)` that drops — the record a board
    /// drag, `translateSelection`, the HUD `move` and a keyboard nudge all yield.
    pub fn drag(targets: Vec<String>, dx: f64, dy: f64) -> Self {
        Self { targets: puzzle2d_unique_targets(targets), motion: Puzzle2dSelectionMotion::Drag { dx, dy }, proximity: Vec::new(), connect: true }
    }

    /// 🔒️ The tool-level refusal: the motion moves, yet nothing this record names can, and a lock is why. A record
    /// with at least one movable target is yielded instead, and its leaf reports the locked rest as `mutation.partial`.
    pub fn refused_as_locked(&self, base: &Puzzle2dSnapshot) -> bool {
        self.moves() && !self.applies_to(base) && self.targets.iter().any(|id| base.nodes.iter().any(|node| &node.id == id && node.locked == Some(true)) || base.target_regions.iter().any(|region| &region.id == id && region.locked))
    }
}

/// 🧹️ `targets` without repeats, in first-seen order — the one target list every leaf the select tool yields
/// carries.
pub fn puzzle2d_unique_targets(targets: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    targets.into_iter().filter(|id| seen.insert(id.clone())).collect()
}
/// 📍️ The pivot a command-driven rotate or scale records: the centroid of the targets' node positions, and with
/// `regions` also of their target-region centres — one recorded point, so the leaf replays on any base.
pub fn puzzle2d_selection_pivot(document: &Puzzle2dSnapshot, targets: &[String], regions: bool) -> Option<(f64, f64)> {
    let nodes = document.nodes.iter().filter(|node| targets.contains(&node.id)).map(|node| (node.x, node.y));
    let centres = document.target_regions.iter().filter(|region| regions && targets.contains(&region.id)).map(|region| {
        let [min_x, min_y, max_x, max_y] = region.bounds();
        ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0)
    });
    let points: Vec<(f64, f64)> = nodes.chain(centres).collect();
    (!points.is_empty()).then(|| (points.iter().map(|point| point.0).sum::<f64>() / points.len() as f64, points.iter().map(|point| point.1).sum::<f64>() / points.len() as f64))
}
//#endregion 🎬️Record

//#region 🛠️SelectTool
/// 📨️ What one select-tool event carries: the committed document it yields against, the window's proximity radius
/// and the records — dispatch inputs, never tool state.
#[derive(Clone, Debug)]
pub struct SelectToolRequest {
    pub base: Arc<Puzzle2dSnapshot>,
    pub proximity_radius: f64,
    pub records: Vec<Puzzle2dSelectionRecord>,
}

/// 🧰️ The select tool's context: the one transform a streamed gesture has accumulated so far (`None` at rest) —
/// tool state, never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectToolContext {
    pub stream: Option<Puzzle2dSelectionRecord>,
}

fn select_tool_context(input: SelectToolContext) -> SelectToolContext {
    input
}

fn records_apply(_context: &SelectToolContext, event: Option<&select_tool::Event>) -> bool {
    matches!(event, Some(select_tool::Event::Records(request)) if request.records.iter().any(|record| record.applies_to(&request.base)))
}

fn yield_records(_context: &mut SelectToolContext, event: Option<&select_tool::Event>, sink: &mut Vec<Command<select_tool::SelectTool>>) {
    let Some(select_tool::Event::Records(request)) = event else { return };
    sink.extend(puzzle2d_selection_yields(&request.base, &request.records, request.proximity_radius).into_iter().map(|(key, mutation)| Command::Effect(ToolYield::upsert(key, mutation))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn stream_applies(_context: &SelectToolContext, event: Option<&select_tool::Event>) -> bool {
    matches!(event, Some(select_tool::Event::Stream(request)) if request.records.first().is_some_and(|record| record.applies_to(&request.base)))
}

fn begin_stream(context: &mut SelectToolContext, event: Option<&select_tool::Event>, sink: &mut Vec<Command<select_tool::SelectTool>>) {
    let Some(select_tool::Event::Stream(request)) = event else { return };
    let mut records = request.records.iter();
    context.stream = records.next().cloned();
    continue_with(context, records, sink);
}

fn continue_stream(context: &mut SelectToolContext, event: Option<&select_tool::Event>, sink: &mut Vec<Command<select_tool::SelectTool>>) {
    let Some(select_tool::Event::Stream(request)) = event else { return };
    continue_with(context, request.records.iter(), sink);
}

fn continue_with<'a>(context: &mut SelectToolContext, ticks: impl Iterator<Item = &'a Puzzle2dSelectionRecord>, sink: &mut Vec<Command<select_tool::SelectTool>>) {
    let Some(mut stream) = context.stream.take() else { return };
    for tick in ticks {
        stream = stream.then(tick).unwrap_or(stream);
    }
    sink.push(Command::Effect(ToolYield::upsert(PUZZLE2D_SELECT_TOOL_LEAF_KEY, stream.mutation())));
    context.stream = Some(stream);
}

fn finish_stream(context: &mut SelectToolContext, event: Option<&select_tool::Event>, sink: &mut Vec<Command<select_tool::SelectTool>>) {
    let Some(select_tool::Event::Finish(request)) = event else { return };
    let Some(mut stream) = context.stream.take() else { return };
    for tick in &request.records {
        stream = stream.then(tick).unwrap_or(stream);
    }
    let yields = puzzle2d_selection_yields(&request.base, std::slice::from_ref(&stream), request.proximity_radius);
    if yields.is_empty() {
        sink.push(Command::Effect(ToolYield::retract(PUZZLE2D_SELECT_TOOL_LEAF_KEY)));
    }
    sink.extend(yields.into_iter().map(|(key, mutation)| Command::Effect(ToolYield::upsert(key, mutation))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel_stream(context: &mut SelectToolContext, _event: Option<&select_tool::Event>, sink: &mut Vec<Command<select_tool::SelectTool>>) {
    context.stream = None;
    sink.push(Command::Effect(ToolYield::Abort));
}

machine::statechart! {
    machine select_tool {
        context: SelectToolContext;
        event Event { Records(SelectToolRequest), Stream(SelectToolRequest), Finish(SelectToolRequest), Cancel }
        input: SelectToolContext;
        output: ();
        effect: ToolYield<Puzzle2dMutation>;
        context_from_input: select_tool_context;
        initial: idle;
        state idle {
            on Records if records_apply => idle do yield_records;
            on Stream if stream_applies => streaming do begin_stream;
        }
        state streaming {
            on Stream => streaming do continue_stream;
            on Finish => idle do finish_stream;
            on Cancel => idle do cancel_stream;
        }
    }
}

/// 🎚️ Where one dispatch of a selection transform sits in a select-tool gesture: a one-shot `Once`, a `Stream`
/// tick into the window's open transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dSelectPhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl Puzzle2dSelectPhase {
    /// 🧩️ Reads a transform verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's
    /// `reason` (`blur`, `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
    pub fn from_args(args: Option<&Value>) -> Option<Self> {
        let text = |key: &str| args.and_then(|args| args.get(key)).and_then(Value::as_str);
        match text("phase") {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => text("reason").map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }
}

/// 🧯️ A host abort of a persisted gesture: the open transaction vanishes with zero trace. Answers whether a gesture
/// was open.
pub fn puzzle2d_select_tool_abort(state: &Puzzle2dSelectToolState, reason: ToolAbortReason) -> bool {
    Puzzle2dSelectTool::resume(state).is_ok_and(|mut tool| matches!(tool.abort(reason), ToolStep::Aborted(..)))
}

/// 🔑️ The transaction key of a gesture's parametric leaf — the one key every record and stream tick upserts, so an
/// open transaction always holds ONE net leaf.
pub const PUZZLE2D_SELECT_TOOL_LEAF_KEY: &str = "selection:0";

/// 🧷️ The select tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct SelectToolHost;

impl machine::Host<select_tool::SelectTool> for SelectToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Puzzle2dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// ⏰️ The host clock a select-tool event runs on: the host's wall time, so a transaction id minted at an upsert is
/// unique per admission AND per moment.
pub fn puzzle2d_select_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🧷️ One provisional entry of a persisted select-tool transaction, its mutation in value form so the window
/// transient retires it like any other value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dSelectToolEntry {
    pub key: String,
    pub mutation: dsl::DslValue,
}

/// 💾️ A window's in-flight select-tool gesture, persisted in its window transient between dispatches — ephemeral
/// local tool state, never history: the statechart configuration by stable ids, the admission and document
/// revision it opened on, whether its drop connects, and the open transaction's provisional entries.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dSelectToolState {
    pub states: Vec<String>,
    pub verb: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub connect: bool,
    pub transaction: protocol::TransactionRef,
    pub entries: Vec<Puzzle2dSelectToolEntry>,
}

store::artifact_retire_struct!(Puzzle2dSelectToolEntry { key, mutation });
store::artifact_retire_struct!(Puzzle2dSelectToolState { states, verb, authoring_seed, base_revision, connect, transaction, entries });

/// 🛠️ One window's select tool for one dispatch: started at rest, or resumed from the gesture its window
/// transient persisted, driven by one event or one host abort, and persisted back while its transaction is open.
pub struct Puzzle2dSelectTool {
    runner: ToolMachineRunner<select_tool::SelectTool, SelectToolHost>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

impl Puzzle2dSelectTool {
    /// 🚀️ The tool at rest, for `<appId>#<verb>` under this admission's seed and document revision.
    pub fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(format!("{PUZZLE2D_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), SelectToolContext::default(), SelectToolHost)?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    /// ⏯️ The gesture a window transient persisted, restored by stable ids with its open transaction; a state the
    /// current chart cannot restore is refused (`Closed`), so the caller drops it with zero trace.
    pub fn resume(state: &Puzzle2dSelectToolState) -> Result<Self, ToolRefusal> {
        let definition = <select_tool::SelectTool as machine::Machine>::definition();
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: definition.fingerprint, states: state.states.clone(), history: Vec::new(), done: false };
        let entries = state.entries.iter().map(|entry| Ok((entry.key.clone(), dsl::FromValue::from_value(entry.mutation.clone()).map_err(|_| ToolRefusal::Closed)?))).collect::<Result<Vec<(String, Puzzle2dMutation)>, ToolRefusal>>()?;
        let stream = entries.iter().find(|(key, _)| key == PUZZLE2D_SELECT_TOOL_LEAF_KEY).and_then(|(_, leaf)| Puzzle2dSelectionRecord::from_leaf(leaf, state.connect));
        let snapshot = machine::restore::<select_tool::SelectTool, machine::NoMigrations>(&persisted, SelectToolContext { stream }, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(state.transaction.clone(), entries);
        let runner = ToolMachineRunner::resume(format!("{PUZZLE2D_EDITOR_APP_ID}#{}", state.verb), protocol::ActorId(state.authoring_seed.clone()), SelectToolContext::default(), snapshot, Some(transaction), SelectToolHost)?;
        Ok(Self { runner, verb: state.verb.clone(), authoring_seed: state.authoring_seed.clone(), base_revision: state.base_revision.clone() })
    }

    /// 📐️ The document revision the open gesture was opened on.
    pub fn base_revision(&self) -> &str {
        &self.base_revision
    }

    /// 🏷️ The transform verb whose gesture the tool drives.
    pub fn verb(&self) -> &str {
        &self.verb
    }

    /// 🛋️ Whether the tool rests (no gesture in flight).
    pub fn at_rest(&self) -> bool {
        self.runner.at_rest()
    }

    /// 📨️ Runs one event on the host clock.
    pub fn send(&mut self, event: select_tool::Event) -> Result<ToolStep<Puzzle2dMutation>, ToolRefusal> {
        self.runner.send(event, puzzle2d_select_tool_clock())
    }

    /// 🧯️ Host abort: the open transaction vanishes with zero trace and the tool rests.
    pub fn abort(&mut self, reason: ToolAbortReason) -> ToolStep<Puzzle2dMutation> {
        self.runner.abort(reason)
    }

    /// 💾️ The state to persist in the window transient: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<Puzzle2dSelectToolState> {
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(Puzzle2dSelectToolState {
            states: machine::persist(&snapshot).states,
            verb: self.verb,
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            connect: snapshot.context.stream.as_ref().is_some_and(|stream| stream.connect),
            transaction: transaction.reference().clone(),
            entries: transaction.entries().iter().map(|(key, mutation)| Puzzle2dSelectToolEntry { key: key.clone(), mutation: dsl::ToValue::to_value(mutation) }).collect(),
        })
    }
}

/// 🛠️ Runs `records` through a select tool at rest as ONE one-shot transaction on `clock` — the ref minted from the
/// admission's `authoring_seed`, the clock and `<appId>#<verb>`, and the yielded mutations in order. `None` when no
/// record moves anything: an all-locked, all-missing or empty request leaves zero trace.
pub fn puzzle2d_select_tool_commit(verb: &str, authoring_seed: &str, clock: protocol::HybridLogicalTimestamp, request: SelectToolRequest) -> Option<(protocol::TransactionRef, Vec<Puzzle2dMutation>)> {
    let mut runner = ToolMachineRunner::<select_tool::SelectTool, SelectToolHost>::start(format!("{PUZZLE2D_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), SelectToolContext::default(), SelectToolHost).ok()?;
    match runner.send(select_tool::Event::Records(request), clock).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}

/// 👁️ The document a window paints while its select tool holds an open transaction: the provisional entries
/// applied to `document` — a preview only this window sees, never history.
pub fn puzzle2d_select_tool_preview(document: &Puzzle2dSnapshot, state: &Puzzle2dSelectToolState) -> Puzzle2dSnapshot {
    let mut preview = document.clone();
    for mutation in state.entries.iter().filter_map(|entry| dsl::FromValue::from_value(entry.mutation.clone()).ok()) {
        let _ = apply_puzzle2d_mutation(&mut preview, &mutation);
    }
    preview
}

/// 🧮️ What the select tool yields for `records` on `base`, keyed: each movable record's parametric leaf, then the
/// `connect-handles` its drop lands — the handle pairs the board recorded first, then (for a drop) the proximity
/// search over the moved state — until the gesture's [`PUZZLE2D_PROXIMITY_GESTURE_MAX`] budget is spent. Every
/// edge id is minted HERE, deterministically from its pair and the document, so a replay never mints again.
pub fn puzzle2d_selection_yields(base: &Puzzle2dSnapshot, records: &[Puzzle2dSelectionRecord], radius: f64) -> Vec<(String, Puzzle2dMutation)> {
    let mut state = base.clone();
    let mut yields = Vec::new();
    let mut connects = 0usize;
    for (index, record) in records.iter().enumerate() {
        let leaf = record.mutation();
        if !record.applies_to(&state) || apply_puzzle2d_mutation(&mut state, &leaf).is_err() {
            continue;
        }
        yields.push((format!("selection:{index}"), leaf));
        if record.proximity.is_empty() && !record.connect {
            continue;
        }
        let mut scratch = Value::from(dsl::ToValue::to_value(&state));
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (source, target) in &record.proximity {
            if puzzle2d_handles_open(&scratch, source, target) {
                puzzle2d_push_edge(&mut scratch, json!({ "source": source, "target": target }));
                pairs.push((source.clone(), target.clone()));
            }
        }
        if record.connect {
            let moved: Vec<String> = puzzle2d_unique_targets(record.targets.iter().cloned()).into_iter().filter(|id| state.nodes.iter().any(|node| &node.id == id)).collect();
            for node_id in &moved {
                for pair in puzzle2d_proximity_pairs(&scratch, node_id, radius) {
                    puzzle2d_push_edge(&mut scratch, json!({ "source": pair.peer, "target": pair.moved }));
                    pairs.push((pair.peer, pair.moved));
                }
            }
        }
        for (source, target) in pairs {
            if connects == PUZZLE2D_PROXIMITY_GESTURE_MAX {
                break;
            }
            let id = puzzle2d_minted_edge_id(&state, &source, &target);
            let connect = connect_handles(id.clone(), source, target, None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None);
            if apply_puzzle2d_mutation(&mut state, &connect).is_ok() {
                yields.push((format!("connect:{id}"), connect));
                connects += 1;
            }
        }
    }
    yields
}

/// 🔌️ Whether `source` → `target` can still connect on `fixture`: two distinct handles on two distinct nodes,
/// neither already carrying an edge.
fn puzzle2d_handles_open(fixture: &Value, source: &str, target: &str) -> bool {
    let owner = |handle_id: &str| fixture_nodes(fixture).iter().find(|node| node.get("handles").and_then(Value::as_array).is_some_and(|handles| handles.iter().any(|handle| handle.get("id").and_then(Value::as_str) == Some(handle_id)))).and_then(|node| node.get("id").and_then(Value::as_str));
    let occupied = puzzle2d_occupied_handles(fixture);
    source != target && !occupied.contains(source) && !occupied.contains(target) && matches!((owner(source), owner(target)), (Some(from), Some(to)) if from != to)
}

/// 🆔️ The edge id a yielded connection carries: `edge-<source>-<target>`, suffixed `-2`, `-3`, … past any id the
/// document already holds — a pure function of the pair and the document, never a process counter.
pub fn puzzle2d_minted_edge_id(document: &Puzzle2dSnapshot, source: &str, target: &str) -> String {
    let candidate = format!("edge-{source}-{target}");
    let taken = |id: &str| document.edges.iter().any(|edge| edge.id == id);
    if !taken(&candidate) {
        return candidate;
    }
    (2usize..).map(|serial| format!("{candidate}-{serial}")).find(|id| !taken(id)).expect("an unbounded serial finds a free id")
}
//#endregion 🛠️SelectTool

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../../../../../🧪️tests/🧪️select-tool/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
