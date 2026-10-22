//! 🔁️ Refold, finalize and entry of the tool run ledger: the refold, settings values and finalize phases and the per-run entry with its progress, trace and publication operations. See [`super`].

use super::*;
use semio_framework::Fault;
use semio_framework::FaultOrigin;
use semio_framework::FaultCode;
use semio_framework_os_kernel::DslValue;
use std::collections::BTreeSet;

//#region 🔖️Ledger
/// 🔁️ A bounded refold of the provisional ops from `base`. `boundary` is whether the job reached a point where
/// the refolded overlay is whole: a retract refold waits for the job's next checkpoint, completion or close,
/// so a retract followed by re-appends spread over several ticks never renders a half re-appended overlay —
/// the previous overlay stays rendered until then.
pub struct ToolRunRefold<P> {
    pub running: Option<P>,
    pub cursor: usize,
    pub boundary: bool,
}

/// 🚦️ Where one refold turn left the overlay.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolRunRefoldTurn {
    Pending,
    AwaitingBoundary,
    Swapped,
}

/// 🎚️ The values a run's declared settings reads named when last observed: config pointers, then per declared
/// window kind the `(window id, values)` of every partition it reads.
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToolRunSettingsValues {
    pub config: Vec<Option<DslValue>>,
    pub window_config: Vec<(String, String, Vec<Option<DslValue>>)>,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolRunFinalizePhase {
    Pending,
    Revalidating,
    Publishing,
    Closing,
}

#[doc(hidden)]
pub struct ToolRunFinalize<A: ArtifactApp> {
    pub phase: ToolRunFinalizePhase,
    pub publication: Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    pub ingress: ToolRunPublicationIngress,
    pub rejected: bool,
    pub retracted: u32,
    pub published: bool,
}

#[doc(hidden)]
pub struct ToolRunEntry<A: ArtifactApp> {
    pub tool_id: String,
    pub actor: semio_framework_value::SharedUtf8,
    pub definition: ToolRunDefinition,
    pub slot: ToolRunSlot,
    pub identity: ToolRunIdentity,
    pub base: Arc<A::Snapshot>,
    pub base_generation: u64,
    pub settings_generation: (u64, u64),
    pub settings_values: ToolRunSettingsValues,
    pub window: Option<(String, String)>,
    pub window_bodies: Vec<String>,
    pub payload: Option<Arc<Vec<u8>>>,
    pub trace_keys: ToolRunTraceKeys,
    pub provisional: Vec<A::Mutation>,
    pub entity_marks: Vec<(u32, u64)>,
    pub entities: Arc<BTreeSet<u64>>,
    pub overlay: Arc<A::Snapshot>,
    pub refold: Option<ToolRunRefold<A::Snapshot>>,
    pub trace: ToolRunTraceStore,
    pub recent_trace: std::collections::VecDeque<u64>,
    pub stage: u16,
    pub completed: u64,
    pub total: Option<u64>,
    pub counters: Vec<ToolRunCounter>,
    pub steps: ToolRunStepRing,
    pub step_sequence: u64,
    pub sequence: u64,
    pub conflicts: u32,
    pub rate_origin: Option<(u64, u64)>,
    pub units_per_second: f32,
    pub job: Option<ToolRunJobSlot<A::Config>>,
    pub checkpoint: Option<Vec<u8>>,
    pub pending_step: bool,
    pub finalize: Option<ToolRunFinalize<A>>,
    pub announced: Option<(ToolRunState, u64, String)>,
    pub port: ToolRunJobPort,
    /// ♻️ Snapshot aliases this run let go (a replaced base or overlay, a superseded fold result), owed to the store's
    /// alias retirement ([`ToolRunLedger::retire_step`]); never dropped plainly.
    pub displaced: Vec<Arc<A::Snapshot>>,
    /// 🧩️ The owned member a member run edits (`ToolRunDefinition::member`); `None` for a run on the document's store.
    pub member: Option<ToolRunMemberRun>,
}

impl<A: ArtifactApp> ToolRunEntry<A> {
    /// 🛣️ The lane this run occupies: the mutating lane, or its tool on its starting window when read-only.
    pub fn lane(&self) -> ToolRunLane {
        ToolRunLane { mutating: self.definition.mutating, tool_id: self.tool_id.clone(), window_id: if self.definition.mutating { None } else { self.window.as_ref().map(|(window_id, _)| window_id.clone()) } }
    }

    pub fn view(&self) -> ToolRunView {
        ToolRunView { tool_id: self.tool_id.clone(), identity: self.identity, state: self.slot.state, provisional_entities: Arc::clone(&self.entities), progress: self.progress(), payload: self.payload.clone() }
    }

    pub fn progress(&self) -> semio_framework_tool_run::ToolRunProgress {
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
    pub fn has_pending_work(&self) -> bool {
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

    pub fn framework_step(&mut self, kind: ToolRunStepKind, reason: u16, args: &[ToolRunStepArg]) {
        self.step_sequence = self.step_sequence.saturating_add(1);
        self.steps.push(ToolRunStep { sequence: self.step_sequence, kind, stage: self.stage, reason, subject: None, repeat: 1, args: args.to_vec() });
    }

    pub fn begin_refold(&mut self, boundary: bool) {
        if let Some(running) = self.refold.replace(ToolRunRefold { running: None, cursor: 0, boundary }).and_then(|refold| refold.running) {
            self.displaced.push(Arc::new(running));
        }
    }

    /// ♻️ Replaces the overlay, owing the previous one to the store's alias retirement.
    pub fn replace_overlay(&mut self, overlay: Arc<A::Snapshot>) {
        let previous = std::mem::replace(&mut self.overlay, overlay);
        self.displaced.push(previous);
    }

    /// ♻️ Replaces the base, owing the previous one to the store's alias retirement.
    pub fn replace_base(&mut self, base: Arc<A::Snapshot>) {
        let previous = std::mem::replace(&mut self.base, base);
        self.displaced.push(previous);
    }

    /// 🏁️ The job reached a point where the refolded overlay is whole.
    pub fn settle_refold(&mut self) {
        if let Some(refold) = self.refold.as_mut() {
            refold.boundary = true;
        }
    }

    fn refold_is_work(&self) -> bool {
        self.refold.as_ref().is_some_and(|refold| refold.boundary || refold.cursor < self.provisional.len())
    }

    pub fn rebind(&mut self) {
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
        protocol::apply_diff_cold(outcome.diff(), snapshot).ok()
    }

    /// 🔁️ Folds provisional ops from `base` until the wall deadline; the overlay is replaced once every op is
    /// folded and the refold reached its boundary.
    pub fn refold_turn(&mut self, deadline_us: u64) -> ToolRunRefoldTurn {
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
    pub fn apply_tick(&mut self, mut tick: ToolRunTick, discarded: &mut Vec<A::Mutation>) -> Result<ToolRunTickReceipt, Fault> {
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
//#endregion 🔖️Ledger
