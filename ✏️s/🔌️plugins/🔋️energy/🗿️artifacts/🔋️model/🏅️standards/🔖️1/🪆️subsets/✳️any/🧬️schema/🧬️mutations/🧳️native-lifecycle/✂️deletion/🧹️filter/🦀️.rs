//! 🧹️ A cascade removes original selected rows through separately funded immutable splices.
use super::*;
use crate::row::{RowListPreparation,RowChange};
use crate::custody::Sourced;
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_framework_value::retained_clone::StagedSourceAdmission;
use protocol::RetainedFieldPreparation;
/// 🧵️ The original list, current private root and displaced aliases retain independent closure.
pub(super)struct CascadeFilter<T:RetireOwned+Send+Sync+'static>{stage:Option<Box<RetainedFieldPreparation<PersistentList<T>>>>,splice:Option<Box<RowListPreparation<T>>>,source:Sourced<PersistentList<T>>,admit:Held<StagedSourceAdmission<PersistentList<T>,PersistentList<T>>>,next:Held<PersistentList<T>>,body:Option<usize>,rank:usize,removed:usize,weight:usize,phase:u8,closing:bool,predicate:fn(&T,u32)->usize,flight:Flight}
impl<T:RetireOwned+Send+Sync+'static>CascadeFilter<T>{
 pub(super)fn new(predicate:fn(&T,u32)->usize)->Self{Self{stage:None,splice:None,source:Default::default(),admit:Default::default(),next:Default::default(),body:None,rank:0,removed:0,weight:0,phase:0,closing:false,predicate,flight:Default::default()}}
 pub(super)fn ready(&self)->bool{!self.closing&&self.phase==11}
 pub(super)fn weight(&self)->usize{self.weight}
 pub(super)fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(splice,stage,source,admit,next);
 fn admit_extra()->usize{2*size_of::<Option<PersistentList<T>>>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<PersistentList<T>>>>()+2*size_of::<Option<StagedSourceAdmission<PersistentList<T>,PersistentList<T>>>>()+1}
 pub(super)fn quote(&self,original:RetainedCloneRef<'_,PersistentList<T>>,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=11||self.body.is_some_and(|v|v!=body)||!self.flight.idle(){return Err(invalid())}let nb=body.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(birth::<RetainedFieldPreparation<PersistentList<T>>>(),size_of::<usize>())?,1)),
   1=>{let s=self.stage.as_ref().ok_or_else(invalid)?;Ok((nested(s.demand(original,nb,max)?,1)?,s.advance_work_demand(original,nb,max)?))},
   2=>Ok((nested(self.stage.as_ref().ok_or_else(invalid)?.exchange_demand()?,size_of::<Option<PersistentList<T>>>()+1)?,1)),
   3=>Ok((Cursor::demand(&self.stage,body)?,Cursor::work(&self.stage,body)?)),
   4=>{let extra=Self::admit_extra();match self.admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,depth:d.depth+1,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<PersistentList<T>,PersistentList<T>>>()+extra,depth:2,..Default::default()},1))}},
   5=>Ok((scalar(size_of::<T>()+8*size_of::<usize>()+1),1)),
   6=>Ok((birth::<RowListPreparation<T>>(),1)),
   7=>{let(d,w)=self.splice.as_ref().ok_or_else(invalid)?.quote(self.source.get().ok_or_else(invalid)?.borrow(),None)?;Ok((nested(d,1)?,w))},
   8=>{let(d,w)=self.splice.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<PersistentList<T>>>()+1)?,w))},
   9=>Ok((Cursor::demand(&self.splice,body)?,Cursor::work(&self.splice,body)?)),
   10=>Ok((Cursor::demand(&self.source,body)?,Cursor::work(&self.source,body)?)),
   _=>Err(invalid()),
  }
 }
 pub(super)fn advance_reply<'r>(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,key:u32,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(original,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);
   match self.phase{
    0=>{self.stage=Some(Box::default());self.phase=1;p=receipt(birth::<RetainedFieldPreparation<PersistentList<T>>>());},
    1=>{let c=self.stage.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_into(original,nb,max,&mut effect.nested(child(g,1)),r),c.recover_advance_into(original,nb,max,&mut effect.nested(child(g,1)),r));p=add(step?.progress(),1)?;if c.value().is_some(){self.phase=2;}},
    2=>{let extra=size_of::<Option<PersistentList<T>>>()+1;let Self{stage,next,..}=self;let c=stage.as_deref_mut().ok_or_else(invalid)?;let slot=next.slot();let step=native_child(effect,&mut work,|e,r|c.take_into(slot,&mut e.nested(child(g,extra)),r))??;p=add(step.progress(),extra)?;self.phase=3;},
    3=>{let step=custody::finish(&mut self.flight,&mut self.stage,body,effect,&mut work,recover)?;p=step.progress();if self.stage.is_none(){self.phase=4;}},
    4=>{let extra=Self::admit_extra();if self.admit.is_none()&&self.source.get().is_none(){let value=self.next.take().ok_or_else(invalid)?;self.admit.set(StagedSourceAdmission::owned(value));p=receipt(scalar(extra));}else{let a=self.admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;p=add(turn.result?.progress(),extra)?;if a.terminal_is_empty(){drop(self.admit.take());}}if self.source.get().is_some(){self.phase=5;}},
    5=>{p=receipt(scalar(1));let rows=self.source.get().ok_or_else(invalid)?.borrow();if self.rank==rows.get().len(){self.phase=11;}else{let weight=(self.predicate)(rows.get().get(self.rank).ok_or_else(invalid)?,key);if weight==0{self.rank+=1;}else{self.removed+=1;self.weight=self.weight.checked_add(weight).ok_or_else(invalid)?;self.phase=6;}}},
    6=>{self.splice=Some(Box::new(RowListPreparation::new(RowChange::Remove,self.rank)));self.phase=7;p=receipt(birth::<RowListPreparation<T>>());},
    7=>{let c=self.splice.as_mut().ok_or_else(invalid)?;let src=self.source.get().ok_or_else(invalid)?;p=add(c.advance(src.borrow(),None,&mut effect.nested(child(g,1)),&mut work)?.progress(),1)?;if c.output_ready(){self.phase=8;}},
    8=>{let extra=2*size_of::<Option<PersistentList<T>>>()+1;let c=self.splice.as_mut().ok_or_else(invalid)?;let(v,paid)=c.take(&mut effect.nested(child(g,extra)),&mut work)?.ok_or_else(invalid)?;self.next.set(v);c.mark();p=add(paid,extra)?;self.phase=9;},
    9=>{let step=custody::finish(&mut self.flight,&mut self.splice,body,effect,&mut work,recover)?;p=step.progress();if self.splice.is_none(){self.phase=10;}},
    10=>{let step=custody::chain_step(&mut self.flight,&mut [&mut self.source as &mut dyn Cursor],None,body,effect,&mut work,recover)?;p=step.progress();if self.source.get().is_none(){self.phase=4;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy cascade list")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub(super)fn take_quote(&self,body:usize)->Result<Quote,ValueError>{if !self.ready(){return Err(invalid())}Ok((nested(self.source.get().ok_or_else(invalid)?.next_take_demand(body)?,2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<PersistentList<T>>>>()+1)?,1))}
 pub(super)fn take(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<(Option<PersistentList<T>>,RetainedCloneProgress),ValueError>{
  let(d,_)=self.take_quote(body)?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok((None,Default::default()))}let extra=2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<PersistentList<T>>>>()+1;let g=effect.grant();
  let Self{source,next,..}=self;let s=source.slot().as_mut().ok_or_else(invalid)?;let slot=next.slot();
  let step=native_child(effect,work,|e,r|s.take_authority_into(slot,body,&mut e.nested(child(g,extra)),r))?.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?;
  let paid=add(step.progress(),extra)?;
  match self.next.take(){Some(v)=>{drop(self.source.slot().take());self.phase=12;Ok((Some(v),paid))},None=>Ok((None,paid))}
 }
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([T:RetireOwned+Send+Sync+'static] CascadeFilter<T>,[],|s|s.begin_close());
