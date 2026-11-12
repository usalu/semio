//! 🔹️ Fixed singleton fields prepare one original sealed child and its enclosing branch without row mirrors.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::RetainedBranchProjection},diff::original::EnergyModelDiff,row::{BranchOutput,RowOutcome,Staged,staged_step},scalar::{NativeScalar,Quote,birth,invalid,scalar,nested,child,add,receipt,bind},leaf::RowFieldPreparation,custody::{self,Cursor,Held,Bound},retained::{Done,Flight,Reply,conclude}};
use semio_framework_value::{RetainedAllocationEffect,RetirementPollExit,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use protocol::retained_mutation_demand_fits;
use std::mem::size_of;
/// 🧭️ Authored scalar and contextual limits name one actual field in its original singleton branch.
pub trait SingleFieldPath:Send+Sync+'static{
 type Scalar:NativeScalar;type Branch:SnapshotProjection<Output:crate::snapshot::state::fields::BranchProjectionOutput<Self::Branch>>+Send+Sync+RetireOwned;type Output:BranchOutput<Self::Branch>;
 fn branch(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>;
 fn read(branch:&Self::Branch)->Self::Scalar;fn select(branch:&mut Self::Branch)->&mut Self::Scalar;
 fn valid(value:Self::Scalar)->bool;
 fn valid_context(_:Self::Scalar,_:&Self::Branch)->bool{true}
 fn output()->Self::Output;
}
/// 🧳️ Candidate scalar, displaced original and enclosing output remain independently funded and closeable.
pub struct SingleFieldPreparation<S:SingleFieldPath>{original:Bound,authored:Bound,staged:Staged<S::Branch>,output:Option<Box<S::Output>>,value:Held<S::Scalar>,inverse:Held<S::Scalar>,decision:Held<EnergyModelDiff>,body:Option<usize>,phase:u8,outcome:Option<RowOutcome>,closing:bool,flight:Flight}
impl<S:SingleFieldPath> SingleFieldPreparation<S>{
 pub fn new()->Self{Self{original:Default::default(),authored:Default::default(),staged:Default::default(),output:None,value:Default::default(),inverse:Default::default(),decision:Default::default(),body:None,phase:0,outcome:None,closing:false,flight:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==99}
 pub fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn check(&self,input:RetainedCloneRef<'_,S::Scalar>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{input.check_binding(&self.authored.0)?;original.check_binding(&self.original.0)?;Ok(())}
 pub fn quote(&self,input:RetainedCloneRef<'_,S::Scalar>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||!self.flight.idle(){return Err(invalid());}self.check(input,original)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}
  match self.phase{
   0=>Ok((nested(scalar(original.binding_copy_bytes()+if self.body.is_none(){size_of::<usize>()}else{0}),1)?,original.binding_work_demand(&self.original.0)?)),
   1=>Ok((nested(scalar(input.binding_copy_bytes()),1)?,input.binding_work_demand(&self.authored.0)?)),
   2|3|4=>Ok((scalar(2*size_of::<S::Branch>()+2*size_of::<S::Scalar>()+2*size_of::<Option<S::Scalar>>()+3),1)),
   5=>self.staged.quote::<S::Scalar>(S::branch(original),body,maximum_work_units),
   6=>Ok((birth::<S::Output>(),1)),
   7=>self.output.as_ref().ok_or_else(invalid)?.quote(self.staged.sealed.get_opt(),original,body.saturating_sub(1),maximum_work_units),
   8=>{let(d,w)=self.output.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
   9=>Ok((Cursor::demand(&self.output,body)?,Cursor::work(&self.output,body)?)),
   _=>Err(invalid()),
  }
 }
 pub fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,S::Scalar>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(input,original,body,maximum_work_units)?;
    if u>maximum_work_units||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();
   match self.phase{
    0=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1+body_birth)),&mut work)?,1+body_birth)?;self.phase=1;},
    1=>{p=add(bind(input,&mut self.authored,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{if S::valid(*input.get()){self.phase=3;}else{self.outcome=Some(RowOutcome::Invalid);self.phase=99;}p=receipt(scalar(1));},
    3=>{if S::read(S::branch(original).get())==*input.get(){self.outcome=Some(RowOutcome::Unchanged);self.phase=99;}else{self.phase=4;}p=receipt(scalar(1));},
    4=>{if S::valid_context(*input.get(),S::branch(original).get()){self.value.set(*input.get());self.phase=5;}else{self.outcome=Some(RowOutcome::InvalidContext);self.phase=99;}p=receipt(scalar(1));},
    5=>{let Self{staged,value,inverse,..}=self;let mut exchange=|stage:&mut RetainedBranchProjection<S::Branch>,e:&mut RetainedAllocationEffect<'_>|stage.exchange(S::select,value.slot(),inverse.slot(),e);
     let step=staged_step::<_,S::Scalar>(staged,S::branch(original),body,maximum_work_units,effect,&mut work,recover,&mut exchange,&mut units)?;p=step.progress();if staged.ready(){self.phase=6;}},
    6=>{self.output=Some(Box::new(S::output()));self.phase=7;p=receipt(birth::<S::Output>());},
    7=>{let output=self.output.as_mut().ok_or_else(invalid)?;let nested_body=body.saturating_sub(1);
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| output.advance_reply(self.staged.sealed.slot(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,false),output.advance_reply(self.staged.sealed.slot(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,true));
     let step=step?;p=add(step.progress(),1)?;if output.ready(){self.phase=8;}},
    8=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let output=self.output.as_mut().ok_or_else(invalid)?;let(diff,progress)=output.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);output.begin_close();self.phase=9;p=add(progress,extra)?;},
    9=>{let step=custody::finish(&mut self.flight,&mut self.output,body,effect,&mut work,recover)?;p=step.progress();if self.output.is_none(){self.outcome=Some(RowOutcome::Changed);self.phase=99;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy singleton field")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn inverse_quote(&self)->Result<Quote,ValueError>{if self.closing||self.phase!=100||self.inverse.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<S::Scalar>>()+1),1))}
 pub fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(S::Scalar,RetainedCloneProgress)>,ValueError>{let(d,_)=self.inverse_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}Ok(Some((self.inverse.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(output,staged,decision,value,inverse,original,authored);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:SingleFieldPath] SingleFieldPreparation<S>,[],|s|s.begin_close());
impl<S:SingleFieldPath> RowFieldPreparation for SingleFieldPreparation<S>{
 type Input=S::Scalar;type Inverse=S::Scalar;
 fn new()->Self{Self::new()}
 fn check(&self,v:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if key.is_some(){return Err(invalid());}Self::check(self,v,o)}
 fn quote(&self,v:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{Self::quote(self,v,o,b,m)}
 fn advance_reply<'r>(&mut self,v:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{Self::advance_reply(self,v,o,b,m,e,r,recover)}
 fn ready(&self)->bool{Self::ready(self)}fn outcome(&self)->Option<RowOutcome>{Self::outcome(self)}
 fn take_quote(&self)->Result<Quote,ValueError>{Self::take_quote(self)}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{Self::take(self,e)}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{Self::inverse_quote(self).map(|_|(scalar(1),1))}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.inverse.is_some()}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{Self::inverse_quote(self)}
 fn take_inverse(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{Self::take_inverse(self,e)}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🪆️leaf/🦀️.rs"]mod leaf;pub use leaf::{SingleLeafSpec,SingleLeafPreparation};

#[path="🧩️transform/🦀️.rs"]mod transform;pub use transform::{OriginalSingleTransform,TransformedSinglePreparation};
