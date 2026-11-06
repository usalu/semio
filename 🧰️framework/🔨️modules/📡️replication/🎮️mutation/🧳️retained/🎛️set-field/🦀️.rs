//! 🎛️ Generic retained leaf of a "set one field" mutation: the authored value is copied under physical grants into a one-field sparse patch, the displaced original value into the one-row inverse. See [`super`].
use super::super::{ApplyCapability,Mutation,MutationMessage};
use super::{retained_mutation_demand_fits,RetainedDiff,RetainedDiffApply,RetainedDiffFragment,RetainedDiffPreflightAdvance,RetainedFieldPreparation,RetainedMutationAdvance,RetainedMutationCloseContext,RetainedMutationDecision,RetainedMutationLeafPreparation,RetainedRowPatch,RetainedRowPatchApply};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{admit_retained_clone_progress,RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::{size_of,ManuallyDrop};

/// 🧷️ The aggregate owner quotes and performs the exact leaf-to-enum handoff.
pub trait RetainedLeafWrapper<L>:Sized{fn wrap_work_demand()->u64;fn wrap(value:L)->(Self,u64);}

/// 🪡️ A leaf that sets one field of the snapshot `R` names its authored value, the original value it displaces, the sparse patch it yields and its exact inverse.
pub trait RetainedSetField<R,P,M>:Send+Sync+Sized+'static where P:RetainedRowPatch<R>+RetainedDiff<R>,M:Mutation<R,Diff=P>+Send+'static{
 type Authored:RetainedClone+Send+Sync+'static;
 type Displaced:RetainedClone+Send+Sync+'static;
 fn source(payload:RetainedCloneRef<'_,Self>)->RetainedCloneRef<'_,Self::Authored>;
 fn original(original:RetainedCloneRef<'_,R>)->RetainedCloneRef<'_,Self::Displaced>;
 fn patch_work_demand(value:&Self::Authored)->Result<u64,ValueError>;
 fn patch(value:Self::Authored)->(P,u64);
 fn inverse_work_demand(value:&Self::Displaced)->Result<u64,ValueError>;
 fn inverse(value:Self::Displaced)->(M,u64);
}

/// 🧳️ Authored and displaced field custody, the yielded patch and the inverse row survive every denied or cancelled turn.
pub struct RetainedSetFieldLeaf<L,R,P,M> where L:RetainedSetField<R,P,M>,P:RetainedRowPatch<R>+RetainedDiff<R>,M:Mutation<R,Diff=P>+Send+'static{
 payload:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,diff_source:Option<RetainedCloneBinding>,
 authored:Option<Box<RetainedFieldPreparation<L::Authored>>>,displaced:Option<Box<RetainedFieldPreparation<L::Displaced>>>,
 diff:ManuallyDrop<Option<P>>,inverse:ManuallyDrop<Option<Vec<M>>>,inverse_close:Option<FactoryVectorRetirement<M>>,
 retirement:RetainedCloneClose,phase:u8,inverse_phase:u8,closing:bool,marker:std::marker::PhantomData<fn()->(L,R)>,
}

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"retained set-field leaf lost its source or phase")}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn add(mut p:RetainedCloneProgress,extra:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(p)}
fn tag(error:ValueError,extra:usize)->ValueError{let p=error.retained_progress();match add(p,extra){Ok(p)=>error.with_retained_progress(p),Err(failure)=>failure.with_retained_progress(p)}}
fn child(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(extra),..g}}
fn field_birth<T:RetainedClone>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<RetainedFieldPreparation<T>>()+2*size_of::<Option<Box<RetainedFieldPreparation<T>>>>(),capacity_bytes:size_of::<RetainedFieldPreparation<T>>(),depth:1,..Default::default()}}
fn field_release<T:RetainedClone>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Box<RetainedFieldPreparation<T>>>>(),release_bytes:size_of::<RetainedFieldPreparation<T>>(),depth:1,..Default::default()}}
fn field_demand<T:RetainedClone>(field:&Option<Box<RetainedFieldPreparation<T>>>,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,take_extra:usize)->Result<RetirementDemand,ValueError>{
 let Some(field)=field else{return Ok(field_birth::<T>())};
 if field.value().is_some(){let mut d=field.exchange_demand()?;d.copy_bytes=d.copy_bytes.checked_add(take_extra).ok_or_else(invalid)?;Ok(d)}else{field.demand(source,body,maximum_work_units)}
}
fn field_close_demand<T:RetainedClone>(field:&RetainedFieldPreparation<T>,body:usize)->Result<RetirementDemand,ValueError>{if field.terminal_is_empty(){Ok(field_release::<T>())}else{field.close_demand(body)}}
fn field_close_step<T:RetainedClone>(slot:&mut Option<Box<RetainedFieldPreparation<T>>>,d:RetirementDemand,g:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{
 let Some(field)=slot.as_mut()else{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
 if field.terminal_is_empty(){drop(slot.take());return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1};}
 let turn=field.close_step(g);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|s|RetainedCloneStep::Progress(s.progress())),work:turn.work}
}

impl<L,R,P,M> RetainedSetFieldLeaf<L,R,P,M> where L:RetainedSetField<R,P,M>,P:RetainedRowPatch<R>+RetainedDiff<R>,M:Mutation<R,Diff=P>+Send+'static{
 fn check(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,R>)->Result<(),ValueError>{L::source(payload).check_binding(&self.payload)?;original.check_binding(&self.original)?;Ok(())}
 fn take_extra_decision()->usize{2*size_of::<P>()+2*size_of::<Option<P>>()}
 fn take_extra_inverse()->usize{2*size_of::<M>()+2*size_of::<Vec<M>>()}
 fn first_binding(&self)->&Option<RetainedCloneBinding>{if self.diff_source.is_some(){&self.diff_source}else if self.original.is_some(){&self.original}else{&self.payload}}
 fn first_binding_mut(&mut self)->&mut Option<RetainedCloneBinding>{if self.diff_source.is_some(){&mut self.diff_source}else if self.original.is_some(){&mut self.original}else{&mut self.payload}}
}

impl<L,R,P,M> RetainedMutationLeafPreparation<R,M,L> for RetainedSetFieldLeaf<L,R,P,M> where L:RetainedSetField<R,P,M>,R:'static,P:RetainedRowPatch<R>+RetainedDiff<R>,M:Mutation<R,Diff=P>+Send+'static{
 fn new()->Self{Self{payload:None,original:None,diff_source:None,authored:None,displaced:None,diff:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),inverse_close:None,retirement:Default::default(),phase:0,inverse_phase:0,closing:false,marker:Default::default()}}
 fn decision_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=5{return Err(invalid());}
  self.check(payload,original)?;let source=L::source(payload);
  match self.phase{
   0=>Ok(scalar(source.binding_copy_bytes())),
   1=>Ok(scalar(original.binding_copy_bytes())),
   2=>Ok(field_birth::<L::Authored>()),
   3=>field_demand(&self.authored,source,body,maximum_work_units,0),
   4=>field_demand(&self.authored,source,body,maximum_work_units,Self::take_extra_decision()),
   _=>Err(invalid()),
  }
 }
 fn decision_work_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.decision_demand(payload,original,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}let source=L::source(payload);match self.phase{3=>{let field=self.authored.as_ref().ok_or_else(invalid)?;if field.value().is_some(){Ok(1)}else{field.advance_work_demand(source,body,maximum_work_units)}},4=>{let value=self.authored.as_ref().and_then(|field|field.value()).ok_or_else(invalid)?;L::patch_work_demand(value)?.checked_add(1).ok_or_else(invalid)},_=>Ok(1)}}
 fn decide(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedMutationAdvance,semio_framework_value::OriginalRetirementFailure>{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
  let selected=self.decision_work_demand(payload,original,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedMutationAdvance::Progress(Default::default()))}let d=self.decision_demand(payload,original,body,maximum_work_units)?;
  if !retained_mutation_demand_fits(d,g){return Ok(RetainedMutationAdvance::Progress(Default::default()));}
  let source=L::source(payload);let mut p=receipt(d);work=1;
  match self.phase{
   0=>{let turn=source.bind(&mut self.payload,g);work=turn.work;p=turn.result?.ok_or_else(invalid)?;self.phase=1;},
   1=>{let turn=original.bind(&mut self.original,g);work=turn.work;p=turn.result?.ok_or_else(invalid)?;self.phase=2;},
   2=>{self.authored=Some(Box::default());self.phase=3;},
   3=>{let field=self.authored.as_mut().ok_or_else(invalid)?;if field.value().is_some(){self.phase=4;}else{let turn=field.advance(source,body,maximum_work_units,g);work=turn.work;let step=turn.result?;p=step.progress();if field.value().is_some(){self.phase=4;}}},
   4=>{let extra=Self::take_extra_decision();let field=self.authored.as_mut().ok_or_else(invalid)?;let quoted=L::patch_work_demand(field.value().ok_or_else(invalid)?)?;let(value,progress)=field.take(child(g,extra)).map_err(|e|tag(e,extra))?;let(patch,callback_work)=L::patch(value.ok_or_else(invalid)?);work+=callback_work;*self.diff=Some(patch);p=add(progress,extra)?;self.phase=5;if callback_work>quoted{return Err(invalid().with_retained_progress(p).into())}},
   _=>return Err(invalid().into()),
  }
  admit_retained_clone_progress(g,p,"retained set-field leaf decision")?;
  Ok(if self.phase==5{RetainedMutationAdvance::Ready(p)}else{RetainedMutationAdvance::Progress(p)})
 })();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn take_decision(&mut self)->RetainedMutationDecision<P>{
  if self.phase!=5||self.closing{return RetainedMutationDecision::Refused(vec![MutationMessage::fatal("mutation.invariant","The retained set-field decision is not ready.")]);}
  self.phase=6;
  match self.diff.take(){Some(diff)=>RetainedMutationDecision::Admitted(diff,Vec::new()),None=>RetainedMutationDecision::Refused(vec![MutationMessage::fatal("mutation.invariant","The retained set-field decision lost its patch.")])}
 }
 fn inverse_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,R>,diff:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=6||self.inverse_phase>=4{return Err(invalid());}
  self.check(payload,original)?;diff.check_binding(&self.diff_source)?;
  if self.diff_source.is_none(){return Ok(scalar(diff.binding_copy_bytes()));}
  match self.inverse_phase{
   0=>Ok(field_birth::<L::Displaced>()),
   1=>field_demand(&self.displaced,L::original(original),body,maximum_work_units,0),
   2=>Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Vec<M>>>(),capacity_bytes:size_of::<M>(),depth:1,..Default::default()}),
   3=>field_demand(&self.displaced,L::original(original),body,maximum_work_units,Self::take_extra_inverse()),
   _=>Err(invalid()),
  }
 }
 fn inverse_work_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,R>,diff:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.inverse_demand(payload,original,diff,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.diff_source.is_none(){return Ok(1)}match self.inverse_phase{1=>{let field=self.displaced.as_ref().ok_or_else(invalid)?;if field.value().is_some(){Ok(1)}else{field.advance_work_demand(L::original(original),body,maximum_work_units)}},3=>{let value=self.displaced.as_ref().and_then(|field|field.value()).ok_or_else(invalid)?;L::inverse_work_demand(value)?.checked_add(1).ok_or_else(invalid)},_=>Ok(1)}}
 fn inverse(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,R>,diff:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedMutationAdvance,semio_framework_value::OriginalRetirementFailure>{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
  let selected=self.inverse_work_demand(payload,original,diff,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedMutationAdvance::Progress(Default::default()))}let d=self.inverse_demand(payload,original,diff,body,maximum_work_units)?;
  if !retained_mutation_demand_fits(d,g){return Ok(RetainedMutationAdvance::Progress(Default::default()));}
  let turn=diff.bind(&mut self.diff_source,g);work+=turn.work;if let Some(p)=turn.result?{return Ok(RetainedMutationAdvance::Progress(p));}
  let mut p=receipt(d);work+=1;
  match self.inverse_phase{
   0=>{self.displaced=Some(Box::default());self.inverse_phase=1;},
   1=>{let field=self.displaced.as_mut().ok_or_else(invalid)?;if field.value().is_some(){self.inverse_phase=2;}else{let turn=field.advance(L::original(original),body,maximum_work_units,g);work=turn.work;let step=turn.result?;p=step.progress();if field.value().is_some(){self.inverse_phase=2;}}},
   2=>{*self.inverse=Some(Vec::with_capacity(1));p.retained_capacity_bytes=self.inverse.as_ref().ok_or_else(invalid)?.capacity()*size_of::<M>();self.inverse_phase=3;},
   3=>{let extra=Self::take_extra_inverse();let field=self.displaced.as_mut().ok_or_else(invalid)?;let quoted=L::inverse_work_demand(field.value().ok_or_else(invalid)?)?;let(value,progress)=field.take(child(g,extra)).map_err(|e|tag(e,extra))?;let(inverse,callback_work)=L::inverse(value.ok_or_else(invalid)?);work+=callback_work;self.inverse.as_mut().ok_or_else(invalid)?.push(inverse);p=add(progress,extra)?;self.inverse_phase=4;if callback_work>quoted{return Err(invalid().with_retained_progress(p).into())}},
   _=>return Err(invalid().into()),
  }
  admit_retained_clone_progress(g,p,"retained set-field leaf inverse")?;
  Ok(if self.inverse_phase==4{RetainedMutationAdvance::Ready(p)}else{RetainedMutationAdvance::Progress(p)})
 })();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn take_inverse(&mut self)->Vec<M>{
  if self.inverse_phase!=4||self.closing{return Vec::new();}
  self.inverse_phase=5;self.inverse.take().unwrap_or_default()
 }
 fn begin_close(&mut self){self.closing=true;if let Some(f)=self.authored.as_mut(){f.begin_close();}if let Some(f)=self.displaced.as_mut(){f.begin_close();}}
 fn close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if let Some(f)=self.authored.as_ref(){return field_close_demand(f.as_ref(),body);}
  if let Some(f)=self.displaced.as_ref(){return field_close_demand(f.as_ref(),body);}
  if let Some(close)=self.inverse_close.as_ref(){return close.demands(context.mutations,body);}
  if self.inverse.is_some(){return Ok(FactoryVectorRetirement::<M>::constructor_demand());}
  if !self.retirement.is_empty()||self.diff.is_some(){return self.retirement.next_owner_demands::<P>(self.diff.is_some(),&None,body);}
  RetainedCloneBinding::close_demand(self.first_binding(),body)
 }
 fn close_work_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>{self.close_demand(context,body)?;
  if let Some(f)=self.authored.as_ref(){return if f.terminal_is_empty(){Ok(1)}else{f.close_work_demand(body)}}
  if let Some(f)=self.displaced.as_ref(){return if f.terminal_is_empty(){Ok(1)}else{f.close_work_demand(body)}}
  if let Some(close)=self.inverse_close.as_ref(){return close.close_work_demand(context.mutations,body)}
  if self.inverse.is_some(){return Ok(1)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<P>(false,&None,body)}
  if self.diff.is_some(){return self.retirement.next_owner_work::<P>(true,&None,body)}
  RetainedCloneBinding::close_work_demand(self.first_binding(),body)
 }
 fn close_step(&mut self,context:RetainedMutationCloseContext<'_,M>,g:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
  let d=self.close_demand(context,g.maximum_copy_bytes)?;
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.authored.is_some(){let turn=field_close_step(&mut self.authored,d,g);work+=turn.work;return turn.result;}
  if self.displaced.is_some(){let turn=field_close_step(&mut self.displaced,d,g);work+=turn.work;return turn.result;}
  if let Some(close)=self.inverse_close.as_mut(){let turn=close.close_step(context.mutations,g);work+=turn.work;let s=turn.result?;if close.terminal_is_empty(){self.inverse_close=None;}return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(rows)=self.inverse.take(){work+=1;return match FactoryVectorRetirement::admit(rows,g){Ok((close,p))=>{self.inverse_close=Some(close);Ok(RetainedCloneStep::Progress(p))},Err((e,rows))=>{*self.inverse=Some(rows);Err(e.into())}};}
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(g);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  let turn=self.retirement.begin_granted(&mut self.diff,g);work+=turn.work;if let Some(s)=turn.result?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  let turn=RetainedCloneBinding::close_one(self.first_binding_mut(),g);work+=turn.work;let s=turn.result?;
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})
 })();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.authored.is_none()&&self.displaced.is_none()&&self.diff.is_none()&&self.inverse.is_none()&&self.inverse_close.is_none()&&self.payload.is_none()&&self.original.is_none()&&self.diff_source.is_none()&&self.retirement.is_empty()}
}
impl<L,R,P,M> Drop for RetainedSetFieldLeaf<L,R,P,M> where L:RetainedSetField<R,P,M>,P:RetainedRowPatch<R>+RetainedDiff<R>,M:Mutation<R,Diff=P>+Send+'static{
 /// 🛡️ SAFETY: `diff` and `inverse` are dropped exactly once and only when every custody slot is empty, so no owner still refers to them and no later access reaches the dropped values.
 fn drop(&mut self){
  let empty=self.authored.is_none()&&self.displaced.is_none()&&self.diff.is_none()&&self.inverse.is_none()&&self.inverse_close.is_none()&&self.payload.is_none()&&self.original.is_none()&&self.diff_source.is_none()&&self.retirement.is_empty();
  debug_assert!(empty||std::thread::panicking(),"retained set-field leaf dropped holding custody: close it through begin_close/close_step");
  if empty{unsafe{ManuallyDrop::drop(&mut self.diff);ManuallyDrop::drop(&mut self.inverse);}}
 }
}

/// 🌳️ Central application of a sparse patch: the authored fields stage once, then exchange into the unpublished snapshot under the central capability.
pub struct RetainedPatchDiffApply<R,P:RetainedRowPatch<R>>{fields:RetainedRowPatchApply<R,P>,phase:u8}
fn apply_invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"retained patch application lost its staged phase")}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(1).ok_or_else(apply_invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(apply_invalid)?;Ok(d)}
fn apply_child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(1),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
impl<R:Send+Sync+'static,P> RetainedDiffApply<R,P> for RetainedPatchDiffApply<R,P> where P:RetainedRowPatch<R>+RetainedDiffFragment<R>{
 fn new()->Self{Self{fields:Default::default(),phase:0}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,P>,s:RetainedCloneRef<'_,R>,_:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.phase!=0{return Err(apply_invalid());}if maximum_work_units==0{return Ok(Default::default())}nested(self.fields.stage_demand(d,s,body.saturating_sub(1),maximum_work_units)?)}
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,P>,s:RetainedCloneRef<'_,R>,p:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.preflight_demand(d,s,p,body,maximum_work_units)?;self.fields.stage_work_demand(d,s,body.saturating_sub(1),maximum_work_units)}
 fn preflight(&mut self,d:RetainedCloneRef<'_,P>,s:RetainedCloneRef<'_,R>,p:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,_:&mut NativeDecodeControl<'_>)->semio_framework_value::RetainedWorkOutcome<RetainedDiffPreflightAdvance,semio_framework_value::OriginalRetirementFailure>{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
  let selected=self.preflight_work_demand(d,s,p,body,maximum_work_units)?;if selected==0{return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()))}let q=self.preflight_demand(d,s,p,body,maximum_work_units)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()))}
  let turn=self.fields.stage(d,s,body.saturating_sub(1),maximum_work_units,apply_child(g));work=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(1);error.with_retained_progress(p)})?;let mut progress=step.progress();progress.copied_bytes+=1;
  if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1;Ok(RetainedDiffPreflightAdvance::Ready(progress,true))}else{Ok(RetainedDiffPreflightAdvance::Progress(progress))}
 })();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,_:RetainedCloneRef<'_,P>,_:RetainedCloneRef<'_,R>,_:&R,_:usize)->Result<RetirementDemand,ValueError>{if self.phase!=1{return Err(apply_invalid());}nested(self.fields.exchange_demand()?)}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,P>,s:RetainedCloneRef<'_,R>,p:&R,body:usize)->Result<u64,ValueError>{self.commit_demand(d,s,p,body)?;self.fields.exchange_work_demand()}
 fn commit(&mut self,d:RetainedCloneRef<'_,P>,s:RetainedCloneRef<'_,R>,p:&mut R,g:RetainedCloneGrant,c:ApplyCapability)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
  let q=self.commit_demand(d,s,p,g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedCloneStep::Progress(Default::default()))}let turn=self.fields.exchange(p,c,apply_child(g));work=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(1);error.with_retained_progress(p)})?;let mut progress=step.progress();progress.copied_bytes+=1;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;Ok(RetainedCloneStep::Complete(progress))}else{Ok(RetainedCloneStep::Progress(progress))}
 })();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn begin_close(&mut self){self.fields.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.fields.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.fields.close_work_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{self.fields.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.fields.terminal_is_empty()}
}

/// 🏗️ Declares the central retained application of a sparse patch beside its snapshot (the patch also declares `retained_row_patch!`).
#[macro_export]
macro_rules! retained_patch_diff{
 ($patch:ty => $row:ty)=>{
  impl $crate::retained::RetainedDiff<$row> for $patch{type Apply=$crate::retained::set_field::RetainedPatchDiffApply<$row,$patch>;}
 }
}

/// 🏗️ Declares the retained leaf of a "set one field" mutation from its schema: payload field, value type, snapshot field and that field's position in the patch roster.
#[macro_export]
macro_rules! retained_set_field{
 ($leaf:ty => $snapshot:ty,$patch:ty,$mutation:ty {$payload:ident:$value:ty => $field:ident @ $index:expr})=>{
  impl $crate::retained::set_field::RetainedSetField<$snapshot,$patch,$mutation> for $leaf{
   type Authored=$value;
   type Displaced=$value;
   fn source(payload:$crate::value::retained_clone::RetainedCloneRef<'_,Self>)->$crate::value::retained_clone::RetainedCloneRef<'_,$value>{payload.project(0,|leaf|&leaf.$payload)}
   fn original(original:$crate::value::retained_clone::RetainedCloneRef<'_,$snapshot>)->$crate::value::retained_clone::RetainedCloneRef<'_,$value>{original.project($index,|row|&row.$field)}
   fn patch_work_demand(_:&$value)->Result<u64,$crate::value::ValueError>{<$patch as $crate::retained::patch::RetainedPatchEmpty>::empty_patch_work_demand().checked_add(1).ok_or_else(||$crate::value::ValueError::invariant("empty patch work exceeds the retained work range"))}
   fn patch(value:$value)->($patch,u64){let(mut patch,work)=<$patch as $crate::retained::patch::RetainedPatchEmpty>::empty_patch();patch.$field=::core::option::Option::Some(value);(patch,work+1)}
   fn inverse_work_demand(_:&$value)->Result<u64,$crate::value::ValueError>{<$mutation as $crate::retained::set_field::RetainedLeafWrapper<$leaf>>::wrap_work_demand().checked_add(1).ok_or_else(||$crate::value::ValueError::invariant("leaf wrapper work exceeds the retained work range"))}
   fn inverse(value:$value)->($mutation,u64){let leaf=Self{$payload:value};let(mutation,work)=<$mutation as $crate::retained::set_field::RetainedLeafWrapper<$leaf>>::wrap(leaf);(mutation,work+1)}
  }
  impl $crate::RetainedMutationLeaf<$snapshot,$mutation> for $leaf{type Preparation=$crate::retained::set_field::RetainedSetFieldLeaf<$leaf,$snapshot,$patch,$mutation>;}
 }
}

/// 🪞️ Aggregate-level retained preparation of a mutation that is its own single set-field leaf (the whole-root `Snapshot` mutation of a one-field transient state).
pub struct RetainedRootMutation<R,P,M> where M:RetainedSetField<R,P,M>+Mutation<R,Diff=P>+Send+'static,P:RetainedRowPatch<R>+RetainedDiff<R>{leaf:RetainedSetFieldLeaf<M,R,P,M>}
impl<R,P,M> super::RetainedMutationPreparation<R,M> for RetainedRootMutation<R,P,M> where M:RetainedSetField<R,P,M>+Mutation<R,Diff=P>+Send+'static,R:'static,P:RetainedRowPatch<R>+RetainedDiff<R>{
 fn new()->Self{Self{leaf:<RetainedSetFieldLeaf<M,R,P,M> as RetainedMutationLeafPreparation<R,M,M>>::new()}}
 fn decision_demand(&self,m:RetainedCloneRef<'_,M>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.leaf.decision_demand(m,o,body,maximum_work_units)}
 fn decision_work_demand(&self,m:RetainedCloneRef<'_,M>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.leaf.decision_work_demand(m,o,body,maximum_work_units)}
 fn decide(&mut self,m:RetainedCloneRef<'_,M>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedMutationAdvance,semio_framework_value::OriginalRetirementFailure>{self.leaf.decide(m,o,body,maximum_work_units,g)}
 fn take_decision(&mut self)->RetainedMutationDecision<P>{self.leaf.take_decision()}
 fn inverse_demand(&self,m:RetainedCloneRef<'_,M>,o:RetainedCloneRef<'_,R>,d:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.leaf.inverse_demand(m,o,d,body,maximum_work_units)}
 fn inverse_work_demand(&self,m:RetainedCloneRef<'_,M>,o:RetainedCloneRef<'_,R>,d:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.leaf.inverse_work_demand(m,o,d,body,maximum_work_units)}
 fn inverse(&mut self,m:RetainedCloneRef<'_,M>,o:RetainedCloneRef<'_,R>,d:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedMutationAdvance,semio_framework_value::OriginalRetirementFailure>{self.leaf.inverse(m,o,d,body,maximum_work_units,g)}
 fn take_inverse(&mut self)->Vec<M>{self.leaf.take_inverse()}
 fn begin_close(&mut self){self.leaf.begin_close()}
 fn close_demand(&self,c:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{self.leaf.close_demand(c,body)}
 fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>{self.leaf.close_work_demand(c,body)}
 fn close_step(&mut self,c:RetainedMutationCloseContext<'_,M>,g:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{self.leaf.close_step(c,g)}
 fn terminal_is_empty(&self)->bool{self.leaf.terminal_is_empty()}
}

/// 🏗️ Declares the retained leaf of a "set several fields" mutation whose authored record is cloned whole under grants and whose displaced original is the whole snapshot: the patch sets every field of the record, the inverse is built from the cloned original.
#[macro_export]
macro_rules! retained_set_record{
 ($leaf:ty => $snapshot:ty,$patch:ty,$mutation:ty {authored: $authored:ty = $source:expr, patch: $patch_fn:expr, patch_work: $patch_work:expr, inverse: $inverse_fn:expr, inverse_work: $inverse_work:expr})=>{
  impl $crate::retained::set_field::RetainedSetField<$snapshot,$patch,$mutation> for $leaf{
   type Authored=$authored;
   type Displaced=$snapshot;
   fn source(payload:$crate::value::retained_clone::RetainedCloneRef<'_,Self>)->$crate::value::retained_clone::RetainedCloneRef<'_,$authored>{payload.project(0,$source)}
   fn original(original:$crate::value::retained_clone::RetainedCloneRef<'_,$snapshot>)->$crate::value::retained_clone::RetainedCloneRef<'_,$snapshot>{original}
   fn patch_work_demand(value:&$authored)->Result<u64,$crate::value::ValueError>{($patch_work)(value)}
   fn patch(value:$authored)->($patch,u64){($patch_fn)(value)}
   fn inverse_work_demand(value:&$snapshot)->Result<u64,$crate::value::ValueError>{($inverse_work)(value)}
   fn inverse(value:$snapshot)->($mutation,u64){($inverse_fn)(value)}
  }
  impl $crate::RetainedMutationLeaf<$snapshot,$mutation> for $leaf{type Preparation=$crate::retained::set_field::RetainedSetFieldLeaf<$leaf,$snapshot,$patch,$mutation>;}
 }
}

/// 🏗️ Declares the retained mutation of a whole-root transient (`transient_root!` `retained`): the `Snapshot` mutation is its own set-record leaf; the patch sets every listed field.
#[macro_export]
macro_rules! retained_root_record{
 (state: $state:path, mutation: $mutation:path, diff: $diff:path, fields: {$($field:ident : $ty:ty),+ $(,)?})=>{
  $crate::retained_row_patch!(pub retained_root_fields for $diff => $state { $($field: $ty),+ });
  $crate::retained_patch_diff!($diff => $state);
  impl $crate::retained::set_field::RetainedSetField<$state,$diff,$mutation> for $mutation{
   type Authored=$state;
   type Displaced=$state;
   fn source(mutation:$crate::value::retained_clone::RetainedCloneRef<'_,Self>)->$crate::value::retained_clone::RetainedCloneRef<'_,$state>{mutation.project(0,|m|match m{Self::Snapshot{transient}=>transient})}
   fn original(original:$crate::value::retained_clone::RetainedCloneRef<'_,$state>)->$crate::value::retained_clone::RetainedCloneRef<'_,$state>{original}
   fn patch_work_demand(_:&$state)->Result<u64,$crate::value::ValueError>{Ok(<$diff as $crate::RetainedRowPatch<$state>>::FIELD_COUNT as u64)}
   fn patch(value:$state)->($diff,u64){($diff{$($field: ::core::option::Option::Some(value.$field)),+},<$diff as $crate::RetainedRowPatch<$state>>::FIELD_COUNT as u64)}
   fn inverse_work_demand(_:&$state)->Result<u64,$crate::value::ValueError>{Ok(1)}
   fn inverse(value:$state)->($mutation,u64){(Self::Snapshot{transient:value},1)}
  }
  impl $crate::RetainedMutation<$state> for $mutation{type Preparation=$crate::retained::set_field::RetainedRootMutation<$state,$diff,$mutation>;}
 }
}
