//! 🏃️ A command the runtime itself originates enters the original typed source through the generic OpBinary producer the declaration crate owns.
use super::*;
use semio_plugin_decl_app::tool_contract::admit_op_binary_producer;
pub use semio_plugin_decl_app::tool_contract::RUNTIME_COMMAND_WIRE_MAXIMUM;
impl<A:ArtifactApp> OriginalTypedCommandSource<A>{
 /// 📣️ Domain command storage is born under its declared original quote before the wire producer receives it.
 pub fn produce_host_event(&mut self,event:&semio_plugin_decl_emission::HostEvent,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
  self.validate(cx)?;if self.runtime_ignored{return Ok(Some(false))}if self.caller.is_none(){if !cx.original_source_parent()?.admit_loan(&mut *self.caller,cx)?{return Ok(None)}}if self.command.is_some()||self.wire.is_some(){if self.command.is_none()||self.wire.is_none(){return Err(refused().into())}if self.producer.is_some(){self.close_producer(cx)?;return Ok(None)}return Ok(Some(true))}
  if self.producer.is_none(){if self.runtime_input.is_none(){let Some(mut d)=A::host_event_demands(event)else{return Ok(Some(false))};let header=2*size_of::<Option<Box<A::Command>>>()+2*size_of::<bool>();d.copy_bytes=d.copy_bytes.checked_add(header).ok_or_else(refused)?;let mut done=None;semio_framework_job::admit_original_construction(cx,&mut done,original(d),|_|{let command=A::host_event(event).map(Box::new);let p=if command.is_some(){progress(d)}else{RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<Option<Box<A::Command>>>()+2*size_of::<bool>(),..Default::default()}};self.runtime_ignored=command.is_none();self.host_event=command.is_some();*self.runtime_input=command;semio_framework_value::RetainedWorkOutcome{result:Ok(((),p)),work:1}})?;return Ok(None)}let original=&mut self.runtime_input;admit_op_binary_producer(||original.take(),None,|_|RUNTIME_COMMAND_WIRE_MAXIMUM,cx,&mut self.producer)?;return Ok(None)}let producer=self.producer.as_mut().unwrap();if !producer.ready(){producer.advance("",&mut None,&mut None,cx)?;return Ok(None)}producer.take_into(&mut self.command,&mut self.wire,cx)?;Ok(None)
 }
 /// 🏃️ A command the runtime originates is born into the producer slot once, then encoded and handed to this source together with its sealed wire.
 pub fn produce_runtime(&mut self,make:impl FnOnce()->Option<Box<A::Command>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;
  if self.caller.is_none(){if !cx.original_source_parent()?.admit_loan(&mut *self.caller,cx)?{return Ok(false)}}
  if self.command.is_some()||self.wire.is_some(){if self.command.is_none()||self.wire.is_none(){return Err(refused().into())}if self.producer.is_some(){self.close_producer(cx)?;return Ok(false)}return Ok(true)}
  if self.producer.is_none(){admit_op_binary_producer(make,None,|_|RUNTIME_COMMAND_WIRE_MAXIMUM,cx,&mut self.producer)?;return Ok(false)}
  let producer=self.producer.as_mut().unwrap();
  if !producer.ready(){producer.advance("",&mut None,&mut None,cx)?;return Ok(false)}
  producer.take_into(&mut self.command,&mut self.wire,cx)?;Ok(false)
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework::{Fault,FaultCode,FaultOrigin,action_bus::{RetainedToolWireInput,TOOL_WIRE_PAGE_BYTES}};
 use semio_plugin_decl_app::tool_contract::{ArtifactOriginalCommandProducer,ArtifactOriginalCommandProducerFactory,OriginalActionCommandBridge,OriginalCommandBirth,admit_op_binary_producer};
 use semio_framework_value::{DslValue,RetainedCloneGrant,retirement::RetireOwned};
 use protocol::OpBinary;
 use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,default_now_us};
 #[global_allocator]
 static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
 #[derive(semio_framework_value::RetireOwned)]struct Command{kind:String,count:u32}
 impl OpBinary for Command{
  fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{Ok(serde_json::to_vec(&serde_json::json!({"kind":self.kind,"count":self.count})).expect("json encodes"))}
  fn decode_op(_:&[u8])->Result<Self,protocol::ProtocolError>{Err(protocol::ProtocolError::TornTail(0))}
 }
 fn bridge(action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>)->Result<Command,Fault>{if let Some(fault)=fault.take(){return Err(fault)}Ok(Command{kind:action.to_owned(),count:args.take().and_then(|value|value.as_str().and_then(|text|text.parse().ok())).unwrap_or(0)})}
 const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:4096};
 const G:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096};
 fn turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{caller.turn_with_decoder(G,decoder,run).unwrap()}
 fn born(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,make:Option<Command>,birth:Option<OriginalCommandBirth<Command>>)->Box<dyn ArtifactOriginalCommandProducer<Command>>{let mut slot=None;let mut make=make.map(Box::new);turn(caller,decoder,|cx|{admit_op_binary_producer(||make.take(),birth,|_|RUNTIME_COMMAND_WIRE_MAXIMUM,cx,&mut slot)?;Ok(())});slot.expect("producer birth fits the fixture grant")}
 fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,value:T){let mut value=Some(value);let mut close=RetainedCloneClose::default();for _ in 0..64{if value.is_none()&&close.is_empty(){return}turn(caller,decoder,|cx|cx.with_original_retirement(|cx|{let d=if !close.is_empty(){close.demands(G.maximum_copy_bytes)?}else{close.next_owner_demands::<T>(true,&None,G.maximum_copy_bytes)?};close_original_retained_step(cx,d,|g|{if !close.is_empty(){return match close.select(G.maximum_copy_bytes){Ok(selected)=>selected.step(g),Err(error)=>semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0}}}let turn=close.begin_granted(&mut value,g);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.expect("original owner admission returns its physical step")),work:turn.work}})?;Ok(())}))}panic!("retirement did not finish")}
 fn finish(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){while !decoder.terminal_is_empty(){caller.turn(G,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(G.maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();}caller.retire(G);}
 fn caller(operation:u64)->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(operation),Generation(1),STOCK,default_now_us,u64::MAX,1<<30)}
 fn produce(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,producer:&mut Box<dyn ArtifactOriginalCommandProducer<Command>>,action:&str,args:&mut Option<DslValue>){for _ in 0..64{if producer.ready(){return}turn(caller,decoder,|cx|producer.advance(action,args,&mut None,cx));}panic!("producer did not become ready")}
 fn wire_bytes(wire:&RetainedToolWireInput)->Vec<u8>{(0..wire.page_count()).flat_map(|index|wire.page(index).unwrap().to_vec()).collect()}
 #[test]
 fn born_command_is_written_as_the_exact_wire_pages_and_handed_over_together(){
  let kind="k".repeat(10_000);let oracle=serde_json::to_vec(&serde_json::json!({"kind":kind,"count":7})).unwrap();
  let mut caller=caller(901);let mut decoder=OriginalJobDecodeOwner::empty();
  let mut producer=born(&mut caller,&mut decoder,Some(Command{kind:kind.clone(),count:7}),None);
  produce(&mut caller,&mut decoder,&mut producer,"",&mut None);
  let(mut command,mut wire)=(None,None);turn(&mut caller,&mut decoder,|cx|producer.take_into(&mut command,&mut wire,cx));
  assert!(producer.terminal_is_empty());
  let wire=wire.unwrap();assert!(wire.original_is_sealed());assert_eq!(wire.page_count(),oracle.len().div_ceil(TOOL_WIRE_PAGE_BYTES));assert_eq!(wire_bytes(&wire),oracle);assert_eq!(command.as_ref().unwrap().count,7);
  retire(&mut caller,&mut decoder,command);retire(&mut caller,&mut decoder,wire);drop(producer);finish(caller,&mut decoder);
 }
 #[test]
 fn declared_action_bridge_births_the_command_from_the_action_and_its_arguments_then_encodes_it(){
  let oracle=serde_json::to_vec(&serde_json::json!({"kind":"setWidget","count":41})).unwrap();
  let mut caller=caller(903);let mut decoder=OriginalJobDecodeOwner::empty();
  let factory=OriginalActionCommandBridge::<Command>(bridge,|_|RUNTIME_COMMAND_WIRE_MAXIMUM);let mut producer=None;
  assert!(turn(&mut caller,&mut decoder,|cx|factory.admit(cx,&mut producer)));let mut producer=producer.unwrap();
  let mut args=Some(DslValue::String("41".into()));produce(&mut caller,&mut decoder,&mut producer,"setWidget",&mut args);assert!(args.is_none());
  let(mut command,mut wire)=(None,None);turn(&mut caller,&mut decoder,|cx|producer.take_into(&mut command,&mut wire,cx));
  assert_eq!(wire_bytes(wire.as_ref().unwrap()),oracle);assert_eq!(command.as_ref().unwrap().kind,"setWidget");
  retire(&mut caller,&mut decoder,command);retire(&mut caller,&mut decoder,wire);drop(producer);finish(caller,&mut decoder);
 }
 #[test]
 fn declared_action_bridge_refusal_keeps_the_apps_fault_and_returns_every_owner(){
  let mut caller=caller(904);let mut decoder=OriginalJobDecodeOwner::empty();
  let mut producer=born(&mut caller,&mut decoder,None,Some(bridge));
  let mut staged=Some(Fault::new(FaultOrigin::App,FaultCode::new("app.args.invalid"),"staged refusal"));
  let failure=caller.turn_with_decoder(G,&mut decoder,|cx|producer.advance("setWidget",&mut None,&mut staged,cx)).unwrap_err();
  assert!(format!("{failure:?}").contains("app.args.invalid"));
  for _ in 0..64{if producer.terminal_is_empty(){break}turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|producer.close_step(cx)));}
  assert!(producer.terminal_is_empty());drop(producer);finish(caller,&mut decoder);
 }
 #[test]
 fn producer_cancelled_at_every_phase_returns_every_owner_through_paid_closure(){
  let kind="k".repeat(10_000);
  for cut in 0..8{
   let mut caller=caller(902);let mut decoder=OriginalJobDecodeOwner::empty();
   let mut producer=born(&mut caller,&mut decoder,Some(Command{kind:kind.clone(),count:7}),None);
   for _ in 0..cut{if producer.ready(){break}turn(&mut caller,&mut decoder,|cx|producer.advance("",&mut None,&mut None,cx));}
   for _ in 0..64{if producer.terminal_is_empty(){break}turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|producer.close_step(cx)));}
   assert!(producer.terminal_is_empty(),"cut {cut}");drop(producer);finish(caller,&mut decoder);
  }
 }
 #[test]
 fn owned_producer_limit_admits_large_wire_and_keeps_ordinary_limit(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("limit.json")).unwrap();let kind="k".repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);let oracle=serde_json::to_vec(&serde_json::json!({"kind":kind,"count":fixture["count"]})).unwrap();
  for large in[true,false]{
   let mut caller=caller(if large{905}else{906});let mut decoder=OriginalJobDecodeOwner::empty();let mut slot=None;let mut command=Some(Box::new(Command{kind:kind.clone(),count:7}));let maximum:fn(&Command)->usize=if large{ |_|70_128 }else{ |_|RUNTIME_COMMAND_WIRE_MAXIMUM };
   turn(&mut caller,&mut decoder,|cx|{admit_op_binary_producer(||command.take(),None,maximum,cx,&mut slot)?;Ok(())});let mut producer=slot.unwrap();let mut failure=None;
   for _ in 0..4096{if producer.ready(){break;}if let Err(error)=caller.turn_with_decoder(G,&mut decoder,|cx|producer.advance("",&mut None,&mut None,cx)){failure=Some(error);break;}}
   if large{assert!(failure.is_none());assert!(producer.ready());let(mut command,mut wire)=(None,None);turn(&mut caller,&mut decoder,|cx|producer.take_into(&mut command,&mut wire,cx));assert_eq!(wire_bytes(wire.as_ref().unwrap()),oracle);assert!(oracle.len()>fixture["ordinaryBytes"].as_u64().unwrap()as usize);assert!(oracle.len()<=fixture["maximumBytes"].as_u64().unwrap()as usize);retire(&mut caller,&mut decoder,command);retire(&mut caller,&mut decoder,wire);}else{assert!(format!("{:?}",failure.expect("ordinary producer refuses oversized wire")).contains("declared maximum"));}
   for _ in 0..4096{if producer.terminal_is_empty(){break;}turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|producer.close_step(cx)));}assert!(producer.terminal_is_empty());drop(producer);finish(caller,&mut decoder);
  }
 }

 #[test]
 fn invalid_owned_wire_limits_refuse_before_allocation_and_keep_command_for_paid_close(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("limit.json")).unwrap();
  for kind in fixture["invalidMaximumKinds"].as_array().unwrap(){let maximum=match kind.as_str().unwrap(){"zero"=>0,"usizeMaximum"=>usize::MAX,_=>panic!("unknown neutral limit case")};
   let mut caller=caller(907);let mut decoder=OriginalJobDecodeOwner::empty();let mut command=Some(Box::new(Command{kind:"owned".into(),count:7}));let mut slot=None;let selector:fn(&Command)->usize=if maximum==0{ |_|0 }else{ |_|usize::MAX };
   turn(&mut caller,&mut decoder,|cx|{admit_op_binary_producer(||command.take(),None,selector,cx,&mut slot)?;Ok(())});assert!(command.is_none());let mut producer=slot.unwrap();let failure=caller.turn_with_decoder(G,&mut decoder,|cx|producer.advance("",&mut None,&mut None,cx)).unwrap_err();assert!(format!("{failure:?}").contains("portable allocation bounds"));
   for _ in 0..64{if producer.terminal_is_empty(){break;}turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|producer.close_step(cx)));}assert!(producer.terminal_is_empty());drop(producer);finish(caller,&mut decoder);
  }
 }

}
