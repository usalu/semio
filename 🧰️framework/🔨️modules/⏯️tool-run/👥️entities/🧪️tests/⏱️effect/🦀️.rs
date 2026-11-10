//! ⏱️ Entity edit laws run every turn inside one actual original Source/Work action over the shared effect bank.
use crate::*;
use semio_framework_value::retained_clone::{OriginalEffectAuthority,OriginalEffectProducer,OriginalSourceTurnIdentity,RetainedWorkOutcome};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::marker::PhantomData;

pub fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(result,(heap.requested_bytes,heap.released_bytes))}
pub fn bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<40,maximum_copy_bytes:1<<40,maximum_capacity_bytes:1<<40,maximum_release_bytes:1<<40,maximum_depth:1<<20},1<<40))}
pub const UNITS:u64=64;
pub fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub fn denials(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut out=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes!=0{out.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g})}if d.capacity_bytes!=0{out.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g})}if d.release_bytes!=0{out.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g})}if d.depth!=0{out.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g})}out}
pub fn edit_grant(d:RetirementDemand,bound:usize,entry:usize,comparison:usize)->ToolRunEntityEditGrant{ToolRunEntityEditGrant{retirement:grant(d),comparison:BoundedOrdGrant{maximum_items:1,maximum_bytes:comparison},maximum_moved_items:17,maximum_moved_bytes:17*entry,maximum_capacity_bytes:bound}}

/// 🧰️ A reply-shaped entity job under test.
pub trait Job:RetireOwned+Send+'static{
 fn complete(&self)->bool;
 fn quote(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn turn<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>;
}
impl<K:BoundedOrd+RetainedClone+Copy> Job for ToolRunEntityEdit<K>{
 fn complete(&self)->bool{ToolRunEntityEdit::complete(self)}
 fn quote(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_demands(body)}
 fn turn<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{if recover{self.recover_advance_into(body,grant,effect,reply)}else{self.advance_into(body,grant,effect,reply)}}
}
impl Job for crate::version::ToolRunEntityVersionEdit{
 fn complete(&self)->bool{crate::version::ToolRunEntityVersionEdit::complete(self)}
 fn quote(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_demands(body)}
 fn turn<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{if recover{self.recover_advance_into(body,grant,effect,reply)}else{self.advance_into(body,grant,effect,reply)}}
}
impl Edited for crate::version::ToolRunEntityVersionEdit{fn editing(&self)->bool{crate::version::ToolRunEntityVersionEdit::editing(self)}}
/// ✏️ A job whose clone phase and edit phase are distinguishable.
pub trait Edited:Job{fn editing(&self)->bool;}
impl Job for crate::provenance::ToolRunEntityProvenanceEdit{
 fn complete(&self)->bool{crate::provenance::ToolRunEntityProvenanceEdit::complete(self)}
 fn quote(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_demands(body)}
 fn turn<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{if recover{self.recover_advance_into(body,grant,effect,reply)}else{self.advance_into(body,grant,effect,reply)}}
}
pub struct Driver<J>(PhantomData<J>);
pub struct Context<'a,J>{job:&'a mut J,grant:ToolRunEntityEditGrant,body:usize,source:OriginalSourceTurnIdentity,output:Option<&'a mut Option<EntityMap>>}
impl<J:Job> OriginalEffectProducer for Driver<J>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Context<'a,J> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>where 'b:'a{c.job.turn(c.body,c.grant,e,r,false)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>where 'b:'a{c.job.turn(c.body,c.grant,e,r,true)}
}
pub struct TakeDriver;
pub struct TakeContext<'a>{job:&'a mut ToolRunEntityEdit,output:&'a mut Option<EntityMap>,source:OriginalSourceTurnIdentity}
impl OriginalEffectProducer for TakeDriver{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=TakeContext<'a> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>where 'b:'a{c.job.take_output_into(c.output,e,r)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:&mut RetainedAllocationEffect<'_>,_:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>where 'b:'a{Err(ValueError::invariant("entity take has no unrecorded callback"))}
}
pub struct RetireDriver<T:RetireOwned>(PhantomData<T>);
impl<T:RetireOwned> OriginalEffectProducer for RetireDriver<T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=(&'a mut ControlledRetirement<T>,OriginalSourceTurnIdentity,usize) where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.1)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>where 'b:'a{c.0.step_into(c.2,e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>where 'b:'a{c.0.recover_step_into(c.2,e,r)}
}
/// 🧾️ The observed result of one paid original action.
pub struct Observed{pub progress:RetainedCloneProgress,pub work:u128,pub heap:(usize,usize)}
fn paid<P:OriginalEffectProducer<Output=RetainedCloneStep,Failure=OriginalRetirementFailure>>(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<P>,context:&mut P::Context<'_,'_>,g:RetainedCloneGrant)->Observed{
 let before=*bank.wallet().original_work();let(mut action,mut output)=(None,None);let mut heap=(0usize,0usize);
 for _ in 0..256{
  let((poll,actual),observed)=measured(||bank.run(g,UNITS,|effect|{let boundary=effect.performed_boundary();let ready=authority.drive(context,effect,&mut action).unwrap();let work=effect.performed_since(boundary).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));
  heap.0+=observed.0;heap.1+=observed.1;if let Some(actual)=actual{assert_eq!(actual,heap.0)}if matches!(poll,RetainedWorkPoll::Pending{..}){continue}
  let progress=match action.as_ref().unwrap().recorded_reply().unwrap(){Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
  assert!(progress.fits(g));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,g,heap);
  let work=before-*bank.wallet().original_work();
  let moved=grant(OriginalEffectAuthority::<P>::transfer_reply_demand().unwrap());let((poll,actual),transfer)=measured(||bank.run(moved,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(context,&mut action,&mut output,effect))));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("entity transfer pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap().unwrap(),moved,transfer);
  let demand=RetirementDemand{copy_bytes:2*size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>(),depth:1,..Default::default()};let released=grant(demand);let mut recorded=None;
  let((poll,actual),reply)=measured(||bank.run(released,1,|effect|{assert!(effect.performed(1).unwrap());recorded=output.take();RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work:1})}));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("entity reply retirement pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap(),released,reply);
  assert!(recorded.is_some());return Observed{progress,work,heap};
 }
 panic!("entity action exceeded its original finite bound")
}
/// 🪜️ Runs one turn of an entity job under its edit grant.
pub fn turn<J:Job>(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<Driver<J>>,job:&mut J,body:usize,grant:ToolRunEntityEditGrant)->Observed{let source=bank.source_witness().identity();paid(bank,authority,&mut Context{job,grant,body,source,output:None},grant.retirement)}
/// 📤️ Runs one candidate transfer of an entity edit.
pub fn take(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<TakeDriver>,job:&mut ToolRunEntityEdit,output:&mut Option<EntityMap>,g:RetainedCloneGrant)->Observed{let source=bank.source_witness().identity();paid(bank,authority,&mut TakeContext{job,output,source},g)}
/// 🪪️ Authorities are never retired by these fixtures, so their drop check is released.
pub fn authority<J:Job>()->std::mem::ManuallyDrop<OriginalEffectAuthority<Driver<J>>>{std::mem::ManuallyDrop::new(OriginalEffectAuthority::new(Driver(PhantomData)))}
pub fn taker()->std::mem::ManuallyDrop<OriginalEffectAuthority<TakeDriver>>{std::mem::ManuallyDrop::new(OriginalEffectAuthority::new(TakeDriver))}
/// 🏷️ Runs one sealed primitive turn inside one actual original effect and observes its heap.
pub fn primitive<T>(bank:&mut EffectBank,g:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<T,ValueError>)->(Result<T,ValueError>,(usize,usize)){
 let((poll,actual),heap)=measured(||bank.run(g,UNITS,|effect|{let boundary=effect.performed_boundary();let result=operation(effect);let work=effect.performed_since(boundary).unwrap();RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("entity primitive pending")};let _=actual;assert_ne!(bank.take_admitted(),Some(false));(turn.result,heap)
}
/// ♻️ Retires one owner through its controlled cursor under paid original actions and returns its heap events.
pub fn retire<T:RetireOwned>(bank:&mut EffectBank,value:T)->(usize,usize){
 let mut owner=ControlledRetirement::retain(value);let mut authority=std::mem::ManuallyDrop::new(OriginalEffectAuthority::new(RetireDriver::<T>(PhantomData)));let(mut born,mut released)=(0,0);
 for _ in 0..50000{if owner.terminal_is_empty(){break}let d=owner.next_demand(0).unwrap();let g=grant(d);let source=bank.source_witness().identity();let observed=paid(bank,&mut authority,&mut(&mut owner,source,0),g);born+=observed.heap.0;released+=observed.heap.1;}
 assert!(owner.terminal_is_empty());assert_eq!(measured(||drop(owner)).1,(0,0));(born,released)
}
