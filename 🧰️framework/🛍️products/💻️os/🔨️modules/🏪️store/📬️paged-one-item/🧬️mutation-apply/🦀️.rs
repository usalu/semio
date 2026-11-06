//! 🧬️ Genuine retained decision, central preflight, inverse and capability commit share one original issuer.
use super::{PagedOneItemEdit,PagedOneItemEditStep,PagedOneItemPreparationFactory,capture_work};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory;
use semio_os_spr::{MutationDiff,RetainedMutation,RetainedMutationOwner,RetainedDiffOwner,RetainedMutationDecisionStep,RetainedDiffPreflightStep,RetainedMutationInverseStep,RetainedMutationCloseContext,MutationMessage,retained_mutation_birth_demand,begin_retained_mutation,decide_retained_mutation,begin_retained_diff,preflight_retained_diff,inverse_retained_mutation,commit_retained_diff,Mutation};
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneSource,RetainedCloneRef,retained_clone::{RetainedClone,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,RetainedCloneSourceAdmission,admit_retained_clone_progress}};
use semio_framework_value_factory::{ArtifactOwnedValueRetirementFactory,vector::{FactoryVectorRetirement,FactoryIssuerIdentity}};
use std::{mem::{size_of,ManuallyDrop},sync::Arc};
use protocol::MutationMessageConcat;
fn fault(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
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
 diff:ManuallyDrop<Option<M::Diff>>,
 source:ManuallyDrop<Option<RetainedCloneSource<M::Diff>>>,
 admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<M::Diff,M::Diff,()>>>,
 plan:ManuallyDrop<Option<RetainedDiffOwner<S,M::Diff>>>,
 inverse:ManuallyDrop<Option<Vec<M>>>,
 inverse_close:Option<FactoryVectorRetirement<M>>,
 messages:ManuallyDrop<[Option<Vec<protocol::MutationMessage>>;2]>,
 join:ManuallyDrop<Option<MutationMessageConcat<2>>>,
 retirement:RetainedCloneClose,
 issuer:Option<FactoryIssuerIdentity<M>>,
 refused:bool,
 closing:bool,
 last_work:u64,
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> Default for MutationApplyEdit<S,M>{fn default()->Self{Self{stage:Stage::Birth,semantics:ManuallyDrop::new(None),birth_effect:Default::default(),semantic_effect:Default::default(),decision_result:ManuallyDrop::new(None),inverse_result:ManuallyDrop::new(None),diff:ManuallyDrop::new(None),source:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),plan:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),inverse_close:None,messages:ManuallyDrop::new([None,None]),join:ManuallyDrop::new(None),retirement:Default::default(),issuer:None,refused:false,closing:false,last_work:0}}}
impl<S:SnapshotProjection,M:RetainedMutation<S>> MutationApplyEdit<S,M>{
 fn context(mutations:&dyn ArtifactOwnedValueRetirementFactory<M>)->RetainedMutationCloseContext<'_,M>{RetainedMutationCloseContext{mutations}}
 fn parent_copy()->usize{size_of::<Stage>()}
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
  let mut demand=match self.stage{
   Stage::Birth=>retained_mutation_birth_demand::<S,M>(),
   Stage::Decide=>self.semantics.as_ref().unwrap().decision_demand(mutation,before,body,maximum_work_units)?,
   Stage::Source=>self.admission.as_ref().ok_or_else(||fault("retained mutation diff admission is absent"))?.next_demand()?,
   Stage::Plan=>semio_os_spr::retained_diff_birth_demand::<S,M::Diff>(),
   Stage::Preflight=>self.plan.as_ref().unwrap().preflight_demand(self.source.as_ref().unwrap().try_borrow()?,before,post.original()?,body)?,
   Stage::Inverse=>{let a=self.plan.as_ref().unwrap().inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)?;self.semantics.as_ref().unwrap().inverse_demand(mutation,&a,body,maximum_work_units)?},
   Stage::Commit=>self.plan.as_ref().unwrap().commit_demand(self.source.as_ref().unwrap().try_borrow()?,before,post.get(),body)?,
   Stage::Join=>if let Some(join)=self.join.as_ref(){join.demand()?}else{MutationMessageConcat::<2>::constructor_demand()},
   Stage::Publish=>self.join.as_ref().unwrap().demand()?,Stage::Done=>unreachable!()
  };demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||fault("retained semantic header quote overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||fault("retained semantic depth quote overflow"))?.max(2);Ok(demand)
 }
 fn advance(&mut self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:PagedOneItemPost<'_,S>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<PagedOneItemEditStep<M>,semio_framework_value::OriginalReceivingPrimary>{
  self.last_work=0;let observed_before=receiving.actual_work();
  let result:Result<PagedOneItemEditStep<M>,semio_framework_value::OriginalReceivingPrimary>=(||{
  if self.closing||self.stage==Stage::Done{return Err(fault("retained mutation edit is closed").into());}
  if grant.maximum_items==0||grant.maximum_copy_bytes<Self::parent_copy()||grant.maximum_depth<2{return Ok(PagedOneItemEditStep::Progress(Default::default()));}
  if self.semantic_effect.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty{
   let turn=receiving.finish_retirement_receiving(&mut self.semantic_effect);self.last_work=turn.work;return Ok(PagedOneItemEditStep::Progress(Self::progress(grant,turn.result?)?));
  }
  let maximum_work_units=receiving.available_work_units()?;
  let handoff=(self.stage==Stage::Decide&&self.decision_result.is_some())||(self.stage==Stage::Inverse&&self.inverse_result.is_some());
  let demand=if handoff{scalar(Self::parent_copy())}else{self.advance_demands(before,mutation,post.read(),body,maximum_work_units)?};if !semio_os_spr::retained_mutation_demand_fits(demand,grant){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
  let birth_handoff=self.stage==Stage::Birth&&self.semantics.is_some()&&self.birth_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty;
  if (!matches!(self.stage,Stage::Birth|Stage::Decide|Stage::Inverse)||birth_handoff||handoff)&&!receiving.prepare_work(1)?{return Ok(PagedOneItemEditStep::Progress(Default::default()));}
  let g=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::parent_copy(),maximum_depth:grant.maximum_depth-1,..grant};
  let previous=self.stage;
  let mut p=match self.stage{
   Stage::Birth=>{
    if birth_handoff{self.last_work=1;self.stage=Stage::Decide;RetainedCloneProgress::default()}
    else{let d=retained_mutation_birth_demand::<S,M>();let q=semio_os_spr::retained_mutation_birth_work::<S,M>()?;let slot=&mut self.semantics;
     let turn=receiving.receive_allocation(d,&mut self.birth_effect,d.capacity_bytes!=0,q,|issued,actual|{
      let mut none=None;let result=begin_retained_mutation::<S,M>(issued.restricted_by(g),q,actual.unwrap_or(&mut none));let work=result.work;
      semio_framework_value::RetainedWorkOutcome{result:result.result.map(|value|match value{Some((owner,p))=>{**slot=Some(owner);RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())}),work}
     });self.last_work=turn.work;turn.result?
    }
   }
   Stage::Decide=>{
    if handoff{self.last_work=1;match self.decision_result.take().ok_or_else(||fault("retained decision handoff is absent"))?{RetainedMutationDecisionStep::Admitted(_,d,rows)=>{*self.admission=Some(RetainedCloneSource::admission_owned(d,()));self.messages[0]=Some(rows);self.stage=Stage::Source;},RetainedMutationDecisionStep::Refused(_,rows)=>{self.messages[0]=Some(rows);self.refused=true;self.stage=Stage::Join;},RetainedMutationDecisionStep::Progress(_)=>return Err(fault("retained decision handoff was not terminal").into())}RetainedCloneProgress::default()}
    else{let owner=self.semantics.as_mut().ok_or_else(||fault("semantics is absent"))?;let d=owner.decision_demand(mutation,before,body,maximum_work_units)?;let q=owner.decision_work_demand(mutation,before,body,maximum_work_units)?;let slot=&mut self.decision_result;
     let turn=receiving.receive_retirement(d,&mut self.semantic_effect,d.capacity_bytes!=0,q,|issued,actual|{let mut none=None;let turn=decide_retained_mutation(owner,mutation,before,body,q,issued.restricted_by(g),actual.unwrap_or(&mut none));let work=turn.work;semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|{let p=step.progress();if !matches!(&step,RetainedMutationDecisionStep::Progress(_)){**slot=Some(step);}RetainedCloneStep::Progress(p)}),work}});self.last_work=turn.work;turn.result?
    }
   }
   Stage::Source=>{
    let admission=self.admission.as_mut().ok_or_else(||fault("retained mutation diff admission is absent"))?;
    if admission.phase()!="ready"{capture_work(&mut self.last_work,admission.advance(g))?.progress()}else{let mut output=None;let step=capture_work(&mut self.last_work,admission.take_output(&mut output,g))?;if let Some(source)=output{*self.source=Some(source);self.admission.take();self.stage=Stage::Plan;}step.progress()}
   }
   Stage::Plan=>{let Some((o,p))=begin_retained_diff::<S,M::Diff>(g)else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};self.last_work=self.last_work.saturating_add(1);*self.plan=Some(o);self.stage=Stage::Preflight;p}
   Stage::Preflight=>{self.last_work=self.last_work.saturating_add(1);let result=preflight_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post.original()?,body,g,receiving).map_err(|error|{let mut p=error.retained_progress();if p!=RetainedCloneProgress::default(){p.copied_bytes=p.copied_bytes.saturating_add(Self::parent_copy());}error.with_retained_progress(p)})?;let p=result.progress();match result{RetainedDiffPreflightStep::Progress(_)=>{},RetainedDiffPreflightStep::Admitted(_,rows)=>{self.messages[1]=Some(rows);self.stage=Stage::Inverse;},RetainedDiffPreflightStep::Refused(_,rows)=>{self.messages[1]=Some(rows);self.refused=true;self.stage=Stage::Join;}}p}
   Stage::Inverse=>{
    if handoff{self.last_work=1;match self.inverse_result.take().ok_or_else(||fault("retained inverse handoff is absent"))?{RetainedMutationInverseStep::Ready(_,rows)=>{*self.inverse=Some(rows);self.stage=Stage::Commit;},RetainedMutationInverseStep::Progress(_)=>return Err(fault("retained inverse handoff was not terminal").into())}RetainedCloneProgress::default()}
    else{let a=self.plan.as_ref().unwrap().inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)?;let owner=self.semantics.as_mut().ok_or_else(||fault("semantics is absent"))?;let d=owner.inverse_demand(mutation,&a,body,maximum_work_units)?;let q=owner.inverse_work_demand(mutation,&a,body,maximum_work_units)?;let slot=&mut self.inverse_result;
     let turn=receiving.receive_retirement(d,&mut self.semantic_effect,d.capacity_bytes!=0,q,|issued,actual|{let mut none=None;let turn=inverse_retained_mutation(owner,mutation,&a,body,q,issued.restricted_by(g),actual.unwrap_or(&mut none));let work=turn.work;semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|{let p=step.progress();if !matches!(&step,RetainedMutationInverseStep::Progress(_)){**slot=Some(step);}RetainedCloneStep::Progress(p)}),work}});self.last_work=turn.work;turn.result?
    }
   }
   Stage::Commit=>{self.last_work=self.last_work.saturating_add(1);let step=commit_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post.mutable()?,g)?;let p=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.stage=Stage::Join;}p}
   Stage::Join=>{
    if self.join.is_none(){let d=MutationMessageConcat::<2>::constructor_demand();if !semio_os_spr::retained_mutation_demand_fits(d,g){return Ok(PagedOneItemEditStep::Progress(Default::default()));}let batches=std::array::from_fn(|i|self.messages[i].take().unwrap_or_default());self.last_work=self.last_work.saturating_add(1);match MutationMessageConcat::admit(batches,g){Ok((j,p))=>{*self.join=Some(j);p},Err((e,batches))=>{for(i,b)in batches.into_iter().enumerate(){self.messages[i]=Some(b);}return Err(e);}}}else{self.last_work=self.last_work.saturating_add(1);let step=self.join.as_mut().unwrap().advance(g)?;if matches!(step,RetainedCloneStep::Complete(_)){self.stage=Stage::Publish;}step.progress()}
   }
   Stage::Publish=>{
    let Some((messages,p))=self.join.as_mut().unwrap().take(g)?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};self.last_work=self.last_work.saturating_add(1);
    self.stage=Stage::Done;
    let rows=if self.refused{None}else{Some(self.inverse.take().ok_or_else(||fault("retained admitted inverse is absent"))?)};
    let p=Self::progress(grant,p)?;
    return Ok(if let Some(rows)=rows{PagedOneItemEditStep::CompleteOutcome(p,rows,messages)}else{PagedOneItemEditStep::RefusedOutcome(p,messages)});
   }
   Stage::Done=>unreachable!(),
  };
  if self.stage!=previous&&p==RetainedCloneProgress::default(){p.copied_items=1;}
  Ok(PagedOneItemEditStep::Progress(Self::progress(grant,p)?))
 
  })();
  let observed=receiving.actual_work().saturating_sub(observed_before);self.last_work=self.last_work.max(observed);let reported=self.last_work;
  let work=semio_os_store_one_item::original_one_item_turn_work(reported,observed,false)?;if work>observed{receiving.commit_work(work-observed)?;}self.last_work=work;result
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
 fn close_demands(&self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.issuer.is_some_and(|i|!i.matches(mutations)){return Err(fault("retained mutation edit original issuer changed").into());}
  let capture=self.issuer_capture_bytes();let mut d=self.owner_close_demands(mutations,body.saturating_sub(capture))?;d.copy_bytes=d.copy_bytes.checked_add(capture).ok_or_else(||fault("retained edit issuer capture overflow"))?;Ok(d)
 }
 fn close_work_demand(&self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,body:usize)->Result<u64,ValueError>{
  self.close_demands(mutations,body)?;
  if self.terminal_is_empty(){return Ok(0)}
  let selected=if self.decision_result.is_some(){u64::try_from(size_of::<Option<RetainedMutationDecisionStep<M::Diff>>>().checked_mul(2).ok_or_else(||fault("retained outcome close extent overflow"))?).ok().and_then(|n|n.checked_add(1)).ok_or_else(||fault("retained outcome close work overflow"))?}
  else if self.inverse_result.is_some(){u64::try_from(size_of::<Option<RetainedMutationInverseStep<M>>>().checked_mul(2).ok_or_else(||fault("retained outcome close extent overflow"))?).ok().and_then(|n|n.checked_add(1)).ok_or_else(||fault("retained outcome close work overflow"))?}
  else if let Some(owner)=self.semantics.as_ref(){if owner.terminal_is_empty(){1}else{owner.close_work_demand(Self::context(mutations),body)?}}
  else if let Some(owner)=self.plan.as_ref(){if owner.terminal_is_empty(){1}else{owner.close_work_demand(body)?}}
  else if let Some(admission)=self.admission.as_ref(){if admission.phase()=="ready"{1}else{admission.return_work_demand()?}}
  else if let Some(owner)=self.source.as_ref(){if owner.terminal_is_empty(){1}else{owner.next_close_work_demand(body)?}}
  else if !self.retirement.is_empty(){self.retirement.next_owner_work::<M::Diff>(false,&None,body)?}
  else if self.diff.is_some(){self.retirement.next_owner_work::<M::Diff>(true,&None,body)?}
  else if let Some(owner)=self.inverse_close.as_ref(){if owner.terminal_is_empty(){1}else{owner.close_work_demand(mutations,body)?}}
  else if self.inverse.is_some(){1}
  else if let Some(owner)=self.join.as_ref(){if owner.terminal_is_empty(){1}else{owner.close_work_demand()?}}
  else if self.messages.iter().any(Option::is_some){1}else{0};
  selected.checked_add(u64::from(self.issuer_capture_bytes()!=0)).ok_or_else(||fault("retained edit close work overflow"))
 }
 fn close_step(&mut self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
  self.last_work=0;if !self.closing{return Err(fault("retained mutation edit closure not started").into());}self.close_work_demand(mutations,grant.maximum_copy_bytes)?;let d=self.close_demands(mutations,grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !semio_os_spr::retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let capture=self.issuer_capture_bytes();if capture>0{self.issuer=Some(FactoryIssuerIdentity::capture(mutations));self.last_work+=1;}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-capture,..grant};
  let result=self.owner_close_step(mutations,child,actual).map_err(|error|{let mut p=error.retained_progress();p.copied_bytes+=capture;if capture>0{p.copied_items=p.copied_items.max(1);}error.with_retained_progress(p)})?;let mut p=result.progress();p.copied_bytes+=capture;if capture>0{p.copied_items=p.copied_items.max(1);}let p=admit_retained_clone_progress(grant,p,"exact original retained editor issuer capture")?;Ok(if matches!(result,RetainedCloneStep::Complete(_)){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn terminal_is_empty(&self)->bool{self.owner_terminal_is_empty()}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> MutationApplyEdit<S,M>{
 pub fn stage(&self)->Stage{self.stage}
 pub fn inverse_rows(&self)->Option<&[M]>{self.inverse.as_deref()}
 fn issuer_capture_bytes(&self)->usize{if self.issuer.is_none()&&!<Self as PagedOneItemEdit<S,M>>::terminal_is_empty(self){FactoryIssuerIdentity::<M>::capture_copy_bytes()}else{0}}
 fn owner_close_demands(&self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.birth_effect.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty||self.semantic_effect.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original receiving frame requires its original recovery loan"));}
  if self.decision_result.is_some(){return Ok(scalar(size_of::<Option<RetainedMutationDecisionStep<M::Diff>>>().saturating_mul(2)));}
  if self.inverse_result.is_some(){return Ok(scalar(size_of::<Option<RetainedMutationInverseStep<M>>>().saturating_mul(2)));}
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
 fn owner_close_step(&mut self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
  if !self.closing{return Err(fault("retained mutation edit closure not started").into());}if self.issuer.is_some_and(|i|!i.matches(mutations)){return Err(fault("retained mutation edit original issuer changed"));}let d=self.close_demands(mutations,grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !semio_os_spr::retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
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
  if let Some(o)=self.semantics.as_mut(){if o.terminal_is_empty(){drop(self.semantics.take());self.last_work+=1;return Ok(moved());}let turn=o.close_step(Self::context(mutations),grant,actual);self.last_work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(o)=self.plan.as_mut(){if o.terminal_is_empty(){drop(self.plan.take());self.last_work+=1;return Ok(moved());}let turn=o.close_step(grant);self.last_work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(a)=self.admission.as_mut(){if a.phase()=="ready"{let mut output=None;let turn=a.take_output(&mut output,grant);self.last_work+=turn.work;let step=turn.result?;if let Some(source)=output{*self.source=Some(source);self.admission.take();}return Ok(RetainedCloneStep::Progress(step.progress()));}let mut back=None;let turn=a.return_inputs(&mut back,grant);self.last_work+=turn.work;let step=turn.result?;if let Some((diff,()))=back{*self.diff=Some(diff);self.admission.take();}return Ok(RetainedCloneStep::Progress(step.progress()));}
   if let Some(o)=self.source.as_mut(){if o.terminal_is_empty(){drop(self.source.take());self.last_work+=1;return Ok(moved());}let turn=o.close_step(grant);self.last_work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant,actual);self.last_work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  let turn=self.retirement.begin_granted(&mut self.diff,grant,actual);self.last_work+=turn.work;if let Some(s)=turn.result?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(o)=self.inverse_close.as_mut(){if o.terminal_is_empty(){self.inverse_close=None;self.last_work+=1;return Ok(moved());}let turn=o.close_step(mutations,grant,actual);self.last_work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(rows)=self.inverse.take(){self.last_work+=1;return match FactoryVectorRetirement::admit(rows,grant){Ok((o,p))=>{self.inverse_close=Some(o);Ok(RetainedCloneStep::Progress(p))},Err((e,rows))=>{*self.inverse=Some(rows);Err(e.into())}};}
  if let Some(o)=self.join.as_mut(){if o.terminal_is_empty(){drop(self.join.take());self.last_work+=1;return Ok(moved());}let turn=o.close_step(grant);self.last_work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into);}
  if self.messages.iter().any(Option::is_some){self.last_work+=1;let batches=std::array::from_fn(|i|self.messages[i].take().unwrap_or_default());return match MutationMessageConcat::admit(batches,grant){Ok((mut o,p))=>{o.begin_close();*self.join=Some(o);Ok(RetainedCloneStep::Progress(p))},Err((e,batches))=>{for(i,b)in batches.into_iter().enumerate(){self.messages[i]=Some(b);}Err(e.into())}};}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn owner_terminal_is_empty(&self)->bool{self.birth_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&self.semantic_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&self.decision_result.is_none()&&self.inverse_result.is_none()&&self.semantics.is_none()&&self.plan.is_none()&&self.source.is_none()&&self.admission.is_none()&&self.diff.is_none()&&self.inverse.is_none()&&self.inverse_close.is_none()&&self.join.is_none()&&self.messages.iter().all(Option::is_none)&&self.retirement.is_empty()}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> Drop for MutationApplyEdit<S,M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"retained mutation edit requires complete original ownership closure");if empty{unsafe{ManuallyDrop::drop(&mut self.decision_result);ManuallyDrop::drop(&mut self.inverse_result);ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.semantics);ManuallyDrop::drop(&mut self.diff);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.plan);ManuallyDrop::drop(&mut self.inverse);ManuallyDrop::drop(&mut self.messages);ManuallyDrop::drop(&mut self.join);}}}}
/// 🏭️ Every mounted mutation family supplies required retained semantics and a typed central apply plan.
pub fn mutation_apply_preparation_factory<S,M>()->Arc<dyn ArtifactStoreOneItemPreparationFactory<S,M>>where S:SnapshotProjection<Output=S>,M:RetainedMutation<S>+ArtifactCanonicalJsonTree{Arc::new(PagedOneItemPreparationFactory::<S,M,MutationApplyEdit<S,M>>::default())}

#[cfg(test)]
#[path="🧪️tests/📋️stages/🦀️.rs"]mod stage_contract;

/// 🔬️ Borrows already owned inverse rows for the external original-custody laws.
#[cfg(feature="test-support")]
pub fn inverse_rows<S:SnapshotProjection,M:RetainedMutation<S>>(owner:&MutationApplyEdit<S,M>)->Option<&[M]>{owner.inverse.as_ref().map(Vec::as_slice)}
