//! 🛬️ Construction retains semantic, quote and allocation causes inline before any refused receipt handoff.
use crate::ValueError;
use crate::{RetainedCloneProgress,retirement::{RetireOwned,RetirementCursor}};

/// 🧳️ Every actual construction cause remains separately owned without allocating an error frame after refusal.
#[derive(Debug)]
pub struct OriginalReceivingPrimary {
    pub primary:Option<ValueError>,
    pub quote_refusal:Option<ValueError>,
    pub allocation_refusal:Option<ValueError>,
    pub progress:RetainedCloneProgress,
}
impl OriginalReceivingPrimary {
    pub fn retained_progress(&self)->RetainedCloneProgress { self.progress }
    pub fn message(&self)->&str { self.primary.as_ref().or(self.quote_refusal.as_ref()).or(self.allocation_refusal.as_ref()).map_or("original native construction refused",|error|error.message.as_ref()) }
}
impl From<ValueError> for OriginalReceivingPrimary {
    fn from(error:ValueError)->Self { let progress=error.retained_progress();Self { primary:Some(error),quote_refusal:None,allocation_refusal:None,progress } }
}
impl RetireOwned for OriginalReceivingPrimary {
    fn retirement(self)->Box<dyn RetirementCursor> { crate::retirement::sequence(vec![crate::retirement::deferred(self.primary),crate::retirement::deferred(self.quote_refusal),crate::retirement::deferred(self.allocation_refusal)]) }
    fn retirement_birth_bytes(&self)->Option<usize> { crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.primary),crate::retirement::deferred_birth_bytes_for(&self.quote_refusal),crate::retirement::deferred_birth_bytes_for(&self.allocation_refusal)]) }
    fn controlled_retirement_supported()->bool { true }
}
