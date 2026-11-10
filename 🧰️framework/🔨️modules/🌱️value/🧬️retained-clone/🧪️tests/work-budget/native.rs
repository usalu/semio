//! ⛽️ Actual original clone owners obey zero-work denial, bounded partial work and funded closure.
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_retained_clone::*;
use semio_framework_value_retained_clone::retirement::RetireOwned;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use semio_framework_value_paged_list::PagedList;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use std::fmt::Debug;
#[path="../../🧬️contract/🧪️tests/⏱️effect/🦀️.rs"]
mod effect;

/// 🧾️ Standard collection node backing is the only unpriced allocation (documented in std-collections), so those owners may exceed their receipt.
fn heap_matches<T>(heap:(usize,usize),p:RetainedCloneProgress){if std::any::type_name::<T>().contains("collections::"){assert!(heap.0>=p.retained_capacity_bytes&&heap.1>=p.released_bytes)}else{assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes))}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:4096.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn close<T:RetainedClone>(cursor:&mut T::Cursor){let q=cursor.begin_close_work_demand().unwrap();let d=cursor.begin_close_demand().unwrap();let turn=effect::begin_close_paid(cursor,grant(d));assert_eq!(turn.work,u128::from(q));turn.result.unwrap();for _ in 0..10000{if cursor.terminal_is_empty(){return}let d=cursor.close_demands(4096).unwrap();let q=cursor.close_work_demand(4096).unwrap();let g=grant(d);let(turn,heap)=observe(||effect::close_step(cursor,g));assert!(turn.work<=u128::from(q));let p=turn.result.unwrap().progress();assert!(p.fits(g));heap_matches::<T>(heap,p);}panic!("original partial child did not close")}
fn retire<T:RetireOwned>(value:T){let mut slot=Some(value);let mut close=RetainedCloneClose::default();for _ in 0..10000{if slot.is_none()&&close.is_empty(){return}let d=close.next_owner_demands::<T>(slot.is_some(),&None,4096).unwrap();let g=grant(d);if close.is_empty(){effect::begin_granted(&mut close,&mut slot,g).result.unwrap();}else{effect::step_granted(&mut close,g).result.unwrap();}}panic!("original cloned payload did not close")}
fn run<T:RetainedClone+PartialEq+Debug>(value:T,cut:Option<usize>,project:impl Fn(&T)->Vec<String>,oracle:&[String]){
 let source=RetainedCloneSource::from_owner(value);let original=source.borrow().get()as*const T;let mut cursor=T::retained_clone_cursor();let mut complete=false;
 for index in 0..10000{if cut==Some(index){break}let limit=if index%3==0{1}else{32};let d=cursor.advance_demands(source.borrow(),4096,limit).unwrap();let q=cursor.advance_work_demand(source.borrow(),4096,limit).unwrap();if q>limit{assert_eq!(cursor.advance_work_demand(source.borrow(),4096,q).unwrap(),q,"the minimum quote above a {limit}-unit limit is limit-independent");}let g=grant(d);let(denied,heap)=observe(||effect::advance(&mut cursor,source.borrow(),4096,0,g));assert_eq!(denied.work,0,"turn {index} zero-work denial of {}",std::any::type_name::<T>());assert_eq!(denied.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.advance_demands(source.borrow(),4096,limit).unwrap(),d);let(turn,heap)=observe(||effect::advance(&mut cursor,source.borrow(),4096,limit,g));assert!(turn.work<=u128::from(limit)&&turn.work<=u128::from(q),"turn {index} work {} limit {limit} quote {q} of {}",turn.work,std::any::type_name::<T>());let step=turn.result.unwrap();let p=step.progress();assert!(p.fits(g));heap_matches::<T>(heap,p);assert_eq!(source.borrow().get()as*const T,original);if matches!(step,RetainedCloneStep::Complete(_)){complete=true;break}}
 if !complete&&cursor.output_ready(){let g=grant(cursor.advance_demands(source.borrow(),4096,32).unwrap());let turn=effect::advance(&mut cursor,source.borrow(),4096,32,g);assert_eq!(turn.work,0,"an output-ready cut cursor completes without work");assert!(matches!(turn.result.unwrap(),RetainedCloneStep::Complete(_)),"an output-ready cut cursor completes on its next turn");complete=true}
 if complete{let q=cursor.take_work_demand().unwrap();let handoff=grant(cursor.take_demand().unwrap());let turn=effect::take(&mut cursor,handoff);assert_eq!(turn.work,u128::from(q));let(output,p)=turn.result.unwrap().unwrap();assert!(p.fits(grant(cursor.take_demand().unwrap())));assert_eq!(&output,source.borrow().get());let actual=project(&output);assert_eq!(actual,oracle);let text=serde_json::to_string(&actual).unwrap();assert_eq!(serde_json::from_str::<Vec<String>>(&text).unwrap(),oracle);retire(output)}else{assert!(cut.is_some());assert!(!cursor.output_ready(),"cut {cut:?} of {} reports output ready without completing; next quote {:?}",std::any::type_name::<T>(),cursor.advance_demands(source.borrow(),4096,32))}close::<T>(&mut cursor);
}
#[test]
fn original_collection_work_limits_preserve_partial_children_and_third_party_values(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/work-budget/🔣️.json")).unwrap();
 for sample in corpus["samples"].as_array().unwrap(){let oracle:Vec<String>=serde_json::from_value(sample.clone()).unwrap();for cut in corpus["cancellationTurns"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain([None]){
  let value=DslValue::Array(oracle.iter().map(|v|DslValue::String(v.clone())).collect());run(value,cut,|v|{let DslValue::Array(values)=v else{panic!("array original changed")};values.iter().map(|v|{let DslValue::String(v)=v else{panic!("text original changed")};v.clone()}).collect()},&oracle);
  let mut value=PagedList::<String,32>::default();for s in &oracle{while !value.has_reserved_slot(){effect::run(effect::STOCK,|e|value.reserve_one(e)).result.unwrap();}value.push_reserved(s.clone()).unwrap();}run(value,cut,|v|(0..v.len()).map(|i|v.get(i).unwrap().clone()).collect(),&oracle);
 }}
}
