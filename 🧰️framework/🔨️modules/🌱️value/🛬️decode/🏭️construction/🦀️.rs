//! 🏭️ Retained native construction requires original physical grants and retains born output before receipt acceptance.
use super::NativeDecodeControl;
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},shared_issuance::{SealedShared,SharedIssuer}};
#[path="🧾️result/🦀️.rs"]
mod result;
#[path="📦️effect/🦀️.rs"]mod effect;
pub use effect::{NativeReceivingEffect,NativeReceivingEffectError,NativeReceivingEffectFailure,NativeReceivingEffectPhase};
pub use result::{NativeConstructionStepResult,NativeConstructionStepOutcome,NativeConstructionStepFailure,OriginalReceivingPrimary};
/// 📦️ The original caller retains empty shared storage and every pending callback until funded publication or release.
pub struct NativeSharedAdmission<T:Send+Sync+'static>{effect:NativeReceivingEffect,sealed:std::mem::ManuallyDrop<Option<crate::shared_issuance::SealedSharedBirth<T>>>,issuer:Option<SharedIssuer<T>>,arc:std::mem::ManuallyDrop<Option<std::sync::Arc<std::mem::MaybeUninit<T>>>>}
impl<T:Send+Sync+'static> Default for NativeSharedAdmission<T>{fn default()->Self{Self{effect:Default::default(),sealed:std::mem::ManuallyDrop::new(None),issuer:None,arc:std::mem::ManuallyDrop::new(None)}}}
impl<T:Send+Sync+'static> NativeSharedAdmission<T>{
 pub fn receiving_effect(&mut self)->&mut NativeReceivingEffect{&mut self.effect}
 /// 🔁️ Completes a callback-resolved turn through its unchanged original work and construction loans.
 pub fn resume(&mut self,control:&mut NativeDecodeControl<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{control.finish_receiving(&mut self.effect)}
 pub fn phase(&self)->NativeReceivingEffectPhase{self.effect.phase()}
 /// ▶️ Allows a fresh turn or the same grant after its interrupted work quote was resolved.
 pub fn can_advance(&self)->bool{matches!(self.effect.phase(),NativeReceivingEffectPhase::Empty|NativeReceivingEffectPhase::Granted)}
 pub fn has_birth(&self)->bool{self.sealed.is_some()||self.arc.is_some()}
 pub fn shared_demand(&self)->RetirementDemand{if self.sealed.is_some(){Self::publication_demand()}else{NativeOwnedDecodeControl::shared_birth_demand::<T>()}}
 pub fn arc_demand(&self)->RetirementDemand{if self.arc.is_some(){Self::publication_demand()}else{NativeOwnedDecodeControl::arc_birth_demand::<T>()}}
 fn publication_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
 pub fn terminal_is_empty(&self)->bool{self.effect.phase()==NativeReceivingEffectPhase::Empty&&!self.has_birth()}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{if self.effect.phase()!=NativeReceivingEffectPhase::Empty{return Err(ValueError::invariant("original shared allocation callback requires resolution"))}if let Some(birth)=self.sealed.as_ref(){return Ok(birth.next_close_demand())}Ok(RetirementDemand{release_bytes:if self.arc.is_some(){crate::retirement::arc_custody::shared_allocation_bytes::<T>()}else{0},depth:usize::from(self.arc.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let demand=match self.close_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0}}if let Some(birth)=self.sealed.as_mut(){let turn=birth.close_step(grant);if birth.terminal_is_empty(){self.sealed.take();self.issuer.take();}return turn}if self.arc.is_some(){self.arc.take();return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})),work:1}}crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0}}
}
impl<T:Send+Sync+'static> Drop for NativeSharedAdmission<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original shared admission requires callback settlement and funded birth closure")}}
/// 🫴️ The owning native operation supplies grants and accepts actual receipts through this mandatory borrowed port.
pub trait NativeDecodeConstructionRecipient:Send{
 /// 📊️ Reports every physically submitted receipt, including refused publications.
 fn submitted(&self)->RetainedCloneProgress;
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>;
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
}
/// 🪪️ Retains one original construction source authority across its short turn loans.
pub struct NativeConstructionRecipientIssuer{identity:u64,next_action:u64,outstanding:usize}
impl NativeConstructionRecipientIssuer{
 /// 📏️ Quotes only the initialized original issuer fields.
 pub const fn birth_bytes()->usize{std::mem::size_of::<u64>()}
 /// 🏷️ Creates one non-reusable caller-owned source authority.
 pub fn new()->Self{static NEXT:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);Self{identity:NEXT.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|value|value.checked_add(1)).expect("native construction issuer identity exhausted"),next_action:1,outstanding:0}}
 /// 🔗️ Delegates this original authority to the actual turn recipient.
 pub fn loan<'a>(&'a mut self,recipient:&'a mut dyn NativeDecodeConstructionRecipient)->NativeConstructionRecipientLoan<'a>{let identity=self.identity;NativeConstructionRecipientLoan{identity,recipient,issuer:self}}
 /// ⏳️ Reports the exact count of original actions awaiting handoff.
 pub fn outstanding(&self)->usize{self.outstanding}
}
impl Default for NativeConstructionRecipientIssuer{fn default()->Self{Self::new()}}
impl Drop for NativeConstructionRecipientIssuer{fn drop(&mut self){assert!(std::thread::panicking()||self.outstanding==0,"native construction issuer retains original actions")}}
/// 🎟️ Keeps an exclusive original issuer loan and the actual turn recipient together.
pub struct NativeConstructionRecipientLoan<'a>{identity:u64,recipient:&'a mut dyn NativeDecodeConstructionRecipient,issuer:&'a mut NativeConstructionRecipientIssuer}
impl NativeConstructionRecipientLoan<'_>{
 pub(super) fn reborrow(&mut self)->NativeConstructionRecipientLoan<'_>{NativeConstructionRecipientLoan{identity:self.identity,recipient:&mut *self.recipient,issuer:&mut *self.issuer}}
 pub(super) fn identity(&self)->u64{self.identity}
 pub(super) fn begin_action(&mut self)->Result<u64,ValueError>{let next=self.issuer.next_action.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native construction action identity exhausted"))?;let count=self.issuer.outstanding.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native construction action count exhausted"))?;let action=self.issuer.next_action;self.issuer.next_action=next;self.issuer.outstanding=count;Ok(action)}
 pub(super) fn has_action(&self)->bool{self.issuer.outstanding!=0}
 pub(super) fn finish_action(&mut self){self.issuer.outstanding-=1}

}
impl<'a> std::ops::Deref for NativeConstructionRecipientLoan<'a>{type Target=dyn NativeDecodeConstructionRecipient+'a;fn deref(&self)->&Self::Target{self.recipient}}
impl<'a> std::ops::DerefMut for NativeConstructionRecipientLoan<'a>{fn deref_mut(&mut self)->&mut Self::Target{self.recipient}}
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,owned:&'owner mut usize,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,work_actual:&'owner mut u64,work_reserved:&'owner mut u64,work_reserved_issuer:&'owner mut Option<u64>,work_reserved_action:&'owner mut Option<u64>,work_quote_pending:&'owner mut bool}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.owned=self.control.owned_bytes;*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.work_actual=self.control.work.actual;*self.work_reserved=self.control.work.reserved;*self.work_reserved_issuer=self.control.work.reserved_issuer;*self.work_reserved_action=self.control.work.reserved_action;*self.work_quote_pending=self.control.work.quote_pending;self.control.work.reserved=0;self.control.work.reserved_issuer=None;self.control.work.reserved_action=None;self.control.work.quote_pending=false;}}
pub(super) fn run<T,E:From<ValueError>>(original:&mut NativeDecodeControl<'_>,recipient:&mut NativeConstructionRecipientLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 if original.construction.is_some(){return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction cannot substitute its original receiving authority")));}
 let mut construction=recipient.reborrow();
 let mut forwarded=match &mut original.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(port)=>Some(port.reborrow())};
 let mut work_loan=original.work.recipient.as_mut().map(|loan|loan.reborrow());
 let control=NativeDecodeControl{work:super::work::Work{actual:original.work.actual,baseline:original.work.baseline,reserved:original.work.reserved,reserved_issuer:original.work.reserved_issuer,reserved_action:original.work.reserved_action,quote_pending:original.work.quote_pending,recipient:work_loan.as_mut()},ledger:original.ledger,scope_maximum:original.scope_maximum,construction:Some(&mut construction),receiving:original.receiving,maximum_bytes:original.maximum_bytes,owned_bytes:original.owned_bytes,completed:original.completed,total:original.total,started:original.started,stage:original.stage,depth:original.depth,callback:&mut *original.callback,retirement:original.retirement.as_deref_mut(),allocation:match forwarded.as_mut(){None=>super::allocation::Binding::Local,Some(port)=>super::allocation::Binding::Forwarded(port)}};
 let mut scope=Scope{control,maximum:&mut original.maximum_bytes,scope_maximum:&mut original.scope_maximum,owned:&mut original.owned_bytes,completed:&mut original.completed,total:&mut original.total,started:&mut original.started,stage:&mut original.stage,depth:&mut original.depth,work_actual:&mut original.work.actual,work_reserved:&mut original.work.reserved,work_reserved_issuer:&mut original.work.reserved_issuer,work_reserved_action:&mut original.work.reserved_action,work_quote_pending:&mut original.work.quote_pending};operation(&mut scope.control)
}
impl NativeDecodeControl<'_>{
 /// 🔭️ Reads actual original construction submissions without issuing another grant.
 pub fn construction_submitted(&self)->Result<RetainedCloneProgress,ValueError>{self.construction.as_deref().map(|recipient|recipient.submitted()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))}
 /// 🎟️ Quotes the real original recipient without introducing another payer.
 pub fn construction_grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?.grant(demand)}
 /// 🧾️ Publishes already-performed fresh effects under the caller’s previously issued grant.
 pub fn submit_constructed_progress(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?.accept(grant,progress)}
 /// 🛬️ One original physical recipient funds and records a bounded typed construction transition.
 pub fn receive_construction_step(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->crate::RetainedWorkOutcome)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneStep,NativeConstructionStepFailure<ValueError>>{
  self.receive_construction_step_typed(demand,|control,grant|{let turn=operation(control,grant);let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};NativeConstructionStepResult{result:turn.result,progress,work:turn.work}}).into_step_outcome()
 }
 /// 🧬️ Typed output or primary failure remains resident even when its physical recipient refuses the same effects.
 pub fn receive_construction_step_typed<T,E:From<ValueError>>(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->NativeConstructionStepResult<T,E>)->NativeConstructionStepOutcome<T,E>{
  let admission=self.checkpoint().and_then(|()|self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction step requires its original recipient"))?.grant(demand));
  let grant=match admission{Ok(grant)=>grant,Err(error)=>return NativeConstructionStepOutcome{result:Some(Err(error.into())),receipt_refusal:None,progress:Default::default(),work:0}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return NativeConstructionStepOutcome{result:None,receipt_refusal:None,progress:Default::default(),work:0}}
  let NativeConstructionStepResult{result,progress,work}=operation(self,grant);
  let receipt_refusal=self.construction.as_deref_mut().expect("original scoped construction recipient").accept(grant,progress).err().map(|error|error.with_retained_progress(progress));
  NativeConstructionStepOutcome{result:Some(result),receipt_refusal,progress,work}
 }
 /// 🏗️ Lends one original construction recipient through this receiving hop without changing its cumulative decoder receipt.
 pub fn with_construction_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut NativeConstructionRecipientLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{run(self,recipient,operation)}
 /// 🧷️ Admits sealed field ownership only under this hop's explicitly supplied original construction recipient.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>,owner:&mut NativeSharedAdmission<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if input.is_none()||output.is_some()||owner.arc.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("native sealed admission requires its original slots").into()),work:0}}
  let demand=owner.shared_demand();let born=owner.sealed.is_some();let frame=&mut owner.effect;let slot=&mut *owner.sealed;let saved=&mut owner.issuer;
   self.receive_allocation(demand,frame,!born,if born{1}else{SealedShared::<T>::birth_work_demand()},|grant,actual|{if !born{let turn=SealedShared::<T>::prepare_birth(grant,actual.expect("allocated birth marker"));let result=turn.result.map(|(birth,progress)|{*slot=Some(birth);*saved=Some(issuer);crate::RetainedCloneStep::Progress(progress)});crate::RetainedWorkOutcome{result,work:turn.work}}else{let mut work=0;let result=(||{let birth=slot.as_mut().ok_or_else(||ValueError::invariant("sealed birth absent"))?;let publication=birth.publication(grant)?.ok_or_else(||ValueError::invariant("sealed publication not admitted"))?;let authority=saved.take().ok_or_else(||ValueError::invariant("original sealed issuer absent"))?;let turn=publication.commit(input.take().ok_or_else(||ValueError::invariant("original sealed input absent"))?,authority);work=turn.work;let (value,mut progress)=turn.result?;*output=Some(value);progress.copied_bytes=demand.copy_bytes;slot.take();Ok(crate::RetainedCloneStep::Complete(progress))})();crate::RetainedWorkOutcome{result,work}}})
 }
 /// 🖼️ Original receiving allocates an ordinary immutable Arc without replacing its recipient.
 pub fn receive_arc<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<std::sync::Arc<T>>,owner:&mut NativeSharedAdmission<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if input.is_none()||output.is_some()||owner.sealed.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("native Arc admission requires its original slots").into()),work:0}}
  let demand=owner.arc_demand();let born=owner.arc.is_some();let frame=&mut owner.effect;let slot=&mut *owner.arc;
  self.receive_allocation(demand,frame,!born,1,|_grant,actual|{if !born{let result=match crate::retirement::arc_custody::try_uninitialized_arc::<T>(){Ok(value)=>{*actual.expect("allocated birth marker")=Some(demand.capacity_bytes);*slot=Some(value);Ok(crate::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))},Err(error)=>{*actual.expect("allocated birth marker")=Some(0);Err(error)}};crate::RetainedWorkOutcome{result,work:1}}else{let mut work=0;let result=(||{let birth=slot.as_mut().ok_or_else(||ValueError::invariant("Arc birth absent"))?;let target=std::sync::Arc::get_mut(birth).ok_or_else(||ValueError::invariant("original Arc birth is not unique"))?;target.write(input.take().ok_or_else(||ValueError::invariant("original Arc input absent"))?);work=1;let birth=slot.take().ok_or_else(||ValueError::invariant("Arc birth absent"))?;*output=Some(unsafe{birth.assume_init()});Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}})
 }
 fn receive_allocation(&mut self,demand:RetirementDemand,frame:&mut NativeReceivingEffect,allocating:bool,work_demand:u64,operation:impl FnOnce(RetainedCloneGrant,Option<&mut Option<usize>>)->crate::RetainedWorkOutcome)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
   if frame.phase()==NativeReceivingEffectPhase::Empty{if let Err(error)=frame.quote(self,demand){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}}else if !frame.matches_demand(demand){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original shared demand changed during recovery").into()),work:0}}
   if frame.phase()==NativeReceivingEffectPhase::Granted{let admission=if self.has_pending_work(){if self.prepared_work_admits(work_demand){Ok(true)}else{return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original work quote requires resolution").into()),work:0}}}else{self.prepare_work(work_demand)};match admission{Ok(true)=>{let result=if allocating{frame.effect(self,|actual,grant|operation(grant,Some(actual)))}else{frame.effect_unallocated(self,|grant|operation(grant,None))};if let Err(error)=result{return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}},Ok(false)=>{if let Err(error)=frame.cancel_unstarted(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}},Err(refusal)=>{if let Err((error,refusal))=frame.record_work_refusal(self,refusal){let mut failure:OriginalReceivingPrimary=error.into();failure.work_refusal=Some(refusal);return crate::RetainedWorkOutcome{result:Err(failure),work:0}}if let Err(error)=frame.cancel_unstarted(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}}}}
   self.finish_receiving(frame)
  }
  fn finish_receiving(&mut self,frame:&mut NativeReceivingEffect)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
   if matches!(frame.phase(),NativeReceivingEffectPhase::EffectStarted|NativeReceivingEffectPhase::EffectFinished){
    if let Some(work)=frame.child_work(){
     if !frame.work_is_settled(){
      let settled=if frame.work_commit_is_pending(){frame.retry_original_work(self)}else{frame.settle_original_work(self,work)};
      if let Err(error)=settled{return crate::RetainedWorkOutcome{result:Err(error.into()),work:if self.has_pending_work(){0}else{frame.claim_child_work()}}}
     }
    }else if frame.phase()==NativeReceivingEffectPhase::EffectFinished&&self.has_pending_work(){self.cancel_prepared_work()}
   }
   if frame.phase()==NativeReceivingEffectPhase::EffectStarted{return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original allocation callback requires resolution").into()),work:frame.claim_child_work()}}
   if frame.phase()==NativeReceivingEffectPhase::EffectFinished{if let Err(error)=frame.receipt(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:frame.claim_child_work()}}}
  match frame.take(self){Ok(turn)=>crate::RetainedWorkOutcome{result:turn.result.map(|step|step.progress()).map_err(|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child;receiving.work_refusal=failure.work_refusal;receiving}),work:turn.work},Err(error)=>crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}
 }

}
/// 🎛️ Construction borrows the original decoder and recipient without deriving retained authority from cumulative totals.
pub struct NativeOwnedDecodeControl<'operation,'decode,'recipient>{original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut NativeConstructionRecipientLoan<'recipient>}
impl<'operation,'decode,'recipient> NativeOwnedDecodeControl<'operation,'decode,'recipient>{
 /// 🔗️ The required recipient belongs to the same owning operation that lends the original decode control.
 pub fn new(original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut NativeConstructionRecipientLoan<'recipient>)->Self{Self{original,recipient}}
 /// 📏️ Original payload and resulting source header movement accompany the exact sealed backing birth.
 pub fn shared_birth_demand<T:Send+Sync+'static>()->RetirementDemand{RetirementDemand{capacity_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()}}
 /// 📐️ An ordinary shared backing quotes the actual standard Arc layout.
 pub fn arc_birth_demand<T:Send+Sync+'static>()->RetirementDemand{RetirementDemand{capacity_bytes:crate::retirement::arc_custody::shared_allocation_bytes::<T>(),depth:1,..Default::default()}}
 /// 📦️ The exact original payload or born owner survives every native primary and receipt refusal.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>,owner:&mut NativeSharedAdmission<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  match self.original.with_construction_recipient(self.recipient,|control|Ok::<_,ValueError>(control.receive_shared(input,output,issuer,owner))){Ok(turn)=>turn,Err(error)=>crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}
 }

}

#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod shared_work_tests;

#[path="📦️backing/🦀️.rs"]mod backing;
pub use backing::NativeBackingTurn;
