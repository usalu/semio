//! 🪆️ A nested sealed branch is adopted through its actual enclosing original branch before root publication.
use super::*;
/// 🧭️ The authored parent path supplies its exact sealed field and next output owner.
pub trait NestedBranchPath:Send+Sync+'static{
 type Branch:RetireOwned+Send+Sync;
 type Parent:SnapshotProjection<Output=Box<Self::Parent>>+Send+Sync;
 type Output:BranchOutput<Self::Parent>;
 fn parent(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Parent>;
 fn select(parent:&mut Self::Parent)->&mut SealedShared<Self::Branch>;
 fn output()->Self::Output;
}
/// 🧳️ Child headers, displaced parent fields and output cursors retain their original paid custody.
pub struct NestedBranchOutput<P:NestedBranchPath>{original:Bound,staged:Staged<P::Parent>,output:Option<Box<P::Output>>,value:Held<SealedShared<P::Branch>>,displaced:Held<SealedShared<P::Branch>>,decision:Held<EnergyModelDiff>,flight:Flight,phase:u8,closing:bool}
impl<P:NestedBranchPath> NestedBranchOutput<P>{
 pub fn new()->Self{Self{original:Default::default(),staged:Default::default(),output:None,value:Default::default(),displaced:Default::default(),decision:Default::default(),flight:Default::default(),phase:0,closing:false}}
 custody_impl!(output,staged,decision,value,displaced,original);
}
impl<P:NestedBranchPath> BranchOutput<P::Branch> for NestedBranchOutput<P>{
 fn quote(&self,value:&Option<SealedShared<P::Branch>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{if self.closing||self.phase>=7||!self.flight.idle(){return Err(invalid());}original.check_binding(&self.original.0)?;if self.phase<2&&value.is_none(){return Err(invalid());}match self.phase{
  0=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),
  1=>Ok((scalar(4*size_of::<Option<SealedShared<P::Branch>>>()+1),1)),
  2=>self.staged.quote::<SealedShared<P::Branch>>(P::parent(original),body,maximum_work_units),
  3=>Ok((birth::<P::Output>(),1)),
  4=>self.output.as_ref().ok_or_else(invalid)?.quote(self.staged.sealed.get_opt(),original,body.saturating_sub(1),maximum_work_units),
  5=>{let(d,w)=self.output.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
  6=>Ok((Cursor::demand(&self.output,body)?,Cursor::work(&self.output,body)?)),
  _=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,value:&mut Option<SealedShared<P::Branch>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(value,original,body,maximum_work_units)?;
    if u>maximum_work_units||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut p=RetainedCloneProgress::default();let nested_body=body.saturating_sub(1);
   match self.phase{
    0=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=1;},
    1=>{self.value.set(value.take().ok_or_else(invalid)?);self.phase=2;p=receipt(scalar(4*size_of::<Option<SealedShared<P::Branch>>>()+1));},
    2=>{let Self{staged,value,displaced,..}=self;let mut exchange=|stage:&mut RetainedBranchProjection<P::Parent>,e:&mut RetainedAllocationEffect<'_>|stage.exchange(P::select,value.slot(),displaced.slot(),e);
     let step=staged_step::<_,SealedShared<P::Branch>>(staged,P::parent(original),body,maximum_work_units,effect,&mut work,recover,&mut exchange,&mut units)?;p=step.progress();if staged.ready(){self.phase=3;}},
    3=>{self.output=Some(Box::new(P::output()));self.phase=4;p=receipt(birth::<P::Output>());},
    4=>{let output=self.output.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| output.advance_reply(self.staged.sealed.slot(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,false),output.advance_reply(self.staged.sealed.slot(),original,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r,true));
     let step=step?;p=add(step.progress(),1)?;if output.ready(){self.phase=5;}},
    5=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let output=self.output.as_mut().ok_or_else(invalid)?;let(diff,progress)=output.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);output.begin_close();self.phase=6;p=add(progress,extra)?;},
    6=>{let step=custody::finish(&mut self.flight,&mut self.output,body,effect,&mut work,recover)?;p=step.progress();if self.output.is_none(){self.phase=7;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy enclosing branch adoption")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn ready(&self)->bool{!self.closing&&self.phase==7}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=8;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
}
crate::custody::std_cursor!([P:NestedBranchPath] NestedBranchOutput<P>,[],|s|s.begin_close());
