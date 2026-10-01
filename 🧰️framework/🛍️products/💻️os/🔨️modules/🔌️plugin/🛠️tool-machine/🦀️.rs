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
//! Every render seam reads the committed document overlaid with every open press's and run's provisional leaves; every
//! committed transaction is published stamped with its `TransactionRef`. Domain-neutral machines:
//! `semio_framework_tool_machine` (`ScrubMachine`, `TypingMachine`).

use super::*;
use semio_framework_tool_machine::{ScrubLedger, ScrubPhase, ToolAbortReason, ToolStep, TypingCommit, TypingInput, TypingLedger, TypingPhase, SCRUB_ABORT_ARG, SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG, TYPING_BUFFER_ARG, TYPING_COMMIT_ARG};
use std::sync::Arc;

/// 🧹️ The grant of one retirement step of a displaced overlay alias and the turns a refold spends; a refold retires its
/// aliases to their terminal-empty witness, the cost a plain drop would have paid (a step that hands the last alias to its
/// owned-value disposer releases nothing yet, so a zero step is progress, never the end).
const TOOL_OVERLAY_RETIREMENT_ITEMS: usize = 4_096;
const TOOL_OVERLAY_RETIREMENT_BYTES: usize = 1 << 24;
const TOOL_OVERLAY_RETIREMENT_TURNS: usize = 1 << 20;

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

//#region 🔖️Runtime
/// 🗂️ The instance's continuous-tool runtime: the per-window [`ScrubLedger`] and [`TypingLedger`], the committed ⊕
/// provisional overlay every render seam reads while a press or run is open, the tag of the dispatch being admitted, the tags
/// of admitted operations until their completion publishes, and the logical tick that makes every typing clock unique.
pub struct ToolMachineRuntime<P, M> {
    scrubs: ScrubLedger<M>,
    child_scrubs: ScrubLedger<ChildEmit>,
    typing: TypingLedger<M>,
    overlay: Option<Arc<P>>,
    overlay_generation: u64,
    provisional_generation: u64,
    pub(super) ingress: Option<ToolTag>,
    operations: Vec<(u64, ToolTag)>,
    tick: u64,
    #[cfg(any(test, feature = "artifact-app-testing"))]
    now_ms: Option<u64>,
}

impl<P, M> Default for ToolMachineRuntime<P, M> {
    fn default() -> Self {
        Self {
            scrubs: ScrubLedger::default(),
            child_scrubs: ScrubLedger::default(),
            typing: TypingLedger::default(),
            overlay: None,
            overlay_generation: 0,
            provisional_generation: 0,
            ingress: None,
            operations: Vec::new(),
            tick: 0,
            #[cfg(any(test, feature = "artifact-app-testing"))]
            now_ms: None,
        }
    }
}

impl<P, M: Mutation<P> + 'static> ToolMachineRuntime<P, M> {
    /// 🔎️ The open presses.
    pub fn scrubs(&self) -> &ScrubLedger<M> {
        &self.scrubs
    }

    /// 🪆️ The open presses' owned-child shares (design §12): absolute child leaves the release publishes with the press's
    /// own leaves in ONE transaction; they are never overlaid on the parent document.
    pub fn child_scrubs(&self) -> &ScrubLedger<ChildEmit> {
        &self.child_scrubs
    }

    /// 🔎️ The open typing runs.
    pub fn typing(&self) -> &TypingLedger<M> {
        &self.typing
    }

    /// 🪞️ The document every render seam reads: `committed` with every open press's and run's provisional leaves, else
    /// `committed`. A command, a poll and a context menu keep deciding over what landed.
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<P>) -> &'a Arc<P> {
        self.overlay.as_ref().unwrap_or(committed)
    }

    /// 🧾️ Every open press's and run's provisional leaves in the overlay's fold order, in value form — what a derived view
    /// (a preview evaluated by a retained job) folds onto the committed document it reads; empty while nothing is open.
    pub fn provisional_values(&self) -> Vec<DslValue> {
        let typing = self.typing.windows().filter_map(|window| self.typing.open(window)).flat_map(|state| state.entries.iter().map(|(_, leaf)| leaf));
        self.scrubs.provisional().chain(typing).map(protocol::ToValue::to_value).collect()
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
        HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: self.tick }
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
        if self.scrubs.is_empty() && self.typing.is_empty() {
            let dropped: Vec<Arc<P>> = self.overlay.take().into_iter().collect();
            self.provisional_generation = self.provisional_generation.wrapping_add(u64::from(!dropped.is_empty()));
            return (dropped, Vec::new());
        }
        if !changed && self.overlay.is_some() && self.overlay_generation == generation {
            return (Vec::new(), Vec::new());
        }
        let mut displaced = Vec::new();
        let mut running: Option<Arc<P>> = None;
        let fold = |leaf: &M, running: &mut Option<Arc<P>>, displaced: &mut Vec<Arc<P>>| -> bool {
            let base: &P = running.as_deref().unwrap_or(committed.as_ref());
            let outcome = leaf.diff(base);
            let applicable = outcome.is_applicable(protocol::MergePolicy::default());
            let (diff, _) = outcome.into_parts();
            let applied = applicable.then(|| diff.apply(base));
            MutationDiff::retire_cold(diff);
            match applied {
                None => false,
                Some(Ok(next)) => {
                    displaced.extend(running.replace(Arc::new(next)));
                    true
                }
                Some(Err(_)) => false,
            }
        };
        for leaf in self.scrubs.provisional() {
            fold(leaf, &mut running, &mut displaced);
        }
        let mut conflicts = Vec::new();
        for window in self.typing.windows() {
            let state = self.typing.open(window).expect("a listed run is open");
            let checkpoint = running.clone();
            if !state.entries.iter().all(|(_, leaf)| fold(leaf, &mut running, &mut displaced)) {
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
        self.tool_machines.scrubs.abort(&window, Some(phase.gesture()), reason);
        self.tool_machines.child_scrubs.abort(&window, Some(phase.gesture()), reason);
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

    /// 💾️ Publishes every committed run as ONE document edit stamped with its `TransactionRef` (no coalesce key, no
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
        let retired = !self.tool_machines.scrubs.retain_windows(keep).is_empty();
        self.tool_machines.child_scrubs.retain_windows(keep);
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
        let presses = self.tool_machines.scrubs.abort_all(ToolAbortReason::Frozen);
        self.tool_machines.child_scrubs.abort_all(ToolAbortReason::Frozen);
        let runs = self.tool_machines.typing.abort_all(ToolAbortReason::Frozen);
        if !presses.is_empty() || !runs.is_empty() {
            self.follow_tool_machines(true);
        }
    }

    /// 🪞️ Refolds the tool overlay on the committed head, retires every displaced snapshot alias through the store (a
    /// snapshot with retire-owned roots panics on a plain drop of its last owner), and drops every run the moved head
    /// conflicts with.
    pub(super) fn follow_tool_machines(&mut self, changed: bool) {
        if self.tool_machines.scrubs.is_empty() && self.tool_machines.typing.is_empty() && self.tool_machines.overlay.is_none() {
            return;
        }
        let committed = self.store.snapshot_owner();
        let generation = self.store.generation();
        let (displaced, _aborted) = self.tool_machines.follow(&committed, generation, changed);
        for alias in displaced {
            let Ok(mut retirement) = self.store.retire_snapshot_alias(alias) else { continue };
            for _ in 0..TOOL_OVERLAY_RETIREMENT_TURNS {
                if retirement.terminal_is_empty() || retirement.close_step(TOOL_OVERLAY_RETIREMENT_ITEMS, retirement.next_close_byte_demand().max(TOOL_OVERLAY_RETIREMENT_BYTES)).is_err() {
                    break;
                }
            }
        }
    }

    /// 🛠️ The ONE point a tagged operation's completion becomes its publication. A press: the emit's artifact leaves and its
    /// owned-child leaves ride the window's press on the operation's document revision; a tick or a press that settles empty
    /// publishes no edit and logs no history row, the release publishes its committed leaves as ONE edit per touched member,
    /// every one stamped with the press's `TransactionRef` (design §12) — a ref the verb minted itself never outlives the press
    /// that owns the transaction. A typed edit: its leaves fold into the window's run; nothing publishes while the run is open, and a
    /// run the edit ended (idle lapse, caret jump, another buffer) publishes in this emit as ONE edit stamped with its own
    /// `TransactionRef`. Every other lane of the emit (config, effects, events, UI scope) publishes as usual.
    pub(super) fn settle_tool_operation(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, publication: &mut ArtifactToolCompletionValue<A>) -> Result<(), Fault> {
        let Some(tag) = self.tool_machines.take_operation(mounted.operation.operation.0) else { return Ok(()) };
        let ArtifactToolCompletionValue::Emit(Ok(emit), _) = publication else { return Ok(()) };
        let leaves = std::mem::take(&mut emit.artifact_mutations);
        let actor = ActorId(mounted.meta.actor.clone());
        let committed = match tag {
            ToolTag::Scrub(tag) => {
                let base: String = mounted.canonical_revision.iter().map(|byte| format!("{byte:02x}")).collect();
                let clock = HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 };
                let refused = |refusal: semio_framework_tool_machine::ToolRefusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("continuous control {:?} refused its press", tag.tool));
                let children = std::mem::take(&mut emit.child_emits);
                emit.transaction = None;
                let step = self.tool_machines.scrubs.send(&tag.window, &tag.tool, &actor, &base, tag.phase.clone().input(leaves), clock).map_err(refused)?;
                let child_step = self.tool_machines.child_scrubs.send(&tag.window, &tag.tool, &actor, &base, tag.phase.clone().input(children), clock).map_err(refused)?;
                if let ToolStep::Committed(transaction, children) = child_step {
                    emit.child_emits = children;
                    emit.transaction = Some(transaction);
                    emit.coalesce_key = None;
                    emit.description = None;
                }
                vec![step]
            }
            ToolTag::Typing(tag) => {
                let clock = self.tool_machines.clock();
                self.tool_machines
                    .typing
                    .send(&tag.window, &tag.tool, &actor, TypingInput::Edit { buffer: tag.buffer, leaves }, A::typing_fold, clock)
                    .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("typing run {:?} refused its edit", tag.tool)))?
            }
        };
        match committed.into_iter().find_map(|step| if let ToolStep::Committed(transaction, mutations) = step { Some((transaction, mutations)) } else { None }) {
            Some((transaction, mutations)) => {
                emit.artifact_mutations = mutations;
                emit.transaction = Some(transaction);
                emit.coalesce_key = None;
                emit.description = None;
            }
            None if emit.transaction.is_none() => mounted.command_logged = true,
            None => {}
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
