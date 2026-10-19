//! 🧾️ Native construction retains its original typed result independently of receipt acceptance.
use crate::{ValueError,retained_clone::{RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor}};

/// 📦️ A bounded operation reports actual effects alongside its complete typed result.
pub struct NativeConstructionStepResult<T,E> { pub result:Result<T,E>,pub progress:RetainedCloneProgress }

/// 🪢️ Refused receipt acceptance never replaces or destroys an already produced typed result.
pub struct NativeConstructionStepOutcome<T,E> { pub result:Option<Result<T,E>>,pub receipt_refusal:Option<ValueError>,pub progress:RetainedCloneProgress }

/// 🧯️ Step metadata failures preserve the actual primary and receipt refusal as separate owners.
#[derive(Debug)]
pub struct NativeConstructionStepFailure<E> { pub primary:Option<E>,pub receipt_refusal:Option<ValueError>,pub progress:RetainedCloneProgress }
impl<E> NativeConstructionStepFailure<E> {
    pub fn retained_progress(&self)->RetainedCloneProgress { self.progress }
}
impl<E:From<ValueError>> From<ValueError> for NativeConstructionStepFailure<E> {
    fn from(error:ValueError)->Self { let progress=error.retained_progress();Self { primary:Some(error.into()),receipt_refusal:None,progress } }
}
impl<E:RetireOwned> RetireOwned for NativeConstructionStepFailure<E> {
    fn retirement(self)->Box<dyn RetirementCursor> { crate::retirement::sequence(vec![crate::retirement::deferred(self.primary),crate::retirement::deferred(self.receipt_refusal)]) }
    fn retirement_birth_bytes(&self)->Option<usize> { crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.primary),crate::retirement::deferred_birth_bytes_for(&self.receipt_refusal)]) }
    fn controlled_retirement_supported()->bool { true }
}
impl<E> NativeConstructionStepOutcome<RetainedCloneStep,E> {
    pub fn into_step_result(self)->Result<RetainedCloneStep,NativeConstructionStepFailure<E>> {
        let primary=match self.result {
            Some(Ok(step)) if self.receipt_refusal.is_none()=>return Ok(step),
            None if self.receipt_refusal.is_none()=>return Ok(RetainedCloneStep::Progress(Default::default())),
            Some(Err(error))=>Some(error),
            _=>None,
        };
        Err(NativeConstructionStepFailure { primary,receipt_refusal:self.receipt_refusal,progress:self.progress })
    }
}

#[path="🛬️receiving/🦀️.rs"]
mod receiving;
pub use receiving::OriginalReceivingPrimary;
