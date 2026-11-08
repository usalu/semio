//! 🎟️ Native AVL primitives and all deferred retirement use one real finite Source bank.
use super::*;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use crate::RetirementDemand;
pub(super) fn funded(row:&serde_json::Value)->EffectBank{EffectBank::new(Wallet::from_profile(&row["stock"]))}
pub(super) fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub(super) fn native(bank:&mut EffectBank,g:RetainedCloneGrant,w:u64,f:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome)->(crate::RetainedWorkOutcome,(usize,usize),Option<usize>){
 let((poll,marker),heap)=crate::allocation_testing::observe_retirement_allocations(||bank.run(g,w,|effect|crate::RetainedWorkPoll::Complete(f(effect))));let crate::RetainedWorkPoll::Complete(turn)=poll else{panic!("closed AVL primitive became pending")};let p=turn.result.as_ref().map_or_else(|e|e.retained_progress(),|s|s.progress());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(g));if let Some(marker)=marker{assert_eq!(marker,heap.0)}else{assert_eq!(heap.0,0)}assert_ne!(bank.take_admitted(),Some(false));bank.post(p,g,heap);(turn,heap,marker)
}
pub(super) fn close<T:crate::retirement::RetireOwned>(value:T,bank:&mut EffectBank){
 let mut seed=crate::ordered::test_bank::RetirementFixture::with_bank(value,bank).unwrap_or_else(|_|panic!("actual original owner supports controlled retirement"));let mut owner=seed.lend();
 for _ in 0..100000{if owner.terminal_is_empty(){owner.finish();return}let d=owner.next_demand(4096).unwrap();let(step,heap)=owner.advance_observed(4096,grant(d)).unwrap();let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes))}
 panic!("actual ordered cleanup exhausted original finite turn bound")
}
fn admit<T>(value:T,bank:&mut EffectBank)->SharedOwner<T>{let mut input=Some(value);let mut output=None;let d=SharedOwner::<T>::source_admission_demand().unwrap();let(turn,_,_)=native(bank,grant(d),SharedOwner::<T>::source_admission_work_demand(),|effect|SharedOwner::admit_from(&mut input,&mut output,effect));turn.result.unwrap();assert!(input.is_none());output.unwrap()}
#[test]
fn actual_ordered_update_stages_each_fallible_birth_and_preserves_every_rotation_cancel_prefix(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 for case in row["cases"].as_array().unwrap(){for cut in row["cancelPrefixes"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap())).chain([None]){
  let mut bank=funded(&row);let mut map=OrderedMap::<u64>::new();let mut oracle=std::collections::BTreeMap::new();
  for(member,pair)in case.as_array().unwrap().iter().enumerate(){
   let selected_cut=if member+1==case.as_array().unwrap().len(){cut}else{None};let name=pair[0].as_str().unwrap().to_owned();let value=pair[1].as_u64().unwrap();let mut key=Some(admit(name.clone(),&mut bank));let mut shared=Some(admit(value,&mut bank));let mut cursor=None;
   let d=map.update_constructor_demand();let(turn,_,_)=native(&mut bank,grant(d),map.update_constructor_work(),|effect|map.admit_update_cursor(&mut key,&mut shared,&mut cursor,effect));assert_eq!(turn.work,if map.is_empty(){row["constructor"]["emptyWork"].as_u64().unwrap()}else{row["constructor"]["nonemptyWork"].as_u64().unwrap()}as u128);turn.result.unwrap();let mut cursor=cursor.unwrap();let mut injected=false;
   for at in 0..row["maximumTurns"].as_u64().unwrap(){
    if selected_cut==Some(at)||cursor.is_complete(){break}let d=cursor.native_demands(4096).unwrap();let work=cursor.native_work_demand(4096).unwrap();let g=grant(d);
    for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}
     let(turn,heap,_)=native(&mut bank,denied,work,|effect|cursor.advance_native(4096,effect));assert_eq!(turn.work,row["deniedWork"].as_u64().unwrap()as u128);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.native_demands(4096).unwrap(),d);
    }
    if !injected&&d.capacity_bytes>0{let mut attempts=0;let(turn,heap,marker)=native(&mut bank,g,work,|effect|{let(turn,count)=crate::allocation_testing::refuse_one_allocation_on_this_thread(||cursor.advance_native(4096,effect));attempts=count;turn});assert_eq!(attempts,1);assert_eq!(heap,(0,0));assert_eq!(marker,Some(0));assert_eq!(turn.work,row["nullWork"].as_u64().unwrap()as u128);let error=turn.result.unwrap_err();assert_eq!(cursor.native_demands(4096).unwrap(),d);close(error,&mut bank);injected=true;}
    let(turn,_,_)=native(&mut bank,g,work,|effect|cursor.advance_native(4096,effect));assert!(turn.work<=row["maximumTurnWork"].as_u64().unwrap()as u128);let progress=turn.result.unwrap().progress();assert!(progress.copied_items<=row["maximumMetadataEvents"].as_u64().unwrap()as usize);
   }
   if cursor.is_complete(){let mut output=None;let d=cursor.result_transfer_demand();native(&mut bank,grant(d),1,|effect|cursor.take_result_into(&mut output,effect)).0.result.unwrap();let previous=std::mem::replace(&mut map,output.unwrap());close(previous,&mut bank);oracle.insert(name,value);let actual:std::collections::BTreeMap<String,u64>=map.iter().map(|(key,value)|(key.to_owned(),*value)).collect();assert_eq!(serde_json::to_value(actual).unwrap(),serde_json::to_value(&oracle).unwrap());}
   close(cursor,&mut bank)
  }
  close(map,&mut bank);assert!(bank.wallet().remaining_work()>0)
 }}
}
#[test]
fn actual_controlled_insert_keeps_its_declared_operation_after_value_moves_to_entry(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let case:serde_json::Value=serde_json::from_str(include_str!("../🎟️controlled/📐️.json")).unwrap();let mut bank=funded(&law);let name=case["input"][0][0].as_str().unwrap();let number=case["input"][0][1].as_u64().unwrap();let mut key=Some(admit(name.to_owned(),&mut bank));let mut value=Some(admit(number,&mut bank));let base=OrderedMap::<u64>::new();let mut cursor=None;native(&mut bank,grant(base.update_constructor_demand()),base.update_constructor_work(),|effect|base.admit_update_cursor(&mut key,&mut value,&mut cursor,effect)).0.result.unwrap();let mut cursor=cursor.unwrap();let mut moved=false;
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if cursor.is_complete(){break}let d=cursor.native_demands(4096).unwrap();let w=cursor.native_work_demand(4096).unwrap();native(&mut bank,grant(d),w,|effect|cursor.advance_native(4096,effect)).0.result.unwrap();moved|=cursor.state.value.is_none();}
 assert!(moved);assert!(cursor.is_complete());let mut output=None;native(&mut bank,grant(cursor.result_transfer_demand()),1,|effect|cursor.take_result_into(&mut output,effect)).0.result.unwrap();assert_eq!(serde_json::to_value(output.as_ref().unwrap().get(name).copied()).unwrap(),case["output"]);close(cursor,&mut bank);close(base,&mut bank);close(output.take().unwrap(),&mut bank)
}
