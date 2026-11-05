use semio_framework_job_context::{Generation,OperationId,OriginalNativeLease,OriginalNativeRetirement,OriginalTurnFailure,RetainedCloneGrant,StepContext};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use std::sync::atomic::{AtomicUsize,Ordering};
static DROPS:AtomicUsize=AtomicUsize::new(0);
static GRAPH_STEPS:AtomicUsize=AtomicUsize::new(0);
struct GraphChild{empty:bool}
impl OriginalNativeRetirement for GraphChild{fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{assert_eq!(GRAPH_STEPS.swap(2,Ordering::Relaxed),1);cx.consume_fuel(1);self.empty=true;Ok(true)}fn terminal_is_empty(&self)->bool{self.empty}}
impl Drop for GraphChild{fn drop(&mut self){assert!(self.empty)}}
struct GraphParent{child:Option<OriginalNativeLease<GraphChild>>,empty:bool}
impl OriginalNativeRetirement for GraphParent{fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{cx.consume_retained(semio_framework_job_context::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);assert_eq!(GRAPH_STEPS.swap(1,Ordering::Relaxed),0);drop(self.child.take());self.empty=true;Ok(true)}fn terminal_is_empty(&self)->bool{self.empty&&self.child.is_none()}}
impl Drop for GraphParent{fn drop(&mut self){assert!(self.terminal_is_empty())}}
struct Native{empty:bool}
impl OriginalNativeRetirement for Native{
 fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if cx.retirement_should_yield(){return Ok(false)}cx.consume_fuel(1);self.empty=true;Ok(true)}
 fn terminal_is_empty(&self)->bool{self.empty}
}
impl Drop for Native{fn drop(&mut self){assert!(self.empty);DROPS.fetch_add(1,Ordering::Relaxed);}}
fn clock()->Option<u64>{Some(1)}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128}}
#[test]
fn native_parent_survives_foreign_context_cancellation_and_return(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();assert_eq!(cases["slots"],8);assert_eq!(cases["graph"]["closeCallbacks"],serde_json::json!([1,0]));
 let grant=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(610),Generation(1),grant,clock,100000,100000);
 let mut foreign=OriginalJobFixtureCaller::admit(OperationId(611),Generation(1),grant,clock,100000,100000);
 let mut native=Some(Native{empty:false});let mut owner:Option<OriginalNativeLease<Native>>=None;let mut child=None;
 let(result,denied)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.narrow_original_work_budget(semio_framework_job_context::StepBudget::new(5,cx.deadline_us(),cx.retained_grant()))?;cx.admit_original_native(&mut native,&mut owner)}));assert!(!result.unwrap());assert!(native.is_some()&&owner.is_none());assert_eq!((denied.requested_bytes,denied.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());
 let (result,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|cx.admit_original_native(&mut native,&mut owner)));
 assert!(result.unwrap());assert!(native.is_none());assert_eq!(birth.requested_bytes,caller.receipt().retained_capacity_bytes);assert_eq!(birth.released_bytes,0);
 let recipient=owner.as_ref().unwrap().recipient_identity();assert!(caller.turn(grant,|cx|cx.original_native_retired_witness(recipient)).unwrap().is_none());assert!(foreign.turn(grant,|cx|cx.original_native_retired_witness(recipient)).is_err());
 assert!(caller.turn(grant,|cx|owner.as_ref().unwrap().lend(cx,&mut child)).unwrap());
 assert!(foreign.turn(grant,|cx|child.as_mut().unwrap().with_native(cx,|_,_|Ok(()))).is_err());
 caller.cancel().cancel_now();
 while !owner.as_ref().unwrap().terminal_is_empty(){caller.turn(grant,|cx|owner.as_mut().unwrap().return_step(cx)).unwrap();}
 assert!(!caller.turn(grant,|cx|cx.close_original_native_step()).unwrap());assert_eq!(DROPS.load(Ordering::Relaxed),0);
 assert!(caller.turn(grant,|cx|child.as_mut().unwrap().with_native(cx,|native,_|{assert!(!native.empty);Ok(())})).unwrap());
 while !child.as_ref().unwrap().terminal_is_empty(){caller.turn(grant,|cx|child.as_mut().unwrap().return_step(cx)).unwrap();}
 let mut released=0;while !caller.turn(grant,|cx|cx.close_original_native_step()).unwrap(){released+=caller.receipt().released_bytes;}
 released+=caller.receipt().released_bytes;assert_eq!(released,birth.requested_bytes);assert_eq!(DROPS.load(Ordering::Relaxed),1);
 assert!(caller.turn(grant,|cx|{let witness=cx.original_native_retired_witness(recipient)?.unwrap();assert!(witness.proves(recipient,cx));Ok(())}).is_ok());
 drop(owner);drop(child);caller.retire(grant);foreign.retire(grant);
 let mut detached=OriginalJobFixtureCaller::admit(OperationId(612),Generation(1),grant,clock,100000,100000);let mut input=Some(Native{empty:false});let mut owner:Option<OriginalNativeLease<Native>>=None;let mut child=None;
 let (result,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||detached.turn(grant,|cx|cx.admit_original_native(&mut input,&mut owner)));assert!(result.unwrap());assert!(detached.turn(grant,|cx|owner.as_ref().unwrap().lend(cx,&mut child)).unwrap());detached.cancel().cancel_now();
 let (_,capture)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(child);drop(owner);});assert_eq!((capture.requested_bytes,capture.released_bytes),(0,0));
 let mut released=0;while !detached.turn(grant,|cx|cx.close_original_native_step()).unwrap(){released+=detached.receipt().released_bytes;}released+=detached.receipt().released_bytes;assert_eq!(released,birth.requested_bytes);detached.retire(grant);
 let mut graph=OriginalJobFixtureCaller::admit(OperationId(613),Generation(1),grant,clock,100000,100000);let mut input=Some(GraphChild{empty:false});let mut owner:Option<OriginalNativeLease<GraphChild>>=None;let mut child=None;assert!(graph.turn(grant,|cx|cx.admit_original_native(&mut input,&mut owner)).unwrap());assert!(graph.turn(grant,|cx|owner.as_ref().unwrap().lend(cx,&mut child)).unwrap());let mut parent=Some(GraphParent{child,empty:false});let mut parent_loan=None;assert!(graph.turn(grant,|cx|cx.admit_original_native(&mut parent,&mut parent_loan)).unwrap());drop(owner);drop(parent_loan);while !graph.turn(grant,|cx|cx.close_original_native_step()).unwrap(){}assert_eq!(GRAPH_STEPS.load(Ordering::Relaxed),2);graph.retire(grant);
 let mut pair=OriginalJobFixtureCaller::admit(OperationId(614),Generation(1),grant,clock,100000,100000);let mut foreign=OriginalJobFixtureCaller::admit(OperationId(615),Generation(1),grant,clock,100000,100000);let mut left=None;let mut right=None;let mut alias=None;let mut input=Some(Native{empty:false});assert!(pair.turn(grant,|cx|cx.admit_original_native(&mut input,&mut left)).unwrap());let mut input=Some(Native{empty:false});assert!(pair.turn(grant,|cx|cx.admit_original_native(&mut input,&mut right)).unwrap());assert!(pair.turn(grant,|cx|left.as_ref().unwrap().lend(cx,&mut alias)).unwrap());
 assert!(pair.turn(grant,|cx|left.as_mut().unwrap().with_native_pair(right.as_mut().unwrap(),cx,|left,right,_|{assert!(!std::ptr::eq(left,right));Ok(())})).unwrap());assert!(pair.turn(grant,|cx|right.as_mut().unwrap().with_native_pair(left.as_mut().unwrap(),cx,|left,right,_|{assert!(!std::ptr::eq(left,right));Ok(())})).unwrap());assert!(pair.turn(grant,|cx|left.as_mut().unwrap().with_native_pair(alias.as_mut().unwrap(),cx,|_,_,_|Ok(()))).is_err());assert!(foreign.turn(grant,|cx|left.as_mut().unwrap().with_native_pair(right.as_mut().unwrap(),cx,|_,_,_|Ok(()))).is_err());drop(alias);drop(left);drop(right);while !pair.turn(grant,|cx|cx.close_original_native_step()).unwrap(){}pair.retire(grant);foreign.retire(grant);
 for (index,row) in cases["cases"].as_array().unwrap().iter().enumerate(){
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(620+index as u64),Generation(1),grant,clock,100000,100000);let mut foreign=OriginalJobFixtureCaller::admit(OperationId(630+index as u64),Generation(1),grant,clock,100000,100000);let mut owner:Option<OriginalNativeLease<Native>>=None;let mut child=None;let mut outputs=Vec::new();
  for event in row["events"].as_array().unwrap(){let output=match event.as_str().unwrap(){
   "birth"=>{let mut input=Some(Native{empty:false});assert!(caller.turn(grant,|cx|cx.admit_original_native(&mut input,&mut owner)).unwrap());"admitted"},
   "lend"=>{assert!(caller.turn(grant,|cx|owner.as_ref().unwrap().lend(cx,&mut child)).unwrap());"borrowed"},
   "foreignBorrow"=>{assert!(foreign.turn(grant,|cx|owner.as_mut().unwrap().with_native(cx,|_,_|Ok(()))).is_err());"refused"},
   "cancel"=>{caller.cancel().cancel_now();"retained"},
   "dropCaller"=>{drop(owner.take());drop(child.take());"captured"},
   "return"=>{let loan=child.as_mut().filter(|loan|!loan.terminal_is_empty()).unwrap_or_else(||owner.as_mut().unwrap());while !loan.terminal_is_empty(){caller.turn(grant,|cx|loan.return_step(cx)).unwrap();}"acknowledged"},
   "close"=>{if let Some(loan)=owner.as_mut(){while !loan.terminal_is_empty(){caller.turn(grant,|cx|loan.return_step(cx)).unwrap();}}loop{let before=caller.fuel_remaining();let complete=caller.turn(grant,|cx|cx.close_original_native_step()).unwrap();if complete{break "closed"}if before==caller.fuel_remaining(){break "blocked"}}},
   _=>unreachable!()};outputs.push(output);
  }
  assert_eq!(serde_json::json!(outputs),row["outputs"]);drop(owner);drop(child);caller.retire(grant);foreign.retire(grant);
 }
}
