//! 🧳️ Typed receiving failures retain their child owner and every callback refusal together.
use crate::{ValueError,RetainedCloneProgress,OriginalReceivingPrimary};
/// 📏️ Selects actual physical progress without erasing a child error.
pub trait NativeReceivingEffectError:crate::retirement::RetireOwned+Send+'static{fn receiving_progress(&self)->RetainedCloneProgress;
 /// 🔎️ Names the first typed cause this failure carries, so a boundary refusal never hides it.
 fn cause(&self)->Option<&ValueError>{None}}
impl NativeReceivingEffectError for OriginalReceivingPrimary{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}fn cause(&self)->Option<&ValueError>{self.primary.as_ref().or(self.quote_refusal.as_ref()).or(self.allocation_refusal.as_ref()).or(self.work_refusal.as_ref()).or(self.work_overrun.as_ref()).or(self.receipt_refusal.as_ref())}}
impl NativeReceivingEffectError for ValueError{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}fn cause(&self)->Option<&ValueError>{Some(self)}}
impl NativeReceivingEffectError for semio_framework_value_retirement::OriginalRetirementFailure{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}}
/// 🛂️ Borrows the actual accepted completion and finished original Work proof after their issuers validate them.
pub struct OriginalConstructionAcceptedProof<'a>{completion:&'a semio_framework_value_control_contract::OriginalConstructionCompletion,proof:&'a semio_framework_value_control_contract::OriginalWorkActionProof}
impl OriginalConstructionAcceptedProof<'_>{
 pub fn authenticate<'a>(loan:&semio_framework_value_control_contract::OriginalConstructionLoan<'_>,completion:&'a semio_framework_value_control_contract::OriginalConstructionCompletion,work:&semio_framework_value_control_contract::OriginalWorkBorrow<'_>,proof:&'a semio_framework_value_control_contract::OriginalWorkActionProof)->Result<Option<OriginalConstructionAcceptedProof<'a>>,crate::ValueError>{
  use semio_framework_value_control_contract::OriginalConstructionAcceptStatus;
  if !completion.is_published()||completion.status()!=Some(OriginalConstructionAcceptStatus::Accepted)||!loan.validates_completion(completion)?||!proof.is_finished(){return Ok(None)}
  let Some(binding)=completion.work_binding()else{return Ok(None)};if work.binding_identity()?!=binding||!work.validates_finished(proof,proof.actual_work())?{return Ok(None)}Ok(Some(OriginalConstructionAcceptedProof{completion,proof}))
 }
 pub fn action_identity(&self)->semio_framework_value_control_contract::OriginalConstructionActionIdentity{self.completion.action_identity().expect("accepted completion retained")}
 pub fn work_binding(&self)->semio_framework_value_control_contract::OriginalWorkBindingIdentity{self.completion.work_binding().expect("accepted Work retained")}
 pub fn work_action(&self)->semio_framework_value_control_contract::OriginalWorkActionIdentity{self.proof.action_identity()}
}
/// 🧳️ Keeps the entire typed child error and each independently refused receiving callback.
#[derive(Debug)]
pub struct NativeReceivingEffectFailure<E:NativeReceivingEffectError>{pub child:Option<E>,pub producer_refusal:Option<ValueError>,pub work_refusal:Option<ValueError>,pub receiving:OriginalReceivingPrimary}
impl<E:NativeReceivingEffectError> From<ValueError> for NativeReceivingEffectFailure<E>{fn from(error:ValueError)->Self{Self{child:None,producer_refusal:None,work_refusal:None,receiving:error.into()}}}
impl<E:NativeReceivingEffectError> NativeReceivingEffectFailure<E>{pub fn retained_progress(&self)->RetainedCloneProgress{self.receiving.retained_progress()}}
impl<E:NativeReceivingEffectError> NativeReceivingEffectError for NativeReceivingEffectFailure<E>{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}fn cause(&self)->Option<&ValueError>{self.child.as_ref().and_then(NativeReceivingEffectError::cause).or(self.producer_refusal.as_ref()).or(self.work_refusal.as_ref()).or_else(||self.receiving.cause())}}
impl<E:NativeReceivingEffectError> crate::retirement::RetireOwned for NativeReceivingEffectFailure<E>{
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.child)&&crate::retirement::RetireOwned::retirement_ready(&self.producer_refusal)&&crate::retirement::RetireOwned::retirement_ready(&self.work_refusal)&&crate::retirement::RetireOwned::retirement_ready(&self.receiving)}
 type RetirementCursor=<(Option<E>,Option<ValueError>,Option<ValueError>,OriginalReceivingPrimary) as crate::retirement::RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed((self.child,self.producer_refusal,self.work_refusal,self.receiving))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{E::controlled_retirement_supported()}
}
