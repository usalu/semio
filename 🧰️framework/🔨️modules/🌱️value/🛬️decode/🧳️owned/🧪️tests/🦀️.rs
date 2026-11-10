use semio_framework_test_fixtures::fixture_text;
use crate::*;
use crate::{DslValue,shared_issuance::SealedShared,retained_clone::RetainedCloneSource};
use crate::driver::{self,Trace};
use crate::owned::{NativeDefault,NativeReceiver,tree};
use semio_framework_value_testing_admission::EffectBank;
const BODY:usize=4096;
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Row{id:SealedShared<String>,name:SealedShared<String>,value:i64}
fn input()->DslValue{let json:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📐️.json")).unwrap();DslValue::Object(vec![("id".into(),DslValue::String(json["before"]["id"].as_str().unwrap().into())),("name".into(),DslValue::String(json["before"]["name"].as_str().unwrap().into())),("value".into(),DslValue::Number(crate::Number::Int(json["before"]["value"].as_i64().unwrap())))])}
fn bank()->EffectBank{let f:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📐️.json")).unwrap();EffectBank::new(semio_framework_value_testing_admission::Wallet::from_profile(&f["stock"]))}
fn finish(mut receiver:<Row as NativeReceive<DslValue>>::Receiver,source:RetainedCloneSource<DslValue>,bank:&mut EffectBank){driver::close(&mut receiver,bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,bank);}
#[test]fn native_receive_original_row_has_funded_fields_and_every_cancellation_frontier(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📐️.json")).unwrap();let mut observed=0;
 for cut in 0..2048u64{
  let mut bank=bank();let original=driver::admit(&mut bank,input());let foreign=driver::admit(&mut bank,input());let mut receiver=Row::receiver();let mut turns=0;
  while turns<cut&&!receiver.output_ready(){let d=receiver.demands(original.borrow(),BODY,1<<20).unwrap();assert!(d.copy_bytes<=BODY);if turns>0{assert!(receiver.demands(foreign.borrow(),BODY,1<<20).is_err())}driver::none(driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,Some(1))).unwrap();turns+=1;}
  if receiver.output_ready(){observed=turns;let row=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");assert_eq!(serde_json::json!({"id":row.id.as_str(),"name":row.name.as_str(),"value":row.value}),law["expected"]);driver::retire(row,&mut bank);}
  driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(original,&mut bank);driver::close_source(foreign,&mut bank);if observed>0&&cut>observed{break}
 }assert!(observed>0);
}
#[test]fn native_receive_constructor_and_output_denials_preserve_exact_owner(){
 let mut bank=bank();let original=driver::admit(&mut bank,input());let mut slot=tree::NativeReceiveSlot::<DslValue,Row>::default();semio_framework_value_testing_admission::session_open(&bank);let original_pointer=original.borrow().get()as*const DslValue;
 let d=slot.demands(original.borrow(),BODY,1<<20).unwrap();let g=bank.loan(1,BODY);let fuel=bank.wallet().remaining_work();
 for denied in semio_framework_value_testing_admission::denials(g,d){let(poll,heap)=crate::observe_retirement_allocations(||semio_framework_value_testing_admission::session_nested(&bank,denied,|control,witness|slot.advance(original.borrow(),control,witness,BODY,fuel,denied)));assert_eq!(heap,(0,0));match poll{RetainedWorkPoll::Complete(turn)=>{assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default())},RetainedWorkPoll::Pending{..}=>panic!("denied slot turn parked")}assert_eq!(slot.demands(original.borrow(),BODY,1<<20).unwrap(),d);assert_eq!(original.borrow().get()as*const DslValue,original_pointer);}
 assert!(slot.terminal_is_empty());drop(slot);
 let mut receiver=Row::receiver();while !receiver.output_ready(){driver::none(driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,Some(1))).unwrap()}
 let row=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");driver::retire(row,&mut bank);finish(receiver,original,&mut bank);
}
#[test]fn native_receive_domain_and_original_callback_refusals_keep_all_partial_custody(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📐️.json")).unwrap();let entries=law["refusedEntries"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_owned(),if let Some(value)=pair[1].as_str(){DslValue::String(value.to_owned())}else{DslValue::Number(crate::Number::Int(pair[1].as_i64().unwrap()))})).collect();
 let mut bank=bank();let original=driver::admit(&mut bank,DslValue::Object(entries));let mut receiver=Row::receiver();let mut refused=false;
 for _ in 0..4096{if driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,Some(1)).is_err(){refused=true;break}}assert!(refused);assert!(!receiver.output_ready());let DslValue::Object(entries)=original.borrow().get()else{panic!("original object changed")};assert_eq!(entries[0].0,entries[1].0);finish(receiver,original,&mut bank);
 for turns in 1..24u64{
  let original=driver::admit(&mut bank,input());let mut receiver=Row::receiver();let admit=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));let mut trace=Trace{admit:Some(admit.clone()),..Default::default()};
  for _ in 0..turns{if receiver.output_ready(){break}driver::none(driver::receive_traced(&mut receiver,original.borrow(),&mut bank,BODY,Some(1),None,&mut trace)).unwrap();}
  if !receiver.output_ready(){admit.store(false,std::sync::atomic::Ordering::SeqCst);let before=trace.advance.len();let refused=driver::receive_traced(&mut receiver,original.borrow(),&mut bank,BODY,Some(1),None,&mut trace);if refused.is_err(){assert_eq!(trace.advance.len(),before+1);assert_eq!(trace.advance.last().copied(),Some(0));assert!(!receiver.output_ready());}admit.store(true,std::sync::atomic::Ordering::SeqCst);}
  finish(receiver,original,&mut bank);
 }
}
fn default_case<T:NativeDefault+PartialEq+std::fmt::Debug>(expected:T){
 let mut bank=bank();let source=driver::admit(&mut bank,DslValue::Null);let(cursor,heap)=crate::observe_retirement_allocations(T::default_cursor);assert_eq!(heap,(0,0));let mut cursor=crate::default_receiver::DefaultReceiver(cursor);
 for _ in 0..10000{if cursor.output_ready(){break}let d=NativeReceiver::<DslValue,T>::demands(&cursor,source.borrow(),BODY,1<<20).unwrap();let before=bank.wallet().remaining().maximum_copy_bytes;driver::none(driver::receive_cut(&mut cursor,source.borrow(),&mut bank,BODY,Some(1))).unwrap();assert_eq!(before-bank.wallet().remaining().maximum_copy_bytes,d.copy_bytes);}
 assert!(cursor.output_ready());assert_eq!(NativeReceiver::<DslValue,T>::take_demand(&cursor).unwrap().copy_bytes,2*std::mem::size_of::<Option<T>>());
 let value:T=driver::receive_cut(&mut cursor,source.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");assert_eq!(value,expected);driver::retire(value,&mut bank);
 driver::close::<DslValue,T,_>(&mut cursor,&mut bank,BODY);assert!(NativeReceiver::<DslValue,T>::terminal_is_empty(&cursor));drop(cursor);driver::close_source(source,&mut bank);
}
#[test]fn fixed_array_native_default_repeats_the_scalar_default_only_under_its_own_grant(){default_case([0.0f64;3]);default_case::<[u8;0]>([]);}
#[test]fn native_receive_slot_send_rests_on_the_receiver_contract(){fn is_send<T:Send>(){}fn is_static<T:'static>(){}fn receiver_contract<I:?Sized,T:NativeReceive<I>>(){is_send::<T::Receiver>();is_static::<T::Receiver>();is_send::<T>();}fn slot_is_send<I:super::tree::NativeValueSource+?Sized,T:NativeReceive<I>>(){is_send::<super::tree::NativeReceiveSlot<I,T>>();}let _=(receiver_contract::<DslValue,Row>,slot_is_send::<DslValue,Row>);}
#[test]fn native_receive_pure_row_header_allocates_no_backing(){let(receiver,heap)=crate::observe_retirement_allocations(Row::receiver);assert_eq!(heap,(0,0));let mut receiver=receiver;let mut bank=bank();let before=bank.wallet().remaining();driver::close::<DslValue,Row,_>(&mut receiver,&mut bank,BODY);let after=bank.wallet().remaining();assert_eq!((before.maximum_capacity_bytes,before.maximum_release_bytes),(after.maximum_capacity_bytes,after.maximum_release_bytes));assert!(receiver.terminal_is_empty());}
