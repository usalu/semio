use semio_framework_test_fixtures::fixture_text;
use super::*;
use semio_framework_value::{retirement::RetireOwned,retained_clone::RetainedCloneSource};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(operation:impl FnOnce()->T)->(T,(usize,usize)){let(result,born,released)=crate::test_allocation::observe_backing(operation);(result,(born,released))}
fn source<T:RetireOwned+Sync>(value:T)->RetainedCloneSource<T>{match RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}){Ok((source,_))=>source,Err((error,_,()))=>panic!("original lookup source refused:{error}")}}
fn close_source<T:RetireOwned+Sync>(source:&mut RetainedCloneSource<T>){while !source.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});assert!(source.close_step(quote).unwrap().progress().fits(quote));}}
fn close(lookup:&mut RetainedKeyLookup){lookup.begin_close();while !lookup.terminal_is_empty(){let quote=grant(lookup.close_demand(4096).unwrap());let(step,heap)=observe(||lookup.close_step(quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}

/// 🧫️ Unique, absent, duplicate and empty keys agree with portable neutral query rows.
#[test]
fn retained_key_lookup_matches_neutral_queries_without_key_or_row_clone(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🔎️key/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let mut rows=source(case["keys"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect::<Vec<_>>());
  let mut key=source(case["needle"].as_str().unwrap().to_owned());let mut lookup=RetainedKeyLookup::new();let mut turns=0;
  while lookup.result().is_none(){
   turns+=1;assert!(turns<1000);
   let(demand,heap)=observe(||lookup.demand(rows.borrow(),key.borrow(),String::as_str,corpus["chunkBytes"].as_u64().unwrap()as usize).unwrap());assert_eq!(heap,(0,0));let quote=grant(demand);
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..quote}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..quote}),Some(RetainedCloneGrant{maximum_depth:0,..quote})].into_iter().flatten(){
    let(before,offset)=(lookup.index,lookup.offset);let(step,heap)=observe(||lookup.advance(rows.borrow(),key.borrow(),String::as_str,denied).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());assert_eq!((lookup.index,lookup.offset),(before,offset));
   }
   let(step,heap)=observe(||lookup.advance(rows.borrow(),key.borrow(),String::as_str,quote).unwrap());assert_eq!(heap,(0,0));assert!(step.progress().fits(quote));
  }
  let expected=case["expectedIndices"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize).collect::<Vec<_>>();
  let actual=match lookup.result().unwrap(){RetainedKeyMatch::Missing=>vec![],RetainedKeyMatch::Unique(index)=>vec![index],RetainedKeyMatch::Duplicate(first,second)=>vec![first,second]};assert_eq!(actual,expected);
  close(&mut lookup);let(_,heap)=observe(||drop(lookup));assert_eq!(heap,(0,0));close_source(&mut key);close_source(&mut rows);
  eprintln!("[DEBUG] retained key {:?}: original query indices={actual:?}, allocation-free bounded comparison turns={turns}",case["id"]);
 }
}

/// 🛑️ Every retained lookup frontier closes the exact list and needle source aliases.
#[test]
fn retained_key_lookup_cancellation_settles_all_original_aliases(){
 for cancel in 0..20{
  let mut rows=source(vec!["long-🧱-node".to_owned(),"different".to_owned()]);let mut key=source("long-🧱-node".to_owned());let mut lookup=RetainedKeyLookup::new();
  for _ in 0..cancel{if lookup.result().is_some(){break;}let quote=grant(lookup.demand(rows.borrow(),key.borrow(),String::as_str,2).unwrap());lookup.advance(rows.borrow(),key.borrow(),String::as_str,quote).unwrap();}
  close(&mut lookup);close_source(&mut key);close_source(&mut rows);
 }
}
