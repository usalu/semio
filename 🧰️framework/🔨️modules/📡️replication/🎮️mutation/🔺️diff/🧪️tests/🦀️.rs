//! 🧪️ Original diff bodies are compiled and exercised against the independent serde_json carrier.
use crate::*;
use std::collections::BTreeMap;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn rows(value:&serde_json::Value)->KeyedRows<u64>{let value:DslValue=serde_json::from_value(value.clone()).unwrap();let DslValue::Object(entries)=value else{panic!("object rows")};entries.into_iter().map(|(key,row)|(key,KeyedRow::from_value(row).unwrap())).collect()}
#[test]
fn exact_keyed_application_inverse_and_composition_match_serde_json(){let f=fixture();let base:BTreeMap<String,u64>=serde_json::from_value(f["base"].clone()).unwrap();let changes=rows(&f["rows"]);let next=keyed_rows_apply(&base,&changes,"rows").unwrap();assert_eq!(serde_json::to_value(&next).unwrap(),f["expected"]);let inverse=keyed_rows_inverse(&base,&changes);assert_eq!(serde_json::to_value(DslValue::Object(inverse.iter().map(|(key,row)|(key.clone(),row.to_value())).collect())).unwrap(),f["expectedInverse"]);assert_eq!(keyed_rows_apply(&next,&inverse,"rows").unwrap(),base);assert_eq!(keyed_rows_changing(&base,&next),changes);for row in f["composition"].as_array().unwrap(){let first:DslValue=serde_json::from_value(row["first"].clone()).unwrap();let later:DslValue=serde_json::from_value(row["later"].clone()).unwrap();let combined=KeyedRow::<u64>::from_value(first).unwrap().then(KeyedRow::from_value(later).unwrap());assert_eq!(combined.map(|row|serde_json::to_value(row.to_value()).unwrap()).unwrap_or(serde_json::Value::Null),row["expected"]);}}
#[test]
fn rejected_rows_retain_exact_target_and_original_base(){let base:BTreeMap<String,u64>=[("a".into(),1)].into_iter().collect();for(key,row,code)in[("a",KeyedRow::Insert(2),"mutation.apply.duplicate-target"),("b",KeyedRow::Replace(2),"mutation.apply.missing-target"),("b",KeyedRow::Remove,"mutation.apply.missing-target")]{let original=base.clone();let rows=[(key.to_owned(),row)].into_iter().collect();let error=keyed_rows_apply(&base,&rows,"items").unwrap_err();assert_eq!(error.code,code);assert_eq!(error.target,["items",key]);assert_eq!(base,original);assert_eq!(MutationApplyError::from_value(error.to_value()).unwrap(),error);}}
#[derive(Clone,Debug,PartialEq)]
struct Record{count:u64,items:BTreeMap<String,u64>}
crate::sparse_record_diff!{record:Record,diff:RecordDiff,fields:{count:u64},keyed:{items:u64}}
#[test]
fn actual_sparse_macro_uses_original_sealed_applier_and_exact_inverse(){let base=Record{count:1,items:[("a".into(),1)].into_iter().collect()};let next=Record{count:2,items:[("b".into(),3)].into_iter().collect()};let diff=RecordDiff::changing(&base,&next);assert_eq!(apply_diff_cold(&diff,&base).unwrap(),next);assert_eq!(apply_diff_cold(&diff.inverse(&base),&next).unwrap(),base);assert_eq!(RecordDiff::from_value(diff.to_value()).unwrap(),diff);let mid=Record{count:3,items:[("b".into(),5)].into_iter().collect()};let mut composed=diff.clone();composed.absorb(RecordDiff::changing(&next,&mid));assert_eq!(apply_diff_cold(&composed,&base).unwrap(),mid);assert!(!diff.is_empty());assert!(RecordDiff::default().is_empty());}

impl ToValue for Record{fn to_value(&self)->DslValue{DslValue::Object(vec![("count".into(),self.count.to_value()),("items".into(),self.items.to_value())])}}
impl FromValue for Record{fn from_value(value:DslValue)->Result<Self,ValueError>{let count=u64::from_value(value["count"].clone())?;let items=BTreeMap::<String,u64>::from_value(value["items"].clone())?;Ok(Self{count,items})}}
crate::sparse_whole_diff!{record:Record,diff:WholeRecordDiff}
#[test]
fn actual_whole_sparse_macro_preserves_exact_original_inverse_and_wire(){let base=Record{count:1,items:BTreeMap::new()};let next=Record{count:3,items:[("x".into(),9)].into_iter().collect()};let diff=WholeRecordDiff::changing(&base,&next);assert_eq!(apply_diff_cold(&diff,&base).unwrap(),next);assert_eq!(apply_diff_cold(&diff.inverse(&base),&next).unwrap(),base);assert_eq!(WholeRecordDiff::from_value(diff.to_value()).unwrap(),diff);assert!(WholeRecordDiff::default().is_empty());}

/// 🗝️ A retained nonroot fragment receives central capability only after every structural grant axis is admitted.
#[test]
fn retained_fragment_capability_preserves_source_and_refuses_unfunded_closures(){
 use semio_framework_value_clone_source::RetainedCloneSource;
 use semio_framework_value_intrinsic::RetainedCloneGrant;
 let birth=RetainedCloneSource::<i64>::owned_constructor_demand::<()>();let(mut source,_)=RetainedCloneSource::admit_owned(73i64,(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<i64>::constructor_copy_bytes(),maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()}).map_err(|(error,..)|error).unwrap();
 let pointer=source.borrow().get()as*const i64;
 for row in fixture()["fragmentCapabilities"].as_array().unwrap(){let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copy"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize,..Default::default()};let mut calls=0;let(result,heap)=crate::observe_retirement_allocations(||with_retained_apply_capability::<i64,_>(source.borrow(),grant,|_|{calls+=1;Ok(73)}));let admitted=row["admitted"].as_bool().unwrap();assert_eq!(result.is_ok(),admitted);assert_eq!(calls,usize::from(admitted));assert_eq!(heap,(0,0));assert_eq!(source.borrow().get()as*const i64,pointer);assert_eq!(*source.borrow().get(),73);}
 for _ in 0..128{if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=crate::observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(source.terminal_is_empty());
}
