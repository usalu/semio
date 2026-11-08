//! 🧪️ Actual native editing preserves original row movement and finite Source retirement.
use semio_framework_value_retained_map::*;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use std::collections::BTreeMap;
#[path="edit-support.rs"]mod native_support;
use native_support::*;
#[test]
fn original_native_insert_preserves_unicode_rows_pointers_denials_and_cancelled_workspace(){
 let law=law();for cut in std::iter::once(None).chain(law["cancellationCuts"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()))){let oracle=(0..70).map(|key|(format!("key-{key:04}"),format!("value-β-{key}"))).collect::<BTreeMap<_,_>>();let map=oracle.clone().into_iter().collect::<RetainedOrderedMap<_,_>>();let pointers=map.iter().map(|(key,value)|(key.clone(),(key.as_ptr()as usize,value.as_ptr()as usize))).collect::<BTreeMap<_,_>>();let key="key-0033a".to_owned();let value="inserted-日本語".to_owned();let(mut cursor,heap)=observe(||RetainedOrderedMapInsertCursor::new(map,key,value));assert_eq!(heap,(0,0));let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let output=drive(&mut cursor,&mut bank,7,cut,10000);if let Some(output)=output{let mut expected=oracle;expected.insert("key-0033a".to_owned(),"inserted-日本語".to_owned());assert_eq!(serde_json::to_value(&output).unwrap(),serde_json::to_value(expected).unwrap());assert_eq!(cursor.inserted_ordinal(),Some(34));for(key,value)in output.iter(){if let Some(pointer)=pointers.get(key){assert_eq!((key.as_ptr()as usize,value.as_ptr()as usize),*pointer)}}retire(output,&mut bank,7,10000);}retire(cursor,&mut bank,7,10000);}
}
#[test]
fn original_native_removal_preserves_every_original_case_and_cancelled_page(){
 let law=law();for row in law["removalCases"].as_array().unwrap(){let count=row["count"].as_u64().unwrap();let target=row["key"].as_u64().unwrap();for cut in std::iter::once(None).chain(law["cancellationCuts"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()))){let mut oracle=(0..count).map(|key|(key,format!("row-β-{key}"))).collect::<BTreeMap<_,_>>();let map=oracle.clone().into_iter().collect::<RetainedOrderedMap<_,_>>();let pointers=map.iter().map(|(key,value)|(*key,value.as_ptr()as usize)).collect::<BTreeMap<_,_>>();let mut cursor=RetainedOrderedMapRemoveCursor::new(map,target);let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let output=drive(&mut cursor,&mut bank,8,cut,10000);if let Some(output)=output{let removed=oracle.remove(&target).is_some();assert_eq!(removed,row["removed"].as_bool().unwrap());assert_eq!(cursor.removed(),Some(removed));assert_eq!(serde_json::to_value(&output).unwrap(),serde_json::to_value(oracle).unwrap());for(key,value)in output.iter(){assert_eq!(value.as_ptr()as usize,pointers[key])}retire(output,&mut bank,7,10000);}retire(cursor,&mut bank,7,10000);}}
}
