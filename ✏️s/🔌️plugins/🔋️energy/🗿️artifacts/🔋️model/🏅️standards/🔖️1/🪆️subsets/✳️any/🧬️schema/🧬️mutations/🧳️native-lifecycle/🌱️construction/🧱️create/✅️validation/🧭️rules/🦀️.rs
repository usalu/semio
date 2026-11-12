//! 🧭️ Authored creation rules inspect original vector items and foreign keys through separately funded native cursors.
use super::*;
use crate::reference::{ReferenceCollection,ReferenceLookup};
/// 🪪️ Each rule reads one actual source predicate or one retained original identifier.
pub enum CreationRule<'a>{Predicate(bool),Reference(RetainedCloneRef<'a,u32>,&'static[ReferenceCollection])}
/// 🧬️ Typed schema rules provide bounded original item readers after their declared base validation.
pub trait CreationRuleSource:Send+Sync+'static{type Payload:Send+Sync+'static;type Base:RowCreationValidation<Self::Payload>;fn base()->Self::Base;fn len(input:&Self::Payload)->Result<usize,ValueError>;fn read(input:RetainedCloneRef<'_,Self::Payload>,index:usize)->Result<CreationRule<'_>,ValueError>;fn reference_target(input:RetainedCloneRef<'_,Self::Payload>,index:usize)->Result<RetainedCloneRef<'_,u32>,ValueError>{match Self::read(input,index)?{CreationRule::Reference(key,_)=>Ok(key),_=>Err(invalid())}}}
/// 🧳️ The failed original item index survives child closure so diagnostics retain the actual foreign identifier.
pub struct RuleRowCreationValidation<S:CreationRuleSource>{input:Bound,original:Bound,base:Option<Box<S::Base>>,reference:Option<Box<ReferenceLookup>>,index:usize,collection:usize,found:bool,result:Option<Option<RowOutcome>>,phase:u8,closing:bool,flight:Flight}
impl<S:CreationRuleSource> RuleRowCreationValidation<S>{
 pub fn new()->Self{Self{input:Default::default(),original:Default::default(),base:None,reference:None,index:0,collection:0,found:false,result:None,phase:0,closing:false,flight:Default::default()}}
 fn reference(input:RetainedCloneRef<'_,S::Payload>,index:usize)->Result<(RetainedCloneRef<'_,u32>,&'static[ReferenceCollection]),ValueError>{if let CreationRule::Reference(key,collections)=S::read(input,index)?{if collections.is_empty(){return Err(invalid());}Ok((key,collections))}else{Err(invalid())}}
 fn extra()->usize{4*size_of::<usize>()+2*size_of::<CreationRule<'static>>()+2*size_of::<Option<RowOutcome>>()+2}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(reference,base,input,original);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:CreationRuleSource] RuleRowCreationValidation<S>,[],|s|s.begin_close());
impl<S:CreationRuleSource> RowCreationValidation<S::Payload> for RuleRowCreationValidation<S>{
 fn quote(&self,input:RetainedCloneRef<'_,S::Payload>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{if self.closing||self.phase==99||!self.flight.idle(){return Err(invalid());}input.check_binding(&self.input.0)?;original.check_binding(&self.original.0)?;let extra=Self::extra();match self.phase{
  0=>Ok((nested(scalar(input.binding_copy_bytes()),1)?,input.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),2=>Ok((birth::<S::Base>(),1)),
  3=>{let(d,w)=self.base.as_ref().ok_or_else(invalid)?.quote(input,original,body.saturating_sub(extra),max)?;Ok((nested(d,extra)?,w))},
  4=>Ok((nested(Cursor::demand(&self.base,body.saturating_sub(1))?,1)?,Cursor::work(&self.base,body.saturating_sub(1))?)),
  5=>Ok((scalar(2*size_of::<S::Payload>()+extra),1)),6=>Ok((birth::<ReferenceLookup>(),1)),
  7=>{let(d,w)=self.reference.as_ref().ok_or_else(invalid)?.quote(original,Self::reference(input,self.index)?.0)?;Ok((nested(d,2)?,w))},
  8=>Ok((nested(Cursor::demand(&self.reference,body.saturating_sub(extra))?,extra)?,Cursor::work(&self.reference,body.saturating_sub(extra))?)),
  _=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,S::Payload>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(input,original,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let extra=Self::extra();let mut p=receipt(scalar(1));
   match self.phase{
    0=>{p=add(bind(input,&mut self.input,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=1;},
    1=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{self.base=Some(Box::new(S::base()));self.phase=3;p=receipt(birth::<S::Base>());},
    3=>{let c=self.base.as_mut().ok_or_else(invalid)?;let nb=body.saturating_sub(extra);
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(input,original,nb,max,&mut effect.nested(child(g,extra)),r,false),c.advance_reply(input,original,nb,max,&mut effect.nested(child(g,extra)),r,true));
     p=add(step?.progress(),extra)?;if let Some(result)=c.result(){if result.is_some(){self.result=Some(result);}c.begin_close();self.phase=4;}},
    4=>{let step=custody::finish(&mut self.flight,&mut self.base,body.saturating_sub(1),effect,&mut work,recover)?;p=add(step.progress(),1)?;if self.base.is_none(){self.phase=if self.result.is_some(){99}else{5};}},
    5=>{if self.index>=S::len(input.get())?{self.result=Some(None);self.phase=99;}else{match S::read(input,self.index)?{CreationRule::Predicate(valid)=>{if valid{self.index+=1;}else{self.result=Some(Some(RowOutcome::Invalid));self.phase=99;}},CreationRule::Reference(_,collections)=>{if collections.is_empty(){return Err(invalid().with_retained_progress(p).into());}self.collection=0;self.phase=6;}}}},
    6=>{let(_,collections)=Self::reference(input,self.index)?;self.reference=Some(Box::new(ReferenceLookup::new(*collections.get(self.collection).ok_or_else(invalid)?)));self.phase=7;p=receipt(birth::<ReferenceLookup>());},
    7=>{let c=self.reference.as_mut().ok_or_else(invalid)?;p=add(c.advance(original,Self::reference(input,self.index)?.0,&mut effect.nested(child(g,2)))?.progress(),2)?;if let Some(found)=c.result(){self.found=found;c.begin_close();self.phase=8;}},
    8=>{let step=custody::finish(&mut self.flight,&mut self.reference,body.saturating_sub(extra),effect,&mut work,recover)?;p=add(step.progress(),extra)?;if self.reference.is_none(){if self.found{self.index+=1;self.phase=5;}else if self.collection+1<Self::reference(input,self.index)?.1.len(){self.collection+=1;self.phase=6;}else{self.result=Some(Some(RowOutcome::InvalidReference));self.phase=99;}}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy creation rule")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn result(&self)->Option<Option<RowOutcome>>{self.result}
 fn reference_index(&self)->Option<usize>{(self.result==Some(Some(RowOutcome::InvalidReference))).then_some(self.index)}
}
