//! 🪪️ Original strict type repair owns local rule receiving and retirement before publication.
use super::{machine::{MachineEnvironment, RetainedMachine}, overlay::PopulationOverlay, receive::PopulationReceiver, retained::*};
use crate::details::{IfcCandidateRules, IfcCandidateRuleStep, IfcTypeCandidate};
use semio_framework_job::{OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::mem::size_of;
#[derive(Clone, Copy)]
struct Binding {
    original: OriginalSourceBinding,
    owner: usize,
    source: usize,
    mutations: usize,
    length: usize,
    position: usize,
}
#[derive(Default)]
pub struct NativeCandidateRules {
    binding: Option<Binding>,
    phase: u8,
    closing: bool,
    overlay: PopulationOverlay,
    receiver: PopulationReceiver,
    machine: RetainedMachine,
}
impl NativeCandidateRules {
    fn metadata() -> RetirementDemand { fixed(4 * size_of::<Option<Binding>>() + 4 * size_of::<u8>() + 4 * size_of::<bool>()) }
    fn check(&self, candidate: &IfcTypeCandidate<'_>, cx: &StepContext<'_>) -> Result<usize, ValueError> {
        let owner = candidate.check(cx)?;
        if self.closing || self.binding.is_some_and(|binding| !binding.original.matches(cx)
            || binding.owner != owner || binding.source != candidate.snapshot() as *const _ as usize
            || binding.mutations != candidate.mutations().as_ptr() as usize || binding.length != candidate.mutations().len()
            || binding.position != candidate.position()) {
            return Err(refusal("express.local-original-candidate-changed"));
        }
        Ok(owner)
    }
}
impl IfcCandidateRules for NativeCandidateRules {
    fn demands(&self, snapshot: &super::overlay::IfcSnapshot, mutations: &[super::overlay::IfcMutation]) -> Result<RetirementDemand, ValueError> {
        let mut demand = match self.phase {
            1 => self.overlay.demands(snapshot, mutations),
            2 => self.receiver.demands(&self.overlay),
            3 => Ok(RetainedMachine::birth_demands()),
            4 => self.machine.demands(),
            6 => self.close_demands(4096),
            _ => Ok(Default::default()),
        }?;
        demand.copy_bytes = demand.copy_bytes.checked_add(Self::metadata().copy_bytes).ok_or_else(|| refusal("express.local-demand-overflow"))?;
        demand.depth = demand.depth.max(1);
        Ok(demand)
    }
    fn advance(&mut self, candidate: &IfcTypeCandidate<'_>, cx: &mut StepContext<'_>) -> Result<IfcCandidateRuleStep, OriginalTurnFailure> {
        let demand = self.demands(candidate.snapshot(), candidate.mutations())?;
        if !admitted(cx, demand)? { return Ok(IfcCandidateRuleStep::Pending); }
        let owner = self.check(candidate, cx)?;
        let before = cx.retained_progress();
        let source = candidate.snapshot();
        let mutations = candidate.mutations();
        let complete = match self.phase {
            0 => {
                self.binding = Some(Binding { original: OriginalSourceBinding::borrow(cx), owner, source: source as *const _ as usize, mutations: mutations.as_ptr() as usize, length: mutations.len(), position: candidate.position() });
                true
            }
            1 => self.overlay.advance(source, mutations, cx)?,
            2 => self.receiver.advance(&self.overlay, source, mutations, cx)?,
            3 => {
                let id = source.entities.get(candidate.position()).ok_or_else(|| refusal("express.local-source-row"))?.id;
                let row = self.overlay.find(id).ok_or_else(|| refusal("express.local-overlay-row"))?;
                self.machine.start_local(row, cx)?
            }
            4 => self.machine.advance_local(&MachineEnvironment { overlay: Some(&self.overlay), receiver: Some(&self.receiver), source: Some(source), mutations }, cx)?,
            5 => match self.machine.local_status() {
                Some("pass") => {
                    self.machine.begin_close(); self.receiver.begin_close(); self.overlay.begin_close();
                    true
                },
                Some("fail") => return Ok(IfcCandidateRuleStep::Invalid),
                _ => return Ok(IfcCandidateRuleStep::Incomplete),
            },
            6 => {
                if self.machine.terminal_is_empty() && self.receiver.terminal_is_empty() && self.overlay.terminal_is_empty() { true }
                else {
                    let demand = self.close_demands(4096)?;
                    cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |loan| {
                        if !self.machine.terminal_is_empty() { return self.machine.close_step(loan); }
                        if !self.receiver.terminal_is_empty() { return self.receiver.close_step(loan); }
                        self.overlay.close_step(loan)
                    }))?;
                    false
                }
            }
            7 => true,
            _ => return Err(refusal("express.local-phase").into()),
        };
        if complete && self.phase < 7 { self.phase += 1; }
        let delegated = cx.retained_progress() != before;
        cx.consume_retained(RetainedCloneProgress { copied_items: usize::from(!delegated), copied_bytes: Self::metadata().copy_bytes, ..Default::default() })?;
        if !delegated { cx.consume_fuel(1); }
        Ok(if complete && self.phase == 7{IfcCandidateRuleStep::Complete}else{IfcCandidateRuleStep::Pending})
    }
    fn begin_close(&mut self) {
        self.closing = true;
        self.machine.begin_close(); self.receiver.begin_close(); self.overlay.begin_close();
    }
    fn terminal_is_empty(&self) -> bool {
        self.binding.is_none() && self.machine.terminal_is_empty() && self.receiver.terminal_is_empty() && self.overlay.terminal_is_empty()
    }
    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.machine.terminal_is_empty() { return self.machine.close_demands(body); }
        if !self.receiver.terminal_is_empty() { return self.receiver.close_demands(body); }
        if !self.overlay.terminal_is_empty() { return self.overlay.close_demands(body); }
        Ok(Self::metadata())
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("express.local-not-closing")); }
        if !self.machine.terminal_is_empty() { return self.machine.close_step(grant); }
        if !self.receiver.terminal_is_empty() { return self.receiver.close_step(grant); }
        if !self.overlay.terminal_is_empty() { return self.overlay.close_step(grant); }
        let demand = Self::metadata();
        if !fits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.binding = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
}
