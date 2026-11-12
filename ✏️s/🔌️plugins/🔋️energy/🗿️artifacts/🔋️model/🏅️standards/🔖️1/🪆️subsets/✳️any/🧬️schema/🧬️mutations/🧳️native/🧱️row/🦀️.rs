//! 🧱️ A prepared original row replaces one persistent page path before its sealed model branch is published.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::{EntityRow,RetainedBranchProjection}},diff::original::EnergyModelDiff,custody::{self,Cursor,Held,Bound,Sourced},scalar::{Quote,invalid,scalar,nested,child,add,receipt,birth,bind}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,shared_issuance::SealedShared,retirement::RetireOwned,retained_clone::{RetainedCloneRef,RetainedCloneStep,RetainedCloneProgress,StagedSourceAdmission,admit_retained_clone_progress}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentSet,PersistentSplice,PersistentSpliceKind};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use protocol::{retained_mutation_demand_fits};
use crate::retained::{Done,Flight,Reply,conclude,native_child};
use std::mem::size_of;
#[path="📤️output/🦀️.rs"]mod output;pub use output::{BranchOutput,DirectBranchOutput,NestedBranchPath,NestedBranchOutput,ScheduleBranchOutput,SiteBranchOutput,GroundBranchOutput,RunPeriodBranchOutput};
#[path="📋️change/🦀️.rs"]mod change;pub use change::RowChange;pub use change::RowListPreparation;
/// 🧭️ The authored row path supplies the exact persistent collection and owning sealed branch.
pub trait RowBranchPath:Send+Sync+'static{
 type Row:RetireOwned+Send+Sync;
 type Branch:SnapshotProjection<Output=Box<Self::Branch>>+Send+Sync;
 fn rows(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>;
 fn branch(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>;
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>;
 type Output:crate::row::BranchOutput<Self::Branch>;
 fn output()->Self::Output;
}
/// 🔢️ Existing scalar schema paths use the same original row publication owner.
pub struct ScalarBranch<S>(std::marker::PhantomData<fn()->S>);
impl<S:crate::scalar::ScalarRowPath> RowBranchPath for ScalarBranch<S>{type Row=S::Row;type Branch=S::Branch;type Output=S::Output;fn rows(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{S::rows(o)}fn branch(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{S::branch(o)}fn rows_mut(b:&mut Self::Branch)->&mut PersistentList<Self::Row>{S::rows_mut(b)}fn output()->Self::Output{S::output()}}

type Exchange<'a,B>=&'a mut dyn FnMut(&mut RetainedBranchProjection<B>,&mut RetainedAllocationEffect<'_>)->Result<(bool,RetainedCloneProgress),ValueError>;
/// 🎭️ One branch is projected, has one field exchanged, is sealed and leaves as an unpublished sealed value; every prefix closes.
pub struct Staged<B:SnapshotProjection<Output:crate::snapshot::state::fields::BranchProjectionOutput<B>>+Send+Sync+RetireOwned>{stage:Option<Box<RetainedBranchProjection<B>>>,pub sealed:Held<SealedShared<B>>,flight:Flight,phase:u8,closing:bool}
impl<B:SnapshotProjection<Output:crate::snapshot::state::fields::BranchProjectionOutput<B>>+Send+Sync+RetireOwned> Default for Staged<B>{fn default()->Self{Self{stage:None,sealed:Default::default(),flight:Default::default(),phase:0,closing:false}}}
impl<B:SnapshotProjection<Output:crate::snapshot::state::fields::BranchProjectionOutput<B>>+Send+Sync+RetireOwned> Staged<B>{
 pub fn ready(&self)->bool{!self.closing&&self.phase==5}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(stage,sealed);
 pub fn quote<F>(&self,source:RetainedCloneRef<'_,B>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=5||!self.flight.idle(){return Err(invalid());}
  match self.phase{
   0=>Ok((birth::<RetainedBranchProjection<B>>(),1)),
   1|3=>{let s=self.stage.as_ref().ok_or_else(invalid)?;Ok((nested(s.demand(source,body.saturating_sub(1),maximum_work_units)?,1)?,s.advance_work_demand(source,body.saturating_sub(1),maximum_work_units)?))},
   2=>Ok((nested(RetainedBranchProjection::<B>::exchange_demand::<F>(),1)?,1)),
   4=>Ok((nested(self.stage.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<SealedShared<B>>>()+1)?,1)),
   _=>Err(invalid()),
  }
 }
 pub fn advance_reply<'r,F>(&mut self,source:RetainedCloneRef<'_,B>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool,exchange:Exchange<'_,B>,units:&mut u64)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote::<F>(source,body,maximum_work_units)?;
    if !retained_mutation_demand_fits(d,g)||u>maximum_work_units||effect.remaining_work_units()?<u{return Ok(RetainedCloneStep::Progress(Default::default()))}
    *units=u;
   }
   let mut p=RetainedCloneProgress::default();let nested_body=body.saturating_sub(1);
   match self.phase{
    0=>{self.stage=Some(Box::new(RetainedBranchProjection::new()));self.phase=1;p=receipt(birth::<RetainedBranchProjection<B>>());},
    1|3=>{
     let stage=self.stage.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| stage.advance_reply(source,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,false),stage.advance_reply(source,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,true));
     let step=step?;p=add(step.progress(),1)?;
     if self.phase==1&&stage.raw_ready(){self.phase=2;}else if self.phase==3&&stage.ready(){self.phase=4;}},
    2=>{let stage=self.stage.as_mut().ok_or_else(invalid)?;let(_,progress)=exchange(stage,&mut effect.nested(child(g,1)))?;stage.begin_seal()?;self.phase=3;p=add(progress,1)?;},
    4=>{let extra=2*size_of::<Option<SealedShared<B>>>()+1;let stage=self.stage.as_mut().ok_or_else(invalid)?;let(row,progress)=stage.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.sealed.set(row);stage.begin_close();self.phase=5;p=add(progress,extra)?;},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy branch staging")?;
   work=work.max(u128::from(*units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==5{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
}
crate::custody::std_cursor!([B:SnapshotProjection<Output:crate::snapshot::state::fields::BranchProjectionOutput<B>>+Send+Sync+RetireOwned] Staged<B>,[],|s|s.begin_close());

/// 🧳️ Original row, document bindings and displaced branch paths remain separately closeable.
pub struct RowBranchPreparation<P:RowBranchPath>{
 rank:usize,kind:RowChange,phase:u8,closing:bool,original:Bound,row:Bound,set:Option<Box<RowListPreparation<P::Row>>>,staged:Staged<P::Branch>,
 output:Option<Box<P::Output>>,next:Held<PersistentList<P::Row>>,previous:Held<PersistentList<P::Row>>,decision:Held<EnergyModelDiff>,flight:Flight,
}
impl<P:RowBranchPath> RowBranchPreparation<P>{
 pub fn new(rank:usize)->Self{Self{rank,kind:RowChange::Set,phase:0,closing:false,original:Default::default(),row:Default::default(),set:None,staged:Default::default(),output:None,next:Default::default(),previous:Default::default(),decision:Default::default(),flight:Default::default()}}
 /// 🌱️ A ranked insertion retains the same original authored row through every page split.
 pub fn insert(rank:usize)->Self{let mut value=Self::new(rank);value.kind=RowChange::Insert;value}
 /// ✂️ A ranked removal rebuilds its actual original collection without a replacement row.
 pub fn remove(rank:usize)->Self{let mut value=Self::new(rank);value.kind=RowChange::Remove;value}
 pub fn ready(&self)->bool{!self.closing&&self.phase==11}
 fn check(&self,row:Option<RetainedCloneRef<'_,SealedShared<P::Row>>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if self.kind!=RowChange::Remove{row.ok_or_else(invalid)?.check_binding(&self.row.0)?;}original.check_binding(&self.original.0)?;Ok(())}
 fn quote_original(&self,row:Option<RetainedCloneRef<'_,SealedShared<P::Row>>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{if self.closing||self.phase>=11||!self.flight.idle(){return Err(invalid());}self.check(row,original)?;match self.phase{
  0=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),
  1=>if self.kind==RowChange::Remove{Ok((scalar(1),1))}else{let r=row.ok_or_else(invalid)?;Ok((nested(scalar(r.binding_copy_bytes()),1)?,r.binding_work_demand(&self.row.0)?))},
  2=>Ok((birth::<RowListPreparation<P::Row>>(),1)),
  3=>self.set.as_ref().ok_or_else(invalid)?.quote(P::rows(original),row),
  4=>{let(d,w)=self.set.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<PersistentList<P::Row>>>()+1)?,w))},
  5=>Ok((Cursor::demand(&self.set,body)?,Cursor::work(&self.set,body)?)),
  6=>self.staged.quote::<PersistentList<P::Row>>(P::branch(original),body,maximum_work_units),
  7=>Ok((birth::<P::Output>(),1)),
  8=>self.output.as_ref().ok_or_else(invalid)?.quote(self.staged.sealed.get_opt(),original,body.saturating_sub(1),maximum_work_units),
  9=>{let(d,w)=self.output.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
  10=>Ok((Cursor::demand(&self.output,body)?,Cursor::work(&self.output,body)?)),
  _=>Err(invalid())}}
 fn advance_original<'r>(&mut self,row:Option<RetainedCloneRef<'_,SealedShared<P::Row>>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote_original(row,original,body,maximum_work_units)?;
    if u>maximum_work_units||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut p=RetainedCloneProgress::default();let nested_body=body.saturating_sub(1);
   match self.phase{
    0=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=1;},
    1=>{if self.kind!=RowChange::Remove{p=add(bind(row.ok_or_else(invalid)?,&mut self.row,&mut effect.nested(child(g,1)),&mut work)?,1)?;}self.phase=2;},
    2=>{self.set=Some(Box::new(RowListPreparation::new(self.kind,self.rank)));self.phase=3;p=receipt(birth::<RowListPreparation<P::Row>>());},
    3=>{let set=self.set.as_mut().ok_or_else(invalid)?;let step=set.advance(P::rows(original),row,&mut effect.nested(child(g,1)),&mut work)?;p=add(step.progress(),1)?;if set.output_ready(){self.phase=4;}},
    4=>{let extra=2*size_of::<Option<PersistentList<P::Row>>>()+1;let set=self.set.as_mut().ok_or_else(invalid)?;let(next,progress)=set.take(&mut effect.nested(child(g,extra)),&mut work)?.ok_or_else(invalid)?;self.next.set(next);set.mark();self.phase=5;p=add(progress,extra)?;},
    5=>{let step=custody::finish(&mut self.flight,&mut self.set,body,effect,&mut work,recover)?;p=step.progress();if self.set.is_none(){self.phase=6;}},
    6=>{let Self{staged,next,previous,..}=self;let mut exchange=|stage:&mut RetainedBranchProjection<P::Branch>,e:&mut RetainedAllocationEffect<'_>|stage.exchange(P::rows_mut,next.slot(),previous.slot(),e);
     let step=staged_step::<_,PersistentList<P::Row>>(staged,P::branch(original),body,maximum_work_units,effect,&mut work,recover,&mut exchange,&mut units)?;p=step.progress();if staged.ready(){self.phase=7;}},
    7=>{self.output=Some(Box::new(P::output()));self.phase=8;p=receipt(birth::<P::Output>());},
    8=>{let output=self.output.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,2,body,maximum_work_units,None,work,|r| output.advance_reply(self.staged.sealed.slot(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,false),output.advance_reply(self.staged.sealed.slot(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,true));
     let step=step?;p=add(step.progress(),1)?;if output.ready(){self.phase=9;}},
    9=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let output=self.output.as_mut().ok_or_else(invalid)?;let(diff,progress)=output.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);output.begin_close();self.phase=10;p=add(progress,extra)?;},
    10=>{let step=custody::finish(&mut self.flight,&mut self.output,body,effect,&mut work,recover)?;p=step.progress();if self.output.is_none(){self.phase=11;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy row branch publication")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 /// 📏️ Replacement and insertion quote the actual authored row as well as the original collection.
 pub fn quote(&self,row:RetainedCloneRef<'_,SealedShared<P::Row>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{self.quote_original(Some(row),original,body,maximum_work_units)}
 /// 🧵️ An authored immutable row supplies exactly the existing replacement or insertion source.
 pub fn advance_reply<'r>(&mut self,row:RetainedCloneRef<'_,SealedShared<P::Row>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.advance_original(Some(row),original,body,maximum_work_units,effect,reply,recover)}
 /// ✂️ Removal borrows only its actual original collection and retained rank.
 pub fn quote_remove(&self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{if self.kind!=RowChange::Remove{return Err(invalid());}self.quote_original(None,original,body,maximum_work_units)}
 /// 🧵️ No replacement row is invented or sealed for a removal.
 pub fn advance_remove_reply<'r>(&mut self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{if self.kind!=RowChange::Remove{return Err(invalid());}self.advance_original(None,original,body,maximum_work_units,effect,reply,recover)}
 pub fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=12;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(output,staged,set,next,previous,decision,original,row);
}
crate::custody::std_cursor!([P:RowBranchPath] RowBranchPreparation<P>,[],|s|s.begin_close());
/// 🎭️ Runs one staged-branch turn inside its owner's turn, on the owner's retained route.
pub fn staged_step<B,F>(staged:&mut Staged<B>,source:RetainedCloneRef<'_,B>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128,recover:bool,exchange:Exchange<'_,B>,units:&mut u64)->Result<RetainedCloneStep,RetirementPollExit> where B:SnapshotProjection<Output:crate::snapshot::state::fields::BranchProjectionOutput<B>>+Send+Sync+RetireOwned{
 let before=effect.performed_work();let mut stash=None;
 let done=crate::retained::slot(&mut stash).and_then(|reply|staged.advance_reply::<F>(source,body,max,effect,reply,recover,exchange,units));drop(done?);
 *work+=effect.performed_work().saturating_sub(before);
 match stash.take(){Some(result)=>Ok(result?),None=>Err(RetirementPollExit::Pending(0))}
}

/// 🪟️ A sealed replacement row becomes an admitted source and is published through its branch without leaving custody gaps.
pub struct RowTail<P:RowBranchPath> where P::Row:SnapshotProjection<Output=Box<P::Row>>{
 staged:Staged<P::Row>,admit:Held<StagedSourceAdmission<SealedShared<P::Row>,SealedShared<P::Row>>>,source:Sourced<SealedShared<P::Row>>,publication:Option<Box<RowBranchPreparation<P>>>,decision:Held<EnergyModelDiff>,rank:usize,phase:u8,closing:bool,flight:Flight,
}
impl<P:RowBranchPath> RowTail<P> where P::Row:SnapshotProjection<Output=Box<P::Row>>{
 pub fn new(rank:usize)->Self{Self{staged:Default::default(),admit:Default::default(),source:Default::default(),publication:None,decision:Default::default(),rank,phase:0,closing:false,flight:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==7}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(publication,source,admit,staged,decision);
 pub fn quote<F>(&self,row:RetainedCloneRef<'_,P::Row>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=7||!self.flight.idle(){return Err(invalid());}
  match self.phase{
   0=>self.staged.quote::<F>(row,body,maximum_work_units),
   1=>Ok((scalar(2*size_of::<Option<SealedShared<P::Row>>>()+2*size_of::<StagedSourceAdmission<SealedShared<P::Row>,SealedShared<P::Row>>>()+1),1)),
   2=>{let a=self.admit.get().ok_or_else(invalid)?;Ok((nested(if a.is_projecting(){a.owned_projection_demand()?}else{a.next_demand()?},1)?,a.next_work_demand()?))},
   3=>Ok((birth::<RowBranchPreparation<P>>(),1)),
   4=>self.publication.as_ref().ok_or_else(invalid)?.quote(self.source.get().ok_or_else(invalid)?.borrow(),original,body.saturating_sub(1),maximum_work_units),
   5=>{let(d,w)=self.publication.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
   6=>Ok((Cursor::demand(&self.publication,body)?,Cursor::work(&self.publication,body)?)),
   _=>Err(invalid()),
  }
 }
 pub fn advance_reply<'r,F>(&mut self,row:RetainedCloneRef<'_,P::Row>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool,exchange:Exchange<'_,P::Row>)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote::<F>(row,original,body,maximum_work_units)?;
    if u>maximum_work_units||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut p=RetainedCloneProgress::default();let nested_body=body.saturating_sub(1);
   match self.phase{
    0=>{let step=staged_step::<_,F>(&mut self.staged,row,body,maximum_work_units,effect,&mut work,recover,exchange,&mut units)?;p=step.progress();if self.staged.ready(){self.phase=1;}},
    1=>{let sealed=self.staged.sealed.take().ok_or_else(invalid)?;self.admit.set(StagedSourceAdmission::owned(sealed));self.phase=2;p=receipt(scalar(2*size_of::<Option<SealedShared<P::Row>>>()+2*size_of::<StagedSourceAdmission<SealedShared<P::Row>,SealedShared<P::Row>>>()+1));},
    2=>{let a=self.admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;p=turn.result?.progress();if a.terminal_is_empty(){drop(self.admit.take());}if self.source.get().is_some(){self.phase=3;}},
    3=>{self.publication=Some(Box::new(RowBranchPreparation::new(self.rank)));self.phase=4;p=receipt(birth::<RowBranchPreparation<P>>());},
    4=>{let publication=self.publication.as_mut().ok_or_else(invalid)?;let source=self.source.get().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| publication.advance_reply(source.borrow(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,false),publication.advance_reply(source.borrow(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,true));
     let step=step?;p=add(step.progress(),1)?;if publication.ready(){self.phase=5;}},
    5=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let publication=self.publication.as_mut().ok_or_else(invalid)?;let(diff,progress)=publication.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);publication.begin_close();self.phase=6;p=add(progress,extra)?;},
    6=>{let step=custody::finish(&mut self.flight,&mut self.publication,body,effect,&mut work,recover)?;p=step.progress();if self.publication.is_none(){self.phase=7;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy row tail")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=8;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
}
crate::custody::std_cursor!([P:RowBranchPath] RowTail<P>,[P::Row:SnapshotProjection<Output=Box<P::Row>>],|s|s.begin_close());
/// 🪟️ Runs one tail turn inside its owner's turn, on the owner's retained route.
pub fn tail_step<P:RowBranchPath,F>(tail:&mut RowTail<P>,row:RetainedCloneRef<'_,P::Row>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128,recover:bool,exchange:Exchange<'_,P::Row>)->Result<RetainedCloneStep,RetirementPollExit> where P::Row:SnapshotProjection<Output=Box<P::Row>>{
 let before=effect.performed_work();let mut stash=None;
 let done=crate::retained::slot(&mut stash).and_then(|reply|tail.advance_reply::<F>(row,original,body,max,effect,reply,recover,exchange));drop(done?);
 *work+=effect.performed_work().saturating_sub(before);
 match stash.take(){Some(result)=>Ok(result?),None=>Err(RetirementPollExit::Pending(0))}
}

/// 🚦️ Row policies report authored semantic outcomes without creating a branch for refused changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum RowOutcome{Changed,Unchanged,Invalid,InvalidReference,InvalidContext,Duplicate,Missing,Referenced}
impl RowOutcome{pub fn as_str(self)->&'static str{match self{Self::Changed=>"changed",Self::Unchanged=>"no-op",Self::Invalid=>"invalid",Self::InvalidReference=>"invalid-reference",Self::InvalidContext=>"invalid-context",Self::Duplicate=>"duplicate",Self::Missing=>"missing",Self::Referenced=>"referenced"}}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🌱️admission/🦀️.rs"]mod admission;pub use admission::{RowAdmission,RowAdmissionScan};
