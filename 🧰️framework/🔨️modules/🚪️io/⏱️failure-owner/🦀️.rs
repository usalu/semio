//! 🧾️ One native failure owns every original cause and the first refused source receipt without cloning or allocating a wrapper.
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
/// 🛬️ Construction causes keep their exact original domain, quote and allocation fields.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum NativeSnapshotCause{Value(ValueError),Receiving(OriginalReceivingPrimary),Retirement(semio_framework_value::OriginalRetirementFailure)}
/// 🧩️ Selects original cause owners while preserving each variant’s exact record shape.
#[doc(hidden)]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
#[value(tag="type",rename_all="camelCase")]
pub enum NativeSnapshotCauseFields{
 Value{value:ValueError},
 Receiving{#[value(flatten)]owner:OriginalReceivingPrimary},
 Retirement{value:semio_framework_value::OriginalRetirementFailure},
}
impl semio_framework_value_native_owned::native_decoding::owned::NativeReceiveMap<NativeSnapshotCauseFields> for NativeSnapshotCause{
 fn from_native(fields:NativeSnapshotCauseFields)->Self{match fields{NativeSnapshotCauseFields::Value{value}=>Self::Value(value),NativeSnapshotCauseFields::Receiving{owner}=>Self::Receiving(owner),NativeSnapshotCauseFields::Retirement{value}=>Self::Retirement(value)}}
}
impl<I:semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource+?Sized> semio_framework_value_native_owned::NativeReceive<I> for NativeSnapshotCause{
 type Receiver=semio_framework_value_native_owned::native_decoding::owned::NativeMappedReceiver<I,NativeSnapshotCauseFields,Self>;
 fn receiver()->Self::Receiver{semio_framework_value_native_owned::native_decoding::owned::NativeMappedReceiver::new()}
}
/// 📦️ Keeps every original allocation outcome without reducing its independent refusals.
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
#[value(tag="type",rename_all="camelCase")]
pub enum NativeSnapshotAllocationFailure{Encode(semio_framework_value::native_encoding::NativeEncodeAllocationOutcome),Decode(semio_framework_value_native_owned::native_decoding::NativeDecodeAllocationOutcome)}
impl NativeSnapshotAllocationFailure{fn primary(&self)->Option<&ValueError>{match self{Self::Encode(value)=>value.primary.as_ref().or(value.forwarded_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(value.scoped_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(value.extent_refusal.as_ref()).or(value.forwarded_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(value.scoped_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)),Self::Decode(value)=>value.primary.as_ref().or(value.quote_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(value.extent_refusal.as_ref()).or(value.settlement_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned))}}}
impl std::fmt::Debug for NativeSnapshotAllocationFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Encode(v)=>f.debug_struct("Encode").field("request",&v.request).field("admitted",&v.admitted).field("primary",&v.primary).field("forwarded_quote",&v.forwarded_quote).field("scoped_quote",&v.scoped_quote).field("extent_refusal",&v.extent_refusal).field("forwarded_refusal",&v.forwarded_refusal).field("scoped_refusal",&v.scoped_refusal).field("actual",&v.actual).finish(),Self::Decode(v)=>f.debug_struct("Decode").field("request",&v.request).field("admitted",&v.admitted).field("primary",&v.primary).field("quote_refusal",&v.quote_refusal).field("extent_refusal",&v.extent_refusal).field("settlement_refusal",&v.settlement_refusal).field("actual",&v.actual).finish()}}}
impl PartialEq for NativeSnapshotAllocationFailure{fn eq(&self,other:&Self)->bool{match(self,other){(Self::Encode(a),Self::Encode(b))=>a.request==b.request&&a.admitted==b.admitted&&a.primary==b.primary&&a.forwarded_quote==b.forwarded_quote&&a.scoped_quote==b.scoped_quote&&a.extent_refusal==b.extent_refusal&&a.forwarded_refusal==b.forwarded_refusal&&a.scoped_refusal==b.scoped_refusal&&a.actual==b.actual,(Self::Decode(a),Self::Decode(b))=>a.request==b.request&&a.admitted==b.admitted&&a.quote_refusal==b.quote_refusal&&a.primary==b.primary&&a.extent_refusal==b.extent_refusal&&a.settlement_refusal==b.settlement_refusal&&a.actual==b.actual,_=>false}}}

/// 🧳️ Body failure ends production; outer scopes forward this same owner and its actual work.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct NativeSnapshotFailure{
 cause:Option<NativeSnapshotCause>,
 allocation:Option<NativeSnapshotAllocationFailure>,
 encode_effect:Option<semio_framework_value::native_encoding::NativeEncodeEffectFailure>,
 receipt_refusal:Option<ValueError>,
 context_refusal:Option<ValueError>,
 work_refusal:Option<ValueError>,
 settlement_refusal:Option<ValueError>,
 progress:RetainedCloneProgress,
}
/// 🪞️ The raw original fields of one snapshot failure, borrowed in declaration order.
pub struct NativeSnapshotFailureFields<'a>{pub cause:Option<&'a NativeSnapshotCause>,pub allocation:Option<&'a NativeSnapshotAllocationFailure>,pub encode_effect:Option<&'a semio_framework_value::native_encoding::NativeEncodeEffectFailure>,pub receipt_refusal:Option<&'a ValueError>,pub context_refusal:Option<&'a ValueError>,pub work_refusal:Option<&'a ValueError>,pub settlement_refusal:Option<&'a ValueError>,pub progress:&'a RetainedCloneProgress}
/// 🧩️ Owns a partially received failure until its original decoder publishes it.
#[derive(Default,semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct NativeSnapshotFailureParts{#[value(required)]pub cause:Option<NativeSnapshotCause>,#[value(required)]pub allocation:Option<NativeSnapshotAllocationFailure>,#[value(required)]pub encode_effect:Option<semio_framework_value::native_encoding::NativeEncodeEffectFailure>,#[value(required)]pub receipt_refusal:Option<ValueError>,#[value(required)]pub context_refusal:Option<ValueError>,#[value(required)]pub work_refusal:Option<ValueError>,#[value(required)]pub settlement_refusal:Option<ValueError>,pub progress:RetainedCloneProgress}
impl NativeSnapshotFailureParts{
 /// 📦️ Transfers all original fields into their completed failure owner.
 pub fn into_failure(self)->NativeSnapshotFailure{NativeSnapshotFailure{cause:self.cause,allocation:self.allocation,encode_effect:self.encode_effect,receipt_refusal:self.receipt_refusal,context_refusal:self.context_refusal,work_refusal:self.work_refusal,settlement_refusal:self.settlement_refusal,progress:self.progress}}
}
impl semio_framework_value_native_owned::native_decoding::owned::NativeReceiveMap<NativeSnapshotFailureParts> for NativeSnapshotFailure{
 fn from_native(parts:NativeSnapshotFailureParts)->Self{parts.into_failure()}
}
impl<I:semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource+?Sized> semio_framework_value_native_owned::NativeReceive<I> for NativeSnapshotFailure{
 type Receiver=semio_framework_value_native_owned::native_decoding::owned::NativeMappedReceiver<I,NativeSnapshotFailureParts,Self>;
 fn receiver()->Self::Receiver{semio_framework_value_native_owned::native_decoding::owned::NativeMappedReceiver::new()}
}
impl NativeSnapshotFailure{
 /// 🪞️ Borrows every raw original field without the derived fallbacks of the accessors.
 pub fn original_fields(&self)->NativeSnapshotFailureFields<'_>{NativeSnapshotFailureFields{cause:self.cause.as_ref(),allocation:self.allocation.as_ref(),encode_effect:self.encode_effect.as_ref(),receipt_refusal:self.receipt_refusal.as_ref(),context_refusal:self.context_refusal.as_ref(),work_refusal:self.work_refusal.as_ref(),settlement_refusal:self.settlement_refusal.as_ref(),progress:&self.progress}}
 /// 🔎️ Reports whether the independent outer context slot is occupied.
 pub fn has_direct_context_refusal(&self)->bool{self.context_refusal.is_some()}
 /// 🧾️ Reports whether the independent outer receipt slot is occupied.
 pub fn has_direct_receipt_refusal(&self)->bool{self.receipt_refusal.is_some()}
 /// 🧾️ Keeps the original operation, work, settlement and receipt causes in one owner.
 pub fn from_effect_parts(cause:Option<NativeSnapshotCause>,work_refusal:Option<ValueError>,settlement_refusal:Option<ValueError>,receipt_refusal:Option<ValueError>,progress:RetainedCloneProgress)->Self{Self{cause,allocation:None,encode_effect:None,context_refusal:None,work_refusal,settlement_refusal,receipt_refusal,progress}}
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
 pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref().or_else(||self.encode_effect.as_ref().and_then(|effect|effect.physical.as_ref().or_else(||effect.physical_receipt.as_ref().and_then(|owner|owner.receipt_refusal.as_ref().or(owner.receipt_completion()))))).or_else(||match &self.cause{Some(NativeSnapshotCause::Receiving(error))=>error.receipt_refusal.as_ref().or(error.receipt_completion()),Some(NativeSnapshotCause::Retirement(error))=>error.receipt_refusal(),_=>None})}
 pub fn context_refusal(&self)->Option<&ValueError>{self.context_refusal.as_ref().or_else(||self.encode_effect.as_ref().and_then(|effect|effect.context.as_ref().or(effect.invariant.as_ref()).or(effect.physical_invariant.as_ref()).or(effect.capacity_invariant.as_ref()).or(effect.marker_failure.as_ref())))}
 /// 🛠️ Borrows the independently retained original work-post refusal.
 pub fn work_refusal(&self)->Option<&ValueError>{self.work_refusal.as_ref().or_else(||self.encode_effect.as_ref().and_then(|effect|effect.work.as_ref().or(effect.work_overrun.as_ref()))).or_else(||match &self.cause{Some(NativeSnapshotCause::Receiving(error))=>error.work_refusal.as_ref().or(error.work_overrun.as_ref()),Some(NativeSnapshotCause::Retirement(error))=>error.work_refusal(),_=>None})}
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
 fn primary_value(&self)->Option<&ValueError>{self.cause.as_ref().and_then(|cause|match cause{NativeSnapshotCause::Value(error)=>Some(error),NativeSnapshotCause::Receiving(error)=>receiving_primary(error),NativeSnapshotCause::Retirement(error)=>retirement_primary(error)}).or_else(||self.encode_effect.as_ref().and_then(encode_effect_primary)).or_else(||self.allocation.as_ref().and_then(NativeSnapshotAllocationFailure::primary)).or(self.receipt_refusal.as_ref()).or(self.context_refusal.as_ref()).or(self.work_refusal.as_ref()).or(self.settlement_refusal.as_ref())}
 /// 🧾️ Adopts the first failed receipt after its caller has cut further validation.
 pub fn with_receipt_refusal(mut self,error:ValueError)->Self{assert!(self.receipt_refusal.is_none(),"native failure already owns its first receipt refusal");self.receipt_refusal=Some(error);self}
 /// 👁️ Adopts the first original observer failure independently of the producer cause.
 pub fn with_context_refusal(mut self,error:ValueError)->Self{assert!(self.context_refusal.is_none(),"native failure already owns its first context refusal");self.context_refusal=Some(error);self}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn with_retained_progress(mut self,progress:RetainedCloneProgress)->Self{self.progress=progress;self}
 pub fn receiving_primary(&self)->Option<&OriginalReceivingPrimary>{match &self.cause{Some(NativeSnapshotCause::Receiving(error))=>Some(error),_=>None}}
 /// 🧾️ Keeps the original independent receipt refusal.
 pub fn receipt(error:ValueError)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:None,receipt_refusal:Some(error),context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}
 /// 👁️ Keeps the original independent observer refusal.
 pub fn context(error:ValueError)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:None,receipt_refusal:None,context_refusal:Some(error),work_refusal:None,settlement_refusal:None,progress}}
}
impl From<semio_framework_value::native_encoding::NativeEncodeOperationFailure> for NativeSnapshotFailure{fn from(error:semio_framework_value::native_encoding::NativeEncodeOperationFailure)->Self{match(error.primary,error.work_refusal){(Some(primary),Some(work))=>Self::primary_with_work(primary,work),(Some(primary),None)=>Self::from(primary),(None,Some(work))=>Self::work_post(work),(None,None)=>Self::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode failure has no cause"))}}}
impl From<ValueError> for NativeSnapshotFailure{fn from(error:ValueError)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:Some(NativeSnapshotCause::Value(error)),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}}
impl From<semio_framework_value::OriginalRetirementFailure> for NativeSnapshotFailure{fn from(error:semio_framework_value::OriginalRetirementFailure)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:Some(NativeSnapshotCause::Retirement(error)),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}}
fn receiving_primary(error:&OriginalReceivingPrimary)->Option<&ValueError>{error.primary.as_ref().or(error.quote_refusal.as_ref()).or(error.allocation_refusal.as_ref()).or(error.work_refusal.as_ref()).or(error.work_overrun.as_ref()).or_else(||error.allocation_failure.as_ref().and_then(|failure|failure.primary.as_ref().or(failure.admission.as_ref()).or(failure.forwarded_admission.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.scoped_admission.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.forwarded_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.scoped_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.forwarded_post.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.scoped_post.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.forwarded_cancel.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.scoped_cancel.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(failure.receipt.as_ref()).or(failure.extent.as_ref()))).or_else(||error.native_allocation.as_ref().and_then(|value|value.primary.as_ref().or(value.quote_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(value.extent_refusal.as_ref()).or(value.settlement_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)))).or(error.receipt_refusal.as_ref()).or(error.receipt_completion()).or_else(||error.retirement_failure.as_ref().and_then(retirement_primary))}
fn encode_effect_primary(error:&semio_framework_value::native_encoding::NativeEncodeEffectFailure)->Option<&ValueError>{error.primary.as_ref().or(error.context.as_ref()).or(error.invariant.as_ref()).or(error.physical_invariant.as_ref()).or(error.capacity_invariant.as_ref()).or(error.marker_failure.as_ref()).or(error.work.as_ref()).or(error.work_overrun.as_ref()).or(error.local.as_ref()).or(error.physical.as_ref()).or_else(||error.physical_receipt.as_ref().and_then(receiving_primary)).or_else(||error.allocation.as_ref().and_then(|allocation|allocation.primary.as_ref().or(allocation.forwarded_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(allocation.scoped_quote.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(allocation.extent_refusal.as_ref()).or(allocation.forwarded_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(allocation.scoped_refusal.as_ref().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned))))}
fn retirement_primary(mut error:&semio_framework_value::OriginalRetirementFailure)->Option<&ValueError>{let mut fallback=None;loop{if let Some(primary)=error.primary(){return Some(primary)}fallback=error.receipt_refusal().or(error.work_refusal()).or(error.allocation_refusal().and_then(semio_framework_value_control_contract::NativeAllocationRefusal::owned)).or(fallback);match error.child(){Some(child)=>error=child,None=>return fallback}}}
impl From<OriginalReceivingPrimary> for NativeSnapshotFailure{fn from(error:OriginalReceivingPrimary)->Self{let progress=error.retained_progress();Self{encode_effect:None,allocation:None,cause:Some(NativeSnapshotCause::Receiving(error)),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress}}}
impl std::fmt::Display for NativeSnapshotFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message())}}
impl std::error::Error for NativeSnapshotFailure{}
fn receiving_eq(a:&OriginalReceivingPrimary,b:&OriginalReceivingPrimary)->bool{if a.receipt_completion_pending()||b.receipt_completion_pending(){std::ptr::eq(a,b)}else{a.retirement_failure==b.retirement_failure&&a.work_refusal==b.work_refusal&&a.work_overrun==b.work_overrun&&a.primary==b.primary&&a.quote_refusal==b.quote_refusal&&a.allocation_refusal==b.allocation_refusal&&a.allocation_failure==b.allocation_failure&&(match(&a.native_allocation,&b.native_allocation){(None,None)=>true,(Some(a),Some(b))=>a.request==b.request&&a.admitted==b.admitted&&a.quote_refusal==b.quote_refusal&&a.primary==b.primary&&a.extent_refusal==b.extent_refusal&&a.settlement_refusal==b.settlement_refusal&&a.actual==b.actual,_=>false})&&a.receipt_refusal==b.receipt_refusal&&a.receipt_completion()==b.receipt_completion()&&a.progress==b.progress}}
fn encode_effect_eq(a:&semio_framework_value::native_encoding::NativeEncodeEffectFailure,b:&semio_framework_value::native_encoding::NativeEncodeEffectFailure)->bool{a.primary==b.primary&&a.context==b.context&&a.invariant==b.invariant&&a.physical_invariant==b.physical_invariant&&a.capacity_invariant==b.capacity_invariant&&a.marker_failure==b.marker_failure&&a.work==b.work&&a.work_overrun==b.work_overrun&&a.local==b.local&&a.physical==b.physical&&match(&a.physical_receipt,&b.physical_receipt){(None,None)=>true,(Some(a),Some(b))=>receiving_eq(a,b),_=>false}&&a.allocation==b.allocation}
impl PartialEq for NativeSnapshotCause{fn eq(&self,other:&Self)->bool{match(self,other){(Self::Value(a),Self::Value(b))=>a==b,(Self::Retirement(a),Self::Retirement(b))=>a==b,(Self::Receiving(a),Self::Receiving(b))=>receiving_eq(a,b),_=>false}}}
impl PartialEq for NativeSnapshotFailure{fn eq(&self,other:&Self)->bool{self.cause==other.cause&&self.allocation==other.allocation&&match(&self.encode_effect,&other.encode_effect){(None,None)=>true,(Some(a),Some(b))=>encode_effect_eq(a,b),_=>false}&&self.receipt_refusal==other.receipt_refusal&&self.context_refusal==other.context_refusal&&self.work_refusal==other.work_refusal&&self.settlement_refusal==other.settlement_refusal&&self.progress==other.progress}}





/// 🧮️ Keeps semantic producer actions independent of physical receipt items.
#[derive(Default)]
pub struct NativeSnapshotWork(u128);
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

#[path="🔁️encode/🦀️.rs"]
mod encode;
pub use encode::{optional_refusal,retirement_value};

#[cfg(test)]
mod cause_equality_tests{
 use super::*;
 #[test]fn receiving_work_refusal_is_an_independent_cause(){
  fn cause(work_refusal:Option<ValueError>)->NativeSnapshotCause{NativeSnapshotCause::Receiving(OriginalReceivingPrimary::from_fields(None,None,None,work_refusal,None,None,None,None,None,Default::default()))}
  let absent=cause(None);let first=cause(Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original work")));let same=cause(Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original work")));let different=cause(Some(ValueError::literal(ValueRefusalKind::Canceled,"original work")));
  assert_ne!(absent,first);assert_eq!(first,same);assert_ne!(first,different);
 }
 #[test]fn receiving_work_overrun_is_an_independent_cause(){fn cause(work_overrun:Option<ValueError>)->NativeSnapshotCause{NativeSnapshotCause::Receiving(OriginalReceivingPrimary::from_fields(None,None,None,None,work_overrun,None,None,None,None,Default::default()))}let absent=cause(None);let first=cause(Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original overrun")));let same=cause(Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original overrun")));let different=cause(Some(ValueError::literal(ValueRefusalKind::Canceled,"original overrun")));assert_ne!(absent,first);assert_eq!(first,same);assert_ne!(first,different);let NativeSnapshotCause::Receiving(owner)=first else{unreachable!()};let failure=NativeSnapshotFailure::from(owner);assert_eq!(failure.kind(),ValueRefusalKind::WorkLimit);assert_eq!(failure.message(),"original overrun");assert_eq!(failure.work_refusal().unwrap().message,"original overrun");}

}
