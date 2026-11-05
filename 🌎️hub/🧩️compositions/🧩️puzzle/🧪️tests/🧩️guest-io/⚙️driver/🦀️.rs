//! 🎟️ The real actor and each job crossing retain their original caller and measured receipts.
use semio_framework_actor::{ActorId,OriginalGuestExecutionBudget,OriginalJobBudget,OriginalJobTurnLoan,OriginalJobSource,OriginalJobOutcome,OriginalJobOutputState,RetainedTurnInput};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,USER_VISIBLE_LANE_FUEL,USER_VISIBLE_LANE_WALL_US};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
use semio_plugin_host::{GuestRuntimes,GuestRuntime,GuestInstance,CompiledHandle,PackageRef,PackageHash};
use semio_plugin_interpreter_runtime::{OwnedRuntime,OriginalGuestJobCall};
use semio_plugin::MountedOwnerPolicyV1;
use std::collections::BTreeSet;

pub fn clock()->Option<u64>{semio_framework_job::default_now_us()}
fn ready<T>(future:impl std::future::Future<Output=T>)->T{semio_framework_async::poll::resolve_ready(future)}
pub struct Actor{pub runtime:GuestRuntimes,pub compiled:CompiledHandle,pub instance:GuestInstance,caller:OriginalJobFixtureCaller,epoch:u64,turns:usize,maximum_turns:usize,maximum_vm_bytes:usize,maximum_result_bytes:usize,jobs_closed:bool,pub closed_sources:BTreeSet<u64>,pub observations:Vec<Observation>}
#[derive(Debug)]
pub struct Observation{pub phase:String,pub input:RetainedTurnInput,pub deadline:u64,pub now:u64,pub returned_at:u64,pub spent:RetainedCloneProgress,pub logical_work:u64,pub instructions:u64,pub source_closed:bool}
pub trait Reply{fn receipt(&self)->(RetainedTurnInput,RetainedCloneProgress,u64);fn source_closed(&self)->bool{false}}
macro_rules! reply{($ty:ty)=>{impl Reply for $ty{fn receipt(&self)->(RetainedTurnInput,RetainedCloneProgress,u64){(self.input,self.spent,self.work_units_spent)}}};}
reply!(semio_framework_actor::OriginalJobStartReply);
reply!(semio_framework_actor::OriginalJobCancelReply);
impl Reply for semio_framework_actor::OriginalJobStepReply<OriginalJobOutcome>{fn receipt(&self)->(RetainedTurnInput,RetainedCloneProgress,u64){(self.input,self.spent,self.work_units_spent)}fn source_closed(&self)->bool{self.source_closed}}
impl Reply for semio_framework_actor::OriginalJobOutputReply{fn receipt(&self)->(RetainedTurnInput,RetainedCloneProgress,u64){(self.input,self.spent,self.work_units_spent)}fn source_closed(&self)->bool{self.source_closed}}
impl Reply for semio_framework_actor::OriginalJobOutputAckReply{fn receipt(&self)->(RetainedTurnInput,RetainedCloneProgress,u64){(self.input,self.spent,self.work_units_spent)}fn source_closed(&self)->bool{self.source_closed}}
impl Actor{
 pub fn open(bytes:&[u8],law:&serde_json::Value)->Self{
  let runtime=GuestRuntimes::Owned(OwnedRuntime::new(clock));
  let package=PackageRef{package:semio_framework_actor::PackageId(law["component"]["packageId"].as_str().unwrap().into()),hash:PackageHash(*semio_framework_hash::hash(bytes).as_bytes())};
  let compiled=ready(runtime.compile(&package,bytes)).expect("actual Puzzle actor compiles");
  let source=&law["operationSource"];let stock=RetainedCloneGrant{maximum_items:source["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:source["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:source["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:source["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:source["maximumDepth"].as_u64().unwrap()as usize};
  let preparation=MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation;
  let limits=&law["limits"];let request=limits["requestBytes"].as_u64().unwrap()as usize;let result=limits["resultBytes"].as_u64().unwrap()as usize;let native=limits["nativeValueAllowanceBytes"].as_u64().unwrap()as usize;let vm=limits["vmBytes"].as_u64().unwrap()as usize;let scaffold=limits["scaffoldingBytes"].as_u64().unwrap()as usize;assert_eq!(limits["nativeValueAllowanceBytes"].as_u64().unwrap()as usize,law["material"]["nodes"].as_u64().unwrap()as usize*4096);assert_eq!(stock.maximum_items,preparation.maximum_items*law["execution"]["userVisibleTurns"].as_u64().unwrap()as usize);assert_eq!(stock.maximum_copy_bytes,vm+2*(2*request+2*result+2*native));assert_eq!(stock.maximum_capacity_bytes,2*vm+2*(2*request+3*result+2*native)+scaffold);assert_eq!(stock.maximum_release_bytes,stock.maximum_capacity_bytes);assert_eq!(stock.maximum_depth,preparation.maximum_depth);
  let maximum_turns=law["execution"]["maximumTurns"].as_u64().unwrap()as usize;
  let deadline=clock().expect("real monotonic caller clock").checked_add(law["execution"]["maximumMilliseconds"].as_u64().unwrap()*1000).unwrap();
  let fuel=USER_VISIBLE_LANE_FUEL.checked_mul(law["execution"]["userVisibleTurns"].as_u64().unwrap()).unwrap();
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(771),Generation(1),stock,clock,deadline,fuel);
  let id=ready(ActorId::new(71,1,3,0));let mut epoch=1;
  let mut opening=caller.turn(preparation,|cx|{cx.bind_original_source_epoch(epoch)?;runtime.begin_original_actor_step(&compiled,id,cx)}).expect("original actor birth");
  for _ in 0..maximum_turns{
   if opening.as_ref().unwrap().ready(){break}
   epoch+=1;
   caller.turn(preparation,|cx|{cx.bind_original_source_epoch(epoch)?;let end=clock().unwrap().saturating_add(USER_VISIBLE_LANE_WALL_US);cx.with_original_work_loan(USER_VISIBLE_LANE_FUEL,end,|cx|{let owner=opening.as_mut().unwrap();let execution=owner.requires_execution().then(||OriginalGuestExecutionBudget{input:RetainedTurnInput::borrow_original(cx).unwrap(),fuel:cx.fuel_remaining(),deadline_us:cx.deadline_us()});let call=runtime.advance_original_actor_step(owner,&compiled,execution,cx)?;assert!(call.result.is_ok(),"actual actor startup {:?}",call.result);Ok(())})}).expect("original bounded actor startup");
  }
  assert!(opening.as_ref().unwrap().ready(),"actual component startup did not reach ready within authored turns");
  let mut instance=None;
  for _ in 0..maximum_turns{if instance.is_some(){break}epoch+=1;instance=caller.turn(preparation,|cx|{cx.bind_original_source_epoch(epoch)?;let input=RetainedTurnInput::borrow_original(cx)?;runtime.take_original_actor_step(&mut opening,&compiled,input,cx)}).expect("paid original actor handoff");}
  Self{runtime,compiled,instance:instance.expect("actual actor handoff"),caller,epoch,turns:epoch as usize,maximum_turns,maximum_vm_bytes:law["limits"]["vmBytes"].as_u64().unwrap()as usize,maximum_result_bytes:law["limits"]["resultBytes"].as_u64().unwrap()as usize,jobs_closed:false,closed_sources:BTreeSet::new(),observations:Vec::new()}
 }
 pub fn turn<T>(&mut self,run:impl FnOnce(&GuestRuntimes,&mut GuestInstance,&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
  self.turns+=1;assert!(self.turns<=self.maximum_turns,"authored actor turn bound");self.epoch+=1;
  let mut output=None;let runtime=&self.runtime;let instance=&mut self.instance;let epoch=self.epoch;
  self.caller.turn(MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance,|cx|{cx.bind_original_source_epoch(epoch)?;let end=clock().unwrap().saturating_add(USER_VISIBLE_LANE_WALL_US);cx.with_original_work_loan(USER_VISIBLE_LANE_FUEL,end,|cx|{output=Some(run(runtime,instance,cx)?);Ok(())})}).expect("actual original actor turn");
  if !self.instance.is_closed(){assert!(self.instance.original_memory(0).expect("actual actor memory").len()<=self.maximum_vm_bytes,"actual VM remains within its explicit operation envelope");}
  output.expect("original turn returned its actual result")
 }
 fn call<T:Reply>(&mut self,phase:&str,job:u64,run:impl FnOnce(&GuestRuntimes,&mut GuestInstance,OriginalJobBudget,OriginalGuestExecutionBudget)->OriginalGuestJobCall<T>)->T{
  let mut observation=None;
  let reply=self.turn(|runtime,instance,cx|{
   let input=RetainedTurnInput::borrow_original(cx)?;let mut loan=OriginalJobTurnLoan::new();let budget=loan.checkout(input,cx)?.expect("funded original job crossing");let execution=OriginalGuestExecutionBudget{input,fuel:cx.fuel_remaining(),deadline_us:cx.deadline_us()};
   let now=clock().expect("real monotonic job clock");let call=run(runtime,instance,budget,execution);let returned_at=clock().expect("real monotonic return clock");call.execution.validate_for(execution)?;let instructions=call.execution.spent_fuel.expect("actual backend instruction measurement");assert!(call.returned_unsubmitted_source.is_none(),"actual guest left source unsubmitted");let reply=call.original.unwrap_or_else(|fault|panic!("actual {phase} crossing failed at {now}..{returned_at}, budget={budget:?}, execution={:?}: {fault:?}",call.execution));
   let(returned,spent,work)=reply.receipt();loan.receive(returned,spent,work,cx)?;assert!(loan.terminal_is_empty());
   observation=Some(Observation{phase:phase.into(),input:returned,deadline:budget.deadline_us,now,returned_at,spent,logical_work:work,instructions,source_closed:reply.source_closed()});Ok(reply)
  });
  if reply.source_closed(){self.closed_sources.insert(job);}self.observations.push(observation.unwrap());reply
 }
 pub fn start(&mut self,source:OriginalJobSource){let job=source.job;let reply=self.call("start",job,|r,i,b,e|ready(r.start_job(i,Some(source),b,e)));assert!(reply.refusal.is_none());assert!(reply.admitted,"real job source must be admitted");assert!(reply.returned_source.is_none());}
 pub fn step(&mut self,job:u64)->Option<OriginalJobOutcome>{let reply=self.call("step",job,|r,i,b,e|ready(r.step_job(i,job,b,e)));assert!(reply.refusal.is_none());reply.outcome}
 pub fn output(&mut self,outcome:OriginalJobOutcome)->Vec<u8>{
  assert!(outcome.byte_length<=self.maximum_result_bytes as u64,"actual serialized result exceeds the closed case envelope");
  let mut bytes=Vec::new();let mut output_closed=false;
  for _ in 0..self.maximum_turns{let reply=self.call("page",outcome.job,|r,i,b,e|ready(r.read_job_output(i,outcome.job,b.for_output_page(),e)));assert!(reply.refusal.is_none());if let Some(chunk)=reply.chunk{assert_eq!(chunk.offset,bytes.len()as u64);if let Some(page)=chunk.bytes{assert!(page.len()<=OriginalJobBudget::OUTPUT_PAGE_BYTES);bytes.extend_from_slice(&page);}if chunk.output_closed{output_closed=true;break}}}
  assert!(output_closed,"actual output source reaches closure");assert_eq!(bytes.len()as u64,outcome.byte_length);
  let mut acknowledged=false;for _ in 0..self.maximum_turns{let reply=self.call("ack",outcome.job,|r,i,b,e|ready(r.acknowledge_job_output(i,outcome.job,b,e)));assert!(reply.refusal.is_none());if reply.acknowledged{acknowledged=true;break}}
  assert!(acknowledged,"exact actual output acknowledgement");bytes
 }
 pub fn cancel(&mut self,job:u64){let reply=self.call("cancel",job,|r,i,b,e|ready(r.cancel_job(i,job,b,e)));assert!(reply.refusal.is_none());assert!(reply.admitted,"funded cancellation acknowledgement");}
 pub fn drain(&mut self,job:u64,allow_done:bool,law:&serde_json::Value){
  let mut cancelled=false;
  for _ in 0..self.maximum_turns{
   if self.closed_sources.contains(&job){assert!(allow_done||cancelled,"source closed without the actual canceled Failed terminal");return}
   if let Some(outcome)=self.step(job){
    assert!(allow_done||outcome.state!=OriginalJobOutputState::Done,"cancelled conversion must not publish completion");let state=outcome.state;let bytes=self.output(outcome);
    if !allow_done&&state==OriginalJobOutputState::Failed{
     let value=semio_os_kernel::pack_rt::decode_wire_value(&bytes).expect("actual cancellation is a strict PACK fault");
     let fault=<semio_framework::kernel::Fault as semio_framework_value::FromValue>::from_value(value).expect("actual typed cancellation fault");
     let terminal=&law["cancellationTerminal"];let matched=fault.code.0==semio_framework_value::ValueRefusalKind::Canceled.as_str()&&fault.code.0==terminal["code"].as_str().unwrap()&&fault.origin==semio_framework::kernel::FaultOrigin::Framework&&fault.severity==semio_framework::kernel::Severity::Error&&!fault.retryable;
     self.retire(law,fault);assert!(matched,"ordinary semantic failure cannot satisfy actual cancellation");cancelled=true;
    }
   }
  }
  panic!("actual guest source did not close within authored turns")
 }
 pub fn retire<T:semio_framework_value::retirement::RetireOwned>(&mut self,law:&serde_json::Value,value:T){
  let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|panic!("actual test source supports paid retirement"));
  for _ in 0..law["execution"]["maximumTurns"].as_u64().unwrap(){
   if owner.terminal_is_empty(){return}
   self.turn(|_,_,cx|cx.with_original_retirement(|cx|{let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=owner.step(grant);semio_framework_value::RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;Ok(())}));
  }
  panic!("actual test source did not reach paid terminal retirement")
 }
 pub fn close_jobs(&mut self){if self.jobs_closed{return}for _ in 0..self.maximum_turns{let reply=self.call("close-jobs",0,|r,i,b,e|ready(r.close_jobs(i,b,e)));assert!(reply.refusal.is_none());if reply.admitted{self.jobs_closed=true;break}}assert!(self.jobs_closed,"actual guest job registry closes");}
 pub fn close(mut self){
  self.close_jobs();
  for _ in 0..self.maximum_turns{if self.instance.is_closed(){break}self.epoch+=1;let epoch=self.epoch;let runtime=&self.runtime;let compiled=&self.compiled;let instance=&mut self.instance;self.caller.turn(MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close,|cx|{cx.bind_original_source_epoch(epoch)?;let end=clock().unwrap().saturating_add(USER_VISIBLE_LANE_WALL_US);cx.with_original_work_loan(USER_VISIBLE_LANE_FUEL,end,|cx|runtime.close_original_instance_step(instance,compiled,cx))}).expect("paid actual actor instance close");}
  assert!(self.instance.is_closed());self.caller.retire(MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close);assert!(self.caller.terminal_is_empty());
 }
}
