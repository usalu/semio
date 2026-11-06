//! 🛬️ Construction retains semantic, quote and allocation causes inline before any refused receipt handoff.
use crate::ValueError;
use crate::{RetainedCloneProgress,retirement::{RetireOwned,RetirementCursor}};

/// 🧳️ Every actual construction cause remains separately owned without allocating an error frame after refusal.
#[derive(Debug)]
pub struct OriginalReceivingPrimary {
    pub primary:Option<ValueError>,
    pub quote_refusal:Option<ValueError>,
    pub allocation_refusal:Option<ValueError>,
    pub work_refusal:Option<ValueError>,
    pub work_overrun:Option<ValueError>,
    pub retirement_failure:Option<crate::OriginalRetirementFailure>,
    pub allocation_failure:Option<semio_framework_value_control_contract::NativeAllocationFailure>,
    pub native_allocation:Option<crate::NativeDecodeAllocationOutcome>,
    pub receipt_refusal:Option<ValueError>,
    pub progress:RetainedCloneProgress,
}
impl OriginalReceivingPrimary {
    pub fn retained_progress(&self)->RetainedCloneProgress { self.progress }
    pub fn message(&self)->&str { let allocation=self.allocation_failure.as_ref().and_then(|v|v.primary.as_ref().or(v.admission.as_ref()).or(v.forwarded_admission.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.scoped_admission.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.forwarded_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.scoped_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.forwarded_post.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.scoped_post.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.forwarded_cancel.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.scoped_cancel.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.receipt.as_ref()).or(v.extent.as_ref()));let native=self.native_allocation.as_ref().and_then(|v|v.primary.as_ref().or(v.quote_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(v.extent_refusal.as_ref()).or(v.settlement_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)));self.primary.as_ref().or(self.quote_refusal.as_ref()).or(self.allocation_refusal.as_ref()).or(self.work_refusal.as_ref()).or(self.work_overrun.as_ref()).or(allocation).or(native).or(self.receipt_refusal.as_ref()).map_or_else(||{let mut child=self.retirement_failure.as_ref();while let Some(error)=child{if let Some(cause)=error.primary().or(error.work_refusal()).or(error.receipt_refusal()).or(error.allocation_refusal().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)){return cause.message.as_ref()}child=error.child()}"original native construction refused"},|error|error.message.as_ref()) }
}
impl From<ValueError> for OriginalReceivingPrimary {
    fn from(error:ValueError)->Self { let progress=error.retained_progress();Self { primary:Some(error),quote_refusal:None,allocation_refusal:None,work_refusal:None,work_overrun:None,retirement_failure:None,allocation_failure:None,native_allocation:None,receipt_refusal:None,progress } }
}
impl From<crate::OriginalRetirementFailure> for OriginalReceivingPrimary {
    fn from(error:crate::OriginalRetirementFailure)->Self { let progress=error.retained_progress();Self { primary:None,quote_refusal:None,allocation_refusal:None,work_refusal:None,work_overrun:None,retirement_failure:Some(error),allocation_failure:None,native_allocation:None,receipt_refusal:None,progress } }
}
impl RetireOwned for OriginalReceivingPrimary {
    fn retirement_ready(&self)->bool{self.retirement_failure.retirement_ready()&&self.allocation_failure.retirement_ready()&&self.native_allocation.retirement_ready()}
    type RetirementCursor=<(Option<ValueError>,Option<ValueError>,Option<ValueError>,Option<ValueError>,Option<ValueError>,Option<crate::OriginalRetirementFailure>,Option<semio_framework_value_control_contract::NativeAllocationFailure>,Option<crate::NativeDecodeAllocationOutcome>,Option<ValueError>) as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{(self.primary,self.quote_refusal,self.allocation_refusal,self.work_refusal,self.work_overrun,self.retirement_failure,self.allocation_failure,self.native_allocation,self.receipt_refusal).retirement_seed()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(size_of::<Self::RetirementCursor>())}
    fn controlled_retirement_supported()->bool { true }
}

