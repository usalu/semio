//! 🧪️ Both real cursor owners reset denied work and close through the same finite Source.
use super::*;
use super::update_work_tests::{funded,grant,native};
use crate::retirement::{RetireOwned,RetirementCursor};
fn finish<T:RetireOwned>(value:T,bank:&mut semio_framework_value_testing_effect_bank::EffectBank,row:&serde_json::Value){
 let mut seed=crate::ordered::test_bank::RetirementFixture::with_bank(value,bank).unwrap_or_else(|_|panic!("actual cursor supports original retirement"));let mut owner=seed.lend();
 for _ in 0..row["stock"]["maximumTurns"].as_u64().unwrap(){
  if owner.terminal_is_empty(){break}let d=owner.next_demand(4096).unwrap();let g=grant(d);let(step,heap)=owner.advance_observed(4096,RetainedCloneGrant{maximum_items:0,..g}).unwrap();assert_eq!(owner.last_turn_work(),row["deniedWork"].as_u64().unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_demand(4096).unwrap(),d);
  let(step,heap)=owner.advance_observed(4096,g).unwrap();assert!(owner.last_turn_work()<=row["maximumTurnWork"].as_u64().unwrap());let p=step.progress();assert!(p.copied_items<=row["maximumMetadataEvents"].as_u64().unwrap()as usize);assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
 }
 assert!(owner.terminal_is_empty());let(step,heap)=owner.advance_observed(4096,Default::default()).unwrap();assert_eq!(step.progress(),Default::default());assert_eq!(owner.last_turn_work(),row["cachedWork"].as_u64().unwrap());assert_eq!(heap,(0,0));owner.finish();assert!(std::collections::BTreeMap::<String,()>::new().is_empty())
}
#[test]
fn actual_ordered_update_lookup_closes_reset_and_forward_defining_work(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 for class in row["cursors"].as_array().unwrap(){
  let mut bank=funded(&row);let mut input=Some(row["key"].as_str().unwrap().to_owned());let mut key=None;let d=SharedOwner::<String>::source_admission_demand().unwrap();native(&mut bank,grant(d),SharedOwner::<String>::source_admission_work_demand(),|effect|SharedOwner::admit_from(&mut input,&mut key,effect)).0.result.unwrap();assert_eq!(serde_json::to_value(key.as_ref().unwrap().get()).unwrap(),row["key"]);
  let base=OrderedMap::<()>::new();
  if class=="update"{let mut cursor=None;native(&mut bank,grant(base.update_constructor_demand()),base.update_constructor_work(),|effect|base.admit_update_cursor(&mut key,&mut None,&mut cursor,effect)).0.result.unwrap();assert!(key.is_none());finish(cursor.take().unwrap(),&mut bank,&row)}
  else{let mut cursor=None;native(&mut bank,grant(base.owned_lookup_cursor_demand()),base.owned_lookup_cursor_work(),|effect|base.admit_owned_lookup_cursor(&mut key,&mut cursor,effect)).0.result.unwrap();assert!(key.is_none());finish(cursor.take().unwrap(),&mut bank,&row)}
  assert!(base.is_empty());drop(base);
 }
}
