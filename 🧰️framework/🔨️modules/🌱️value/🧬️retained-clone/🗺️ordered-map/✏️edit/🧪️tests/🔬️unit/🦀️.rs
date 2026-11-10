use super::edit_test_support as edit_native;
use crate::FixtureSourceBirth;
use super::*;
use crate::retained_clone::RetainedCloneSource;
use std::collections::BTreeMap;
#[path="../../../🧪️tests/fixture-support.rs"]
mod fixture_support;
use fixture_support::{Fixture,fixture,oracle,retained_map};

fn edit_bank()->EffectBank{EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::from_profile(&edit_native::law()["stock"]))}
fn edit_action<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>(cursor:&mut RetainedOrderedMapInsertCursor<K,V>,bank:&mut EffectBank,body:usize)->edit_native::Observed{let p=cursor.next_advance_demands(body).unwrap();assert!(p.maximum_moved_items<=32);assert!(p.maximum_moved_bytes<=4096);assert!(p.maximum_capacity_bytes<=65536);let d=cursor.advance_demands(body,p).unwrap();let work=cursor.advance_work_demand(body,p).unwrap();edit_native::action(cursor,bank,body,p,edit_native::grant(d),work)}
fn insert(map:RetainedOrderedMap<String,String>,key:String,value:String)->RetainedOrderedMapInsertCursor<String,String>{let(cursor,heap)=crate::observe_retirement_allocations(||RetainedOrderedMapInsertCursor::new(map,key,value));assert_eq!(heap,(0,0));cursor}


fn close_insert(cursor:RetainedOrderedMapInsertCursor<String,String>,bank:&mut EffectBank){edit_native::retire(cursor,bank,7,100000);}


use semio_framework_value_control_contract::{OriginalEffectProducer,OriginalEffectAuthority,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity};
use semio_framework_value_retirement::OriginalRetirementFailure;
use semio_framework_value_testing_effect_bank::EffectBank;
use crate::ordered_map::bounded_test_support::{paid as lookup_paid,grant as lookup_grant};
#[derive(Clone,Copy)]
enum LookupOp{Advance(BoundedOrdGrant,usize),Take,Begin,Close(usize)}
struct LookupDriver(OriginalSourceTurnIdentity);
struct LookupContext<'a,'b>{cursor:&'a mut RetainedOrderedMapLookupCursor<String>,map:RetainedCloneRef<'b,RetainedOrderedMap<String,String>>,key:RetainedCloneRef<'b,String>,output:&'a mut Option<RetainedOrderedMapLookup>,source:OriginalSourceTurnIdentity,op:LookupOp}
impl OriginalEffectProducer for LookupDriver{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=LookupContext<'a,'b>where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&LookupContext<'a,'b>)->Result<(),ValueError>where 'b:'a{if self.0!=c.source{return Err(ValueError::invariant("lookup fixture changed its real Source"))}Ok(())}
 fn source_turn<'a,'b>(&self,c:&LookupContext<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut LookupContext<'a,'b>,e:&mut crate::RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{match c.op{LookupOp::Advance(g,b)=>c.cursor.advance_into(c.map,c.key,g,b,e,r),LookupOp::Take=>c.cursor.take_output_into(c.output,e,r),LookupOp::Begin=>c.cursor.begin_close_into(e,r),LookupOp::Close(b)=>c.cursor.close_into(b,e,r)}}
 fn recover<'r,'a,'b>(&mut self,c:&mut LookupContext<'a,'b>,e:&mut crate::RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{match c.op{LookupOp::Advance(g,b)=>c.cursor.recover_advance_into(c.map,c.key,g,b,e,r),LookupOp::Take=>c.cursor.recover_take_output_into(c.output,e,r),LookupOp::Begin=>c.cursor.recover_begin_close_into(e,r),LookupOp::Close(b)=>c.cursor.recover_close_into(b,e,r)}}
}
fn close_lookup<'a>(cursor:&mut RetainedOrderedMapLookupCursor<String>,map:RetainedCloneRef<'a,RetainedOrderedMap<String,String>>,key:RetainedCloneRef<'a,String>,bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<LookupDriver>){
 let source=bank.source_witness().identity();let mut output=None;let d=cursor.begin_close_demand();let work=cursor.begin_close_work_demand();let mut c=LookupContext{cursor,map,key,output:&mut output,source,op:LookupOp::Begin};lookup_paid(bank,authority,&mut c,lookup_grant(d),work);
 for _ in 0..100000{if cursor.terminal_is_empty(){break}let d=cursor.close_demands(7).unwrap();let work=cursor.close_work_demand(7).unwrap();let g=lookup_grant(d);for axis in 0..5{let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..g},1 if d.copy_bytes!=0=>RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},2 if d.capacity_bytes!=0=>RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g},3 if d.release_bytes!=0=>RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g},4 if d.depth!=0=>RetainedCloneGrant{maximum_depth:d.depth-1,..g},_=>continue};let mut c=LookupContext{cursor,map,key,output:&mut output,source,op:LookupOp::Close(7)};assert_eq!(lookup_paid(bank,authority,&mut c,denied,work).progress(),Default::default());assert_eq!(cursor.close_demands(7).unwrap(),d)}let mut c=LookupContext{cursor,map,key,output:&mut output,source,op:LookupOp::Close(7)};assert!(lookup_paid(bank,authority,&mut c,g,work).progress().fits(g))}assert!(cursor.terminal_is_empty())
}

#[test]
fn ordered_comparison_custody_refuses_unadmitted_constructor_and_closes_original_aliases(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../♻️custody/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut oracle=std::process::Command::new("repo").args(["--service=off","tools","exec","bun","--","-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(x.fixture.keys.map(x=>[x,...new TextEncoder().encode(x)])));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();use std::io::Write;oracle.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=oracle.wait_with_output().unwrap();assert!(output.status.success());let rows:Vec<serde_json::Value>=serde_json::from_slice(&output.stdout).unwrap();
 for(key,row)in fixture["keys"].as_array().unwrap().iter().zip(rows){let key=key.as_str().unwrap();assert_eq!(row.as_array().unwrap()[1..],key.as_bytes().iter().map(|x|serde_json::json!(*x)).collect::<Vec<_>>());let key=key.to_owned();let pointer=key.as_ptr();let value=String::from("value");let original=key.capacity()+value.capacity();let cursor=insert(RetainedOrderedMap::default(),key,value);assert_eq!(cursor.original_key().map(|key|key.as_ptr()),Some(pointer));let mut bank=edit_bank();let heap=edit_native::retire_observed(cursor,&mut bank,3,10000);assert_eq!(original+heap.0,heap.1);}
}


fn lookup(map:&RetainedOrderedMap<String,String>,key:&String,fixture:&Fixture)->RetainedOrderedMapLookup{
 let mut map=RetainedCloneSource::from_owner(map.clone());let mut key=RetainedCloneSource::from_owner(key.clone());let comparison=BoundedOrdGrant{maximum_items:fixture.comparison_grant.maximum_items,maximum_bytes:fixture.comparison_grant.maximum_bytes};
 let result=map.with_bank(|map,bank|{let source=bank.source_witness().identity();let mut authority=OriginalEffectAuthority::new(LookupDriver(source));let mut cursor=RetainedOrderedMapLookupCursor::default();let mut output=None;loop{let d=cursor.next_demand(map,key.borrow(),comparison,7).unwrap();let work=cursor.next_work_demand(map,key.borrow(),comparison,7).unwrap();let g=lookup_grant(d);let mut c=LookupContext{cursor:&mut cursor,map,key:key.borrow(),output:&mut output,source,op:LookupOp::Advance(comparison,7)};assert!(lookup_paid(bank,&mut authority,&mut c,g,work).progress().fits(g));if cursor.outcome().is_some(){break}}let d=cursor.take_output_demand();let work=cursor.take_output_work_demand();let mut c=LookupContext{cursor:&mut cursor,map,key:key.borrow(),output:&mut output,source,op:LookupOp::Take};lookup_paid(bank,&mut authority,&mut c,lookup_grant(d),work);let result=output.take().unwrap();close_lookup(&mut cursor,map,key.borrow(),bank,&mut authority);assert!(authority.into_producer().is_ok());result}).unwrap();for source in [&mut map]{for _ in 0..100000{if source.terminal_is_empty(){break}let _=source.close_once().unwrap();}assert!(source.terminal_is_empty())}for _ in 0..100000{if key.terminal_is_empty(){break}let _=key.close_once().unwrap();}assert!(key.terminal_is_empty());result
}

#[test]
fn bounded_lookup_insert_and_duplicate_refusal_match_btree_oracle(){
 let fixture=fixture();let mut oracle=oracle(&fixture);let source=RetainedOrderedMap::from_sorted_entries(oracle.clone().into_iter().collect()).unwrap();let(mut map,mut bank)=retained_map(&source);
 for operation in &fixture.operations{let before=lookup(&map,&operation.key,&fixture);match operation.kind.as_str(){
 "lookup"=>{assert_eq!(matches!(before,RetainedOrderedMapLookup::Found(_)),operation.expected.found);assert_eq!(match before{RetainedOrderedMapLookup::Found(n)|RetainedOrderedMapLookup::Missing(n)=>n},operation.expected.ordinal);},
 "insert"=>{assert!(matches!(before,RetainedOrderedMapLookup::Missing(n)if n==operation.expected.ordinal));let value=operation.value.clone().unwrap();let mut cursor=insert(map,operation.key.clone(),value.clone());for _ in 0..100000{if cursor.output_ready(){break}edit_action(&mut cursor,&mut bank,7);}assert_eq!(cursor.inserted_ordinal(),Some(operation.expected.ordinal));map=edit_native::take(&mut cursor,&mut bank,7);assert!(!cursor.output_ready());oracle.insert(operation.key.clone(),value);close_insert(cursor,&mut bank);},
 "duplicate"=>{assert!(matches!(before,RetainedOrderedMapLookup::Found(n)if n==operation.expected.ordinal));assert!(operation.expected.found);let before_json=serde_json::to_value(&map).unwrap();let mut cursor=insert(map,operation.key.clone(),operation.value.clone().unwrap());let mut error=None;for _ in 0..100000{let mut p=cursor.next_advance_demands(7).unwrap();p.maximum_moved_items=1;p.maximum_moved_bytes=7;p.maximum_capacity_bytes=65536;let d=cursor.advance_demands(7,p).unwrap();let work=cursor.advance_work_demand(7,p).unwrap();let result=edit_native::action_result(&mut cursor,&mut bank,7,p,edit_native::grant(d),work);if let Err(cause)=result.result{error=Some(cause);break}assert!(!cursor.output_ready());}let error=error.expect("native duplicate refusal");assert!(error.primary().unwrap().message.contains("duplicate"));assert!(cursor.refused_workspace_ready());map=edit_native::take_refused(&mut cursor,&mut bank,7);assert!(!cursor.refused_workspace_ready());assert_eq!(serde_json::to_value(&map).unwrap(),before_json);edit_native::retire(error,&mut bank,7,100000);close_insert(cursor,&mut bank);},
 other=>panic!("unexpected fixture operation {other}")}assert_eq!(map.len(),operation.expected.entry_count);assert_eq!(map.iter().map(|(key,value)|(key.clone(),value.clone())).collect::<BTreeMap<_,_>>(),oracle);}edit_native::retire(map,&mut bank,7,100000);
}


#[test]
fn immutable_lookup_target_and_repeated_directory_growth_match_btree_oracle(){
 let fixture=fixture();let source=RetainedOrderedMap::from_sorted_entries(oracle(&fixture).into_iter().collect()).expect("lookup source");let mut source=RetainedCloneSource::from_owner(source);let mut external_target=fixture.immutable_lookup.captured_target.clone();let mut target=RetainedCloneSource::from_owner(external_target.clone());let comparison=BoundedOrdGrant{maximum_items:1,maximum_bytes:7};
 let result=source.with_bank(|map,bank|{let identity=bank.source_witness().identity();let mut authority=OriginalEffectAuthority::new(LookupDriver(identity));let mut cursor=RetainedOrderedMapLookupCursor::default();let mut output=None;let d=cursor.next_demand(map,target.borrow(),comparison,7).unwrap();let work=cursor.next_work_demand(map,target.borrow(),comparison,7).unwrap();let mut c=LookupContext{cursor:&mut cursor,map,key:target.borrow(),output:&mut output,source:identity,op:LookupOp::Advance(comparison,7)};lookup_paid(bank,&mut authority,&mut c,lookup_grant(d),work);external_target=fixture.immutable_lookup.external_after_capture.clone();
 loop{let d=cursor.next_demand(map,target.borrow(),comparison,7).unwrap();let work=cursor.next_work_demand(map,target.borrow(),comparison,7).unwrap();let mut c=LookupContext{cursor:&mut cursor,map,key:target.borrow(),output:&mut output,source:identity,op:LookupOp::Advance(comparison,7)};let g=lookup_grant(d);assert!(lookup_paid(bank,&mut authority,&mut c,g,work).progress().fits(g));if cursor.outcome().is_some(){break}}
 let mut changed=RetainedCloneSource::from_owner(external_target.clone());let mut c=LookupContext{cursor:&mut cursor,map,key:changed.borrow(),output:&mut output,source:identity,op:LookupOp::Advance(comparison,7)};crate::ordered_map::bounded_test_support::refusal(bank,&mut authority,&mut c,RetainedCloneGrant::default(),0,"projected path changed");let d=cursor.take_output_demand();let work=cursor.take_output_work_demand();let mut c=LookupContext{cursor:&mut cursor,map,key:target.borrow(),output:&mut output,source:identity,op:LookupOp::Take};lookup_paid(bank,&mut authority,&mut c,lookup_grant(d),work);let result=output.take().unwrap();close_lookup(&mut cursor,map,target.borrow(),bank,&mut authority);assert!(authority.into_producer().is_ok());for _ in 0..100000{if changed.terminal_is_empty(){break}let _=changed.close_once().unwrap();}assert!(changed.terminal_is_empty());result}).unwrap();
 assert_eq!(result,RetainedOrderedMapLookup::Missing(fixture.immutable_lookup.expected_ordinal));assert_ne!(external_target,fixture.immutable_lookup.captured_target);for _ in 0..100000{if source.terminal_is_empty(){break}let _=source.close_once().unwrap();}assert!(source.terminal_is_empty());for _ in 0..100000{if target.terminal_is_empty(){break}let _=target.close_once().unwrap();}assert!(target.terminal_is_empty());

 let growth=&fixture.repeated_growth;let mut oracle=(0..growth.entry_count).map(|ordinal|(format!("{}{:04}",growth.key_prefix,ordinal*2),format!("{}{}",growth.value_prefix,ordinal))).collect::<BTreeMap<_,_>>();let source=RetainedOrderedMap::from_sorted_entries(oracle.clone().into_iter().collect()).unwrap();let(mut map,mut bank)=retained_map(&source);for ordinal in 0..growth.insertions{let key=format!("{}{:04}",growth.key_prefix,ordinal*2+1);let value=format!("{}insert-{ordinal}",growth.value_prefix);let mut cursor=insert(map,key.clone(),value.clone());for _ in 0..100000{if cursor.output_ready(){break}edit_action(&mut cursor,&mut bank,7);}assert!(cursor.output_ready());map=edit_native::take(&mut cursor,&mut bank,7);oracle.insert(key,value);close_insert(cursor,&mut bank);}assert_eq!(map.len(),growth.expected_entry_count);assert_eq!(map.iter().map(|(key,value)|(key.clone(),value.clone())).collect::<BTreeMap<_,_>>(),oracle);edit_native::retire(map,&mut bank,7,100000);
}

fn edit_bank_with_copy(copy_bytes:usize)->EffectBank{let mut stock=edit_native::law()["stock"].clone();stock["copyBytes"]=serde_json::json!(copy_bytes);EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::from_profile(&stock))}

fn insertion_reports_run(bank:EffectBank)->usize{
 let mut bank=bank;
 let fixture=fixture();let prefix="k".repeat(fixture.long_key_byte_length);let entries=(0..RETAINED_ORDERED_MAP_PAGE_CAPACITY).map(|ordinal|(format!("{prefix}-{:04}",ordinal*2),format!("value-{ordinal}"))).collect::<Vec<_>>();let mut oracle=entries.iter().cloned().collect::<BTreeMap<_,_>>();let map=RetainedOrderedMap::from_sorted_entries(entries).unwrap();let key=format!("{prefix}-0001");let value="inserted-β".to_owned();let mut cursor=insert(map,key.clone(),value.clone());let start=bank.wallet().remaining().maximum_copy_bytes;let(mut compared,mut moved,mut capacity)=(0,0,0);
 for _ in 0..100000{if cursor.output_ready(){break}let before=cursor.comparison_progress();let result=edit_action(&mut cursor,&mut bank,7);let after=cursor.comparison_progress();let bytes=after.compared_bytes-before.compared_bytes;let progress=cursor.movement_progress();let movement=if cursor.state.phase==0{0}else{progress.moved_bytes};assert!(!fixture.progress_channels.comparison_only||bytes==0||(movement==0&&result.heap.0==0));assert!(!fixture.progress_channels.capacity_only||result.heap.0==0||(bytes==0&&movement==0));compared+=bytes;moved+=movement;capacity+=progress.retained_capacity_bytes;}
 assert!(cursor.output_ready());assert!(compared>=fixture.long_key_byte_length);assert!(moved>=fixture.progress_channels.minimum_moved_items*size_of::<(String,String)>());assert!(capacity>=RETAINED_ORDERED_MAP_PAGE_CAPACITY*size_of::<(String,String)>());let map=edit_native::take(&mut cursor,&mut bank,7);oracle.insert(key,value);assert_eq!(map.iter().map(|(key,value)|(key.clone(),value.clone())).collect::<BTreeMap<_,_>>(),oracle);close_insert(cursor,&mut bank);edit_native::retire(map,&mut bank,7,100000);start-bank.wallet().remaining().maximum_copy_bytes
}

/// 🧮️ The stock is this law's own total copy demand: a metering pass under an ample stock derives it, then the law must succeed on exactly that stock (every quote and reply handoff is exact).
#[test]
fn insertion_reports_comparison_movement_and_capacity_independently(){
 let total=insertion_reports_run(edit_bank_with_copy(1<<32));
 assert_eq!(insertion_reports_run(edit_bank_with_copy(total)),total);
}


#[test]
fn cancelled_partial_insertion_retires_candidate_and_shifted_workspace(){
 let fixture=fixture();let source=RetainedOrderedMap::from_sorted_entries(oracle(&fixture).into_iter().collect()).unwrap();let(workspace,mut bank)=retained_map(&source);let original_pages=workspace.pages.iter().map(Vec::len).collect::<Vec<_>>();let mut cursor=insert(workspace,"key-0000a".to_owned(),"cancelled-β".to_owned());let mut turns=0;while cursor.state.map.as_ref().is_none_or(|map|map.pages.iter().map(Vec::len).eq(original_pages.iter().copied())){assert!(matches!(edit_action(&mut cursor,&mut bank,7).step,RetainedCloneStep::Progress(_)));turns+=1;assert!(turns<1000);}assert_eq!(cursor.state.map.as_ref().unwrap().len(),fixture.entry_count);close_insert(cursor,&mut bank);
}


#[test]
fn insertion_next_action_quotes_every_page_directory_and_move_receipt(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📋️wire/🔣️.json")).unwrap();let count=fixture["pageBoundaryEntries"].as_u64().unwrap();let mut map=RetainedOrderedMap::<u64,()>::new();let mut bank=edit_bank();for key in(0..count).rev(){let(mut cursor,heap)=crate::observe_retirement_allocations(||RetainedOrderedMapInsertCursor::new(map,key,()));assert_eq!(heap,(0,0));for _ in 0..10000{if cursor.output_ready(){break}let p=cursor.next_advance_demands(128).unwrap();let d=cursor.advance_demands(128,p).unwrap();let work=cursor.advance_work_demand(128,p).unwrap();let g=edit_native::grant(d);for denied in edit_native::denials(g,d){let result=edit_native::action(&mut cursor,&mut bank,128,p,denied,work);assert_eq!(result.step.progress(),Default::default());assert_eq!(result.work,0);assert_eq!(result.heap,(0,0));assert_eq!(cursor.advance_demands(128,p).unwrap(),d);}let result=edit_native::action(&mut cursor,&mut bank,128,p,g,work);assert_eq!(result.heap,(result.step.progress().retained_capacity_bytes,result.step.progress().released_bytes));}assert!(cursor.output_ready(),"exactly quoted insertion key {key}, phase {}",cursor.state.phase);map=edit_native::take(&mut cursor,&mut bank,128);edit_native::retire(cursor,&mut bank,128,10000);}assert_eq!(map.iter().map(|(key,())|*key).collect::<Vec<_>>(),(0..count).collect::<Vec<_>>());edit_native::retire(map,&mut bank,128,10000);
}


#[test]
fn staged_insert_authority_birth_charges_exact_native_permanent_and_transient_owners(){
 type W=crate::ordered_map::edit_input::EditWorkspace<u64,()>;let(map,original)=crate::observe_retirement_allocations(||(0..33u64).map(|key|(key,())).collect::<RetainedOrderedMap<_,_>>());let(mut cursor,heap)=crate::observe_retirement_allocations(||RetainedOrderedMapInsertCursor::new(map,100,()));assert_eq!(heap,(0,0));let permanent=crate::RetainedCloneSource::<W>::owned_constructor_capacity_bytes::<()>();let transient=size_of::<crate::RetainedCloneSourceAdmission<W,W>>();let mut bank=edit_bank();let(mut born,mut freed,mut copied,mut turns)=(0,0,0,0);let mut events=Vec::new();while !cursor.birth_complete(){let p=cursor.next_advance_demands(8).unwrap();let d=cursor.advance_demands(8,p).unwrap();let work=cursor.advance_work_demand(8,p).unwrap();let result=edit_native::action(&mut cursor,&mut bank,8,p,edit_native::grant(d),work);events.push(serde_json::json!({"allocated":result.heap.0,"released":result.heap.1,"copied":result.step.progress().copied_bytes,"work":result.work}));born+=result.heap.0;freed+=result.heap.1;copied+=result.step.progress().copied_bytes;turns+=1;assert!(turns<64);}edit_native::assert_birth(permanent,transient,events);assert_eq!((born,freed),(permanent+transient,transient));assert!(copied>0);assert!(turns>1);let heap=edit_native::retire_observed(cursor,&mut bank,8,100000);assert_eq!(original.0-original.1+born+heap.0,freed+heap.1);
}


