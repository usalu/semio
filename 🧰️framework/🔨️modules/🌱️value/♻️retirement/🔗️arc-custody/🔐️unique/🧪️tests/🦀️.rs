use super::*;
use crate::allocation_testing::observe_retirement_allocations as observe;
use semio_framework_test_fixtures::fixture_text;
#[test]
fn shared_unique_original_arc_retires_despite_weak_aliases_and_credits_actual_frame(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🔗️arc-custody/🔐️unique/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for copy in fixture["copyGrants"].as_array().unwrap(){
  let value=row["value"].as_u64().unwrap();let (source,birth)=observe(||Arc::new(value));assert_eq!(birth,(arc_bytes::<u64>(),0));let pointer=Arc::as_ptr(&source);
  let mut weak:Vec<_>=(0..row["weakAliases"].as_u64().unwrap()).map(|_|Arc::downgrade(&source)).collect();
  let (mut cursor,heap)=observe(||SharedControlledRetirement::new(source));assert_eq!(heap,(0,0));
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:0,maximum_release_bytes:arc_bytes::<u64>(),maximum_depth:1};
  if weak.is_empty(){let (step,heap)=crate::retirement::effect_fixture::observed(RetainedCloneGrant{maximum_release_bytes:0,..grant},|effect|crate::retirement::effect_fixture::settled(cursor.step(effect)).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(cursor.last_turn_work(),0);assert_eq!(heap,(0,0));assert_eq!(Arc::as_ptr(cursor.source.as_ref().unwrap()),pointer);}
  let frame=if weak.is_empty(){arc_bytes::<u64>()}else{0};
  let quote=cursor.next_computational_work(grant.maximum_copy_bytes).unwrap();let(step,heap)=crate::retirement::effect_fixture::observed(grant,|effect|crate::retirement::effect_fixture::settled(cursor.step(effect)).result.unwrap());assert_eq!(cursor.last_turn_work(),u128::from(quote));assert_eq!(cursor.last_turn_work(),1);assert_eq!(step.progress().released_bytes,frame);assert_eq!(heap,(0,frame));assert!(weak.iter().all(|alias|alias.upgrade().is_none()));
  let(_,heap)=observe(||weak.clear());assert_eq!(heap,(0,arc_bytes::<u64>()-frame));assert!(cursor.source.is_none());assert_eq!(cursor.owned.as_ref().unwrap().original(),Some(&value));
  let mut births=0;let mut released=0;
  for _ in 0..64{if cursor.terminal_is_empty(){break;}let copy=grant.maximum_copy_bytes;let grant=crate::retirement::effect_fixture::quoted(copy,|body|cursor.next_demand(body).unwrap());let quote=cursor.next_computational_work(copy).unwrap();let(step,heap)=crate::retirement::effect_fixture::observed(grant,|effect|crate::retirement::effect_fixture::settled(cursor.step(effect)).result.unwrap());assert!(cursor.last_turn_work()<=u128::from(quote));let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));births+=heap.0;released+=heap.1;}
  assert!(cursor.terminal_is_empty());assert_eq!(births,released);let(step,heap)=crate::retirement::effect_fixture::observed(Default::default(),|effect|crate::retirement::effect_fixture::settled(cursor.step(effect)).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(cursor.last_turn_work(),0);assert_eq!(heap,(0,0));let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));
 }}
}
