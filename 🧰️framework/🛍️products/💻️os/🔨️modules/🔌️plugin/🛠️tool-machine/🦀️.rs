//! 🛠️ OS runtime of the framework's continuous tools (design §13 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), window
//! transient, ephemeral, local-only, never history:
//! - 🎚️ continuous controls (§13.1): every dispatch whose args carry a `gesture` rides the dispatching window's press in the
//!   instance's [`ScrubLedger`]; a tick publishes nothing, the release publishes ONE edit, every host abort leaves zero trace.
//!   Apps supply only the ABSOLUTE leaves of a value: their verb's `Emit::mutations`, and the owned children's share
//!   (`Emit::child_emits`, design §12) — ONE press commits ONE transaction stamped on every member it touched.
//! - ⌨️ typing (§13.2): every live typing delivery (args carry a `typing` buffer) folds its leaves into the window's run in
//!   the [`TypingLedger`] through the app's typing algebra (`ArtifactApp::typing_fold`); the run commits as ONE edit on idle,
//!   a caret jump, blur, page hide, Enter, an explicit apply or any other verb, and aborts only on a conflicting base or a
//!   frozen document.
//!
//! - 🎛️ the config lanes of a press (§20.1): a tick's app-config and window-config mutations ride the same press as its
//!   document leaves ([`PressLeaf`]), rendered over the committed config and never published; the release publishes them
//!   as ONE config edit in the very ledger step that commits the document leaves, a host abort drops them with zero
//!   trace; config edits are never history rows.
//!
//! - 🖐️ gestures (§5, §22.10): every window owns ONE gesture slot in the instance's [`GestureLedger`]. A dispatch drives its
//!   window's streamed tool against a copy of the slot ([`GestureSlot::drive`]); the slot follows only when the dispatch
//!   publishes. Host facts end a window's gesture here, never in an editor: a blur `blur`, a lost pointer capture
//!   `captureLost`, a utility switch or a closing window `retired`, a moved base `baseMoved` (a gesture pinned to a revision),
//!   and an opened history edit `frozen` for every window — zero trace each time.
//!
//! Every render seam reads the committed document (and config) overlaid with every open press's, gesture's and run's
//! provisional leaves; every committed transaction is published stamped with its `TransactionRef`. Domain-neutral machines:
//! `semio_framework_tool_machine` (`ScrubMachine`, `TypingMachine`, `ChartGesture`).

use super::*;
use semio_framework_tool_machine::{
    GestureHostEvent, GestureLedger, GesturePhase, GestureState, GestureTool, ScrubLedger, ScrubPhase, ToolAbortReason, ToolStep, TypingCommit, TypingInput, TypingLedger, TypingPhase, SCRUB_ABORT_ARG, SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG,
    TYPING_BUFFER_ARG, TYPING_COMMIT_ARG,
};
use std::collections::BTreeMap;
use std::sync::Arc;

/// 🧹️ The grant of one retirement step of a displaced overlay alias and the turns a refold spends; a refold retires its
/// aliases to their terminal-empty witness, the cost a plain drop would have paid (a step that hands the last alias to its
/// owned-value disposer releases nothing yet, so a zero step is progress, never the end).
const TOOL_OVERLAY_RETIREMENT_ITEMS: usize = 4_096;
const TOOL_OVERLAY_RETIREMENT_BYTES: usize = 1 << 24;
const TOOL_OVERLAY_RETIREMENT_TURNS: usize = 1 << 20;

/// ♻️ Steps the retirement of one displaced overlay alias to its terminal-empty witness — the cost a plain drop would have
/// paid; a store that cannot retire the alias leaves nothing to step.
pub(crate) fn retire_overlay_alias<E>(retirement: Result<Box<dyn store::ErasedSnapshotRetirement>, E>) {
    let Ok(mut retirement) = retirement else { return };
    for _ in 0..TOOL_OVERLAY_RETIREMENT_TURNS {
        if retirement.terminal_is_empty() || retirement.close_step(TOOL_OVERLAY_RETIREMENT_ITEMS, retirement.next_close_byte_demand().max(TOOL_OVERLAY_RETIREMENT_BYTES)).is_err() {
            break;
        }
    }
}

/// 🧮️ Folds the provisional `leaf` onto `running` (else `committed`) and answers whether it applied (a leaf its base refuses
/// is skipped); the displaced intermediate joins `displaced`, which the caller retires through its store, never plainly.
pub(crate) fn fold_leaf<P, M: Mutation<P>>(committed: &Arc<P>, running: &mut Option<Arc<P>>, displaced: &mut Vec<Arc<P>>, leaf: &M) -> bool {
    let base: &P = running.as_deref().unwrap_or(committed.as_ref());
    let outcome = leaf.diff(base);
    let applicable = outcome.is_applicable(protocol::MergePolicy::default());
    let (diff, _) = outcome.into_parts();
    let applied = applicable.then(|| diff.apply(base));
    MutationDiff::retire_cold(diff);
    match applied {
        Some(Ok(next)) => {
            displaced.extend(running.replace(Arc::new(next)));
            true
        }
        _ => false,
    }
}

//#region 🔖️Tag
/// 🏷️ One continuous-control dispatch as its operation carries it to publication: the dispatching window, the tool
/// `<appId>#<verb>` and where the dispatch sits in its press.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrubTag {
    pub window: String,
    pub tool: String,
    pub phase: ScrubPhase,
}

/// ⌨️ One live typing delivery as its operation carries it to publication: the typing window, the tool `<appId>#<verb>` and
/// the buffer typed into.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypingTag {
    pub window: String,
    pub tool: String,
    pub buffer: String,
}

/// 🏷️ What an admitted operation carries to its publication: a press of a continuous control or a typed edit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolTag {
    Scrub(ScrubTag),
    Typing(TypingTag),
}

/// 🚦️ How `dispatch_action` proceeds once it read the tool arguments: a plain dispatch, a press or typed edit whose operation
/// carries the tag, or a dispatch the tool runtime settled itself (a host abort, a commit signal, a frozen edit).
pub(super) enum ToolDispatch {
    Plain,
    Tagged(ToolTag),
    Settled(Result<InvocationResult, Fault>),
}
//#endregion 🔖️Tag

//#region 🖐️Gesture
/// 🎰️ The dispatching window's gesture slot as ONE admitted dispatch sees it (design §22.10): the gesture the window held
/// at admission with the press it last closed, the document revision the dispatch runs on, and what the dispatch decided — kept by the runtime only when
/// the dispatch publishes, and only while the window still holds the gesture it was admitted on. Shared with the dispatch's
/// retained job, so its state sits behind one lock.
pub struct GestureSlot<M> {
    window: String,
    base_revision: String,
    state: std::sync::Mutex<GestureSlotState<M>>,
}

struct GestureSlotState<M> {
    admitted: Option<GestureState<M>>,
    closed: Option<String>,
    decided: Option<Option<GestureState<M>>>,
    closing: Option<String>,
    driven: bool,
}

/// 📮️ What a driven slot hands the runtime at publication: the gesture it was admitted on, the gesture it decided (when it
/// changed) and the press it closed.
struct GestureSlotDecision<M> {
    admitted: Option<GestureState<M>>,
    decided: Option<Option<GestureState<M>>>,
    closing: Option<String>,
}

impl<M> GestureSlot<M> {
    /// 🕳️ The slot of a dispatch outside a live window (a preview, a view without command authority): it holds no gesture
    /// and nothing it decides is kept.
    pub fn detached() -> Self {
        Self::of(String::new(), String::new(), None, None)
    }

    fn of(window: String, base_revision: String, admitted: Option<GestureState<M>>, closed: Option<String>) -> Self {
        Self { window, base_revision, state: std::sync::Mutex::new(GestureSlotState { admitted, closed, decided: None, closing: None, driven: false }) }
    }

    /// 🪟️ The window this slot belongs to.
    pub fn window(&self) -> &str {
        &self.window
    }

    fn take(&self) -> Option<GestureSlotDecision<M>> {
        let mut state = self.state.lock().ok()?;
        std::mem::take(&mut state.driven).then(|| GestureSlotDecision { admitted: state.admitted.take(), decided: state.decided.take(), closing: state.closing.take() })
    }

    /// 🔦️ The window's gesture as this dispatch sees it: what it decided so far, else the gesture the window held when the
    /// dispatch was admitted.
    pub fn open(&self) -> Option<GestureState<M>>
    where
        M: Clone,
    {
        let state = self.state.lock().ok()?;
        match &state.decided {
            Some(decided) => decided.clone(),
            None => state.admitted.clone(),
        }
    }

    /// 🚃️ Drives the window's streamed tool `T` through ONE dispatch of `verb` ([`semio_framework_tool_machine::drive_press`])
    /// against this slot — a second drive of the same dispatch continues from the first — and answers the transaction it
    /// committed (publish it as ONE edit: [`gesture_emit`]). `press` is the host press the dispatch names (the verb's
    /// `gesture` argument; `None`: none): a dispatch of the press the window last closed is dropped with zero trace, a
    /// dispatch of another press interrupts the open gesture first. A tool that is not base-bound pins its gesture to no revision. A
    /// refused start or tick is the dispatch's fault (`toolTransaction.closed` | `toolTransaction.unclosed`) and decides nothing.
    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(&self, press: Option<&str>, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str) -> Result<Option<(protocol::TransactionRef, Vec<M>)>, Fault>
    where
        M: PartialEq,
    {
        let mut state = self.state.lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("toolTransaction.slot-poisoned"), format!("the gesture slot of window {:?} is poisoned", self.window)))?;
        let held = match state.decided.as_ref() {
            Some(decided) => decided.as_ref(),
            None => state.admitted.as_ref(),
        };
        let closed = state.closing.as_deref().or(state.closed.as_deref());
        let drive = semio_framework_tool_machine::drive_press::<T, M>(held, closed, press, verb, phase, tick, authoring_seed, if T::BASE_BOUND { self.base_revision.as_str() } else { "" })
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("gesture tool {verb:?} refused its dispatch")))?;
        state.driven = true;
        if let Some(decided) = drive.next {
            state.decided = Some(decided);
        }
        if let Some(press) = drive.closed {
            state.closing = Some(press);
        }
        Ok(drive.committed)
    }
}

/// 📤️ What one gesture dispatch publishes: its committed transaction as ONE edit stamped with the ref — plainly when the
/// dispatch carries no admission (`authoring_seed` empty: a render or test view without command authority) — or nothing.
pub fn gesture_emit<Mutation, ConfigMutation, DraftMutation>(committed: Option<(protocol::TransactionRef, Vec<Mutation>)>, authoring_seed: &str) -> Emit<Mutation, ConfigMutation, DraftMutation> {
    match committed {
        Some((transaction, mutations)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    }
}

/// 📡️ The gesture fact a host event is (law table `hostEvents` of the tool-machine gesture-drive corpus).
pub(super) fn gesture_host_event(event: &HostEvent) -> GestureHostEvent {
    match event {
        HostEvent::WindowBlurred { .. } => GestureHostEvent::Blur,
        HostEvent::PointerCaptureLost { .. } => GestureHostEvent::CaptureLost,
        HostEvent::UtilityChanged { .. } => GestureHostEvent::UtilityChanged,
        HostEvent::Retiring { .. } => GestureHostEvent::Retiring,
        HostEvent::TimeTravelFrozen { .. } => GestureHostEvent::TimeTravelFrozen,
        HostEvent::BaseMoved { .. } => GestureHostEvent::BaseMoved,
    }
}
//#endregion 🖐️Gesture

//#region 🔖️Runtime
/// 🎚️ One leaf of a press — every lane of its emit (design §13.1, §12, §20.1): an absolute document leaf, an owned child's
/// share, an app-config or a window-config mutation. Every lane rides the window's ONE scrub, so ONE ledger step decides
/// them all: a tick holds them as the press's provisional overlay, the release commits them as ONE transaction, a late input
/// of a closed press and a refused input leave every lane as it was, a host abort drops them all with zero trace.
#[derive(Clone)]
enum PressLeaf<M, CM> {
    Member(M),
    Child(ChildEmit),
    Config(CM),
    WindowConfig(WindowConfigMutation),
}

impl<M, CM> PressLeaf<M, CM> {
    fn member(&self) -> Option<&M> {
        match self {
            Self::Member(leaf) => Some(leaf),
            _ => None,
        }
    }

    fn config(&self) -> Option<&CM> {
        match self {
            Self::Config(leaf) => Some(leaf),
            _ => None,
        }
    }

    fn window_config(&self) -> Option<&WindowConfigMutation> {
        match self {
            Self::WindowConfig(leaf) => Some(leaf),
            _ => None,
        }
    }
}

/// 🗂️ The instance's continuous-tool runtime: the per-window press ledger (every lane of a press as [`PressLeaf`]s),
/// [`GestureLedger`] (each window's ONE gesture slot, the slots admitted dispatches hold until they publish, and whether a
/// host fact ended a gesture since the last repaint) and [`TypingLedger`], the committed ⊕ provisional overlays every render seam reads while a press or run is open — the
/// document, the app config and each window's config —, the tag of the dispatch being admitted, the tags of admitted
/// operations until their completion publishes, and the logical tick that makes every typing clock unique.
pub struct ToolMachineRuntime<P, M, C = NoConfig, CM = NoConfigMutation> {
    presses: ScrubLedger<PressLeaf<M, CM>>,
    gestures: GestureLedger<M>,
    gesture_operations: Vec<(u64, Arc<GestureSlot<M>>)>,
    gesture_ended: bool,
    typing: TypingLedger<M>,
    overlay: Option<Arc<P>>,
    overlay_generation: u64,
    provisional_generation: u64,
    config_overlay: Option<Arc<C>>,
    config_overlay_generation: u64,
    window_overlays: BTreeMap<String, WindowConfigSnapshot>,
    pub(super) ingress: Option<ToolTag>,
    operations: Vec<(u64, ToolTag)>,
    tick: u64,
    #[cfg(any(test, feature = "artifact-app-testing"))]
    now_ms: Option<u64>,
}

impl<P, M, C, CM> Default for ToolMachineRuntime<P, M, C, CM> {
    fn default() -> Self {
        Self {
            presses: ScrubLedger::default(),
            gestures: GestureLedger::default(),
            gesture_operations: Vec::new(),
            gesture_ended: false,
            typing: TypingLedger::default(),
            overlay: None,
            overlay_generation: 0,
            provisional_generation: 0,
            config_overlay: None,
            config_overlay_generation: 0,
            window_overlays: BTreeMap::new(),
            ingress: None,
            operations: Vec::new(),
            tick: 0,
            #[cfg(any(test, feature = "artifact-app-testing"))]
            now_ms: None,
        }
    }
}

impl<P, M: Mutation<P> + 'static, C, CM: Mutation<C> + 'static> ToolMachineRuntime<P, M, C, CM> {
    /// 🔎️ The open typing runs.
    pub fn typing(&self) -> &TypingLedger<M> {
        &self.typing
    }

    /// 🖐️ Every window's open gesture.
    pub fn gestures(&self) -> &GestureLedger<M> {
        &self.gestures
    }

    /// 🪞️ The document every render seam reads: `committed` with every open press's, gesture's and run's provisional leaves, else
    /// `committed`. A command, a poll and a context menu keep deciding over what landed.
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<P>) -> &'a Arc<P> {
        self.overlay.as_ref().unwrap_or(committed)
    }

    /// 🧾️ Every open press's and run's provisional leaves in the overlay's fold order, in value form — what a derived view
    /// (a preview evaluated by a retained job) folds onto the committed document it reads; empty while nothing is open.
    pub fn provisional_values(&self) -> Vec<DslValue> {
        let typing = self.typing.windows().filter_map(|window| self.typing.open(window)).flat_map(|state| state.entries.iter().map(|(_, leaf)| leaf));
        self.presses.provisional().filter_map(PressLeaf::member).chain(self.gestures.provisional()).chain(typing).map(semio_framework_value::ToValue::to_value).collect()
    }

    /// 🔢️ Bumped whenever the overlay is refolded or dropped (a tick, a release, a host abort, a moved base), so a derived
    /// view knows its provisional input changed even when no edit landed.
    pub fn provisional_generation(&self) -> u64 {
        self.provisional_generation
    }

    /// ⏰️ The clock of one typing input: the wall clock with a logical tick unique to this instance, so two runs opened in
    /// the same millisecond never mint the same transaction id.
    fn clock(&mut self) -> HybridLogicalTimestamp {
        self.tick = self.tick.wrapping_add(1);
        #[cfg(any(test, feature = "artifact-app-testing"))]
        if let Some(physical_ms) = self.now_ms {
            return HybridLogicalTimestamp { actor: 0, physical_ms, logical: self.tick };
        }
        semio_framework_tool_machine::authoring_clock(self.tick)
    }

    /// 🏷️ Keeps `tag` for the admitted `operation` until its completion publishes; the oldest tag of an operation that
    /// never completed (a superseded latest-wins tick) yields its place.
    pub(super) fn bind(&mut self, operation: u64, tag: ToolTag) {
        if self.operations.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            self.operations.remove(0);
        }
        self.operations.push((operation, tag));
    }

    fn take_operation(&mut self, operation: u64) -> Option<ToolTag> {
        let index = self.operations.iter().position(|(owner, _)| *owner == operation)?;
        Some(self.operations.remove(index).1)
    }

    /// 🪞️ Refolds the overlay on `committed` (store generation `generation`) when a press or run changed or the document
    /// moved under one. Scrub leaves are absolute and fold on any base (a leaf the base refuses is skipped); a typing run
    /// whose leaves the moved base refuses is a conflict and aborts with zero trace (`baseMoved`). Answers every snapshot
    /// alias the refold displaced (the caller retires them through the store, never plainly) and every aborted run.
    fn follow(&mut self, committed: &Arc<P>, generation: u64, changed: bool) -> (Vec<Arc<P>>, Vec<ToolStep<M>>) {
        if self.presses.provisional().all(|leaf| leaf.member().is_none()) && self.gestures.is_empty() && self.typing.is_empty() {
            let dropped: Vec<Arc<P>> = self.overlay.take().into_iter().collect();
            self.provisional_generation = self.provisional_generation.wrapping_add(u64::from(!dropped.is_empty()));
            return (dropped, Vec::new());
        }
        if !changed && self.overlay.is_some() && self.overlay_generation == generation {
            return (Vec::new(), Vec::new());
        }
        let mut displaced = Vec::new();
        let mut running: Option<Arc<P>> = None;
        for leaf in self.presses.provisional().filter_map(PressLeaf::member).chain(self.gestures.provisional()) {
            fold_leaf(committed, &mut running, &mut displaced, leaf);
        }
        let mut conflicts = Vec::new();
        for window in self.typing.windows() {
            let state = self.typing.open(window).expect("a listed run is open");
            let checkpoint = running.clone();
            if !state.entries.iter().all(|(_, leaf)| fold_leaf(committed, &mut running, &mut displaced, leaf)) {
                displaced.extend(std::mem::replace(&mut running, checkpoint));
                conflicts.push(window.to_string());
            }
        }
        let aborted = conflicts.iter().map(|window| self.typing.abort(window, ToolAbortReason::BaseMoved)).collect();
        displaced.extend(self.overlay.replace(running.unwrap_or_else(|| Arc::clone(committed))));
        self.overlay_generation = generation;
        self.provisional_generation = self.provisional_generation.wrapping_add(1);
        (displaced, aborted)
    }

    /// 🪞️ The app config every render seam reads: committed with every held press's config mutations, else `committed`.
    pub fn config_overlay_or<'a>(&'a self, committed: &'a Arc<C>) -> &'a Arc<C> {
        self.config_overlay.as_ref().unwrap_or(committed)
    }

    /// 🪞️ The window config a render seam reads for `committed`'s window: the held presses' window-config mutations folded
    /// on that very base, else `committed` (a base that moved is refolded on the next refresh).
    pub fn window_config_overlay_or<'a>(&'a self, committed: &'a WindowConfigSnapshot) -> &'a WindowConfigSnapshot {
        self.window_overlays.get(committed.window_id()).filter(|overlay| overlay.window_kind_id() == committed.window_kind_id() && overlay.generation() == committed.generation()).unwrap_or(committed)
    }

    /// 🪞️ Refolds the app-config overlay on `committed` (config generation `generation`) when a held press changed or the
    /// config moved under one; answers the displaced aliases (retired by the caller through the config store).
    fn follow_config(&mut self, committed: &Arc<C>, generation: u64, changed: bool) -> Vec<Arc<C>> {
        if self.presses.provisional().all(|leaf| leaf.config().is_none()) {
            return self.config_overlay.take().into_iter().collect();
        }
        if !changed && self.config_overlay.is_some() && self.config_overlay_generation == generation {
            return Vec::new();
        }
        let (mut running, mut displaced) = (None, Vec::new());
        for leaf in self.presses.provisional().filter_map(PressLeaf::config) {
            fold_leaf(committed, &mut running, &mut displaced, leaf);
        }
        displaced.extend(self.config_overlay.replace(running.unwrap_or_else(|| Arc::clone(committed))));
        self.config_overlay_generation = generation;
        self.provisional_generation = self.provisional_generation.wrapping_add(1);
        displaced
    }
}
//#endregion 🔖️Runtime

//#region 🔖️Driver
impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧪️ Pins the continuous tools' clock (`None`: the wall clock again), so a law types, pauses and lapses deterministically
    /// however slowly its machine runs.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn set_tool_clock_ms(&mut self, now_ms: Option<u64>) {
        self.tool_machines.now_ms = now_ms;
    }

    /// 🧪️ The document every render seam reads now: the committed head with every open press's and typing run's provisional
    /// leaves.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn rendered_snapshot(&self) -> Arc<A::Snapshot> {
        let committed = self.store.snapshot_owner();
        Arc::clone(self.tool_machines.overlay_or(&committed))
    }

    /// 🧪️ The committed generations of the config lanes — the app config, and window `window_id` of kind `window_kind_id` once
    /// its partition exists — so a law counts the config edits a press published.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn config_generations(&self, window_kind_id: &str, window_id: &str) -> (u64, Option<u64>) {
        (self.config_store.generation(), self.window_config_store.snapshot(window_kind_id, window_id).map(|snapshot| snapshot.generation()))
    }

    /// 🧪️ The app config every render seam reads now: the committed config with every held press's config mutations.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn rendered_config(&self) -> Arc<A::Config> {
        Arc::clone(self.tool_machines.config_overlay_or(&self.config_store.snapshot_owner()))
    }

    /// 🧪️ The `TransactionRef` each edit of the log carries, oldest first (`None`: an edit no tool transaction authored) — a
    /// law counts edits and reads which run or press authored each.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn edit_transactions(&self) -> Vec<Option<protocol::TransactionRef>> {
        self.store.envelope().vcs.edits.iter().map(|edit| edit.mutation_meta.first().and_then(|meta| meta.transaction.clone())).collect()
    }

    /// 🛠️ Reads a dispatch's continuous-tool arguments. A typing delivery (`typing`) folds into its window's run: a commit
    /// signal (`typingCommit`) publishes the run as ONE edit and never reaches the app, an edit while a history edit freezes
    /// the document drops the run and is refused `timeTravel.frozen`, every other edit runs the verb with its operation
    /// tagged. A continuous control (`gesture`, `commit`, `abort`): a host abort drops the window's press with zero trace and
    /// never reaches the app, a frozen press is dropped and refused, every other press runs the verb tagged. Windows that left
    /// the roster retire their presses and commit their runs first.
    pub(super) async fn admit_tool_dispatch(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> ToolDispatch {
        let text = |key: &str| args.and_then(|args| args.get(key)).and_then(DslValue::as_str);
        let window = meta.view_state.as_ref().and_then(|view| view.window_id.clone()).unwrap_or_default();
        if let Some(phase) = TypingPhase::parse(text(TYPING_BUFFER_ARG), text(TYPING_COMMIT_ARG)) {
            if let Err(fault) = Box::pin(self.retire_tool_windows(meta)).await {
                return ToolDispatch::Settled(Err(fault));
            }
            let clock = self.tool_machines.clock();
            let (step, fault) = match phase {
                TypingPhase::Commit { reason, .. } => (self.tool_machines.typing.commit(&window, reason, clock), None),
                TypingPhase::Edit { .. } if self.time_travel.freezes_local_emits() => (Ok(self.tool_machines.typing.abort(&window, ToolAbortReason::Frozen)), Some(time_travel_frozen_fault(action))),
                TypingPhase::Edit { buffer } => {
                    let tool = format!("{}#{action}", self.app.instance_id().await);
                    return ToolDispatch::Tagged(ToolTag::Typing(TypingTag { window, tool, buffer }));
                }
            };
            let published = match step {
                Ok(step) => Box::pin(self.publish_typing_commits(vec![step], meta)).await,
                Err(refusal) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("typing run of window {window:?} refused its commit"))),
            };
            return ToolDispatch::Settled(match (published, fault) {
                (Err(fault), _) | (Ok(()), Some(fault)) => Err(fault),
                (Ok(()), None) => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await),
            });
        }
        let Some(phase) = ScrubPhase::parse(text(SCRUB_GESTURE_ARG), args.and_then(|args| args.get(SCRUB_COMMIT_ARG)).and_then(DslValue::as_bool), text(SCRUB_ABORT_ARG)) else {
            return ToolDispatch::Plain;
        };
        if let Err(fault) = Box::pin(self.retire_tool_windows(meta)).await {
            return ToolDispatch::Settled(Err(fault));
        }
        let (reason, fault) = match &phase {
            ScrubPhase::Abort { reason, .. } => (*reason, None),
            _ if self.time_travel.freezes_local_emits() => (ToolAbortReason::Frozen, Some(time_travel_frozen_fault(action))),
            _ => {
                let tool = format!("{}#{action}", self.app.instance_id().await);
                return ToolDispatch::Tagged(ToolTag::Scrub(ScrubTag { window, tool, phase }));
            }
        };
        self.tool_machines.presses.abort(&window, Some(phase.gesture()), reason);
        self.follow_tool_machines(true);
        ToolDispatch::Settled(match fault {
            Some(fault) => Err(fault),
            None => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await),
        })
    }

    /// ⌨️ Before any dispatch reaches its verb: every typing run idle past its deadline commits, and every run commits before
    /// another verb (`otherVerb`) — so the verb, an undo or a history edit sees the typed text, and the history lists the run
    /// before the verb. A typing delivery of its own and a view verb (a caret report, a hover, a scroll) leave the runs open:
    /// a host ends its run on a caret jump with its own commit signal.
    pub(super) async fn commit_typing_before(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<(), Fault> {
        if self.tool_machines.typing.is_empty() {
            return Ok(());
        }
        let clock = self.tool_machines.clock();
        let mut steps = self.tool_machines.typing.lapse(clock);
        let typing = args.and_then(|args| args.get(TYPING_BUFFER_ARG)).and_then(DslValue::as_str).is_some_and(|buffer| !buffer.is_empty());
        let view = self.registry.get(action).is_some_and(|definition| definition.kind == ActionKind::View);
        if !typing && !view {
            steps.extend(self.tool_machines.typing.commit_all(TypingCommit::OtherVerb, clock));
        }
        let steps = steps.into_iter().map(|(window, step)| step.map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("typing run of window {window:?} refused its commit")))).collect::<Result<Vec<_>, _>>()?;
        Box::pin(self.publish_typing_commits(steps, meta)).await
    }

    /// 💾️ Publishes every committed run as ONE document edit stamped with its `TransactionRef` (no
    /// description: the history row is labelled from the run's net leaves), through the verb that typed it.
    async fn publish_typing_commits(&mut self, steps: Vec<ToolStep<A::Mutation>>, meta: &ActionMeta) -> Result<(), Fault> {
        let mut published = false;
        for step in steps {
            if let ToolStep::Committed(transaction, mutations) = step {
                let verb = transaction.tool.rsplit_once('#').map_or(transaction.tool.as_str(), |(_, verb)| verb).to_string();
                Box::pin(self.dispatch_emit(&verb, Emit::commit_transaction(transaction, mutations), meta)).await?;
                published = true;
            }
        }
        if published {
            self.follow_tool_machines(true);
        }
        Ok(())
    }

    /// 🪦️ `retired`: the press of every window the dispatching view's roster no longer lists leaves zero trace; its typing
    /// run commits like a blur.
    async fn retire_tool_windows(&mut self, meta: &ActionMeta) -> Result<(), Fault> {
        let Some(view) = meta.view_state.as_ref().filter(|view| !view.window_instances.is_empty()) else { return Ok(()) };
        let keep = |window: &str| window.is_empty() || view.window_instances.iter().any(|instance| instance.id == window);
        let retired = !self.tool_machines.presses.retain_windows(keep).is_empty();
        let clock = self.tool_machines.clock();
        let committed = self.tool_machines.typing.retain_windows(keep, clock).into_iter().filter_map(|(_, step)| step.ok()).collect::<Vec<_>>();
        if retired {
            self.follow_tool_machines(true);
        }
        Box::pin(self.publish_typing_commits(committed, meta)).await
    }

    /// 🧊️ `frozen`: a history edit opens, so every open press and every run still open leaves zero trace (a run normally
    /// commits before the history verb reaches here).
    pub(super) fn freeze_tool_machines(&mut self) {
        let presses = self.tool_machines.presses.abort_all(ToolAbortReason::Frozen);
        let runs = self.tool_machines.typing.abort_all(ToolAbortReason::Frozen);
        if !presses.is_empty() || !runs.is_empty() {
            self.follow_tool_machines(true);
        }
    }

    /// 🎟️ The gesture slot of the dispatch `meta` admits as `operation` on the document revision `base`: the gesture its
    /// window holds now, bound to the operation until its completion publishes (the oldest slot of an operation that never
    /// completed yields its place). Windows that left the dispatching view's roster retire their gestures first.
    pub(super) fn admit_gesture_slot(&mut self, operation: u64, base: &[u8; 32], meta: &ActionMeta) -> Arc<GestureSlot<A::Mutation>> {
        let view = meta.view_state.as_ref();
        if let Some(view) = view.filter(|view| !view.window_instances.is_empty()) {
            let retired = self.tool_machines.gestures.retain_windows(|window| window.is_empty() || view.window_instances.iter().any(|instance| instance.id == window));
            if !retired.is_empty() {
                self.tool_machines.gesture_ended = true;
                self.follow_tool_machines(true);
            }
        }
        let window = view.and_then(|view| view.window_id.clone()).unwrap_or_default();
        let base_revision = base.iter().map(|byte| format!("{byte:02x}")).collect();
        let open = self.tool_machines.gestures.open(&window).cloned();
        let closed = self.tool_machines.gestures.closed(&window).map(str::to_string);
        let slot = Arc::new(GestureSlot::of(window, base_revision, open, closed));
        if self.tool_machines.gesture_operations.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            self.tool_machines.gesture_operations.remove(0);
        }
        self.tool_machines.gesture_operations.push((operation, Arc::clone(&slot)));
        slot
    }

    /// 🖋️ Keeps what `operation`'s dispatch decided for its window's gesture, now that it publishes: only while no history
    /// edit freezes the document and the window still holds the gesture the dispatch was admitted on (a host fact or another
    /// dispatch that moved the slot since wins). Answers whether the dispatch drove its slot at all.
    pub(super) fn settle_gesture_slot(&mut self, operation: u64, publishes: bool) -> bool {
        let Some(index) = self.tool_machines.gesture_operations.iter().position(|(owner, _)| *owner == operation) else { return false };
        let slot = self.tool_machines.gesture_operations.remove(index).1;
        let Some(decision) = slot.take() else { return false };
        if publishes && !self.time_travel.freezes_local_emits() && self.tool_machines.gestures.open(slot.window()) == decision.admitted.as_ref() {
            if let Some(press) = decision.closing {
                self.tool_machines.gestures.close(slot.window(), press);
            }
            if let Some(decided) = decision.decided {
                self.tool_machines.gestures.settle(slot.window(), decided);
                self.follow_tool_machines(true);
            }
        }
        true
    }

    /// 🛎️ A host fact of one window: its open gesture ends with the fact's reason and zero trace — the ONE place a blur, a
    /// lost capture, a utility switch, a closing window, a moved base or a history edit ends a gesture; no editor maps it.
    pub(super) fn end_window_gesture(&mut self, event: &HostEvent) {
        if matches!(self.tool_machines.gestures.host_event(event.window_id(), gesture_host_event(event)), ToolStep::Aborted(..)) {
            self.tool_machines.gesture_ended = true;
            self.follow_tool_machines(true);
        }
    }

    /// 🌐️ An instance-wide host fact (a history edit froze the document, a remote edit moved the base): every window's
    /// open gesture ends with the fact's reason and zero trace, whether or not a view lists the window.
    pub(super) fn end_every_gesture(&mut self, event: &HostEvent) {
        if !self.tool_machines.gestures.host_event_all(gesture_host_event(event)).is_empty() {
            self.tool_machines.gesture_ended = true;
            self.follow_tool_machines(true);
        }
    }

    /// 🎨️ Whether a host fact ended a gesture since the last ask: the windows that previewed it owe a repaint.
    pub(super) fn take_gesture_ended(&mut self) -> bool {
        std::mem::take(&mut self.tool_machines.gesture_ended)
    }

    /// 🪞️ Refolds the tool overlays — the document on the committed head, the app config on the committed config, every held
    /// window config on its window's committed partition — retires every displaced alias through its store (a snapshot with
    /// retire-owned roots panics on a plain drop of its last owner), and drops every run the moved head conflicts with.
    pub(super) fn follow_tool_machines(&mut self, changed: bool) {
        let machines = &self.tool_machines;
        if machines.presses.is_empty() && machines.gestures.is_empty() && machines.typing.is_empty() && machines.overlay.is_none() && machines.config_overlay.is_none() && machines.window_overlays.is_empty() {
            return;
        }
        let committed = self.store.snapshot_owner();
        let generation = self.store.generation();
        let (displaced, _aborted) = self.tool_machines.follow(&committed, generation, changed);
        for alias in displaced {
            retire_overlay_alias(self.store.retire_snapshot_alias(alias));
        }
        let committed_config = self.config_store.snapshot_owner();
        for alias in self.tool_machines.follow_config(&committed_config, self.config_store.generation(), changed) {
            retire_overlay_alias(self.config_store.retire_snapshot_alias(alias));
        }
        let mut targets: BTreeMap<String, (String, Vec<&WindowConfigMutation>)> = BTreeMap::new();
        for mutation in self.tool_machines.presses.provisional().filter_map(PressLeaf::window_config) {
            targets.entry(mutation.window_id().to_string()).or_insert_with(|| (mutation.window_kind_id().to_string(), Vec::new())).1.push(mutation);
        }
        let stale: Vec<String> = self.tool_machines.window_overlays.keys().filter(|window| !targets.contains_key(*window)).cloned().collect();
        for window in stale {
            if let Some(overlay) = self.tool_machines.window_overlays.remove(&window) {
                self.window_config_store.retire_preview(overlay);
            }
        }
        for (window, (kind, mutations)) in targets {
            let committed_generation = self.window_config_store.snapshot(&kind, &window).map(|snapshot| snapshot.generation());
            let current = self.tool_machines.window_overlays.get(&window).map(|overlay| overlay.generation());
            if !changed && current.is_some() && current == committed_generation {
                continue;
            }
            let preview = self.window_config_store.preview(&kind, &window, &mutations);
            let displaced = match preview {
                Some(preview) => self.tool_machines.window_overlays.insert(window, preview),
                None => self.tool_machines.window_overlays.remove(&window),
            };
            if let Some(displaced) = displaced {
                self.window_config_store.retire_preview(displaced);
            }
        }
    }

    /// 🎚️ Rides every lane of `emit` — its artifact leaves, its owned children's shares (design §12), its app-config and
    /// window-config mutations (§20.1) — on `tag`'s press as ONE scrub opened on the document revision `base` by `actor`. A
    /// tick holds them as the press's provisional overlay and publishes nothing; the release commits them as ONE transaction
    /// and hands every lane back to `emit` — the document lanes stamped with the press's `TransactionRef` (a ref the verb
    /// minted itself never outlives the press), the config lanes as ONE config edit that is never a history row; a late input
    /// of the press the window already closed stays silent on every lane. One ledger step decides every lane, so a refused
    /// input leaves the press, its overlays and its held config exactly as they were until a retry or a host abort decides.
    /// Answers whether `emit` publishes the press.
    pub(super) fn settle_press<D>(&mut self, tag: &ScrubTag, base: &[u8], actor: &str, emit: &mut Emit<A::Mutation, A::ConfigMutation, D>) -> Result<bool, Fault> {
        let base: String = base.iter().map(|byte| format!("{byte:02x}")).collect();
        let leaves = std::mem::take(&mut emit.artifact_mutations)
            .into_iter()
            .map(PressLeaf::Member)
            .chain(std::mem::take(&mut emit.child_emits).into_iter().map(PressLeaf::Child))
            .chain(std::mem::take(&mut emit.config_mutations).into_iter().map(PressLeaf::Config))
            .chain(std::mem::take(&mut emit.window_config_mutations).into_iter().map(PressLeaf::WindowConfig))
            .collect();
        emit.transaction = None;
        let step = self
            .tool_machines
            .presses
            .send(&tag.window, &tag.tool, &ActorId(actor.to_string()), &base, tag.phase.clone().input(leaves), semio_framework_tool_machine::authoring_clock(0))
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("continuous control {:?} refused its press", tag.tool)))?;
        let ToolStep::Committed(transaction, leaves) = step else { return Ok(false) };
        for leaf in leaves {
            match leaf {
                PressLeaf::Member(leaf) => emit.artifact_mutations.push(leaf),
                PressLeaf::Child(child) => emit.child_emits.push(child),
                PressLeaf::Config(config) => emit.config_mutations.push(config),
                PressLeaf::WindowConfig(window_config) => emit.window_config_mutations.push(window_config),
            }
        }
        if !emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() {
            emit.transaction = Some(transaction);
        }
        Ok(true)
    }

    /// 🛠️ The ONE point an operation's completion becomes its tools' publication. The gesture slot its dispatch drove follows
    /// first ([`Self::settle_gesture_slot`]); a dispatch that only advanced or ended its gesture logs no history row. A tagged
    /// operation then settles its press or run: a press settles every lane of its emit on
    /// the window's press ([`Self::settle_press`]) on the operation's document revision. A typed edit: its leaves fold into
    /// the window's run; nothing publishes while the run is open, and a run the edit ended (idle lapse, caret jump, another
    /// buffer) publishes in this emit as ONE edit stamped with its own `TransactionRef`. An emit that carries no transaction
    /// logs no history row; every other lane of the emit (effects, events, UI scope) publishes as usual.
    pub(super) fn settle_tool_operation(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, publication: &mut ArtifactToolCompletionValue<A>) -> Result<(), Fault> {
        let operation = mounted.operation.operation.0;
        let driven = self.settle_gesture_slot(operation, matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _)));
        let tag = self.tool_machines.take_operation(operation);
        let ArtifactToolCompletionValue::Emit(Ok(emit), _) = publication else { return Ok(()) };
        if driven && emit.transaction.is_none() && emit.artifact_mutations.is_empty() {
            mounted.command_logged = true;
        }
        let Some(tag) = tag else { return Ok(()) };
        match tag {
            ToolTag::Scrub(tag) => {
                self.settle_press(&tag, &mounted.canonical_revision, &mounted.meta.actor, emit)?;
            }
            ToolTag::Typing(tag) => {
                let leaves = std::mem::take(&mut emit.artifact_mutations);
                let clock = self.tool_machines.clock();
                let steps = self
                    .tool_machines
                    .typing
                    .send(&tag.window, &tag.tool, &ActorId(mounted.meta.actor.clone()), TypingInput::Edit { buffer: tag.buffer, leaves }, A::typing_fold, clock)
                    .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("typing run {:?} refused its edit", tag.tool)))?;
                if let Some((transaction, mutations)) = steps.into_iter().find_map(|step| if let ToolStep::Committed(transaction, mutations) = step { Some((transaction, mutations)) } else { None }) {
                    emit.artifact_mutations = mutations;
                    emit.transaction = Some(transaction);
                }
            }
        }
        if emit.transaction.is_none() {
            mounted.command_logged = true;
        }
        self.follow_tool_machines(true);
        Ok(())
    }
}
//#endregion 🔖️Driver

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🧪️scrub/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../🧪️tests/🧪️typing/🦀️.rs"]
mod typing_tests;
//#endregion 🧪️Tests
