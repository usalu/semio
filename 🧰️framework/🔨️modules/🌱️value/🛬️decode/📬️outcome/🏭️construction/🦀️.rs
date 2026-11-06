//! 🧾️ Native construction retains its original typed result independently of receipt acceptance.
use crate::{ValueError,RetainedCloneProgress,RetainedCloneStep,retirement::RetireOwned};

/// 📦️ A bounded operation reports actual effects alongside its complete typed result.
pub struct NativeConstructionStepResult<T,E> { pub result:Result<T,E>,pub progress:RetainedCloneProgress,pub work:u128 }

/// 🪢️ Refused receipt acceptance never replaces or destroys an already produced typed result.
pub struct NativeConstructionStepOutcome<T,E> { pub result:Option<Result<T,E>>,pub receipt_refusal:Option<ValueError>,pub progress:RetainedCloneProgress,pub work:u128 }

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
 type RetirementCursor=<(Option<E>,Option<ValueError>)as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.primary.retirement_ready()&&self.receipt_refusal.retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{(self.primary,self.receipt_refusal).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<E> NativeConstructionStepOutcome<RetainedCloneStep,E> {
    pub fn into_step_outcome(self)->crate::RetainedWorkOutcome<RetainedCloneStep,NativeConstructionStepFailure<E>> {
        let primary=match self.result {
            Some(Ok(step)) if self.receipt_refusal.is_none()=>return crate::RetainedWorkOutcome{result:Ok(step),work:self.work},
            None if self.receipt_refusal.is_none()=>return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:self.work},
            Some(Err(error))=>Some(error),
            _=>None,
        };
        crate::RetainedWorkOutcome{result:Err(NativeConstructionStepFailure { primary,receipt_refusal:self.receipt_refusal,progress:self.progress }),work:self.work}
    }
}

