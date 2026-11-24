//! 🔢️ The same declared storage rows exercise the actual array initialized prefix instead of a removed Option-array container.
use super::*;
use crate::source_fixture::{self as driver,EffectBank,Wallet,Trace};
const BODY:usize=4096;
fn stock(fixture:&serde_json::Value)->EffectBank{EffectBank::new(Wallet::from_profile(&fixture["stock"]))}
#[test]
fn sequence_storage_refusal_retains_the_exact_unpublished_value(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🌳️value/📖️source/🧪️tests/🧫️fixtures/📐️.json")).unwrap();
 for row in fixture["storage"].as_array().unwrap(){
  let value=row["value"].as_u64().unwrap();let index=row["index"].as_u64().unwrap()as usize;let accepted=row["accepted"].as_bool().unwrap();let mut bank=stock(&fixture);
  if row["kind"]=="vector"{
   let mut storage=Vec::with_capacity(row["capacity"].as_u64().unwrap()as usize);let mut input=Some(value);let g=bank.loan(1,BODY);
   let fuel=bank.wallet().remaining_work();let(turn,_,h)=driver::funded(&mut bank,g,fuel,|effect|{let maximum=effect.maximum_work_units();let turn=<Vec<u64>as NativeSequenceCollection<u64>>::push_step(&mut storage,index,&mut input,maximum,g);if turn.work>0{assert!(effect.performed(turn.work as u64).unwrap())}turn});
   let p=turn.result.as_ref().map(|p|p.progress()).unwrap_or_else(|e|e.retained_progress());bank.post(p,g,h);assert_eq!(turn.result.is_ok(),accepted);assert_eq!(input,if accepted{None}else{Some(value)});driver::retire(storage,&mut bank);continue
  }
  assert_eq!(row["capacity"],2);let source=driver::admit(&mut bank,DslValue::from(serde_json::json!([3,value])));let mut receiver=NativeArrayReceiver::<DslValue,u64,2>::new();
  for _ in 0..10000{if receiver.phase==5&&receiver.index==1{break}driver::none(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(1))).unwrap();}
  assert_eq!(receiver.phase,5);assert_eq!(receiver.live,1);assert_eq!(receiver.index,1);assert_eq!(*receiver.pending,Some(value));let before=(receiver.live,receiver.index,receiver.rows.as_ptr());
  receiver.index=index;
  let placed=driver::receive_traced(&mut receiver,source.borrow(),&mut bank,BODY,Some(1),None,&mut Trace::default());
  if accepted{
   assert_eq!(placed,Ok(None));assert_eq!(receiver.live,2);assert!(receiver.pending.is_none());let actual=unsafe{std::slice::from_raw_parts(receiver.rows.as_ptr().cast::<u64>(),receiver.live)};assert_eq!(serde_json::to_value(actual).unwrap(),serde_json::json!([3,value]));
  }else{
   assert!(placed.is_err(),"{placed:?}");receiver.index=before.1;assert_eq!(*receiver.pending,Some(value));assert_eq!((receiver.live,receiver.index,receiver.rows.as_ptr()),before);assert_eq!(row["occupied"].as_bool().unwrap(),index<receiver.live);
  }
  driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
 }
}

#[derive(crate::RetireOwned)]
#[value(crate="crate")]
struct PartialCollection{prepared:bool,value:Option<u64>}
impl NativeSequenceCollection<u64> for PartialCollection{
 type Storage=Self;type Reservation=();
 fn storage_demand(_:usize)->Result<RetirementDemand,ValueError>{Ok(fixed(size_of::<Option<Self>>()))}
 fn storage_work_demand(_:usize)->Result<u64,ValueError>{Ok(1)}
 fn reserve_demand(_: &Self,_:usize,_:usize,_:&())->Result<Option<RetirementDemand>,ValueError>{Ok(None)}
 fn reserve_work_demand(_: &Self,_:usize,_:usize,_:&(),_:u64)->Result<u64,ValueError>{Ok(0)}
 fn reserve_step(_: &mut Self,_:usize,_:usize,_:&mut(),_:&mut NativeDecodeControl<'_>,_:u64,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
 fn admit_storage(out:&mut Option<Self>,_:usize,frame:&mut NativeReceivingEffect,c:&mut NativeDecodeControl<'_>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{if maximum==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let d=Self::storage_demand(1).unwrap();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let turn=c.receive_allocation(d,frame,false,1,|_,_|{if out.is_some(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}*out=Some(Self{prepared:false,value:None});RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}});RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}
 fn push_copy_bytes(storage:&Self,_:usize)->Result<usize,ValueError>{Ok(if storage.prepared{2*size_of::<Option<u64>>()}else{size_of::<bool>()})}
 fn push_work_demand(storage:&Self,_:usize)->Result<u64,ValueError>{Ok(if storage.prepared{2}else{1})}
 fn push_step(storage:&mut Self,index:usize,input:&mut Option<u64>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{let q=Self::push_work_demand(storage,index)?;let d=fixed(Self::push_copy_bytes(storage,index)?);if maximum<q||!retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if index!=0||storage.value.is_some()||input.is_none(){return Err(invalid())}work=u128::from(q);if !storage.prepared{storage.prepared=true;Ok(RetainedCloneStep::Progress(receipt(d)))}else{storage.value=input.take();Ok(RetainedCloneStep::Complete(receipt(d)))}})();RetainedWorkOutcome{result,work}}
 fn finish(storage:Self)->Self{storage}
}
#[test]fn sequence_parent_preserves_actual_child_progress_and_only_advances_on_complete(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../../📦️storage/📐️.json")).unwrap();let mut bank=stock(&f);let source=driver::admit(&mut bank,DslValue::Null);let mut receiver=NativeSequenceReceiver::<DslValue,u64,PartialCollection>::new();
 driver::none(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(1))).unwrap();
 *receiver.storage=Some(PartialCollection{prepared:false,value:None});*receiver.pending=Some(f["push"]["value"].as_u64().unwrap());receiver.count=1;receiver.phase=3;
 for row in f["partialPush"].as_array().unwrap(){
  let copied=bank.wallet().remaining().maximum_copy_bytes;let mut trace=Trace::default();driver::none(driver::receive_traced(&mut receiver,source.borrow(),&mut bank,BODY,Some(1),None,&mut trace)).unwrap();
  assert_eq!(trace.advance,vec![u128::from(row["childWork"].as_u64().unwrap()+row["parentWork"].as_u64().unwrap())]);
  assert_eq!(copied-bank.wallet().remaining().maximum_copy_bytes,if row["complete"]==true{2*size_of::<Option<u64>>()}else{size_of::<bool>()});
  assert_eq!(receiver.index,row["index"].as_u64().unwrap()as usize);assert_eq!(receiver.phase,row["phase"].as_u64().unwrap()as u8);assert_eq!(*receiver.pending,row["donor"].as_u64());
 }
 assert_eq!(receiver.storage.as_ref().unwrap().value,Some(f["push"]["value"].as_u64().unwrap()));
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
}
