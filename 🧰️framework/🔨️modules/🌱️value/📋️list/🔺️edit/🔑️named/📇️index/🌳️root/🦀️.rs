//! 🌳️ Independent immutable named roots retain their original row source and admitted sorted directory.
use super::*;
use semio_framework_value_retirement::{OriginalRetirementPoll,RetirementPollExit};
use crate::retained_clone::{RetainedWorkPoll,RetainedOwnedProjectionAdmission};
use crate::{list::persistent::PersistentList,named_edit::PersistentRankedKeys,retained_clone::RetainedCloneSource};
#[path="🛬️receive/🦀️.rs"]mod receiving;
pub use receiving::{NamedRecordKey,NamedPersistentRootReceiver};
/// 🪢️ Immutable payload contains no mutable directory retirement or projection-binding cursor.
pub struct NamedPersistentRoot<T:RetireOwned+Send+Sync+'static>{source:ManuallyDrop<Option<RetainedCloneSource<PersistentList<T>>>>,directory:ManuallyDrop<Option<SealedShared<Vec<usize>>>>,len:usize,key:fn(&T)->&str}
impl<T:RetireOwned+Send+Sync+'static> NamedPersistentRoot<T>{
 pub fn constructor_demand()->RetirementDemand{fixed(size_of::<NamedPersistentRootPreparation<T>>()*2)}
 pub fn admit_parts(source:&mut Option<RetainedCloneSource<PersistentList<T>>>,index:&mut Option<AdmittedNamedKeyIndex>,key:fn(&T)->&str,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(NamedPersistentRootPreparation<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}index.as_ref().ok_or_else(invalid)?.check_source(&PersistentRankedKeys{original:source.as_ref().ok_or_else(invalid)?.try_borrow()?,key})?;work+=1;Ok(Some((NamedPersistentRootPreparation{source:ManuallyDrop::new(source.take()),index:ManuallyDrop::new(index.take()),key,closing:false,ready:false},paid(d))))})();crate::RetainedWorkOutcome{result,work}}
 pub fn ranked(&self)->Result<PersistentRankedKeys<'_,T>,ValueError>{Ok(PersistentRankedKeys{original:self.source.as_ref().ok_or_else(invalid)?.try_borrow()?,key:self.key})}
 /// 📖️ Quotes the independently retained row reader that keeps the original source alive after named-root closure.
 pub fn project_rows_demand(&self,output:&RetainedOwnedProjectionAdmission<PersistentList<T>>)->Result<RetirementDemand,ValueError>{output.demand(self.ranked()?.original)}
 pub fn project_rows_work_demand(&self,output:&RetainedOwnedProjectionAdmission<PersistentList<T>>)->Result<u64,ValueError>{output.work_demand(self.ranked()?.original)}
 pub fn project_rows_into<'r>(&self,output:&mut RetainedOwnedProjectionAdmission<PersistentList<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{output.admit(self.ranked()?.original,effect,reply)}
 pub fn recover_project_rows_into<'r>(&self,output:&mut RetainedOwnedProjectionAdmission<PersistentList<T>>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{output.recover_admit(reply)}
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
 pub fn project_index_demand(&self)->Result<RetirementDemand,ValueError>{let original=self.ranked()?.original;Ok(fixed(size_of::<AdmittedNamedKeyIndex>()*2+original.binding_copy_bytes()+size_of::<SealedShared<Vec<usize>>>()))}
 pub fn project_index(&self,output:&mut Option<AdmittedNamedKeyIndex>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let grant=effect.grant();let mut work=0;let result=(||{
 let d=self.project_index_demand()?;let ranked=self.ranked()?;let directory=self.directory.as_ref().ok_or_else(invalid)?;
 if let Some(owner)=output.as_ref(){if owner.closing||owner.len!=self.len||owner.projection!=self.key as usize||!ranked.original.check_binding(&owner.original)?||owner.directory.as_ref().is_some_and(|actual|actual.identity()!=directory.identity()){return Err(invalid())}if owner.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}}
 if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
 if output.is_none(){*output=Some(AdmittedNamedKeyIndex::new(None,None,self.len,self.key as usize));work+=1;}
 let owner=output.as_mut().ok_or_else(||ValueError::invariant("output absent"))?;if owner.original.is_none(){take_work!(work,ranked.original.bind(&mut owner.original,effect)).ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(paid(d)))}
 let(alias,_)=take_work!(work,directory.try_duplicate(effect));*owner.directory=Some(alias);Ok(RetainedCloneStep::Complete(paid(d)))
 })();crate::finish(effect,work,result)}

 fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.directory.is_none()}
}
impl<T:RetireOwned+Send+Sync+'static> Drop for NamedPersistentRoot<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"independent named root requires its original owned retirement cursor");if empty{unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.directory);}}}}
/// 📥️ Source validation and paid binding closure precede immutable root publication.
pub struct NamedPersistentRootPreparation<T:RetireOwned+Send+Sync+'static>{source:ManuallyDrop<Option<RetainedCloneSource<PersistentList<T>>>>,index:ManuallyDrop<Option<AdmittedNamedKeyIndex>>,key:fn(&T)->&str,closing:bool,ready:bool}
impl<T:RetireOwned+Send+Sync+'static> NamedPersistentRootPreparation<T>{
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.ready{return Err(invalid());}RetainedCloneBinding::close_demand(&self.index.as_ref().ok_or_else(invalid)?.original,body)}
 pub fn advance(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(||->Result<RetainedCloneStep,crate::OriginalRetirementFailure>{let d=self.demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let index=self.index.as_mut().ok_or_else(invalid)?;let step=take_work!(work,RetainedCloneBinding::close_one(&mut index.original,grant.maximum_copy_bytes,effect));if index.original.is_none(){self.ready=true;work+=1;}Ok(if self.ready{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})})();crate::finish(effect,work,result)}
 pub fn output_ready(&self)->bool{!self.closing&&self.ready&&self.source.is_some()&&self.index.is_some()}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(size_of::<NamedPersistentRoot<T>>()*2+size_of::<AdmittedNamedKeyIndex>()))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(NamedPersistentRoot<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let index=self.index.as_mut().ok_or_else(invalid)?;work+=1;let root=NamedPersistentRoot{source:ManuallyDrop::new(self.source.take()),directory:ManuallyDrop::new(index.directory.take()),len:index.len,key:self.key};*self.index=None;Ok(Some((root,paid(d))))})();crate::RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(index)=self.index.as_mut(){index.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if let Some(index)=self.index.as_ref(){return if index.terminal_is_empty(){Ok(fixed(size_of::<AdmittedNamedKeyIndex>()*2))}else{index.close_demand(body)};}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed(size_of::<RetainedCloneSource<PersistentList<T>>>()*2))}else{source.next_close_demand(body)};}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if let Some(index)=self.index.as_ref(){return if index.terminal_is_empty(){Ok(1)}else{index.close_work_demand(body)}}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(1)}else{source.next_close_work_demand(body)}}Ok(0)}
 pub fn close_poll(&mut self,mut original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,recover:bool)->OriginalRetirementPoll{let grant=effect.grant();let mut work=0u128;let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
 if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
 let d=self.close_demand(body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
 if let Some(index)=self.index.as_mut(){if index.terminal_is_empty(){*self.index=None;work+=1;return Ok(RetainedCloneStep::Progress(paid(d)))}let step=exit_work!(work,close_via(index,original.take(),body,&mut effect.nested(grant),recover));return Ok(RetainedCloneStep::Progress(step.progress()))}
 if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){*self.source=None;work+=1;return Ok(RetainedCloneStep::Progress(paid(d)))}let step=exit_work!(work,source.close_step(body,&mut effect.nested(grant)));return Ok(RetainedCloneStep::Progress(step.progress()))}
 Ok(RetainedCloneStep::Complete(Default::default()))
 })();settle_poll(effect,work,result)}
 pub fn requires_original_birth(&self)->bool{self.index.as_ref().is_some_and(|index|!index.terminal_is_empty()&&index.requires_original_birth())}
 pub fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{match self.index.as_mut(){Some(index)=>crate::Settle::settled(index.close_original_birth_step(original,grant)),None=>crate::RetainedWorkOutcome{result:Err(invalid().into()),work:0}}}
 pub fn requires_original_control(&self)->bool{self.index.as_ref().is_some_and(|index|!index.terminal_is_empty()&&index.requires_original_control())}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.index.is_none()}
}
impl<T:RetireOwned+Send+Sync+'static> Drop for NamedPersistentRootPreparation<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"independent named root preparation requires funded original closure");if empty{unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.index);}}}}
#[doc(hidden)]pub struct RootRetirement<T:RetireOwned+Send+Sync+'static>{root:NamedPersistentRoot<T>,retirement:RetainedCloneClose,route:Option<(bool,u128)>}
/// 🚪️ Drives one reply-shaped named close entry as a poll whose pending answer is recovered by the same entry.
pub(super) fn close_via<C:CloseReply>(child:&mut C,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,recover:bool)->OriginalRetirementPoll{crate::native_decoding::owned::native_close_settle(effect,|effect,reply|child.close_reply(original,body,effect,reply,recover))}
type Reply<'r>=crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>;
type Complete<'r>=crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>;
/// 💳️ Settles one close action into a poll after charging exactly the work its children did not record.
fn settle_poll(effect:&mut crate::RetainedAllocationEffect<'_>,work:u128,result:Result<RetainedCloneStep,RetirementPollExit>)->OriginalRetirementPoll{let result=match(result,crate::record_owed(effect,work)){(Ok(step),Ok(()))=>Ok(step),(Err(exit),_)=>Err(exit),(Ok(_),Err(error))=>Err(error.into())};RetirementPollExit::finish(result,work)}
impl<T:RetireOwned+Send+Sync+'static> RetireOwned for NamedPersistentRoot<T>{type RetirementCursor=RootRetirement<T>;fn retirement_seed(self)->Self::RetirementCursor{RootRetirement{root:self,retirement:Default::default(),route:None}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<RootRetirement<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:RetireOwned+Send+Sync+'static> RootRetirement<T>{
 fn drained(&self)->bool{self.root.terminal_is_empty()&&self.retirement.is_empty()}
 fn poll(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let g=effect.grant();let mut work=0u128;let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
 if self.drained(){return Ok(RetainedCloneStep::Complete(Default::default()))}
 let d=self.next_demand(body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
 if !self.retirement.is_empty(){let step=match original{Some(original)=>exit_work!(work,self.retirement.step_original(original,body,effect)),None=>exit_work!(work,self.retirement.step_granted(body,effect))};return Ok(RetainedCloneStep::Progress(step.progress()))}
 if let Some(step)=take_work!(work,self.retirement.begin_granted(&mut self.root.directory,effect)){return Ok(RetainedCloneStep::Progress(step.progress()))}
 if let Some(source)=self.root.source.as_mut(){if source.terminal_is_empty(){*self.root.source=None;work+=1;return Ok(RetainedCloneStep::Progress(paid(d)))}let step=exit_work!(work,source.close_step(body,&mut effect.nested(g)));return Ok(RetainedCloneStep::Progress(step.progress()))}
 Ok(RetainedCloneStep::Complete(Default::default()))
 })();settle_poll(effect,work,result)}
 fn relay<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
 let controlled=original.is_some();let start=match(self.route,recover){(Some((was,start)),true)if was==controlled=>start,(Some(_),_)|(None,true)=>return Err(ValueError::invariant("root retirement changed its original action")),(None,false)=>{let start=effect.performed_work();self.route=Some((controlled,start));start}};
 match self.poll(original,body,effect){RetainedWorkPoll::Pending{..}=>Ok(reply.pending()),RetainedWorkPoll::Complete(turn)=>{self.route=None;let work=effect.performed_work().checked_sub(start).ok_or_else(||ValueError::invariant("root retirement work regressed"))?;Ok(reply.complete(turn.result.map(|step|match step{RetainedCloneStep::Complete(p)if p==Default::default()=>RetirementStep::Complete(work),step=>RetirementStep::Progress(step.progress(),work)})))}}
 }
}
impl<T:RetireOwned+Send+Sync+'static> RetirementCursor for RootRetirement<T>{
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.relay(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.relay(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.relay(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.relay(Some(original),body,effect,reply,true)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty()||self.root.directory.is_some(){return self.retirement.next_owner_work::<SealedShared<Vec<usize>>>(self.root.directory.is_some(),&None,body)}if let Some(source)=self.root.source.as_ref(){return if source.terminal_is_empty(){Ok(1)}else{source.next_close_work_demand(body)}}Ok(0)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{match self.retirement.step_original_birth(original,grant){RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(s)=>RetirementStep::Progress(s.progress(),turn.work),Err(e)=>RetirementStep::Failure(e,turn.work)}}}
 fn requires_original_control(&self)->bool{self.route.map_or_else(||self.retirement.requires_original_control(),|(controlled,_)|controlled)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty()||self.root.directory.is_some(){return self.retirement.next_owner_demands::<SealedShared<Vec<usize>>>(self.root.directory.is_some(),&None,body);}if let Some(source)=self.root.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed(size_of::<RetainedCloneSource<PersistentList<T>>>()*2))}else{source.next_close_demand(body)};}Ok(Default::default())}
 fn terminal_is_empty(&self)->bool{self.drained()&&self.route.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
