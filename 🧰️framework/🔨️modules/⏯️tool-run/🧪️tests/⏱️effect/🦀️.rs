//! ⏱️ Reply-shaped test turns run each action as one actual original effect over the shared bank and observe its heap.
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,retained_clone::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use std::mem::{size_of,ManuallyDrop};

pub type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
pub type Done<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
type CallFn<'t>=dyn for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,Reply<'r>,bool)->Result<Done<'r>,ValueError>+'t;
pub struct Call<'t>(&'t mut CallFn<'t>);
pub struct Action{turn:OriginalSourceTurnIdentity}
impl OriginalEffectProducer for Action{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'turn,'source>=Call<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Call<'turn>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Call<'turn>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn receive<'r,'turn,'source>(&mut self,c:&mut Call<'turn>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Done<'r>,ValueError>where 'source:'turn{(c.0)(e,r,false)}
 fn recover<'r,'turn,'source>(&mut self,c:&mut Call<'turn>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Done<'r>,ValueError>where 'source:'turn{(c.0)(e,r,true)}
}
pub const UNITS:u64=64;
thread_local!{static STALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)}}
fn track(progress:RetainedCloneProgress,g:RetainedCloneGrant,refusal:Option<&OriginalRetirementFailure>){STALLS.with(|n|if progress==RetainedCloneProgress::default(){n.set(n.get()+1);assert!(n.get()<32,"stalled: 32 consecutive turns without progress under {g:?}; last refusal {refusal:?}");}else{n.set(0)});}
pub fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(result,(heap.requested_bytes,heap.released_bytes))}
pub fn bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<40,maximum_copy_bytes:1<<40,maximum_capacity_bytes:1<<40,maximum_release_bytes:1<<40,maximum_depth:1<<20},1<<40))}
pub fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🧾️ The observed result of one paid original action.
pub struct Observed{pub progress:RetainedCloneProgress,pub work:u128,pub heap:(usize,usize),pub complete:bool}
/// 🪜️ Runs one reply-shaped action to its recorded reply under grant `g`, pending turns included, and settles the reply transfer.
pub fn paid(bank:&mut EffectBank,g:RetainedCloneGrant,mut call:impl for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,Reply<'r>,bool)->Result<Done<'r>,ValueError>)->Observed{
 let call:&mut CallFn<'_>=&mut call;
 let turn=bank.source_witness().identity();
 let mut authority=ManuallyDrop::new(OriginalEffectAuthority::new(Action{turn}));
 let mut context=Call(call);
 let before=*bank.wallet().original_work();let(mut action,mut output)=(None,None);let mut heap=(0usize,0usize);
 for _ in 0..256{
  let((poll,actual),observed)=measured(||bank.run(g,UNITS,|effect|{let boundary=effect.performed_boundary();let ready=authority.drive(&mut context,effect,&mut action).unwrap();let work=effect.performed_since(boundary).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));
  heap.0+=observed.0;heap.1+=observed.1;if let Some(actual)=actual{assert_eq!(actual,heap.0)}if matches!(poll,RetainedWorkPoll::Pending{..}){continue}
  let step=action.as_ref().unwrap().recorded_reply().unwrap();
  let(progress,complete)=match &step{Ok(step)=>(step.progress(),matches!(step,RetainedCloneStep::Complete(_))),Err(error)=>(error.retained_progress(),false)};
  track(progress,g,step.as_ref().err());assert!(progress.fits(g));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,g,heap);
  let work=before-*bank.wallet().original_work();
  let moved=grant(OriginalEffectAuthority::<Action>::transfer_reply_demand().unwrap());let((poll,actual),transfer)=measured(||bank.run(moved,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(&context,&mut action,&mut output,effect))));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("test transfer pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap().unwrap(),moved,transfer);
  let demand=RetirementDemand{copy_bytes:2*size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>(),depth:1,..Default::default()};let released=grant(demand);let mut recorded=None;
  let((poll,actual),reply)=measured(||bank.run(released,1,|effect|{assert!(effect.performed(1).unwrap());recorded=output.take();RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work:1})}));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("test reply retirement pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap(),released,reply);
  assert!(recorded.is_some());return Observed{progress,work,heap,complete};
 }
 panic!("test action exceeded its original finite bound")
}
/// 🚧️ Fails fast when a test loop keeps paying turns that make no progress, naming the loop.
pub fn expect_progress(o:&Observed,stalls:&mut usize,what:&str){if o.progress==RetainedCloneProgress::default(){*stalls+=1;assert!(*stalls<16,"stalled: {what} work={} complete={}",o.work,o.complete);}else{*stalls=0;}}
/// ♻️ Retires one owner through its controlled cursor under paid original actions and returns its heap events.
pub fn retire<T:RetireOwned>(bank:&mut EffectBank,value:T)->(usize,usize){
 let mut owner=ControlledRetirement::retain(value);let(mut born,mut released)=(0,0);
 for _ in 0..1<<20{if owner.terminal_is_empty(){break}let d=owner.next_demand(0).unwrap();let g=grant(d);let observed=paid(bank,g,|e,r,recover|if recover{owner.recover_step_into(0,e,r)}else{owner.step_into(0,e,r)});born+=observed.heap.0;released+=observed.heap.1;}
 assert!(owner.terminal_is_empty());assert_eq!(measured(||drop(owner)).1,(0,0));(born,released)
}
/// 🏷️ Runs one primitive turn inside one actual original effect and observes its heap.
pub fn primitive<T>(bank:&mut EffectBank,g:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<T,ValueError>)->(Result<T,ValueError>,(usize,usize)){
 let((poll,_),heap)=measured(||bank.run(g,UNITS,|effect|{let boundary=effect.performed_boundary();let result=operation(effect);let work=effect.performed_since(boundary).unwrap();RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("test primitive pending")};assert_ne!(bank.take_admitted(),Some(false));(turn.result,heap)
}
