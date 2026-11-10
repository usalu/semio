use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::field::harness::{Harness,Recorded,observe,refuse_one};
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::{retained_clone::RetainedCloneSource,retirement::{RetireOwned,controlled::ControlledRetirement}};
#[derive(semio_framework_value::RetireOwned,Clone,Debug,PartialEq)]struct Row{title:Option<String>,count:i64,body:Vec<u8>}
#[derive(semio_framework_value::RetireOwned,Default,Clone,Debug,PartialEq)]struct Patch{title:Option<Option<String>>,count:Option<i64>,body:Option<Vec<u8>>}
std::thread_local!{static PROJECTIONS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
fn count_projection(){PROJECTIONS.with(|n|n.set(n.get()+1));}
struct Title;struct Count;struct Body;
impl RetainedRecordFieldProjection<Row,Patch,Option<String>> for Title{const PROJECT_WORK:u64=2;const INSTALL_WORK:u64=1;fn project(r:RetainedCloneRef<'_,Row>)->RetainedCloneRef<'_,Option<String>>{count_projection();r.project(0,|r|&r.title)}fn install(p:&mut Patch,v:Option<Option<String>>){p.title=v}}
impl RetainedRecordFieldProjection<Row,Patch,i64> for Count{const PROJECT_WORK:u64=2;const INSTALL_WORK:u64=1;fn project(r:RetainedCloneRef<'_,Row>)->RetainedCloneRef<'_,i64>{count_projection();r.project(1,|r|&r.count)}fn install(p:&mut Patch,v:Option<i64>){p.count=v}}
impl RetainedRecordFieldProjection<Row,Patch,Vec<u8>> for Body{const PROJECT_WORK:u64=2;const INSTALL_WORK:u64=1;fn project(r:RetainedCloneRef<'_,Row>)->RetainedCloneRef<'_,Vec<u8>>{count_projection();r.project(2,|r|&r.body)}fn install(p:&mut Patch,v:Option<Vec<u8>>){p.body=v}}
impl RetainedRecordChange<Row> for Patch{
 const FIELD_COUNT:usize=3;
 const EMPTY_WORK:u64=3;
 fn field_work(index:usize)->Result<u64,ValueError>{let work=match index{0=>boxed_record_field_work::<Row,Patch,Option<String>,Title>()?,1=>boxed_record_field_work::<Row,Patch,i64,Count>()?,2=>boxed_record_field_work::<Row,Patch,Vec<u8>,Body>()?,_=>return Err(invalid("test record field out of range"))};work.checked_add(1).ok_or_else(||invalid("test birth work overflow"))}
 fn empty_result()->Self{Self::default()}
 fn field_bytes(index:usize)->Result<usize,ValueError>{Ok(match index{0=>size_of::<RetainedRecordFieldChange<Row,Self,Option<String>,Title>>(),1=>size_of::<RetainedRecordFieldChange<Row,Self,i64,Count>>(),2=>size_of::<RetainedRecordFieldChange<Row,Self,Vec<u8>,Body>>(),_=>return Err(invalid("test record field out of range"))})}
 fn field(index:usize,work:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<Option<(Box<dyn RetainedRecordChangeField<Row,Self>>,RetainedCloneProgress)>>{if work==0{return RetainedWorkOutcome{result:Ok(None),work:0}}let turn=match index{0=>boxed_record_field::<Row,Self,Option<String>,Title>(work.saturating_sub(1),grant,actual),1=>boxed_record_field::<Row,Self,i64,Count>(work.saturating_sub(1),grant,actual),2=>boxed_record_field::<Row,Self,Vec<u8>,Body>(work.saturating_sub(1),grant,actual),_=>RetainedWorkOutcome{result:Err(invalid("test record field out of range")),work:0}};RetainedWorkOutcome{result:turn.result,work:turn.work+1}}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(4096),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn row(v:&serde_json::Value)->Row{Row{title:v["title"].as_str().map(str::to_owned),count:v["count"].as_i64().unwrap(),body:v["body"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect()}}
fn patch(v:&serde_json::Value)->Patch{Patch{title:v.get("title").map(|v|v.as_str().map(str::to_owned)),count:v.get("count").map(|v|v.as_i64().unwrap()),body:v.get("body").map(|v|v.as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect())}}
fn step(h:&mut Harness,c:&mut RetainedRecordChangeOwner<Row,Patch>,a:&RetainedCloneSource<Row>,o:&RetainedCloneSource<Row>,body:usize,work:u64,g:RetainedCloneGrant)->Recorded{h.turn(g,work,&mut |e,r,rec|if rec{c.recover_advance_into(a.borrow(),o.borrow(),body,work,e,r)}else{c.advance_into(a.borrow(),o.borrow(),body,work,e,r)})}
fn close_turn(h:&mut Harness,c:&mut RetainedRecordChangeOwner<Row,Patch>,g:RetainedCloneGrant,work:u64)->Recorded{h.turn(g,work,&mut |e,r,rec|if rec{c.recover_close_into(4096,e,r)}else{c.close_into(4096,e,r)})}
fn close(h:&mut Harness,c:&mut RetainedRecordChangeOwner<Row,Patch>){c.begin_close();let mut turns=0;while !c.terminal_is_empty(){let g=grant(c.close_demand(4096).unwrap());let quoted=c.close_work_demand(g.maximum_copy_bytes).unwrap();assert_eq!(close_turn(h,c,RetainedCloneGrant{maximum_items:0,..g},quoted).result.unwrap().progress(),Default::default());let turn=close_turn(h,c,g,quoted);assert!(turn.work>0&&turn.work<=u128::from(quoted));assert!(turn.result.unwrap().progress().fits(g));turns+=1;assert!(turns<100000);}}
fn exercise(v:&serde_json::Value,cut:Option<usize>,body:usize,work:u64){
 let mut h=Harness::new();let mut a=h.source(row(&v["authored"]));let mut o=h.source(row(&v["original"]));let mut foreign=h.source(row(&v["authored"]));
 let pointers=(a.borrow().get().title.as_ref().map(|text|text.as_ptr()),a.borrow().get().body.as_ptr(),o.borrow().get().title.as_ref().map(|text|text.as_ptr()),o.borrow().get().body.as_ptr());
 let(mut c,heap)=observe(RetainedRecordChangeOwner::<Row,Patch>::new);assert_eq!(heap,(0,0));let mut turns=0;
 while !c.ready()&&cut.is_none_or(|cut|turns<cut){
  let d=c.demand(a.borrow(),o.borrow(),body,work).unwrap();let g=grant(d);
  let denied=step(&mut h,&mut c,&a,&o,body,work,RetainedCloneGrant{maximum_items:0,..g});assert_eq!(denied.result.unwrap().progress(),Default::default());assert_eq!(denied.heap,(0,0));assert_eq!(denied.marker,None);
  let quoted=c.work_demand(a.borrow(),o.borrow(),body,work).unwrap();
  let zero=step(&mut h,&mut c,&a,&o,body,0,g);assert_eq!(zero.work,0);assert_eq!(zero.result.unwrap().progress(),Default::default());
  let turn=step(&mut h,&mut c,&a,&o,body,work,g);assert!(turn.work>0&&turn.work<=u128::from(quoted));let p=turn.result.unwrap().progress();if let Some(bytes)=turn.marker{assert_eq!(bytes,p.retained_capacity_bytes);}assert!(p.fits(g));assert_eq!(turn.heap,(p.retained_capacity_bytes,p.released_bytes),"original record allocation and release receipt");turns+=1;assert!(turns<10000);
 }
 if turns>0&&!c.ready(){let g=grant(c.demand(a.borrow(),o.borrow(),body,work).unwrap());let refused=step(&mut h,&mut c,&foreign,&o,body,work,g);assert!(refused.marker.is_none());h.retire(refused.result.err().expect("foreign Source must be refused"));}
 if cut.is_none(){
  let g=grant(c.take_demand().unwrap());let mut out=None;let(taken,_,_)=h.native(g,1,|e|{let mut slot=None;drop(c.take_into(&mut out,e,OriginalEffectReply::borrow(&mut slot).unwrap()).unwrap());slot.take().unwrap().unwrap()});let RetainedCloneStep::Complete(p)=taken else{panic!("record result refused")};assert!(p.fits(g));let out=out.unwrap();assert_eq!(out,patch(&v["patch"]));
  let mut result=row(&v["original"]);if let Some(title)=out.title.as_ref(){result.title=title.clone();}if let Some(count)=out.count{result.count=count;}if let Some(body)=out.body.as_ref(){result.body=body.clone();}assert_eq!(result,row(&v["authored"]));h.retire(result);h.retire(out);
 }
 close(&mut h,&mut c);assert_eq!(observe(||drop(c)).1,(0,0));
 assert_eq!((a.borrow().get().title.as_ref().map(|text|text.as_ptr()),a.borrow().get().body.as_ptr(),o.borrow().get().title.as_ref().map(|text|text.as_ptr()),o.borrow().get().body.as_ptr()),pointers);
 h.close_source(a);h.close_source(o);h.close_source(foreign);
}
#[test]fn record_changes_original_sparse_fields_and_paid_frontier_match_neutral_cases(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪡️field/🟰️change/🎚️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in f["cases"].as_array().unwrap(){exercise(row,None,f["bodyBytes"].as_u64().unwrap()as usize,f["birthWorkUnits"].as_u64().unwrap());}}
#[test]fn record_changes_partial_original_output_and_child_frontier_close(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪡️field/🟰️change/🎚️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in f["cases"].as_array().unwrap(){for cut in f["cancelCuts"].as_array().unwrap(){let cut=cut.as_u64().unwrap()as usize;exercise(row,Some(cut),f["bodyBytes"].as_u64().unwrap()as usize,f["birthWorkUnits"].as_u64().unwrap());}}}
#[test]fn record_field_allocation_refusal_reports_attempt_and_preserves_custody(){
 let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪡️field/🟰️change/🎚️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let case=&f["cases"][0];let body=f["bodyBytes"].as_u64().unwrap()as usize;let work=f["birthWorkUnits"].as_u64().unwrap();
 let mut h=Harness::new();let mut a=h.source(row(&case["authored"]));let mut o=h.source(row(&case["original"]));let mut c=RetainedRecordChangeOwner::<Row,Patch>::new();
 while c.phase!=3{let g=grant(c.demand(a.borrow(),o.borrow(),body,work).unwrap());step(&mut h,&mut c,&a,&o,body,work,g).result.unwrap();}
 let g=grant(c.demand(a.borrow(),o.borrow(),body,work).unwrap());let(turn,refused)=refuse_one(||step(&mut h,&mut c,&a,&o,body,work,g));
 assert_eq!(refused,1);assert_eq!(turn.marker,Some(0));let failure=turn.result.err().expect("refused allocation is recorded");h.retire(failure);assert_eq!(c.phase,3);
 close(&mut h,&mut c);h.close_source(a);h.close_source(o);
}

#[test]fn record_field_constructor_work_matches_neutral_vectors(){let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/birth.json")).unwrap();let mut mismatches=Vec::new();for row in rows.as_array().unwrap(){let(actual,bytes)=match row["type"].as_str().unwrap(){"optionalText"=>(boxed_record_field_work::<Row,Patch,Option<String>,Title>(),size_of::<RetainedRecordFieldChange<Row,Patch,Option<String>,Title>>()),"integer"=>(boxed_record_field_work::<Row,Patch,i64,Count>(),size_of::<RetainedRecordFieldChange<Row,Patch,i64,Count>>()),"bytes"=>(boxed_record_field_work::<Row,Patch,Vec<u8>,Body>(),size_of::<RetainedRecordFieldChange<Row,Patch,Vec<u8>,Body>>()),_=>unreachable!()};let expected=row["baseWork"].as_u64().unwrap()+u64::try_from(bytes*2).unwrap();let actual=actual.unwrap();if actual!=expected{mismatches.push((row["type"].as_str().unwrap().to_owned(),actual,expected,bytes));}}assert!(mismatches.is_empty(),"constructor work (type, actual, expected, bytes): {mismatches:?}");}
