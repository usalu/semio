//! 🗑️ Original typed Store effects prepare their custody before the Session publishes Discard.
use super::*;
use std::mem::{ManuallyDrop,size_of,size_of_val};
use semio_framework_value::{ErasedSnapshotRetirement,FactoryAuthority,ArtifactOwnedValueRetirementFactory,retirement::{RetireOwned,controlled::ControlledRetirement}};

pub(super) struct TimeTravelDiscardStoreEffects<P,Mu:protocol::Mutation<P>> {
 residue:ManuallyDrop<Option<TimeTravelStoreState<P,Mu>>>,
 frames:ManuallyDrop<[Option<store::PreparedHistoryReadRetirement<P,Mu>>;5]>,
 input:Option<ControlledRetirement<protocol::InputReplacement>>,
 editor:Option<ControlledRetirement<TimeTravelEditor>>,
 factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>>>,
 factory_close:Option<FactoryAuthority>,
 bound:(usize,u64,[u8;32],TimeTravelStage),
 prepared:usize,
 pub(super) closing:bool,
}
fn refusal(message:&'static str)->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message)}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn admits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn demand(owner:&dyn store::ErasedSnapshotRetirement,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||refusal("Discard original child depth overflow"))?})}
impl<P,Mu> TimeTravelStoreState<P,Mu> where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+Send+Sync+'static,Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>+protocol::OpBinary+protocol::OpText+Send+'static {
 pub(super) fn prepare_discard_effects(&mut self,store:&ArtifactStore<P,Mu>,stage:TimeTravelStage,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
  if !TimeTravelEditor::controlled_retirement_supported()||!protocol::InputReplacement::controlled_retirement_supported(){return Err(refusal("Discard original editor schema ownership is unsupported"))}
  let bound=(self as *const _ as usize,store.generation_now(),store.content_revision_now(),stage);
  if self.discard_effects.is_none(){let layout=std::alloc::Layout::new::<TimeTravelDiscardStoreEffects<P,Mu>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<layout.size(){return Ok((false,Default::default()))}let factory=store.owned_mutation_retirement_factory()?;let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<TimeTravelDiscardStoreEffects<P,Mu>>())else{return Err(refusal("Discard effect frame allocation failed"))};let owner=TimeTravelDiscardStoreEffects{residue:ManuallyDrop::new(None),frames:ManuallyDrop::new(std::array::from_fn(|_|None)),input:None,editor:None,factory:ManuallyDrop::new(Some(Arc::clone(factory))),factory_close:None,bound,prepared:0,closing:false};self.discard_effects=Some(unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())});return Ok((false,RetainedCloneProgress{retained_capacity_bytes:layout.size(),..item()}))}
  let owner=self.discard_effects.as_mut().unwrap();if owner.bound!=bound||owner.closing{return Err(refusal("Discard prepared original Store authority changed"))}
  if owner.prepared<owner.frames.len(){if grant.maximum_depth<2{return Ok((false,Default::default()))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let Some((frame,receipt))=store.prepare_history_read_retirement(child)?else{return Ok((false,Default::default()))};owner.frames[owner.prepared]=Some(frame);owner.prepared+=1;return Ok((false,receipt))}
  if self.preview_job.as_ref().is_some_and(|original|!owner.frames[0].as_ref().unwrap().accepts_preview(original))||self.replay.as_ref().is_some_and(|original|!owner.frames[1].as_ref().unwrap().accepts_replay(original))||self.preview.as_ref().is_some_and(|original|!owner.frames[3].as_ref().unwrap().accepts_derived_snapshot(original))||self.head.as_ref().is_some_and(|original|!owner.frames[4].as_ref().unwrap().accepts_derived_snapshot(original)){return Err(refusal("Discard prepared effects refuse foreign original derivation authority"))}Ok((true,Default::default()))
 }
 pub(super) fn commit_discard_effects(&mut self,stage:TimeTravelStage){
  let mut frame=self.discard_effects.take().expect("validated original Discard effects");assert_eq!(frame.prepared,5);assert_eq!(frame.bound.3,stage);let label_of=self.label_of;let mut original=std::mem::replace(self,Self::new(label_of));if stage==TimeTravelStage::Reviewing{self.finished=original.finished.take();self.head=original.head.take()}*frame.residue=Some(original);frame.closing=true;self.discard_effects=Some(frame);
 }
 pub(super) fn publish_discard(&mut self,store:&ArtifactStore<P,Mu>,cursor:&mut semio_framework_time_travel::TimeTravelDiscardCursor,session:&mut TimeTravelSession,editor:&mut Option<TimeTravelEditor>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if session.base.content_revision!=store.content_revision_now(){return Err(refusal("Discard original Store base changed before publication"))}
  let stage=cursor.prepared_stage().ok_or_else(||refusal("Discard prospective original stage is absent"))?;let (ready,receipt)=self.prepare_discard_effects(store,stage,grant)?;if !ready{return Ok(receipt)}
  let parent_copy=size_of::<Self>()+size_of::<Option<Self>>()+size_of::<Option<Box<TimeTravelDiscardStoreEffects<P,Mu>>>>()+size_of::<bool>()+if editor.is_some(){size_of::<Option<TimeTravelEditor>>()+size_of::<Option<ControlledRetirement<TimeTravelEditor>>>()}else{0}+if stage==TimeTravelStage::Reviewing{size_of_val(&self.finished)+size_of_val(&self.head)}else{0};let child_demand=cursor.commit_demands(session)?;let copied_bytes=parent_copy.checked_add(child_demand.copy_bytes).ok_or_else(||refusal("Discard original publication header overflow"))?;let depth=child_demand.depth.checked_add(1).ok_or_else(||refusal("Discard original publication depth overflow"))?;if grant.maximum_items==0||grant.maximum_depth<depth||grant.maximum_copy_bytes<copied_bytes||grant.maximum_capacity_bytes<child_demand.capacity_bytes||grant.maximum_release_bytes<child_demand.release_bytes{return Ok(Default::default())}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=cursor.commit(session,child)?;let mut receipt=step.progress();if !receipt.fits(child){return Err(refusal("Discard original Session commit exceeded its caller"))}if receipt==Default::default(){return Ok(receipt)}self.commit_discard_effects(stage);if let Some(original)=editor.take(){self.discard_effects.as_mut().unwrap().editor=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original editor authority preflight")))}receipt.copied_bytes+=parent_copy;Ok(receipt)
 }
 pub(super) fn discard_effects_retirement_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{self.discard_effects.as_ref().filter(|owner|owner.closing).map(|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{demand(owner.as_ref(),body)}).transpose()}
 pub(super) fn discard_effects_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{let Some(quote)=self.discard_effects_retirement_demands(grant.maximum_copy_bytes)?else{return Ok(None)};if !admits(grant,quote){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}let owner=self.discard_effects.as_mut().unwrap();if owner.terminal_is_empty(){drop(self.discard_effects.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:quote.release_bytes,..item()})))}owner.closing=true;let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.close_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"Actor original prepared Discard effects")?;Ok(Some(RetainedCloneStep::Progress(step.progress())))}
}
impl<P,Mu> store::ErasedSnapshotRetirement for TimeTravelDiscardStoreEffects<P,Mu> where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+Send+Sync+'static,Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>+protocol::OpBinary+protocol::OpText+Send+'static {
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !admits(grant,self.next_demand(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}self.closing=true;let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(original)=self.residue.as_mut(){
   let receipt=if original.preview_job.is_some(){self.frames[0].as_mut().unwrap().install_preview(&mut original.preview_job,child)?}else if original.replay.is_some(){self.frames[1].as_mut().unwrap().install_replay(&mut original.replay,child)?}else if original.finished.is_some(){self.frames[2].as_mut().unwrap().install_finished(&mut original.finished,child)?}else if original.preview.is_some(){self.frames[3].as_mut().unwrap().install_derived_snapshot(&mut original.preview,child)?}else if original.head.is_some(){self.frames[4].as_mut().unwrap().install_derived_snapshot(&mut original.head,child)?}else{None};if let Some(receipt)=receipt{return Ok(RetainedCloneStep::Progress(receipt))}
   if let Some(input)=original.preview_input.take(){self.input=Some(ControlledRetirement::new(input).unwrap_or_else(|_|unreachable!("prepared original InputReplacement authority")));return Ok(RetainedCloneStep::Progress(item()))}
   if original.discarded_factory.is_none()&&(original.kind.is_some()||original.staged.is_some()||!original.discarded.is_empty()||original.discarded_pending.is_some()){original.discarded_factory=Some(Arc::clone(self.factory.as_ref().unwrap()));return Ok(RetainedCloneStep::Progress(item()))}
   if original.discarded_pending.is_none()&&(original.kind.is_some()||original.staged.is_some()){original.discarded_pending=original.staged.take().or_else(||original.kind.take());return Ok(RetainedCloneStep::Progress(item()))}
   if let Some(step)=original.original_retirement_step(child)?{return Ok(RetainedCloneStep::Progress(step.progress()))}
   if original.terminal_is_empty(){drop(self.residue.take());return Ok(RetainedCloneStep::Progress(item()))}
   return Err(refusal("Discard original Store residue did not reach its exact witness"))
  }
  if let Some(editor)=self.editor.as_mut(){if editor.terminal_is_empty(){drop(self.editor.take());return Ok(RetainedCloneStep::Progress(item()))}let step=editor.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,editor.terminal_is_empty(),"Discard original native editor")?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(input)=self.input.as_mut(){if input.terminal_is_empty(){drop(self.input.take());return Ok(RetainedCloneStep::Progress(item()))}let step=input.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,input.terminal_is_empty(),"Discard original preview input")?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(slot)=self.frames.iter_mut().find(|slot|slot.is_some()){let owner=slot.as_mut().unwrap();if owner.terminal_is_empty(){drop(slot.take());return Ok(RetainedCloneStep::Progress(item()))}let step=owner.close_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"Discard prepared original history effect")?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(item()))}
  if let Some(owner)=self.factory_close.as_mut(){if owner.terminal_is_empty(){drop(self.factory_close.take());return Ok(RetainedCloneStep::Progress(item()))}let step=owner.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"Discard exact original mutation issuer")?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.next_demand(body).map(|d|d.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.depth)}
 fn terminal_is_empty(&self)->bool{self.residue.is_none()&&self.input.is_none()&&self.editor.is_none()&&self.frames.iter().all(Option::is_none)&&self.factory.is_none()&&self.factory_close.is_none()}
}
impl<P,Mu> TimeTravelDiscardStoreEffects<P,Mu> where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+Send+Sync+'static,Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>+protocol::OpBinary+protocol::OpText+Send+'static {
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(original)=self.residue.as_ref(){if original.preview_job.is_some()||original.replay.is_some()||original.finished.is_some()||original.preview.is_some()||original.head.is_some(){return Ok(RetirementDemand{depth:2,..Default::default()})}if original.preview_input.is_some()||original.discarded_factory.is_none()&&(original.kind.is_some()||original.staged.is_some()||!original.discarded.is_empty()||original.discarded_pending.is_some())||original.discarded_pending.is_none()&&(original.kind.is_some()||original.staged.is_some()){return Ok(RetirementDemand{depth:1,..Default::default()})}let mut demand=original.original_retirement_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||refusal("Discard original typed residue depth overflow"))?;return Ok(demand)}
  if let Some(owner)=self.editor.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{demand(owner,body)}}
  if let Some(owner)=self.input.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{demand(owner,body)}}
  if let Some(owner)=self.frames.iter().find_map(|slot|slot.as_ref()){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{demand(owner,body)}}
  if self.factory.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}if let Some(owner)=self.factory_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{let mut quote=owner.demands(body)?;quote.depth=quote.depth.checked_add(1).ok_or_else(||refusal("Discard factory alias depth overflow"))?;Ok(quote)}}Ok(Default::default())
 }
}
impl<P,Mu:protocol::Mutation<P>> Drop for TimeTravelDiscardStoreEffects<P,Mu>{fn drop(&mut self){let empty=self.residue.is_none()&&self.frames.iter().all(Option::is_none)&&self.input.is_none()&&self.editor.is_none()&&self.factory.is_none()&&self.factory_close.is_none();assert!(std::thread::panicking()||empty,"Discard effect custody abandoned original Store owners");if empty{unsafe{ManuallyDrop::drop(&mut self.residue);ManuallyDrop::drop(&mut self.frames);ManuallyDrop::drop(&mut self.factory)}}}}

pub(super) struct DiscardMemberPublication<'a>{pub owners:&'a mut dyn TimeTravelOwners,pub cursor:&'a mut semio_framework_time_travel::TimeTravelDiscardCursor,pub session:&'a mut TimeTravelSession,pub editor:&'a mut Option<TimeTravelEditor>,pub review_children:&'a mut Option<TimeTravelReviewChildren>,pub children:&'a mut Option<ChildContentView>,pub retirements:&'a mut ArtifactFixedRegistry<ChildContentRetirement>,pub child_generation:&'a mut u64,pub view_copy_bytes:usize,pub grant:RetainedCloneGrant}
impl store::MemberStoreVisitor for DiscardMemberPublication<'_>{
 type Output=Result<RetainedCloneProgress,ValueError>;
 fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>+protocol::SemanticMutation<P>+protocol::OpBinary+protocol::OpText+Send+'static {
  let state=self.owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P,Mu>>().ok_or_else(||refusal("Discard original member Store kind changed"))?;
  if self.cursor.prepared_stage()==Some(TimeTravelStage::Reviewing){
   let review=self.review_children.as_ref().ok_or_else(||refusal("Discard original reviewing child custody is absent"))?;if review.base!=self.session.base{return Err(refusal("Discard original reviewed child base changed"))}
   let generation=self.child_generation.checked_add(1).filter(|generation|self.retirements.can_insert(*generation));
   if self.grant.maximum_items==0||self.grant.maximum_depth<2||self.grant.maximum_copy_bytes<self.view_copy_bytes||generation.is_none(){return Ok(Default::default())}
   let child=RetainedCloneGrant{maximum_copy_bytes:self.grant.maximum_copy_bytes-self.view_copy_bytes,..self.grant};let receipt=state.publish_discard(store,self.cursor,self.session,self.editor,child)?;
   if self.session.stage!=TimeTravelStage::Reviewing{return Ok(receipt)}
   let review=self.review_children.take().expect("original reviewed view was preflighted");if let Some(previous)=self.children.replace(review.view){let generation=generation.unwrap();self.retirements.insert_admitted(generation,ChildContentRetirement::new(previous,false));*self.child_generation=generation}
   return Ok(RetainedCloneProgress{copied_bytes:receipt.copied_bytes+self.view_copy_bytes,..receipt})
  }
  state.publish_discard(store,self.cursor,self.session,self.editor,self.grant)
 }
}
