//! 🧬️ Genuine retained decision, central preflight, inverse and capability commit share one original issuer.
use super::{PagedOneItemEdit,PagedOneItemEditStep,PagedOneItemPreparationFactory};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_os_store_one_item_publication::{ArtifactStoreOneItemPreparationFactory,native_source::pay};
use semio_os_spr::{RetainedMutation,RetainedMutationOwner,RetainedDiffOwner,RetainedMutationDecisionStep,RetainedDiffPreflightStep,RetainedMutationInverseStep,RetainedMutationCloseContext,retained_mutation_birth_demand,RetainedDiffAdmission,begin_retained_mutation,decide_retained_mutation_into,recover_decide_retained_mutation_into,begin_retained_diff,preflight_retained_diff_into,recover_preflight_retained_diff_into,inverse_retained_mutation_into,recover_inverse_retained_mutation_into,commit_retained_diff};
use protocol::retained::retained_mutation_birth_work;
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,RetainedCloneSource,RetainedCloneRef,native_decoding::{NativeReceivingEffect,NativeDecodeControl,NativeReceivingEffectPoll,NativeReceivingEffectError},retained_clone::{OriginalEffectAction,OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,RetainedCloneSourceAdmission,admit_retained_clone_progress}};
use semio_framework_value_factory::{ArtifactOwnedValueRetirementFactory,vector::FactoryVectorRetirement};
use std::{mem::{size_of,ManuallyDrop},sync::Arc};
use protocol::MutationMessageConcat;
/// ⛽️ The work ceiling one retained edit turn quotes; a quote beyond the Job's fuel is refused and the turn makes no progress.
const EDIT_TURN_WORK:u64=1<<12;
fn fault(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn perform(effect:&mut RetainedAllocationEffect<'_>,units:u64)->Result<(),ValueError>{if effect.performed(units)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained mutation action exceeds its work ceiling"))}}
fn once<R>(last_work:&mut u64,receiving:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,quote:u64,run:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->R)->Result<Option<R>,ValueError>{
 Ok(receiving.with_effect(grant,quote,|control,effect|{let result=run(control,effect);(result,effect.performed_work())})?.map(|(result,work)|{*last_work=last_work.saturating_add(u64::try_from(work).unwrap_or(u64::MAX));result}))
}
fn framed<E:NativeReceivingEffectError>(last_work:&mut u64,poll:NativeReceivingEffectPoll<E>,primary:impl FnOnce(semio_framework_value::native_decoding::NativeReceivingEffectFailure<E>)->semio_framework_value::OriginalReceivingPrimary)->Result<RetainedCloneProgress,semio_framework_value::OriginalReceivingPrimary>{
 match poll{
  NativeReceivingEffectPoll::Pending{work}=>{*last_work=last_work.saturating_add(u64::try_from(work).unwrap_or(u64::MAX));Ok(Default::default())}
  NativeReceivingEffectPoll::Complete(turn)=>{*last_work=last_work.saturating_add(u64::try_from(turn.work).unwrap_or(u64::MAX));turn.result.map(|step|step.progress()).map_err(primary)}
 }
}
/// 🛰️ Borrows exactly the owners and sources one original semantic action reads.
enum Action<'t,S:SnapshotProjection,M:RetainedMutation<S>>{
 Decide{owner:&'t mut RetainedMutationOwner<S,M>,mutation:RetainedCloneRef<'t,M>,before:RetainedCloneRef<'t,S>,slot:&'t mut Option<RetainedMutationDecisionStep<M::Diff>>},
 Inverse{owner:&'t mut RetainedMutationOwner<S,M>,mutation:RetainedCloneRef<'t,M>,admission:RetainedDiffAdmission<'t,S,M::Diff>,slot:&'t mut Option<RetainedMutationInverseStep<M>>},
 Preflight{plan:&'t mut RetainedDiffOwner<S,M::Diff>,diff:RetainedCloneRef<'t,M::Diff>,before:RetainedCloneRef<'t,S>,post:RetainedCloneRef<'t,S>,slot:&'t mut Option<RetainedDiffPreflightStep>},
 CloseSemantics{owner:&'t mut RetainedMutationOwner<S,M>,context:RetainedMutationCloseContext<'t,M>},
 ClosePlan{plan:&'t mut RetainedDiffOwner<S,M::Diff>},
}
/// 🛰️ Lends one turn's request to the resident producer; its results stay in the edit's own slots.
pub struct Call<'t,'s,S:SnapshotProjection,M:RetainedMutation<S>>{body:usize,maximum:u64,control:Option<&'t mut NativeDecodeControl<'s>>,action:Action<'t,S,M>}
/// 🏭️ Runs each pending-capable semantic child as an original action whose reply is recovered on later turns.
pub struct ApplyProducer<S,M>(std::marker::PhantomData<fn()->(S,M)>);
impl<S,M> Default for ApplyProducer<S,M>{fn default()->Self{Self(std::marker::PhantomData)}}
fn recorded<'r,T>(result:Option<Result<T,OriginalRetirementFailure>>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>,keep:impl FnOnce(T)->RetainedCloneProgress)->OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>{
 match result{None=>reply.pending(),Some(Ok(step))=>reply.complete(Ok(RetainedCloneStep::Progress(keep(step)))),Some(Err(failure))=>reply.complete(Err(failure))}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> ApplyProducer<S,M>{
 fn run<'r>(call:&mut Call<'_,'_,S,M>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let(body,maximum)=(call.body,call.maximum);let control=call.control.as_deref_mut();
  match &mut call.action{
   Action::Decide{owner,mutation,before,slot}=>{
    let mut stash=None;let inner=OriginalEffectReply::borrow(&mut stash)?;
    drop(if recover{recover_decide_retained_mutation_into(owner,*mutation,*before,body,maximum,effect,inner)}else{decide_retained_mutation_into(owner,*mutation,*before,body,maximum,effect,inner)}?);
    Ok(recorded(stash.take(),reply,|step|{let progress=step.progress();if !matches!(&step,RetainedMutationDecisionStep::Progress(_)){**slot=Some(step);}progress}))
   }
   Action::Inverse{owner,mutation,admission,slot}=>{
    let mut stash=None;let inner=OriginalEffectReply::borrow(&mut stash)?;
    drop(if recover{recover_inverse_retained_mutation_into(owner,*mutation,admission,body,maximum,effect,inner)}else{inverse_retained_mutation_into(owner,*mutation,admission,body,maximum,effect,inner)}?);
    Ok(recorded(stash.take(),reply,|step|{let progress=step.progress();if !matches!(&step,RetainedMutationInverseStep::Progress(_)){**slot=Some(step);}progress}))
   }
   Action::CloseSemantics{owner,context}=>{
    let mut stash=None;let inner=OriginalEffectReply::borrow(&mut stash)?;
    drop(if recover{owner.recover_close_into(*context,body,effect,inner)}else{owner.close_into(*context,body,effect,inner)}?);
    Ok(recorded(stash.take(),reply,|step|step.progress()))
   }
   Action::ClosePlan{plan}=>{
    let mut stash=None;let inner=OriginalEffectReply::borrow(&mut stash)?;
    drop(if recover{plan.recover_close_into(body,effect,inner)}else{plan.close_into(body,effect,inner)}?);
    Ok(recorded(stash.take(),reply,|step|step.progress()))
   }
   Action::Preflight{plan,diff,before,post,slot}=>{
    let mut stash=None;let inner=OriginalEffectReply::borrow(&mut stash)?;
    let control=control.ok_or_else(||fault("retained preflight requires its receiving control"))?;
    drop(if recover{recover_preflight_retained_diff_into(plan,*diff,*before,*post,body,maximum,effect,control,inner)}else{preflight_retained_diff_into(plan,*diff,*before,*post,body,maximum,effect,control,inner)}?);
    Ok(recorded(stash.take(),reply,|step|{let progress=match &step{RetainedDiffPreflightStep::Progress(p)|RetainedDiffPreflightStep::Admitted(p,_)|RetainedDiffPreflightStep::Refused(p,_)=>*p};if !matches!(&step,RetainedDiffPreflightStep::Progress(_)){**slot=Some(step);}progress}))
   }
  }
 }
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> OriginalEffectProducer for ApplyProducer<S,M>{
 type Output=RetainedCloneStep;
 type Failure=OriginalRetirementFailure;
 type Context<'turn,'source>=Call<'turn,'source,S,M> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'r,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{Self::run(context,effect,reply,false)}
 fn recover<'r,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{Self::run(context,effect,reply,true)}
}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Stage{Birth,Decide,Source,Plan,Preflight,Inverse,Commit,Join,Publish,Done}

/// 🧬️ Owns original retained semantics, the admitted central diff plan and separately funded publication rows.
pub struct MutationApplyEdit<S:SnapshotProjection,M:RetainedMutation<S>>{
 pub stage:Stage,
 semantics:ManuallyDrop<Option<RetainedMutationOwner<S,M>>>,
 birth_effect:semio_framework_value::native_decoding::NativeReceivingEffect,
 semantic_effect:semio_framework_value::native_decoding::NativeReceivingEffect<semio_framework_value::OriginalRetirementFailure>,
 decision_result:ManuallyDrop<Option<RetainedMutationDecisionStep<M::Diff>>>,
 inverse_result:ManuallyDrop<Option<RetainedMutationInverseStep<M>>>,
 preflight_result:ManuallyDrop<Option<RetainedDiffPreflightStep>>,
 authority:Option<OriginalEffectAuthority<ApplyProducer<S,M>>>,
 pending_work:u64,
 close_action:Option<OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>,
 diff:ManuallyDrop<Option<M::Diff>>,
 source:ManuallyDrop<Option<RetainedCloneSource<M::Diff>>>,
 admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<M::Diff,M::Diff,()>>>,
 plan:ManuallyDrop<Option<RetainedDiffOwner<S,M::Diff>>>,
 inverse:ManuallyDrop<Option<Vec<M>>>,
 inverse_close:Option<FactoryVectorRetirement<M>>,
 messages:ManuallyDrop<[Option<Vec<protocol::MutationMessage>>;2]>,
 join:ManuallyDrop<Option<MutationMessageConcat<2>>>,
 retirement:RetainedCloneClose,
 refused:bool,
 closing:bool,
 last_work:u64,
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> Default for MutationApplyEdit<S,M>{fn default()->Self{Self{stage:Stage::Birth,semantics:ManuallyDrop::new(None),birth_effect:Default::default(),semantic_effect:Default::default(),decision_result:ManuallyDrop::new(None),inverse_result:ManuallyDrop::new(None),preflight_result:ManuallyDrop::new(None),authority:Some(OriginalEffectAuthority::new(Default::default())),pending_work:0,close_action:None,diff:ManuallyDrop::new(None),source:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),plan:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),inverse_close:None,messages:ManuallyDrop::new([None,None]),join:ManuallyDrop::new(None),retirement:Default::default(),refused:false,closing:false,last_work:0}}}
impl<S:SnapshotProjection,M:RetainedMutation<S>> MutationApplyEdit<S,M>{
 fn context(mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)->RetainedMutationCloseContext<'_,M>{RetainedMutationCloseContext{mutations}}
 fn parent_copy()->usize{size_of::<Stage>()}
 fn handoff(&self)->bool{self.semantic_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&(self.stage==Stage::Decide&&self.decision_result.is_some()||self.stage==Stage::Inverse&&self.inverse_result.is_some()||self.stage==Stage::Preflight&&self.preflight_result.is_some())}
 fn close_child(authority:&mut Option<OriginalEffectAuthority<ApplyProducer<S,M>>>,slot:&mut Option<OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>,effect:&mut RetainedAllocationEffect<'_>,body:usize,action:Action<'_,S,M>)->Result<RetainedCloneProgress,RetirementPollExit>{
  let authority=authority.as_mut().ok_or_else(||fault("retained edit authority is absent"))?;
  if !authority.drive(&mut Call{body,maximum:0,control:None,action},effect,slot)?{return Err(fault("retained child close left its reply pending").into())}
  let mut output=None;slot.as_mut().ok_or_else(||fault("retained child close action is absent"))?.transfer_reply(&mut output)?;authority.finish(slot)?;
  match output{Some(Ok(step))=>Ok(step.progress()),Some(Err(failure))=>Err(RetirementPollExit::Failed(failure)),None=>Err(fault("retained child close recorded no reply").into())}
 }
 fn drive(last_work:&mut u64,receiving:&mut NativeDecodeControl<'_>,frame:&mut NativeReceivingEffect<OriginalRetirementFailure>,authority:&mut OriginalEffectAuthority<ApplyProducer<S,M>>,demand:RetirementDemand,work:u64,body:usize,action:Action<'_,S,M>)->Result<RetainedCloneProgress,semio_framework_value::OriginalReceivingPrimary>{
  let poll=receiving.receive_original_effect(demand,work,frame,authority,|control,effect,authority,slot|authority.drive(&mut Call{body,maximum:work,control:Some(control),action},effect,slot))?;
  framed(last_work,poll,|failure|{let mut receiving=failure.receiving;receiving.retirement_failure=failure.child;receiving.work_refusal=failure.work_refusal;receiving})
 }
 fn progress(grant:RetainedCloneGrant,mut progress:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{if progress!=Default::default(){progress.copied_bytes=progress.copied_bytes.checked_add(Self::parent_copy()).ok_or_else(||fault("retained semantic receipt overflow"))?;}admit_retained_clone_progress(grant,progress,"original structural semantic parent")}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> PagedOneItemEdit<S,M> for MutationApplyEdit<S,M>{
 const PREFIX:&'static str="mutation-apply";
 fn recognizes(_:&M)->bool{true}
 fn preflight(_:&M)->Result<usize,String>{if !M::Diff::controlled_retirement_supported(){return Err("mutation-apply-diff-retirement-unsupported".into());}Ok(size_of::<M>())}
 fn post_mode(&self)->PagedOneItemPostMode{if self.stage==Stage::Preflight{PagedOneItemPostMode::Original}else{PagedOneItemPostMode::Mutable}}
 fn advance_demands(&self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:PagedOneItemPostRead<'_,S>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.stage==Stage::Done{return Err(fault("retained mutation edit is closed"));}
  let parent=Self::parent_copy();
  let mut demand=if self.handoff(){Default::default()}else if let Some(pending)=self.semantic_effect.pending_demand(){pending}else{match self.stage{
   Stage::Birth=>retained_mutation_birth_demand::<S,M>(),
   Stage::Decide=>self.semantics.as_ref().unwrap().decision_demand(mutation,before,body,maximum_work_units)?,
   Stage::Source=>self.admission.as_ref().ok_or_else(||fault("retained mutation diff admission is absent"))?.next_demand()?,
   Stage::Plan=>semio_os_spr::retained_diff_birth_demand::<S,M::Diff>(),
   Stage::Preflight=>self.plan.as_ref().unwrap().preflight_demand(self.source.as_ref().unwrap().try_borrow()?,before,post.original()?,body,maximum_work_units)?,
   Stage::Inverse=>{let a=self.plan.as_ref().unwrap().inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)?;self.semantics.as_ref().unwrap().inverse_demand(mutation,&a,body,maximum_work_units)?},
   Stage::Commit=>self.plan.as_ref().unwrap().commit_demand(self.source.as_ref().unwrap().try_borrow()?,before,post.get(),body)?,
   Stage::Join=>if let Some(join)=self.join.as_ref(){join.demand()?}else{MutationMessageConcat::<2>::constructor_demand()},
   Stage::Publish=>self.join.as_ref().unwrap().demand()?,Stage::Done=>unreachable!()
  }};demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||fault("retained semantic header quote overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||fault("retained semantic depth quote overflow"))?.max(2);Ok(demand)
 }
 fn advance(&mut self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:PagedOneItemPost<'_,S>,body:usize,grant:RetainedCloneGrant,receiving:&mut NativeDecodeControl<'_>)->Result<PagedOneItemEditStep<M>,semio_framework_value::OriginalReceivingPrimary>{
  self.last_work=0;
  if self.closing||self.stage==Stage::Done{return Err(fault("retained mutation edit is closed").into());}
  if grant.maximum_items==0||grant.maximum_copy_bytes<Self::parent_copy()||grant.maximum_depth<2{return Ok(PagedOneItemEditStep::Progress(Default::default()));}
  let recovering=self.semantic_effect.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty;
  let handoff=self.handoff();
  let demand=if handoff{scalar(Self::parent_copy())}else{self.advance_demands(before,mutation,post.read(),body,EDIT_TURN_WORK)?};if !semio_os_spr::retained_mutation_demand_fits(demand,grant){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
  let birth_handoff=self.stage==Stage::Birth&&self.semantics.is_some()&&self.birth_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty;
  let g=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::parent_copy(),maximum_depth:grant.maximum_depth-1,..grant};
  let previous=self.stage;
  let mut p=match self.stage{
   Stage::Birth=>{
    if birth_handoff{
     if once(&mut self.last_work,receiving,g,1,|_,effect|perform(effect,1))?.transpose()?.is_none(){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
     self.stage=Stage::Decide;RetainedCloneProgress::default()
    }else{
     let d=retained_mutation_birth_demand::<S,M>();let q=retained_mutation_birth_work::<S,M>()?;let slot=&mut self.semantics;
     let poll=receiving.receive_effect(d,q,&mut self.birth_effect,|_,effect|{let maximum=effect.maximum_work_units();let turn=begin_retained_mutation::<S,M>(maximum,effect);RetainedWorkOutcome{result:turn.result.map(|value|match value{Some((owner,p))=>{**slot=Some(owner);RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())}),work:turn.work}})?;
     framed(&mut self.last_work,poll,|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child;receiving.work_refusal=failure.work_refusal;receiving})?
    }
   }
   Stage::Decide=>{
    if handoff{
     if once(&mut self.last_work,receiving,g,1,|_,effect|perform(effect,1))?.transpose()?.is_none(){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
     match self.decision_result.take().ok_or_else(||fault("retained decision handoff is absent"))?{RetainedMutationDecisionStep::Admitted(_,d,rows)=>{*self.admission=Some(RetainedCloneSource::admission_owned(d,()));self.messages[0]=Some(rows);self.stage=Stage::Source;},RetainedMutationDecisionStep::Refused(_,rows)=>{self.messages[0]=Some(rows);self.refused=true;self.stage=Stage::Join;},RetainedMutationDecisionStep::Progress(_)=>return Err(fault("retained decision handoff was not terminal").into())}RetainedCloneProgress::default()
    }else{
     let owner=self.semantics.as_mut().ok_or_else(||fault("semantics is absent"))?;
     let(d,q)=if recovering{(self.semantic_effect.pending_demand().ok_or_else(||fault("retained decision recovery lost its demand"))?,self.pending_work)}else{(owner.decision_demand(mutation,before,body,EDIT_TURN_WORK)?,owner.decision_work_demand(mutation,before,body,EDIT_TURN_WORK)?)};
     self.pending_work=q;
     Self::drive(&mut self.last_work,receiving,&mut self.semantic_effect,self.authority.as_mut().ok_or_else(||fault("retained edit authority is absent"))?,d,q,body,Action::Decide{owner,mutation,before,slot:&mut self.decision_result})?
    }
   }
   Stage::Source=>{
    let admission=self.admission.as_mut().ok_or_else(||fault("retained mutation diff admission is absent"))?;
    if admission.phase()!="ready"{
     let quote=admission.advance_work_demand()?;
     let Some(step)=once(&mut self.last_work,receiving,g,quote,|_,effect|admission.advance(effect).result)?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
     step?.progress()
    }else{
     let mut output=None;
     let Some(step)=once(&mut self.last_work,receiving,g,1,|_,effect|{admission.take_output(&mut output,effect).result})?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
     let step=step?;
     if let Some(source)=output{*self.source=Some(source);self.admission.take();self.stage=Stage::Plan;}
     step.progress()
    }
   }
   Stage::Plan=>{
    let Some(begun)=once(&mut self.last_work,receiving,g,1,|_,effect|perform(effect,1).map(|()|begin_retained_diff::<S,M::Diff>(g)))?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
    let Some((o,p))=begun?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};*self.plan=Some(o);self.stage=Stage::Preflight;p
   }
   Stage::Preflight=>{
    if handoff{
     if once(&mut self.last_work,receiving,g,1,|_,effect|perform(effect,1))?.transpose()?.is_none(){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
     match self.preflight_result.take().ok_or_else(||fault("retained preflight handoff is absent"))?{RetainedDiffPreflightStep::Admitted(_,rows)=>{self.messages[1]=Some(rows);self.stage=Stage::Inverse;},RetainedDiffPreflightStep::Refused(_,rows)=>{self.messages[1]=Some(rows);self.refused=true;self.stage=Stage::Join;},RetainedDiffPreflightStep::Progress(_)=>return Err(fault("retained preflight handoff was not terminal").into())}RetainedCloneProgress::default()
    }else{
     let plan=self.plan.as_mut().unwrap();let diff=self.source.as_ref().unwrap().try_borrow()?;let original=post.original()?;
     let(d,q)=if recovering{(self.semantic_effect.pending_demand().ok_or_else(||fault("retained preflight recovery lost its demand"))?,self.pending_work)}else{(plan.preflight_demand(diff,before,original,body,EDIT_TURN_WORK)?,plan.preflight_work_demand(diff,before,original,body,EDIT_TURN_WORK)?)};
     self.pending_work=q;
     Self::drive(&mut self.last_work,receiving,&mut self.semantic_effect,self.authority.as_mut().ok_or_else(||fault("retained edit authority is absent"))?,d,q,body,Action::Preflight{plan,diff,before,post:original,slot:&mut self.preflight_result})?
    }
   }
   Stage::Inverse=>{
    if handoff{
     if once(&mut self.last_work,receiving,g,1,|_,effect|perform(effect,1))?.transpose()?.is_none(){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
     match self.inverse_result.take().ok_or_else(||fault("retained inverse handoff is absent"))?{RetainedMutationInverseStep::Ready(_,rows)=>{*self.inverse=Some(rows);self.stage=Stage::Commit;},RetainedMutationInverseStep::Progress(_)=>return Err(fault("retained inverse handoff was not terminal").into())}RetainedCloneProgress::default()
    }else{
     let admission=self.plan.as_ref().unwrap().inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)?;let owner=self.semantics.as_mut().ok_or_else(||fault("semantics is absent"))?;
     let(d,q)=if recovering{(self.semantic_effect.pending_demand().ok_or_else(||fault("retained inverse recovery lost its demand"))?,self.pending_work)}else{(owner.inverse_demand(mutation,&admission,body,EDIT_TURN_WORK)?,owner.inverse_work_demand(mutation,&admission,body,EDIT_TURN_WORK)?)};
     self.pending_work=q;
     Self::drive(&mut self.last_work,receiving,&mut self.semantic_effect,self.authority.as_mut().ok_or_else(||fault("retained edit authority is absent"))?,d,q,body,Action::Inverse{owner,mutation,admission,slot:&mut self.inverse_result})?
    }
   }
   Stage::Commit=>{
    let plan=self.plan.as_mut().unwrap();let diff=self.source.as_ref().unwrap().try_borrow()?;
    let quote=plan.commit_work_demand(diff,before,post.read().get(),body)?;
    let target=post.mutable()?;
    let Some(step)=once(&mut self.last_work,receiving,g,quote,|_,effect|commit_retained_diff(plan,diff,before,target,effect).result)?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
    let step=step?;let p=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.stage=Stage::Join;}p
   }
   Stage::Join=>{
    if self.join.is_none(){
     let d=MutationMessageConcat::<2>::constructor_demand();if !semio_os_spr::retained_mutation_demand_fits(d,g){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
     let messages=&mut self.messages;
     let Some(admitted)=once(&mut self.last_work,receiving,g,1,|_,effect|{perform(effect,1)?;let batches=std::array::from_fn(|i|messages[i].take().unwrap_or_default());Ok::<_,ValueError>(MutationMessageConcat::admit(batches,g))})?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
     match admitted?{Ok((j,p))=>{*self.join=Some(j);p},Err((e,batches))=>{for(i,b)in batches.into_iter().enumerate(){self.messages[i]=Some(b);}return Err(e.into());}}
    }else{
     let join=self.join.as_mut().unwrap();
     let Some(step)=once(&mut self.last_work,receiving,g,1,|_,effect|{perform(effect,1)?;join.advance(g)})?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
     let step=step?;if matches!(step,RetainedCloneStep::Complete(_)){self.stage=Stage::Publish;}step.progress()
    }
   }
   Stage::Publish=>{
    let join=self.join.as_mut().unwrap();
    let Some(taken)=once(&mut self.last_work,receiving,g,1,|_,effect|{perform(effect,1)?;join.take(g)})?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
    let Some((messages,p))=taken?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
    self.stage=Stage::Done;
    let rows=if self.refused{None}else{Some(self.inverse.take().ok_or_else(||fault("retained admitted inverse is absent"))?)};
    let p=Self::progress(grant,p)?;
    return Ok(if let Some(rows)=rows{PagedOneItemEditStep::CompleteOutcome(p,rows,messages)}else{PagedOneItemEditStep::RefusedOutcome(p,messages)});
   }
   Stage::Done=>unreachable!(),
  };
  if self.stage!=previous&&p==RetainedCloneProgress::default(){p.copied_items=1;}
  Ok(PagedOneItemEditStep::Progress(Self::progress(grant,p)?))
 }
 fn last_turn_work(&self)->u64{self.last_work}

 fn has_pending_receiving_recovery(&self)->bool{self.birth_effect.has_pending_recovery()||self.semantic_effect.has_pending_recovery()}
 fn pending_receiving_demand(&self)->Option<RetirementDemand>{self.birth_effect.pending_demand().or_else(||self.semantic_effect.pending_demand())}
 fn pending_receiving_recovery(&mut self)->Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>>{
  use semio_framework_value::native_decoding::construction::NativeRecoveryLoan;
  if self.birth_effect.has_pending_recovery(){return self.birth_effect.pending_recovery().map(NativeRecoveryLoan::Value)}
  self.semantic_effect.pending_recovery().map(NativeRecoveryLoan::Retirement)
 }
 fn finish_receiving_recovery(&mut self,control:&mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,semio_framework_value::OriginalReceivingPrimary>{
  let turn=if self.birth_effect.has_pending_recovery(){control.finish_receiving(&mut self.birth_effect)}else if self.semantic_effect.has_pending_recovery(){control.finish_retirement_receiving(&mut self.semantic_effect)}else{semio_framework_value::RetainedWorkOutcome{result:Ok(Default::default()),work:0}};
  if self.closing&&!self.birth_effect.has_pending_recovery()&&!self.semantic_effect.has_pending_recovery(){if let Some(o)=self.semantics.as_mut(){o.begin_close();}if let Some(o)=self.plan.as_mut(){o.begin_close();}if let Some(o)=self.join.as_mut(){o.begin_close();}}
  turn
 }
 fn begin_close(&mut self){self.closing=true;if !self.birth_effect.has_pending_recovery()&&!self.semantic_effect.has_pending_recovery(){if let Some(o)=self.semantics.as_mut(){o.begin_close();}if let Some(o)=self.plan.as_mut(){o.begin_close();}if let Some(o)=self.join.as_mut(){o.begin_close();}}}
 fn close_demands(&self,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,body:usize)->Result<RetirementDemand,ValueError>{self.owner_close_demands(mutations,body)}
 fn close_work_demand(&self,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,body:usize)->Result<u64,ValueError>{
  self.close_demands(mutations,body)?;
  if self.terminal_is_empty(){return Ok(0)}
  if self.decision_result.is_some(){return u64::try_from(size_of::<Option<RetainedMutationDecisionStep<M::Diff>>>().checked_mul(2).ok_or_else(||fault("retained outcome close extent overflow"))?).ok().and_then(|n|n.checked_add(1)).ok_or_else(||fault("retained outcome close work overflow"))}
  if self.preflight_result.is_some(){return u64::try_from(size_of::<Option<RetainedDiffPreflightStep>>().checked_mul(2).ok_or_else(||fault("retained outcome close extent overflow"))?).ok().and_then(|n|n.checked_add(1)).ok_or_else(||fault("retained outcome close work overflow"))}
  if self.inverse_result.is_some(){return u64::try_from(size_of::<Option<RetainedMutationInverseStep<M>>>().checked_mul(2).ok_or_else(||fault("retained outcome close extent overflow"))?).ok().and_then(|n|n.checked_add(1)).ok_or_else(||fault("retained outcome close work overflow"))}
  if let Some(owner)=self.semantics.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.close_work_demand(Self::context(mutations),body)}}
  if let Some(owner)=self.plan.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.close_work_demand(body)}}
  if let Some(admission)=self.admission.as_ref(){return if admission.phase()=="ready"{Ok(1)}else{admission.return_work_demand()}}
  if let Some(owner)=self.source.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_close_work_demand(body)}}
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<M::Diff>(false,&None,body)}
  if self.diff.is_some(){return self.retirement.next_owner_work::<M::Diff>(true,&None,body)}
  if let Some(owner)=self.inverse_close.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.close_work_demand(mutations,body)}}
  if self.inverse.is_some(){return Ok(1)}
  if let Some(owner)=self.join.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.close_work_demand()}}
  Ok(u64::from(self.messages.iter().any(Option::is_some)))
 }
 fn close_step(&mut self,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   if !self.closing{return Err(fault("retained mutation edit closure not started").into());}
   let grant=effect.grant();
   let d=self.close_demands(mutations,grant.maximum_copy_bytes)?;
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
   if !semio_os_spr::retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
   self.owner_close_step(mutations,d,effect)
  })();
  RetirementPollExit::settle(effect,result)
 }
 fn terminal_is_empty(&self)->bool{self.owner_terminal_is_empty()}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> MutationApplyEdit<S,M>{
 pub fn stage(&self)->Stage{self.stage}
 pub fn inverse_rows(&self)->Option<&[M]>{self.inverse.as_deref()}
 fn owner_close_demands(&self,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.birth_effect.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty||self.semantic_effect.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original receiving frame requires its original recovery loan"));}
  if self.decision_result.is_some(){return Ok(scalar(size_of::<Option<RetainedMutationDecisionStep<M::Diff>>>().saturating_mul(2)));}
  if self.inverse_result.is_some(){return Ok(scalar(size_of::<Option<RetainedMutationInverseStep<M>>>().saturating_mul(2)));}
  if self.preflight_result.is_some(){return Ok(scalar(size_of::<Option<RetainedDiffPreflightStep>>().saturating_mul(2)));}
  if let Some(o)=self.semantics.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<RetainedMutationOwner<S,M>>>()))}else{o.close_demand(Self::context(mutations),body)};}
  if let Some(o)=self.plan.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<RetainedDiffOwner<S,M::Diff>>>()))}else{o.close_demand(body)};}
  if let Some(a)=self.admission.as_ref(){return if a.phase()=="ready"{a.next_demand()}else{a.next_return_demand()};}
   if let Some(o)=self.source.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<RetainedCloneSource<M::Diff>>>()))}else{o.next_close_demand(body)};}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<M::Diff>(false,&None,body);}
  if self.diff.is_some(){return self.retirement.next_owner_demands::<M::Diff>(true,&None,body);}
  if let Some(o)=self.inverse_close.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<FactoryVectorRetirement<M>>>()))}else{o.demands(mutations,body)};}
  if self.inverse.is_some(){return Ok(FactoryVectorRetirement::<M>::constructor_demand());}
  if let Some(o)=self.join.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<MutationMessageConcat<2>>>()))}else{o.close_demand()};}
  if self.messages.iter().any(Option::is_some){return Ok(MutationMessageConcat::<2>::constructor_demand());}
  Ok(Default::default())
 }
 fn owner_close_step(&mut self,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,d:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();
  let moved=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if self.decision_result.is_some(){
   if self.diff.is_some()||self.messages[0].is_some(){return Err(fault("retained decision close slots are occupied").into())}
   match self.decision_result.take().ok_or_else(||fault("retained decision close lost its result"))?{RetainedMutationDecisionStep::Progress(_)=>{},RetainedMutationDecisionStep::Admitted(_,diff,rows)=>{*self.diff=Some(diff);self.messages[0]=Some(rows)},RetainedMutationDecisionStep::Refused(_,rows)=>self.messages[0]=Some(rows)}
   return Ok(moved())
  }
  if self.inverse_result.is_some(){
   if self.inverse.is_some(){return Err(fault("retained inverse close slot is occupied").into())}
   if let RetainedMutationInverseStep::Ready(_,rows)=self.inverse_result.take().ok_or_else(||fault("retained inverse close lost its result"))?{*self.inverse=Some(rows)}
   return Ok(moved())
  }
  if self.preflight_result.is_some(){
   if self.messages[1].is_some(){return Err(fault("retained preflight close slot is occupied").into())}
   match self.preflight_result.take().ok_or_else(||fault("retained preflight close lost its result"))?{RetainedDiffPreflightStep::Progress(_)=>{},RetainedDiffPreflightStep::Admitted(_,rows)|RetainedDiffPreflightStep::Refused(_,rows)=>self.messages[1]=Some(rows)}
   return Ok(moved())
  }
  if let Some(o)=self.semantics.as_mut(){if o.terminal_is_empty(){drop(self.semantics.take());return Ok(moved());}let step=Self::close_child(&mut self.authority,&mut self.close_action,effect,grant.maximum_copy_bytes,Action::CloseSemantics{owner:o,context:Self::context(mutations)})?;return Ok(RetainedCloneStep::Progress(step));}
  if let Some(o)=self.plan.as_mut(){if o.terminal_is_empty(){drop(self.plan.take());return Ok(moved());}let step=Self::close_child(&mut self.authority,&mut self.close_action,effect,grant.maximum_copy_bytes,Action::ClosePlan{plan:o})?;return Ok(RetainedCloneStep::Progress(step));}
  if let Some(a)=self.admission.as_mut(){
   if a.phase()=="ready"{let mut output=None;let step=a.take_output(&mut output,&mut effect.nested(grant)).result?;if let Some(source)=output{*self.source=Some(source);self.admission.take();}return Ok(RetainedCloneStep::Progress(step.progress()));}
   let mut back=None;let step=a.return_inputs(&mut back,&mut effect.nested(grant)).result?;if let Some((diff,()))=back{*self.diff=Some(diff);self.admission.take();}return Ok(RetainedCloneStep::Progress(step.progress()));
  }
  if let Some(o)=self.source.as_mut(){if o.terminal_is_empty(){drop(self.source.take());return Ok(moved());}let step=RetirementPollExit::resume(o.close_step(grant.maximum_copy_bytes,&mut effect.nested(grant)))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
  if !self.retirement.is_empty(){let step=RetirementPollExit::resume(self.retirement.step_granted(grant.maximum_copy_bytes,&mut effect.nested(grant)))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.diff,&mut effect.nested(grant)).result?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(o)=self.inverse_close.as_mut(){if o.terminal_is_empty(){self.inverse_close=None;return Ok(moved());}let step=RetirementPollExit::resume(o.close_step(mutations,grant.maximum_copy_bytes,&mut effect.nested(grant)))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.inverse.is_some(){
   let step=match FactoryVectorRetirement::admit(&mut self.inverse,&mut self.inverse_close,&mut effect.nested(grant)){RetainedWorkPoll::Pending{work}=>return Err(RetirementPollExit::Pending(work)),RetainedWorkPoll::Complete(turn)=>turn.result?};
   return Ok(RetainedCloneStep::Progress(step.progress()));
  }
  if let Some(o)=self.join.as_mut(){if o.terminal_is_empty(){drop(self.join.take());return Ok(moved());}let turn=o.close_step(grant);RetirementPollExit::pay(effect,turn.work)?;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  if self.messages.iter().any(Option::is_some){let batches=std::array::from_fn(|i|self.messages[i].take().unwrap_or_default());return match MutationMessageConcat::admit(batches,grant){Ok((mut o,p))=>{o.begin_close();*self.join=Some(o);Ok(RetainedCloneStep::Progress(p))},Err((e,batches))=>{for(i,b)in batches.into_iter().enumerate(){self.messages[i]=Some(b);}Err(e.into())}};}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn owner_terminal_is_empty(&self)->bool{self.birth_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&self.semantic_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&self.decision_result.is_none()&&self.inverse_result.is_none()&&self.preflight_result.is_none()&&self.close_action.is_none()&&self.semantics.is_none()&&self.plan.is_none()&&self.source.is_none()&&self.admission.is_none()&&self.diff.is_none()&&self.inverse.is_none()&&self.inverse_close.is_none()&&self.join.is_none()&&self.messages.iter().all(Option::is_none)&&self.retirement.is_empty()}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> Drop for MutationApplyEdit<S,M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"retained mutation edit requires complete original ownership closure");if let Some(authority)=self.authority.take(){drop(authority.into_producer());}if empty{unsafe{ManuallyDrop::drop(&mut self.decision_result);ManuallyDrop::drop(&mut self.inverse_result);ManuallyDrop::drop(&mut self.preflight_result);ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.semantics);ManuallyDrop::drop(&mut self.diff);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.plan);ManuallyDrop::drop(&mut self.inverse);ManuallyDrop::drop(&mut self.messages);ManuallyDrop::drop(&mut self.join);}}}}
/// 🏭️ Every mounted mutation family supplies required retained semantics and a typed central apply plan.
pub fn mutation_apply_preparation_factory<S,M>()->Arc<dyn ArtifactStoreOneItemPreparationFactory<S,M>>where S:SnapshotProjection<Output=S>,M:RetainedMutation<S>+ArtifactCanonicalJsonTree{Arc::new(PagedOneItemPreparationFactory::<S,M,MutationApplyEdit<S,M>>::default())}

#[cfg(test)]
#[path="🧪️tests/📋️stages/🦀️.rs"]mod stage_contract;

/// 🔬️ Borrows already owned inverse rows for the external original-custody laws.
#[cfg(feature="test-support")]
pub fn inverse_rows<S:SnapshotProjection,M:RetainedMutation<S>>(owner:&MutationApplyEdit<S,M>)->Option<&[M]>{owner.inverse.as_ref().map(Vec::as_slice)}
