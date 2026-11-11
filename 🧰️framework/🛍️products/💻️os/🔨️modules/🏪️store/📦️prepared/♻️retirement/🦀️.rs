//! 📦️ Original unpublished candidates retire their real edit, root, seal and metadata through captured issuers.
use crate::ArtifactStoreOneItemPreparedParts;
use std::sync::Arc;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, ValueError};
use semio_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement;
use crate::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_os_spr::{Edit, HybridLogicalTimestamp};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,RetirementHeaderRelease,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete}};
use semio_framework_value::{FactoryAuthority,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneBirthDemand}};
use std::mem::ManuallyDrop;
use semio_framework_value::{RetainedAllocationEffect,OriginalRetirementPoll,RetainedWorkOutcome,RetainedWorkPoll,retirement::{record_retirement_work,arc_custody::{arc_bytes,original::SharedOriginalStorage}}};
/// 🧳️ An owning cancellation cursor retains every original prepared allocation and declared issuer.
pub struct ArtifactStorePreparedRetirement<P,M>{pub outcome_messages:ManuallyDrop<Option<semio_os_spr::command::MutationMessagePublication>>,pub fold_digest:[u8;32],pub fold_clock:HybridLogicalTimestamp,pub foreign_step_presence:Option<bool>,pub edit:ManuallyDrop<Option<Box<Edit<M>>>>,unboxed:ManuallyDrop<Option<Edit<M>>>,pub post:ManuallyDrop<Option<Arc<P>>>,pub authority:ManuallyDrop<Option<Arc<ArtifactStoreOneItemLiveAuthority>>>,pub strings:ManuallyDrop<[Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>;3]>,pub mutation_factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,pub snapshot_factory:ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,pub active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub factories:ManuallyDrop<[Option<FactoryAuthority>;2]>,ticket:semio_framework_value::FactoryTicketClose,reply:semio_framework_value::ReplyStash<OriginalRetirementFailure>}
impl<P,M> ArtifactStoreOneItemPrepared<P,M>{
 /// 📏️ The exact cursor frame is priced before any original owner moves.
 pub fn retirement_birth_demand()->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:size_of::<ArtifactStorePreparedRetirement<P,M>>(),depth:1}}
}
impl<P:Send+Sync+'static,M:Send+'static> ArtifactStoreOneItemPrepared<P,M>{
 /// 🎟️ Refusal returns the actual candidate and both already captured issuers unchanged.
 pub fn admit_retirement(self,mutation_factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,snapshot_factory:Arc<dyn SnapshotRetirementFactory<P>>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Self,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,Arc<dyn SnapshotRetirementFactory<P>>)>{self.admit_fold_retirement(mutation_factory,snapshot_factory,grant).map(|(owner,receipt)|(owner as Box<dyn ErasedSnapshotRetirement>,receipt))}
  pub fn admit_fold_retirement(self,mutation_factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,snapshot_factory:Arc<dyn SnapshotRetirementFactory<P>>,grant:RetainedCloneGrant)->Result<(Box<ArtifactStorePreparedRetirement<P,M>>,RetainedCloneProgress),(ValueError,Self,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,Arc<dyn SnapshotRetirementFactory<P>>)>{
  let receipt=match Self::retirement_birth_demand().admit(grant){Ok(receipt)=>receipt,Err(error)=>return Err((error,self,mutation_factory,snapshot_factory))};let ArtifactStoreOneItemPreparedParts{outcome_messages,edit,post_snapshot,local_actor,applied_edit_id,tail_edit_id,authority,edit_digest,next_clock,foreign_step_presence}=self.into_parts();let owner=ArtifactStorePreparedRetirement{outcome_messages:ManuallyDrop::new(outcome_messages),fold_digest:edit_digest,fold_clock:next_clock,foreign_step_presence,edit:ManuallyDrop::new(Some(edit)),unboxed:ManuallyDrop::new(None),post:ManuallyDrop::new(Some(post_snapshot)),authority:ManuallyDrop::new(Some(authority)),strings:ManuallyDrop::new([local_actor,Some(applied_edit_id),Some(tail_edit_id)]),mutation_factory:ManuallyDrop::new(Some(mutation_factory)),snapshot_factory:ManuallyDrop::new(Some(snapshot_factory)),active:ManuallyDrop::new(None),factories:ManuallyDrop::new(Default::default()),ticket:Default::default(),reply:Default::default()};Ok((Box::new(owner),receipt))
 }
}
#[cfg(feature = "durable-group-testing")]
impl<P:Send+Sync+'static,M:Send+'static> ArtifactStorePreparedRetirement<P,M> {
 /// 🫴️ Admits the actual returned fixture edit and post with their original issuers before retaining its cursor frame.
 pub fn admit_returned(edit:Edit<M>,post:Arc<P>,mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,snapshots:Arc<dyn SnapshotRetirementFactory<P>>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Edit<M>,Arc<P>,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,Arc<dyn SnapshotRetirementFactory<P>>)> {
  let receipt=match ArtifactStoreOneItemPrepared::<P,M>::retirement_birth_demand().admit(grant){Ok(receipt)=>receipt,Err(error)=>return Err((error,edit,post,mutations,snapshots))};
  Ok((Box::new(Self{outcome_messages:ManuallyDrop::new(None),fold_digest:[0;32],fold_clock:HybridLogicalTimestamp{physical_ms:0,logical:0,actor:0},foreign_step_presence:None,edit:ManuallyDrop::new(None),unboxed:ManuallyDrop::new(Some(edit)),post:ManuallyDrop::new(Some(post)),authority:ManuallyDrop::new(None),strings:ManuallyDrop::new(Default::default()),mutation_factory:ManuallyDrop::new(Some(mutations)),snapshot_factory:ManuallyDrop::new(Some(snapshots)),active:ManuallyDrop::new(None),factories:ManuallyDrop::new(Default::default()),ticket:Default::default(),reply:Default::default()}),receipt))
 }
}
impl<P,M> ArtifactStorePreparedRetirement<P,M>{
 pub fn empty(&self)->bool{self.outcome_messages.is_none()&&self.edit.is_none()&&self.unboxed.is_none()&&self.post.is_none()&&self.authority.is_none()&&self.strings.iter().all(Option::is_none)&&self.mutation_factory.is_none()&&self.snapshot_factory.is_none()&&self.active.is_none()&&self.factories.iter().all(Option::is_none)&&self.ticket.is_empty()}
}
impl<P:Send+Sync+'static,M:Send+'static> ArtifactStorePreparedRetirement<P,M>{
 /// 📐️ Every physical child extent is admitted independently from logical copy work.
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let nested=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared retirement depth overflow"))?;Ok(demand)};
  if let Some(messages)=self.outcome_messages.as_ref(){return nested(messages.close_demands()?);}
  if self.active.is_some(){return nested(self.ticket.next_demand(&self.active,body)?);}
  if self.edit.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Edit<M>>(),release_bytes:size_of::<Edit<M>>(),depth:1,..Default::default()});}
  if self.unboxed.is_some(){return Ok(RetirementDemand{capacity_bytes:size_of::<ArtifactStoreDecodedEditRetirement<M>>(),depth:2,..Default::default()});}
  if self.post.is_some(){return Ok(RetirementDemand{capacity_bytes:size_of::<semio_framework_value::factory::ArcRootRetirement<P>>(),depth:2,..Default::default()});}
  if let Some(string)=self.strings.iter().find_map(Option::as_ref){return nested(semio_framework_value::retirement::controlled::controlled_retirement_birth_demand(string)?);}
  if let Some(authority)=self.authority.as_ref(){let birth=authority.retirement_birth_demand();return Ok(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth+1,..Default::default()});}
  if self.mutation_factory.is_some()||self.snapshot_factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
  self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()),|factory|nested(factory.demands(body)?))
 }
}
impl<P:Send+Sync+'static,M:Send+'static> ArtifactStorePreparedRetirement<P,M>{
 fn stage(&mut self, body: usize, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll { let result = (|| -> Result<RetainedCloneStep, semio_framework_value::RetirementPollExit> { let grant = effect.grant();
  if self.empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let demand=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared retirement exceeds admitted depth").into());}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(messages)=self.outcome_messages.as_mut(){messages.begin_close();let turn=messages.close_step(child);crate::native_source::pay(effect,turn.work)?;let step=turn.result?;if messages.terminal_is_empty(){*self.outcome_messages=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(edit)=self.edit.take(){*self.unboxed=Some(*edit);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
  if let Some(edit)=self.unboxed.take(){let factory=self.mutation_factory.as_ref().expect("prepared original mutation issuer");return match admit_artifact_retirement(edit,child,|original|ArtifactStoreDecodedEditRetirement::new(original,Arc::clone(factory))){Ok((active,receipt))=>{*self.active=Some(active);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*self.unboxed=Some(original);Err(error.into())}};}
  if let Some(post)=self.post.take(){let factory=Arc::clone(self.snapshot_factory.as_ref().ok_or_else(||ValueError::invariant("prepared original root issuer"))?);return match admit_artifact_retirement(post,child,|root|semio_framework_value::factory::ArcRootRetirement::new_snapshot(root,factory)){Ok((active,receipt))=>{*self.active=Some(active);Ok(RetainedCloneStep::Progress(receipt))},Err((error,post))=>{*self.post=Some(post);Err(error.into())}};}
  if let Some(slot)=self.strings.iter_mut().find(|slot|slot.is_some()){let turn=semio_framework_value::retirement::admit_owned_retirement(slot,&mut effect.nested(child));return match turn.result{Ok(Some((owner,receipt)))=>{*self.active=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error.into())};}
  if let Some(authority)=self.authority.take(){return match authority.retire(child) {Ok((active,receipt))=>{*self.active=Some(active);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*self.authority=Some(original);Err(error.into())}};}
  if let Some(factory)=self.mutation_factory.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;self.factories[0]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if let Some(factory)=self.snapshot_factory.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;self.factories[1]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  let slot=self.factories.iter_mut().find(|slot|slot.is_some()).ok_or_else(||ValueError::invariant("prepared original factory close slot is absent"))?;let factory=match slot.as_mut(){Some(v)=>v,None=>return Err(ValueError::invariant("slot absent").into())};let step=semio_framework_value::RetirementPollExit::resume(factory.step(body,&mut effect.nested(child)))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,factory.terminal_is_empty(),"prepared original factory")?;if factory.terminal_is_empty(){*slot=None;}Ok(if self.empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 })(); semio_framework_value::RetirementPollExit::settle(effect, result) }
 /// 🎟️ Drives the selected boxed child through its resident ticket and every other stage through its synchronous step, recording the one reply.
 fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if self.outcome_messages.is_some()||(self.active.is_none()&&self.ticket.is_empty()){
   if recover{return Err(ValueError::invariant("prepared retirement has no original child to recover"))}
   return match self.stage(body,effect){RetainedWorkPoll::Complete(turn)=>Ok(reply.complete(turn.result)),RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("prepared retirement stage left its producer reply pending"))};
  }
  let grant=effect.grant();
  if grant.maximum_items==0||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let depth=grant.maximum_depth.checked_sub(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared retirement child depth absent"))?;
  let remaining=effect.remaining_work_units()?;let mut child=effect.reborrow(RetainedCloneGrant{maximum_depth:depth,..grant},remaining)?;let reply=reply.progress();
  match(original,recover){
   (Some(original),true)=>self.ticket.recover_close_original_into(&mut self.active,original,body,&mut child,reply),
   (Some(original),false)=>self.ticket.close_original_into(&mut self.active,original,body,&mut child,reply),
   (None,true)=>self.ticket.recover_close_into(&mut self.active,body,&mut child,reply),
   (None,false)=>self.ticket.close_into(&mut self.active,body,&mut child,reply),
  }
 }
}
impl<P:Send+Sync+'static,M:Send+'static> ErasedSnapshotRetirement for ArtifactStorePreparedRetirement<P,M>{
 fn native_header_release(&self)->Option<RetirementHeaderRelease>{self.empty().then(||unsafe{RetirementHeaderRelease::native(self,0,0)})}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.outcome_messages.is_none()&&self.ticket.is_empty()&&self.active.as_ref().is_some_and(|child|!child.terminal_is_empty()&&child.requires_original_birth())}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  let Some(depth)=grant.maximum_depth.checked_sub(1)else{return RetainedWorkPoll::Pending{work:0}};
  match self.active.as_mut(){Some(child)=>child.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:depth,..grant}),None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"prepared retirement has no selected original birth").into()),work:0})}
 }
 fn requires_original_control(&self)->bool{self.ticket.pending_original_control().unwrap_or_else(||self.outcome_messages.is_none()&&self.active.as_ref().is_some_and(|child|!child.terminal_is_empty()&&child.requires_original_control()))}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{
  if let Some(messages)=self.outcome_messages.as_ref(){return messages.close_work_demand()}
  if self.active.is_some()||!self.ticket.is_empty(){return self.ticket.next_work(&self.active,body)}
  if self.strings.iter().any(Option::is_some){return Ok(1)}
  if let Some(factory)=self.factories.iter().find_map(Option::as_ref){return factory.next_computational_work(body)}
  Ok(0)
 }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }
 fn resident_reply(&mut self)->Option<&mut semio_framework_value::ReplyStash<OriginalRetirementFailure>>{Some(&mut self.reply)}
 fn close_step(&mut self,_body:usize,_effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"prepared retirement requires its borrowed producer").into()),work:0})}
 fn terminal_is_empty(&self)->bool{self.empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<P,M> Drop for ArtifactStorePreparedRetirement<P,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"prepared retirement retains original owners until funded terminal closure");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.outcome_messages);ManuallyDrop::drop(&mut self.edit);ManuallyDrop::drop(&mut self.unboxed);ManuallyDrop::drop(&mut self.post);ManuallyDrop::drop(&mut self.authority);ManuallyDrop::drop(&mut self.strings);ManuallyDrop::drop(&mut self.mutation_factory);ManuallyDrop::drop(&mut self.snapshot_factory);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.factories);}}}}

