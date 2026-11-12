//! 🤝️ Multiple authored restrictions close each original scan before admitting the next actual source.
use super::*;
/// 🧵️ Child frames are born and retired independently under the same original deletion caller.
pub struct AllRowReferences<A:Cursor,B:Cursor>{left:Option<Box<A>>,right:Option<Box<B>>,allowed:Option<bool>,phase:u8,closing:bool,flight:Flight}
impl<A:Cursor,B:Cursor> AllRowReferences<A,B>{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(right,left);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([A:Cursor,B:Cursor] AllRowReferences<A,B>,[],|s|s.begin_close());
impl<R:EntityRow,A:RowDeletionGuard<R>,B:RowDeletionGuard<R>> RowDeletionGuard<R> for AllRowReferences<A,B>{
 fn new()->Self{Self{left:None,right:None,allowed:None,phase:0,closing:false,flight:Default::default()}}
 fn quote(&self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64)->Result<Quote,ValueError>{if self.closing||!self.flight.idle(){return Err(invalid());}match self.phase{
  0=>Ok((birth::<A>(),1)),1=>{let(d,w)=self.left.as_ref().ok_or_else(invalid)?.quote(row,original,body.saturating_sub(2),m)?;Ok((nested(d,2)?,w))},2=>Ok((nested(Cursor::demand(&self.left,body.saturating_sub(1))?,1)?,Cursor::work(&self.left,body.saturating_sub(1))?)),
  3=>Ok((birth::<B>(),1)),4=>{let(d,w)=self.right.as_ref().ok_or_else(invalid)?.quote(row,original,body.saturating_sub(2),m)?;Ok((nested(d,2)?,w))},5=>Ok((nested(Cursor::demand(&self.right,body.saturating_sub(1))?,1)?,Cursor::work(&self.right,body.saturating_sub(1))?)),_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=<Self as RowDeletionGuard<R>>::quote(self,row,original,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(2);
   match self.phase{
    0=>{self.left=Some(Box::new(A::new()));self.phase=1;p=receipt(birth::<A>());},
    1=>{let c=self.left.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(row,original,nb,max,&mut effect.nested(child(g,2)),r,false),c.advance_reply(row,original,nb,max,&mut effect.nested(child(g,2)),r,true));p=add(step?.progress(),2)?;if let Some(allowed)=c.result(){self.allowed=Some(allowed);RowDeletionGuard::<R>::begin_close(c.as_mut());self.phase=2;}},
    2=>{let step=custody::finish(&mut self.flight,&mut self.left,body.saturating_sub(1),effect,&mut work,recover)?;p=add(step.progress(),1)?;if self.left.is_none(){self.phase=if self.allowed==Some(false){99}else{3};}},
    3=>{self.right=Some(Box::new(B::new()));self.phase=4;p=receipt(birth::<B>());},
    4=>{let c=self.right.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(row,original,nb,max,&mut effect.nested(child(g,2)),r,false),c.advance_reply(row,original,nb,max,&mut effect.nested(child(g,2)),r,true));p=add(step?.progress(),2)?;if let Some(allowed)=c.result(){self.allowed=Some(allowed);RowDeletionGuard::<R>::begin_close(c.as_mut());self.phase=5;}},
    5=>{let step=custody::finish(&mut self.flight,&mut self.right,body.saturating_sub(1),effect,&mut work,recover)?;p=add(step.progress(),1)?;if self.right.is_none(){self.phase=99;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy combined reference restrictions")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn result(&self)->Option<bool>{if self.phase==99&&!self.closing{self.allowed}else{None}}
}
