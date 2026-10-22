//! 🧬️ Generic structural edit for any document lane: the `Mutation<S>` contract (inverse rows, diff, diff application) driven through the paged one-item preparation.
//!
//! Original immutable aliases feed the required retained semantics, whose typed diff the shared central plan preflights and commits; every child keeps its own funded close custody.
use super::{PagedOneItemEdit, PagedOneItemEditStep, PagedOneItemPreparationFactory};
use semio_framework_os_spr::{
    begin_retained_diff, begin_retained_mutation, commit_retained_diff, decide_retained_mutation, inverse_retained_mutation, preflight_retained_diff, retained_diff_birth_demand, retained_mutation_birth_demand,
    retained_mutation_demand_fits, MutationMessage, RetainedDiffOwner, RetainedDiffPreflightStep, RetainedMutation, RetainedMutationCloseContext, RetainedMutationDecisionStep, RetainedMutationInverseStep, RetainedMutationOwner,
};
use protocol::MutationMessageConcat;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::retained_clone::{admit_retained_clone_progress, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_factory::vector::{FactoryIssuerIdentity, FactoryVectorRetirement};
use semio_framework_value_factory::ArtifactOwnedValueRetirementFactory;
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use std::{mem::{size_of, ManuallyDrop}, sync::Arc};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stage { Birth, Decide, Source, Plan, Preflight, Inverse, Commit, Join, Publish, Done }

fn fault(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }
fn scalar(copy_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, depth: 1, ..Default::default() } }

/// 🧬️ Bounded turns over original aliases: semantic decision, owned diff source, central plan, preflight, inverse, commit and diagnostic join, each with its own close custody.
pub struct MutationApplyEdit<S:SnapshotProjection, M: RetainedMutation<S>> {
    stage: Stage,
    closing: bool,
    refused: bool,
    semantics: ManuallyDrop<Option<RetainedMutationOwner<S, M>>>,
    diff: ManuallyDrop<Option<M::Diff>>,
    source: ManuallyDrop<Option<RetainedCloneSource<M::Diff>>>,
    plan: ManuallyDrop<Option<RetainedDiffOwner<S, M::Diff>>>,
    inverse: ManuallyDrop<Option<Vec<M>>>,
    inverse_close: Option<FactoryVectorRetirement<M>>,
    join: ManuallyDrop<Option<MutationMessageConcat<2>>>,
    messages: ManuallyDrop<[Option<Vec<MutationMessage>>; 2]>,
    retirement: RetainedCloneClose,
    issuer: Option<FactoryIssuerIdentity<M>>,
}

impl<S:SnapshotProjection, M: RetainedMutation<S>> Default for MutationApplyEdit<S, M> {
    fn default() -> Self {
        Self { stage: Stage::Birth, closing: false, refused: false, semantics: ManuallyDrop::new(None), diff: ManuallyDrop::new(None), source: ManuallyDrop::new(None), plan: ManuallyDrop::new(None), inverse: ManuallyDrop::new(None), inverse_close: None, join: ManuallyDrop::new(None), messages: ManuallyDrop::new([None, None]), retirement: RetainedCloneClose::default(), issuer: None }
    }
}

impl<S:SnapshotProjection, M: RetainedMutation<S>> MutationApplyEdit<S, M> {
    /// 🎚️ The stage this edit has reached, observable by the test targets that drive it turn by turn.
    pub fn stage(&self) -> Stage { self.stage }
    /// ↩️ The decided inverse rows held until the edit publishes them or closes.
    pub fn inverse_rows(&self) -> Option<&[M]> { self.inverse.as_deref() }
    fn parent_copy() -> usize { size_of::<u8>() }
    fn context(mutations: &dyn ArtifactOwnedValueRetirementFactory<M>) -> RetainedMutationCloseContext<'_, M> { RetainedMutationCloseContext { mutations } }
    fn progress(grant: RetainedCloneGrant, progress: RetainedCloneProgress) -> Result<RetainedCloneProgress, ValueError> {
        let mut progress = progress;
        progress.copied_bytes = progress.copied_bytes.checked_add(Self::parent_copy()).ok_or_else(|| fault("retained semantic parent receipt overflow"))?;
        admit_retained_clone_progress(grant, progress, "retained mutation edit")
    }
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> PagedOneItemEdit<S,M> for MutationApplyEdit<S,M>{
 const PREFIX:&'static str="mutation-apply";
 fn recognizes(_:&M)->bool{true}
 fn preflight(_:&M)->Result<usize,String>{if !M::Diff::controlled_retirement_supported(){return Err("mutation-apply-diff-retirement-unsupported".into());}Ok(size_of::<M>())}
 fn advance_demands(&self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:&S,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.stage==Stage::Done{return Err(fault("retained mutation edit is closed"));}
  let parent=Self::parent_copy();
  let mut demand=match self.stage{
   Stage::Birth=>retained_mutation_birth_demand::<S,M>(),
   Stage::Decide=>self.semantics.as_ref().unwrap().decision_demand(mutation,before,body)?,
   Stage::Source=>RetirementDemand{copy_bytes:RetainedCloneSource::<M::Diff>::constructor_copy_bytes()+size_of::<Option<M::Diff>>(),capacity_bytes:RetainedCloneSource::<M::Diff>::owned_constructor_capacity_bytes::<()>(),depth:1,..Default::default()},
   Stage::Plan=>retained_diff_birth_demand::<S,M::Diff>(),
   Stage::Preflight=>self.plan.as_ref().unwrap().preflight_demand(self.source.as_ref().unwrap().try_borrow()?,before,post,body)?,
   Stage::Inverse=>{let a=self.plan.as_ref().unwrap().inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)?;self.semantics.as_ref().unwrap().inverse_demand(mutation,&a,body)?},
   Stage::Commit=>self.plan.as_ref().unwrap().commit_demand(self.source.as_ref().unwrap().try_borrow()?,before,post,body)?,
   Stage::Join=>if let Some(join)=self.join.as_ref(){join.demand()?}else{MutationMessageConcat::<2>::constructor_demand()},
   Stage::Publish=>self.join.as_ref().unwrap().demand()?,Stage::Done=>unreachable!()
  };demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||fault("retained semantic header quote overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||fault("retained semantic depth quote overflow"))?.max(2);Ok(demand)
 }
 fn advance(&mut self,before:RetainedCloneRef<'_,S>,mutation:RetainedCloneRef<'_,M>,post:&mut S,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<PagedOneItemEditStep<M>,ValueError>{
  if self.closing||self.stage==Stage::Done{return Err(fault("retained mutation edit is closed"));}
  if grant.maximum_items==0||grant.maximum_copy_bytes<Self::parent_copy()||grant.maximum_depth<2{return Ok(PagedOneItemEditStep::Progress(Default::default()));}
  let demand=self.advance_demands(before,mutation,post,body)?;if !retained_mutation_demand_fits(demand,grant){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
  let g=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::parent_copy(),maximum_depth:grant.maximum_depth-1,..grant};
  let previous=self.stage;
  let mut p=match self.stage{
   Stage::Birth=>{let Some((o,p))=begin_retained_mutation::<S,M>(g)else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};*self.semantics=Some(o);self.stage=Stage::Decide;p}
   Stage::Decide=>{let result=decide_retained_mutation(self.semantics.as_mut().unwrap(),mutation,before,body,g)?;let p=result.progress();match result{RetainedMutationDecisionStep::Progress(_)=>{},RetainedMutationDecisionStep::Admitted(_,d,rows)=>{*self.diff=Some(d);self.messages[0]=Some(rows);self.stage=Stage::Source;},RetainedMutationDecisionStep::Refused(_,rows)=>{self.messages[0]=Some(rows);self.refused=true;self.stage=Stage::Join;}}p}
   Stage::Source=>{
    let d=RetirementDemand{copy_bytes:RetainedCloneSource::<M::Diff>::constructor_copy_bytes()+size_of::<Option<M::Diff>>(),capacity_bytes:RetainedCloneSource::<M::Diff>::owned_constructor_capacity_bytes::<()>(),depth:1,..Default::default()};
    if !retained_mutation_demand_fits(d,g){return Ok(PagedOneItemEditStep::Progress(Default::default()));}
    let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<Option<M::Diff>>(),..g};
    match RetainedCloneSource::admit_owned(self.diff.take().unwrap(),(),child){Ok((s,mut p))=>{*self.source=Some(s);self.stage=Stage::Plan;p.copied_bytes+=size_of::<Option<M::Diff>>();p},Err((e,d,()))=>{*self.diff=Some(d);return Err(e);}}
   }
   Stage::Plan=>{let Some((o,p))=begin_retained_diff::<S,M::Diff>(g)else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};*self.plan=Some(o);self.stage=Stage::Preflight;p}
   Stage::Preflight=>{let result=preflight_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post,body,g,receiving).map_err(|error|{let mut p=error.retained_progress();if p!=RetainedCloneProgress::default(){p.copied_bytes=p.copied_bytes.saturating_add(Self::parent_copy());}error.with_retained_progress(p)})?;let p=result.progress();match result{RetainedDiffPreflightStep::Progress(_)=>{},RetainedDiffPreflightStep::Admitted(_,rows)=>{self.messages[1]=Some(rows);self.stage=Stage::Inverse;},RetainedDiffPreflightStep::Refused(_,rows)=>{self.messages[1]=Some(rows);self.refused=true;self.stage=Stage::Join;}}p}
   Stage::Inverse=>{let a=self.plan.as_ref().unwrap().inverse_admission(self.source.as_ref().unwrap().try_borrow()?,before)?;let result=inverse_retained_mutation(self.semantics.as_mut().unwrap(),mutation,&a,body,g)?;let p=result.progress();if let RetainedMutationInverseStep::Ready(_,rows)=result{*self.inverse=Some(rows);self.stage=Stage::Commit;}p}
   Stage::Commit=>{let step=commit_retained_diff(self.plan.as_mut().unwrap(),self.source.as_ref().unwrap().try_borrow()?,before,post,g)?;let p=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.stage=Stage::Join;}p}
   Stage::Join=>{
    if self.join.is_none(){let d=MutationMessageConcat::<2>::constructor_demand();if !retained_mutation_demand_fits(d,g){return Ok(PagedOneItemEditStep::Progress(Default::default()));}let batches=std::array::from_fn(|i|self.messages[i].take().unwrap_or_default());match MutationMessageConcat::admit(batches,g){Ok((j,p))=>{*self.join=Some(j);p},Err((e,batches))=>{for(i,b)in batches.into_iter().enumerate(){self.messages[i]=Some(b);}return Err(e);}}}else{let step=self.join.as_mut().unwrap().advance(g)?;if matches!(step,RetainedCloneStep::Complete(_)){self.stage=Stage::Publish;}step.progress()}
   }
   Stage::Publish=>{
    let Some((messages,p))=self.join.as_mut().unwrap().take(g)?else{return Ok(PagedOneItemEditStep::Progress(Default::default()));};
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
 fn begin_close(&mut self){self.closing=true;if let Some(o)=self.semantics.as_mut(){o.begin_close();}if let Some(o)=self.plan.as_mut(){o.begin_close();}if let Some(o)=self.join.as_mut(){o.begin_close();}}
 fn close_demands(&self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.issuer.is_some_and(|i|!i.matches(mutations)){return Err(fault("retained mutation edit original issuer changed"));}
  let capture=self.issuer_capture_bytes();let mut d=self.owner_close_demands(mutations,body.saturating_sub(capture))?;d.copy_bytes=d.copy_bytes.checked_add(capture).ok_or_else(||fault("retained edit issuer capture overflow"))?;Ok(d)
 }
 fn close_step(&mut self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(fault("retained mutation edit closure not started"));}let d=self.close_demands(mutations,grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let capture=self.issuer_capture_bytes();if capture>0{self.issuer=Some(FactoryIssuerIdentity::capture(mutations));}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-capture,..grant};
  let result=self.owner_close_step(mutations,child).map_err(|error|{let mut p=error.retained_progress();p.copied_bytes+=capture;if capture>0{p.copied_items=p.copied_items.max(1);}error.with_retained_progress(p)})?;let mut p=result.progress();p.copied_bytes+=capture;if capture>0{p.copied_items=p.copied_items.max(1);}let p=admit_retained_clone_progress(grant,p,"exact original retained editor issuer capture")?;Ok(if matches!(result,RetainedCloneStep::Complete(_)){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn terminal_is_empty(&self)->bool{self.owner_terminal_is_empty()}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> MutationApplyEdit<S,M>{
 fn issuer_capture_bytes(&self)->usize{if self.issuer.is_none()&&!<Self as PagedOneItemEdit<S,M>>::terminal_is_empty(self){FactoryIssuerIdentity::<M>::capture_copy_bytes()}else{0}}
 fn owner_close_demands(&self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(o)=self.semantics.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<RetainedMutationOwner<S,M>>>()))}else{o.close_demand(Self::context(mutations),body)};}
  if let Some(o)=self.plan.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<RetainedDiffOwner<S,M::Diff>>>()))}else{o.close_demand(body)};}
  if let Some(o)=self.source.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<RetainedCloneSource<M::Diff>>>()))}else{o.next_close_demand(body)};}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<M::Diff>(false,&None,body);}
  if self.diff.is_some(){return self.retirement.next_owner_demands::<M::Diff>(true,&None,body);}
  if let Some(o)=self.inverse_close.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<FactoryVectorRetirement<M>>>()))}else{o.demands(mutations,body)};}
  if self.inverse.is_some(){return Ok(FactoryVectorRetirement::<M>::constructor_demand());}
  if let Some(o)=self.join.as_ref(){return if o.terminal_is_empty(){Ok(scalar(size_of::<Option<MutationMessageConcat<2>>>()))}else{o.close_demand()};}
  if self.messages.iter().any(Option::is_some){return Ok(MutationMessageConcat::<2>::constructor_demand());}
  Ok(Default::default())
 }
 fn owner_close_step(&mut self,mutations:&dyn ArtifactOwnedValueRetirementFactory<M>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(fault("retained mutation edit closure not started"));}if self.issuer.is_some_and(|i|!i.matches(mutations)){return Err(fault("retained mutation edit original issuer changed"));}let d=self.close_demands(mutations,grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let moved=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if let Some(o)=self.semantics.as_mut(){if o.terminal_is_empty(){drop(self.semantics.take());return Ok(moved());}return o.close_step(Self::context(mutations),grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(o)=self.plan.as_mut(){if o.terminal_is_empty(){drop(self.plan.take());return Ok(moved());}return o.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(o)=self.source.as_mut(){if o.terminal_is_empty(){drop(self.source.take());return Ok(moved());}return o.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.diff,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(o)=self.inverse_close.as_mut(){if o.terminal_is_empty(){self.inverse_close=None;return Ok(moved());}return o.close_step(mutations,grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(rows)=self.inverse.take(){return match FactoryVectorRetirement::admit(rows,grant){Ok((o,p))=>{self.inverse_close=Some(o);Ok(RetainedCloneStep::Progress(p))},Err((e,rows))=>{*self.inverse=Some(rows);Err(e)}};}
  if let Some(o)=self.join.as_mut(){if o.terminal_is_empty(){drop(self.join.take());return Ok(moved());}return o.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if self.messages.iter().any(Option::is_some){let batches=std::array::from_fn(|i|self.messages[i].take().unwrap_or_default());return match MutationMessageConcat::admit(batches,grant){Ok((mut o,p))=>{o.begin_close();*self.join=Some(o);Ok(RetainedCloneStep::Progress(p))},Err((e,batches))=>{for(i,b)in batches.into_iter().enumerate(){self.messages[i]=Some(b);}Err(e)}};}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn owner_terminal_is_empty(&self)->bool{self.semantics.is_none()&&self.plan.is_none()&&self.source.is_none()&&self.diff.is_none()&&self.inverse.is_none()&&self.inverse_close.is_none()&&self.join.is_none()&&self.messages.iter().all(Option::is_none)&&self.retirement.is_empty()}
}
impl<S:SnapshotProjection,M:RetainedMutation<S>> Drop for MutationApplyEdit<S,M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"retained mutation edit requires complete original ownership closure");if empty{unsafe{ManuallyDrop::drop(&mut self.semantics);ManuallyDrop::drop(&mut self.diff);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.plan);ManuallyDrop::drop(&mut self.inverse);ManuallyDrop::drop(&mut self.messages);ManuallyDrop::drop(&mut self.join);}}}}
/// 🏭️ Every mounted mutation family supplies required retained semantics and a typed central apply plan.
pub fn mutation_apply_preparation_factory<S,M>()->Arc<dyn ArtifactStoreOneItemPreparationFactory<S,M>>where S:SnapshotProjection<Output=S>,M:RetainedMutation<S>+ArtifactCanonicalJsonTree{Arc::new(PagedOneItemPreparationFactory::<S,M,MutationApplyEdit<S,M>>::default())}
