use super::*;
use crate::FixtureSourceBirth;
use crate::retained_clone::RetainedCloneSource;
use std::collections::BTreeMap;
#[path="../fixture-support.rs"]
mod fixture_support;
use fixture_support::{clone_support,fixture,oracle,retained_map};
#[path="../bank-support.rs"]
mod bank_support;

#[test]
fn ordered_map_clone_advance_quotes_balance_original_heap_and_refused_currencies(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️clone/🔣️.json")).unwrap();assert_eq!(fixture["pageCapacity"].as_u64().unwrap()as usize,RETAINED_ORDERED_MAP_PAGE_CAPACITY);let body=fixture["workBytes"].as_u64().unwrap()as usize;
 let quote=|d:crate::RetirementDemand|RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
 for row in fixture["cases"].as_array().unwrap(){let count=row["entryCount"].as_u64().unwrap();let expected:Vec<u64>=serde_json::from_value(row["expectedKeys"].clone()).unwrap();for stop in std::iter::once(None).chain(fixture["interruptAfter"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap()as usize))){
  let(map,original)=crate::observe_retirement_allocations(||(0..count).map(|key|(key,())).collect::<RetainedOrderedMap<u64,()>>());assert_eq!(original.1,0);
  let(mut source,heap)=semio_framework_value_clone_source_testing::observe_fixture_source_allocations(||RetainedCloneSource::from_owner(map));let(mut born,mut freed)=heap;
  source.with_bank(|input,bank|{let mut clone=clone_support::Clone::<RetainedOrderedMap<u64,()>>::new(bank);for turn in 0..10000{if stop.is_some_and(|stop|turn>=stop){break}let d=clone.cursor.advance_demands(input,body,64).unwrap();let work=clone.cursor.advance_work_demand(input,body,64).unwrap();let g=quote(d);for axis in fixture["deniedAxes"].as_array().unwrap(){let mut denied=g;match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copy" if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,"capacity" if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,"release" if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,"depth" if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}let observed=clone.advance(input,bank,body,denied,work);assert_eq!(observed.heap,(0,0));assert_eq!(observed.work,0);assert_eq!(observed.step.progress(),Default::default());assert_eq!(clone.cursor.advance_demands(input,body,64).unwrap(),d)}
   let observed=clone.advance(input,bank,body,g,work);let receipt=observed.step.progress();assert!(receipt.fits(g));assert_eq!(observed.heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=observed.heap.0;freed+=observed.heap.1;if matches!(observed.step,RetainedCloneStep::Complete(_)){let observed=clone.take(input,bank,body);born+=observed.heap.0;freed+=observed.heap.1;assert!(clone.output.is_some());break}}
   if stop.is_none(){assert_eq!(clone.output.as_ref().expect("funded clone completed").iter().map(|(key,())|*key).collect::<Vec<_>>(),expected)}assert_eq!(input.get().iter().map(|(key,())|*key).collect::<Vec<_>>(),expected);
   let(cursor,output)=clone.finish();let heap=clone_support::retire(cursor,bank,body,10000);born+=heap.0;freed+=heap.1;if let Some(output)=output{let heap=clone_support::retire(output,bank,body,10000);born+=heap.0;freed+=heap.1;}
  }).unwrap();let heap=clone_support::close_source(&mut source,body,10000);born+=heap.0;freed+=heap.1;let(_,heap)=crate::observe_retirement_allocations(||drop(source));assert_eq!(heap,(0,0));assert_eq!(original.0+born,freed);
 }}
}

#[test]
fn fixed_page_clone_matches_btree_and_serde_oracles(){
 let fixture=fixture();assert_eq!(fixture.page_capacity,RETAINED_ORDERED_MAP_PAGE_CAPACITY);let oracle=oracle(&fixture);let source=RetainedOrderedMap::from_sorted_entries(oracle.clone().into_iter().collect()).expect("ordered source");let(copied,mut bank)=retained_map(&source);
 assert_eq!(copied.iter().map(|(key,value)|(key.clone(),value.clone())).collect::<BTreeMap<_,_>>(),oracle);assert_eq!(serde_json::to_value(&copied).expect("retained ordered-map JSON"),serde_json::to_value(&oracle).expect("BTreeMap JSON"));assert_eq!(copied.page_count(),fixture.entry_count.div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY));
 clone_support::retire(copied,&mut bank,7,100000);clone_support::retire(source,&mut bank,7,100000);
}

#[test]
fn long_string_comparison_is_byte_paged_and_shape_pinned(){
 use crate::ordered_map::bounded_test_support::{paid,grant as native_grant,close_comparator,refusal,Driver,Context,Op};
 use semio_framework_value_control_contract::OriginalEffectAuthority;
 let fixture=fixture();let left="k".repeat(fixture.long_key_byte_length);let right=format!("{left}z");let comparison=BoundedOrdGrant{maximum_items:fixture.comparison_grant.maximum_items,maximum_bytes:fixture.comparison_grant.maximum_bytes};
 let mut left_source=RetainedCloneSource::from_owner(left.clone());let mut right_source=RetainedCloneSource::from_owner(right.clone());
 left_source.with_bank(|left,bank|{let source=bank.source_witness().identity();let mut authority=OriginalEffectAuthority::new(Driver::<String>(source,std::marker::PhantomData));let(mut cursor,mut order)=(None,None);let mut c=Context{cursor:&mut cursor,left,right:right_source.borrow(),order:&mut order,source,op:Op::Birth};paid(bank,&mut authority,&mut c,native_grant(String::bounded_ord_birth_demand()),1);let mut turns=0usize;loop{turns+=1;let d=cursor.as_ref().unwrap().compare_demands(left,right_source.borrow(),comparison,comparison.maximum_bytes).unwrap();let work=cursor.as_ref().unwrap().compare_work_demand(left,right_source.borrow(),comparison,comparison.maximum_bytes).unwrap();let g=native_grant(d);let mut c=Context{cursor:&mut cursor,left,right:right_source.borrow(),order:&mut order,source,op:Op::Compare(comparison,comparison.maximum_bytes)};assert!(paid(bank,&mut authority,&mut c,g,work).progress().fits(g));assert!(cursor.as_ref().unwrap().comparison_progress().fits(comparison));if let Some(ordering)=cursor.as_ref().unwrap().ordering(){assert_eq!(ordering,Ordering::Less);break}}assert!(turns>fixture.long_key_byte_length/fixture.comparison_grant.maximum_bytes);close_comparator(&mut cursor,left,right_source.borrow(),bank,&mut authority,7);assert!(authority.into_producer().is_ok());}).unwrap();
 let mut changed=RetainedCloneSource::from_owner(left.clone());let mut replacement=RetainedCloneSource::from_owner(format!("{}x",&left[..left.len()-1]));let mut right=RetainedCloneSource::from_owner(right);
 changed.with_bank(|left,bank|{let source=bank.source_witness().identity();let mut authority=OriginalEffectAuthority::new(Driver::<String>(source,std::marker::PhantomData));let(mut cursor,mut order)=(None,None);let mut c=Context{cursor:&mut cursor,left,right:right.borrow(),order:&mut order,source,op:Op::Birth};paid(bank,&mut authority,&mut c,native_grant(String::bounded_ord_birth_demand()),1);let d=cursor.as_ref().unwrap().compare_demands(left,right.borrow(),comparison,7).unwrap();let work=cursor.as_ref().unwrap().compare_work_demand(left,right.borrow(),comparison,7).unwrap();let mut c=Context{cursor:&mut cursor,left,right:right.borrow(),order:&mut order,source,op:Op::Compare(comparison,7)};paid(bank,&mut authority,&mut c,native_grant(d),work);let d=cursor.as_ref().unwrap().compare_demands(left,right.borrow(),comparison,7).unwrap();let work=cursor.as_ref().unwrap().compare_work_demand(left,right.borrow(),comparison,7).unwrap();let mut c=Context{cursor:&mut cursor,left:replacement.borrow(),right:right.borrow(),order:&mut order,source,op:Op::Compare(comparison,7)};refusal(bank,&mut authority,&mut c,native_grant(d),work,"projected path changed");close_comparator(&mut cursor,left,right.borrow(),bank,&mut authority,7);assert!(authority.into_producer().is_ok());}).unwrap();
 for source in [&mut left_source,&mut right_source,&mut changed,&mut replacement,&mut right]{for _ in 0..100000{if source.terminal_is_empty(){break}let _=source.close_once().unwrap();}assert!(source.terminal_is_empty())}
}

#[test]
fn ordered_wire_cold_context_map_matches_serde_and_bun() {
    use crate::{FromValue,ToValue};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📋️wire/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let entries:Vec<(String,String)>=serde_json::from_value(case["entries"].clone()).unwrap();
        let map:RetainedOrderedMap<String,String>=entries.into_iter().collect();
        assert_eq!(serde_json::to_value(&map).unwrap(),case["expected"]);
        let from_serde:RetainedOrderedMap<String,String>=serde_json::from_value(case["expected"].clone()).unwrap();
        assert_eq!(map,from_serde);
        let from_native=RetainedOrderedMap::<String,String>::from_value(map.to_value()).unwrap();
        assert_eq!(map,from_native);
        let keys:Vec<_>=map.iter().map(|(key,_)|key.clone()).collect();
        assert!(keys.windows(2).all(|pair|pair[0]<pair[1]));
        for(key,value)in map.iter(){assert_eq!(map.get(key.as_str()),Some(value));}
    }
    assert!(serde_json::from_str::<RetainedOrderedMap<String,String>>(fixture["rejectDuplicateJson"].as_str().unwrap()).is_err());
    let count=fixture["pageBoundaryEntries"].as_u64().unwrap()as usize;
    let map:RetainedOrderedMap<String,String>=(0..count).rev().map(|index|(format!("{index:03}"),format!("value-{index}"))).collect();
    assert_eq!(map.page_count(),3);
    let original=map.pages.capacity()*size_of::<Vec<(String,String)>>()+map.pages.iter().map(|page|page.capacity()*size_of::<(String,String)>()).sum::<usize>()+map.iter().map(|(key,value)|key.capacity()+value.capacity()).sum::<usize>();
    let mut bank=bank_support::stock_bank();let heap=bank_support::retire_observed(map,&mut bank,7,10000);assert_eq!(original+heap.0,heap.1);
    let mut oracle=std::process::Command::new("repo").args(["--service=off","tools","exec","bun","--","-e","const f=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(f.cases.map(c=>Object.fromEntries(c.entries))));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    use std::io::Write;oracle.stdin.take().unwrap().write_all(fixture.to_string().as_bytes()).unwrap();
    let result=oracle.wait_with_output().unwrap();assert!(result.status.success());
    let independent:Vec<serde_json::Value>=serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(independent,fixture["cases"].as_array().unwrap().iter().map(|case|case["expected"].clone()).collect::<Vec<_>>());
}

