//! ♻️ Existing receiver header allocations move and free through the reply-shaped close, running the real header destructor exactly once on the paid free.
use semio_framework_value_native_receiver_protocol::*;
use semio_framework_value_native_storage::NativeOwnedStorage;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_native_clone_engine::RetainedCursorRelease;
use std::{alloc::Layout,sync::atomic::{AtomicUsize,Ordering}};
type Reply<'a>=OriginalEffectReply<'a,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,RetainedCloneStep,OriginalRetirementFailure>;
static POISON_DROPS:AtomicUsize=AtomicUsize::new(0);
struct PoisonHeader{byte:u8}
impl Drop for PoisonHeader{fn drop(&mut self){POISON_DROPS.fetch_add(1,Ordering::SeqCst);}}
fn closed()->ValueError{ValueError::invariant("closed header performs no receiving operation")}
impl NativeReceiver<(),()> for PoisonHeader{
 fn demands(&self,_:retained_clone::RetainedCloneRef<'_,()>,_:usize,_:u64)->Result<RetirementDemand,ValueError>{Err(closed())}
 fn select_advance(&self,_:retained_clone::RetainedCloneRef<'_,()>,_:usize,_:u64)->Result<NativeAdvanceSelection,ValueError>{Err(closed())}
 fn advance_nested(&mut self,_:retained_clone::RetainedCloneRef<'_,()>,_:&mut NativeDecodeControl<'_>,_:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,_:usize,_:u64,_:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{panic!("closed header algorithm called")}
 fn advance_local<'r>(&mut self,_:retained_clone::RetainedCloneRef<'_,()>,_:usize,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Err(closed())}
 fn advance_pending(&self)->bool{false}
 fn output_ready(&self)->bool{false}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Err(closed())}
 fn take_work_demand(&self)->Result<u64,ValueError>{Err(closed())}
 fn take<'r>(&mut self,_:&mut Option<()>,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Err(closed())}
 fn is_closing(&self)->bool{true}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(0)}
 fn begin_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("closed header starts no close")}
 fn recover_begin_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("closed header starts no close")}
 fn close_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn close_step<'r>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("closed header cursor called")}
 fn recover_close_step<'r>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("closed header cursor called")}
 fn close_original_step<'r>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("closed header cursor called")}
 fn recover_close_original_step<'r>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("closed header cursor called")}
 fn terminal_is_empty(&self)->bool{true}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{Some(unsafe{RetainedCursorRelease::native(Self::terminal_is_empty)})}
}
struct Context<'a>{storage:&'a mut NativeOwnedStorage<PoisonHeader>,retired:&'a mut Option<Box<PoisonHeader>>,close:&'a mut NativeReceiverBackingClose,input:&'a mut Option<PoisonHeader>,operation:u8}
struct Producer;
impl OriginalEffectProducer for Producer{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'s>=Context<'a>where 's:'a;
 fn validate_context<'a,'s>(&self,_:&Self::Context<'a,'s>)->Result<(),ValueError>where 's:'a{Ok(())}
 fn source_turn<'a,'s>(&self,_:&Self::Context<'a,'s>)->Option<OriginalSourceTurnIdentity>where 's:'a{None}
 fn receive<'r,'a,'s>(&mut self,c:&mut Self::Context<'a,'s>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>where 's:'a{
  if c.operation==2{return c.close.close_into::<(),(),_>(c.storage,c.retired,4096,effect,reply)}
  if c.operation==0{return Ok(match c.storage.reserve_empty(effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into))})}
  Ok(reply.complete(c.storage.put_from(c.input,effect).result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)))
 }
 fn recover<'r,'a,'s>(&mut self,_:&mut Self::Context<'a,'s>,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>where 's:'a{Ok(reply.pending())}
}
struct Turn{step:RetainedCloneStep,work:u128,heap:(usize,usize)}
fn run(bank:&mut EffectBank,c:&mut Context<'_>,grant:RetainedCloneGrant,maximum:u64)->Turn{
 let mut authority=OriginalEffectAuthority::new(Producer);let mut action=None;let mut reply=None;
 let((turn,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,maximum,|effect|{let before=effect.performed_work();let complete=authority.drive(c,effect,&mut action).unwrap();let work=effect.performed_work()-before;if complete{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));
 let RetainedWorkPoll::Complete(done)=turn else{panic!("closed header close must settle")};
 assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();let step=reply.take().unwrap().unwrap();
 if let Some(actual)=actual{assert_eq!(actual,heap.0)}
 let _=bank.take_admitted();
 bank.post(step.progress(),grant,heap);
 assert!(authority.into_producer().is_ok());
 Turn{step,work:done.work,heap}
}
#[test]fn actual_closed_receiver_header_moves_its_existing_allocation_and_runs_its_destructor_once(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../../♻️header/🎟️work/📐️.json")).unwrap();
 let mut bank=EffectBank::new(Wallet::from_profile(&f["stock"]));let mut storage=NativeOwnedStorage::<PoisonHeader>::empty();let mut retired:Option<Box<PoisonHeader>>=None;let mut close=NativeReceiverBackingClose::default();let mut input=Some(PoisonHeader{byte:17});
 macro_rules! ctx{($operation:expr)=>{Context{storage:&mut storage,retired:&mut retired,close:&mut close,input:&mut input,operation:$operation}}}
 let g=bank.loan(1,NativeOwnedStorage::<PoisonHeader>::reserve_demand().copy_bytes);let t=run(&mut bank,&mut ctx!(0),g,1);assert_eq!(t.heap.0,t.step.progress().retained_capacity_bytes);assert_eq!(t.step.progress().retained_capacity_bytes,Layout::new::<PoisonHeader>().size());
 let g=bank.loan(1,4096);run(&mut bank,&mut ctx!(1),g,1);assert!(input.is_none());
 let pointer=storage.address().unwrap();
 let d=close.demands::<(),(),PoisonHeader>(&storage,&retired,4096).unwrap();
 let layout=f["handoff"].as_array().unwrap().iter().find(|r|r["pointerBytes"].as_u64()==Some(std::mem::size_of::<usize>()as u64)).unwrap();let moved=NativeOwnedStorage::<PoisonHeader>::box_demand();assert_eq!(moved.copy_bytes,layout["copyBytes"].as_u64().unwrap()as usize);let header=d.copy_bytes.checked_sub(moved.copy_bytes).unwrap();assert!(header>0);
 let full=bank.loan(1,d.copy_bytes);
 for denied in[RetainedCloneGrant{maximum_items:0,..full},RetainedCloneGrant{maximum_depth:0,..full}]{let t=run(&mut bank,&mut ctx!(2),denied,1);assert_eq!(t.work,u128::from(f["deniedWork"].as_u64().unwrap()));assert_eq!(t.step.progress(),Default::default());assert_eq!(t.heap,(0,0));assert_eq!(storage.address(),Some(pointer));assert!(retired.is_none());}
 let move_work=close.work_demand::<(),(),PoisonHeader>(&storage,&retired,4096).unwrap();assert_eq!(move_work,f["moveWork"].as_u64().unwrap());
 let t=run(&mut bank,&mut ctx!(2),full,move_work);assert_eq!(t.work,u128::from(f["moveWork"].as_u64().unwrap()));assert_eq!(t.heap,(0,0));assert_eq!(t.step.progress().copied_bytes,d.copy_bytes);assert!(storage.terminal_is_empty());assert_eq!(retired.as_ref().unwrap().byte,17);assert_eq!(&**retired.as_ref().unwrap()as*const PoisonHeader as usize,pointer);
 let d=close.demands::<(),(),PoisonHeader>(&storage,&retired,4096).unwrap();assert_eq!(d.release_bytes,Layout::new::<PoisonHeader>().size());
 let full=RetainedCloneGrant{maximum_release_bytes:bank.wallet().remaining().maximum_release_bytes,..bank.loan(1,d.copy_bytes)};
 for denied in[RetainedCloneGrant{maximum_items:0,..full},RetainedCloneGrant{maximum_depth:0,..full},RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..full}]{let t=run(&mut bank,&mut ctx!(2),denied,1);assert_eq!(t.work,u128::from(f["deniedWork"].as_u64().unwrap()));assert_eq!(t.step.progress(),Default::default());assert_eq!(t.heap,(0,0));assert!(retired.is_some());assert_eq!(POISON_DROPS.load(Ordering::SeqCst),0);}
 let free_work=close.work_demand::<(),(),PoisonHeader>(&storage,&retired,4096).unwrap();assert_eq!(free_work,f["freeWork"].as_u64().unwrap());
 let t=run(&mut bank,&mut ctx!(2),full,free_work);assert_eq!(t.work,u128::from(f["freeWork"].as_u64().unwrap()));let p=t.step.progress();assert_eq!(t.heap,(0,p.released_bytes));assert_eq!(p.released_bytes,Layout::new::<PoisonHeader>().size());assert!(retired.is_none());assert_eq!(POISON_DROPS.load(Ordering::SeqCst),f["poisonDropCalls"].as_u64().unwrap()as usize);
 assert!(storage.terminal_is_empty());
 let settle_work=close.work_demand::<(),(),PoisonHeader>(&storage,&retired,4096).unwrap();let settle=RetainedCloneGrant{maximum_release_bytes:bank.wallet().remaining().maximum_release_bytes,..bank.loan(1,4096)};let t=run(&mut bank,&mut ctx!(2),settle,settle_work);let _=t;assert!(close.terminal_is_empty());
 let t=run(&mut bank,&mut ctx!(2),Default::default(),0);assert_eq!(t.work,u128::from(f["cachedWork"].as_u64().unwrap()));
 assert!(bank.into_wallet().is_ok());
}
