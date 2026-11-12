//! 🪪️ Local rule authority borrows the one completed original type candidate owner.
use super::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
/// ⚖️ The completed candidate owner proves its local rules against the original request.
pub trait IfcCandidateCheck {
    fn check(&self, cx: &StepContext<'_>) -> Result<usize, ValueError>;
    fn position(&self) -> usize;
}
pub struct IfcTypeCandidate<'a> {
    owner: &'a dyn IfcCandidateCheck,
    snapshot: &'a IfcSnapshot,
    mutations: &'a [IfcMutation],
}
impl<'a> IfcTypeCandidate<'a> {
    pub fn new(owner: &'a dyn IfcCandidateCheck, snapshot: &'a IfcSnapshot, mutations: &'a [IfcMutation]) -> Self {
        Self { owner, snapshot, mutations }
    }
    pub fn check(&self, cx: &StepContext<'_>) -> Result<usize, ValueError> {
        self.owner.check(cx)
    }
    pub fn snapshot(&self) -> &IfcSnapshot { self.snapshot }
    pub fn mutations(&self) -> &[IfcMutation] { self.mutations }
    pub fn position(&self) -> usize { self.owner.position() }
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IfcCandidateRuleStep{Pending,Complete,Invalid,Incomplete}
pub trait IfcCandidateRules: Default + Send {
    fn demands(&self, snapshot: &IfcSnapshot, mutations: &[IfcMutation]) -> Result<RetirementDemand, ValueError>;
    fn advance(&mut self, candidate: &IfcTypeCandidate<'_>, cx: &mut StepContext<'_>) -> Result<IfcCandidateRuleStep, OriginalTurnFailure>;
    fn begin_close(&mut self);
    fn terminal_is_empty(&self) -> bool;
    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> RetainedWorkOutcome;
}
impl IfcCandidateRules for () {
    fn demands(&self, _: &IfcSnapshot, _: &[IfcMutation]) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
    fn advance(&mut self, candidate: &IfcTypeCandidate<'_>, cx: &mut StepContext<'_>) -> Result<IfcCandidateRuleStep, OriginalTurnFailure> { candidate.check(cx)?; Ok(IfcCandidateRuleStep::Complete) }
    fn begin_close(&mut self) {}
    fn terminal_is_empty(&self) -> bool { true }
    fn close_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
    fn close_step(&mut self, _: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> RetainedWorkOutcome { RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 } }
}
