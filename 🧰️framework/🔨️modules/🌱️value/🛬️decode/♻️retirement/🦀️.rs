//! ♻️ Original retirement loans retain decoder custody after new semantic work is cancelled.
use super::{NativeDecodeControl,NativeDecodeProgress,allocation};
use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneStep,retained_clone_grant_admits}};
/// 🪢️ Exposes only original semantic admission during a prepared work action.
pub trait NativePreparedDecodeControl{
 fn available_work_units(&mut self)->Result<u64,ValueError>;
 fn checkpoint(&mut self)->Result<(),ValueError>;
 fn admit_turn_capacity(&mut self,bytes:usize)->Result<(),ValueError>;
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
}
impl NativePreparedDecodeControl for NativeDecodeControl<'_>{
 fn available_work_units(&mut self)->Result<u64,ValueError>{Ok(NativeDecodeControl::pending_work(self))}
 fn checkpoint(&mut self)->Result<(),ValueError>{NativeDecodeControl::checkpoint(self)}
 fn admit_turn_capacity(&mut self,bytes:usize)->Result<(),ValueError>{NativeDecodeControl::admit_turn_capacity(self,bytes)}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{NativeDecodeControl::charge(self,bytes)}
}
/// 🫴️ A borrowed original ledger permits only the paid capacity of one already-owned retirement hop.
pub struct NativeRetirementDecodeControl<'operation,'decode>{original:&'operation mut NativeDecodeControl<'decode>,funding:Option<&'operation mut super::recipient::NativeDecodeRetirementFunding>,capacity:usize,spent:usize,previous_maximum:usize}
fn invalid(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::literal(kind,message)}
impl<'operation,'decode> NativeRetirementDecodeControl<'operation,'decode>{
 fn begin(original:&'operation mut NativeDecodeControl<'decode>,grant:RetainedCloneGrant)->Result<Self,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Err(invalid(ValueRefusalKind::WorkLimit,"original retirement requires its paid caller turn"));}if original.retirement.is_none(){return Err(invalid(ValueRefusalKind::UnsupportedOwner,"original retirement requires its exact original recipient"));}let previous_maximum=original.maximum_bytes;original.admit_turn_capacity(grant.maximum_capacity_bytes)?;Ok(Self{original,funding:None,capacity:grant.maximum_capacity_bytes,spent:0,previous_maximum})}
 /// 🪪️ Observes the unchanged original decoder issuance.
 pub fn ledger_identity(&self)->crate::NativeAllocationIdentity{self.original.ledger_identity()}
 /// 🧷️ Observes the exact original recipient retained by this decoder continuation.
 pub fn retirement_recipient_identity(&self)->Option<u64>{self.funding.as_ref().map(|funding|funding.identity).or_else(||self.original.retirement_recipient_identity())}
 /// 🔭️ Reads the actual current or already-held work without another reservation.
 /// ⏳️ Reports the original issuer's retained work action before callback resolution.
 pub fn has_pending_work(&self)->bool{self.original.has_pending_work()}
 /// 🚦️ Reports the original unresolved work quote without invoking it again.
 pub fn work_quote_pending(&self)->bool{self.original.work_quote_pending()}
 /// 🛟️ Settles an existing value effect through its unchanged original recipient and work issuer.
 /// ♻️ Births only an owned retirement frontier through the same physical and work recipients.
 pub fn receive_retirement(&mut self,demand:RetirementDemand,frame:&mut super::NativeReceivingEffect<crate::OriginalRetirementFailure>,allocating:bool,work:u64,operation:impl FnOnce(RetainedCloneGrant,Option<&mut Option<usize>>)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,super::OriginalReceivingPrimary>{let before=self.original.owned_bytes();let spend=RetirementEncodingSpend{loan:self,before};spend.loan.original.receive_retirement(demand,frame,allocating,work,operation)}
 pub fn finish_receiving(&mut self,frame:&mut super::NativeReceivingEffect)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,super::OriginalReceivingPrimary>{self.original.finish_receiving(frame)}
 /// ♻️ Settles an existing retirement effect while preserving its whole original child failure.
 pub fn finish_retirement_receiving(&mut self,frame:&mut super::NativeReceivingEffect<crate::OriginalRetirementFailure>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,super::OriginalReceivingPrimary>{self.original.finish_retirement_receiving(frame)}
 /// 🧳️ Dispatches only an already-owned recovery loan without exposing semantic admission.
 pub fn finish_recovery(&mut self,loan:super::construction::NativeRecoveryLoan<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,super::OriginalReceivingPrimary>{match loan{super::construction::NativeRecoveryLoan::Value(frame)=>self.finish_receiving(frame),super::construction::NativeRecoveryLoan::Retirement(frame)=>self.finish_retirement_receiving(frame)}}
 /// 🔁️ Recovers the actual recorded grant without an injected reply.
 pub fn recover_receiving_grant<E:super::NativeReceivingEffectError>(&mut self,frame:&mut super::NativeReceivingEffect<E>)->Result<bool,ValueError>{frame.recover_grant(self.original)}
 /// 🔁️ Recovers the actual recorded allocation callback without an injected reply.
 pub fn recover_receiving_allocation_callback<E:super::NativeReceivingEffectError>(&mut self,frame:&mut super::NativeReceivingEffect<E>)->Result<bool,ValueError>{frame.recover_allocation_callback(self.original)}
 /// ⛽️ Resolves only the pending quote of this unchanged original work issuer.
 pub fn recover_work_quote(&mut self)->Result<Option<bool>,ValueError>{self.original.recover_work_quote()}
 /// 🔁️ Recovers the actual recorded receipt without an injected reply.
 pub fn recover_receiving_receipt<E:super::NativeReceivingEffectError>(&mut self,frame:&mut super::NativeReceivingEffect<E>)->Result<bool,ValueError>{frame.recover_receipt(self.original)}
 /// 🏷️ Reconciles only the retained actual marker of an already-started physical effect.
 pub fn resolve_receiving_effect_marker<E:super::NativeReceivingEffectError>(&mut self,frame:&mut super::NativeReceivingEffect<E>,actual:Option<usize>)->Result<(),ValueError>{frame.resolve_effect_marker(self.original,actual)}
 /// 🏁️ Finishes the already-started allocation through its original captured port.
 pub fn finish_receiving_allocation<E:super::NativeReceivingEffectError>(&mut self,frame:&mut super::NativeReceivingEffect<E>)->Result<(),ValueError>{frame.finish_allocation(self.original)}
 /// 🫴️ Retains a real allocated child's exact response or returns the entire donor on refusal.
 pub fn resolve_receiving_child<E:super::NativeReceivingEffectError>(&mut self,frame:&mut super::NativeReceivingEffect<E>,reply:crate::RetainedWorkOutcome<RetainedCloneStep,E>)->Result<(),(ValueError,crate::RetainedWorkOutcome<RetainedCloneStep,E>)>{frame.resolve_child(self.original,reply)}
 /// 🧮️ Returns one real unallocated child's response only to its pending original frontier.
 pub fn resolve_unallocated_receiving_child<E:super::NativeReceivingEffectError>(&mut self,frame:&mut super::NativeReceivingEffect<E>,reply:crate::RetainedWorkOutcome<RetainedCloneStep,E>)->Result<(),(ValueError,crate::RetainedWorkOutcome<RetainedCloneStep,E>)>{frame.resolve_unallocated_child(self.original,reply)}
 /// 📊️ Reads cumulative original allocation without issuing another capacity.
 pub fn owned_bytes(&self)->usize{self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes()}
 /// 🧾️ Retirement bytes already charged against this loan's paid capacity.
 pub fn spent_bytes(&self)->usize{self.spent}
 /// 🎟️ Admits one real cleanup birth against this original hop before reporting the original cancellation observer.
 pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{let spent=self.spent.checked_add(bytes).filter(|spent|*spent<=self.capacity).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original retirement exceeds its paid hop capacity"))?;let next=self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes().checked_add(bytes).filter(|next|*next<=self.original.maximum_bytes).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original retirement exceeds its cumulative ownership ceiling"))?;if matches!(&self.original.allocation,allocation::Binding::Forwarded(_)){return Err(invalid(ValueRefusalKind::UnsupportedOwner,"forwarded retirement allocation requires actual receipt admission"))}(self.original.callback)(NativeDecodeProgress{completed:self.original.completed,total:self.original.total,owned_bytes:self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes()});allocation::commit_admission(&mut self.original.ledger,super::NativeAllocationRequest{bytes,owned_bytes:next-bytes,next_owned_bytes:next,maximum_bytes:self.original.maximum_bytes})?;self.spent=spent;Ok(())}
 fn failure_protocol<T>(&mut self,error:ValueError)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<T>{let recipient=match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding};if let Err(error)=recipient.failure_birth.retain_protocol_refusal(error){assert!(recipient.failure_protocol_refusal.is_none(),"original failure protocol refusal slot remains occupied");recipient.failure_protocol_refusal=Some(error)}semio_framework_value_retirement_contract::OriginalFailureStorageStep::Pending}
 fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<RetainedCloneGrant>{
  use semio_framework_value_retirement_contract::OriginalFailureStorageStep as Step;use crate::NativeAllocationTransactionPhase as Phase;
  let available=self.capacity.saturating_sub(self.spent).min(self.original.maximum_bytes.saturating_sub(self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes()));let quoted=RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.min(available),..grant};if !retained_clone_grant_admits(quoted,demand){return Step::Complete(Err(invalid(ValueRefusalKind::OwnershipLimit,"original failure storage exceeds its retirement grant").into()))}
  let recipient=match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding};if recipient.failure_protocol_refusal.is_some(){return Step::Pending}if recipient.failure_birth.phase()!=semio_framework_value_retirement_contract::OriginalFailureStorageBirthPhase::Quote{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure quote phase changed"))}if recipient.failure_birth.work_action().is_none()||recipient.failure_birth.work_action()!=self.original.work.action_identity()||!self.original.work.same_issuer()||self.original.work.quote_pending()||self.original.work.reserved()<1{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure work reservation changed"))}let request=match recipient.failure_birth.request(){Some(request)=>request,None=>{let Some(next)=self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes().checked_add(demand.capacity_bytes).filter(|next|*next<=self.original.maximum_bytes)else{return Step::Complete(Err(invalid(ValueRefusalKind::OwnershipLimit,"original failure storage exceeds its ownership ceiling").into()))};let request=super::NativeAllocationRequest{bytes:demand.capacity_bytes,owned_bytes:self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes(),next_owned_bytes:next,maximum_bytes:self.original.maximum_bytes};if let Err(error)=recipient.failure_birth.install_request(request){return Step::Complete(Err(error.into()))}request}};if self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes()!=request.owned_bytes||self.original.maximum_bytes!=request.maximum_bytes{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure quote ledger changed"))}
  let identity=match &self.original.allocation{allocation::Binding::Local=>None,allocation::Binding::Forwarded(port)=>Some(port.identity())};match recipient.failure_port{None if recipient.failure_ledger.is_none()=>{recipient.failure_port=Some(identity);recipient.failure_ledger=Some(self.original.ledger.as_ref().expect("original decode ledger present").identity())},Some(original)if original==identity&&recipient.failure_ledger==Some(self.original.ledger.as_ref().expect("original decode ledger present").identity())=>{},_=>return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure allocation authority or ledger changed"))}let port=match &mut self.original.allocation{allocation::Binding::Local=>return Step::Complete(Ok(quoted)),allocation::Binding::Forwarded(port)=>port};
  if recipient.failure_allocation.is_none(){if let Err(error)=port.begin(request,true,&mut recipient.failure_allocation){return self.failure_protocol(error)}}else{let state=recipient.failure_allocation.as_mut().expect("original allocation transaction present");if state.request()!=request{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"failure allocation request changed"))}if state.phase()==Phase::QuoteStarted{match port.recover(state){Ok(true)=>{},Ok(false)=>return Step::Pending,Err(error)=>return self.failure_protocol(error)}}}
  let state=recipient.failure_allocation.as_mut().expect("original allocation transaction present");if state.phase()==Phase::QuoteStarted{return Step::Pending}if recipient.failure_verdict.is_none(){recipient.failure_verdict=state.take_verdict()}let accepted=matches!(recipient.failure_verdict.as_ref(),Some(Ok(())));if !accepted{if let Err(error)=port.finish(&mut recipient.failure_allocation){return self.failure_protocol(error)}}match recipient.failure_verdict.take(){Some(Ok(()))=>Step::Complete(Ok(quoted)),Some(Err(error))=>{recipient.failure_port=None;recipient.failure_ledger=None;Step::Complete(Err(error))},None=>self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original allocation quote verdict is absent"))}
 }
 fn settle_failure_storage(&mut self,actual:Option<usize>)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<()>{
  use semio_framework_value_retirement_contract::OriginalFailureStorageStep as Step;use crate::NativeAllocationTransactionPhase as Phase;
  let recipient=match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding};if recipient.failure_protocol_refusal.is_some(){return Step::Pending}let expected=if actual.is_some(){semio_framework_value_retirement_contract::OriginalFailureStorageBirthPhase::Post}else{semio_framework_value_retirement_contract::OriginalFailureStorageBirthPhase::CancelAllocation};if recipient.failure_birth.phase()!=expected{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure settlement phase changed"))}if actual.is_none()&&recipient.failure_birth.work_proof().is_none()&&(recipient.failure_birth.work_action().is_none()||recipient.failure_birth.work_action()!=self.original.work.action_identity()||!self.original.work.same_issuer()||self.original.work.quote_pending()||self.original.work.reserved()<1){return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure work reservation changed"))}let Some(request)=recipient.failure_birth.request()else{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure storage request is absent"))};
  let identity=match &self.original.allocation{allocation::Binding::Local=>None,allocation::Binding::Forwarded(port)=>Some(port.identity())};if recipient.failure_port!=Some(identity)||recipient.failure_ledger!=Some(self.original.ledger.as_ref().expect("original decode ledger present").identity()){return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure allocation authority or ledger changed"))}
  if actual.is_some()||recipient.failure_birth.work_proof().is_some(){let Some(proof)=recipient.failure_birth.work_proof()else{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure work proof is absent"))};match self.original.work.validates(proof,recipient.failure_birth.work()){Ok(true)=>{},Ok(false)=>return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure work proof changed")),Err(error)=>return self.failure_protocol(error)}if let Some(bytes)=actual{if recipient.failure_birth.actual()!=Some(bytes)||recipient.failure_birth.progress().retained_capacity_bytes!=bytes{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure physical marker changed"))}if self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes()==request.owned_bytes{let Some(spent)=self.spent.checked_add(bytes).filter(|spent|*spent<=self.capacity)else{return self.failure_protocol(invalid(ValueRefusalKind::OwnershipLimit,"original failure storage exceeds its paid capacity"))};self.spent=spent;if let Err(error)=allocation::commit_admission(&mut self.original.ledger,request){return self.failure_protocol(error)}}else if self.original.ledger.as_ref().expect("original decode ledger present").owned_bytes()!=request.next_owned_bytes{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure admission changed"))}}}
  let port=match &mut self.original.allocation{allocation::Binding::Local=>{recipient.failure_port=None;recipient.failure_ledger=None;return Step::Complete(Ok(()))},allocation::Binding::Forwarded(port)=>port};let Some(state)=recipient.failure_allocation.as_mut()else{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure quote is absent"))};if state.request()!=request{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure quote changed"))}match state.phase(){Phase::QuoteAccepted=>if let Err(error)=port.settle(state,actual){return self.failure_protocol(error)},Phase::PostStarted|Phase::CancelStarted=>match port.recover(state){Ok(true)=>{},Ok(false)=>return Step::Pending,Err(error)=>return self.failure_protocol(error)},Phase::Settled=>{},_=>return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure settlement phase changed"))}
  let state=recipient.failure_allocation.as_mut().expect("original allocation transaction present");if matches!(state.phase(),Phase::PostStarted|Phase::CancelStarted){return Step::Pending}if recipient.failure_verdict.is_none(){recipient.failure_verdict=state.take_verdict()}if recipient.failure_verdict.is_none(){return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure settlement verdict is absent"))}if let Err(error)=port.finish(&mut recipient.failure_allocation){return self.failure_protocol(error)}recipient.failure_port=None;recipient.failure_ledger=None;Step::Complete(recipient.failure_verdict.take().expect("original verdict retained"))
 }
 fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:crate::RetainedCloneProgress)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<()>{let recipient=match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding};if recipient.failure_protocol_refusal.is_some(){return semio_framework_value_retirement_contract::OriginalFailureStorageStep::Pending}if recipient.failure_birth.quoted_grant()!=Some(grant)||recipient.failure_birth.progress()!=progress||!progress.fits(grant){return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure receipt changed"))}let Some(actual)=recipient.failure_birth.actual()else{return self.failure_protocol(invalid(ValueRefusalKind::InvariantViolated,"original failure allocator marker is absent"))};self.settle_failure_storage(Some(actual))}
 fn cancel_failure_storage(&mut self)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<()>{self.settle_failure_storage(None)}
 fn failure_storage_pool(&mut self)->&mut semio_framework_value_retirement_contract::OriginalFailureStoragePool{&mut (match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding}).failure_pool}
 /// 📨️ Forwards an accepted result through this same paid retirement ledger and its original observer.
 pub fn with_encoding_receiver<T,E>(&mut self,frame:&mut Option<semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverFrame>,result:&mut semio_framework_value_call::NativeCallFailure<T,E>,operation:impl FnOnce(usize,&mut dyn FnMut(crate::native_encoding::NativeEncodeProgress)->bool,&mut dyn crate::NativeSendAllocationLoan,&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverWork<'_,'_,'_>,Option<&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverConstruction<'_,'_>>,&mut Option<T>,&mut Option<E>))->Result<(),ValueError>{
  let before=self.original.owned_bytes();let scope=RetirementEncodingSpend{loan:self,before};scope.loan.original.with_encoding_receiver(frame,result,operation)
 }
 /// 🔓️ Releases the settled receiver to this loan's exact original ledger and authorities.
 pub fn release_encoding_receiver(&mut self,frame:&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverFrame)->Result<(),ValueError>{self.original.release_encoding_receiver(frame)}
 /// 👓️ Reports already-born ownership in this exact original recipient.
 pub fn has_retirement_owner(&self)->bool{self.original.has_retirement_owner()}
 /// 🧺️ Observes spare failure custody separately from a pending returned owner.
 pub fn has_retirement_failure_storage(&self)->bool{self.original.has_retirement_failure_storage()}
 /// 📏️ Quotes this original recipient once under the paid cleanup body span.
 /// 👶️ Selects the original resident failure birth before a general physical effect.
 pub fn requires_original_birth(&self)->bool{self.original.retirement.as_ref().is_some_and(|r|!r.funding.failure_birth.terminal_is_empty()||!r.funding.reserved&&r.owner.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&(owner.requires_original_birth()||r.funding.failure_pool.is_empty())))}
 /// ⛽️ Quotes only the selected genuine recipient close action.
 pub fn retirement_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.requires_original_birth(){return Err(invalid(ValueRefusalKind::UnsupportedOwner,"original failure birth requires its separate driver"))}self.original.retirement.as_ref().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement recipient absent"))?.next_computational_work(body)}
 /// 🍼️ Advances the same original failure birth without opening a second allocation effect.
 pub fn close_original_birth_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{
  if !self.requires_original_birth()||self.funding.is_some(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(invalid(ValueRefusalKind::InvariantViolated,"original recipient has no selected birth outside a general effect").into()),work:0})}
  let mut lease=RetirementRecipientLease::new(&mut *self.original);let recipient=lease.recipient.as_deref_mut().expect("original retirement recipient remains installed");let previous_maximum=lease.original.maximum_bytes;
  let mut scope=RetirementFundingScope{loan:NativeRetirementDecodeControl{original:&mut *lease.original,funding:Some(&mut recipient.funding),capacity:self.capacity,spent:self.spent,previous_maximum},spent:&mut self.spent};
  match scope.loan.with_original_close(|original|Ok::<_,ValueError>(semio_framework_value_retirement::close_retirement_birth(recipient.owner.as_deref_mut(),grant.maximum_copy_bytes,original,grant))){Ok(poll)=>poll,Err(error)=>crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})}
 }
 /// 📐️ Quotes the selected resident birth, child birth or ordinary close in execution order.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let recipient=self.original.retirement.as_ref().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement has no recipient"))?;
  if !recipient.funding.failure_birth.terminal_is_empty(){return Ok(recipient.funding.failure_birth.demand())}
  if !recipient.funding.reserved{if let Some(owner)=recipient.owner.as_ref(){if !owner.terminal_is_empty(){if owner.requires_original_birth(){return owner.next_demand(body)}if recipient.funding.failure_pool.is_empty(){return Ok(semio_framework_value_retirement::original_failure_storage_birth_demand())}}}}
  recipient.next_demand(body)
 }
 /// 🚮️ Charges and closes the original returned owner through the same performed-work sink.
 pub fn close_retirement_recipient(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{  use crate::RetainedWorkPoll;
  let grant=effect.grant();
  if self.requires_original_birth(){return RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(invalid(ValueRefusalKind::InvariantViolated,"original failure birth must precede the general effect").into()),work:0})}
  let demand=match self.retirement_demands(body){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if !retained_clone_grant_admits(grant,demand){return RetainedWorkPoll::Pending{work:0}}
  let recipient=match self.original.retirement.as_deref_mut(){Some(recipient)=>recipient,None=>return RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0})};
  if recipient.terminal_is_empty(){return RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
  if recipient.funding.reserved{return recipient.close_reserved_step(effect).map_err(Into::into)}
  if recipient.owner.is_none()&&!recipient.funding.failure_pool.is_empty(){return recipient.close_spare_step(effect).map_err(Into::into)}
  let mut lease=RetirementRecipientLease::new(&mut *self.original);let recipient=lease.recipient.as_deref_mut().expect("original retirement recipient remains installed");let previous_maximum=lease.original.maximum_bytes;
  let mut scope=RetirementFundingScope{loan:NativeRetirementDecodeControl{original:&mut *lease.original,funding:Some(&mut recipient.funding),capacity:self.capacity,spent:self.spent,previous_maximum},spent:&mut self.spent};
  semio_framework_value_retirement::close_factory_ticket_controlled(&mut recipient.owner,body,effect,&mut scope.loan)
 }


}


impl NativeRetirementDecodeControl<'_,'_>{
 /// 🛠️ Lends the original cleanup funding to the same resident producer authority.
 pub fn receive_original_effect<P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut super::NativeReceivingEffect<P::Failure>,authority:&mut super::OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeRetirementDecodeControl<'_,'_>,&mut crate::RetainedAllocationEffect<'_>,&mut super::OriginalEffectAuthority<P>,&mut Option<super::OriginalEffectAction<RetainedCloneStep,P::Failure>>)->Result<bool,ValueError>)->Result<super::NativeReceivingEffectPoll<P::Failure>,ValueError>where P:super::OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:super::NativeReceivingEffectError{
  if self.requires_original_birth(){return Err(invalid(ValueRefusalKind::InvariantViolated,"original failure birth must precede the cleanup effect"))}
  if frame.phase()==super::NativeReceivingEffectPhase::Empty&&demand.capacity_bytes>self.capacity.saturating_sub(self.spent){return Ok(super::NativeReceivingEffectPoll::Pending{work:0})}
  if self.funding.is_some(){return Err(invalid(ValueRefusalKind::InvariantViolated,"cleanup effect cannot replace its active original funding"))}
  let before=self.original.owned_bytes();let initial_spent=self.spent;let accounting=RetirementEffectSpend{loan:self,before,initial_spent};let owner=&mut *accounting.loan;
  if frame.phase()==super::NativeReceivingEffectPhase::Empty{frame.quote_retirement(owner.original,demand,maximum_work)?}
  let capacity=owner.capacity;let spent=&mut owner.spent;
  owner.original.receive_original_effect(demand,maximum_work,frame,authority,|control,effect,authority,action|{let previous_maximum=control.maximum_bytes;let mut funding=RetirementFundingScope{loan:NativeRetirementDecodeControl{original:control,funding:None,capacity,spent:*spent,previous_maximum},spent};invoke(&mut funding.loan,effect,authority,action)})
 }
 /// 🛠️ Lends the same cleanup funding through the shared resident receiving driver.
 pub fn receive_effect<E:super::NativeReceivingEffectError>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut super::NativeReceivingEffect<E>,operation:impl FnOnce(&mut NativeRetirementDecodeControl<'_,'_>,&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,E>)->Result<super::NativeReceivingEffectPoll<E>,ValueError>{
  if self.requires_original_birth(){return Err(invalid(ValueRefusalKind::InvariantViolated,"original failure birth must precede the cleanup effect"))}
  if frame.phase()==super::NativeReceivingEffectPhase::Empty&&demand.capacity_bytes>self.capacity.saturating_sub(self.spent){return Ok(super::NativeReceivingEffectPoll::Pending{work:0})}
  if self.funding.is_some(){return Err(invalid(ValueRefusalKind::InvariantViolated,"cleanup effect cannot replace its active original funding"))}
  let before=self.original.owned_bytes();let initial_spent=self.spent;let accounting=RetirementEffectSpend{loan:self,before,initial_spent};let owner=&mut *accounting.loan;
  if frame.phase()==super::NativeReceivingEffectPhase::Empty{frame.quote_retirement(owner.original,demand,maximum_work)?}
  let capacity=owner.capacity;let spent=&mut owner.spent;
  owner.original.receive_effect(demand,maximum_work,frame,|control,effect|{let previous_maximum=control.maximum_bytes;let mut funding=RetirementFundingScope{loan:NativeRetirementDecodeControl{original:control,funding:None,capacity,spent:*spent,previous_maximum},spent};operation(&mut funding.loan,effect)})
 }
}
struct RetirementEffectSpend<'turn,'operation,'decode>{loan:&'turn mut NativeRetirementDecodeControl<'operation,'decode>,before:usize,initial_spent:usize}
impl Drop for RetirementEffectSpend<'_,'_,'_>{fn drop(&mut self){let admitted=self.loan.original.owned_bytes().checked_sub(self.before).expect("original cumulative cleanup admission regressed");self.loan.spent=self.loan.spent.max(self.initial_spent.checked_add(admitted).expect("original cleanup admission overflow"))}}
struct RetirementRecipientLease<'turn,'decode>{original:&'turn mut NativeDecodeControl<'decode>,recipient:Option<&'decode mut super::NativeDecodeRetirementRecipient>}
impl<'turn,'decode> RetirementRecipientLease<'turn,'decode>{fn new(original:&'turn mut NativeDecodeControl<'decode>)->Self{let recipient=original.retirement.take();Self{original,recipient}}}
impl Drop for RetirementRecipientLease<'_,'_>{fn drop(&mut self){assert!(self.original.retirement.is_none(),"original retirement reference was replaced during funding");self.original.retirement=self.recipient.take()}}
struct RetirementFundingScope<'turn,'operation,'decode>{loan:NativeRetirementDecodeControl<'operation,'decode>,spent:&'turn mut usize}
impl Drop for RetirementFundingScope<'_,'_,'_>{fn drop(&mut self){*self.spent=self.loan.spent}}
struct RetirementEncodingSpend<'loan,'operation,'decode>{loan:&'loan mut NativeRetirementDecodeControl<'operation,'decode>,before:usize}
impl Drop for RetirementEncodingSpend<'_,'_,'_>{fn drop(&mut self){self.loan.spent+=self.loan.original.owned_bytes()-self.before;}}
impl Drop for NativeRetirementDecodeControl<'_, '_>{fn drop(&mut self){if !self.original.receiving{self.original.maximum_bytes=self.previous_maximum;}}}
impl semio_framework_value_retirement::OriginalRetirementControl for NativeRetirementDecodeControl<'_, '_>{
 fn failure_storage_birth(&mut self)->&mut semio_framework_value_retirement_contract::OriginalFailureStorageBirth{&mut (match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding}).failure_birth}
 fn failure_storage_performed_scope(&mut self)->Result<semio_framework_value_retirement_contract::OriginalFailurePerformedScope<'_>,ValueError>{let funding=match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement recipient is absent"))?.funding};self.original.work.validate_action_source(funding.failure_birth.work_action())?;semio_framework_value_retirement_contract::OriginalFailurePerformedScope::new(&mut self.original.work.state,&mut funding.failure_birth,&mut funding.failure_pool)}
 fn settle_failure_storage_performed(&mut self)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<(),ValueError>{let funding=match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding};funding.failure_birth.settle_performed(&mut self.original.work)}
 fn prepare_failure_storage_work(&mut self,work:u64)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<bool,ValueError>{use semio_framework_value_retirement_contract::OriginalFailureStorageStep as Step;let birth=&mut (match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").funding}).failure_birth;match birth.work_action_output().and_then(|action|self.original.work.prepare_for_action(work,action)){Ok(Some(accepted))=>Step::Complete(Ok(accepted)),Ok(None)=>Step::Pending,Err(error)=>Step::Complete(Err(error))}}
 fn commit_failure_storage_work(&mut self,actual:u128)->Result<(),ValueError>{let birth=&mut (match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement recipient is absent"))?.funding}).failure_birth;let action=birth.work_action();self.original.work.commit_for_action(actual,action,birth.work_proof_output()?).and_then(|accepted|if accepted{Ok(())}else{Err(invalid(ValueRefusalKind::WorkLimit,"original failure work exceeded its quote"))})}
 fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>{let birth=&mut (match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement recipient is absent"))?.funding}).failure_birth;self.original.work.cancel_for_action(birth.work_action_slot()?)}
 fn finish_failure_storage_work(&mut self)->Result<(),ValueError>{let birth=&mut (match self.funding.as_deref_mut(){Some(funding)=>funding,None=>&mut self.original.retirement.as_deref_mut().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement recipient is absent"))?.funding}).failure_birth;let proof=birth.work_proof().ok_or_else(||invalid(ValueRefusalKind::InvariantViolated,"original failure action proof is absent"))?;if !self.original.work.validates(proof,birth.work())?{return Err(invalid(ValueRefusalKind::InvariantViolated,"original failure action proof changed"))}let(proof,action)=birth.finishing_work_slots()?;self.original.work.finish_for_action(proof,action)}

 fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{self.original.prepare_work(additional)}
 fn commit_work(&mut self,actual:u128)->Result<(),ValueError>{self.original.commit_work(actual)}
 fn cancel_prepared_work(&mut self){self.original.cancel_prepared_work()}
 fn ledger_identity(&self)->crate::NativeAllocationIdentity{self.original.ledger_identity()}
 fn retirement_recipient_identity(&self)->Option<u64>{NativeRetirementDecodeControl::retirement_recipient_identity(self)}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{NativeRetirementDecodeControl::charge(self,bytes)}
 fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<RetainedCloneGrant>{NativeRetirementDecodeControl::quote_failure_storage(self,demand,grant)}
 fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:crate::RetainedCloneProgress)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<()>{NativeRetirementDecodeControl::post_failure_storage(self,grant,progress)}
 fn cancel_failure_storage(&mut self)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<()>{NativeRetirementDecodeControl::cancel_failure_storage(self)}
 fn failure_storage_pool(&mut self)->&mut semio_framework_value_retirement_contract::OriginalFailureStoragePool{NativeRetirementDecodeControl::failure_storage_pool(self)}
}
impl<'decode> NativeDecodeControl<'decode>{
 /// 🛟️ Loans only retirement capacity while keeping this operation's original ledger, observer and recipient.
 pub fn with_retirement<T,E:From<ValueError>>(&mut self,grant:RetainedCloneGrant,operation:impl FnOnce(&mut NativeRetirementDecodeControl<'_, 'decode>)->Result<T,E>)->Result<T,E>{let mut loan=NativeRetirementDecodeControl::begin(self,grant).map_err(E::from)?;operation(&mut loan)}
 /// 🧺️ Funds a collision node on its own original turn before a semantic child can run.
 pub fn prepare_failure_storage(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{
  match self.with_retirement::<_,ValueError>(grant,|original|{let funding:&mut dyn semio_framework_value_retirement::OriginalRetirementControl=original;Ok(semio_framework_value_retirement::refill_original_failure_storage(funding,grant))}){Ok(turn)=>turn,Err(error)=>crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})}
 }
 /// 🧾️ Advances only with prepaid collision custody and settles the actual typed child work on this original source.
 pub fn run_prepared_work<T:Copy>(&mut self,maximum_work:u64,operation:impl FnOnce(&mut dyn NativePreparedDecodeControl)->crate::RetainedWorkOutcome<T,semio_framework_value_retirement::OriginalRetirementFailure>,effects:impl FnOnce(&T)->crate::RetainedCloneProgress)->Option<crate::RetainedWorkOutcome<T,semio_framework_value_retirement::OriginalRetirementFailure>>{
  if !self.has_retirement_failure_storage(){return None}
  match self.prepare_work(maximum_work){Ok(true)=>{},Ok(false)=>return None,Err(error)=>return Some(crate::RetainedWorkOutcome{result:Err(semio_framework_value_retirement::OriginalRetirementFailure::from_work_refusal(error)),work:0})}
  let Some(recipient)=self.retirement.as_deref_mut()else{self.cancel_prepared_work();return None};let recipient=std::ptr::from_mut(recipient);
  let mut storage=unsafe{(&mut *recipient).funding.failure_pool.take()};
  if storage.is_none(){self.cancel_prepared_work();return None}
  let attempted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||semio_framework_value_retirement_contract::run_with_failure_storage_transaction(&mut storage,||{let turn=operation(self);let settlement=self.commit_work(turn.work);(turn,settlement)},effects)));
  if let Some(storage)=storage{unsafe{(&mut *recipient).funding.failure_pool.put(storage)}}
  match attempted{Ok(Some(turn))=>Some(turn),Ok(None)=>{self.cancel_prepared_work();None},Err(payload)=>std::panic::resume_unwind(payload)}
 }
}

#[path="🔒️close/🦀️.rs"]
mod close;
pub use close::NativeDecodeCloseControl;

