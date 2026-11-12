//! 🪆️ All authored row categories share diagnostics, inverse issuance and original-owner cancellation.
use crate::{snapshot::original::EnergyModelSnapshot,diff::original::EnergyModelDiff,row::RowOutcome,scalar::{invalid,scalar,child,add,receipt},custody::{self,Cursor,Held,Bound,Ledger}};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use std::{mem::{size_of,ManuallyDrop},sync::Arc};
use protocol::{Mutation,MutationMessage,retained_mutation_demand_fits,RetainedMutationLeafPreparation,RetainedMutationCloseContext,RetainedMutationDecision,RetainedDiagnosticPreparation,RetainedDiagnosticSpec};
use crate::retained::{Done,Flight,Reply,conclude};
use semio_framework_value::factory::vector::FactoryVectorRetirement;
type Quote=(RetirementDemand,u64);
/// 🧩️ A category lends its typed original field cursor to the shared semantic leaf lifecycle.
pub trait RowFieldPreparation:Cursor+Send+'static{
 type Input:Send+Sync;type Inverse:Send;
 fn new()->Self;
 fn check(&self,input:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>;
 fn quote(&self,input:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>;
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn ready(&self)->bool;fn outcome(&self)->Option<RowOutcome>;
 fn reference_index(&self)->Option<usize>{None}
 fn changed_diagnostic(&self)->Option<RetainedDiagnosticSpec>{None}
 fn take_quote(&self)->Result<Quote,ValueError>;
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>;
 fn inverse_count(&self)->Result<usize,ValueError>{Ok(1)}
 fn inverse_quote(&self,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>;
 fn prepare_inverse_reply<'r>(&mut self,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.inverse_quote(body,maximum_work_units))};
  custody::gated(effect,reply,quote,maximum_work_units,|_,_|Ok(RetainedCloneStep::Complete(receipt(scalar(1)))))
 }
 fn inverse_ready(&self)->bool;
 fn take_inverse_quote(&self)->Result<Quote,ValueError>;
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self){Cursor::mark(self)}
}
/// 🏷️ Schema leaves preserve their actual input/key projections, diagnostic policy and inverse constructor.
pub trait RowLeafSpec<L,M,const N:usize=1>:Send+Sync+'static{
 type Input:Send+Sync;type Inverse:Send;type Core:RowFieldPreparation<Input=Self::Input,Inverse=Self::Inverse>;
 const INVALID:RetainedDiagnosticSpec;const INVALID_REFERENCE:RetainedDiagnosticSpec=Self::INVALID;const INVALID_CONTEXT:RetainedDiagnosticSpec=Self::INVALID;const DUPLICATE:RetainedDiagnosticSpec=Self::INVALID;const REFERENCED:RetainedDiagnosticSpec=Self::INVALID;
 const UNCHANGED:RetainedDiagnosticSpec;const MISSING:RetainedDiagnosticSpec;
 fn input(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,Self::Input>;
 fn key(payload:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,u32>>;
 fn inverse(payload:RetainedCloneRef<'_,L>,value:Self::Inverse)->M;
 fn diagnostic_targets(payload:RetainedCloneRef<'_,L>,outcome:RowOutcome)->Result<[protocol::RetainedDiagnosticTarget<'_>;N],ValueError>;
 fn diagnostic_targets_from_core<'a>(payload:RetainedCloneRef<'a,L>,outcome:RowOutcome,_:&Self::Core)->Result<[protocol::RetainedDiagnosticTarget<'a>;N],ValueError>{Self::diagnostic_targets(payload,outcome)}
}
/// 🧳️ Diagnostics and inverse rows keep the exact original factory through every denied or cancelled turn.
pub struct RowLeafPreparation<L:Send+Sync+'static,M:Mutation<EnergyModelSnapshot,Diff=EnergyModelDiff>+Send+'static,S:RowLeafSpec<L,M,N>,const N:usize=1>{
 core:S::Core,invalid:Option<Box<RetainedDiagnosticPreparation<N>>>,unchanged:Option<Box<RetainedDiagnosticPreparation<0>>>,messages:Ledger,diff:Held<EnergyModelDiff>,
 inverse:Inverse<M>,diff_source:Bound,flight:Flight,
 body:Option<usize>,inverse_count:Option<usize>,phase:u8,inverse_phase:u8,closing:bool,marker:std::marker::PhantomData<fn()->(L,S)>,
}
struct Inverse<M:Send+'static>{rows:ManuallyDrop<Option<Vec<M>>>,close:Option<FactoryVectorRetirement<M>>}
impl<M:Send+'static> Default for Inverse<M>{fn default()->Self{Self{rows:ManuallyDrop::new(None),close:None}}}
impl<M:Send+'static> Drop for Inverse<M>{fn drop(&mut self){let empty=self.rows.is_none()&&self.close.is_none();assert!(std::thread::panicking()||empty,"native Energy inverse rows retain custody");if empty{unsafe{ManuallyDrop::drop(&mut self.rows);}}}}
struct InverseClose<'a,M:Send+'static>{inverse:&'a mut Inverse<M>,mutations:&'a Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>}
impl<M:Send+'static> Cursor for InverseClose<'_,M>{
 fn finished(&self)->bool{self.inverse.rows.is_none()&&self.inverse.close.is_none()}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.inverse.close.as_ref(){Some(close)=>close.demands(self.mutations,body),None=>Ok(FactoryVectorRetirement::<M>::constructor_demand())}}
 fn work(&self,body:usize)->Result<u64,ValueError>{match self.inverse.close.as_ref(){Some(close)=>close.close_work_demand(self.mutations,body),None=>Ok(1)}}
 fn birth(&self)->bool{self.inverse.close.as_ref().is_some_and(|c|c.requires_original_birth())}
 fn control(&self)->bool{self.inverse.close.as_ref().is_some_and(|c|c.requires_original_control())}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{match self.inverse.close.as_mut(){Some(c)=>c.close_original_birth_step(o,g),None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,_:bool)->Result<Done<'r>,ValueError>{
  let poll=match self.inverse.close.as_mut(){
   None=>{let poll=FactoryVectorRetirement::admit(&mut self.inverse.rows,&mut self.inverse.close,e);return match poll{RetainedWorkPoll::Complete(turn)=>Ok(r.complete(turn.result.map_err(Into::into))),RetainedWorkPoll::Pending{..}=>Ok(r.pending())}},
   Some(close)=>match o{Some(o)=>close.close_original_step(o,body,e),None=>close.close_step(self.mutations,body,e)},
  };
  match poll{RetainedWorkPoll::Complete(turn)=>{if self.inverse.close.as_ref().is_some_and(|c|c.terminal_is_empty()){self.inverse.close=None;}Ok(r.complete(turn.result))},RetainedWorkPoll::Pending{..}=>Ok(r.pending())}
 }
}
struct InverseView<'a,M:Send+'static>{inverse:&'a Inverse<M>,mutations:Option<&'a Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>>}
impl<M:Send+'static> Cursor for InverseView<'_,M>{
 fn finished(&self)->bool{self.inverse.rows.is_none()&&self.inverse.close.is_none()}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.inverse.close.as_ref(){Some(close)=>close.demands(self.mutations.ok_or_else(invalid)?,body),None=>Ok(FactoryVectorRetirement::<M>::constructor_demand())}}
 fn work(&self,body:usize)->Result<u64,ValueError>{match self.inverse.close.as_ref(){Some(close)=>close.close_work_demand(self.mutations.ok_or_else(invalid)?,body),None=>Ok(1)}}
 fn birth(&self)->bool{self.inverse.close.as_ref().is_some_and(|c|c.requires_original_birth())}
 fn control(&self)->bool{self.inverse.close.as_ref().is_some_and(|c|c.requires_original_control())}
 fn birth_step(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:RetainedCloneGrant)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn close<'r>(&mut self,_:Option<&mut OriginalRetirementCloseLoan<'_>>,_:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>,_:bool)->Result<Done<'r>,ValueError>{Err(invalid())}
}
impl<L:Send+Sync+'static,M:Mutation<EnergyModelSnapshot,Diff=EnergyModelDiff>+Send+'static,S:RowLeafSpec<L,M,N>,const N:usize> RowLeafPreparation<L,M,S,N>{
 fn view<R>(&self,mutations:Option<&Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>>,f:impl FnOnce(&[&dyn Cursor])->R)->R{let inverse=InverseView{inverse:&self.inverse,mutations};f(&[&inverse,&self.invalid,&self.unchanged,&self.messages,&self.diff,&self.core,&self.diff_source])}
 fn check(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{self.core.check(S::input(payload),S::key(payload),original)}
 fn diagnostic_quote(&self,payload:RetainedCloneRef<'_,L>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{let outcome=self.core.outcome().ok_or_else(invalid)?;match outcome{RowOutcome::Unchanged=>{let p=self.unchanged.as_ref().ok_or_else(invalid)?;Ok((p.demand([],body,maximum_work_units)?,p.advance_work_demand([],body,maximum_work_units)?))},_=>{let p=self.invalid.as_ref().ok_or_else(invalid)?;let t=S::diagnostic_targets_from_core(payload,outcome,&self.core)?;Ok((p.target_demand(t,body,maximum_work_units)?,p.advance_targets_work_demand(S::diagnostic_targets_from_core(payload,outcome,&self.core)?,body,maximum_work_units)?))}}}
 fn decision_quote(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{if self.closing||self.phase>=6||!self.flight.idle(){return Err(invalid());}self.check(payload,original)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}match self.phase{
  0=>{let(d,w)=self.core.quote(S::input(payload),S::key(payload),original,body.saturating_sub(1),maximum_work_units)?;Ok((nest(d,1+if self.body.is_none(){size_of::<usize>()}else{0})?,w.saturating_add(1)))},
  1=>{let bytes=match self.core.outcome().ok_or_else(invalid)?{RowOutcome::Unchanged=>size_of::<RetainedDiagnosticPreparation<0>>(),_=>size_of::<RetainedDiagnosticPreparation<N>>()};Ok((RetirementDemand{copy_bytes:2*bytes+2*size_of::<Option<Box<RetainedDiagnosticPreparation<N>>>>()+1,capacity_bytes:bytes,depth:1,..Default::default()},u64::try_from(bytes).map_err(|_|invalid())?.saturating_add(1)))},
  2=>Ok((nest(RetainedDiagnosticPreparation::<0>::start_demand(),1)?,1)),
  3=>{let(d,w)=self.diagnostic_quote(payload,body.saturating_sub(1),maximum_work_units)?;Ok((nest(d,1)?,w))},
  4=>{let take=if let Some(p)=self.invalid.as_ref(){p.take_work_demand()?}else{self.unchanged.as_ref().ok_or_else(invalid)?.take_work_demand()?};Ok((nest(scalar(size_of::<Vec<MutationMessage>>()),2*size_of::<Option<Vec<MutationMessage>>>()+1)?,take))},
  5=>if self.core.outcome()==Some(RowOutcome::Changed){let(d,w)=self.core.take_quote()?;Ok((nest(d,2*size_of::<Option<EnergyModelDiff>>()+2*size_of::<Option<Vec<MutationMessage>>>()+1)?,w))}else{Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+size_of::<EnergyModelDiff>()+1),1))},
  _=>Err(invalid()),
 }}
 fn inverse_quote(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,diff:RetainedCloneRef<'_,EnergyModelDiff>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=7||self.inverse_phase>=5||!self.flight.idle(){return Err(invalid());}self.check(payload,original)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}diff.check_binding(&self.diff_source.0)?;if self.diff_source.0.is_none(){return Ok((scalar(diff.binding_copy_bytes()),diff.binding_work_demand(&self.diff_source.0)?));}match self.inverse_phase{
  0=>Ok((scalar(size_of::<Option<Vec<M>>>()+2*size_of::<Option<usize>>()+1),1)),
  1=>Ok((RetirementDemand{copy_bytes:2*size_of::<Option<Vec<M>>>()+1,capacity_bytes:self.inverse_count.ok_or_else(invalid)?.checked_mul(size_of::<M>()).ok_or_else(invalid)?,depth:1,..Default::default()},1)),
  2=>{let(d,w)=self.core.inverse_quote(body.saturating_sub(1),maximum_work_units)?;Ok((nest(d,1)?,w))},
  3=>{let(d,w)=self.core.take_inverse_quote()?;Ok((nest(d,2*size_of::<M>()+size_of::<S::Inverse>()+2*size_of::<usize>()+2*size_of::<Vec<M>>()+1)?,w.saturating_add(1)))},
  _=>Err(invalid()),
 }}
}
fn nest(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
impl<L:Send+Sync+'static,M:Mutation<EnergyModelSnapshot,Diff=EnergyModelDiff>+Send+'static,S:RowLeafSpec<L,M,N>,const N:usize> RetainedMutationLeafPreparation<EnergyModelSnapshot,M,L> for RowLeafPreparation<L,M,S,N>{
 fn new()->Self{Self{core:S::Core::new(),invalid:None,unchanged:None,messages:Default::default(),diff:Default::default(),inverse:Default::default(),diff_source:Default::default(),flight:Default::default(),body:None,inverse_count:None,phase:0,inverse_phase:0,closing:false,marker:Default::default()}}
 fn birth_work()->Result<u64,ValueError>{Ok(1)}
 fn decision_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{Ok(self.decision_quote(payload,original,body,maximum_work_units)?.0)}
 fn decision_work_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{Ok(self.decision_quote(payload,original,body,maximum_work_units)?.1)}
 fn decide_reply<'r>(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.decision_quote(payload,original,body,maximum_work_units)?;
    if u==0||u>maximum_work_units||effect.remaining_work_units()?<u64::from(u)||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();
   match self.phase{
    0=>{let nested_body=body.saturating_sub(1);
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| self.core.advance_reply(S::input(payload),S::key(payload),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1+body_birth)),r,false),self.core.advance_reply(S::input(payload),S::key(payload),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1+body_birth)),r,true));
     let step=step?;p=add(step.progress(),1+body_birth)?;
     if self.core.ready(){self.phase=if self.core.outcome()==Some(RowOutcome::Changed)&&self.core.changed_diagnostic().is_none(){5}else{1};}},
    1=>{match self.core.outcome().ok_or_else(invalid)?{RowOutcome::Unchanged=>self.unchanged=Some(Box::new(Default::default())),RowOutcome::Invalid|RowOutcome::InvalidReference|RowOutcome::InvalidContext|RowOutcome::Duplicate|RowOutcome::Referenced|RowOutcome::Missing|RowOutcome::Changed=>self.invalid=Some(Box::new(Default::default()))}self.phase=2;p=receipt(units_demand(units));},
    2=>{let outcome=self.core.outcome().ok_or_else(invalid)?;
     p=add(if let Some(prep)=self.invalid.as_mut(){prep.start(match outcome{RowOutcome::Changed=>self.core.changed_diagnostic().ok_or_else(invalid)?,RowOutcome::Missing=>S::MISSING,RowOutcome::InvalidReference=>S::INVALID_REFERENCE,RowOutcome::InvalidContext=>S::INVALID_CONTEXT,RowOutcome::Duplicate=>S::DUPLICATE,RowOutcome::Referenced=>S::REFERENCED,_=>S::INVALID},child(g,1))}else{self.unchanged.as_mut().ok_or_else(invalid)?.start(S::UNCHANGED,child(g,1))}?,1)?;self.phase=3;},
    3=>{let outcome=self.core.outcome().ok_or_else(invalid)?;let nested_body=body.saturating_sub(1);
     let step=protocol::retained_routed!(self.flight,effect,recover,2,body,maximum_work_units,None,work,|r| if let Some(prep)=self.invalid.as_mut(){prep.advance_targets_into(S::diagnostic_targets_from_core(payload,outcome,&self.core)?,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r)}else{self.unchanged.as_mut().ok_or_else(invalid)?.advance_into([],nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r)},if let Some(prep)=self.invalid.as_mut(){prep.recover_advance_targets_into(S::diagnostic_targets_from_core(payload,outcome,&self.core)?,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r)}else{self.unchanged.as_mut().ok_or_else(invalid)?.recover_advance_into([],nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r)});
     let step=step?;p=add(step.progress(),1)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=4;}},
    4=>{let extra=2*size_of::<Option<Vec<MutationMessage>>>()+1;let turn=if let Some(prep)=self.invalid.as_mut(){prep.take(child(g,extra))}else{self.unchanged.as_mut().ok_or_else(invalid)?.take(child(g,extra))};work+=turn.work;let(messages,progress)=turn.result?.ok_or_else(invalid)?;self.messages.set(messages);p=add(progress,extra)?;self.phase=5;},
    5=>{if self.core.outcome()==Some(RowOutcome::Changed){let extra=2*size_of::<Option<EnergyModelDiff>>()+2*size_of::<Option<Vec<MutationMessage>>>()+1;let(diff,progress)=self.core.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.diff.set(diff);if !self.messages.is_some(){self.messages.set(Vec::new());}p=add(progress,extra)?;}else if self.core.outcome()==Some(RowOutcome::Unchanged){self.diff.set(Default::default());}self.phase=6;},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy row leaf decision")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==6{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn take_decision(&mut self)->RetainedMutationDecision<EnergyModelDiff>{
  if self.phase!=6||self.closing{return RetainedMutationDecision::Refused(vec![MutationMessage::fatal("mutation.invariant","The native Energy row decision is not ready.")]);}
  self.phase=7;let messages=self.messages.take().unwrap_or_default();
  if matches!(self.core.outcome(),Some(RowOutcome::Invalid|RowOutcome::InvalidReference|RowOutcome::InvalidContext|RowOutcome::Duplicate|RowOutcome::Referenced|RowOutcome::Missing)){RetainedMutationDecision::Refused(messages)}else{match self.diff.take(){Some(diff)=>RetainedMutationDecision::Admitted(diff,messages),None=>RetainedMutationDecision::Refused(vec![MutationMessage::fatal("mutation.invariant","The native Energy row decision lost its diff.")])}}
 }
 fn inverse_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,diff:RetainedCloneRef<'_,EnergyModelDiff>,body:usize,m:u64)->Result<RetirementDemand,ValueError>{Ok(Self::inverse_quote(self,payload,original,diff,body,m)?.0)}
 fn inverse_work_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,diff:RetainedCloneRef<'_,EnergyModelDiff>,body:usize,m:u64)->Result<u64,ValueError>{Ok(Self::inverse_quote(self,payload,original,diff,body,m)?.1)}
 fn inverse_reply<'r>(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,diff:RetainedCloneRef<'_,EnergyModelDiff>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=Self::inverse_quote(self,payload,original,diff,body,maximum_work_units)?;
    if u==0||u>maximum_work_units||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
    if self.diff_source.0.is_none(){let turn=diff.bind(&mut self.diff_source.0,&mut effect.nested(g));work+=turn.work;let p=turn.result?.ok_or_else(invalid)?;work=work.max(u128::from(units));return Ok(RetainedCloneStep::Progress(p))}
   }
   let mut p=RetainedCloneProgress::default();
   match self.inverse_phase{
    0=>if self.core.outcome()==Some(RowOutcome::Changed){let count=self.core.inverse_count()?;if count==0{return Err(invalid().into());}count.checked_mul(size_of::<M>()).ok_or_else(invalid)?;self.inverse_count=Some(count);self.inverse_phase=1;p=receipt(units_demand(units));}else{*self.inverse.rows=Some(Vec::new());self.inverse_phase=5;p=receipt(units_demand(units));},
    1=>{if effect.actual().is_some(){return Err(ValueError::invariant("original inverse allocation marker is occupied").into())}let count=self.inverse_count.ok_or_else(invalid)?;let bytes=semio_framework_value::retained_clone::allocation::backing::exact_vector_bytes::<M>(count)?;if bytes!=0{*effect.actual()=Some(0);}let rows=semio_framework_value::retained_clone::allocation::backing::allocate_exact_vector::<M>(count)?;if bytes!=0{*effect.actual()=Some(bytes);}*self.inverse.rows=Some(rows);p=receipt(RetirementDemand{copy_bytes:2*size_of::<Option<Vec<M>>>()+1,capacity_bytes:bytes,depth:1,..Default::default()});self.inverse_phase=2;},
    2=>{let nb=body.saturating_sub(1);
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| self.core.prepare_inverse_reply(nb,maximum_work_units,&mut effect.nested(child(g,1)),r,false),self.core.prepare_inverse_reply(nb,maximum_work_units,&mut effect.nested(child(g,1)),r,true));
     p=add(step?.progress(),1)?;if self.core.inverse_ready(){self.inverse_phase=3;}},
    3=>{let count=self.inverse_count.ok_or_else(invalid)?;let rows=self.inverse.rows.as_ref().ok_or_else(invalid)?;if rows.len()>=count||rows.capacity()<count{return Err(invalid().into());}let extra=2*size_of::<M>()+size_of::<S::Inverse>()+2*size_of::<usize>()+2*size_of::<Vec<M>>()+1;let(value,progress)=self.core.take_inverse(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;let rows=self.inverse.rows.as_mut().ok_or_else(invalid)?;rows.push(S::inverse(payload,value));self.inverse_phase=if rows.len()==count{5}else{2};p=add(progress,extra)?;},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy row leaf inverse rows")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.inverse_phase==5{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn take_inverse(&mut self)->Vec<M>{if self.inverse_phase!=5||self.closing{return Vec::new();}self.inverse_phase=6;self.inverse.rows.take().unwrap_or_default()}
 fn begin_close(&mut self){self.closing=true;self.core.begin_close();if let Some(p)=self.invalid.as_mut(){p.begin_close();}if let Some(p)=self.unchanged.as_mut(){p.begin_close();}}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->RetirementDemand{custody::begin_demand(self.closing)}
 fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{let mut closing=self.closing;let turn=custody::begin_paid(&mut closing,effect);if closing&&!self.closing{<Self as RetainedMutationLeafPreparation<EnergyModelSnapshot,M,L>>::begin_close(self);}turn}
 fn close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}let body=self.body.unwrap_or(body);self.view(Some(context.mutations),|chain|custody::chain_demand(chain,body))}
 fn close_work_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid());}let body=self.body.unwrap_or(body);self.view(Some(context.mutations),|chain|custody::chain_work(chain,body))}
 fn requires_original_birth(&self)->bool{self.closing&&self.flight.idle()&&self.view(None,|chain|custody::chain_birth(&self.flight,chain))}
 fn requires_original_control(&self)->bool{self.closing&&self.view(None,|chain|custody::chain_control(&self.flight,chain))}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{let Self{core,invalid,unchanged,messages,diff,diff_source,..}=self;custody::chain_birth_step(&mut [&mut *invalid as &mut dyn Cursor,&mut *unchanged,&mut *messages,&mut *diff,&mut *core,&mut *diff_source],original,grant)}
 fn close_reply<'r>(&mut self,context:RetainedMutationCloseContext<'_,M>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if !self.closing{return Err(invalid());}
  let body=self.body.unwrap_or(body);let Self{core,invalid,unchanged,messages,diff,diff_source,inverse,flight,..}=self;
  let mut inverse=InverseClose{inverse,mutations:context.mutations};
  custody::close_chain(flight,&mut [&mut inverse as &mut dyn Cursor,&mut *invalid,&mut *unchanged,&mut *messages,&mut *diff,&mut *core,&mut *diff_source],original,body,effect,reply,recover)
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.flight.idle()&&self.inverse.rows.is_none()&&self.inverse.close.is_none()&&self.invalid.is_none()&&self.unchanged.is_none()&&self.messages.finished()&&self.diff.finished()&&self.core.finished()&&self.diff_source.finished()}
}
fn units_demand(units:u64)->RetirementDemand{RetirementDemand{copy_bytes:usize::try_from(units).unwrap_or(usize::MAX).min(1),depth:1,..Default::default()}}
