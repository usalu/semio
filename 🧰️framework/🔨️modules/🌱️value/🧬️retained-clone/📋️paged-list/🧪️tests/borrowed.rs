//! 📋️ The real paged clone engine retains its child replies and native headers on one original Source bank.
use semio_framework_value_paged_clone::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_native_clone_engine::RetainedCursorRelease;
use semio_framework_value_clone_source::{RetainedCloneSource,RetainedCloneRef,RetainedCloneClose};
use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};
use semio_framework_value_testing_effect_bank::EffectBank;
use std::{mem::size_of,sync::atomic::{AtomicUsize,Ordering}};
type List=PagedList<u64,16>;
type Engine=PagedListCloneEngine<u64,Element,16>;
type Reply<'a>=OriginalEffectReply<'a,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,RetainedCloneStep,OriginalRetirementFailure>;
static DROPS:AtomicUsize=AtomicUsize::new(0);
static MODE:AtomicUsize=AtomicUsize::new(0);
static RECEIVES:AtomicUsize=AtomicUsize::new(0);
static RECOVERS:AtomicUsize=AtomicUsize::new(0);
static CHILD_WORK:AtomicUsize=AtomicUsize::new(0);
struct Element{output:Option<u64>,closing:bool,started:bool}
impl PagedCloneElement<u64> for Element{
 fn new()->Self{Self{output:None,closing:false,started:false}}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(Self::terminal_is_empty)})}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 fn advance_demands(&self,_:RetainedCloneRef<'_,u64>,_:usize,_:u64)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()})}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,u64>,_:usize,_:u64)->Result<u64,ValueError>{Ok(if *source.get()==13&&matches!(MODE.load(Ordering::SeqCst),1|2){2}else{1})}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,u64>,_:usize,_:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{let target=*source.get()==13;let mode=MODE.load(Ordering::SeqCst);assert!(!self.started);assert!(effect.performed(1)?);self.started=true;if target{RECEIVES.fetch_add(1,Ordering::SeqCst);CHILD_WORK.fetch_add(1,Ordering::SeqCst);if mode==1{return Ok(reply.pending())}if mode==2{panic!("native paged child interrupted before its original reply")}}self.output=Some(*source.get());let complete=reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),..Default::default()})));if target&&mode==3{panic!("native paged child interrupted after its original reply")}Ok(complete)}
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,u64>,_:usize,_:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{assert!(self.started&&self.output.is_none());assert_eq!(*source.get(),13);assert!(matches!(MODE.load(Ordering::SeqCst),1|2));RECOVERS.fetch_add(1,Ordering::SeqCst);assert!(effect.performed(1)?);CHILD_WORK.fetch_add(1,Ordering::SeqCst);self.output=Some(*source.get());Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),..Default::default()}))))}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{retained_clone_take_demand::<u64>()}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn take<'r>(&mut self,output:&mut Option<u64>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{let turn=take_retained_clone_output(&mut self.output,output,effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Complete)))}
 fn recover_take<'r>(&mut self,_:&mut Option<u64>,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{panic!("recorded native take must not replay")}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{assert!(effect.performed(1)?);self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}))))}
 fn recover_begin_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("recorded native close-start must not replay")}
 fn close_started(&self)->bool{self.closing}
 fn close_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn close_into<'r>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{assert!(self.output.is_none());Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
 fn recover_close_into<'r>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{panic!("recorded native close must not replay")}
 fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()}
}
impl Drop for Element{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking());DROPS.fetch_add(1,Ordering::SeqCst);}}
enum Command<'a>{Advance,Begin,Close,Retire(&'a mut RetainedCloneClose,&'a mut Option<List>)}
struct Context<'a,'s>{source:RetainedCloneRef<'s,List>,engine:&'a mut Engine,command:Command<'a>}
struct Producer;
impl OriginalEffectProducer for Producer{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'s>=Context<'a,'s>where 's:'a;
 fn validate_context<'a,'s>(&self,_:&Self::Context<'a,'s>)->Result<(),ValueError>where 's:'a{Ok(())}
 fn source_turn<'a,'s>(&self,_:&Self::Context<'a,'s>)->Option<OriginalSourceTurnIdentity>where 's:'a{None}
 fn receive<'r,'a,'s>(&mut self,c:&mut Self::Context<'a,'s>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>where 's:'a{match &mut c.command{Command::Advance=>c.engine.advance_into(c.source,65536,1024,effect,reply),Command::Begin=>c.engine.begin_close_into(effect,reply),Command::Close=>c.engine.close_into(65536,effect,reply),Command::Retire(close,output)=>{if close.is_empty(){let turn=close.begin_granted(output,effect);Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}else{close.step_into(65536,effect,reply)}}}}
 fn recover<'r,'a,'s>(&mut self,c:&mut Self::Context<'a,'s>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>where 's:'a{match &mut c.command{Command::Advance=>c.engine.recover_advance_into(c.source,65536,1024,effect,reply),Command::Begin=>c.engine.recover_begin_close_into(effect,reply),Command::Close=>c.engine.recover_close_into(65536,effect,reply),Command::Retire(close,_)=>close.recover_step_into(65536,effect,reply)}}
}
struct TakeContext<'a>{engine:&'a mut Engine,output:&'a mut Option<List>}
struct Take;
impl OriginalEffectProducer for Take{
 type Output=RetainedCloneStep;type Failure=ValueError;type Context<'a,'s>=TakeContext<'a>where 's:'a;
 fn validate_context<'a,'s>(&self,_:&Self::Context<'a,'s>)->Result<(),ValueError>where 's:'a{Ok(())}
 fn source_turn<'a,'s>(&self,_:&Self::Context<'a,'s>)->Option<OriginalSourceTurnIdentity>where 's:'a{None}
 fn receive<'r,'a,'s>(&mut self,c:&mut Self::Context<'a,'s>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>where 's:'a{c.engine.take(c.output,effect,reply)}
 fn recover<'r,'a,'s>(&mut self,c:&mut Self::Context<'a,'s>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>where 's:'a{c.engine.recover_take(c.output,effect,reply)}
}
struct Call<P:OriginalEffectProducer<Output=RetainedCloneStep>>{authority:OriginalEffectAuthority<P>,action:Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>,answer:Option<Result<RetainedCloneStep,P::Failure>>,selected:Option<(RetainedCloneGrant,u64)>,observed:(usize,usize)}
impl<P:OriginalEffectProducer<Output=RetainedCloneStep>> Call<P>{
 fn new(producer:P)->Self{Self{authority:OriginalEffectAuthority::new(producer),action:None,answer:None,selected:None,observed:(0,0)}}
 fn into_producer(self)->Result<P,OriginalEffectAuthority<P>>{assert!(self.action.is_none()&&self.answer.is_none()&&self.selected.is_none());self.authority.into_producer()}
}
fn run<'a,'s,P:OriginalEffectProducer<Output=RetainedCloneStep>>(bank:&mut EffectBank,call:&mut Call<P>,context:&mut P::Context<'a,'s>,demand:RetirementDemand,work:u64)->Option<RetainedCloneStep>where 's:'a,P::Failure:std::fmt::Debug{
 let (grant,work)=*call.selected.get_or_insert_with(||(bank.loan(1,demand.copy_bytes),work));let((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,work,|effect|{let before=effect.performed_work();let ready=call.authority.drive(context,effect,&mut call.action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));call.observed.0+=heap.0;call.observed.1+=heap.1;let RetainedWorkPoll::Complete(_)=poll else{return None};assert!(call.action.as_mut().unwrap().transfer_reply(&mut call.answer).unwrap());call.authority.finish(&mut call.action).unwrap();let step=call.answer.take().unwrap().unwrap();if let Some(actual)=actual{assert_eq!(actual,call.observed.0)}assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),grant,call.observed);call.selected=None;call.observed=(0,0);Some(step)
}
#[test]fn actual_paged_engine_copies_original_values_and_pays_real_child_and_page_retirement(){
 let law:serde_json::Value=serde_json::from_str(include_str!("borrowed.json")).unwrap();let refused=unsafe{RetainedCursorRelease::<u64>::native(|_|false)};assert_eq!(!refused.validates(&0),law["nativeLifecycle"]["rejectsNonMatchingWitness"].as_bool().unwrap());for case in law["interruptions"].as_array().unwrap(){MODE.store(case["mode"].as_u64().unwrap()as usize,Ordering::SeqCst);RECEIVES.store(0,Ordering::SeqCst);RECOVERS.store(0,Ordering::SeqCst);CHILD_WORK.store(0,Ordering::SeqCst);let mut interrupted=0;let values=law["values"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap());let original=List::try_from_iter(values).unwrap();let mut source:FixtureSource<List>=RetainedCloneSource::from_owner(original);let mut engine=Engine::default();let mut output=None;let mut producer=Call::new(Producer);let mut take=Call::new(Take);DROPS.store(0,Ordering::SeqCst);
 source.with_bank(|source,bank|{
  let before=bank.wallet().remaining_work();let d=engine.advance_demands(source,65536,1024).unwrap();run(bank,&mut producer,&mut Context{source,engine:&mut engine,command:Command::Advance},d,0);assert_eq!(bank.wallet().remaining_work(),before);assert!(!engine.output_ready());
  for _ in 0..512{if engine.output_ready(){break}let d=engine.advance_demands(source,65536,1024).unwrap();let work=engine.advance_work_demand(source,65536,1024).unwrap();let before=bank.wallet().remaining_work();let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||run(bank,&mut producer,&mut Context{source,engine:&mut engine,command:Command::Advance},d,work)));match result{Ok(None)=>{assert!(bank.has_pending_action()&&producer.action.is_some());assert!(!engine.output_ready());assert!(bank.wallet().remaining_work()<before)},Err(_)=>{interrupted+=1;assert_eq!(bank.actual(),None);assert!(bank.has_pending_action()&&producer.action.is_some()&&producer.selected.is_some());assert!(bank.wallet().remaining_work()<before);assert!(!engine.output_ready());},Ok(Some(_))=>{}}}
  assert!(engine.output_ready());assert_eq!(DROPS.load(Ordering::SeqCst),law["values"].as_array().unwrap().len());let d=engine.take_demand().unwrap();let work=engine.take_work_demand().unwrap();run(bank,&mut take,&mut TakeContext{engine:&mut engine,output:&mut output},d,work);assert_eq!(serde_json::to_value(output.as_ref().unwrap()).unwrap(),law["values"]);
  let d=engine.begin_close_demand().unwrap();let work=engine.begin_close_work_demand().unwrap();run(bank,&mut producer,&mut Context{source,engine:&mut engine,command:Command::Begin},d,work);for _ in 0..512{if engine.terminal_is_empty(){break}let d=engine.close_demands(65536).unwrap();let work=engine.close_work_demand(65536).unwrap();run(bank,&mut producer,&mut Context{source,engine:&mut engine,command:Command::Close},d,work);}assert!(engine.terminal_is_empty());let mut close=RetainedCloneClose::default();for _ in 0..512{if output.is_none()&&close.is_empty(){break}let d=close.next_owner_demands::<List>(output.is_some(),&None,65536).unwrap();let work=close.next_owner_work::<List>(output.is_some(),&None,65536).unwrap();run(bank,&mut producer,&mut Context{source,engine:&mut engine,command:Command::Retire(&mut close,&mut output)},d,work);}assert!(output.is_none()&&close.is_empty());assert!(bank.wallet().remaining_work()<before);
 }).unwrap();for _ in 0..512{if source.terminal_is_empty(){break}source.close_once().unwrap();}assert!(source.terminal_is_empty());assert!(producer.into_producer().is_ok());assert!(take.into_producer().is_ok());assert_eq!(RECEIVES.load(Ordering::SeqCst),case["receiveCalls"].as_u64().unwrap()as usize);assert_eq!(RECOVERS.load(Ordering::SeqCst),case["recoverCalls"].as_u64().unwrap()as usize);assert_eq!(CHILD_WORK.load(Ordering::SeqCst),case["childWork"].as_u64().unwrap()as usize);assert_eq!(interrupted,case["interruptions"].as_u64().unwrap()as usize);}
}
