//! ✂️ Native row deletion preserves its genuine original row and rank through removal, typed inverse construction and cancellation.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::{EntityRow,RetainedEntityLookup}},diff::original::EnergyModelDiff,row::{RowBranchPath,RowBranchPreparation,RowOutcome},leaf::RowFieldPreparation,construction::{RowBirthSpec,RowBirthPreparation,OwnedRecordForm},scalar::{Quote,invalid,scalar,nested,child,add,receipt,birth,bind},custody::{self,Cursor,Held,Bound},retained::{Done,Flight,Reply,conclude,native_child}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,RetainedWorkOutcome,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneRef,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use protocol::retained_mutation_demand_fits;
use std::mem::size_of;
#[path="🛂️guard/🦀️.rs"]mod guard;pub use guard::{RowDeletionGuard,NoRowReferences,RowReferenceSource,ReferenceRowGuard,AllRowReferences,RootReferenceSource,RootReferenceGuard};
/// 🧭️ Authored delete paths provide their actual inverse create record and retained reference restriction.
pub trait DeleteRowPath:RowBranchPath where Self::Row:EntityRow{
 type Payload:Send+Sync+'static;type Inverse:RetireOwned+Send+Sync;type Recreate:RowBirthSpec<Payload=Self::Row,Row=Self::Inverse,Context=usize>;type Guard:RowDeletionGuard<Self::Row>;
 fn key(p:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,u32>;
}
fn retained_row_extra<T:?Sized+Sync>(more:usize)->usize{2*size_of::<RetainedOwnedProjection<T>>()+2*size_of::<Option<Box<RetainedOwnedProjection<T>>>>()+more}
/// 🧳️ The selected original row outlives every private removal path and original inverse field copy.
pub struct DeleteRowPreparation<S:DeleteRowPath> where S::Row:EntityRow{
 body:Option<usize>,input:Bound,original:Bound,lookup:Option<Box<RetainedEntityLookup<S::Row>>>,row:Option<Box<RetainedOwnedProjection<S::Row>>>,guard:Option<Box<S::Guard>>,publication:Option<Box<RowBranchPreparation<S>>>,inverse_cursor:Option<Box<RowBirthPreparation<S::Recreate,OwnedRecordForm>>>,
 decision:Held<EnergyModelDiff>,inverse_value:Held<S::Inverse>,rank:Option<usize>,outcome:Option<RowOutcome>,phase:u8,inverse_phase:u8,closing:bool,flight:Flight,
}
impl<S:DeleteRowPath> DeleteRowPreparation<S> where S::Row:EntityRow{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(inverse_cursor,publication,guard,lookup,row,decision,inverse_value,original,input);
 fn row(&self)->Result<RetainedCloneRef<'_,S::Row>,ValueError>{self.row.as_ref().ok_or_else(invalid)?.borrow()}
 fn delete_quote(&self,p:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||self.body.is_some_and(|v|v!=body)||!self.flight.idle(){return Err(invalid());}p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;let nb=body.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(scalar(p.binding_copy_bytes()+size_of::<usize>()),1)?,p.binding_work_demand(&self.input.0)?)),
   1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),
   2=>Ok((birth::<RetainedEntityLookup<S::Row>>(),1)),
   3=>{let l=self.lookup.as_ref().ok_or_else(invalid)?;Ok((nested(l.demand(S::rows(o),S::key(p))?,1)?,l.advance_work_demand(S::rows(o),S::key(p))?))},
   4=>{let l=self.lookup.as_ref().ok_or_else(invalid)?;if l.result()==Some(None){Ok((scalar(2*size_of::<Option<RowOutcome>>()+1),1))}else{let mut d=nested(l.take_demand()?,retained_row_extra::<S::Row>(2*size_of::<Option<usize>>()+1))?;d.capacity_bytes=d.capacity_bytes.checked_add(size_of::<RetainedOwnedProjection<S::Row>>()).ok_or_else(invalid)?;Ok((d,1))}},
   5=>Ok((Cursor::demand(&self.lookup,body)?,Cursor::work(&self.lookup,body)?)),
   6=>Ok((birth::<S::Guard>(),1)),
   7=>{let extra=2*size_of::<Option<RowOutcome>>()+1;let(d,w)=self.guard.as_ref().ok_or_else(invalid)?.quote(self.row()?,o,body.saturating_sub(extra),max)?;Ok((nested(d,extra)?,w))},
   8=>Ok((Cursor::demand(&self.guard,body)?,Cursor::work(&self.guard,body)?)),
   9=>Ok((birth::<RowBranchPreparation<S>>(),1)),
   10=>{let(d,w)=self.publication.as_ref().ok_or_else(invalid)?.quote_remove(o,nb,max)?;Ok((nested(d,1)?,w))},
   11=>{let(d,w)=self.publication.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
   12=>Ok((Cursor::demand(&self.publication,body)?,Cursor::work(&self.publication,body)?)),
   _=>Err(invalid()),
  }
 }
 fn delete_advance<'r>(&mut self,payload:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.delete_quote(payload,o,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);
   match self.phase{
    0=>{p=add(bind(payload,&mut self.input,&mut effect.nested(child(g,1+size_of::<usize>())),&mut work)?,1+size_of::<usize>())?;self.phase=1;},
    1=>{p=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{self.lookup=Some(Box::new(RetainedEntityLookup::new()));self.phase=3;p=receipt(birth::<RetainedEntityLookup<S::Row>>());},
    3=>{let c=self.lookup.as_mut().ok_or_else(invalid)?;p=add(c.advance(S::rows(o),S::key(payload),&mut effect.nested(child(g,1)))?.progress(),1)?;if c.result().is_some(){self.phase=4;}},
    4=>{let c=self.lookup.as_mut().ok_or_else(invalid)?;if c.result()==Some(None){self.outcome=Some(RowOutcome::Missing);self.phase=99;p=receipt(scalar(2));}else{
     let extra=retained_row_extra::<S::Row>(2*size_of::<Option<usize>>()+1);let cap=size_of::<RetainedOwnedProjection<S::Row>>();
     let(row,progress)=c.take(&mut effect.nested(RetainedCloneGrant{maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(cap),..child(g,extra)}))?.ok_or_else(invalid)?;
     self.rank=c.result().ok_or_else(invalid)?;self.row=Some(Box::new(row));c.begin_close();self.phase=5;p=add(progress,extra)?;p.retained_capacity_bytes=p.retained_capacity_bytes.checked_add(cap).ok_or_else(invalid)?;}},
    5=>{let step=custody::finish(&mut self.flight,&mut self.lookup,body,effect,&mut work,recover)?;p=step.progress();if self.lookup.is_none(){self.phase=6;}},
    6=>{self.guard=Some(Box::new(<S::Guard as RowDeletionGuard<S::Row>>::new()));self.phase=7;p=receipt(birth::<S::Guard>());},
    7=>{let extra=2*size_of::<Option<RowOutcome>>()+1;let Self{guard,row,flight,..}=self;let c=guard.as_mut().ok_or_else(invalid)?;let r=row.as_ref().ok_or_else(invalid)?.borrow()?;let gb=body.saturating_sub(extra);
     let step=protocol::retained_routed!(*flight,effect,recover,1,body,max,None,work,|rep| c.advance_reply(r,o,gb,max,&mut effect.nested(child(g,extra)),rep,false),c.advance_reply(r,o,gb,max,&mut effect.nested(child(g,extra)),rep,true));
     p=add(step?.progress(),extra)?;if let Some(allowed)=<S::Guard as RowDeletionGuard<S::Row>>::result(c){if !allowed{self.outcome=Some(RowOutcome::Referenced);}<S::Guard as RowDeletionGuard<S::Row>>::begin_close(c);self.phase=8;}},
    8=>{let step=custody::finish(&mut self.flight,&mut self.guard,body,effect,&mut work,recover)?;p=step.progress();if self.guard.is_none(){self.phase=if self.outcome.is_some(){99}else{9};}},
    9=>{self.publication=Some(Box::new(RowBranchPreparation::remove(self.rank.ok_or_else(invalid)?)));self.phase=10;p=receipt(birth::<RowBranchPreparation<S>>());},
    10=>{let c=self.publication.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_remove_reply(o,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_remove_reply(o,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=11;}},
    11=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let c=self.publication.as_mut().ok_or_else(invalid)?;let(diff,progress)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);c.begin_close();self.phase=12;p=add(progress,extra)?;},
    12=>{let step=custody::finish(&mut self.flight,&mut self.publication,body,effect,&mut work,recover)?;p=step.progress();if self.publication.is_none(){self.outcome=Some(RowOutcome::Changed);self.phase=99;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original native Energy row deletion")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn inverse_phase_quote(&self,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase!=100||self.inverse_phase>=4||self.body.is_some_and(|v|v!=body)||!self.flight.idle(){return Err(invalid());}let nb=body.saturating_sub(1);
  match self.inverse_phase{
   0=>Ok((birth::<RowBirthPreparation<S::Recreate,OwnedRecordForm>>(),1)),
   1=>{let(d,w)=self.inverse_cursor.as_ref().ok_or_else(invalid)?.quote(self.row()?,nb,max)?;Ok((nested(d,1)?,w))},
   2=>{let(d,w)=self.inverse_cursor.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<S::Inverse>>()+1)?,w))},
   3=>Ok((Cursor::demand(&self.inverse_cursor,body)?,Cursor::work(&self.inverse_cursor,body)?)),
   _=>Err(invalid()),
  }
 }
 fn inverse_advance<'r>(&mut self,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let q=if recover{None}else{Some(self.inverse_phase_quote(body,max))};let g=effect.grant();let nb=body.saturating_sub(1);
  custody::gated(effect,reply,q,max,|effect,work|{let mut p=RetainedCloneProgress::default();
   match self.inverse_phase{
    0=>{self.inverse_cursor=Some(Box::new(RowBirthPreparation::with_context(self.rank.ok_or_else(invalid)?)));self.inverse_phase=1;p=receipt(birth::<RowBirthPreparation<S::Recreate,OwnedRecordForm>>());},
    1=>{let Self{inverse_cursor,row,flight,..}=self;let c=inverse_cursor.as_mut().ok_or_else(invalid)?;let r=row.as_ref().ok_or_else(invalid)?.borrow()?;
     let step=protocol::retained_routed!(*flight,effect,recover,1,body,max,None,*work,|rep| c.advance_reply(r,nb,max,&mut effect.nested(child(g,1)),rep,false),c.advance_reply(r,nb,max,&mut effect.nested(child(g,1)),rep,true));
     p=add(step?.progress(),1)?;if c.ready(){self.inverse_phase=2;}},
    2=>{let extra=2*size_of::<Option<S::Inverse>>()+1;let c=self.inverse_cursor.as_mut().ok_or_else(invalid)?;let(value,progress)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.inverse_value.set(value);c.begin_close();self.inverse_phase=3;p=add(progress,extra)?;},
    3=>{let step=custody::finish(&mut self.flight,&mut self.inverse_cursor,body,effect,work,recover)?;p=step.progress();if self.inverse_cursor.is_none(){self.inverse_phase=4;}},
    _=>return Err(invalid().into()),
   }
   Ok(if self.inverse_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn inverse_ready(&self)->bool{!self.closing&&self.inverse_phase==4&&self.inverse_value.is_some()}
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:DeleteRowPath] DeleteRowPreparation<S>,[S::Row:EntityRow],|s|s.begin_close());
impl<S:DeleteRowPath> RowFieldPreparation for DeleteRowPreparation<S> where S::Row:EntityRow{
 type Input=S::Payload;type Inverse=S::Inverse;
 fn new()->Self{Self{body:None,input:Default::default(),original:Default::default(),lookup:None,row:None,guard:None,publication:None,inverse_cursor:None,decision:Default::default(),inverse_value:Default::default(),rank:None,outcome:None,phase:0,inverse_phase:0,closing:false,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;Ok(())}
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64)->Result<Quote,ValueError>{self.delete_quote(p,o,body,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.delete_advance(p,o,body,m,e,r,recover)}
 fn ready(&self)->bool{!self.closing&&self.phase==99}
 fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_quote(&self,b:usize,m:u64)->Result<Quote,ValueError>{self.inverse_phase_quote(b,m)}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.inverse_advance(b,m,e,r,recover)}
 fn inverse_ready(&self)->bool{Self::inverse_ready(self)}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !Self::inverse_ready(self){return Err(invalid());}Ok((scalar(2*size_of::<Option<S::Inverse>>()+1),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(S::Inverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}self.inverse_phase=5;Ok(Some((self.inverse_value.take().ok_or_else(invalid)?,receipt(d))))}
}

#[path="📚️multi/🦀️.rs"]mod multi;pub use multi::{MultiDeletePath,MultiDeleteRowPreparation};

#[path="🧹️filter/🦀️.rs"]mod filter;

#[path="🧱️surface/🦀️.rs"]mod surface;pub use surface::{SurfaceDeletePath,SurfaceDeletePreparation};
