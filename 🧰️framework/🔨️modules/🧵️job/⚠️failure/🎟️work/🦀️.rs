//! 🎟️ Inline work failures preserve original causes until their existing paid domain admission.
use super::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,OriginalTurnFailure,OriginalFailureContext,ValueError,project_original_value_error,key_original_value_error,project_original_turn_failure,key_original_turn_failure};
use semio_framework_value_retained_clone::{Number,RetainedCloneProgress,ValueRefusalKind};

/// 🪢️ Body, actual receipt and complete source qualification causes retain independent custody.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
pub struct OriginalWorkFailure<E:OriginalDomainFailure>{primary:Option<E>,effects:RetainedCloneProgress,receipt_refusal:Option<ValueError>,context_refusal:Option<OriginalTurnFailure>}
fn qualify(cx:&impl OriginalFailureContext)->Result<(),OriginalTurnFailure>{cx.original_failure_identity().map(|_|()).map_err(Into::into)}
impl<E:OriginalDomainFailure> OriginalWorkFailure<E>{
 /// 🪪️ Qualifies the actual supplied context before any producer effects.
 pub fn qualify(cx:&impl OriginalFailureContext)->Result<(),Self>{qualify(cx).map_err(|context|Self{primary:None,effects:Default::default(),receipt_refusal:None,context_refusal:Some(context)})}
 /// 📊️ Posts actual effects once and spends actual CPU independently before retaining every refusal.
 pub fn fold(cx:&mut impl OriginalFailureContext,primary:Option<E>,effects:RetainedCloneProgress,cpu:u64)->Result<(),Self>{
  let receipt_refusal=cx.consume_retained(effects).err();cx.consume_fuel(cpu);let context_refusal=qualify(cx).err();
  if primary.is_none()&&receipt_refusal.is_none()&&context_refusal.is_none(){Ok(())}else{Err(Self{primary,effects,receipt_refusal,context_refusal})}
 }
 /// 🧬️ Borrows the original typed producer cause without erasing or cloning it.
 pub fn primary(&self)->Option<&E>{self.primary.as_ref()}
 /// 🧾️ Borrows the complete actual receipt refusal.
 pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref()}
 /// 🧳️ Borrows the complete original qualification failure.
 pub fn context_refusal(&self)->Option<&OriginalTurnFailure>{self.context_refusal.as_ref()}
 /// 📏️ Reports actual effects already posted by this turn independently of its cause tree.
 pub fn retained_progress(&self)->RetainedCloneProgress{self.effects}
 fn key(&self,index:usize)->Result<&'static str,ValueError>{[self.primary.as_ref().map(|_|"primary"),self.receipt_refusal.as_ref().map(|_|"receiptRefusal"),self.context_refusal.as_ref().map(|_|"contextRefusal"),Some("effects")].into_iter().flatten().nth(index).ok_or_else(invalid)}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original work failure projection ordinal changed")}
impl<E:OriginalDomainFailure> OriginalDomainFailure for OriginalWorkFailure<E>{
 fn domain_code(&self)->&str{"original.work"}
 fn domain_message(&self)->&str{if let Some(primary)=&self.primary{primary.domain_message()}else if let Some(receipt)=&self.receipt_refusal{&receipt.message}else{self.context_refusal.as_ref().expect("original work failure retains a cause").primary.message()}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
impl<E:OriginalDomainFailure> OriginalDomainProjection for OriginalWorkFailure<E>{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{
  let Some((&index,rest))=path.split_first()else{return Ok(V::IntrinsicObject(1+usize::from(self.primary.is_some())+usize::from(self.receipt_refusal.is_some())+usize::from(self.context_refusal.is_some())))};
  match self.key(index)?{
   "primary"=>self.primary.as_ref().ok_or_else(invalid)?.projection_view(rest),
   "receiptRefusal"=>project_original_value_error(self.receipt_refusal.as_ref().ok_or_else(invalid)?,rest),
   "contextRefusal"=>project_original_turn_failure(self.context_refusal.as_ref().ok_or_else(invalid)?,rest),
   "effects"=>match rest{[]=>Ok(V::IntrinsicObject(4)),[axis]=>[self.effects.copied_items,self.effects.copied_bytes,self.effects.retained_capacity_bytes,self.effects.released_bytes].get(*axis).map(|n|V::IntrinsicNumber(Number::UInt(*n as u64))).ok_or_else(invalid),_=>Err(invalid())},
   _=>Err(invalid())
  }
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  let Some((&field,rest))=path.split_first()else{return self.key(index)};
  match self.key(field)?{
   "primary"=>self.primary.as_ref().ok_or_else(invalid)?.projection_key(rest,index),
   "receiptRefusal"=>key_original_value_error(self.receipt_refusal.as_ref().ok_or_else(invalid)?,rest,index),
   "contextRefusal"=>key_original_turn_failure(self.context_refusal.as_ref().ok_or_else(invalid)?,rest,index),
   "effects" if rest.is_empty()=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(invalid),
   _=>Err(invalid())
  }
 }
}
