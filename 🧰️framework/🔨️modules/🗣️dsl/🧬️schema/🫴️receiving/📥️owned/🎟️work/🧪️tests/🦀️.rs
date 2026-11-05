use super::*;
#[path="../../../🎟️work/🧪️tests/📥️bank/🦀️.rs"]mod funding;
use funding::{Bank,observe};
use std::sync::atomic::{AtomicUsize,Ordering};
static CONSTRUCTORS:AtomicUsize=AtomicUsize::new(0);
static DROPS:AtomicUsize=AtomicUsize::new(0);
#[derive(semio_framework_value::RetireOwned)]struct HeaderValue(u64);
struct HeaderCursor{value:Option<HeaderValue>}
impl Drop for HeaderCursor{fn drop(&mut self){DROPS.fetch_add(1,Ordering::SeqCst);panic!("closed generic cursor Drop must never run")}}
impl DslOwnedReceivingField for HeaderValue{type Cursor=HeaderCursor;fn receiving_cursor()->Self::Cursor{CONSTRUCTORS.fetch_add(1,Ordering::SeqCst);HeaderCursor{value:None}}}
impl OwnedFieldReceiving<HeaderValue>for HeaderCursor{
 fn demands(&self,_:&Option<FieldValue>,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:std::mem::size_of::<u64>(),depth:1,..Default::default()})}
 fn step(&mut self,source:&mut Option<FieldValue>,_:&mut NativeDecodeControl<'_>,_:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let d=self.demands(source,4096).unwrap();if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let Some(FieldValue::UInt(value))=source.as_ref()else{return RetainedWorkOutcome{result:Err(invalid().into()),work:0}};self.value=Some(HeaderValue(*value));source.take();RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}
 fn is_ready(&self)->bool{self.value.is_some()}
 fn take_into(&mut self,output:&mut Option<HeaderValue>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if output.is_some(){return RetainedWorkOutcome{result:Err(invariant()),work:0}}if !funds(g,metadata::<()>()){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}*output=self.value.take();RetainedWorkOutcome{result:Ok(receipt(metadata::<()>())),work:1}}
 fn close_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_step(&mut self,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{assert!(self.value.is_none());RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
 fn terminal_is_empty(&self)->bool{self.value.is_none()}
}
#[test]fn actual_owned_cursor_header_birth_denials_and_paid_close_skip_generic_drop(){let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🎟️work/📐️.json")).unwrap();CONSTRUCTORS.store(0,Ordering::SeqCst);DROPS.store(0,Ordering::SeqCst);let mut bank=Bank::new(&law["stock"]);let mut source=Some(FieldValue::UInt(law["cases"][0]["input"].as_u64().unwrap()));let(mut slot,h)=observe(OwnedReceivingSlot::<HeaderValue>::new);assert_eq!(h,(0,0));assert_eq!(CONSTRUCTORS.load(Ordering::SeqCst),0);let mut callback=|_|true;let mut native=NativeDecodeControl::new(1<<20,&mut callback);let mut allocated=false;for _ in 0..100{if slot.is_ready(){break}let d=slot.demands(&source,4096).unwrap();let g=bank.grant(4096);for denied in funding::denied(g,d){let before=CONSTRUCTORS.load(Ordering::SeqCst);let(turn,h)=observe(||funding::receive(&mut native,denied,|native|slot.step(&mut source,native,4096,denied)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());assert_eq!(h,(0,0));assert_eq!(CONSTRUCTORS.load(Ordering::SeqCst),before);assert_eq!(slot.demands(&source,4096).unwrap(),d);}let(turn,h)=observe(||funding::receive(&mut native,g,|native|slot.step(&mut source,native,4096,g)));let p=turn.result.as_ref().copied().unwrap_or_else(|e|e.retained_progress());if p.retained_capacity_bytes>0{allocated=true;assert_eq!(CONSTRUCTORS.load(Ordering::SeqCst),0);}if CONSTRUCTORS.load(Ordering::SeqCst)>0{assert!(allocated)}bank.post(g,p,turn.work,h);turn.result.unwrap();}assert!(slot.is_ready());let mut output=None;let g=bank.grant(4096);let(turn,h)=observe(||slot.take_into(&mut output,g));let p=*turn.result.as_ref().unwrap();assert_eq!(p.copied_bytes,law["ownershipCopyBytes"].as_u64().unwrap()as usize);bank.post(g,p,turn.work,h);turn.result.unwrap();assert_eq!(serde_json::to_value(output.as_ref().unwrap().0).unwrap(),law["cases"][0]["expected"]);for _ in 0..100{if slot.terminal_is_empty(){break}let d=slot.close_demands(4096).unwrap();let g=bank.grant(4096);for denied in funding::denied(g,d){let(turn,h)=observe(||slot.close_step(denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());assert_eq!(h,(0,0));}let(turn,h)=observe(||slot.close_step(g));let p=turn.result.as_ref().copied().unwrap_or_else(|e|e.retained_progress());bank.post(g,p,turn.work,h);turn.result.unwrap();}assert!(slot.terminal_is_empty());assert_eq!(CONSTRUCTORS.load(Ordering::SeqCst),law["headerConstructorCalls"].as_u64().unwrap()as usize);assert_eq!(DROPS.load(Ordering::SeqCst),law["closedHeaderDropCalls"].as_u64().unwrap()as usize);assert_eq!(observe(||drop(slot)).1,(0,0));funding::close(output.take().unwrap(),&mut bank);assert!(source.is_none());}

#[test]
fn declared_record_work_matches_neutral_success_refusal_and_independent_serde() {
 #[derive(serde::Deserialize)]
 #[serde(deny_unknown_fields)]
 struct Oracle{#[serde(rename="0")]required:u64,#[serde(rename="1")]optional:Option<String>}
 let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🎟️work/📐️.json")).unwrap();
 let shape=&law["recordShape"];
 let declared=shape["declared"].as_array().unwrap().iter().map(|row|(row[0].as_u64().unwrap()as u16,row[1].as_bool().unwrap())).collect::<Vec<_>>();
 let grant=RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
 for row in shape["cases"].as_array().unwrap(){
  let allowed=row["allowed"].as_bool().unwrap();
  let independent=serde_json::from_value::<Oracle>(row["value"].clone()).map(|value|(value.required,value.optional));assert_eq!(independent.is_ok(),allowed,"{}",row["id"]);
  let mut record=RecordValue::default();
  for(key,value)in row["value"].as_object().unwrap(){record.fields.insert(key.parse().unwrap(),if let Some(value)=value.as_u64(){FieldValue::UInt(value)}else{FieldValue::Text(value.as_str().unwrap().into())});}
  let original=record.clone();
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
   let mut callback=|_|true;let mut native=NativeDecodeControl::new(65536,&mut callback);let mut owner=super::super::NativeSnapshotDecodeOwner::new(&mut native,denied);
   let(turn,heap)=observe(||super::super::receiving_turn(|_|declared_record(&record,&declared,&mut owner).map_err(Into::into)));
   assert!(turn.result.is_err());assert_eq!(turn.work,0);assert_eq!(owner.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(record,original);
  }
  let mut callback=|_|true;let mut native=NativeDecodeControl::new(65536,&mut callback);let mut owner=super::super::NativeSnapshotDecodeOwner::new(&mut native,grant);
  let(turn,heap)=observe(||super::super::receiving_turn(|_|declared_record(&record,&declared,&mut owner).map_err(Into::into)));
  assert_eq!(turn.result.is_ok(),allowed,"{}",row["id"]);assert_eq!(turn.work,row["work"].as_u64().unwrap(),"{}",row["id"]);
  assert_eq!(owner.progress().copied_items,row["probes"].as_u64().unwrap()as usize);assert!(owner.progress().fits(grant));assert_eq!(heap,(0,0));assert_eq!(record,original);
 }
}
