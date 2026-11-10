use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::field::harness::{Harness,Recorded,observe,refuse_one};
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::{retained_clone::RetainedCloneSource,retirement::{RetireOwned,controlled::ControlledRetirement}};
#[derive(semio_framework_value::RetireOwned,Debug,PartialEq)]struct Added<T:RetireOwned>{index:usize,value:T}
struct Mapper;
impl RetainedIndexedMap<String,Added<String>> for Mapper{const WORK:u64=2;fn map(index:usize,value:String)->Added<String>{Added{index,value}}}
type Replacement=RetainedIndexedReplacement<String,Added<String>,Mapper>;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(4096),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn strings(v:&serde_json::Value)->Vec<String>{v.as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_owned()).collect()}
fn step(h:&mut Harness,p:&mut Replacement,a:&RetainedCloneSource<Vec<String>>,o:&RetainedCloneSource<Vec<String>>,body:usize,work:u64,g:RetainedCloneGrant)->Recorded{h.turn(g,work,&mut |e,r,rec|if rec{p.recover_advance_into(a.borrow(),o.borrow(),body,work,e,r)}else{p.advance_into(a.borrow(),o.borrow(),body,work,e,r)})}
fn close_turn(h:&mut Harness,p:&mut Replacement,g:RetainedCloneGrant,work:u64)->Recorded{h.turn(g,work,&mut |e,r,rec|if rec{p.recover_close_into(4096,e,r)}else{p.close_into(4096,e,r)})}
fn close(h:&mut Harness,p:&mut Replacement){p.begin_close();let mut turns=0;while !p.terminal_is_empty(){let g=grant(p.close_demand(4096).unwrap());let quoted=p.close_work_demand(g.maximum_copy_bytes).unwrap();assert_eq!(close_turn(h,p,RetainedCloneGrant{maximum_items:0,..g},quoted).result.unwrap().progress(),Default::default());let turn=close_turn(h,p,g,quoted);assert!(turn.work>0&&turn.work<=u128::from(quoted));assert!(turn.result.unwrap().progress().fits(g));turns+=1;assert!(turns<100000);}}
fn exercise(row:&serde_json::Value,cut:Option<usize>,body:usize,work:u64){
 let mut h=Harness::new();let mut a=h.source(strings(&row["authored"]));let mut o=h.source(strings(&row["original"]));let mut foreign=h.source(strings(&row["authored"]));
 let pointers=(a.borrow().get().iter().map(|v|v.as_ptr()).collect::<Vec<_>>(),o.borrow().get().iter().map(|v|v.as_ptr()).collect::<Vec<_>>());
 let(mut p,heap)=observe(Replacement::new);assert_eq!(heap,(0,0));let mut turns=0;
 while !p.ready()&&cut.is_none_or(|cut|turns<cut){
  let g=grant(p.demand(a.borrow(),o.borrow(),body,work).unwrap());
  let denied=step(&mut h,&mut p,&a,&o,body,work,RetainedCloneGrant{maximum_items:0,..g});assert_eq!(denied.result.unwrap().progress(),Default::default());assert_eq!(denied.heap,(0,0));assert_eq!(denied.marker,None);
  let moved=if p.phase==5{p.source.as_ref().and_then(|s|s.front().map(|v|v.as_ptr()))}else{None};let before=p.added.as_ref().map_or(0,Vec::len);
  let quoted=p.work_demand(a.borrow(),o.borrow(),body,work).unwrap();
  let zero=step(&mut h,&mut p,&a,&o,body,0,g);assert_eq!(zero.work,0);assert_eq!(zero.result.unwrap().progress(),Default::default());
  let turn=step(&mut h,&mut p,&a,&o,body,work,g);assert!(turn.work>0&&turn.work<=u128::from(quoted));let receipt=turn.result.unwrap().progress();if let Some(bytes)=turn.marker{assert_eq!(bytes,receipt.retained_capacity_bytes);}assert!(receipt.fits(g));assert_eq!(turn.heap,(receipt.retained_capacity_bytes,receipt.released_bytes),"indexed actual allocation and release receipt");
  if let Some(pointer)=moved{if p.added.as_ref().map_or(0,Vec::len)>before{assert_eq!(p.added.as_ref().unwrap().last().unwrap().value.as_ptr(),pointer);}}
  turns+=1;assert!(turns<10000);
 }
 if turns>0{assert!(p.demand(foreign.borrow(),o.borrow(),body,work).is_err());}
 if cut.is_none(){
  let g=grant(p.take_demand().unwrap());let mut out=None;let(taken,_,_)=h.native(g,1,|e|{let mut slot=None;drop(p.take_into(&mut out,e,OriginalEffectReply::borrow(&mut slot).unwrap()).unwrap());slot.take().unwrap().unwrap()});let RetainedCloneStep::Complete(receipt)=taken else{panic!("indexed result refused")};assert!(receipt.fits(g));
  assert_eq!(out.is_none(),row["expected"].is_null());if let Some(out)=out{assert_eq!(out.removed,(0..o.borrow().get().len()).collect::<Vec<_>>());assert_eq!(out.added.iter().map(|v|(v.index,&v.value)).collect::<Vec<_>>(),a.borrow().get().iter().enumerate().collect::<Vec<_>>());h.retire(out);}
 }
 close(&mut h,&mut p);assert_eq!(observe(||drop(p)).1,(0,0));
 assert_eq!((a.borrow().get().iter().map(|v|v.as_ptr()).collect::<Vec<_>>(),o.borrow().get().iter().map(|v|v.as_ptr()).collect::<Vec<_>>()),pointers);
 h.close_source(a);h.close_source(o);h.close_source(foreign);
}
#[test]fn indexed_replacement_original_rows_and_moved_payloads_match_neutral_cases(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪡️field/🟰️change/📋️indexed/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in f["cases"].as_array().unwrap(){exercise(row,None,f["bodyBytes"].as_u64().unwrap()as usize,f["workUnits"].as_u64().unwrap());}}
#[test]fn indexed_replacement_partial_original_deque_and_output_children_close(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪡️field/🟰️change/📋️indexed/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in f["cases"].as_array().unwrap(){for cut in f["cancelCuts"].as_array().unwrap(){let cut=cut.as_u64().unwrap()as usize;exercise(row,Some(cut),f["bodyBytes"].as_u64().unwrap()as usize,f["workUnits"].as_u64().unwrap());}}}
#[test]fn indexed_replacement_records_a_refused_backing_allocation_and_closes_originals(){
 let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪡️field/🟰️change/📋️indexed/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let row=f["cases"].as_array().unwrap().iter().find(|row|row["id"]=="set").unwrap();let body=f["bodyBytes"].as_u64().unwrap()as usize;let work=f["workUnits"].as_u64().unwrap();
 let mut h=Harness::new();let mut a=h.source(strings(&row["authored"]));let mut o=h.source(strings(&row["original"]));let mut p=Replacement::new();
 while p.phase!=3{let g=grant(p.demand(a.borrow(),o.borrow(),body,work).unwrap());step(&mut h,&mut p,&a,&o,body,work,g).result.unwrap();}
 let g=grant(p.demand(a.borrow(),o.borrow(),body,work).unwrap());let(turn,refused)=refuse_one(||step(&mut h,&mut p,&a,&o,body,work,g));
 assert_eq!(refused,1);assert_eq!(turn.marker,Some(0));h.retire(turn.result.err().expect("refused allocation is recorded"));assert_eq!(p.phase,3);
 close(&mut h,&mut p);h.close_source(a);h.close_source(o);
}
