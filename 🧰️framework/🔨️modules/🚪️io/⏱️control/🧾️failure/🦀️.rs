//! 🧾️ One native failure owns every original cause and the first refused source receipt without cloning or allocating a wrapper.
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,retirement::{RetireOwned,RetirementCursor,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes}};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
/// 🛬️ Construction causes keep their exact original domain, quote and allocation fields.
#[derive(Debug)]
pub enum NativeSnapshotCause{Value(ValueError),Receiving(OriginalReceivingPrimary),Retirement(semio_framework_value::OriginalRetirementFailure)}
/// 📦️ Keeps every original allocation outcome without reducing its independent refusals.
pub enum NativeSnapshotAllocationFailure{Encode(semio_framework_value::native_encoding::NativeEncodeAllocationOutcome),Decode(semio_framework_value_native_owned::native_decoding::NativeDecodeAllocationOutcome)}
impl NativeSnapshotAllocationFailure{fn primary(&self)->Option<&ValueError>{match self{Self::Encode(value)=>value.primary.as_ref().or(value.forwarded_quote.as_ref()).or(value.scoped_quote.as_ref()).or(value.extent_refusal.as_ref()).or(value.forwarded_refusal.as_ref()).or(value.scoped_refusal.as_ref()),Self::Decode(value)=>value.primary.as_ref().or(value.quote_refusal.as_ref()).or(value.extent_refusal.as_ref()).or(value.settlement_refusal.as_ref())}}}
impl std::fmt::Debug for NativeSnapshotAllocationFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Encode(v)=>f.debug_struct("Encode").field("request",&v.request).field("admitted",&v.admitted).field("primary",&v.primary).field("forwarded_quote",&v.forwarded_quote).field("scoped_quote",&v.scoped_quote).field("extent_refusal",&v.extent_refusal).field("forwarded_refusal",&v.forwarded_refusal).field("scoped_refusal",&v.scoped_refusal).field("actual",&v.actual).finish(),Self::Decode(v)=>f.debug_struct("Decode").field("request",&v.request).field("admitted",&v.admitted).field("primary",&v.primary).field("quote_refusal",&v.quote_refusal).field("extent_refusal",&v.extent_refusal).field("settlement_refusal",&v.settlement_refusal).field("actual",&v.actual).finish()}}}
impl PartialEq for NativeSnapshotAllocationFailure{fn eq(&self,other:&Self)->bool{match(self,other){(Self::Encode(a),Self::Encode(b))=>a.request==b.request&&a.admitted==b.admitted&&a.primary==b.primary&&a.forwarded_quote==b.forwarded_quote&&a.scoped_quote==b.scoped_quote&&a.extent_refusal==b.extent_refusal&&a.forwarded_refusal==b.forwarded_refusal&&a.scoped_refusal==b.scoped_refusal&&a.actual==b.actual,(Self::Decode(a),Self::Decode(b))=>a.request==b.request&&a.admitted==b.admitted&&a.quote_refusal==b.quote_refusal&&a.primary==b.primary&&a.extent_refusal==b.extent_refusal&&a.settlement_refusal==b.settlement_refusal&&a.actual==b.actual,_=>false}}}
impl RetireOwned for NativeSnapshotAllocationFailure{fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Encode(value)=>value.retirement(),Self::Decode(value)=>value.retirement()}}fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Encode(value)=>value.retirement_birth_bytes(),Self::Decode(value)=>value.retirement_birth_bytes()}}fn controlled_retirement_supported()->bool{true}}
/// 🧳️ Body failure ends production; outer scopes forward this same owner and its actual work.
#[derive(Debug)]
pub struct NativeSnapshotFailure{
 pub(super) cause:Option<NativeSnapshotCause>,
 pub(super) allocation:Option<NativeSnapshotAllocationFailure>,
 pub(super) encode_effect:Option<semio_framework_value::native_encoding::NativeEncodeEffectFailure>,
 pub(super) receipt_refusal:Option<ValueError>,
 pub(super) context_refusal:Option<ValueError>,
 pub(super) work_refusal:Option<ValueError>,
 pub(super) settlement_refusal:Option<ValueError>,
 pub(super) progress:RetainedCloneProgress,
}
impl NativeSnapshotFailure{
 /// 🎟️ Retains the complete original encoder effect failure and its actual physical progress.
 pub fn from_encode_effect(effect:semio_framework_value::native_encoding::NativeEncodeEffectFailure,progress:RetainedCloneProgress)->Self{Self{cause:None,allocation:None,encode_effect:Some(effect),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}
 /// 👁️ Borrows every independent original encoder effect cause.
 pub fn encode_effect(&self)->Option<&semio_framework_value::native_encoding::NativeEncodeEffectFailure>{self.encode_effect.as_ref()}

 /// 📦️ Borrows the actual original allocation outcome with every independent refusal.
 pub fn allocation(&self)->Option<&NativeSnapshotAllocationFailure>{self.allocation.as_ref()}
 /// 🧳️ Adopts one original allocation outcome or returns both owners if already occupied.
 pub fn with_allocation(mut self,allocation:NativeSnapshotAllocationFailure)->Result<Self,(Self,NativeSnapshotAllocationFailure)>{if self.allocation.is_some(){return Err((self,allocation))}self.allocation=Some(allocation);Ok(self)}
 /// 🏗️ Creates an original failure retaining the complete allocation outcome.
 pub fn from_allocation(allocation:NativeSnapshotAllocationFailure)->Self{Self{cause:None,allocation:Some(allocation),encode_effect:None,receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress:Default::default()}}
 pub fn cause(&self)->Option<&NativeSnapshotCause>{self.cause.as_ref()}
 pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref().or_else(||self.encode_effect.as_ref().and_then(|effect|effect.physical.as_ref())).or_else(||match &self.cause{Some(NativeSnapshotCause::Receiving(error))=>error.receipt_refusal.as_ref(),Some(NativeSnapshotCause::Retirement(error))=>error.receipt_refusal(),_=>None})}
 pub fn context_refusal(&self)->Option<&ValueError>{self.context_refusal.as_ref().or_else(||self.encode_effect.as_ref().and_then(|effect|effect.invariant.as_ref().or(effect.physical_invariant.as_ref()).or(effect.capacity_invariant.as_ref())))}
 /// 🛠️ Borrows the independently retained original work-post refusal.
 pub fn work_refusal(&self)->Option<&ValueError>{self.work_refusal.as_ref().or_else(||self.encode_effect.as_ref().and_then(|effect|effect.work.as_ref())).or_else(||match &self.cause{Some(NativeSnapshotCause::Retirement(error))=>error.work_refusal(),_=>None})}
 pub fn retirement_failure(&self)->Option<&semio_framework_value::OriginalRetirementFailure>{match &self.cause{Some(NativeSnapshotCause::Retirement(error))=>Some(error),_=>None}}
 /// 🛠️ Creates a failure owning the original work-post refusal without changing its classification.
 pub fn work_post(error:ValueError)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:None,receipt_refusal:None,context_refusal:None,work_refusal:Some(error),settlement_refusal:None,progress}}
 /// 🪢️ Retains an original operation failure and its independent work-post failure in one fresh owner.
 pub fn primary_with_work(primary:ValueError,work:ValueError)->Self{let progress=primary.retained_progress();Self{encode_effect:None,allocation:None,cause:Some(NativeSnapshotCause::Value(primary)),receipt_refusal:None,context_refusal:None,work_refusal:Some(work),settlement_refusal:None,progress}}
 /// 🧳️ Adopts one work-post refusal or returns both owners when the slot is already occupied.
 pub fn with_work_post(mut self,error:ValueError)->Result<Self,(Self,ValueError)>{if self.work_refusal.is_some(){return Err((self,error))}self.work_refusal=Some(error);Ok(self)}
 /// 🫴️ Borrows the separately owned foreign settlement refusal.
 pub fn settlement_refusal(&self)->Option<&ValueError>{self.settlement_refusal.as_ref().or_else(||self.encode_effect.as_ref().and_then(|effect|effect.local.as_ref()))}
 /// 🧳️ Adopts one settlement cause; a repeated handoff returns both original owners.
 pub fn with_settlement_refusal(mut self,error:ValueError)->Result<Self,(Self,ValueError)>{if self.settlement_refusal.is_some(){return Err((self,error))}self.settlement_refusal=Some(error);Ok(self)}
 pub fn kind(&self)->ValueRefusalKind{self.primary_value().map_or(ValueRefusalKind::InvalidValue,|error|error.kind)}
 pub fn message(&self)->&str{self.primary_value().map_or("original native snapshot refused",|error|error.message.as_ref())}
 fn primary_value(&self)->Option<&ValueError>{self.cause.as_ref().and_then(|cause|match cause{NativeSnapshotCause::Value(error)=>Some(error),NativeSnapshotCause::Receiving(error)=>error.primary.as_ref().or(error.quote_refusal.as_ref()).or(error.allocation_refusal.as_ref()).or_else(||error.allocation_failure.as_ref().and_then(|failure|failure.primary.as_ref().or(failure.admission.as_ref()).or(failure.forwarded_admission.as_ref()).or(failure.scoped_admission.as_ref()).or(failure.forwarded_quote.as_ref()).or(failure.scoped_quote.as_ref()).or(failure.forwarded_post.as_ref()).or(failure.scoped_post.as_ref()).or(failure.forwarded_cancel.as_ref()).or(failure.scoped_cancel.as_ref()).or(failure.receipt.as_ref()).or(failure.extent.as_ref()))).or_else(||error.native_allocation.as_ref().and_then(|value|value.primary.as_ref().or(value.quote_refusal.as_ref()).or(value.extent_refusal.as_ref()).or(value.settlement_refusal.as_ref()))).or(error.receipt_refusal.as_ref()),NativeSnapshotCause::Retirement(error)=>retirement_primary(error)}).or_else(||self.encode_effect.as_ref().and_then(encode_effect_primary)).or_else(||self.allocation.as_ref().and_then(NativeSnapshotAllocationFailure::primary)).or(self.receipt_refusal.as_ref()).or(self.context_refusal.as_ref()).or(self.work_refusal.as_ref()).or(self.settlement_refusal.as_ref())}
 /// 🧾️ Adopts the first failed receipt after its caller has cut further validation.
 pub fn with_receipt_refusal(mut self,error:ValueError)->Self{assert!(self.receipt_refusal.is_none(),"native failure already owns its first receipt refusal");self.receipt_refusal=Some(error);self}
 /// 👁️ Adopts the first original observer failure independently of the producer cause.
 pub fn with_context_refusal(mut self,error:ValueError)->Self{assert!(self.context_refusal.is_none(),"native failure already owns its first context refusal");self.context_refusal=Some(error);self}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn with_retained_progress(mut self,progress:RetainedCloneProgress)->Self{self.progress=progress;self}
 pub fn receiving_primary(&self)->Option<&OriginalReceivingPrimary>{match &self.cause{Some(NativeSnapshotCause::Receiving(error))=>Some(error),_=>None}}
 pub(super) fn receipt(error:ValueError)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:None,receipt_refusal:Some(error),context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}
 pub(super) fn context(error:ValueError)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:None,receipt_refusal:None,context_refusal:Some(error),work_refusal:None,settlement_refusal:None,progress}}
}
impl From<semio_framework_value::native_encoding::NativeEncodeOperationFailure> for NativeSnapshotFailure{fn from(error:semio_framework_value::native_encoding::NativeEncodeOperationFailure)->Self{match(error.primary,error.work_refusal){(Some(primary),Some(work))=>Self::primary_with_work(primary,work),(Some(primary),None)=>Self::from(primary),(None,Some(work))=>Self::work_post(work),(None,None)=>Self::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode failure has no cause"))}}}
impl From<ValueError> for NativeSnapshotFailure{fn from(error:ValueError)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:Some(NativeSnapshotCause::Value(error)),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}}
impl From<semio_framework_value::OriginalRetirementFailure> for NativeSnapshotFailure{fn from(error:semio_framework_value::OriginalRetirementFailure)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:Some(NativeSnapshotCause::Retirement(error)),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}}
fn encode_effect_primary(error:&semio_framework_value::native_encoding::NativeEncodeEffectFailure)->Option<&ValueError>{error.primary.as_ref().or(error.invariant.as_ref()).or(error.physical_invariant.as_ref()).or(error.capacity_invariant.as_ref()).or(error.work.as_ref()).or(error.local.as_ref()).or(error.physical.as_ref()).or_else(||error.allocation.as_ref().and_then(|allocation|allocation.primary.as_ref().or(allocation.forwarded_quote.as_ref()).or(allocation.scoped_quote.as_ref()).or(allocation.extent_refusal.as_ref()).or(allocation.forwarded_refusal.as_ref()).or(allocation.scoped_refusal.as_ref())))}
fn retirement_primary(mut error:&semio_framework_value::OriginalRetirementFailure)->Option<&ValueError>{let mut fallback=None;loop{if let Some(primary)=error.primary(){return Some(primary)}fallback=error.receipt_refusal().or(error.work_refusal()).or(fallback);match error.child(){Some(child)=>error=child,None=>return fallback}}}
impl From<OriginalReceivingPrimary> for NativeSnapshotFailure{fn from(error:OriginalReceivingPrimary)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:Some(NativeSnapshotCause::Receiving(error)),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}}
impl std::fmt::Display for NativeSnapshotFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message())}}
impl std::error::Error for NativeSnapshotFailure{}
impl PartialEq for NativeSnapshotCause{fn eq(&self,other:&Self)->bool{match(self,other){(Self::Value(a),Self::Value(b))=>a==b,(Self::Retirement(a),Self::Retirement(b))=>a==b,(Self::Receiving(a),Self::Receiving(b))=>a.primary==b.primary&&a.quote_refusal==b.quote_refusal&&a.allocation_refusal==b.allocation_refusal&&a.allocation_failure==b.allocation_failure&&(match(&a.native_allocation,&b.native_allocation){(None,None)=>true,(Some(a),Some(b))=>a.request==b.request&&a.admitted==b.admitted&&a.quote_refusal==b.quote_refusal&&a.primary==b.primary&&a.extent_refusal==b.extent_refusal&&a.settlement_refusal==b.settlement_refusal&&a.actual==b.actual,_=>false})&&a.receipt_refusal==b.receipt_refusal&&a.progress==b.progress,_=>false}}}
impl PartialEq for NativeSnapshotFailure{fn eq(&self,other:&Self)->bool{self.cause==other.cause&&self.allocation==other.allocation&&self.encode_effect==other.encode_effect&&self.receipt_refusal==other.receipt_refusal&&self.context_refusal==other.context_refusal&&self.work_refusal==other.work_refusal&&self.settlement_refusal==other.settlement_refusal&&self.progress==other.progress}}
impl RetireOwned for NativeSnapshotCause{
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Value(error)=>error.retirement(),Self::Receiving(error)=>error.retirement(),Self::Retirement(error)=>error.retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Value(error)=>error.retirement_birth_bytes(),Self::Receiving(error)=>error.retirement_birth_bytes(),Self::Retirement(error)=>error.retirement_birth_bytes()}}
 fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for NativeSnapshotFailure{
 fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.cause),deferred(self.allocation),deferred(self.encode_effect),deferred(self.receipt_refusal),deferred(self.context_refusal),deferred(self.work_refusal),deferred(self.settlement_refusal)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.cause),deferred_birth_bytes_for(&self.allocation),deferred_birth_bytes_for(&self.encode_effect),deferred_birth_bytes_for(&self.receipt_refusal),deferred_birth_bytes_for(&self.context_refusal),deferred_birth_bytes_for(&self.work_refusal),deferred_birth_bytes_for(&self.settlement_refusal)])}
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
  let result=native.commit_work(1);self.performed();result.map_err(NativeSnapshotFailure::work_post)
 }
 /// 🛫️ Admits one real encoder action against its distinct caller loan.
 pub fn admit_encode(&mut self,native:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),NativeSnapshotFailure>{
  native.checkpoint()?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native snapshot encoder action requires original work authority").into())}
  let result=native.commit_work(1);self.performed();result.map_err(NativeSnapshotFailure::work_post)
 }
 /// 🔭️ Preserves measured encoder child effects even if that child refuses.
 pub fn measured_encode<T,E:Into<NativeSnapshotFailure>>(&mut self,native:&mut semio_framework_value::NativeEncodeControl<'_>,operation:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,E>)->Result<T,NativeSnapshotFailure>{
  let before=native.actual_work();let result=operation(native);let actual=native.actual_work().checked_sub(before).expect("native encoder work remains cumulative");self.0=self.0.checked_add(actual).expect("performed native work fits the original caller ledger");result.map_err(Into::into)
 }
 /// 🫴️ Conserves a delegated work report before observing its owning result.
 pub fn take<T,E:Into<NativeSnapshotFailure>>(&mut self,turn:semio_framework_value::RetainedWorkOutcome<T,E>)->Result<T,NativeSnapshotFailure>{
  self.0=self.0.checked_add(turn.work).expect("performed native work fits the original caller ledger");turn.result.map_err(Into::into)
 }
 /// 🔭️ Preserves the original decoder’s committed child work on success and refusal.
 pub fn measured<T,E:Into<NativeSnapshotFailure>>(&mut self,native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<T,E>)->Result<T,NativeSnapshotFailure>{
  let before=native.actual_work();let result=operation(native);let actual=native.actual_work().checked_sub(before).expect("native work remains cumulative");self.0=self.0.checked_add(actual).expect("performed native work fits the original caller ledger");result.map_err(Into::into)
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
