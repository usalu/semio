//! ✅️ Named creation keeps the original Unicode and collection uniqueness rules before fixed scalar checks.
use super::*;
use crate::text::{TextRowPath,TextValidation};
/// 🏷️ Authored create rules borrow their actual name and identity fields.
pub trait NamedCreationRule:TextRowPath{type Payload:Send+Sync+'static;fn name(p:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,String>;fn key(p:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,u32>;fn valid(p:&Self::Payload)->bool;}
/// 🧳️ The existing bounded text policy retains every source until original cancellation completes.
pub struct NamedRowCreationValidation<S:NamedCreationRule>{text:TextValidation<S>,input:Bound,original:Bound,phase:u8,result:Option<Option<RowOutcome>>,closing:bool,flight:Flight}
impl<S:NamedCreationRule> NamedRowCreationValidation<S>{
 pub fn new()->Self{Self{text:TextValidation::new(),input:Default::default(),original:Default::default(),phase:0,result:None,closing:false,flight:Default::default()}}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(text,input,original);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:NamedCreationRule] NamedRowCreationValidation<S>,[],|s|s.begin_close());
impl<S:NamedCreationRule> RowCreationValidation<S::Payload> for NamedRowCreationValidation<S>{
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{if self.closing||self.result.is_some()||!self.flight.idle(){return Err(invalid());}p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;match self.phase{
  0=>Ok((nested(scalar(p.binding_copy_bytes()),1)?,p.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),
  2=>{let extra=2*size_of::<Option<RowOutcome>>()+1;let(d,w)=self.text.quote_original(S::name(p),S::key(p),o,b.saturating_sub(extra),m)?;Ok((nested(d,extra)?,w))},
  3=>Ok((scalar(2*size_of::<S::Payload>()+2*size_of::<Option<RowOutcome>>()+1),1)),_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,payload:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.quote(payload,o,b,m))};let g=effect.grant();
  custody::gated(effect,reply,quote,m,|effect,work|{let p=match self.phase{
   0=>{let p=add(bind(payload,&mut self.input,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=1;p},
   1=>{let p=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;p},
   2=>{let extra=2*size_of::<Option<RowOutcome>>()+1;let nb=b.saturating_sub(extra);let Self{text,flight,..}=self;
    let step=protocol::retained_routed!(*flight,effect,recover,1,b,m,None,*work,|r| text.advance_original_reply(S::name(payload),S::key(payload),o,nb,m,&mut effect.nested(child(g,extra)),r,false),text.advance_original_reply(S::name(payload),S::key(payload),o,nb,m,&mut effect.nested(child(g,extra)),r,true));
    let p=add(step?.progress(),extra)?;if let Some(result)=self.text.result(){if result.is_some(){self.result=Some(result);}else{self.phase=3;}}p},
   3=>{self.result=Some((!S::valid(payload.get())).then_some(RowOutcome::Invalid));receipt(scalar(1))},
   _=>return Err(invalid().into())};
   Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn result(&self)->Option<Option<RowOutcome>>{self.result}
}

/// 🔢️ Fixed creation rules inspect only the authored bounded scalar payload.
pub trait FixedCreationRule:Send+Sync+'static{type Payload:Send+Sync+'static;fn valid(p:&Self::Payload)->bool;}
/// 🧳️ Even fixed checks bind their actual payload and document before the semantic decision.
pub struct FixedRowCreationValidation<S:FixedCreationRule>{input:Bound,original:Bound,phase:u8,result:Option<Option<RowOutcome>>,closing:bool,flight:Flight,marker:std::marker::PhantomData<fn()->S>}
impl<S:FixedCreationRule> FixedRowCreationValidation<S>{
 pub fn new()->Self{Self{input:Default::default(),original:Default::default(),phase:0,result:None,closing:false,flight:Default::default(),marker:Default::default()}}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(input,original);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:FixedCreationRule] FixedRowCreationValidation<S>,[],|s|s.begin_close());
impl<S:FixedCreationRule> RowCreationValidation<S::Payload> for FixedRowCreationValidation<S>{
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.result.is_some()||!self.flight.idle(){return Err(invalid());}p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;match self.phase{0=>Ok((nested(scalar(p.binding_copy_bytes()),1)?,p.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),2=>Ok((scalar(2*size_of::<S::Payload>()+2*size_of::<Option<RowOutcome>>()+1),1)),_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,payload:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.quote(payload,o,b,m))};let g=effect.grant();
  custody::gated(effect,reply,quote,m,|effect,work|{let p=match self.phase{
   0=>{let p=add(bind(payload,&mut self.input,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=1;p},
   1=>{let p=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;p},
   2=>{self.result=Some((!S::valid(payload.get())).then_some(RowOutcome::Invalid));receipt(scalar(1))},
   _=>return Err(invalid().into())};
   Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn result(&self)->Option<Option<RowOutcome>>{self.result}
}

#[path="🧭️rules/🦀️.rs"]mod rules;pub use rules::{CreationRule,CreationRuleSource,RuleRowCreationValidation};
