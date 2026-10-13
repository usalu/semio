//! ⏯️ OS runtime of an interactive, transactional tool run with a visible process: the per-instance
//! [`ToolRunLedger`] (provisional document ops folded into an overlay the renderer reads as the document), the
//! [`ToolRunDriver`] turn stepping run and revalidate jobs, the one-edit finalize publication and the
//! framework ToolRun panel body.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md`
//! §2.7, §2.8, §3.3, §4.1 (layers 1 and 3). Domain-neutral types: `semio_framework_tool_run`.
use crate::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use crate::tool_run_contract::ToolRunJobPurpose;
use crate::tool_run_contract::ToolRunJob;
use crate::tool_run_contract::ToolRunTraceKeys;
use crate::tool_run_contract::ToolRunRetargetableJob;
use crate::tool_run_contract::ToolRunJobPort;
use crate::tool_run_contract::ToolRunView;

use super::*;
use semio_framework_tool_run::{
    tool_run_format, ToolRunAction, ToolRunCounter, ToolRunDefinition, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLabel, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy,
    ToolRunRejection, ToolRunSlot, ToolRunState, ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceOp, ToolRunTraceStore, ToolRunVerdict,
    TOOL_RUN_ACTION_IDS, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_MEMBER_OPS_MAX, TOOL_RUN_PROVISIONAL_OPS_MAX, TOOL_RUN_REASON_PROVISIONAL_CAP,
    TOOL_RUN_REASON_TRACE_TRUNCATED, TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS,
};
use semio_framework_ui_scene::{scene_lane_hash, Board2dScene, Board2dSceneLane, Canvas2dScene, Canvas2dSceneLane, SceneDoc, SceneLaneRef, World3dScene, World3dSceneLane};
use std::collections::BTreeMap;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
/// ⏱️ Wall budget of one driver turn; every job step inside it keeps its own interactive-lane slice.
pub const TOOL_RUN_TURN_WALL_US: u64 = 4_000;
/// 🧮️ Folded ops between two wall-clock checks of a bounded refold.
pub const TOOL_RUN_REFOLD_CHECK_OPS: usize = 64;
/// 🧹️ Discarded provisional ops cold-retired per driver or close turn.
pub const TOOL_RUN_DISCARD_OPS_PER_TURN: usize = 4_096;
/// 📜️ Newest step-log rows the panel shows.
pub const TOOL_RUN_PANEL_STEP_ROWS: usize = 16;
/// 🧾️ Newest trace records the panel's keyboard trace list shows.
pub const TOOL_RUN_PANEL_TRACE_ROWS: usize = 32;
/// 📦️ Encoded trace delta bytes one renderer refresh receives.
pub const TOOL_RUN_TRACE_DELTA_BYTES: usize = 262_144;
/// 🔁️ Refreshes one window may re-request a trace backlog while its renderer's echoed cursor stands still;
/// a renderer that never echoes therefore costs a bounded number of refreshes, never a refresh loop.
pub const TOOL_RUN_TRACE_STALL_REFRESHES: u8 = 4;
pub(crate) const TOOL_RUN_JOB_SITE: &str = "tool-run.step";
#[path="🎟️publication/🦀️.rs"]
mod publication_ingress;
use publication_ingress::ToolRunPublicationIngress;
//#endregion 🔖️Limits

//#region 🔖️Selection
/// 🎯️ The run the ledger currently operates on (`ToolRunLedger::selected`), borrowing only `entries`.
macro_rules! selected_entry {
    ($ledger:expr) => {
        $ledger.selected.and_then(|index| $ledger.entries.get(index)).map(|entry| &**entry)
    };
}

/// 🎯️ The run the ledger currently operates on, mutably, borrowing only `entries`.
macro_rules! selected_entry_mut {
    ($ledger:expr) => {
        match $ledger.selected {
            Some(index) => $ledger.entries.get_mut(index).map(|entry| &mut **entry),
            None => None,
        }
    };
}
use selected_entry;
use selected_entry_mut;

#[path="🎞️tick/📦️payload/🦀️.rs"]
mod original_tick_payload;
#[cfg(test)] use original_tick_payload::{advance_original_tick_payload,close_original_tick_payload};

#[cfg(test)]
#[path="📮️port/♻️retirement/🧪️tests/🦀️.rs"]
mod original_port_retirement_tests;
#[cfg(test)]
#[path="📮️port/🎟️admission/🧪️tests/🦀️.rs"]
mod original_port_admission_tests;

/// 🔖️ True for the seven framework-reserved tool run action ids (§2.5).
pub fn is_tool_run_action_id(action: &str) -> bool {
    TOOL_RUN_ACTION_IDS.contains(&action)
}
//#endregion 🔖️Request

//#region 🔖️Job
pub(crate) enum ToolRunJobHandle<C> {
    Plain(ToolRunJob),
    Retargetable(Box<dyn ToolRunRetargetableJob<C>>),
}

pub(crate) struct ToolRunJobSlot<C> {
    job: ToolRunJobHandle<C>,
    purpose: ToolRunJobPurpose,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    cancel: Option<semio_framework_job::CancelToken>,
    cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    preview_sequence: u64,
    context: Option<semio_framework_job::StepContextOwner>,
    closing: bool,
}

impl<C> ToolRunJobSlot<C> {
    pub(crate) fn new(job: ToolRunJobHandle<C>, purpose: ToolRunJobPurpose, generation: u32) -> Self {
        Self { job, purpose, operation: semio_framework_job::allocate_operation_id(), generation: semio_framework_job::Generation(u64::from(generation)), cancel: Some(semio_framework_job::CancelToken::root_now()), cancel_retirement: None, preview_sequence: 0, context: None, closing: false }
    }

    fn interactive(&mut self) -> &mut dyn semio_framework_job::InteractiveJob {
        match &mut self.job {
            ToolRunJobHandle::Plain(job) => job.as_mut(),
            ToolRunJobHandle::Retargetable(job) => job.as_mut(),
        }
    }

    fn job_is_empty(&self) -> bool {
        match &self.job {
            ToolRunJobHandle::Plain(job) => job.terminal_is_empty(),
            ToolRunJobHandle::Retargetable(job) => job.terminal_is_empty(),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.job_is_empty() && self.context.as_ref().is_none_or(semio_framework_job::StepContextOwner::terminal_is_empty) && self.cancel.is_none() && self.cancel_retirement.is_none()
    }

    /// 📐️ Bytes of the boxed job frame once every owner inside it is returned: the last allocation a retiring slot still holds.
    fn scaffold_release_bytes(&self) -> usize {
        match &self.job {
            ToolRunJobHandle::Plain(job) => size_of_val(job.as_ref()),
            ToolRunJobHandle::Retargetable(job) => size_of_val(job.as_ref()),
        }
    }

    fn retargetable(&mut self) -> Option<&mut dyn ToolRunRetargetableJob<C>> {
        match &mut self.job {
            ToolRunJobHandle::Retargetable(job) if !self.closing => Some(job.as_mut()),
            _ => None,
        }
    }

    fn begin_close(&mut self) {
        if !self.closing {
            if let Some(cancel) = self.cancel.as_ref() {
                cancel.cancel_now();
            }
            self.interactive().begin_close();
            self.closing = true;
        }
    }

    fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let job:&dyn semio_framework_job::InteractiveJob=match &self.job{ToolRunJobHandle::Plain(job)=>job.as_ref(),ToolRunJobHandle::Retargetable(job)=>job.as_ref()};
        if self.closing&&job.terminal_is_empty(){
            if let Some(owner)=self.context.as_ref(){return Ok(RetirementDemand{release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?,..Default::default()});}
            if let Some(retirement)=self.cancel_retirement.as_ref(){return if retirement.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_async::CancelTokenRetirement>>(),depth:1,..Default::default()})}else{retirement.retirement_demands()};}
            if self.cancel.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job::CancelToken>>(),depth:1,..Default::default()});}
        }
        if !self.closing||job.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"a terminal tool job slot is removed by its ledger, never quoted"));}
        Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool job close depth overflow"))?})
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.next_demands(grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?;
        if !tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.job_is_empty(){if let Some(owner)=self.context.as_mut(){return match owner.close_step(grant){
            semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>{if owner.terminal_is_empty(){self.context=None;}Ok(RetainedCloneStep::Progress(progress))},
            semio_framework_job::InteractiveJobCloseStep::Blocked=>Ok(RetainedCloneStep::Progress(Default::default())),
            semio_framework_job::InteractiveJobCloseStep::Refused{kind,progress}=>Err(ValueError::literal(kind,"tool job step context original close refused its admitted grant").with_retained_progress(progress).into_fault()),
        };}
        if let Some(retirement)=self.cancel_retirement.as_mut(){
            if retirement.terminal_is_empty(){drop(self.cancel_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            return match retirement.close_step(grant){
                Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),
                Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(RetainedCloneStep::Progress(Default::default())),
                Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error.into_fault()),
            };
        }
        if let Some(token)=self.cancel.take(){self.cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(token));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        }
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        let terminal=self.job_is_empty();
        let step=self.interactive().close_step(child).admit(child,terminal);
        match step{
            semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>Ok(RetainedCloneStep::Progress(progress)),
            semio_framework_job::InteractiveJobCloseStep::Blocked=>Ok(RetainedCloneStep::Progress(Default::default())),
            semio_framework_job::InteractiveJobCloseStep::Refused{kind,progress}=>Err(ValueError::literal(kind,"tool job original close refused its admitted grant").with_retained_progress(progress).into_fault()),
        }
    }
}

pub(crate) fn job_payload_bytes(payload: &semio_framework_job::RetainedJobPayload, limit: usize) -> Option<Vec<u8>> {
    if payload.len() > limit {
        return None;
    }
    let mut bytes = Vec::with_capacity(payload.len());
    for index in 0..payload.page_count() {
        bytes.extend_from_slice(payload.page(index)?);
    }
    Some(bytes)
}

/// 📼️ Lane text of one delta: `None` for an empty, non-clearing delta, else its base64url (unpadded) pack body.
fn tool_run_trace_lane_text(delta: &ToolRunTraceDelta) -> Option<String> {
    if !delta.clear && delta.pages.is_empty() {
        return None;
    }
    delta.encode().ok().map(base64_codec::base64_url_encode)
}

/// 🔎️ The first World3d, Canvas2d or Board2d scene surface of a rendered body, depth first.
pub(crate) fn tool_run_scene_surface(node: &mut BuiltNode) -> Option<&mut BuiltNode> {
    let is_scene = matches!(&node.component, Component::Surface(props) if [World3dScene::SCHEMA, Canvas2dScene::SCHEMA, Board2dScene::SCHEMA].contains(&props.doc_schema.as_str()));
    if is_scene {
        return Some(node);
    }
    node.children.iter_mut().find_map(tool_run_scene_surface)
}

/// 📎️ Re-attaches `lane` to a scene surface: the spine's lane refs gain the `toolRunTrace` entry and the
/// surface gains its paged carrier, exactly as `SceneDoc::split_lanes` plus `scene_surface` would emit it.
/// A surface whose producer already carries the lane is left untouched.
pub(crate) fn inject_tool_run_trace_lane_into(surface: &mut BuiltNode, lane: &str) -> UiAssemblyResult<()> {
    let Component::Surface(props) = &mut surface.component else { return Ok(()) };
    let (name, key) = match props.doc_schema.as_str() {
        World3dScene::SCHEMA => (World3dSceneLane::ToolRunTrace.name(), World3dSceneLane::ToolRunTrace.body_key()),
        Board2dScene::SCHEMA => (Board2dSceneLane::ToolRunTrace.name(), Board2dSceneLane::ToolRunTrace.body_key()),
        _ => (Canvas2dSceneLane::ToolRunTrace.name(), Canvas2dSceneLane::ToolRunTrace.body_key()),
    };
    if surface.children.iter().any(|child| child.key.as_str() == key) {
        return Ok(());
    }
    let reference = SceneLaneRef { lane: name.to_string(), bytes: lane.len() as u32, hash: scene_lane_hash(lane) };
    let encoded = match props.doc_schema.as_str() {
        World3dScene::SCHEMA => {
            let mut spine: World3dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
        Board2dScene::SCHEMA => {
            let mut spine: Board2dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
        _ => {
            let mut spine: Canvas2dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
    };
    let mut encoded = encoded.map_err(|error| ui_assembly_error_because("tool-run-trace.encode", error))?;
    let carrier = paged_text_carrier(key, lane)?;
    if surface.children.len() == UI_BUILT_CHILDREN_MAX {
        return Err(ui_assembly_error("tool-run-trace.carrier"));
    }
    encoded.bindings = std::mem::take(&mut props.bindings);
    *props = encoded;
    surface.children.try_push(carrier).map_err(|_| ui_assembly_error("tool-run-trace.carrier"))
}

//#endregion 🔖️Job

//#region 🔖️Ledger
/// 🔁️ A bounded refold of the provisional ops from `base`. `boundary` is whether the job reached a point where
/// the refolded overlay is whole: a retract refold waits for the job's next checkpoint, completion or close,
/// so a retract followed by re-appends spread over several ticks never renders a half re-appended overlay —
/// the previous overlay stays rendered until then.
struct ToolRunRefold<P> {
    running: Option<P>,
    cursor: usize,
    boundary: bool,
}

/// 🚦️ Where one refold turn left the overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ToolRunRefoldTurn {
    Pending,
    AwaitingBoundary,
    Swapped,
}

/// 🎚️ The values a run's declared settings reads named when last observed: config pointers, then per declared
/// window kind the `(window id, values)` of every partition it reads.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ToolRunSettingsValues {
    config: Vec<Option<DslValue>>,
    window_config: Vec<(String, String, Vec<Option<DslValue>>)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ToolRunFinalizePhase {
    Pending,
    Revalidating,
    Publishing,
    Closing,
}

pub(crate) struct ToolRunFinalize<A: ArtifactApp> {
    phase: ToolRunFinalizePhase,
    pub(crate) publication: Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    pub(crate) ingress: ToolRunPublicationIngress,
    rejected: bool,
    retracted: u32,
    published: bool,
}

pub(crate) struct ToolRunEntry<A: ArtifactApp> {
    pub(crate) tool_id: String,
    actor: semio_framework_value::SharedUtf8,
    pub(crate) definition: ToolRunDefinition,
    pub(crate) slot: ToolRunSlot,
    pub(crate) identity: ToolRunIdentity,
    pub(crate) base: Arc<A::Snapshot>,
    pub(crate) base_generation: u64,
    pub(crate) settings_generation: (u64, u64),
    settings_values: ToolRunSettingsValues,
    pub(crate) window: Option<(String, String)>,
    window_bodies: Vec<String>,
    payload: Option<Arc<Vec<u8>>>,
    pub(crate) trace_keys: ToolRunTraceKeys,
    pub(crate) provisional: Vec<A::Mutation>,
    pub(crate) entity_marks: Vec<(u32, u64)>,
    entities: Arc<BTreeSet<u64>>,
    overlay: Arc<A::Snapshot>,
    refold: Option<ToolRunRefold<A::Snapshot>>,
    trace: ToolRunTraceStore,
    recent_trace: std::collections::VecDeque<u64>,
    stage: u16,
    completed: u64,
    total: Option<u64>,
    counters: Vec<ToolRunCounter>,
    steps: ToolRunStepRing,
    step_sequence: u64,
    sequence: u64,
    conflicts: u32,
    rate_origin: Option<(u64, u64)>,
    units_per_second: f32,
    job: Option<ToolRunJobSlot<A::Config>>,
    pub(crate) checkpoint: Option<Vec<u8>>,
    pending_step: bool,
    pub(crate) finalize: Option<ToolRunFinalize<A>>,
    announced: Option<(ToolRunState, u64, String)>,
    pub(crate) port: ToolRunJobPort,
    /// ♻️ Snapshot aliases this run let go (a replaced base or overlay, a superseded fold result), owed to the store's
    /// alias retirement ([`ToolRunLedger::retire_step`]); never dropped plainly.
    displaced: Vec<Arc<A::Snapshot>>,
    /// 🧩️ The owned member a member run edits (`ToolRunDefinition::member`); `None` for a run on the document's store.
    pub(crate) member: Option<ToolRunMemberRun>,
}

impl<A: ArtifactApp> ToolRunEntry<A> {
    /// 🛣️ The lane this run occupies: the mutating lane, or its tool on its starting window when read-only.
    pub(crate) fn lane(&self) -> ToolRunLane {
        ToolRunLane { mutating: self.definition.mutating, tool_id: self.tool_id.clone(), window_id: if self.definition.mutating { None } else { self.window.as_ref().map(|(window_id, _)| window_id.clone()) } }
    }

    fn view(&self) -> ToolRunView {
        ToolRunView { tool_id: self.tool_id.clone(), identity: self.identity, state: self.slot.state, provisional_entities: Arc::clone(&self.entities), progress: self.progress(), payload: self.payload.clone() }
    }

    fn progress(&self) -> semio_framework_tool_run::ToolRunProgress {
        semio_framework_tool_run::ToolRunProgress {
            identity: self.identity,
            sequence: self.sequence,
            state: self.slot.state,
            stage: self.stage,
            completed: self.completed,
            total: self.total,
            counters: self.counters.clone(),
            units_per_second: self.units_per_second,
            conflicts: self.conflicts,
            steps: self.steps.clone(),
        }
    }

    /// 🏃️ Whether a driver turn has work for this run; a job waiting on its port is not work.
    fn has_pending_work(&self) -> bool {
        let stepping = self.job.is_none() || !self.port.is_waiting();
        self.port.has_effects()
            || self.refold_is_work()
            || !self.displaced.is_empty()
            || self.member.as_ref().is_some_and(ToolRunMemberRun::is_work)
            || match self.slot.state {
                ToolRunState::Running => stepping,
                ToolRunState::Starting | ToolRunState::Finalizing | ToolRunState::Aborting => true,
                ToolRunState::Paused => self.pending_step && stepping,
                ToolRunState::Complete => false,
                _ => self.job.is_some(),
            }
    }

    fn framework_step(&mut self, kind: ToolRunStepKind, reason: u16, args: &[ToolRunStepArg]) {
        self.step_sequence = self.step_sequence.saturating_add(1);
        self.steps.push(ToolRunStep { sequence: self.step_sequence, kind, stage: self.stage, reason, subject: None, repeat: 1, args: args.to_vec() });
    }

    fn begin_refold(&mut self, boundary: bool) {
        if let Some(running) = self.refold.replace(ToolRunRefold { running: None, cursor: 0, boundary }).and_then(|refold| refold.running) {
            self.displaced.push(Arc::new(running));
        }
    }

    /// ♻️ Replaces the overlay, owing the previous one to the store's alias retirement.
    fn replace_overlay(&mut self, overlay: Arc<A::Snapshot>) {
        let previous = std::mem::replace(&mut self.overlay, overlay);
        self.displaced.push(previous);
    }

    /// ♻️ Replaces the base, owing the previous one to the store's alias retirement.
    fn replace_base(&mut self, base: Arc<A::Snapshot>) {
        let previous = std::mem::replace(&mut self.base, base);
        self.displaced.push(previous);
    }

    /// 🏁️ The job reached a point where the refolded overlay is whole.
    fn settle_refold(&mut self) {
        if let Some(refold) = self.refold.as_mut() {
            refold.boundary = true;
        }
    }

    fn refold_is_work(&self) -> bool {
        self.refold.as_ref().is_some_and(|refold| refold.boundary || refold.cursor < self.provisional.len())
    }

    fn rebind(&mut self) {
        self.identity.generation = self.slot.generation;
        self.trace.rebind(self.identity);
    }

    fn rebuild_entities(&mut self) {
        let length = self.provisional.len() as u32;
        self.entity_marks.retain(|(end, _)| *end <= length);
        self.entities = Arc::new(self.entity_marks.iter().map(|(_, entity)| *entity).collect());
    }

    fn fold_one(snapshot: &A::Snapshot, op: &A::Mutation) -> Option<A::Snapshot> {
        let outcome = op.diff(snapshot);
        if !outcome.is_applicable(protocol::MergePolicy::default()) {
            return None;
        }
        protocol::apply_diff(outcome.diff(), snapshot).ok()
    }

    /// 🔁️ Folds provisional ops from `base` until the wall deadline; the overlay is replaced once every op is
    /// folded and the refold reached its boundary.
    fn refold_turn(&mut self, deadline_us: u64) -> ToolRunRefoldTurn {
        let Some(mut refold) = self.refold.take() else { return ToolRunRefoldTurn::Swapped };
        while refold.cursor < self.provisional.len() {
            let source = refold.running.as_ref().unwrap_or(&self.base);
            match Self::fold_one(source, &self.provisional[refold.cursor]) {
                Some(next) => {
                    if let Some(previous) = refold.running.replace(next) {
                        self.displaced.push(Arc::new(previous));
                    }
                }
                None => self.conflicts = self.conflicts.saturating_add(1),
            }
            refold.cursor += 1;
            if refold.cursor % TOOL_RUN_REFOLD_CHECK_OPS == 0 && semio_framework_job::default_now_us().is_none_or(|now| now >= deadline_us) {
                self.refold = Some(refold);
                return ToolRunRefoldTurn::Pending;
            }
        }
        if !refold.boundary {
            self.refold = Some(refold);
            return ToolRunRefoldTurn::AwaitingBoundary;
        }
        let overlay = refold.running.map_or_else(|| Arc::clone(&self.base), Arc::new);
        self.replace_overlay(overlay);
        ToolRunRefoldTurn::Swapped
    }

    /// 🧬️ Applies one tick of the current identity: retract, append (O(k) fold), entities, steps, progress, trace.
    fn apply_tick(&mut self, mut tick: ToolRunTick, discarded: &mut Vec<A::Mutation>) -> Result<ToolRunTickReceipt, Fault> {
        let mut receipt = ToolRunTickReceipt::default();
        if tick.identity.id != self.identity.id || tick.identity.generation != self.identity.generation {
            receipt.stale = true;
            return Ok(receipt);
        }
        if let Some(payload) = tick.payload.take() {
            self.payload = Some(Arc::new(payload));
        }
        self.sequence = self.sequence.max(tick.sequence);
        if let Some(member) = self.member.as_mut() {
            if let Some(retract) = tick.retract_to.map(|length| length as usize).filter(|length| *length < member.ops.len()) {
                receipt.retracted = (member.ops.len() - retract) as u32;
                member.ops.truncate(retract);
                member.rebase = true;
            }
            let room = (TOOL_RUN_MEMBER_OPS_MAX as usize).saturating_sub(member.ops.len());
            receipt.capped = tick.append_ops.len() > room;
            receipt.appended = tick.append_ops.len().min(room) as u32;
            member.ops.extend(tick.append_ops.drain(..).take(room));
            let length = member.ops.len() as u32;
            self.entity_marks.retain(|(end, _)| *end <= length);
            self.entities = Arc::new(self.entity_marks.iter().map(|(_, entity)| *entity).collect());
        } else if let Some(retract) = tick.retract_to.map(|length| length as usize).filter(|length| *length < self.provisional.len()) {
            receipt.retracted = (self.provisional.len() - retract) as u32;
            discarded.extend(self.provisional.drain(retract..));
            self.rebuild_entities();
            self.begin_refold(false);
        }
        let mut appended = Vec::with_capacity(tick.append_ops.len());
        for bytes in &tick.append_ops {
            if self.provisional.len() + appended.len() >= TOOL_RUN_PROVISIONAL_OPS_MAX as usize {
                receipt.capped = true;
                break;
            }
            appended.push(<A::Mutation as ::protocol::OpBinary>::decode_op(bytes).map_err(|error| error.into_fault())?);
        }
        if self.refold.is_none() && !appended.is_empty() {
            let mut running: Option<A::Snapshot> = None;
            for op in &appended {
                match Self::fold_one(running.as_ref().unwrap_or(&self.overlay), op) {
                    Some(next) => {
                        if let Some(previous) = running.replace(next) {
                            self.displaced.push(Arc::new(previous));
                        }
                    }
                    None => self.conflicts = self.conflicts.saturating_add(1),
                }
            }
            if let Some(running) = running {
                self.replace_overlay(Arc::new(running));
            }
        }
        receipt.appended += appended.len() as u32;
        self.provisional.extend(appended);
        let end = self.member.as_ref().map_or(self.provisional.len(), |member| member.ops.len()) as u32;
        if !tick.append_entities.is_empty() {
            let entities = Arc::make_mut(&mut self.entities);
            for entity in tick.append_entities {
                self.entity_marks.push((end, entity));
                entities.insert(entity);
            }
        }
        for mut step in tick.steps {
            self.step_sequence = self.step_sequence.saturating_add(1);
            step.sequence = self.step_sequence;
            self.steps.push(step);
        }
        if let Some(progress) = tick.progress {
            self.stage = progress.stage;
            self.total = progress.total;
            self.completed = progress.total.map_or(progress.completed, |total| progress.completed.min(total));
            self.counters = progress.counters;
        }
        for page in &tick.trace {
            let applied = self.trace.apply_page(page).map_err(|rejection| Fault::new(FaultOrigin::Framework, FaultCode::new(rejection.code()), "tool run trace page does not belong to the current run generation"))?;
            receipt.evicted = receipt.evicted.saturating_add(applied.evicted);
            receipt.capped |= applied.overflowed > 0;
            for op in &page.ops {
                match op {
                    ToolRunTraceOp::Upsert { key, .. } => {
                        self.trace_keys.observe(*key);
                        self.recent_trace.retain(|recent| recent != key);
                        if self.recent_trace.len() == TOOL_RUN_PANEL_TRACE_ROWS {
                            self.recent_trace.pop_front();
                        }
                        self.recent_trace.push_back(*key);
                    }
                    ToolRunTraceOp::Clear => self.recent_trace.clear(),
                    ToolRunTraceOp::Retire { key } => self.recent_trace.retain(|recent| recent != key),
                }
            }
        }
        if receipt.evicted > 0 {
            self.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_TRACE_TRUNCATED, &[ToolRunStepArg::Unsigned(u64::from(receipt.evicted))]);
        }
        if receipt.capped {
            let cap = if self.member.is_some() { TOOL_RUN_MEMBER_OPS_MAX } else { TOOL_RUN_PROVISIONAL_OPS_MAX };
            self.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_PROVISIONAL_CAP, &[ToolRunStepArg::Unsigned(u64::from(cap))]);
        }
        let now = semio_framework_job::default_now_us().unwrap_or(0);
        match self.rate_origin {
            None => self.rate_origin = Some((now, self.completed)),
            Some((origin_us, origin_completed)) if now > origin_us => self.units_per_second = self.completed.saturating_sub(origin_completed) as f32 * 1_000_000.0 / (now - origin_us) as f32,
            Some(_) => {}
        }
        Ok(receipt)
    }
}

/// 📬️ What one applied tick changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolRunTickReceipt {
    pub stale: bool,
    pub appended: u32,
    pub retracted: u32,
    pub evicted: u32,
    pub capped: bool,
}

/// 🧭️ Driver watch state and turn budget (§3.3): settings publications the store generations cannot see.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolRunDriver {
    pub turn_wall_us: u64,
    pub(crate) window_config_publications: u64,
}

impl Default for ToolRunDriver {
    fn default() -> Self {
        Self { turn_wall_us: TOOL_RUN_TURN_WALL_US, window_config_publications: 0 }
    }
}

/// 🗄️ The tool runs of one document instance for its local actor (§2.2 invariant 5): at most one non-terminal
/// mutating run, and concurrent read-only runs, one per tool and window ([`ToolRunLane`]); plus the owners still
/// retiring.
///
/// 🎯️ The ledger operates on one *selected* run. Outside its own calls the selection is the primary run — the
/// non-terminal mutating run, else the most recent one — so the single-run accessors (`state`, `slot`, `view`, …)
/// answer for it; [`ToolRunLedger::select_run`] addresses another.
pub struct ToolRunLedger<A: ArtifactApp> {
    pub(crate) next_run: u64,
    pub(crate) driver_cursor: usize,
    pub(crate) entries: Vec<Box<ToolRunEntry<A>>>,
    pub(crate) selected: Option<usize>,
    pub(crate) driver: ToolRunDriver,
    pub(crate) retired_jobs: Vec<ToolRunJobSlot<A::Config>>,
    pub(crate) retired_publications: Vec<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    /// ♻️ Snapshot aliases owed to the store's alias retirement, and the one being retired now.
    retired_snapshots: Vec<Arc<A::Snapshot>>,
    pub(crate) snapshot_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pending_snapshot_alias: Option<Arc<A::Snapshot>>,
    pending_discarded: Option<A::Mutation>,
    pub(crate) discarded: Vec<A::Mutation>,
    /// 🧩️ Member runs that left their slot, whose typed owners and composed reads still retire against their member store.
    pub(crate) retired_members: Vec<ToolRunMemberRun>,
    pub(crate) closing: bool,
    pub(crate) trace_windows: BTreeMap<String, ToolRunTraceWindow>,
    pub(crate) ui_dirty: bool,
    pub(crate) document_dirty: bool,
    original_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
}

/// 🪟️ One scene window the trace lane was delivered to: its body key and what its renderer last echoed.
struct ToolRunTraceWindow {
    body_key: String,
    echoed: Option<ToolRunTraceCursor>,
    answered: bool,
    stalls: u8,
}

impl<A: ArtifactApp> Default for ToolRunLedger<A> {
    fn default() -> Self {
        Self {
            next_run: 1,
            driver_cursor: 0,
            entries: Vec::new(),
            selected: None,
            driver: ToolRunDriver::default(),
            retired_jobs: Vec::new(),
            retired_publications: Vec::new(),
            retired_snapshots: Vec::new(),
            snapshot_retirement: None,
            pending_snapshot_alias: None,
            pending_discarded: None,
            discarded: Vec::new(),
            retired_members: Vec::new(),
            closing: false,
            trace_windows: BTreeMap::new(),
            ui_dirty: false,
            document_dirty: false,
            original_receipt: None,
        }
    }
}

impl<A: ArtifactApp> ToolRunLedger<A> {
    /// 🎟️ A retained original receipt stops the next publication callback until transport collects it.
    pub fn has_original_receipt(&self)->bool{self.original_receipt.is_some()}
    /// 🧾️ Transfers the actual caller grant and actual currencies without inventing another operation.
    pub fn take_original_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.original_receipt.take()}
    pub(crate) fn retain_original_receipt(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),Fault>{
        if progress==RetainedCloneProgress::default(){return Ok(());}
        if self.original_receipt.is_some()||!progress.fits(grant)||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"ToolRun original receipt has no unchanged granted transport slot").with_retained_progress(progress).into_fault());}
        self.original_receipt=Some((grant,progress));Ok(())
    }
    /// 👑️ The primary run: the non-terminal mutating run, else the most recent run.
    fn primary_index(&self) -> Option<usize> {
        self.entries.iter().position(|entry| entry.definition.mutating && !entry.slot.state.is_terminal()).or_else(|| self.entries.iter().enumerate().max_by_key(|(_, entry)| entry.slot.run).map(|(index, _)| index))
    }

    /// 👑️ Selects the primary run.
    pub fn select_primary(&mut self) {
        self.selected = self.primary_index();
    }

    /// 🎯️ Selects run `run`; `false` (and no selection) when this instance holds no such run.
    pub fn select_run(&mut self, run: u64) -> bool {
        self.selected = self.entries.iter().position(|entry| entry.slot.run == run);
        self.selected.is_some()
    }

    /// 🗂️ Every run this instance holds, oldest first.
    pub fn views(&self) -> Vec<ToolRunView> {
        self.entries.iter().map(|entry| entry.view()).collect()
    }

    /// 🪟️ The run a window renders: the most recent read-only run started from `window_id`, else the primary run.
    pub fn view_for(&self, window_id: Option<&str>) -> Option<ToolRunView> {
        self.window_index(window_id).or_else(|| self.primary_index()).map(|index| self.entries[index].view())
    }

    fn window_index(&self, window_id: Option<&str>) -> Option<usize> {
        let window_id = window_id?;
        self.entries.iter().enumerate().filter(|(_, entry)| !entry.definition.mutating && entry.window.as_ref().is_some_and(|(id, _)| id == window_id)).max_by_key(|(_, entry)| entry.slot.run).map(|(index, _)| index)
    }

    pub fn state(&self) -> Option<ToolRunState> {
        selected_entry!(self).map(|entry| entry.slot.state)
    }

    pub fn slot(&self) -> Option<ToolRunSlot> {
        selected_entry!(self).map(|entry| entry.slot)
    }

    pub fn identity(&self) -> Option<ToolRunIdentity> {
        selected_entry!(self).map(|entry| entry.identity)
    }

    pub fn tool_id(&self) -> Option<&str> {
        selected_entry!(self).map(|entry| entry.tool_id.as_str())
    }

    /// 📮️ The port every job of the selected run shares with its app instance.
    pub fn port(&self) -> Option<&ToolRunJobPort> {
        selected_entry!(self).map(|entry| &entry.port)
    }

    pub fn provisional(&self) -> &[A::Mutation] {
        selected_entry!(self).map_or(&[], |entry| entry.provisional.as_slice())
    }

    pub fn provisional_entities(&self) -> Option<&BTreeSet<u64>> {
        selected_entry!(self).map(|entry| entry.entities.as_ref())
    }

    pub fn steps(&self) -> Option<&ToolRunStepRing> {
        selected_entry!(self).map(|entry| &entry.steps)
    }

    pub fn trace(&self) -> Option<&ToolRunTraceStore> {
        selected_entry!(self).map(|entry| &entry.trace)
    }

    pub fn conflicts(&self) -> u32 {
        selected_entry!(self).map_or(0, |entry| entry.conflicts)
    }

    pub fn is_refolding(&self) -> bool {
        selected_entry!(self).is_some_and(|entry| entry.refold.is_some())
    }

    /// 📸️ The current run's last `StepOutcome::CheckpointReady` state, the one a rebuilt job continues from.
    pub fn checkpoint(&self) -> Option<&[u8]> {
        selected_entry!(self)?.checkpoint.as_deref()
    }

    /// 🔑️ The trace key allocator every job of the current run shares.
    pub fn trace_keys(&self) -> Option<&ToolRunTraceKeys> {
        selected_entry!(self).map(|entry| &entry.trace_keys)
    }

    /// 🏷️ `(provisional length at the end of the appending tick, entity)` per provisional entity, in append order.
    pub fn entity_marks(&self) -> &[(u32, u64)] {
        selected_entry!(self).map_or(&[], |entry| entry.entity_marks.as_slice())
    }

    /// 🪟️ `(window instance id, window kind id)` the current run was started from.
    pub fn window(&self) -> Option<(&str, &str)> {
        selected_entry!(self)?.window.as_ref().map(|(window_id, window_kind_id)| (window_id.as_str(), window_kind_id.as_str()))
    }

    /// 📊️ Progress snapshot derived from the ledger (§2.3); the step ring is the framework's own.
    pub fn progress(&self) -> Option<semio_framework_tool_run::ToolRunProgress> {
        selected_entry!(self).map(ToolRunEntry::progress)
    }

    /// 🪞️ The document a renderer reads: the mutating run's overlay while it holds provisional state, else `committed`.
    /// Read-only runs author no provisional edit, so their overlays never compose.
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<A::Snapshot>) -> &'a Arc<A::Snapshot> {
        match self.entries.iter().find(|entry| entry.definition.mutating && !entry.slot.state.is_terminal()) {
            Some(entry) if matches!(entry.slot.state, ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing) && !Arc::ptr_eq(&entry.overlay, &entry.base) => &entry.overlay,
            _ => committed,
        }
    }

    /// 🧩️ The composed children a renderer reads (§20.15): the mutating member run's composed read while it holds provisional
    /// state, else `live`. The parent document is untouched by a member run, so [`Self::overlay_or`] answers `committed`.
    pub fn children_or<'a>(&'a self, live: &'a ChildContentView) -> &'a ChildContentView {
        match self.entries.iter().find(|entry| entry.definition.mutating && !entry.slot.state.is_terminal()) {
            Some(entry) if matches!(entry.slot.state, ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing) => entry.member.as_ref().and_then(|member| member.children.as_ref()).unwrap_or(live),
            _ => live,
        }
    }

    /// 🧩️ The `(slot, child id)` of the member the selected run edits, `None` for a run on the document's own store.
    pub fn member(&self) -> Option<(&str, &str)> {
        selected_entry!(self)?.member.as_ref().map(|member| (member.slot.as_str(), member.child_id.as_str()))
    }

    /// 🧩️ The selected member run's provisional ops (that member's `OpBinary` ops); empty for a document run.
    pub fn member_ops(&self) -> &[Vec<u8>] {
        selected_entry!(self).and_then(|entry| entry.member.as_ref()).map_or(&[], |member| member.ops.as_slice())
    }

    pub fn view(&self) -> Option<ToolRunView> {
        selected_entry!(self).map(ToolRunEntry::view)
    }

    /// ⏪️ Whether a non-terminal mutating run holds provisional state on this instance: history editing cannot open then.
    pub fn holds_mutating_run(&self) -> bool {
        self.entries.iter().any(|entry| entry.definition.mutating && !entry.slot.state.is_terminal())
    }

    /// 🧊️ `freeze` rebase policy: local artifact emits fail with `toolRun.busy` while a freezing run is non-terminal.
    pub fn freezes_local_emits(&self) -> bool {
        self.entries.iter().any(|entry| entry.definition.rebase == ToolRunRebasePolicy::Freeze && !entry.slot.state.is_terminal())
    }

    /// 🏃️ Whether a driver turn has work: any run's stepping job, refold, close, finalize or port effects, or cold
    /// retirement. A job waiting on its port is not work.
    pub fn has_pending_work(&self) -> bool {
        !self.retired_jobs.is_empty()
            || !self.retired_publications.is_empty()
            || !self.retired_snapshots.is_empty()
            || self.snapshot_retirement.is_some()
            || self.pending_snapshot_alias.is_some()
            || self.pending_discarded.is_some()
            || !self.discarded.is_empty()
            || !self.retired_members.is_empty()
            || self.entries.iter().any(|entry| entry.has_pending_work())
    }

    /// 🧬️ Applies one decoded tick to the current run (the driver's per-tick O(k) overlay append);
    /// ticks of another run or generation are stale no-ops.
    pub fn apply_tick(&mut self, tick: ToolRunTick) -> Result<ToolRunTickReceipt, Fault> {
        let Some(entry) = selected_entry_mut!(self) else { return Ok(ToolRunTickReceipt { stale: true, ..ToolRunTickReceipt::default() }) };
        entry.apply_tick(tick, &mut self.discarded)
    }

    /// 🪟️ A window-config publication happened; the driver treats it as a settings change.
    pub fn note_window_config_published(&mut self) {
        self.driver.window_config_publications = self.driver.window_config_publications.wrapping_add(1);
    }

    /// 👥️ Ephemeral-shared presence summary (§3.4), `completed ≤ total` by construction.
    pub fn presence(&self) -> Option<protocol::PresenceToolRun> {
        let entry = &self.entries[self.primary_index()?];
        Some(protocol::PresenceToolRun {
            tool_id: entry.tool_id.clone(),
            state: protocol::PresenceToolRunState::from_wire_name(entry.slot.state.as_str())?,
            stage: entry.stage,
            completed: entry.total.map_or(entry.completed, |total| entry.completed.min(total)),
            total: entry.total,
        })
    }

    /// 📼️ Base64url (unpadded) `ToolRunTraceDelta` after `cursor`, for the scene lane `toolRunTrace` (§3.2).
    ///
    /// - A cursor of another run or generation, or none, gets `clear` plus the resend of the live log.
    /// - A caught-up cursor gets `None`, so an idle run costs nothing.
    /// - Without a run (dismissed or retired), a renderer still holding pages (`page > 0`) gets one empty
    ///   `clear` delta; `page == 0` already means an empty layer.
    pub fn trace_delta(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String> {
        let index = cursor.and_then(|cursor| self.entries.iter().position(|entry| entry.slot.run == cursor.run)).or_else(|| self.primary_index());
        let delta = match index.map(|index| &self.entries[index]) {
            Some(entry) => entry.trace.delta_after(cursor, byte_budget),
            None => {
                let cursor = cursor.filter(|cursor| cursor.page > 0)?;
                ToolRunTraceDelta { identity: ToolRunIdentity { id: ToolRunId { app_instance_id: 0, run: cursor.run }, generation: cursor.generation, base_revision: [0; 32] }, clear: true, next: 0, pages: Vec::new() }
            }
        };
        tool_run_trace_lane_text(&delta)
    }

    /// 🪟️ The lane text scene window `window_id` (body `body_key`) receives for its echoed `cursor`, and the
    /// bookkeeping that keeps a delivery backlog moving: pages left after the byte budget mark the ledger
    /// UI-dirty until the renderer's cursor stands still for `TOOL_RUN_TRACE_STALL_REFRESHES` refreshes.
    pub fn trace_lane(&mut self, window_id: &str, body_key: &str, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String> {
        let Some(entry) = self.window_index(Some(window_id)).or_else(|| self.primary_index()).map(|index| &self.entries[index]) else {
            self.trace_windows.remove(window_id);
            return self.trace_delta(cursor, byte_budget);
        };
        let delta = entry.trace.delta_after(cursor, byte_budget);
        let backlog = delta.next < entry.trace.next_page();
        let window = self.trace_windows.entry(window_id.to_string()).or_insert_with(|| ToolRunTraceWindow { body_key: body_key.to_string(), echoed: None, answered: false, stalls: 0 });
        if window.body_key != body_key {
            window.body_key = body_key.to_string();
        }
        window.stalls = if window.answered && window.echoed == cursor { window.stalls.saturating_add(1) } else { 0 };
        window.echoed = cursor;
        window.answered = true;
        if backlog && window.stalls < TOOL_RUN_TRACE_STALL_REFRESHES {
            self.ui_dirty = true;
        }
        tool_run_trace_lane_text(&delta)
    }

    /// 🧹️ Forgets scene windows that closed or stopped hosting a scene surface.
    pub fn retain_trace_windows(&mut self, live: impl Fn(&str) -> bool) {
        self.trace_windows.retain(|window_id, _| live(window_id));
    }

    /// 🎯️ The minimal UI scope a run change dirties: the ToolRun panel body, the body of every scene window its trace
    /// lane rides in and the body of every window kind the run declares it renders in (`ToolRunDefinition::windows`) —
    /// never unrelated windows or panels.
    pub fn dirty_scope(&self) -> UiDirtyScope {
        let mut window_bodies: Vec<String> = self.trace_windows.values().map(|window| window.body_key.clone()).chain(self.entries.iter().flat_map(|entry| entry.window_bodies.iter().cloned())).collect();
        window_bodies.sort_unstable();
        window_bodies.dedup();
        UiDirtyScope::Partial { window_bodies, panel_bodies: vec![FRAMEWORK_TOOL_RUN_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    /// 🚩️ Whether a run change still owes the host a dirty scope.
    pub fn is_ui_dirty(&self) -> bool {
        self.ui_dirty || self.document_dirty
    }

    pub(crate) fn apply_event(&mut self, event: ToolRunEvent) -> Result<ToolRunEffect, ToolRunRejection> {
        let transition = ToolRunMachine::apply(self.slot(), event)?;
        if transition.slot.is_none()&&selected_entry!(self).and_then(|entry|entry.finalize.as_ref()).is_some_and(|finalize|!finalize.ingress.terminal_is_empty()||finalize.publication.is_some()){return Err(ToolRunRejection::Busy);}
        match (self.selected, transition.slot) {
            (Some(index), Some(slot)) => {
                let entry = &mut self.entries[index];
                let generation_changed = entry.slot.generation != slot.generation;
                entry.slot = slot;
                if generation_changed {
                    entry.rebind();
                }
            }
            (Some(index), None) => {
                let entry = self.entries.remove(index);
                self.retire_entry(*entry);
                self.select_primary();
            }
            (None, _) => {}
        }
        Ok(transition.effect)
    }

    pub(crate) fn retire_entry(&mut self, entry: ToolRunEntry<A>) {
        let ToolRunEntry { job, provisional, finalize, base, overlay, refold, mut displaced, member, .. } = entry;
        self.retired_members.extend(member);
        self.retired_snapshots.push(base);
        self.retired_snapshots.push(overlay);
        if let Some(running) = refold.and_then(|refold| refold.running) {
            self.retired_snapshots.push(Arc::new(running));
        }
        self.retired_snapshots.append(&mut displaced);
        self.retire_owners(job, provisional, finalize);
    }

    pub(crate) fn retire_owners(&mut self, job: Option<ToolRunJobSlot<A::Config>>, mut provisional: Vec<A::Mutation>, finalize: Option<ToolRunFinalize<A>>) {
        assert!(finalize.as_ref().is_none_or(|finalize|finalize.ingress.terminal_is_empty()),"original ToolRun ingress remains in its entry until granted closure");
        if let Some(mut job) = job {
            job.begin_close();
            self.retired_jobs.push(job);
        }
        self.discarded.append(&mut provisional);
        if let Some(publication) = finalize.and_then(|finalize| finalize.publication) {
            self.retired_publications.push(publication);
        }
    }

    pub(crate) fn close_current_job(&mut self) {
        let Some(entry) = selected_entry_mut!(self) else { return };
        entry.settle_refold();
        if let Some(mut job) = entry.job.take() {
            job.begin_close();
            self.retired_jobs.push(job);
        }
    }

    /// 🎯️ Hands the current job the entry's identity through `retarget`; a job that is not retargetable, or
    /// declines, is closed so the driver rebuilds it from its checkpoint.
    pub(crate) fn retarget_current_job(&mut self, retarget: impl FnOnce(&mut dyn ToolRunRetargetableJob<A::Config>, ToolRunIdentity) -> bool) {
        let Some(entry) = selected_entry_mut!(self) else { return };
        let identity = entry.identity;
        let Some(slot) = entry.job.as_mut() else { return };
        if slot.retargetable().is_some_and(|job| retarget(job, identity)) {
            slot.generation = semio_framework_job::Generation(u64::from(identity.generation));
        } else {
            self.close_current_job();
        }
    }

    /// 🧹️ Throws the run's provisional work away, payload included: an abort, a fault, a rebase-restart
    /// or a reconfigure-restart invalidates the run's whole intermediate result, so the windows that
    /// read `ToolRunView::payload` must stop showing it.
    pub(crate) fn discard_provisional(&mut self) {
        self.release_provisional(false);
    }

    /// 🧹️ Releases the provisional ops a COMPLETED finalize has already published, keeping the run's
    /// `payload`. The payload is not provisional work — it is the run's own result (§2.3: "the latest
    /// intermediate result of the run … the run's windows read it back through
    /// `ToolRunView::payload`"), and a `mutating: false` run publishes nothing else at all, so clearing
    /// it on finalize is what made a finished read-only run's windows revert to their pre-run render the
    /// instant it reached `Finalized`. It is cleared on a genuine discard and when the entry retires.
    pub(crate) fn release_provisional(&mut self, keep_payload: bool) {
        let Some(entry) = selected_entry_mut!(self) else { return };
        self.discarded.append(&mut entry.provisional);
        entry.entity_marks.clear();
        entry.entities = Arc::new(BTreeSet::new());
        if let Some(running) = entry.refold.take().and_then(|refold| refold.running) {
            entry.displaced.push(Arc::new(running));
        }
        let overlay = Arc::clone(&entry.base);
        entry.replace_overlay(overlay);
        entry.checkpoint = None;
        if let Some(member) = entry.member.as_mut() {
            member.release();
        }
        if !keep_payload {
            entry.payload = None;
        }
    }

    /// 🧹️ Advances the owners that outlived their slot by one bounded unit; `None` when nothing is retiring.
    pub(crate) fn retirement_demands(&self,store:&ArtifactStore<A::Snapshot,A::Mutation>,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing&&!self.entries.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool entries retain job cancellation and native metadata until granted entry retirement is available"));}
        if let Some(job)=self.retired_jobs.last(){
            if job.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolRunJobSlot<A::Config>>(),release_bytes:job.scaffold_release_bytes(),depth:1,..Default::default()});}
            return job.next_demands(body);
        }
        if let Some(publication)=self.retired_publications.last(){
            if publication.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot,A::Mutation>>(),depth:1,..Default::default()});}
            let mut demand=publication.retirement_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool publication close depth overflow"))?;return Ok(demand);
        }
        if self.snapshot_retirement.is_some(){return tool_erased_retirement_demand(&self.snapshot_retirement,body);}
        if self.pending_snapshot_alias.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Arc<A::Snapshot>>>(),capacity_bytes:ArtifactStore::<A::Snapshot,A::Mutation>::snapshot_alias_retirement_birth_bytes(),depth:2,..Default::default()});}
        if self.pending_discarded.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<A::Mutation>>(),capacity_bytes:store.owned_mutation_retirement_birth_bytes()?,depth:2,..Default::default()});}
        if !self.retired_snapshots.is_empty()||self.entries.iter().any(|entry|!entry.displaced.is_empty()){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Arc<A::Snapshot>>>(),depth:1,..Default::default()});}
        if !self.discarded.is_empty(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<A::Mutation>>(),depth:1,..Default::default()});}
        if let Some((key,window))=self.trace_windows.iter().next(){return Ok(RetirementDemand{copy_bytes:size_of::<(String,ToolRunTraceWindow)>(),release_bytes:key.capacity().saturating_add(window.body_key.capacity()),depth:1,..Default::default()});}
        macro_rules! ledger_quote {($field:ident)=>{if let Some(demand)=ledger_empty_backing_demand(&self.$field)?{return Ok(demand);}};}
        ledger_quote!(entries);ledger_quote!(retired_jobs);ledger_quote!(retired_publications);ledger_quote!(retired_snapshots);ledger_quote!(discarded);ledger_quote!(retired_members);
        if !self.retired_members.is_empty()||!self.trace_windows.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool member and trace metadata retain their granted retirement authority"));}
        Ok(Default::default())
    }
    pub(crate) fn retire_step(&mut self,store:&mut ArtifactStore<A::Snapshot,A::Mutation>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,Fault>{
        if self.has_original_receipt(){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let demand=self.retirement_demands(store,grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?;
        if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(job)=self.retired_jobs.last_mut(){
            if job.terminal_is_empty(){let released_bytes=job.scaffold_release_bytes();drop(self.retired_jobs.pop());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()})));}
            return job.close_step(grant).map(Some);
        }
        if let Some(publication)=self.retired_publications.last_mut(){
            if publication.terminal_is_empty(){drop(self.retired_publications.pop());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
            publication.begin_close();let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=publication.close_step(child).map_err(FaultFrom::into_fault)?;
            let step=admit_retained_clone_close(child,step,publication.terminal_is_empty(),"tool publication original").map_err(FaultFrom::into_fault)?;return Ok(Some(RetainedCloneStep::Progress(step.progress())));
        }
        if self.snapshot_retirement.is_some(){return close_tool_erased_retirement(&mut self.snapshot_retirement,grant).map(Some).map_err(FaultFrom::into_fault);}
        if self.pending_snapshot_alias.is_some(){let result=store.retire_snapshot_alias(&mut self.pending_snapshot_alias,grant).map_err(FaultFrom::into_fault)?;return Ok(Some(RetainedCloneStep::Progress(match result{Some((owner,progress))=>{self.snapshot_retirement=Some(owner);progress},None=>Default::default()})));}
        if self.pending_discarded.is_some(){let result=store.retire_owned_mutation(&mut self.pending_discarded,grant).map_err(FaultFrom::into_fault)?;return Ok(Some(RetainedCloneStep::Progress(match result{Some((owner,progress))=>{self.snapshot_retirement=Some(owner);progress},None=>Default::default()})));}
        let alias=self.retired_snapshots.pop().or_else(||self.entries.iter_mut().find(|entry|!entry.displaced.is_empty()).and_then(|entry|entry.displaced.pop()));
        if let Some(alias)=alias{self.pending_snapshot_alias=Some(alias);return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
        if let Some(operation)=self.discarded.pop(){self.pending_discarded=Some(operation);return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
        if let Some((key,window))=self.trace_windows.pop_first(){let released_bytes=key.capacity().saturating_add(window.body_key.capacity());drop((key,window));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()})));}
        macro_rules! ledger_release {($field:ident)=>{if let Some(step)=close_ledger_backing(&mut self.$field,grant).map_err(FaultFrom::into_fault)?{return Ok(Some(step));}};}
        ledger_release!(entries);ledger_release!(retired_jobs);ledger_release!(retired_publications);ledger_release!(retired_snapshots);ledger_release!(discarded);ledger_release!(retired_members);
        if !self.retired_members.is_empty()||!self.trace_windows.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool metadata retains its granted retirement authority").into_fault());}
        Ok(None)
    }

    /// 🚪️ Document close, instance retirement or reload: same as abort, ephemeral state vanishes (§2.2).
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.selected = None;
    }

    /// 🧹️ One bounded close unit of every retiring owner.
    pub fn close_step(&mut self,store:&mut ArtifactStore<A::Snapshot,A::Mutation>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool run close requires granted entry cancellation before cleanup").into_fault());}
        Ok(self.retire_step(store,grant)?.unwrap_or(RetainedCloneStep::Complete(Default::default())))
    }

    pub fn terminal_is_empty(&self) -> bool {
        ledger_backing_is_empty(&self.entries)&&ledger_backing_is_empty(&self.retired_jobs)&&ledger_backing_is_empty(&self.retired_publications)&&ledger_backing_is_empty(&self.retired_snapshots)&&ledger_backing_is_empty(&self.discarded)&&ledger_backing_is_empty(&self.retired_members)&&self.snapshot_retirement.is_none()&&self.pending_snapshot_alias.is_none()&&self.pending_discarded.is_none()&&self.trace_windows.is_empty()&&!self.has_original_receipt()
    }
}
//#endregion 🔖️Ledger

//#region 🔖️Member
/// 🧬️ The typed owners of a member run (`ToolRunDefinition::member`): the member snapshot the run rests on, the overlay its
/// provisional ops fold to, and every alias it let go — all owed to that member store's alias retirement, never dropped plainly.
trait ToolRunMemberOwners: Send {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    pub(crate) fn has_displaced(&self) -> bool;
    fn is_empty(&self) -> bool;
}

struct ToolRunMemberState<P> {
    base: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    overlay: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    displaced: Vec<Arc<P>>,
    pending_alias: Option<Arc<P>>,
}

impl<P: Send + Sync + 'static> ToolRunMemberOwners for ToolRunMemberState<P> {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn has_displaced(&self) -> bool {
        !self.displaced.is_empty() || self.pending_alias.is_some()
    }

    fn is_empty(&self) -> bool {
        self.base.is_none() && self.overlay.is_none() && self.displaced.is_empty() && self.displaced.capacity()==0 && self.pending_alias.is_none()
    }
}

/// 🧩️ A run whose provisional ops are one owned member's ops (design §12, §20.15): where they land, the member generation
/// they rest on, how many the overlay already shows, the typed owners they fold through, and the composed read — the live
/// children with the overlay in the member's place — the render seams show while the run holds them.
pub(crate) struct ToolRunMemberRun {
    pub(crate) slot: String,
    pub(crate) child_id: String,
    pub(crate) dialect: ArtifactDialect,
    pub(crate) base_generation: u64,
    pub(crate) ops: Vec<Vec<u8>>,
    pub(crate) shown: usize,
    pub(crate) rebase: bool,
    pub(crate) owners: Option<Box<dyn ToolRunMemberOwners>>,
    pub(crate) children: Option<ChildContentView>,
    pub(crate) stale: Vec<ChildContentView>,
    pub(crate) emit:Option<ChildEmit>,
    pub(crate) emission_owner:Option<Box<dyn ToolRunMemberEmissionOwner>>,
    pub(crate) retired_emits:std::collections::VecDeque<ChildEmit>,
    pub(crate) retired_emission_owners:std::collections::VecDeque<Box<dyn ToolRunMemberEmissionOwner>>,
    pub(crate) retired_ops:std::collections::VecDeque<Vec<Vec<u8>>>,
}

impl ToolRunMemberRun {
    /// 🏃️ Owed work: ops the overlay does not show yet, a refold from the member head, or reads, aliases, released ops
    /// and published emissions to retire.
    pub(crate) fn is_work(&self) -> bool {
        self.rebase || self.shown != self.ops.len() || !self.stale.is_empty() || !self.retired_ops.is_empty() || !self.retired_emits.is_empty() || !self.retired_emission_owners.is_empty() || self.owners.as_deref().is_some_and(ToolRunMemberOwners::has_displaced)
    }

    /// 🏁️ Every typed owner, queued emission, operation and composed read of this member is returned; only its address metadata remains.
    pub(crate) fn is_retired(&self) -> bool {
        self.ops.is_empty() && self.ops.capacity() == 0 && self.emit.is_none() && self.emission_owner.is_none() && self.retired_emits.is_empty() && self.retired_emission_owners.is_empty() && self.retired_ops.is_empty() && self.retired_emits.capacity() == 0 && self.retired_emission_owners.capacity() == 0 && self.retired_ops.capacity() == 0 && self.children.is_none() && self.stale.is_empty() && self.owners.as_deref().is_none_or(ToolRunMemberOwners::is_empty)
    }

    /// 📐️ Bytes of the address strings and the empty queues a retired member still holds.
    pub(crate) fn address_backing_bytes(&self) -> usize {
        self.slot.capacity().saturating_add(self.child_id.capacity()).saturating_add(self.stale.capacity().saturating_mul(size_of::<ChildContentView>()))
    }

    /// 🧹️ Drops every provisional op: the composed read retires and the next refresh rests on the member head.
    fn release(&mut self) {
        if !self.ops.is_empty()||self.ops.capacity()!=0{self.retired_ops.push_back(std::mem::take(&mut self.ops));}
        self.shown = 0;
        self.rebase = true;
        self.retired_emits.extend(self.emit.take());
        self.retired_emission_owners.extend(self.emission_owner.take());
        self.stale.extend(self.children.take());
    }
}

/// 📏️ Where one member fold left the overlay: the ops it shows, the ops that did not apply, and — once every op is shown —
/// the leased read of the overlay with the member's revision (`None`: no op, the render seams read the live member).
struct ToolRunMemberFolded {
    pub(crate) shown: usize,
    pub(crate) conflicts: u32,
    pub(crate) read: Option<(store::os_store::snapshot_read::ErasedSnapshotRead, [u8; 32])>,
}

/// 🔁️ Folds a member run's ops onto its overlay within the turn wall budget: a rebase restarts from the member head.
pub(crate) struct ToolRunMemberFold<'a> {
    owners: &'a mut Option<Box<dyn ToolRunMemberOwners>>,
    ops: &'a [Vec<u8>],
    shown: usize,
    rebase: bool,
    deadline_us: u64,
}

impl store::MemberStoreVisitor for ToolRunMemberFold<'_> {
    type Output = Result<ToolRunMemberFolded, Fault>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        let owners = self.owners.get_or_insert_with(|| Box::new(ToolRunMemberState::<P> { base: Some(store.derived_snapshot_head()), overlay: None, displaced: Vec::new(), pending_alias: None }));
        let state = owners.as_any_mut().downcast_mut::<ToolRunMemberState<P>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("toolRun.member-store-kind"), "a member tool run's owners belong to another store kind"))?;
        let mut shown = self.shown;
        if self.rebase {
            let previous = state.base.replace(store.derived_snapshot_head());
            state.displaced.extend(previous.map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));
            state.displaced.extend(state.overlay.take().map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));
            shown = 0;
        }
        let mut conflicts = 0u32;
        while shown < self.ops.len() {
            let op = <Mu as ::protocol::OpBinary>::decode_op(&self.ops[shown]).map_err(|error| error.into_fault())?;
            let source = state.overlay.as_ref().or(state.base.as_ref()).ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("toolRun.member-base-lost"), "a member tool run lost its base"))?;
            let next = store.derive_provisional_snapshot(source, &op);
            <Mu as ::protocol::Mutation<P>>::retire_cold(op);
            match next.map_err(|error| error.into_fault())? {
                Some(next) => state.displaced.extend(state.overlay.replace(next).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner)),
                None => conflicts = conflicts.saturating_add(1),
            }
            shown += 1;
            if shown % TOOL_RUN_REFOLD_CHECK_OPS == 0 && shown < self.ops.len() && semio_framework_job::default_now_us().is_none_or(|now| now >= self.deadline_us) {
                return Ok(ToolRunMemberFolded { shown, conflicts, read: None });
            }
        }
        let read = match state.overlay.as_ref().filter(|_| !self.ops.is_empty()) {
            Some(overlay) => Some((store.snapshot_read_derived(overlay).map_err(|error| error.into_fault())?, store.content_revision())),
            None => None,
        };
        Ok(ToolRunMemberFolded { shown, conflicts, read })
    }
}

fn ledger_backing_is_empty<T>(owner:&Vec<T>)->bool{owner.is_empty()&&(owner.capacity()==0||size_of::<T>()==0)}
fn ledger_empty_backing_demand<T>(owner:&Vec<T>)->Result<Option<RetirementDemand>,ValueError>{
    if !owner.is_empty()||ledger_backing_is_empty(owner){return Ok(None);}
    Ok(Some(RetirementDemand{copy_bytes:2*size_of::<Vec<T>>(),release_bytes:owner.capacity().checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool ledger backing demand overflow"))?,depth:1,..Default::default()}))
}
fn close_ledger_backing<T>(owner:&mut Vec<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
    let Some(demand)=ledger_empty_backing_demand(owner)?else{return Ok(None);};
    if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
    drop(std::mem::take(owner));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})))
}
fn tool_erased_retirement_demand(owner:&Option<Box<dyn store::ErasedSnapshotRetirement>>,body:usize)->Result<RetirementDemand,ValueError>{
 let Some(owner)=owner.as_ref()else{return Ok(Default::default())};
 if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),copy_bytes:size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>(),depth:1,..Default::default()});}
 Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool retained close depth overflow"))?})
}
pub(crate) fn tool_grant_funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{
 grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth
}
fn close_tool_erased_retirement(owner:&mut Option<Box<dyn store::ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
 if owner.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()));}
 if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
 let demand=tool_erased_retirement_demand(owner,grant.maximum_copy_bytes)?;
 if !tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
 let current=owner.as_mut().expect("tool retained owner admitted in place");
 if current.terminal_is_empty(){drop(owner.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
 let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=current.close_step(child)?;
 let step=admit_retained_clone_close(child,step,current.terminal_is_empty(),"tool original retirement")?;
 Ok(RetainedCloneStep::Progress(step.progress()))
}
pub(crate) trait ToolRunMemberEmissionOwner:Send{
    fn as_any_mut(&mut self)->&mut dyn std::any::Any;
    fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>;
    fn is_empty(&self)->bool;
}
struct ToolRunMemberEmissionState<M>{
    current:ManuallyDrop<Option<M>>,
    retirement:ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    schema:ManuallyDrop<Option<String>>,
    refusal:ManuallyDrop<Option<::protocol::ProtocolError>>,
    retirement_refusal:ManuallyDrop<Option<ValueError>>,
    retirement_refusal_close:ManuallyDrop<Option<ControlledRetirement<ValueError>>>,
}
impl<M> Default for ToolRunMemberEmissionState<M>{
    fn default()->Self{Self{current:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),schema:ManuallyDrop::new(None),refusal:ManuallyDrop::new(None),retirement_refusal:ManuallyDrop::new(None),retirement_refusal_close:ManuallyDrop::new(None)}}
}
impl<M:Send+'static> ToolRunMemberEmissionOwner for ToolRunMemberEmissionState<M>{
    fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}
    fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.current.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"decoded member mutation retains its exact Store retirement issuer"));}
        if let Some(owner)=self.retirement_refusal_close.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<ValueError>>>(),depth:1,..Default::default()});}
            return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"member original refusal close depth overflow"))?});
        }
        if self.retirement_refusal.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ControlledRetirement<ValueError>>()+size_of::<Option<ValueError>>(),depth:1,..Default::default()});}
        if let Some(schema)=self.schema.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<String>>(),release_bytes:schema.capacity(),depth:1,..Default::default()});}
        if self.retirement.is_some(){return tool_erased_retirement_demand(&self.retirement,body);}
        ::protocol::protocol_error_retirement_demand(&self.refusal)
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        if self.is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.next_demands(grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?;
        if !tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()};
        if let Some(owner)=self.retirement_refusal_close.as_mut(){
            if owner.terminal_is_empty(){drop(self.retirement_refusal_close.take());return Ok(RetainedCloneStep::Progress(progress));}
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            let step=owner.step(child).map_err(FaultFrom::into_fault)?;let step=admit_retained_clone_close(child,step,owner.terminal_is_empty(),"member original typed refusal").map_err(FaultFrom::into_fault)?;return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(original)=self.retirement_refusal.take(){
            match ControlledRetirement::new(original){Ok(owner)=>*self.retirement_refusal_close=Some(owner),Err((error,original))=>{*self.retirement_refusal=Some(original);return Err(error.into_fault());}}
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if self.schema.is_some(){drop(self.schema.take());return Ok(RetainedCloneStep::Progress(progress));}
        if self.retirement.is_some(){return close_tool_erased_retirement(&mut self.retirement,grant).map_err(FaultFrom::into_fault);}
        ::protocol::close_protocol_error_one(&mut self.refusal,grant).map_err(FaultFrom::into_fault)
    }
    fn is_empty(&self)->bool{self.current.is_none()&&self.retirement.is_none()&&self.schema.is_none()&&self.refusal.is_none()&&self.retirement_refusal.is_none()&&self.retirement_refusal_close.is_none()}
}
impl<M> Drop for ToolRunMemberEmissionState<M>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||(self.current.is_none()&&self.retirement.is_none()&&self.schema.is_none()&&self.refusal.is_none()&&self.retirement_refusal.is_none()&&self.retirement_refusal_close.is_none()),"member emission dropped its decoded mutation or exact refusal before retirement");
        unsafe{ManuallyDrop::drop(&mut self.current);ManuallyDrop::drop(&mut self.retirement);ManuallyDrop::drop(&mut self.schema);ManuallyDrop::drop(&mut self.refusal);ManuallyDrop::drop(&mut self.retirement_refusal);ManuallyDrop::drop(&mut self.retirement_refusal_close);}
    }
}

pub(crate) struct ToolRunMemberEmit<'a>{
    slot:&'a str,
    child_id:&'a str,
    ops:&'a [Vec<u8>],
    emit:&'a mut Option<ChildEmit>,
    owner:&'a mut Option<Box<dyn ToolRunMemberEmissionOwner>>,
    grant:RetainedCloneGrant,
}
impl store::MemberStoreVisitor for ToolRunMemberEmit<'_>{
    type Output=Result<(bool,RetainedCloneProgress),Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
    {
        if self.grant.maximum_items==0{return Ok((false,Default::default()));}
        if self.owner.is_none(){
            let demand=RetirementDemand{copy_bytes:size_of::<Option<Box<dyn ToolRunMemberEmissionOwner>>>(),capacity_bytes:size_of::<ToolRunMemberEmissionState<Mu>>(),depth:1,..Default::default()};
            if !tool_grant_funds(self.grant,demand){return Ok((false,Default::default()));}
            *self.owner=Some(Box::new(ToolRunMemberEmissionState::<Mu>::default()));
            return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}));
        }
        let typed=self.owner.as_mut().unwrap().as_any_mut().downcast_mut::<ToolRunMemberEmissionState<Mu>>().ok_or_else(||plugin_sdk_fault("member emission owns another exact mutation type"))?;
        if let Some(refusal)=typed.refusal.as_ref(){return Err(refusal.to_fault());}
        if typed.current.is_some(){
            if let Some((retirement,progress))=store.retire_owned_mutation(&mut typed.current,self.grant).map_err(FaultFrom::into_fault)?{*typed.retirement=Some(retirement);return Ok((false,progress));}
            return Ok((false,Default::default()));
        }
        if !typed.is_empty(){return Ok((false,typed.close_step(self.grant)?.progress()));}
        if self.emit.as_ref().is_some_and(|emit|emit.ops.len()==self.ops.len()){return Ok((true,Default::default()));}
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool member emission requires genuinely granted typed operation decoding and wire preparation").into_fault())
    }
}
pub(crate) struct ToolRunMemberEmissionRetire<'a>{owner:&'a mut(dyn ToolRunMemberEmissionOwner+'static),grant:RetainedCloneGrant}
impl store::MemberStoreVisitor for ToolRunMemberEmissionRetire<'_>{
    type Output=Result<RetainedCloneStep,Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
    {
        if self.grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let typed=self.owner.as_any_mut().downcast_mut::<ToolRunMemberEmissionState<Mu>>().ok_or_else(||plugin_sdk_fault("member retirement owns another exact mutation type"))?;
        if typed.current.is_some(){
            return Ok(RetainedCloneStep::Progress(match store.retire_owned_mutation(&mut typed.current,self.grant).map_err(FaultFrom::into_fault)?{Some((retirement,progress))=>{*typed.retirement=Some(retirement);progress},None=>Default::default()}));
        }
        typed.close_step(self.grant)
    }
}

/// 🔢️ The member store's generation, the base a member run watches instead of the document's.
pub(crate) struct ToolRunMemberGeneration;

impl store::MemberStoreVisitor for ToolRunMemberGeneration {
    type Output = u64;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        store.generation()
    }
}

/// ♻️ Hands one alias of a member run's owners to the member store's bounded alias retirement: displaced aliases first,
/// and with `everything` the base and overlay too (the run left its slot).
pub(crate) struct ToolRunMemberRetire<'a>{owners:&'a mut(dyn ToolRunMemberOwners+'static),everything:bool,grant:RetainedCloneGrant}
impl store::MemberStoreVisitor for ToolRunMemberRetire<'_>{
    type Output=Result<(Option<Box<dyn store::ErasedSnapshotRetirement>>,RetainedCloneProgress),Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
    {
        if self.grant.maximum_items==0{return Ok((None,Default::default()));}
        let state=self.owners.as_any_mut().downcast_mut::<ToolRunMemberState<P>>().ok_or_else(||plugin_sdk_fault("member alias retirement owns another exact Store type"))?;
        if state.pending_alias.is_some(){return Ok(store.retire_snapshot_alias(&mut state.pending_alias,self.grant).map_err(FaultFrom::into_fault)?.map_or((None,Default::default()),|(owner,progress)|(Some(owner),progress)));}
        let copy=2*size_of::<Option<Arc<P>>>();
        if self.grant.maximum_copy_bytes<copy||self.grant.maximum_depth==0{return Ok((None,Default::default()));}
        let alias=state.displaced.pop().or_else(||if self.everything{state.overlay.take().or_else(||state.base.take()).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner)}else{None});
        if let Some(alias)=alias{state.pending_alias=Some(alias);return Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));}
        let release=state.displaced.capacity().checked_mul(size_of::<Arc<P>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"member original alias backing overflow").into_fault())?;
        if release!=0&&self.grant.maximum_release_bytes>=release{let original=std::mem::take(&mut state.displaced);drop(original);return Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()}));}
        Ok((None,Default::default()))
    }
}
//#endregion 🔖️Member

//#region 🔖️Driver
/// 🎛️ Outcome of one tool run action dispatch; rejections are silent no-ops (§2.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolRunActionOutcome {
    Applied(ToolRunEffect),
    Rejected(ToolRunRejection),
}

/// 🔢️ One §2.5 action argument read as a `u64`, in every shape the wire can hand it over.
///
/// 🪪️ A JSON number that is an exact non-negative integer IS a `u64` here, and reading it as anything
/// else is not strictness, it is a silent refusal. `toolRunFinalize`'s `generation` crosses as
/// `Number(Float(1.0))` — the shell's own action arguments build it with `UiValue::Number(f64)`, and a
/// guest that mints it with `DslValue::uint` arrives the same way — so accepting only an exact `u64`
/// carrier or a numeric string made `generation` read `None`, and `(_, None)` is
/// `ToolRunRejection::Stale`. EVERY finalize of EVERY app was rejected: runs reached `Complete` and
/// stayed there, and because a start against a live run is `ToolRunRejection::Busy`, no tool run could
/// ever be started a second time. Measured on 6018 as a 3d preview that never re-evaluated after the
/// first evaluation of a session (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
pub(crate) fn tool_run_arg_u64(args: Option<&DslValue>, key: &str) -> Option<u64> {
    let value = args?.get(key)?;
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
        .or_else(|| value.as_f64().filter(|number| number.is_finite() && *number >= 0.0 && number.fract() == 0.0 && *number < 18_446_744_073_709_551_616.0).map(|number| number as u64))
}

//#endregion 🔖️Driver

//#region 🔖️Panel
fn tool_run_step_text(definition: &ToolRunDefinition, step: &ToolRunStep, locale: Locale) -> String {
    let template = match ToolRunLabel::for_reason(step.reason) {
        Some(label) => label.text(locale).to_string(),
        None => definition.reason(step.reason).map_or_else(|| step.reason.to_string(), |reason| reason.template.resolve(Terminology::Native, locale).to_string()),
    };
    let text = tool_run_format(&template, |name| name.parse::<usize>().ok().and_then(|index| step.args.get(index)).map(|arg| arg.to_plain_string()));
    if step.repeat > 1 {
        format!("{text} ×{}", step.repeat)
    } else {
        text
    }
}

fn tool_run_step_tone(kind: ToolRunStepKind) -> Tone {
    match kind {
        ToolRunStepKind::Info => Tone::Info,
        ToolRunStepKind::Success => Tone::Success,
        ToolRunStepKind::Warning => Tone::Warning,
        ToolRunStepKind::Danger => Tone::Danger,
    }
}

fn tool_run_verdict_tone(verdict: ToolRunVerdict) -> Tone {
    match verdict {
        ToolRunVerdict::Testing => Tone::Neutral,
        ToolRunVerdict::Success => Tone::Success,
        ToolRunVerdict::Warning => Tone::Warning,
        ToolRunVerdict::Danger => Tone::Danger,
    }
}

fn tool_run_state_label(state: ToolRunState) -> ToolRunLabel {
    state.label()
}

impl<A: ArtifactApp> ToolRunLedger<A> {
    /// 🪧️ The framework ToolRun panel (§2.6) as a `ComponentTree` root `framework.toolRun`: a ready group
    /// `framework.toolRun.ready` with an enabled Start for `ready_tool` (the active run-declaring tool) while this
    /// instance holds no run of it, then one group `framework.toolRun.<run>` per run, oldest first, labelled by
    /// `tool_label` of its tool.
    pub fn panel(&mut self, controller_id: &str, locale: Locale, ready_tool: Option<&str>, tool_label: impl Fn(&str) -> Option<String>) -> UiAssemblyResult<BuiltNode> {
        let error = ui_assembly_error;
        let mut groups = BuiltChildren::default();
        if let Some(tool_id) = ready_tool.filter(|tool_id| !self.entries.iter().any(|entry| entry.tool_id == *tool_id)) {
            let label = tool_label(tool_id).unwrap_or_else(|| tool_id.to_string());
            groups.try_push(tool_run_panel_ready_group(tool_id, controller_id, locale, &label)?).map_err(|_| error("tool-run-panel.groups"))?;
        }
        for entry in &mut self.entries {
            let label = tool_label(&entry.tool_id).unwrap_or_else(|| entry.tool_id.clone());
            groups.try_push(tool_run_panel_group(entry, controller_id, locale, &label)?).map_err(|_| error("tool-run-panel.groups"))?;
        }
        column()
            .try_label(if locale == Locale::De { "Werkzeugläufe" } else { "Tool runs" })
            .map_err(|_| error("tool-run-panel.label"))?
            .try_id(semio_framework_tool_run::TOOL_RUN_PANEL_ID)
            .map_err(|_| error("tool-run-panel.id"))?
            .try_children(groups)
            .map_err(|_| error("tool-run-panel.groups"))?
            .try_build()
            .map_err(|_| error("tool-run-panel.build"))
    }
}

/// ▶️ The ready group of a run-declaring tool with no run yet: its status and a real Start button carrying the tool id.
fn tool_run_panel_ready_group(tool_id: &str, controller_id: &str, locale: Locale, label: &str) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = format!("{}.ready", semio_framework_tool_run::TOOL_RUN_PANEL_ID);
    let status = text(Label(UiText::clipped(ToolRunLabel::ReadyToStart.text(locale))))
        .live(Liveness::Polite)
        .try_id(format!("{scope}.status"))
        .map_err(|_| error("tool-run-panel.ready-status-id"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.ready-status"))?;
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| error("tool-run-panel.ready-args"))?;
    arguments.push(TOOL_RUN_ARG_TOOL_ID.to_string(), UiValue::Text(UiText::clipped(tool_id))).map_err(|_| error("tool-run-panel.ready-args"))?;
    let action = ToolRunAction::Start;
    let action_id = ActionId::try_v1(controller_id, action.id()).ok_or_else(|| error("tool-run-panel.action-id"))?;
    let start = button(Label(UiText::clipped(action.label().text(locale))))
        .try_id(format!("{scope}.{}", action.id()))
        .map_err(|_| error("tool-run-panel.button-id"))?
        .try_shortcut(action.chord())
        .map_err(|_| error("tool-run-panel.button-shortcut"))?
        .try_on_with(Trigger::Activate, action_id, UiValue::Map(arguments.finish()))
        .map_err(|_| error("tool-run-panel.button-binding"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.button"))?;
    let mut buttons = BuiltChildren::default();
    buttons.try_push(start).map_err(|_| error("tool-run-panel.buttons"))?;
    let toolbar = row().try_id(format!("{scope}.actions")).map_err(|_| error("tool-run-panel.actions-id"))?.try_children(buttons).map_err(|_| error("tool-run-panel.actions"))?.try_build().map_err(|_| error("tool-run-panel.actions-build"))?;
    let mut children = BuiltChildren::default();
    for child in [status, toolbar] {
        children.try_push(child).map_err(|_| error("tool-run-panel.children"))?;
    }
    column()
        .try_label(label)
        .map_err(|_| error("tool-run-panel.label"))?
        .try_id(scope)
        .map_err(|_| error("tool-run-panel.id"))?
        .try_children(children)
        .map_err(|_| error("tool-run-panel.children"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.build"))
}

/// 🪧️ One run's panel group (§2.6): polite status, progressbar, real buttons with `aria-keyshortcuts` addressing the
/// run by id, step log (live off) and a keyboard-navigable trace list; every id is scoped `framework.toolRun.<run>.`.
///
/// 👁️ The throttle belongs to the ANNOUNCEMENT, never to the text. Publishing the last ANNOUNCED
/// string froze the visible pill at whatever stage the run was in when it was first announced:
/// a `Finalized` run still read `Evaluating nodes (1/2)` on 6021 at 18:28 while its own progress
/// bar had moved on (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A sighted reader is watching the
/// same string a reader hears, so the string is always current and only `live` is rationed.
fn tool_run_panel_group<A: ArtifactApp>(entry: &mut ToolRunEntry<A>, controller_id: &str, locale: Locale, label: &str) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = semio_framework_tool_run::tool_run_panel_group_id(entry.slot.run);
    let state = entry.slot.state;
    let stage_count = entry.definition.stages.len();
    let stage_label = entry.definition.stage(entry.stage).map_or_else(String::new, |stage| stage.label.resolve(Terminology::Native, locale).to_string());
    let status = format!("{} · {stage_label} ({}/{stage_count})", tool_run_state_label(state).text(locale), usize::from(entry.stage) + 1);
    let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
    let announce = match entry.announced.as_ref() {
        Some((announced_state, at, announced)) => *announced_state != state || (*announced != status && now_ms.saturating_sub(*at) >= TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS),
        None => true,
    };
    if announce {
        entry.announced = Some((state, now_ms, status.clone()));
    }
    let live = if !announce {
        Liveness::Off
    } else if state == ToolRunState::Faulted || entry.conflicts > 0 {
        Liveness::Assertive
    } else {
        Liveness::Polite
    };
    let status_node = text(Label(UiText::clipped(&status))).live(live).try_id(format!("{scope}.status")).map_err(|_| error("tool-run-panel.status-id"))?.try_build().map_err(|_| error("tool-run-panel.status"))?;
    let unit = entry.definition.unit.resolve(Terminology::Native, locale).to_string();
    let value_text = match entry.total {
        Some(total) => {
            let percent = if total == 0 { 100 } else { entry.completed.saturating_mul(100) / total };
            tool_run_format(ToolRunLabel::ProgressValueText.text(locale), |name| match name {
                "stage" => Some(stage_label.clone()),
                "i" => Some((usize::from(entry.stage) + 1).to_string()),
                "n" => Some(stage_count.to_string()),
                "completed" => Some(entry.completed.to_string()),
                "total" => Some(total.to_string()),
                "unit" => Some(unit.clone()),
                "pct" => Some(percent.to_string()),
                _ => None,
            })
        }
        None => format!("{stage_label} ({}/{stage_count}): {} {unit}", usize::from(entry.stage) + 1, entry.completed),
    };
    let mut bar = progress(entry.completed as f64, Label(UiText::clipped(&value_text)));
    if let Some(total) = entry.total {
        bar = bar.total(total as f64);
    }
    let bar = bar.try_id(format!("{scope}.progress")).map_err(|_| error("tool-run-panel.progress-id"))?.try_build().map_err(|_| error("tool-run-panel.progress"))?;
    let (run, generation) = (entry.slot.run, entry.slot.generation);
    let mut buttons = BuiltChildren::default();
    let toggle = if state == ToolRunState::Paused { ToolRunAction::Resume } else { ToolRunAction::Pause };
    let actions = if state.is_terminal() { vec![ToolRunAction::Start, ToolRunAction::Dismiss] } else { vec![toggle, ToolRunAction::Step, ToolRunAction::Abort, ToolRunAction::Finalize] };
    for action in actions {
        let mut arguments = UiMapBuilder::try_new().ok_or_else(|| error("tool-run-panel.args"))?;
        let pushed = match action {
            ToolRunAction::Start => arguments.push(TOOL_RUN_ARG_TOOL_ID.to_string(), UiValue::Text(UiText::clipped(&entry.tool_id))).is_ok(),
            _ => arguments.push(TOOL_RUN_ARG_RUN_ID.to_string(), UiValue::Text(UiText::clipped(&run.to_string()))).is_ok() && arguments.push(TOOL_RUN_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).is_ok(),
        };
        if !pushed {
            return Err(error("tool-run-panel.args"));
        }
        let action_id = ActionId::try_v1(controller_id, action.id()).ok_or_else(|| error("tool-run-panel.action-id"))?;
        let legal = action.is_legal_in(Some(state));
        let mut builder = button(Label(UiText::clipped(action.label().text(locale)))).disabled(!legal);
        builder = builder.try_id(format!("{scope}.{}", action.id())).map_err(|_| error("tool-run-panel.button-id"))?;
        builder = builder.try_shortcut(action.chord()).map_err(|_| error("tool-run-panel.button-shortcut"))?;
        if action == ToolRunAction::Finalize && !legal {
            builder = builder.try_describe(ToolRunLabel::FinalizeDisabled.text(locale)).map_err(|_| error("tool-run-panel.finalize-description"))?;
        }
        let node = builder.try_on_with(Trigger::Activate, action_id, UiValue::Map(arguments.finish())).map_err(|_| error("tool-run-panel.button-binding"))?.try_build().map_err(|_| error("tool-run-panel.button"))?;
        buttons.try_push(node).map_err(|_| error("tool-run-panel.buttons"))?;
    }
    let toolbar = row().try_id(format!("{scope}.actions")).map_err(|_| error("tool-run-panel.actions-id"))?.try_children(buttons).map_err(|_| error("tool-run-panel.actions"))?.try_build().map_err(|_| error("tool-run-panel.actions-build"))?;
    let mut step_rows = BuiltChildren::default();
    let newest_steps: Vec<&ToolRunStep> = entry.steps.iter().collect();
    for step in newest_steps.into_iter().rev().take(TOOL_RUN_PANEL_STEP_ROWS) {
        let row = text(Label(UiText::clipped(&tool_run_step_text(&entry.definition, step, locale))))
            .tone(tool_run_step_tone(step.kind))
            .try_id(format!("{scope}.step.{}", step.sequence))
            .map_err(|_| error("tool-run-panel.step-id"))?
            .try_build()
            .map_err(|_| error("tool-run-panel.step"))?;
        step_rows.try_push(row).map_err(|_| error("tool-run-panel.steps"))?;
    }
    let steps = column()
        .live(Liveness::Off)
        .try_label(if locale == Locale::De { "Schritte" } else { "Steps" })
        .map_err(|_| error("tool-run-panel.steps-label"))?
        .try_id(format!("{scope}.steps"))
        .map_err(|_| error("tool-run-panel.steps-id"))?
        .try_children(step_rows)
        .map_err(|_| error("tool-run-panel.steps-children"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.steps-build"))?;
    let mut trace_rows = BuiltChildren::default();
    for key in entry.recent_trace.iter().rev() {
        let Some(record) = entry.trace.record(*key) else { continue };
        let reason = ToolRunStep { sequence: 0, kind: ToolRunStepKind::Info, stage: entry.stage, reason: record.reason, subject: Some(*key), repeat: 1, args: Vec::new() };
        let item = tree_item(Label(UiText::clipped(&tool_run_step_text(&entry.definition, &reason, locale))))
            .tone(tool_run_verdict_tone(record.verdict))
            .try_id(format!("{scope}.trace.{key}"))
            .map_err(|_| error("tool-run-panel.trace-id"))?
            .try_build()
            .map_err(|_| error("tool-run-panel.trace-item"))?;
        trace_rows.try_push(item).map_err(|_| error("tool-run-panel.trace-rows"))?;
    }
    let trace = tree()
        .try_label(if locale == Locale::De { "Versuche" } else { "Attempts" })
        .map_err(|_| error("tool-run-panel.trace-label"))?
        .try_id(format!("{scope}.trace"))
        .map_err(|_| error("tool-run-panel.trace-id"))?
        .try_children(trace_rows)
        .map_err(|_| error("tool-run-panel.trace"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.trace-build"))?;
    let mut children = BuiltChildren::default();
    for child in [status_node, bar, toolbar, steps, trace] {
        children.try_push(child).map_err(|_| error("tool-run-panel.children"))?;
    }
    column()
        .try_label(label)
        .map_err(|_| error("tool-run-panel.label"))?
        .try_id(scope)
        .map_err(|_| error("tool-run-panel.id"))?
        .try_children(children)
        .map_err(|_| error("tool-run-panel.children"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.build"))
}
//#endregion 🔖️Panel

#[cfg(test)]
#[path="♻️retirement/🧪️tests/🦀️.rs"]
mod granted_ledger_retirement_tests;
