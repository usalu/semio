//! 🪢️ A refused original projector retains every partial field until funded closure.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand};
use semio_framework_value::native_encoding::{NativeEncodeRetirementRecipient,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalSourceTurnIdentity,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient};
use crate::original_close::close_encode;
use crate::original_close::Cell;

struct Recipient<'a>{grant:RetainedCloneGrant,received:RetainedCloneProgress,armed:&'a Cell<bool>,deny:bool,refused:bool}
impl OriginalConstructionRecipient for Recipient<'_>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a> where 'source:'turn{quote.complete(Ok(self.grant))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a> where 'source:'turn{let progress=receipt.progress();let posted=self.received.checked_add(progress).map(|total|self.received=total).and_then(|()|if self.deny&&self.armed.get()&&!self.refused{self.refused=true;Err(ValueError::literal(ValueRefusalKind::Canceled,"original projection receipt refused"))}else{Ok(())});receipt.complete(posted)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.received}
}
struct Work(u64);
impl OriginalWorkRecipient for Work{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
}
fn partial(frame:&ProjectFrame,total:usize)->bool{
 let Some(DslValue::Object(root))=frame.value.as_ref()else{return false};let Some((_,DslValue::Array(rows)))=root.iter().find(|(key,_)|key=="checkpoints")else{return false};let Some(DslValue::Object(fields))=rows.first()else{return false};let Some((key,DslValue::String(message)))=fields.iter().find(|(key,_)|key=="message")else{return false};root.capacity()==2&&rows.capacity()==1&&fields.capacity()==5&&key.capacity()==key.len()&&!message.is_empty()&&message.len()<=total&&message.capacity()==total
}
#[test]
fn original_partial_projection_keeps_keys_values_and_backing_until_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪢️partial-projection/🔣️.json")).unwrap();let text=law["unit"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize);assert_eq!(text.len(),law["utf8Bytes"].as_u64().unwrap()as usize);let canonical=serde_json::json!({"value":text.as_str()});assert_eq!(serde_json::from_str::<serde_json::Value>(&serde_json::to_string(&canonical).unwrap()).unwrap(),canonical);
 for case in law["cases"].as_array().unwrap(){
  let deny=case.as_str().unwrap()=="receiptRefusal";let mut source=SpaceHistorySnapshot::default();source.checkpoints.push(SpaceCheckpoint{id:"checkpoint".into(),parent_id:None,message:text.clone(),authors:Vec::new(),timestamp:HybridLogicalTimestamp{actor:0,physical_ms:0,logical:0},members:Vec::new()});let original=source.clone();let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:128};let armed=Cell::new(false);let live=Cell::new(true);let seen=Cell::new(false);let limit=law["cancelAfterBytes"].as_u64().unwrap()as usize;let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if event.total==text.len()&&event.completed>=limit&&event.completed<event.total{armed.set(true);if !deny{live.set(false)}}live.get()};let mut retirement=NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(grant.maximum_capacity_bytes,&mut observer);native.install_retirement_recipient(&mut retirement).unwrap();let mut recipient=OriginalConstructionAuthority::new(Recipient{grant,received:Default::default(),armed:&armed,deny,refused:false});let mut output=None;
  let mut work=OriginalWorkAuthority::new(Work(1000000));let reported=Cell::new(0);
  work.with_loan(&mut (),|work_loan|native.with_work_recipient(work_loan,|native|{
   let (turn,born,released)=semio_framework_trace_memory_testing::observe_backing(||{recipient.with_loan(&mut (),|construction_loan|native.with_construction_recipient(construction_loan,|native|{let mut owner=semio_framework_io_control::NativeSnapshotEncodeOwner::new(native,grant);Ok::<_,ValueError>(owner.receive::<ProjectFrame,(),ProjectError>(&mut output,&mut |frame,_output,marker,unwind,native,body|{let turn=semio_framework_io_control::native_snapshot_turn(|work|{encoded_effect(work,native,body,marker,unwind,size_of::<Option<ProjectFrame>>(),1,||*frame=Some(ProjectFrame{value:None,allocation:Default::default()}))?;project_into(&source,frame.as_mut().unwrap(),work,native,body,marker,unwind)});seen.set(frame.as_ref().is_some_and(|frame|partial(frame,text.len())));turn}))})).unwrap().unwrap()});
  assert!(armed.get());assert!(seen.get());assert!(turn.result.is_err());assert!(turn.work>0);reported.set(turn.work);assert!(output.is_none());assert_eq!(recipient.inspect(|recipient|recipient.refused),deny);assert!(recipient.inspect(|recipient|recipient.received.retained_capacity_bytes)>=text.len());assert!(native.has_retirement_owner());assert_eq!(released,0);live.set(true);let(zero,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||close_encode(native,Default::default()).result);assert_eq!((allocated,released),(0,0));if let Ok(step)=zero{assert_eq!(step.progress(),Default::default())}assert!(native.has_retirement_owner());let mut total_born=born;let mut total_released=0;for _ in 0..100000{if !native.has_retirement_owner(){break}let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||close_encode(native,grant).result.unwrap());assert!(step.progress().fits(grant));total_born+=allocated;total_released+=released;}assert!(!native.has_retirement_owner());assert_eq!(total_released,total_born);assert_eq!(source,original);Ok::<(),ValueError>(())
  })).unwrap().unwrap();

 }
}
