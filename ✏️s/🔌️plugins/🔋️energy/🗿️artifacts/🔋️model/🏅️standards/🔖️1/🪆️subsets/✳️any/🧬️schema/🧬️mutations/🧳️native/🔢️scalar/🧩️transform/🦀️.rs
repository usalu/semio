//! 🧩️ Fixed authored facets retain their original payload while a paid typed value feeds the scalar row pipeline.
use super::*;
use crate::leaf::RowFieldPreparation;
use semio_framework_value::retained_clone::StagedSourceAdmission;
use crate::custody::Sourced;
/// 🧬️ A facet supplies its real typed value and validity from bounded original fixed fields.
pub trait OriginalScalarTransform:ScalarRowPath{type Payload:Send+Sync+'static;fn value(payload:&Self::Payload)->Self::Scalar;fn valid_payload(payload:&Self::Payload)->bool;}
/// 🧳️ Original input, transformed value and scalar preparation remain live until funded closure.
pub struct TransformedScalarPreparation<S:OriginalScalarTransform>{body:Option<usize>,input:Bound,source:Sourced<S::Scalar>,admit:Held<StagedSourceAdmission<S::Scalar,S::Scalar>>,core:Option<Box<ScalarRowPreparation<S>>>,phase:u8,closing:bool,flight:Flight}
impl<S:OriginalScalarTransform> TransformedScalarPreparation<S>{
 fn source(&self)->Result<RetainedCloneRef<'_,S::Scalar>,ValueError>{Ok(self.source.get().ok_or_else(invalid)?.borrow())}
 fn core(&self)->Result<&ScalarRowPreparation<S>,ValueError>{self.core.as_deref().ok_or_else(invalid)}
 fn core_mut(&mut self)->Result<&mut ScalarRowPreparation<S>,ValueError>{self.core.as_deref_mut().ok_or_else(invalid)}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(core,admit,source,input);
}
crate::custody::std_cursor!([S:OriginalScalarTransform] TransformedScalarPreparation<S>,[],|s|s.begin_close());
impl<S:OriginalScalarTransform> RowFieldPreparation for TransformedScalarPreparation<S>{
 type Input=S::Payload;type Inverse=S::Scalar;
 fn new()->Self{Self{body:None,input:Default::default(),source:Default::default(),admit:Default::default(),core:None,phase:0,closing:false,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.input.0)?;if let Some(core)=self.core.as_ref(){core.check(self.source()?,key.ok_or_else(invalid)?,o)?;}Ok(())}
 fn quote(&self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64)->Result<Quote,ValueError>{
  if self.closing||self.body.is_some_and(|v|v!=body)||!self.flight.idle(){return Err(invalid());}self.check(p,key,o)?;
  match self.phase{
   0=>Ok((nested(scalar(p.binding_copy_bytes()+size_of::<usize>()),1)?,p.binding_work_demand(&self.input.0)?)),
   1=>{let extra=2*size_of::<S::Payload>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<S::Scalar>>>()+2*size_of::<Option<StagedSourceAdmission<S::Scalar,S::Scalar>>>()+1;match self.admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<S::Scalar,S::Scalar>>()+extra,depth:1,..Default::default()},1))}},
   2=>{let mut d=birth::<ScalarRowPreparation<S>>();d.copy_bytes+=2*size_of::<S::Payload>();Ok((d,1))},
   3=>self.core()?.quote(self.source()?,key.ok_or_else(invalid)?,o,body,m),
   _=>Err(invalid()),
  }
 }
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(p,key,o,body,m)?;
    if u>m||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut progress=RetainedCloneProgress::default();
   match self.phase{
    0=>{progress=add(bind(p,&mut self.input,&mut effect.nested(child(g,1+size_of::<usize>())),&mut work)?,1+size_of::<usize>())?;self.phase=1;},
    1=>{if self.admit.is_none()&&self.source.get().is_none(){self.admit.set(StagedSourceAdmission::owned(S::value(p.get())));progress=receipt(scalar(1));}else{let a=self.admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;progress=turn.result?.progress();if a.terminal_is_empty(){drop(self.admit.take());}}if self.source.get().is_some(){self.phase=2;}},
    2=>{self.core=Some(Box::new(ScalarRowPreparation::with_authored_validation(S::valid_payload(p.get()))));self.phase=3;progress=receipt(birth::<ScalarRowPreparation<S>>());},
    3=>{let Self{core,source,flight,..}=self;let core=core.as_deref_mut().ok_or_else(invalid)?;let src=source.get().ok_or_else(invalid)?;let key=key.ok_or_else(invalid)?;let nested_body=body.saturating_sub(1);
     let step=protocol::retained_routed!(*flight,effect,recover,1,body,m,None,work,|r| core.advance_reply(src.borrow(),key,o,nested_body,m,&mut effect.nested(child(g,1)),r,false),core.advance_reply(src.borrow(),key,o,nested_body,m,&mut effect.nested(child(g,1)),r,true));
     progress=add(step?.progress(),1)?;},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,progress,"original fixed Energy facet")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.core.as_ref().is_some_and(|c|c.ready()){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn ready(&self)->bool{!self.closing&&self.core.as_ref().is_some_and(|c|c.ready())}fn outcome(&self)->Option<ScalarOutcome>{self.core.as_ref().and_then(|c|c.outcome())}
 fn take_quote(&self)->Result<Quote,ValueError>{self.core()?.take_quote()}fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{self.core_mut()?.take(e)}
 fn inverse_quote(&self,b:usize,m:u64)->Result<Quote,ValueError>{RowFieldPreparation::inverse_quote(self.core()?,b,m)}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{RowFieldPreparation::prepare_inverse_reply(self.core_mut()?,b,m,e,r,recover)}
 fn inverse_ready(&self)->bool{self.core.as_ref().is_some_and(|c|RowFieldPreparation::inverse_ready(&**c))}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{self.core()?.inverse_quote()}
 fn take_inverse(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{self.core_mut()?.take_inverse(e)}
}
