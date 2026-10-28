//! 🪪️ Local rule authority borrows the one completed original type candidate owner.
use super::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
type Validate = fn(&constituent_roster::RetainedTypeAdmission<'_>, &editing::SnapshotEditEvent, &IfcSnapshot, &pointer::PathAdmission, &[IfcMutation], &StepContext<'_>) -> Result<usize, ValueError>;
pub struct IfcTypeCandidate<'a> {
    admission: constituent_roster::RetainedTypeAdmission<'a>,
    event: &'a editing::SnapshotEditEvent,
    snapshot: &'a IfcSnapshot,
    address: &'a pointer::PathAdmission,
    mutations: &'a [IfcMutation],
    validate: Validate,
}
impl<'a> IfcTypeCandidate<'a> {
    pub(super) fn new<C: IfcDetailsProfile>(admission: constituent_roster::RetainedTypeAdmission<'a>, event: &'a editing::SnapshotEditEvent, snapshot: &'a IfcSnapshot, address: &'a pointer::PathAdmission, mutations: &'a [IfcMutation]) -> Option<Self> {
        admission.same_name_repair().then_some(Self { admission, event, snapshot, address, mutations, validate: constituent_roster::RetainedTypeAdmission::validate::<C> })
    }
    pub fn check(&self, cx: &StepContext<'_>) -> Result<usize, ValueError> {
        (self.validate)(&self.admission, self.event, self.snapshot, self.address, self.mutations, cx)
    }
    pub fn snapshot(&self) -> &IfcSnapshot { self.snapshot }
    pub fn mutations(&self) -> &[IfcMutation] { self.mutations }
    pub fn position(&self) -> usize { self.admission.position() }
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IfcCandidateRuleStep{Pending,Complete,Invalid,Incomplete}
pub trait IfcCandidateRules: Default + Send {
    fn demands(&self, snapshot: &IfcSnapshot, mutations: &[IfcMutation]) -> Result<RetirementDemand, ValueError>;
    fn advance(&mut self, candidate: &IfcTypeCandidate<'_>, cx: &mut StepContext<'_>) -> Result<IfcCandidateRuleStep, OriginalTurnFailure>;
    fn begin_close(&mut self);
    fn terminal_is_empty(&self) -> bool;
    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
}
impl IfcCandidateRules for () {
    fn demands(&self, _: &IfcSnapshot, _: &[IfcMutation]) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
    fn advance(&mut self, candidate: &IfcTypeCandidate<'_>, cx: &mut StepContext<'_>) -> Result<IfcCandidateRuleStep, OriginalTurnFailure> { candidate.check(cx)?; Ok(IfcCandidateRuleStep::Complete) }
    fn begin_close(&mut self) {}
    fn terminal_is_empty(&self) -> bool { true }
    fn close_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
    fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
}
