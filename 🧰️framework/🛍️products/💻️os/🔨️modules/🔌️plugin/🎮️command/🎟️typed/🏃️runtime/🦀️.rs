//! 🏃️ A command the runtime itself originates enters the original typed source through the generic OpBinary producer the declaration crate owns.
use super::*;
use semio_plugin_decl_app::tool_contract::admit_op_binary_producer;
pub use semio_plugin_decl_app::tool_contract::RUNTIME_COMMAND_WIRE_MAXIMUM;
impl<A:ArtifactApp> OriginalTypedCommandSource<A>{
 /// 📣️ Domain command storage is born under its declared original quote before the wire producer receives it.
 pub fn produce_host_event(&mut self,event:&semio_plugin_decl_emission::HostEvent,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
  self.validate(cx)?;if self.runtime_ignored{return Ok(Some(false))}if self.caller.is_none(){if !cx.original_source_parent()?.admit_loan(&mut *self.caller,cx)?{return Ok(None)}}if self.command.is_some()||self.wire.is_some(){if self.command.is_none()||self.wire.is_none(){return Err(refused().into())}if self.producer.is_some(){self.close_producer(cx)?;return Ok(None)}return Ok(Some(true))}
  if self.producer.is_none(){if self.runtime_input.is_none(){let Some(mut d)=A::host_event_demands(event)else{return Ok(Some(false))};let header=2*size_of::<Option<Box<A::Command>>>()+2*size_of::<bool>();d.copy_bytes=d.copy_bytes.checked_add(header).ok_or_else(refused)?;stepped(cx,self,|s|&mut s.construction,d,|s|{let command=A::host_event(event).map(Box::new);let p=if command.is_some(){receipt(d)}else{semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<Option<Box<A::Command>>>()+2*size_of::<bool>(),..Default::default()}};s.runtime_ignored=command.is_none();s.host_event=command.is_some();*s.runtime_input=command;Ok(p)})?;return Ok(None)}let original=&mut self.runtime_input;admit_op_binary_producer(||original.take(),None,|_|RUNTIME_COMMAND_WIRE_MAXIMUM,cx,&mut self.construction,&mut self.producer)?;return Ok(None)}let producer=self.producer.as_mut().unwrap();if !producer.ready(){producer.advance("",&mut None,&mut None,cx)?;return Ok(None)}producer.take_into(&mut self.command,&mut self.wire,cx)?;Ok(None)
 }
 /// 🏃️ A command the runtime originates is born into the producer slot once, then encoded and handed to this source together with its sealed wire.
 pub fn produce_runtime(&mut self,make:impl FnOnce()->Option<Box<A::Command>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;
  if self.caller.is_none(){if !cx.original_source_parent()?.admit_loan(&mut *self.caller,cx)?{return Ok(false)}}
  if self.command.is_some()||self.wire.is_some(){if self.command.is_none()||self.wire.is_none(){return Err(refused().into())}if self.producer.is_some(){self.close_producer(cx)?;return Ok(false)}return Ok(true)}
  if self.producer.is_none(){admit_op_binary_producer(make,None,|_|RUNTIME_COMMAND_WIRE_MAXIMUM,cx,&mut self.construction,&mut self.producer)?;return Ok(false)}
  let producer=self.producer.as_mut().unwrap();
  if !producer.ready(){producer.advance("",&mut None,&mut None,cx)?;return Ok(false)}
  producer.take_into(&mut self.command,&mut self.wire,cx)?;Ok(false)
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 use crate::command_admission::{begun,retired_poll};
 use semio_framework::{Fault,FaultCode,FaultOrigin,action_bus::{RetainedToolWireInput,TOOL_WIRE_PAGE_BYTES}};
 use semio_plugin_decl_app::tool_contract::{ArtifactOriginalCommandProducer,ArtifactOriginalCommandProducerFactory,OriginalActionCommandBridge,OriginalCommandBirth,admit_op_binary_producer};
 use semio_framework_value::{DslValue,RetainedCloneGrant,RetirementDemand,retirement::RetireOwned};
 use protocol::OpBinary;
 use semio_framework_job::{OriginalConstructionAdmission,OriginalConstructionDemand,OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,default_now_us};
 #[derive(semio_framework_value::RetireOwned)]struct Command{kind:String,count:u32}
 impl OpBinary for Command{
  fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{Ok(serde_json::to_vec(&serde_json::json!({"kind":self.kind,"count":self.count})).expect("json encodes"))}
  fn decode_op(_:&[u8])->Result<Self,protocol::ProtocolError>{Err(protocol::ProtocolError::TornTail(0))}
 }
 fn bridge(action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>)->Result<Command,Fault>{if let Some(fault)=fault.take(){return Err(fault)}Ok(Command{kind:action.to_owned(),count:args.take().and_then(|value|value.as_str().and_then(|text|text.parse().ok())).unwrap_or(0)})}
 /// 📐️ The wallet of the test caller; its capacity, release and depth are not the bound under test, each turn's grant is, derived from the owner's own quote by `funded`.
 const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:4096};
 const LARGEST_WIRE:usize=70_128;
 fn funded(demand:RetirementDemand)->RetainedCloneGrant{funded_items(demand,1)}
 fn funded_items(demand:RetirementDemand,items:usize)->RetainedCloneGrant{
  let demand=OriginalConstructionDemand{items,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth};
  let quote=|candidate:fn(OriginalConstructionDemand)->RetainedCloneGrant|candidate(demand);
  fn of<O>(demand:OriginalConstructionDemand)->RetainedCloneGrant{semio_framework_job::original_construction_settled_turn_quote::<O>(demand,1).expect("bounded construction quote").0}
  [quote(of::<Box<Command>>),quote(of::<Vec<u8>>),quote(of::<RetainedToolWireInput>),quote(of::<()>)].into_iter().reduce(|a,b|RetainedCloneGrant{maximum_items:a.maximum_items.max(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.max(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.max(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.max(b.maximum_release_bytes),maximum_depth:a.maximum_depth.max(b.maximum_depth)}).expect("candidate quotes")
 }
 fn grant()->RetainedCloneGrant{
  let birth=OriginalActionCommandBridge::<Command>(bridge,|_|RUNTIME_COMMAND_WIRE_MAXIMUM).birth_demands().expect("producer birth quote");
  let pages=3*std::mem::size_of::<semio_framework::action_bus::ToolWirePage>()+TOOL_WIRE_PAGE_BYTES;let wire=RetainedToolWireInput::original_birth_demands(LARGEST_WIRE,LARGEST_WIRE).expect("wire birth quote");
  funded(RetirementDemand{copy_bytes:birth.copy_bytes.max(pages).max(wire.copy_bytes),capacity_bytes:birth.capacity_bytes,release_bytes:birth.release_bytes,depth:birth.depth.max(wire.depth)})
 }
 fn body()->usize{grant().maximum_copy_bytes}
 fn turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{caller.turn_with_decoder(grant(),decoder,run).unwrap()}
 fn quoted<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,demand:Result<RetirementDemand,semio_framework_value::ValueError>,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{caller.turn_with_decoder(funded(demand.expect("owner step quote")),decoder,run).unwrap()}
 fn closing<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,(demand,items):(Result<RetirementDemand,semio_framework_value::ValueError>,usize),run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{caller.turn_with_decoder(funded_items(demand.expect("owner step quote"),items),decoder,run).unwrap()}
 fn born(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,make:Option<Command>,birth:Option<OriginalCommandBirth<Command>>)->Box<dyn ArtifactOriginalCommandProducer<Command>>{let mut slot=None;let mut admission=OriginalConstructionAdmission::default();let mut make=make.map(Box::new);for _ in 0..64{if slot.is_some(){break}turn(caller,decoder,|cx|{admit_op_binary_producer(||make.take(),birth,|_|RUNTIME_COMMAND_WIRE_MAXIMUM,cx,&mut admission,&mut slot)?;Ok(())});}slot.expect("producer birth fits the fixture grant")}
 fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,value:T){let mut value=Some(value);let mut close=RetainedCloneClose::default();let body=body();for _ in 0..64{if value.is_none()&&close.is_empty(){return}let d=if close.is_empty(){close.next_owner_demands::<T>(true,&None,body)}else{close.demands(body)};quoted(caller,decoder,d,|cx|cx.with_original_retirement(|cx|{let d=if close.is_empty(){close.next_owner_demands::<T>(true,&None,body)?}else{close.demands(body)?};if close.is_empty(){begun(cx,d,|effect|close.begin_granted(&mut value,effect))?;}else{retired_poll(cx,d,|effect|close.step_granted(body,effect))?;}Ok(())}))}panic!("retirement did not finish")}
 fn finish(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for _ in 0..semio_framework_job::ORIGINAL_DECODER_TEARDOWN_STEPS{let(quote,_)=semio_framework_job::original_decoder_teardown_quote(decoder).expect("bounded teardown quote");caller.turn(quote,|cx|{decoder.close_on_source(cx)?;Ok(())}).unwrap();}assert!(decoder.terminal_is_empty(),"decoder teardown exceeded its quoted steps");let source=funded(caller.retirement_demands().expect("Source closure quote"));caller.retire(source);}
 fn caller(operation:u64)->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(operation),Generation(1),STOCK,default_now_us,u64::MAX,1<<30)}
 fn produce(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,producer:&mut Box<dyn ArtifactOriginalCommandProducer<Command>>,action:&str,args:&mut Option<DslValue>){for _ in 0..64{if producer.ready(){return}let d=producer.demands(action,args,&None,body());quoted(caller,decoder,d,|cx|producer.advance(action,args,&mut None,cx));}panic!("producer did not become ready")}
 fn close_producer(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,producer:&mut Box<dyn ArtifactOriginalCommandProducer<Command>>){while !producer.terminal_is_empty(){let fuel=caller.fuel_remaining();closing(caller,decoder,(producer.close_demands(body()),producer.close_items()),|cx|cx.with_original_retirement(|cx|producer.close_step(cx)));assert!(caller.fuel_remaining()<fuel,"a paid close step must spend its quoted Work")}}
 fn wire_bytes(wire:&RetainedToolWireInput)->Vec<u8>{(0..wire.page_count()).flat_map(|index|wire.page(index).unwrap().to_vec()).collect()}
 #[test]
 fn born_command_is_written_as_the_exact_wire_pages_and_handed_over_together(){
  let kind="k".repeat(10_000);let oracle=serde_json::to_vec(&serde_json::json!({"kind":kind,"count":7})).unwrap();
  let mut caller=caller(901);let mut decoder=caller.admitted_decoder();
  let mut producer=born(&mut caller,&mut decoder,Some(Command{kind:kind.clone(),count:7}),None);
  produce(&mut caller,&mut decoder,&mut producer,"",&mut None);
  let(mut command,mut wire)=(None,None);quoted(&mut caller,&mut decoder,producer.take_demands(),|cx|producer.take_into(&mut command,&mut wire,cx));
  close_producer(&mut caller,&mut decoder,&mut producer);assert!(producer.terminal_is_empty());
  let wire=wire.unwrap();assert!(wire.original_is_sealed());assert_eq!(wire.page_count(),oracle.len().div_ceil(TOOL_WIRE_PAGE_BYTES));assert_eq!(wire_bytes(&wire),oracle);assert_eq!(command.as_ref().unwrap().count,7);
  retire(&mut caller,&mut decoder,command);retire(&mut caller,&mut decoder,wire);drop(producer);finish(caller,&mut decoder);
 }
 #[test]
 fn declared_action_bridge_births_the_command_from_the_action_and_its_arguments_then_encodes_it(){
  let oracle=serde_json::to_vec(&serde_json::json!({"kind":"setWidget","count":41})).unwrap();
  let mut caller=caller(903);let mut decoder=caller.admitted_decoder();
  let factory=OriginalActionCommandBridge::<Command>(bridge,|_|RUNTIME_COMMAND_WIRE_MAXIMUM);let mut producer=None;let mut admission=OriginalConstructionAdmission::default();
  for _ in 0..64{if producer.is_some(){break}turn(&mut caller,&mut decoder,|cx|factory.admit(cx,&mut admission,&mut producer));}assert!(producer.is_some());let mut producer=producer.unwrap();
  let mut args=Some(DslValue::String("41".into()));produce(&mut caller,&mut decoder,&mut producer,"setWidget",&mut args);assert!(args.is_none());
  let(mut command,mut wire)=(None,None);quoted(&mut caller,&mut decoder,producer.take_demands(),|cx|producer.take_into(&mut command,&mut wire,cx));close_producer(&mut caller,&mut decoder,&mut producer);
  assert_eq!(wire_bytes(wire.as_ref().unwrap()),oracle);assert_eq!(command.as_ref().unwrap().kind,"setWidget");
  retire(&mut caller,&mut decoder,command);retire(&mut caller,&mut decoder,wire);drop(producer);finish(caller,&mut decoder);
 }
 #[test]
 fn declared_action_bridge_refusal_keeps_the_apps_fault_and_returns_every_owner(){
  let mut caller=caller(904);let mut decoder=caller.admitted_decoder();
  let mut producer=born(&mut caller,&mut decoder,None,Some(bridge));
  let mut staged=Some(Fault::new(FaultOrigin::App,FaultCode::new("app.args.invalid"),"staged refusal"));
  let failure=caller.turn_with_decoder(grant(),&mut decoder,|cx|producer.advance("setWidget",&mut None,&mut staged,cx)).unwrap_err();
  assert!(format!("{failure:?}").contains("app.args.invalid"));
  close_producer(&mut caller,&mut decoder,&mut producer);
  assert!(producer.terminal_is_empty());drop(producer);finish(caller,&mut decoder);
 }
 #[test]
 fn producer_cancelled_at_every_phase_returns_every_owner_through_paid_closure(){
  let kind="k".repeat(10_000);
  for cut in 0..8{
   let mut caller=caller(902);let mut decoder=caller.admitted_decoder();
   let mut producer=born(&mut caller,&mut decoder,Some(Command{kind:kind.clone(),count:7}),None);
   for _ in 0..cut{if producer.ready(){break}quoted(&mut caller,&mut decoder,producer.demands("",&None,&None,body()),|cx|producer.advance("",&mut None,&mut None,cx));}
   close_producer(&mut caller,&mut decoder,&mut producer);
   assert!(producer.terminal_is_empty(),"cut {cut}");drop(producer);finish(caller,&mut decoder);
  }
 }
 #[test]
 fn owned_producer_limit_admits_large_wire_and_keeps_ordinary_limit(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("limit.json")).unwrap();let kind="k".repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);let oracle=serde_json::to_vec(&serde_json::json!({"kind":kind,"count":fixture["count"]})).unwrap();
  for large in[true,false]{
   let mut caller=caller(if large{905}else{906});let mut decoder=caller.admitted_decoder();let mut slot=None;let mut command=Some(Box::new(Command{kind:kind.clone(),count:7}));let maximum:fn(&Command)->usize=if large{ |_|70_128 }else{ |_|RUNTIME_COMMAND_WIRE_MAXIMUM };
   let mut admission=OriginalConstructionAdmission::default();for _ in 0..64{if slot.is_some(){break}turn(&mut caller,&mut decoder,|cx|{admit_op_binary_producer(||command.take(),None,maximum,cx,&mut admission,&mut slot)?;Ok(())});}let mut producer=slot.unwrap();let mut failure=None;
   for _ in 0..4096{if producer.ready(){break;}let attempt=match producer.demands("",&None,&None,body()){Ok(demand)=>caller.turn_with_decoder(funded(demand),&mut decoder,|cx|producer.advance("",&mut None,&mut None,cx)).map(drop),Err(error)=>Err(error.into())};if let Err(error)=attempt{failure=Some(error);break;}}
   if large{assert!(failure.is_none());assert!(producer.ready());let(mut command,mut wire)=(None,None);quoted(&mut caller,&mut decoder,producer.take_demands(),|cx|producer.take_into(&mut command,&mut wire,cx));assert_eq!(wire_bytes(wire.as_ref().unwrap()),oracle);assert!(oracle.len()>fixture["ordinaryBytes"].as_u64().unwrap()as usize);assert!(oracle.len()<=fixture["maximumBytes"].as_u64().unwrap()as usize);retire(&mut caller,&mut decoder,command);retire(&mut caller,&mut decoder,wire);}else{assert!(format!("{:?}",failure.expect("ordinary producer refuses oversized wire")).contains("declared maximum"));}
   close_producer(&mut caller,&mut decoder,&mut producer);assert!(producer.terminal_is_empty());drop(producer);finish(caller,&mut decoder);
  }
 }

 #[test]
 fn invalid_owned_wire_limits_refuse_before_allocation_and_keep_command_for_paid_close(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("limit.json")).unwrap();
  for kind in fixture["invalidMaximumKinds"].as_array().unwrap(){let maximum=match kind.as_str().unwrap(){"zero"=>0,"usizeMaximum"=>usize::MAX,_=>panic!("unknown neutral limit case")};
   let mut caller=caller(907);let mut decoder=caller.admitted_decoder();let mut command=Some(Box::new(Command{kind:"owned".into(),count:7}));let mut slot=None;let selector:fn(&Command)->usize=if maximum==0{ |_|0 }else{ |_|usize::MAX };
   let mut admission=OriginalConstructionAdmission::default();for _ in 0..64{if slot.is_some(){break}turn(&mut caller,&mut decoder,|cx|{admit_op_binary_producer(||command.take(),None,selector,cx,&mut admission,&mut slot)?;Ok(())});}assert!(command.is_none());let mut producer=slot.unwrap();let failure=OriginalTurnFailure::from(producer.demands("",&None,&None,body()).unwrap_err());assert!(format!("{failure:?}").contains("portable allocation bounds"));
   close_producer(&mut caller,&mut decoder,&mut producer);assert!(producer.terminal_is_empty());drop(producer);finish(caller,&mut decoder);
  }
 }

}
