//! 📥️ Items of `operation` that belong to `os-store-space-member` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`.
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_os_store_member_open::source_admission::{MemberOpenSourceAdmission, diagnostic_error as member_admission_diagnostic};
use semio_framework_os_store_member_open::{MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenOperation, MemberOpenRequest, MemberOpenStep};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
use semio_framework_job::{StepContext, OriginalTurnFailure, OriginalCallerSourceParent, OriginalCallerSourceLoan};
use std::{marker::PhantomData, mem::ManuallyDrop};
use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenStep;
use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenOperation;
use semio_framework_value::RetirementDemand;
pub struct UnsupportedMemberSnapshotOpen<P> {
    pub request: ManuallyDrop<Option<MemberOpenRequest>>,
    pub diagnostic: Option<MemberOpenDiagnostic>,
    pub marker: PhantomData<fn() -> P>,
}

impl<P: Send> MemberSnapshotOpenOperation for UnsupportedMemberSnapshotOpen<P> {
    type Snapshot = P;
    fn begin_birth_bytes(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic> { request.admitted_expected().map(|_| 0) }

    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        if let Err(diagnostic) = request.admitted_expected() {
            return Err(MemberOpenAdmissionError { diagnostic, request });
        }
        Ok(Self { request: ManuallyDrop::new(Some(request)), diagnostic: None, marker: PhantomData })
    }

    fn step(&mut self, cx: &mut StepContext<'_>, _grant: RetainedCloneGrant) -> Result<MemberSnapshotOpenStep,OriginalTurnFailure> {
        let diagnostic = self.diagnostic.or_else(|| self.request.as_ref().and_then(|request| request.check_step_authority(cx).err())).unwrap_or(MemberOpenDiagnostic::Decode);
        self.diagnostic = Some(diagnostic);
        Ok(MemberSnapshotOpenStep::Rejected(diagnostic))
    }

    fn take_ready(&mut self, _cx: &mut StepContext<'_>) -> Result<Option<(P, MemberOpenRequest)>,OriginalTurnFailure> {
        Ok(None)
    }
}

impl<P: Send> ErasedSnapshotRetirement for UnsupportedMemberSnapshotOpen<P> {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { semio_framework_os_store_read::retirement_owner_demands(&self.request, body) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items != 0 && grant.maximum_depth >= self.next_depth_demand()? { self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled); }
        semio_framework_os_store_read::retirement_owner_close(&mut self.request, grant)
    }
    fn terminal_is_empty(&self) -> bool { self.request.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(semio_framework_os_store_read::retirement_owner_demands(&self.request, 0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(semio_framework_os_store_read::retirement_owner_demands(&self.request, body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(semio_framework_os_store_read::retirement_owner_demands(&self.request, 0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(semio_framework_os_store_read::retirement_owner_demands(&self.request, 0)?.depth) }
}

impl<P> Drop for UnsupportedMemberSnapshotOpen<P> {
    fn drop(&mut self) {
        assert!(self.request.is_none(), "unsupported member decoder dropped retained request authority");
    }
}

pub struct UnsupportedMemberFactoryOpen<M: Send> {
    pub source_loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,
    pub snapshot: UnsupportedMemberSnapshotOpen<M>,
}

impl<M: Send> UnsupportedMemberFactoryOpen<M> {
    pub fn begin(admission:&mut MemberOpenSourceAdmission,opened:&mut Option<Self>,cx:&mut StepContext<'_>,parent:&OriginalCallerSourceParent)->Result<bool,OriginalTurnFailure>{
        if opened.is_some(){return Err(member_admission_diagnostic(MemberOpenDiagnostic::Stale).into())}
        if !admission.admit_source(parent,cx)?||cx.should_yield()||cx.retained_grant().maximum_items==0{return Ok(false)}
        let request=admission.request.take().unwrap();
        match UnsupportedMemberSnapshotOpen::begin(request){Ok(snapshot)=>{*opened=Some(Self{snapshot,source_loan:ManuallyDrop::new(admission.source_loan.take())});semio_framework_os_store_member_open::receive_member_close(cx,Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})))?;Ok(true)},Err(rejected)=>{*admission.request=Some(rejected.request);Err(member_admission_diagnostic(rejected.diagnostic).into())}}
    }
}

impl<M: Send> MemberOpenOperation for UnsupportedMemberFactoryOpen<M> {
    type Member = M;

    fn step(&mut self, cx: &mut StepContext<'_>, _grant: RetainedCloneGrant) -> Result<MemberOpenStep<M>,OriginalTurnFailure> {
        self.source_loan.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.validate_context(cx)?;
        Ok(match self.snapshot.step(cx, _grant)? {
            MemberSnapshotOpenStep::Rejected(diagnostic) => MemberOpenStep::Rejected(diagnostic),
            _ => MemberOpenStep::Rejected(MemberOpenDiagnostic::Decode),
        })
    }

    fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>) -> Result<RetainedCloneStep, OriginalTurnFailure> {
        let empty=RetainedCloneProgress::default();
        if self.snapshot.terminal_is_empty(){if let Some(loan)=self.source_loan.as_mut(){let step=loan.close_step(parent,cx)?;let progress=step.progress();if loan.terminal_is_empty(){self.source_loan.take();}return Ok(if self.source_loan.is_none(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}return Ok(RetainedCloneStep::Complete(empty))}
        self.source_loan.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.validate_context(cx)?;
        let request=self.snapshot.request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"unsupported factory lost original request"))?;
        if cx.operation()!=request.operation()||cx.generation()!=request.generation(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unsupported factory close changed operation identity").into())}
        let mut grant=cx.retained_grant();
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"unsupported factory requires original child depth").into())}
        if cx.should_yield(){return Ok(RetainedCloneStep::Progress(empty))}
        grant.maximum_copy_bytes=grant.maximum_copy_bytes.min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        let result=self.snapshot.close_step(child).and_then(|step|admit_retained_clone_close(child,step,self.snapshot.terminal_is_empty(),"unsupported factory request"));
        semio_framework_os_store_member_open::receive_member_close(cx,result).map(|step|RetainedCloneStep::Progress(step.progress()))
    }
    fn terminal_is_empty(&self) -> bool { self.snapshot.terminal_is_empty()&&self.source_loan.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.snapshot.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.snapshot.next_capacity_byte_demand(body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.snapshot.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { if self.snapshot.terminal_is_empty() { return Ok(usize::from(self.source_loan.is_some())); } self.snapshot.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "unsupported factory child depth overflow")) }
    fn terminal_drop_byte_demand(&self) -> Option<usize> { self.terminal_is_empty().then_some(0) }
}

impl<M:Send> Drop for UnsupportedMemberFactoryOpen<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"unsupported factory abandoned original request or source loan")}}
