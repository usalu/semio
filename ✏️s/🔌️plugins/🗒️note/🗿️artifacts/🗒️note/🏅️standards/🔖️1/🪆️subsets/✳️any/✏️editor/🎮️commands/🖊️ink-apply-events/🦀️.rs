//! 🖊️ Note play app command — `ink-apply-events`: every ink-canvas gesture as ONE ink tool transaction.
//!
//! 🛠️ The ink tool is a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the `🛠️tool-machine` runner.
//! The host streams a gesture (`phase: "stream"`), commits it (`"commit"`) or aborts it (`"abort"` with a `reason`);
//! an absent phase is a one-shot. A block drag yields ONE relative `drag-blocks` leaf from the host's gesture record;
//! strokes, erasers, resizes and placements yield the net per-block leaves their events make against the committed
//! document. Between dispatches the open transaction rides the composite window's transient (ephemeral, local, never
//! history) and the window paints committed ⊕ provisional; the commit publishes ONE edit stamped with its
//! `TransactionRef`. Design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5.
#![allow(unexpected_cfgs)]

use crate::op::NoteMutation;
use crate::schema::mutations::{apply_note_mutation, change_block_ink_width, change_block_locked, change_block_visible, create_asset, create_block, delete_block, drag_blocks, edit_block_ink_stroke, move_block, rename_block, replace_asset_payload, resize_block};
use crate::schema::{block_bounds, block_id, block_locked, block_name, block_visible, find_block, find_block_location, insert_block, remove_block_from_tree, update_block_in_tree};
use crate::{NoteBlockNode, NoteCamera, NoteImageAsset, NoteSnapshot};
use machine::Command;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::atomic::{AtomicU64, Ordering};

//#region 🔖️CanvasEvents
/// 🖱️ Batched canvas-event wire shape the `ink-canvas-host` surface emits (`addBlock`/`updateBlock`/
/// `removeBlock`/`putAsset`/`setCamera`); content events become the gesture's net leaves via
/// `note_event_yields`, while `setCamera` publishes to the exact composite-window config.
#[derive(Clone, Debug, FromValue)]
#[value(tag = "operation")]
enum NoteCanvasEvent {
    #[value(rename = "addBlock", rename_all = "camelCase")]
    AddBlock {
        block: NoteBlockNode,
        #[value(default)]
        parent_id: Option<String>,
        #[value(default)]
        index: Option<usize>,
    },
    #[value(rename = "updateBlock", rename_all = "camelCase")]
    UpdateBlock { block_id: String, block: NoteBlockNode },
    #[value(rename = "removeBlock", rename_all = "camelCase")]
    RemoveBlock { block_id: String },
    #[value(rename = "putAsset", rename_all = "camelCase")]
    PutAsset { key: String, asset: NoteImageAsset },
    #[value(rename = "setCamera", rename_all = "camelCase")]
    SetCamera { camera: NoteCamera },
}

/// 🎯 Which single/pair of narrow mutations turns `before` into `after` for one block — spatial
/// (`move`/`resize`) and flag (`visible`/`locked`/`name`) fields are compared generically; an `Ink`
/// block's `points`/bbox move together as one `edit-block-ink-stroke` (an authored stroke, never
/// split into a move+resize pair) unless only its `stroke_width` changed.
fn block_update_mutations(id: &str, before: &NoteBlockNode, after: &NoteBlockNode) -> Vec<NoteMutation> {
    let mut ops = Vec::new();
    let (bx, by, bw, bh) = block_bounds(before);
    let (ax, ay, aw, ah) = block_bounds(after);
    if let (NoteBlockNode::Ink { points: before_points, stroke_width: before_width, .. }, NoteBlockNode::Ink { points: after_points, stroke_width: after_width, .. }) = (before, after) {
        if before_points != after_points || (bx, by, bw, bh) != (ax, ay, aw, ah) {
            ops.push(edit_block_ink_stroke(id.to_string(), after_points.clone(), ax, ay, aw, ah));
        } else if before_width != after_width {
            ops.push(change_block_ink_width(id.to_string(), *after_width));
        }
    } else {
        if (bx, by) != (ax, ay) {
            ops.push(move_block(id.to_string(), ax, ay));
        }
        if (bw, bh) != (aw, ah) {
            ops.push(resize_block(id.to_string(), aw, ah));
        }
    }
    if block_visible(before) != block_visible(after) {
        ops.push(change_block_visible(id.to_string(), block_visible(after)));
    }
    if block_locked(before) != block_locked(after) {
        ops.push(change_block_locked(id.to_string(), block_locked(after)));
    }
    if block_name(before) != block_name(after) {
        ops.push(rename_block(id.to_string(), block_name(after).to_string()));
    }
    ops
}

/// 🔀️ `document` after a batch of canvas events, with every block id and asset key the batch touched in first-seen
/// order — the camera never touches the document (the caller publishes it to the composite-window config).
fn apply_canvas_events(document: &NoteSnapshot, events: &[NoteCanvasEvent]) -> (NoteSnapshot, Vec<String>, Vec<String>) {
    let mut next = document.clone();
    let (mut blocks, mut assets) = (Vec::new(), Vec::new());
    let touch = |list: &mut Vec<String>, id: &str| {
        if !list.iter().any(|seen| seen == id) {
            list.push(id.to_string());
        }
    };
    for event in events {
        match event {
            NoteCanvasEvent::AddBlock { block, parent_id, index } => {
                insert_block(&mut next.blocks, parent_id.as_deref(), index.unwrap_or(usize::MAX), block.clone());
                touch(&mut blocks, block_id(block));
            }
            NoteCanvasEvent::UpdateBlock { block_id: id, block } => {
                update_block_in_tree(&mut next.blocks, id, block.clone());
                touch(&mut blocks, id);
            }
            NoteCanvasEvent::RemoveBlock { block_id: id } => {
                remove_block_from_tree(&mut next.blocks, id);
                touch(&mut blocks, id);
            }
            NoteCanvasEvent::PutAsset { key, asset } => {
                next.assets.insert(key.clone(), asset.clone());
                touch(&mut assets, key);
            }
            NoteCanvasEvent::SetCamera { .. } => {}
        }
    }
    (next, blocks, assets)
}

/// 🧮️ The net leaves the gesture so far makes for block `id` against the committed `base`: one `create-block` at its
/// place in `next`, one `delete-block`, or the narrow field updates — nothing when the block ended where it began.
fn block_net_leaves(base: &NoteSnapshot, next: &NoteSnapshot, id: &str) -> Vec<NoteMutation> {
    match (find_block(&base.blocks, id), find_block(&next.blocks, id)) {
        (None, Some(block)) => {
            let (parent_id, index) = find_block_location(&next.blocks, id).map_or((None, None), |(parent_id, index)| (parent_id, Some(index)));
            vec![create_block(block.clone(), parent_id, index)]
        }
        (Some(_), None) => vec![delete_block(id.to_string())],
        (Some(before), Some(after)) if before != after => block_update_mutations(id, before, after),
        _ => Vec::new(),
    }
}

/// 🧾️ What one batch of canvas events yields into a gesture's transaction: for every block and asset the batch touches,
/// the net leaves the whole gesture makes against `base` so far, upserted under `block:<id>#<n>` / `asset:<key>` with
/// the stale ones retracted — so an open transaction holds exactly the gesture's net effect.
fn note_event_yields(base: &NoteSnapshot, provisional: &[(String, NoteMutation)], events: &[NoteCanvasEvent]) -> Result<Vec<ToolYield<NoteMutation>>, Fault> {
    let mut current = base.clone();
    for (_, leaf) in provisional.iter().filter(|(key, _)| key.starts_with("block:") || key.starts_with("asset:")) {
        current = apply_note_mutation(&current, leaf).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("note.ink-tool.provisional"), error.to_string()))?;
    }
    let (next, blocks, assets) = apply_canvas_events(&current, events);
    let mut yields = Vec::new();
    for id in &blocks {
        let prefix = format!("block:{id}#");
        let leaves = block_net_leaves(base, &next, id);
        let count = leaves.len();
        yields.extend(provisional.iter().filter(|(key, _)| key.strip_prefix(&prefix).and_then(|index| index.parse::<usize>().ok()).is_some_and(|index| index >= count)).map(|(key, _)| ToolYield::retract(key.clone())));
        yields.extend(leaves.into_iter().enumerate().map(|(index, leaf)| ToolYield::upsert(format!("{prefix}{index}"), leaf)));
    }
    for key in &assets {
        let entry = format!("asset:{key}");
        match next.assets.get(key).filter(|asset| base.assets.get(key) != Some(*asset)) {
            Some(asset) => yields.push(ToolYield::upsert(entry, if base.assets.contains_key(key) { replace_asset_payload(key.clone(), asset.clone()) } else { create_asset(key.clone(), asset.clone()) })),
            None => yields.push(ToolYield::retract(entry)),
        }
    }
    Ok(yields)
}

/// 🤏️ The host's record of a block drag — `{"kind":"drag","ids":[…],"dx":…,"dy":…}`, the offset from the gesture's
/// start — as the ONE relative `drag-blocks` leaf it yields under the gesture key; the identity drag retracts it.
fn note_gesture_yields(gesture_json: &str) -> Result<Vec<ToolYield<NoteMutation>>, Fault> {
    let invalid = |detail: &str| Fault::new(FaultOrigin::App, FaultCode::new("note.ink-gesture.invalid"), detail.to_string());
    let record: serde_json::Value = serde_json::from_str(gesture_json).map_err(|_| invalid("the ink gesture record is not JSON"))?;
    if record.get("kind").and_then(serde_json::Value::as_str) != Some("drag") {
        return Err(invalid("the ink gesture record kind is not drag"));
    }
    let number = |key: &str| record.get(key).and_then(serde_json::Value::as_f64).filter(|value| value.is_finite()).ok_or_else(|| invalid("the ink gesture offset is not a finite number"));
    let (dx, dy) = (number("dx")?, number("dy")?);
    let mut ids: Vec<String> = Vec::new();
    for id in record.get("ids").and_then(serde_json::Value::as_array).ok_or_else(|| invalid("the ink gesture names no blocks"))?.iter() {
        let id = id.as_str().ok_or_else(|| invalid("an ink gesture block id is not text"))?;
        if !ids.iter().any(|seen| seen == id) {
            ids.push(id.to_string());
        }
    }
    Ok(vec![if ids.is_empty() || (dx, dy) == (0.0, 0.0) { ToolYield::retract(NOTE_INK_GESTURE_KEY) } else { ToolYield::upsert(NOTE_INK_GESTURE_KEY, drag_blocks(ids, dx, dy)) }])
}

/// 🖋️ Decodes the ink-canvas host's `InkCanvasEvent[]` batch (framework `ink.document` block shapes) into
/// note events — a malformed batch is a fault, never a silently empty gesture.
fn decode_canvas_events(events_json: &str) -> Result<Vec<NoteCanvasEvent>, Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("note.ink-events.invalid"), detail);
    let parsed = semio_framework_pack_json::parse(events_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| invalid(format!("ink events are not JSON: {error:?}")))?;
    let mut value = semio_framework_pack_json::to_dsl_value(&parsed);
    if let semio_framework_value::DslValue::Array(events) = &mut value {
        for event in events {
            if let semio_framework_value::DslValue::Object(entries) = event {
                if let Some((_, block)) = entries.iter_mut().find(|(name, _)| name == "block") {
                    crate::note_block_value_from_ink_wire(block).map_err(invalid)?;
                }
            }
        }
    }
    <Vec<NoteCanvasEvent> as semio_framework_value::FromValue>::from_value(value).map_err(|error| invalid(format!("ink events do not decode: {error}")))
}
//#endregion 🔖️CanvasEvents

//#region 🛠️InkTool
/// 🔑️ The transaction key of a gesture's relative drag leaf — every drag tick upserts it, so an open transaction always
/// holds ONE net `drag-blocks`.
pub const NOTE_INK_GESTURE_KEY: &str = "gesture";

/// 📦️ One ink tool event's yields, computed against the committed document by the dispatch — never tool state.
#[derive(Clone, Debug, PartialEq)]
pub struct NoteInkBatch {
    pub yields: Vec<ToolYield<NoteMutation>>,
}

/// 🎛️ The ink tool's context: empty — a gesture's whole state is its open transaction.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NoteInkToolContext;

fn ink_tool_context(_input: ()) -> NoteInkToolContext {
    NoteInkToolContext
}

fn batch(event: Option<&ink_tool::Event>) -> Option<&NoteInkBatch> {
    match event {
        Some(ink_tool::Event::Once(batch) | ink_tool::Event::Stream(batch) | ink_tool::Event::Finish(batch)) => Some(batch),
        _ => None,
    }
}

fn batch_upserts(_context: &NoteInkToolContext, event: Option<&ink_tool::Event>) -> bool {
    batch(event).is_some_and(|batch| batch.yields.iter().any(|yielded| matches!(yielded, ToolYield::Upsert { .. })))
}

fn yield_batch(_context: &mut NoteInkToolContext, event: Option<&ink_tool::Event>, sink: &mut Vec<Command<ink_tool::InkTool>>) {
    sink.extend(batch(event).into_iter().flat_map(|batch| batch.yields.iter().cloned()).map(Command::Effect));
}

fn yield_and_commit(context: &mut NoteInkToolContext, event: Option<&ink_tool::Event>, sink: &mut Vec<Command<ink_tool::InkTool>>) {
    yield_batch(context, event, sink);
    sink.push(Command::Effect(ToolYield::Commit));
}

fn abort_gesture(_context: &mut NoteInkToolContext, _event: Option<&ink_tool::Event>, sink: &mut Vec<Command<ink_tool::InkTool>>) {
    sink.push(Command::Effect(ToolYield::Abort));
}

// 🎭️ The ink tool's control flow (plain comment: rustdoc cannot document a macro invocation): a one-shot yields and
// commits at rest; a stream opens `streaming`, whose ticks upsert and whose finish commits ONE transaction.
machine::statechart! {
    machine ink_tool {
        context: NoteInkToolContext;
        event Event { Once(NoteInkBatch), Stream(NoteInkBatch), Finish(NoteInkBatch), Cancel }
        input: ();
        output: ();
        effect: ToolYield<NoteMutation>;
        context_from_input: ink_tool_context;
        initial: idle;
        state idle {
            on Once if batch_upserts => idle do yield_and_commit;
            on Finish if batch_upserts => idle do yield_and_commit;
            on Stream if batch_upserts => streaming do yield_batch;
        }
        state streaming {
            on Stream => streaming do yield_batch;
            on Finish => idle do yield_and_commit;
            on Cancel => idle do abort_gesture;
        }
    }
}

/// 🧷️ The ink tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct NoteInkToolHost;

impl machine::Host<ink_tool::InkTool> for NoteInkToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<NoteMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

static NOTE_INK_TOOL_TICK: AtomicU64 = AtomicU64::new(0);

/// ⏰️ The host clock an ink tool event runs on: wall time and a process-monotone tick, so a transaction id minted at an
/// upsert is unique per author, moment and event even when two gestures share a millisecond.
pub fn note_ink_tool_clock() -> protocol::HybridLogicalTimestamp {
    semio_framework_tool_machine::authoring_clock(NOTE_INK_TOOL_TICK.fetch_add(1, Ordering::Relaxed))
}

/// 🧷️ One provisional entry of a persisted ink tool transaction, its mutation in value form so the window transient
/// retires it like any other value.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NoteInkToolEntry {
    pub key: String,
    pub mutation: semio_framework_value::DslValue,
}

/// 💾️ A composite window's in-flight ink gesture, persisted in its window transient between dispatches — ephemeral
/// local tool state, never history: the statechart configuration by stable ids, the verb, the admission and document
/// revision it opened on, and the open transaction's provisional entries.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NoteInkToolState {
    pub states: Vec<String>,
    pub verb: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: protocol::TransactionRef,
    pub entries: Vec<NoteInkToolEntry>,
}

semio_framework_value::artifact_retire_struct!(NoteInkToolEntry { key, mutation });
semio_framework_value::artifact_retire_struct!(NoteInkToolState { states, verb, authoring_seed, base_revision, transaction, entries });

/// 🎚️ Where one dispatch sits in an ink gesture: a one-shot `Once`, a `Stream` tick into the window's open
/// transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoteInkPhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl NoteInkPhase {
    /// 🧩️ Reads `phase` (`stream` | `commit` | `abort`; absent = one-shot) and an abort's `reason` (`blur`,
    /// `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); an unknown spelling is a fault.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Result<Self, Fault> {
        let invalid = || Fault::new(FaultOrigin::App, FaultCode::new("note.ink-phase.invalid"), "unknown ink gesture phase or abort reason");
        match phase {
            None => Ok(Self::Once),
            Some("stream") => Ok(Self::Stream),
            Some("commit") => Ok(Self::Commit),
            Some("abort") => reason.map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort).ok_or_else(invalid),
            Some(_) => Err(invalid()),
        }
    }
}

/// 🛠️ One composite window's ink tool for one dispatch: started at rest, or resumed from the gesture its window
/// transient persisted, driven by one event or one host abort, and persisted back while its transaction is open.
pub struct NoteInkTool {
    runner: ToolMachineRunner<ink_tool::InkTool, NoteInkToolHost>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

fn refusal(refused: ToolRefusal) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(refused.code()), "the Note ink tool refused its own transaction")
}

impl NoteInkTool {
    /// 🚀️ The tool at rest for `<appId>#<verb>` under this admission's seed and document revision.
    pub fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(format!("{}#{verb}", crate::editor::note::NOTE_PLAY_CONTROLLER_ID), protocol::ActorId(authoring_seed.into()), (), NoteInkToolHost)?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    /// ⏯️ The gesture a window transient persisted, restored by stable ids with its open transaction; a state the
    /// current chart cannot restore is refused (`Closed`), so the caller drops it with zero trace.
    pub fn resume(state: &NoteInkToolState) -> Result<Self, ToolRefusal> {
        let definition = <ink_tool::InkTool as machine::Machine>::definition();
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: definition.fingerprint, states: state.states.clone(), history: Vec::new(), done: false };
        let entries = state.entries.iter().map(|entry| Ok((entry.key.clone(), semio_framework_value::FromValue::from_value(entry.mutation.clone()).map_err(|_| ToolRefusal::Closed)?))).collect::<Result<Vec<(String, NoteMutation)>, ToolRefusal>>()?;
        let snapshot = machine::restore::<ink_tool::InkTool, machine::NoMigrations>(&persisted, NoteInkToolContext, &[]).map_err(|_| ToolRefusal::Closed)?;
        let runner = ToolMachineRunner::resume(format!("{}#{}", crate::editor::note::NOTE_PLAY_CONTROLLER_ID, state.verb), protocol::ActorId(state.authoring_seed.as_str().into()), (), snapshot, Some(ToolTransaction::resume(state.transaction.clone(), entries)), NoteInkToolHost)?;
        Ok(Self { runner, verb: state.verb.clone(), authoring_seed: state.authoring_seed.clone(), base_revision: state.base_revision.clone() })
    }

    /// 🛋️ Whether the tool rests (no gesture in flight).
    pub fn at_rest(&self) -> bool {
        self.runner.at_rest()
    }

    /// 📝️ The open transaction's provisional entries, in first-insertion order.
    pub fn provisional(&self) -> Vec<(String, NoteMutation)> {
        self.runner.transaction().map(|transaction| transaction.entries().to_vec()).unwrap_or_default()
    }

    /// 📨️ Runs one event on the host clock.
    pub fn send(&mut self, event: ink_tool::Event) -> Result<ToolStep<NoteMutation>, ToolRefusal> {
        self.runner.send(event, note_ink_tool_clock())
    }

    /// 🧯️ Host abort: the open transaction vanishes with zero trace and the tool rests.
    pub fn abort(&mut self, reason: ToolAbortReason) -> ToolStep<NoteMutation> {
        self.runner.abort(reason)
    }

    /// 💾️ The state to persist in the window transient: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<NoteInkToolState> {
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(NoteInkToolState {
            states: machine::persist(&snapshot).states,
            verb: self.verb,
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            transaction: transaction.reference().clone(),
            entries: transaction.entries().iter().map(|(key, mutation)| NoteInkToolEntry { key: key.clone(), mutation: semio_framework_value::ToValue::to_value(mutation) }).collect(),
        })
    }
}

/// 📐️ The admission's canonical document revision in hex — what a persisted gesture is pinned to.
pub fn note_base_revision(doc: &ArtifactView<'_, NoteSnapshot>) -> String {
    doc.operation_optional().map(|operation| operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect()).unwrap_or_default()
}

/// 🛠️ Drives the window's ink tool through ONE dispatch of `verb`. `Once` commits its yields as one transaction;
/// `Stream` upserts them into the window's open transaction — opening it on the first tick — which `slot` persists;
/// `Commit` folds the final yields in and commits the whole gesture as ONE edit; `Abort` drops the open gesture with
/// zero trace. An open gesture another verb (or a one-shot) interrupts is aborted `captureLost`; one whose document
/// moved under it is aborted `baseMoved`, and a stream tick or commit that found it is dropped with it. Without an
/// authoring seed (a test or render view without command authority) the yielded mutations publish plainly.
pub fn note_ink_dispatch(
    doc: &ArtifactView<'_, NoteSnapshot>,
    slot: &mut Option<NoteInkToolState>,
    verb: &str,
    phase: NoteInkPhase,
    yields: impl FnOnce(&[(String, NoteMutation)]) -> Result<Vec<ToolYield<NoteMutation>>, Fault>,
) -> Result<Emit<NoteMutation, NoConfigMutation>, Fault> {
    let seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
    let base_revision = note_base_revision(doc);
    let open = slot.take().and_then(|state| NoteInkTool::resume(&state).ok());
    let open = match (open, phase) {
        (Some(mut tool), NoteInkPhase::Abort(reason)) => {
            tool.abort(reason);
            return Ok(Emit::default());
        }
        (None, NoteInkPhase::Abort(_)) => return Ok(Emit::default()),
        (Some(mut tool), _) if tool.base_revision != base_revision => {
            tool.abort(ToolAbortReason::BaseMoved);
            if phase != NoteInkPhase::Once {
                return Ok(Emit::default());
            }
            None
        }
        (Some(mut tool), _) if tool.verb != verb || phase == NoteInkPhase::Once => {
            tool.abort(ToolAbortReason::CaptureLost);
            None
        }
        (open, _) => open,
    };
    let mut tool = match open {
        Some(tool) => tool,
        None => NoteInkTool::start(verb, seed, &base_revision).map_err(refusal)?,
    };
    let batch = NoteInkBatch { yields: yields(&tool.provisional())? };
    let event = match phase {
        NoteInkPhase::Stream => ink_tool::Event::Stream(batch),
        NoteInkPhase::Commit if !tool.at_rest() => ink_tool::Event::Finish(batch),
        _ => ink_tool::Event::Once(batch),
    };
    let step = tool.send(event).map_err(refusal)?;
    *slot = tool.persist();
    Ok(match step {
        ToolStep::Committed(transaction, mutations) if !seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        ToolStep::Committed(_, mutations) => Emit::mutations(mutations),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => Emit::default(),
    })
}

/// 👁️ The document a composite window paints while its ink tool holds an open transaction: the provisional entries
/// applied to `document` — a preview only this window sees, never history.
pub fn note_ink_tool_preview(document: &NoteSnapshot, state: &NoteInkToolState) -> NoteSnapshot {
    state.entries.iter().filter_map(|entry| semio_framework_value::FromValue::from_value(entry.mutation.clone()).ok()).fold(document.clone(), |preview, mutation: NoteMutation| apply_note_mutation(&preview, &mutation).unwrap_or(preview))
}
//#endregion 🛠️InkTool

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "ink-apply-events")]
pub struct InkApplyEvents {
    #[value(default)]
    pub events_json: String,
    pub phase: Option<String>,
    pub reason: Option<String>,
    pub gesture_json: Option<String>,
    pub select_ids: Option<Vec<String>>,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `payload.select_ids` used to become the new selection
// after a gesture — selection is framework-owned `InteractionState` now, only ever mutated by the framework's own
// injected `interactionSelect` handling; the field stays on the wire (the ink-canvas host still sends it) but is not
// acted on.
pub fn handle(payload: &InkApplyEvents, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoConfigMutation>, Fault> {
    let phase = NoteInkPhase::parse(payload.phase.as_deref(), payload.reason.as_deref())?;
    let events = if payload.events_json.is_empty() { Vec::new() } else { decode_canvas_events(&payload.events_json)? };
    let mut window_config_mutations = Vec::new();
    for event in &events {
        if let NoteCanvasEvent::SetCamera { camera } = event {
            let view = ctx.view_state.as_ref().ok_or_else(|| Fault::from("note-composite-window-context-required"))?;
            window_config_mutations.push(crate::editor::note::window::addressed_config(view, crate::editor::note::window::NoteCompositeWindowConfig { camera: camera.clone() })?);
        }
    }
    let base = doc.snapshot;
    let mut emit = note_ink_dispatch(doc, &mut ctx.window_transient.ink_tool, "inkApplyEvents", phase, |provisional| {
        let mut yields = note_event_yields(base, provisional, &events)?;
        if let Some(gesture) = payload.gesture_json.as_deref() {
            yields.extend(note_gesture_yields(gesture)?);
        }
        Ok(yields)
    })?;
    emit.window_config_mutations.extend(window_config_mutations);
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
