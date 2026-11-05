//! 🧾️ One native failure owns every original cause and the first refused source receipt without cloning or allocating a wrapper.
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,retirement::{RetireOwned,RetirementCursor,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes}};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
/// 🛬️ Construction causes keep their exact original domain, quote and allocation fields.
#[derive(Debug)]
pub enum NativeSnapshotCause{Value(ValueError),Receiving(OriginalReceivingPrimary)}
/// 🧳️ Body failure ends production; outer scopes forward this same owner and its actual work.
#[derive(Debug)]
pub struct NativeSnapshotFailure{
 pub(super) cause:Option<NativeSnapshotCause>,
 pub(super) receipt_refusal:Option<ValueError>,
 pub(super) context_refusal:Option<ValueError>,
 pub(super) settlement_refusal:Option<ValueError>,
 pub(super) progress:RetainedCloneProgress,
}
impl NativeSnapshotFailure{
 pub fn cause(&self)->Option<&NativeSnapshotCause>{self.cause.as_ref()}
 pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref()}
 pub fn context_refusal(&self)->Option<&ValueError>{self.context_refusal.as_ref()}
 /// 🫴️ Borrows the separately owned foreign settlement refusal.
 pub fn settlement_refusal(&self)->Option<&ValueError>{self.settlement_refusal.as_ref()}
 /// 🧳️ Adopts one settlement cause; a repeated handoff returns both original owners.
 pub fn with_settlement_refusal(mut self,error:ValueError)->Result<Self,(Self,ValueError)>{if self.settlement_refusal.is_some(){return Err((self,error))}self.settlement_refusal=Some(error);Ok(self)}
 pub fn kind(&self)->ValueRefusalKind{self.primary_value().map_or(ValueRefusalKind::InvalidValue,|error|error.kind)}
 pub fn message(&self)->&str{self.primary_value().map_or("original native snapshot refused",|error|error.message.as_ref())}
 fn primary_value(&self)->Option<&ValueError>{self.cause.as_ref().and_then(|cause|match cause{NativeSnapshotCause::Value(error)=>Some(error),NativeSnapshotCause::Receiving(error)=>error.primary.as_ref().or(error.quote_refusal.as_ref()).or(error.allocation_refusal.as_ref())}).or(self.receipt_refusal.as_ref()).or(self.context_refusal.as_ref()).or(self.settlement_refusal.as_ref())}
 /// 🧾️ Adopts the first failed receipt after its caller has cut further validation.
 pub fn with_receipt_refusal(mut self,error:ValueError)->Self{assert!(self.receipt_refusal.is_none(),"native failure already owns its first receipt refusal");self.receipt_refusal=Some(error);self}
 /// 👁️ Adopts the first original observer failure independently of the producer cause.
 pub fn with_context_refusal(mut self,error:ValueError)->Self{assert!(self.context_refusal.is_none(),"native failure already owns its first context refusal");self.context_refusal=Some(error);self}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn with_retained_progress(mut self,progress:RetainedCloneProgress)->Self{self.progress=progress;self}
 pub fn receiving_primary(&self)->Option<&OriginalReceivingPrimary>{match &self.cause{Some(NativeSnapshotCause::Receiving(error))=>Some(error),_=>None}}
 pub(super) fn receipt(error:ValueError)->Self{let progress=error.retained_progress();Self{cause:None,receipt_refusal:Some(error),context_refusal:None,settlement_refusal:None,progress}}
 pub(super) fn context(error:ValueError)->Self{let progress=error.retained_progress();Self{cause:None,receipt_refusal:None,context_refusal:Some(error),settlement_refusal:None,progress}}
}
impl From<ValueError> for NativeSnapshotFailure{fn from(error:ValueError)->Self{let progress=error.retained_progress();Self{cause:Some(NativeSnapshotCause::Value(error)),receipt_refusal:None,context_refusal:None,settlement_refusal:None,progress}}}
impl From<OriginalReceivingPrimary> for NativeSnapshotFailure{fn from(error:OriginalReceivingPrimary)->Self{let progress=error.retained_progress();Self{cause:Some(NativeSnapshotCause::Receiving(error)),receipt_refusal:None,context_refusal:None,settlement_refusal:None,progress}}}
impl std::fmt::Display for NativeSnapshotFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message())}}
impl std::error::Error for NativeSnapshotFailure{}
impl PartialEq for NativeSnapshotCause{fn eq(&self,other:&Self)->bool{match(self,other){(Self::Value(a),Self::Value(b))=>a==b,(Self::Receiving(a),Self::Receiving(b))=>a.primary==b.primary&&a.quote_refusal==b.quote_refusal&&a.allocation_refusal==b.allocation_refusal&&a.progress==b.progress,_=>false}}}
impl PartialEq for NativeSnapshotFailure{fn eq(&self,other:&Self)->bool{self.cause==other.cause&&self.receipt_refusal==other.receipt_refusal&&self.context_refusal==other.context_refusal&&self.settlement_refusal==other.settlement_refusal&&self.progress==other.progress}}
impl RetireOwned for NativeSnapshotCause{
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Value(error)=>error.retirement(),Self::Receiving(error)=>error.retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Value(error)=>error.retirement_birth_bytes(),Self::Receiving(error)=>error.retirement_birth_bytes()}}
 fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for NativeSnapshotFailure{
 fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.cause),deferred(self.receipt_refusal),deferred(self.context_refusal),deferred(self.settlement_refusal)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.cause),deferred_birth_bytes_for(&self.receipt_refusal),deferred_birth_bytes_for(&self.context_refusal),deferred_birth_bytes_for(&self.settlement_refusal)])}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 🧮️ Keeps semantic producer actions independent of physical receipt items.
#[derive(Default)]
pub struct NativeSnapshotWork(u64);
impl NativeSnapshotWork{
 /// 🚦️ Admits one bounded primitive through the original decoder loan before its effects.
 pub fn admit(&mut self,native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),NativeSnapshotFailure>{
  native.checkpoint()?;
  if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native snapshot primitive requires original work authority").into())}
  let result=native.commit_work(1);self.performed();result.map_err(Into::into)
 }
 /// 🫴️ Conserves a delegated work report before observing its owning result.
 pub fn take<T,E:Into<NativeSnapshotFailure>>(&mut self,turn:semio_framework_value::RetainedWorkOutcome<T,E>)->Result<T,NativeSnapshotFailure>{
  self.0=self.0.checked_add(turn.work).expect("performed native work fits the original caller ledger");turn.result.map_err(Into::into)
 }
 /// ✍️ Records one explicitly selected admitted action, including its possible refusal.
 pub fn performed(&mut self){self.0=self.0.checked_add(1).expect("performed native work fits the original caller ledger");}
}
/// 🧵️ Returns every completed action even when a subsequent action refuses.
pub fn native_snapshot_turn<T>(operation:impl FnOnce(&mut NativeSnapshotWork)->Result<T,NativeSnapshotFailure>)->semio_framework_value::RetainedWorkOutcome<T,NativeSnapshotFailure>{
 let mut work=NativeSnapshotWork::default();let result=operation(&mut work);semio_framework_value::RetainedWorkOutcome{result,work:work.0}
}

#[path="🔁️codec/🦀️.rs"]
mod codec;
