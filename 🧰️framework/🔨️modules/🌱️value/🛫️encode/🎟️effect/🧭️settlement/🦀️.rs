//! 🧭️ Control lends the actual original authorities to the shared effect settlement owner.
use super::super::NativeEncodeControl;
use crate::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};

static EFFECT_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
/// 🚦️ Records whether the original callback has started or completed.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeEncodeEffectPostPhase{Waiting,Started,Completed}
/// 🎟️ Proves that both original post callbacks completed for one physical effect.
pub struct NativeEncodeEffectReceipt{identity:u64,progress:RetainedCloneProgress}
impl NativeEncodeEffectReceipt{
 /// 📊️ Reports the exact effect accepted by both original authorities.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🪪️ Names the single original action that produced this receipt.
 pub fn identity(&self)->u64{self.identity}
}
/// ↩️ Proves that an original reservation was canceled before an effect.
pub struct NativeEncodeEffectCancellation{identity:u64}
impl NativeEncodeEffectCancellation{
 /// 🪪️ Names the canceled original action.
 pub fn identity(&self)->u64{self.identity}
}
impl crate::retirement::RetireOwned for NativeEncodeEffectReceipt{
 type Retirement=crate::retirement::Leaf<(u64,RetainedCloneProgress)>;
 fn retirement(self)->Self::Retirement{crate::retirement::leaf((self.identity,self.progress))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::Retirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl crate::retirement::RetireOwned for NativeEncodeEffectCancellation{
 type Retirement=crate::retirement::Leaf<u64>;
 fn retirement(self)->Self::Retirement{crate::retirement::leaf(self.identity)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::Retirement>())}
 fn controlled_retirement_supported()->bool{true}
}
/// 🧳️ Retains the exact quote and independent callback phases across unwind.
pub struct NativeEncodeEffectSettlement{identity:u64,construction:u64,work_issuer:u64,work_quote:u64,actual_work:Option<u64>,grant:Option<RetainedCloneGrant>,demand:RetirementDemand,quote_phase:NativeEncodeEffectPostPhase,admission_phase:NativeEncodeEffectPostPhase,admitted:bool,progress:Option<RetainedCloneProgress>,work_phase:NativeEncodeEffectPostPhase,physical_phase:NativeEncodeEffectPostPhase,work_failure:Option<ValueError>,physical_failure:Option<ValueError>,invariant_failure:Option<ValueError>,released:bool}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn release_unstarted(slot:&mut Option<NativeEncodeEffectSettlement>){if let Some(state)=slot.as_mut(){state.released=true}slot.take();}
impl Drop for NativeEncodeEffectSettlement{fn drop(&mut self){assert!(std::thread::panicking()||self.released||(self.work_phase==NativeEncodeEffectPostPhase::Completed&&self.physical_phase==NativeEncodeEffectPostPhase::Completed),"original effect settlement remains unresolved")}}
impl NativeEncodeEffectSettlement{
 /// 🪪️ Names this original action for its caller-owned frame.
 pub fn identity(&self)->u64{self.identity}
 /// 🚦️ Reports whether the original quote and work reservation are ready.
 pub fn effect_ready(&self)->bool{self.grant.is_some()&&self.admitted&&self.admission_phase==NativeEncodeEffectPostPhase::Completed&&self.progress.is_none()}
 /// 🧾️ Records the actual caller-owned effect before any settlement callback.
 pub fn record_effect(&mut self,progress:RetainedCloneProgress,actual_work:u64)->Result<(),ValueError>{if self.progress.is_some()||self.grant.is_none()||!self.admitted||self.admission_phase!=NativeEncodeEffectPostPhase::Completed||self.work_phase!=NativeEncodeEffectPostPhase::Waiting{return Err(invalid("original effect cannot replace its retained progress"))}self.progress=Some(progress);self.actual_work=Some(actual_work);if actual_work>self.work_quote||!progress.fits(self.grant.unwrap()){self.invariant_failure=Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original effect exceeds its retained physical or work quote"));return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original effect exceeds its retained physical or work quote"))}Ok(())}
 /// ⛽️ Reports the work admitted before this original effect.
 pub fn work_quote(&self)->u64{self.work_quote}
 /// 🧮️ Reports actual work retained before either post callback.
 pub fn actual_work(&self)->Option<u64>{self.actual_work}
 /// 🎫️ Borrows the actual original physical quote, including a refused quote.
 pub fn quoted_grant(&self)->Option<RetainedCloneGrant>{self.grant}
 /// 📊️ Borrows the retained actual physical receipt.
 pub fn progress(&self)->Option<RetainedCloneProgress>{self.progress}
 /// 🔭️ Reports the original physical quote boundary.
 pub fn quote_phase(&self)->NativeEncodeEffectPostPhase{self.quote_phase}
 /// 🔭️ Reports the original work admission boundary.
 pub fn admission_phase(&self)->NativeEncodeEffectPostPhase{self.admission_phase}
 /// 🧾️ Borrows independent original post refusals without moving them.
 pub fn failures(&self)->(Option<&ValueError>,Option<&ValueError>,Option<&ValueError>){(self.work_failure.as_ref(),self.physical_failure.as_ref(),self.invariant_failure.as_ref())}
 /// 🔭️ Reports the original work callback boundary.
 pub fn work_phase(&self)->NativeEncodeEffectPostPhase{self.work_phase}
 /// 🔭️ Reports the original physical callback boundary.
 pub fn physical_phase(&self)->NativeEncodeEffectPostPhase{self.physical_phase}
 /// 🫴️ Moves the proof only after both original callbacks accepted the effect.
 pub fn take_receipt(slot:&mut Option<Self>)->Result<NativeEncodeEffectReceipt,ValueError>{let state=slot.as_ref().ok_or_else(||invalid("original effect settlement is absent"))?;if state.work_phase!=NativeEncodeEffectPostPhase::Completed||state.physical_phase!=NativeEncodeEffectPostPhase::Completed||state.work_failure.is_some()||state.physical_failure.is_some()||state.invariant_failure.is_some(){return Err(invalid("original effect settlement is not accepted"))}let progress=state.progress.ok_or_else(||invalid("original effect has no progress"))?;let identity=state.identity;slot.take();Ok(NativeEncodeEffectReceipt{identity,progress})}
 /// 🪪️ Checks both original callback issuers before touching caller state.
 pub fn same_original(&self,native:&NativeEncodeControl<'_>)->bool{native.work_recipient_identity()==Some(self.work_issuer)&&native.construction_recipient_identity()==Some(self.construction)}
}
impl crate::retirement::RetireOwned for NativeEncodeEffectSettlement{
 type Retirement=super::super::NativeEncodeRetirement<Self,<(Option<ValueError>,(Option<ValueError>,(Option<ValueError>,()))) as crate::retirement::RetireOwned>::Retirement>;
 fn retirement_ready(&self)->bool{self.work_phase==NativeEncodeEffectPostPhase::Completed&&self.physical_phase==NativeEncodeEffectPostPhase::Completed}
 fn retirement(self)->Self::Retirement{if !crate::retirement::RetireOwned::retirement_ready(&self){return super::super::NativeEncodeRetirement::Pending(Some(self))}let mut state=self;super::super::NativeEncodeRetirement::Ready(semio_framework_value_retirement::artifact_retirement_sequence!(state.work_failure.take(),state.physical_failure.take(),state.invariant_failure.take()))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::Retirement>())}
 fn controlled_retirement_supported()->bool{true}
}
/// 🪢️ Reborrows original work and physical callbacks beside a retained frame.
pub struct NativeEncodeEffectRecovery<'scope,'control>{action:&'scope mut super::super::action::Action<'control>,construction:&'scope mut Option<&'control mut dyn super::super::NativeEncodeConstructionRecipient>,construction_identity:Option<u64>}
impl NativeEncodeEffectRecovery<'_,'_>{
 fn matches(&self,state:&NativeEncodeEffectSettlement)->bool{self.action.identity==Some(state.work_issuer)&&self.construction_identity==Some(state.construction)}
 /// 🪢️ Reconciles the original quote without consuming an unmatched response.
 pub fn resolve_quote(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>,result:&mut Option<Result<RetainedCloneGrant,ValueError>>)->Result<(),ValueError>{let state=slot.as_mut().ok_or_else(||invalid("original effect settlement is absent"))?;if !self.matches(state)||state.quote_phase!=NativeEncodeEffectPostPhase::Started{return Err(invalid("original quote authority or phase changed"))}let result=result.take().ok_or_else(||invalid("original quote response is absent"))?;state.quote_phase=NativeEncodeEffectPostPhase::Completed;let grant=match result{Ok(grant)=>grant,Err(error)=>{release_unstarted(slot);return Err(error)}};state.grant=Some(grant);if grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,state.demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original effect quote is insufficient"))}Ok(())}
 /// 🎟️ Resumes work admission after reconciling the original quote.
 pub fn resume_admission(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<(),ValueError>{let state=slot.as_mut().ok_or_else(||invalid("original effect settlement is absent"))?;if !self.matches(state)||state.grant.is_none_or(|grant|grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,state.demand))||state.admission_phase!=NativeEncodeEffectPostPhase::Waiting{return Err(invalid("original work admission authority or phase changed"))}state.admission_phase=NativeEncodeEffectPostPhase::Started;let result=super::super::action::prepare(self.action,state.work_quote);slot.as_mut().unwrap().admission_phase=NativeEncodeEffectPostPhase::Completed;match result{Ok(true)=>{slot.as_mut().unwrap().admitted=true;Ok(())},Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original effect work is refused")),Err(error)=>Err(error)}}
 /// 🪢️ Reconciles the original admission without replaying its callback.
 pub fn resolve_admission(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>,result:&mut Option<Result<bool,ValueError>>)->Result<(),ValueError>{let state=slot.as_mut().ok_or_else(||invalid("original effect settlement is absent"))?;if !self.matches(state)||state.admission_phase!=NativeEncodeEffectPostPhase::Started{return Err(invalid("original work admission authority or phase changed"))}if matches!(result.as_ref(),Some(Ok(true)))&&self.action.reserved!=0{return Err(invalid("original work reservation is occupied"))}let result=result.take().ok_or_else(||invalid("original admission response is absent"))?;state.admission_phase=NativeEncodeEffectPostPhase::Completed;match result{Ok(true)=>{self.action.reserved=state.work_quote;state.admitted=true;Ok(())},Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original effect work is refused")),Err(error)=>Err(error)}}
 /// ↩️ Cancels a matching reservation before the effect.
 pub fn cancel(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<NativeEncodeEffectCancellation,ValueError>{let state=slot.as_ref().ok_or_else(||invalid("original effect settlement is absent"))?;if !self.matches(state)||state.progress.is_some()||!state.admitted||state.admission_phase!=NativeEncodeEffectPostPhase::Completed||state.work_phase!=NativeEncodeEffectPostPhase::Waiting||state.physical_phase!=NativeEncodeEffectPostPhase::Waiting{return Err(invalid("original effect cannot cancel its retained state"))}let identity=state.identity;super::super::action::cancel(self.action)?;release_unstarted(slot);Ok(NativeEncodeEffectCancellation{identity})}
 /// ↩️ Releases a refused quote or admission before any action was reserved.
 pub fn release_refused(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<NativeEncodeEffectCancellation,ValueError>{let state=slot.as_ref().ok_or_else(||invalid("original effect settlement is absent"))?;if !self.matches(state)||state.quote_phase!=NativeEncodeEffectPostPhase::Completed||state.admitted||state.progress.is_some()||state.admission_phase==NativeEncodeEffectPostPhase::Started{return Err(invalid("original refused effect is not settled"))}let identity=state.identity;release_unstarted(slot);Ok(NativeEncodeEffectCancellation{identity})}
 /// 🧾️ Posts each actual callback once using the exact original source loans.
 pub fn settle(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<(),ValueError>{let state=slot.as_mut().ok_or_else(||invalid("original effect settlement is absent"))?;if !self.matches(state){return Err(invalid("original effect issuer changed"))}let progress=state.progress.ok_or_else(||invalid("original effect progress is absent"))?;let actual_work=state.actual_work.ok_or_else(||invalid("original effect actual work is absent"))?;let grant=state.grant.ok_or_else(||invalid("original effect quote is absent"))?;if state.work_phase==NativeEncodeEffectPostPhase::Started||state.physical_phase==NativeEncodeEffectPostPhase::Started{return Err(invalid("original effect callback requires reconciliation"))}if state.work_phase==NativeEncodeEffectPostPhase::Waiting{state.work_phase=NativeEncodeEffectPostPhase::Started;state.work_failure=super::super::action::commit(self.action,actual_work).err();state.work_phase=NativeEncodeEffectPostPhase::Completed}if state.physical_phase==NativeEncodeEffectPostPhase::Waiting{state.physical_phase=NativeEncodeEffectPostPhase::Started;state.physical_failure=match self.construction.as_deref_mut(){Some(port)=>port.accept(grant,progress).err(),None=>Some(invalid("original construction source is absent"))};state.physical_phase=NativeEncodeEffectPostPhase::Completed}Ok(())}
 /// 🪢️ Reconciles a started callback without consuming an unmatched original response.
 pub fn resolve_post(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>,work:bool,result:&mut Option<Result<(),ValueError>>)->Result<(),ValueError>{let state=slot.as_mut().ok_or_else(||invalid("original effect settlement is absent"))?;if !self.matches(state){return Err(invalid("original effect issuer changed"))}let(phase,failure)=if work{(&mut state.work_phase,&mut state.work_failure)}else{(&mut state.physical_phase,&mut state.physical_failure)};if *phase!=NativeEncodeEffectPostPhase::Started{return Err(invalid("original effect callback did not start"))}*failure=result.take().ok_or_else(||invalid("original post response is absent"))?.err();*phase=NativeEncodeEffectPostPhase::Completed;Ok(())}
}
impl NativeEncodeControl<'_>{
 /// 🎫️ Captures both original sources and quotes before a caller-owned effect.
 pub fn begin_effect_settlement(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>,demand:RetirementDemand,work_quote:u64)->Result<(),ValueError>{
  if slot.is_some(){return Err(invalid("original effect settlement slot is occupied"))}if work_quote==0{return Err(invalid("original effect requires a nonzero work quote"))}self.checkpoint()?;let construction=self.construction_recipient_identity().ok_or_else(||invalid("original construction issuer is absent"))?;let work=self.work_recipient_identity().ok_or_else(||invalid("original work issuer is absent"))?;
  *slot=Some(NativeEncodeEffectSettlement{identity:EFFECT_IDS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|id|id.checked_add(1)).expect("effect identity exhausted"),construction,work_issuer:work,work_quote,actual_work:None,grant:None,demand,quote_phase:NativeEncodeEffectPostPhase::Started,admission_phase:NativeEncodeEffectPostPhase::Waiting,admitted:false,progress:None,work_phase:NativeEncodeEffectPostPhase::Waiting,physical_phase:NativeEncodeEffectPostPhase::Waiting,work_failure:None,physical_failure:None,invariant_failure:None,released:false});
  let quote=self.construction_grant(demand);slot.as_mut().unwrap().quote_phase=NativeEncodeEffectPostPhase::Completed;let grant=match quote{Ok(grant)=>grant,Err(error)=>{release_unstarted(slot);return Err(error)}};slot.as_mut().unwrap().grant=Some(grant);if grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original effect quote is insufficient"))}let state=slot.as_mut().unwrap();state.admission_phase=NativeEncodeEffectPostPhase::Started;let admitted=self.prepare_work(work_quote);slot.as_mut().unwrap().admission_phase=NativeEncodeEffectPostPhase::Completed;match admitted{Ok(true)=>{slot.as_mut().unwrap().admitted=true;Ok(())},Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original effect work is refused")),Err(error)=>Err(error)}
 }
 /// 🪢️ Reconciles an interrupted original quote without invoking it a second time.
 /// 🎟️ Reserves the original work after a reconciled quote.
 pub fn recover_effect_quote(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).recover_quote(slot)}
 pub fn resume_effect_admission(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).resume_admission(slot)}
 /// 🪢️ Reconciles a started work-admission callback without replaying it.
 /// ↩️ Returns a reserved action that never performed an effect.
 pub fn cancel_effect_settlement(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<NativeEncodeEffectCancellation,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).cancel(slot)}
 /// ↩️ Releases an unperformed refused original action with the same source loans.
 pub fn release_refused_effect(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<NativeEncodeEffectCancellation,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).release_refused(slot)}
 /// 🧾️ Posts each actual receipt at most once to its original issuer.
 pub fn settle_effect(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).settle(slot)}
 /// 🪢️ Reconciles the original physical reply without replaying its producer.
 pub fn recover_effect_receipt(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{self.settle_effect(slot)}
 /// 🪢️ Reconciles one interrupted callback using the same original authorities without replay.
 /// 👁️ Visits a caller-retained frame while lending its original effect authorities.
 pub fn visit_effect_original<T:Send+'static,O:Send+'static>(&mut self,recipient_identity:u64,visitor:&mut impl FnMut(&mut Option<T>,&mut NativeEncodeEffectRecovery<'_,'_>))->Result<bool,ValueError>{let recipient=self.retirement.as_deref_mut().ok_or_else(||invalid("original effect recovery recipient is absent"))?;if recipient.identity()!=recipient_identity{return Err(invalid("original effect recovery recipient changed"))}let mut recovery=NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity);Ok(crate::ErasedSnapshotRetirement::visit_original(recipient,std::any::TypeId::of::<(T,O)>(),&mut |original|{if let Some(frame)=original.downcast_mut::<Option<T>>(){visitor(frame,&mut recovery)}}))}
}
