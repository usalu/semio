//! 🧳️ Typed receiving failures retain their child owner and every callback refusal together.
use crate::{ValueError,RetainedCloneProgress,OriginalReceivingPrimary};
/// 📏️ Selects actual physical progress without erasing a child error.
pub trait NativeReceivingEffectError:crate::retirement::RetireOwned+Send+'static{fn receiving_progress(&self)->RetainedCloneProgress;}
impl NativeReceivingEffectError for OriginalReceivingPrimary{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}}
impl NativeReceivingEffectError for ValueError{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}}
impl NativeReceivingEffectError for semio_framework_value_retirement::OriginalRetirementFailure{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}}
/// 🧳️ Keeps the entire typed child error and each independently refused receiving callback.
#[derive(Debug)]
pub struct NativeReceivingEffectFailure<E:NativeReceivingEffectError>{pub child:Option<E>,pub producer_refusal:Option<ValueError>,pub work_refusal:Option<ValueError>,pub receiving:OriginalReceivingPrimary}
impl<E:NativeReceivingEffectError> From<ValueError> for NativeReceivingEffectFailure<E>{fn from(error:ValueError)->Self{Self{child:None,producer_refusal:None,work_refusal:None,receiving:error.into()}}}
impl<E:NativeReceivingEffectError> NativeReceivingEffectFailure<E>{pub fn retained_progress(&self)->RetainedCloneProgress{self.receiving.retained_progress()}}
impl<E:NativeReceivingEffectError> NativeReceivingEffectError for NativeReceivingEffectFailure<E>{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}}
impl<E:NativeReceivingEffectError> crate::retirement::RetireOwned for NativeReceivingEffectFailure<E>{
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.child)&&crate::retirement::RetireOwned::retirement_ready(&self.producer_refusal)&&crate::retirement::RetireOwned::retirement_ready(&self.work_refusal)&&crate::retirement::RetireOwned::retirement_ready(&self.receiving)}
 type RetirementCursor=<(Option<E>,Option<ValueError>,Option<ValueError>,OriginalReceivingPrimary) as crate::retirement::RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed((self.child,self.producer_refusal,self.work_refusal,self.receiving))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{E::controlled_retirement_supported()}
}
