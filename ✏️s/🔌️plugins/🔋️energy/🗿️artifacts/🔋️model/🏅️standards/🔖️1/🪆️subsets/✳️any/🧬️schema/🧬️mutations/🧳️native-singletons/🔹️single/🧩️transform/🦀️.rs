//! 🧩️ Authored fixed contextual fields retain their original payload and scalar source through native singleton publication.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneSource,StagedSourceAdmission};
use crate::custody::Sourced;
use semio_framework_value::RetirementDemand;
/// 🧬️ The authored transformation reads only fixed original fields and retains the exact inverse selector.
pub trait OriginalSingleTransform:SingleFieldPath{type Payload:Send+Sync+'static;type Context:NativeScalar;type Inverse:NativeScalar;fn context(p:&Self::Payload)->Self::Context;fn valid_payload(p:&Self::Payload)->bool;fn value(p:&Self::Payload,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Self::Scalar;fn inverse(c:Self::Context,v:Self::Scalar)->Self::Inverse;}
pub struct TransformedSinglePreparation<S:OriginalSingleTransform>{input:Bound,body:Option<usize>,context:Held<S::Context>,source:Sourced<S::Scalar>,admit:Held<StagedSourceAdmission<S::Scalar,S::Scalar>>,core:Option<Box<SingleFieldPreparation<S>>>,phase:u8,closing:bool,flight:Flight}
impl<S:OriginalSingleTransform> TransformedSinglePreparation<S>{
 fn source(&self)->Result<RetainedCloneRef<'_,S::Scalar>,ValueError>{Ok(self.source.get().ok_or_else(invalid)?.borrow())}
 fn core(&self)->Result<&SingleFieldPreparation<S>,ValueError>{self.core.as_deref().ok_or_else(invalid)}
 fn core_mut(&mut self)->Result<&mut SingleFieldPreparation<S>,ValueError>{self.core.as_deref_mut().ok_or_else(invalid)}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 fn input_extra()->usize{size_of::<usize>()+2*size_of::<Option<S::Context>>()+2*size_of::<S::Payload>()+1}
 fn admit_extra()->usize{2*size_of::<S::Payload>()+4*size_of::<S::Scalar>()+2*size_of::<Option<RetainedCloneSource<S::Scalar>>>()+2*size_of::<Option<StagedSourceAdmission<S::Scalar,S::Scalar>>>()+1}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(core,admit,source,input,context);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:OriginalSingleTransform] TransformedSinglePreparation<S>,[],|s|s.begin_close());
impl<S:OriginalSingleTransform> RowFieldPreparation for TransformedSinglePreparation<S>{
 type Input=S::Payload;type Inverse=S::Inverse;
 fn new()->Self{Self{input:Default::default(),body:None,context:Default::default(),source:Default::default(),admit:Default::default(),core:None,phase:0,closing:false,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if key.is_some(){return Err(invalid());}p.check_binding(&self.input.0)?;if let Some(c)=self.core.as_ref(){c.check(self.source()?,o)?;}Ok(())}
 fn quote(&self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||self.body.is_some_and(|b|b!=body)||!self.flight.idle(){return Err(invalid());}RowFieldPreparation::check(self,p,key,o)?;
  match self.phase{
   0=>Ok((nested(scalar(p.binding_copy_bytes()+Self::input_extra()),1)?,p.binding_work_demand(&self.input.0)?)),
   1=>Ok((scalar(2*size_of::<Self::Input>()+2),1)),
   2=>{let extra=Self::admit_extra();match self.admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<S::Scalar,S::Scalar>>()+extra,depth:1,..Default::default()},1))}},
   3=>Ok((birth::<SingleFieldPreparation<S>>(),1)),
   4=>self.core()?.quote(self.source()?,o,body,m),
   _=>Err(invalid()),
  }
 }
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=RowFieldPreparation::quote(self,p,key,o,body,m)?;
    if u>m||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut progress=RetainedCloneProgress::default();
   match self.phase{
    0=>{let extra=Self::input_extra();progress=add(bind(p,&mut self.input,&mut effect.nested(child(g,extra)),&mut work)?,extra)?;self.context.set(S::context(p.get()));self.phase=1;},
    1=>{if S::valid_payload(p.get()){self.phase=2;}else{self.phase=99;}progress=receipt(scalar(1));},
    2=>{if self.admit.is_none()&&self.source.get().is_none(){self.admit.set(StagedSourceAdmission::owned(S::value(p.get(),o)));progress=receipt(scalar(1));}else{let a=self.admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;progress=turn.result?.progress();if a.terminal_is_empty(){drop(self.admit.take());}}if self.source.get().is_some(){self.phase=3;}},
    3=>{self.core=Some(Box::new(SingleFieldPreparation::new()));self.phase=4;progress=receipt(birth::<SingleFieldPreparation<S>>());},
    4=>{let Self{core,source,flight,..}=self;let core=core.as_deref_mut().ok_or_else(invalid)?;let src=source.get().ok_or_else(invalid)?;let nested_body=body.saturating_sub(1);
     let step=protocol::retained_routed!(*flight,effect,recover,1,body,m,None,work,|r| core.advance_reply(src.borrow(),o,nested_body,m,&mut effect.nested(child(g,1)),r,false),core.advance_reply(src.borrow(),o,nested_body,m,&mut effect.nested(child(g,1)),r,true));
     progress=add(step?.progress(),1)?;},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,progress,"original contextual Energy singleton")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if RowFieldPreparation::ready(self){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn ready(&self)->bool{!self.closing&&(self.phase==99||self.core.as_ref().is_some_and(|c|c.ready()))}
 fn outcome(&self)->Option<RowOutcome>{if self.phase==99{Some(RowOutcome::Invalid)}else{self.core.as_ref().and_then(|c|c.outcome())}}
 fn take_quote(&self)->Result<Quote,ValueError>{self.core()?.take_quote()}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{self.core_mut()?.take(e)}
 fn inverse_quote(&self,b:usize,m:u64)->Result<Quote,ValueError>{RowFieldPreparation::inverse_quote(self.core()?,b,m)}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{RowFieldPreparation::prepare_inverse_reply(self.core_mut()?,b,m,e,r,recover)}
 fn inverse_ready(&self)->bool{self.core.as_ref().is_some_and(|c|RowFieldPreparation::inverse_ready(&**c))}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{let(d,w)=self.core()?.inverse_quote()?;Ok((nested(d,2*size_of::<Self::Inverse>()+2*size_of::<S::Scalar>()+size_of::<S::Context>())?,w))}
 fn take_inverse(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{
  let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,e.grant()){return Ok(None);}
  let extra=2*size_of::<Self::Inverse>()+2*size_of::<S::Scalar>()+size_of::<S::Context>();let c=*self.context.get().ok_or_else(invalid)?;let g=e.grant();
  let Some((value,p))=self.core_mut()?.take_inverse(&mut e.nested(child(g,extra)))? else{return Ok(None)};Ok(Some((S::inverse(c,value),add(p,extra)?)))
 }
}
