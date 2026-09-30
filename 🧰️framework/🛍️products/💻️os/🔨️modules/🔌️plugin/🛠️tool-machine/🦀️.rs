//! 🎚️ OS runtime of the continuous-control tool (design §13.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every
//! dispatch whose args carry a `gesture` rides the dispatching window's press in the instance's [`ScrubLedger`] —
//! window transient, ephemeral, local-only, never history. A tick publishes nothing (every render seam overlays the
//! committed document with the provisional leaves), the release publishes ONE edit stamped with the press's
//! `TransactionRef`, and every host abort leaves zero trace. Apps supply only the ABSOLUTE leaves of a value: their
//! verb's `Emit::mutations`. Domain-neutral machine: `semio_framework_tool_machine` (`ScrubMachine`, `Scrub`).

use super::*;
use semio_framework_tool_machine::{ScrubLedger, ScrubPhase, ToolAbortReason, ToolStep, SCRUB_ABORT_ARG, SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG};
use std::sync::Arc;

/// 🧹️ The grant of one retirement step of a displaced overlay alias; a refold retires its aliases to completion, the
/// cost a plain drop would have paid.
const SCRUB_RETIREMENT_ITEMS: usize = 4_096;
const SCRUB_RETIREMENT_BYTES: usize = 1 << 24;

//#region 🔖️Tag
/// 🏷️ One continuous-control dispatch as its operation carries it to publication: the dispatching window, the tool
/// `<appId>#<verb>` and where the dispatch sits in its press.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrubTag {
    pub window: String,
    pub tool: String,
    pub phase: ScrubPhase,
}

/// 🚦️ How `dispatch_action` proceeds once it read the scrub arguments: a plain dispatch, a press whose operation
/// carries the tag, or a dispatch the scrub runtime settled itself (a host abort, a frozen press).
pub(super) enum ScrubDispatch {
    Plain,
    Press(ScrubTag),
    Settled(Result<InvocationResult, Fault>),
}
//#endregion 🔖️Tag

//#region 🔖️Runtime
/// 🗂️ The instance's scrub runtime: the per-window [`ScrubLedger`], the committed ⊕ provisional overlay every render
/// seam reads while a press is open, the tag of the dispatch being admitted, and the tags of admitted operations until
/// their completion publishes.
pub struct ScrubRuntime<P, M> {
    ledger: ScrubLedger<M>,
    overlay: Option<Arc<P>>,
    overlay_generation: u64,
    pub(super) ingress: Option<ScrubTag>,
    operations: Vec<(u64, ScrubTag)>,
}

impl<P, M> Default for ScrubRuntime<P, M> {
    fn default() -> Self {
        Self { ledger: ScrubLedger::default(), overlay: None, overlay_generation: 0, ingress: None, operations: Vec::new() }
    }
}

impl<P, M: Mutation<P> + 'static> ScrubRuntime<P, M> {
    /// 🔎️ The open presses.
    pub fn ledger(&self) -> &ScrubLedger<M> {
        &self.ledger
    }

    /// 🪞️ The document every render seam reads: `committed` with every open press's provisional leaves, else
    /// `committed`. A command, a poll and a context menu keep deciding over what landed.
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<P>) -> &'a Arc<P> {
        self.overlay.as_ref().unwrap_or(committed)
    }

    /// 🏷️ Keeps `tag` for the admitted `operation` until its completion publishes; the oldest tag of an operation that
    /// never completed (a superseded latest-wins tick) yields its place.
    pub(super) fn bind(&mut self, operation: u64, tag: ScrubTag) {
        if self.operations.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            self.operations.remove(0);
        }
        self.operations.push((operation, tag));
    }

    fn take_operation(&mut self, operation: u64) -> Option<ScrubTag> {
        let index = self.operations.iter().position(|(owner, _)| *owner == operation)?;
        Some(self.operations.remove(index).1)
    }

    /// 🪞️ Refolds the overlay on `committed` (store generation `generation`) when a press changed or the document moved
    /// under an open press — the leaves are absolute, so they fold on any base; a leaf the base refuses is skipped.
    /// Answers every snapshot alias the refold displaced: the caller retires them through the store, never plainly.
    fn follow(&mut self, committed: &Arc<P>, generation: u64, changed: bool) -> Vec<Arc<P>> {
        if self.ledger.is_empty() {
            return self.overlay.take().into_iter().collect();
        }
        if !changed && self.overlay.is_some() && self.overlay_generation == generation {
            return Vec::new();
        }
        let mut displaced = Vec::new();
        let mut running: Option<Arc<P>> = None;
        for leaf in self.ledger.provisional() {
            let base: &P = running.as_deref().unwrap_or(committed.as_ref());
            let outcome = leaf.diff(base);
            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                continue;
            }
            if let Ok(next) = outcome.diff().apply(base) {
                displaced.extend(running.replace(Arc::new(next)));
            }
        }
        displaced.extend(self.overlay.replace(running.unwrap_or_else(|| Arc::clone(committed))));
        self.overlay_generation = generation;
        displaced
    }
}
//#endregion 🔖️Runtime

//#region 🔖️Driver
impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🎚️ Reads a dispatch's scrub arguments (`gesture`, `commit`, `abort`). A host abort drops the window's press with
    /// zero trace and never reaches the app; a press while a history edit freezes the document is dropped and refused
    /// `timeTravel.frozen`; every other press runs the verb with its operation tagged. Windows that left the roster
    /// retire their presses first.
    pub(super) async fn admit_scrub_dispatch(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> ScrubDispatch {
        let text = |key: &str| args.and_then(|args| args.get(key)).and_then(DslValue::as_str);
        let Some(phase) = ScrubPhase::parse(text(SCRUB_GESTURE_ARG), args.and_then(|args| args.get(SCRUB_COMMIT_ARG)).and_then(DslValue::as_bool), text(SCRUB_ABORT_ARG)) else {
            return ScrubDispatch::Plain;
        };
        let window = meta.view_state.as_ref().and_then(|view| view.window_id.clone()).unwrap_or_default();
        self.retire_scrub_windows(meta);
        let (reason, fault) = match &phase {
            ScrubPhase::Abort { reason, .. } => (*reason, None),
            _ if self.time_travel.freezes_local_emits() => (ToolAbortReason::Frozen, Some(time_travel_frozen_fault(action))),
            _ => {
                let tool = format!("{}#{action}", self.app.instance_id().await);
                return ScrubDispatch::Press(ScrubTag { window, tool, phase });
            }
        };
        self.scrubs.ledger.abort(&window, Some(phase.gesture()), reason);
        self.follow_scrubs(true);
        ScrubDispatch::Settled(match fault {
            Some(fault) => Err(fault),
            None => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await),
        })
    }

    /// 🪦️ `retired`: the press of every window the dispatching view's roster no longer lists leaves zero trace.
    fn retire_scrub_windows(&mut self, meta: &ActionMeta) {
        let Some(view) = meta.view_state.as_ref().filter(|view| !view.window_instances.is_empty()) else { return };
        if !self.scrubs.ledger.retain_windows(|window| window.is_empty() || view.window_instances.iter().any(|instance| instance.id == window)).is_empty() {
            self.follow_scrubs(true);
        }
    }

    /// 🧊️ `frozen`: a history edit opens, so every open press leaves zero trace.
    pub(super) fn freeze_scrubs(&mut self) {
        if !self.scrubs.ledger.abort_all(ToolAbortReason::Frozen).is_empty() {
            self.follow_scrubs(true);
        }
    }

    /// 🪞️ Refolds the scrub overlay on the committed head and retires every displaced snapshot alias through the store (a
    /// snapshot with retire-owned roots panics on a plain drop of its last owner).
    pub(super) fn follow_scrubs(&mut self, changed: bool) {
        if self.scrubs.ledger.is_empty() && self.scrubs.overlay.is_none() {
            return;
        }
        let committed = self.store.snapshot_owner();
        let generation = self.store.generation();
        for alias in self.scrubs.follow(&committed, generation, changed) {
            let Ok(mut retirement) = self.store.retire_snapshot_alias(alias) else { continue };
            while let Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) = retirement.close_step(SCRUB_RETIREMENT_ITEMS, retirement.next_close_byte_demand().max(SCRUB_RETIREMENT_BYTES)) {
                if released_items == 0 && released_bytes == 0 {
                    break;
                }
            }
        }
    }

    /// 🎚️ The ONE point a tagged operation's completion becomes its publication: the emit's artifact leaves ride the
    /// window's press on the operation's document revision. A tick or a press that settles empty publishes no artifact
    /// edit and logs no history row; the release publishes its committed leaves as ONE edit stamped with the press's
    /// `TransactionRef` (no coalesce key, no description: the row is labelled from the leaves). Every other lane of the
    /// emit (config, effects, events, UI scope) publishes as usual.
    pub(super) fn settle_scrub_operation(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, publication: &mut ArtifactToolCompletionValue<A>) -> Result<(), Fault> {
        let Some(tag) = self.scrubs.take_operation(mounted.operation.operation.0) else { return Ok(()) };
        let ArtifactToolCompletionValue::Emit(Ok(emit), _) = publication else { return Ok(()) };
        let base: String = mounted.canonical_revision.iter().map(|byte| format!("{byte:02x}")).collect();
        let clock = HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 };
        let input = tag.phase.clone().input(std::mem::take(&mut emit.artifact_mutations));
        let step = self
            .scrubs
            .ledger
            .send(&tag.window, &tag.tool, &ActorId(mounted.meta.actor.clone()), &base, input, clock)
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("continuous control {:?} refused its press", tag.tool)))?;
        match step {
            ToolStep::Committed(transaction, mutations) => {
                emit.artifact_mutations = mutations;
                emit.transaction = Some(transaction);
                emit.coalesce_key = None;
                emit.description = None;
            }
            ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => mounted.command_logged = true,
        }
        self.follow_scrubs(true);
        Ok(())
    }
}
//#endregion 🔖️Driver

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🧪️scrub/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
