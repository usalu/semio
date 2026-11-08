//! 🧳️ Actual receiver backing remains resident through original callback unwind and real header destruction.
use semio_framework_value_native_receiver_protocol::*;
use semio_framework_value_native_storage::NativeOwnedStorage;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_native_clone_engine::RetainedCursorRelease;
use std::{mem::size_of,sync::atomic::{AtomicUsize,Ordering}};
type Reply<'a>=OriginalEffectReply<'a,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,RetainedCloneStep,OriginalRetirementFailure>;
static CLOSE:AtomicUsize=AtomicUsize::new(0);
static RECOVER:AtomicUsize=AtomicUsize::new(0);
static DROPS:AtomicUsize=AtomicUsize::new(0);
static UNWIND:AtomicUsize=AtomicUsize::new(0);
struct Receiver{closing:bool,empty:bool}
fn invalid()->ValueError{ValueError::invariant("backing fixture has no receiving operation")}
impl NativeReceiver<(),u64> for Receiver{
 fn demands(&self,_:retained_clone::RetainedCloneRef<'_,()>,_:usize,_:u64)->Result<RetirementDemand,ValueError>{Err(invalid())}
 fn select_advance(&self,_:retained_clone::RetainedCloneRef<'_,()>,_:usize,_:u64)->Result<NativeAdvanceSelection,ValueError>{Err(invalid())}
 fn advance_nested(&mut self,_:retained_clone::RetainedCloneRef<'_,()>,_:&mut NativeDecodeControl<'_>,_:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,_:usize,_:u64,_:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{panic!("backing does not receive values")}
 fn advance_local<'r>(&mut self,_:retained_clone::RetainedCloneRef<'_,()>,_:usize,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Err(invalid())}
 fn advance_pending(&self)->bool{false}
 fn output_ready(&self)->bool{false}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Err(invalid())}
 fn take_work_demand(&self)->Result<u64,ValueError>{Err(invalid())}
 fn take<'r>(&mut self,_:&mut Option<u64>,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Err(invalid())}
 fn is_closing(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{assert!(effect.performed(1)?);self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}))))}
 fn recover_begin_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("recorded close-start must not replay")}
 fn close_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{self.begin_close_demand()}
 fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{Ok(1)}
 fn close_step<'r>(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{CLOSE.fetch_add(1,Ordering::SeqCst);assert!(self.closing&&!self.empty);assert!(effect.performed(1)?);self.empty=true;let done=reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()})));if UNWIND.load(Ordering::SeqCst)!=0{panic!("recorded original receiver close interrupted")}Ok(done)}
 fn recover_close_step<'r>(&mut self,_body:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{RECOVER.fetch_add(1,Ordering::SeqCst);panic!("recorded receiver close must not replay")}
 fn close_original_step<'r>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close_step(body,effect,reply)}
 fn recover_close_original_step<'r>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.recover_close_step(body,effect,reply)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.empty}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(Self::terminal_is_empty)})}
}
impl Drop for Receiver{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking());DROPS.fetch_add(1,Ordering::SeqCst);}}
struct Context<'a>{storage:&'a mut NativeOwnedStorage<Receiver>,retired:&'a mut Option<Box<Receiver>>,close:&'a mut NativeReceiverBackingClose,operation:u8}
struct Producer;
impl OriginalEffectProducer for Producer{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'s>=Context<'a>where 's:'a;
 fn validate_context<'a,'s>(&self,_:&Self::Context<'a,'s>)->Result<(),ValueError>where 's:'a{Ok(())}
 fn source_turn<'a,'s>(&self,_:&Self::Context<'a,'s>)->Option<OriginalSourceTurnIdentity>where 's:'a{None}
 fn receive<'r,'a,'s>(&mut self,c:&mut Self::Context<'a,'s>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>where 's:'a{if c.operation==2{return c.close.close_into::<(),u64,_>(c.storage,c.retired,65536,effect,reply)}let turn=if c.operation==0{c.storage.reserve_empty(effect)}else{c.storage.initialize_seed(effect,||Ok(Receiver{closing:false,empty:false}))};Ok(match turn{RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result.map_err(Into::into))})}
 fn recover<'r,'a,'s>(&mut self,c:&mut Self::Context<'a,'s>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>where 's:'a{assert_eq!(c.operation,2);c.close.recover_close_into::<(),u64,_>(c.storage,c.retired,65536,effect,reply)}
}
struct Driver{authority:OriginalEffectAuthority<Producer>,action:Option<OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>,reply:Option<Result<RetainedCloneStep,OriginalRetirementFailure>>,selected:Option<(RetainedCloneGrant,u64)>,heap:(usize,usize)}
impl Driver{
 fn new()->Self{Self{authority:OriginalEffectAuthority::new(Producer),action:None,reply:None,selected:None,heap:(0,0)}}
 fn run(&mut self,bank:&mut EffectBank,c:&mut Context<'_>,demand:RetirementDemand,work:u64)->bool{let(grant,work)=*self.selected.get_or_insert_with(||(bank.loan(1,demand.copy_bytes),work));let((turn,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,work,|effect|{let before=effect.performed_work();let complete=self.authority.drive(c,effect,&mut self.action).unwrap();let work=effect.performed_work()-before;if complete{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));self.heap.0+=heap.0;self.heap.1+=heap.1;if matches!(turn,RetainedWorkPoll::Pending{..}){return false}assert!(self.action.as_mut().unwrap().transfer_reply(&mut self.reply).unwrap());self.authority.finish(&mut self.action).unwrap();let step=self.reply.take().unwrap().unwrap();assert_ne!(bank.take_admitted(),Some(false));if let Some(actual)=actual{assert_eq!(actual,self.heap.0)}bank.post(step.progress(),grant,self.heap);self.heap=(0,0);self.selected=None;true}
}
#[test]fn actual_native_receiver_backing_retires_original_child_and_runs_real_header_drop_once(){
 let law:serde_json::Value=serde_json::from_str(include_str!("backing.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){CLOSE.store(0,Ordering::SeqCst);RECOVER.store(0,Ordering::SeqCst);DROPS.store(0,Ordering::SeqCst);UNWIND.store(usize::from(row["unwind"].as_bool().unwrap()),Ordering::SeqCst);let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let mut storage=NativeOwnedStorage::empty();let mut retired=None;let mut close=NativeReceiverBackingClose::default();let mut driver=Driver::new();
  for(operation,demand)in[(0,NativeOwnedStorage::<Receiver>::reserve_demand()),(1,NativeOwnedStorage::<Receiver>::seed_demand())]{assert!(driver.run(&mut bank,&mut Context{storage:&mut storage,retired:&mut retired,close:&mut close,operation},demand,1))}
  let address=storage.address();let before=bank.wallet().remaining_work();let demand=close.demands::<(),u64,_>(&storage,&retired,65536).unwrap();assert!(driver.run(&mut bank,&mut Context{storage:&mut storage,retired:&mut retired,close:&mut close,operation:2},demand,0));assert_eq!(bank.wallet().remaining_work(),before);assert_eq!(storage.address(),address);assert_eq!(CLOSE.load(Ordering::SeqCst),0);assert_eq!(DROPS.load(Ordering::SeqCst),0);
  let mut interrupted=0;for _ in 0..64{if storage.terminal_is_empty()&&retired.is_none()&&close.terminal_is_empty(){break}let demand=close.demands::<(),u64,_>(&storage,&retired,65536).unwrap();let work=close.work_demand::<(),u64,_>(&storage,&retired,65536).unwrap();let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||driver.run(&mut bank,&mut Context{storage:&mut storage,retired:&mut retired,close:&mut close,operation:2},demand,work)));if outcome.is_err(){interrupted+=1;assert!(bank.has_pending_action()&&driver.action.is_some());assert_eq!(bank.actual(),None)}}
  assert!(storage.terminal_is_empty()&&retired.is_none()&&close.terminal_is_empty());assert_eq!(interrupted,usize::from(row["unwind"].as_bool().unwrap()));assert_eq!(CLOSE.load(Ordering::SeqCst),row["closeCalls"].as_u64().unwrap()as usize);assert_eq!(RECOVER.load(Ordering::SeqCst),row["recoverCalls"].as_u64().unwrap()as usize);assert_eq!(DROPS.load(Ordering::SeqCst),row["drops"].as_u64().unwrap()as usize);assert!(driver.authority.into_producer().is_ok());assert!(bank.into_wallet().is_ok());
 }
}

#[test]
fn slot_context_rejects_a_foreign_actual_source_before_capture(){
 use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};
 use semio_framework_value_native_decoding::native_decoding::{OriginalEffectAuthority,OriginalEffectProducer};
 let mut source=FixtureSource::<()>::from_owner(());
 let mut foreign=EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:16},16));
 let mut authority=Some(OriginalEffectAuthority::new(NativeSlotProducer::<(),u64,Receiver>::new()));
 let mut action=None;
 let (outer,_) = foreign.run_with_source(RetainedCloneGrant::default(),0,|_,foreign_witness|{
  let (inner,_) = source.with_bank(|input,bank|bank.run_with_source(RetainedCloneGrant::default(),0,|effect,witness|{
   let local=NativeSlotOperation::Capture(input).with_witness(witness);
   assert_eq!(authority.as_ref().unwrap().inspect(|producer|producer.source_turn(&local)),Some(witness.identity()));
   let mut other=NativeSlotOperation::Capture(input).with_witness(foreign_witness);
   assert_ne!(witness.identity(),foreign_witness.identity());
   assert!(authority.as_mut().unwrap().drive(&mut other,effect,&mut action).is_err());
   assert!(action.is_none());
   assert!(authority.as_ref().unwrap().inspect(|producer|producer.identity().is_none()));
   RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:0})
  })).unwrap();
  assert!(matches!(inner,RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(()),work:0})));
  RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:0})
 });
 assert!(matches!(outer,RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(()),work:0})));
 assert!(authority.take().unwrap().into_producer().is_ok());
}
