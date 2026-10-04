//! 🔗️ Real borrowed-key scratch admission with full System request and release observation.
use semio_framework_value::{DslValue,NativeDecodeControl,ValueRefusalKind};
use std::collections::{HashMap,HashSet};
use std::hash::Hasher;

#[test]
fn value_borrowed_key_index_full_requests_and_same_caller_are_admitted(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🛬️controlled/🔗️borrowed-keys.json")).unwrap();let contract=&fixture["indexAuthority"];
 let mut cases:Vec<(String,Vec<String>,bool)>=contract["cases"].as_array().unwrap().iter().map(|row|(row["id"].as_str().unwrap().into(),row["keys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap().into()).collect(),row["unique"].as_bool().unwrap())).collect();
 let count=usize::try_from(contract["wide"]["count"].as_u64().unwrap()).unwrap();let prefix=contract["wide"]["prefix"].as_str().unwrap();let wide:Vec<String>=(0..count).map(|i|format!("{prefix}{i}")).collect();let mut slots=HashSet::new();let mask=count.checked_mul(2).unwrap().next_power_of_two()-1;let mut collisions=0;for key in &wide{let mut hash=std::collections::hash_map::DefaultHasher::new();hash.write(key.as_bytes());hash.write_u8(0xff);if !slots.insert(hash.finish()&(mask as u64)){collisions+=1;}}assert!(collisions>0,"the real wide UTF8 corpus must require slot collision resolution");
 let mut duplicate=wide.clone();duplicate.push(wide[usize::try_from(contract["wide"]["duplicateOrdinal"].as_u64().unwrap()).unwrap()].clone());cases.push(("wide".into(),wide,true));cases.push(("distantDuplicate".into(),duplicate,false));
 for(label,keys,unique)in cases{
  let mut oracle=HashMap::new();let mut oracle_unique=true;for(index,key)in keys.iter().enumerate(){oracle_unique&=oracle.insert(key,index).is_none();}assert_eq!(oracle_unique,unique,"{label} independent std HashMap identity");
  let input=DslValue::Object(keys.into_iter().map(|key|(key,DslValue::Bool(true))).collect());let expected=input.as_object().unwrap();let maximum=16*1024*1024;let mut accept=|_|true;let mut control=NativeDecodeControl::new(maximum,&mut accept);
  let ((result,diagnostic),requests,released)=crate::test_allocation::observe_backing(||match input.object_controlled(&mut control){Ok(entries)=>{assert!(std::ptr::eq(entries,expected));(Ok(entries.len()),0)},Err(error)=>{let kind=error.kind;let capacity=error.message.capacity();drop(error);(Err(kind),capacity)}});
  assert_eq!(requests,released,"{label} complete scratch and diagnostic release");
  if unique{assert_eq!(result.unwrap(),expected.len());assert_eq!(control.owned_bytes(),requests,"{label} complete exact scratch requests");}else{assert_eq!(result.unwrap_err(),ValueRefusalKind::InvalidValue);assert!(requests<=control.owned_bytes()+diagnostic);}
  if !unique||requests==0{continue;}
  for allowance in[requests,0,requests-1]{let mut accept=|_|true;let mut c=NativeDecodeControl::new(allowance,&mut accept);let ((result,diagnostic),requested,released)=crate::test_allocation::observe_backing(||match input.object_controlled(&mut c){Ok(entries)=>(Ok(entries.len()),0),Err(error)=>{let kind=error.kind;let capacity=error.message.capacity();drop(error);(Err(kind),capacity)}});assert_eq!(requested,released,"{label} refused scratch and error fully release");assert!(requested<=c.owned_bytes()+diagnostic);if allowance==requests{result.unwrap();assert_eq!(c.owned_bytes(),requests);}else{assert_eq!(result.unwrap_err(),ValueRefusalKind::OwnershipLimit);}}
  let mut accept=|_|true;let mut c=NativeDecodeControl::new(requests.checked_mul(2).unwrap(),&mut accept);for _ in 0..2{input.object_controlled(&mut c).unwrap();}assert_eq!(c.owned_bytes(),requests*2);assert_eq!(input.object_controlled(&mut c).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(c.owned_bytes(),requests*2);
  for materialized in[false,true]{let mut reached=false;let mut cancel=|_|{let stop=!materialized||crate::test_allocation::observed_requested_bytes().is_some_and(|bytes|bytes>0);reached|=stop;!stop};let mut c=NativeDecodeControl::new(maximum,&mut cancel);let ((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=input.object_controlled(&mut c).unwrap_err();let answer=(error.kind,error.message.capacity());drop(error);answer});assert_eq!(kind,ValueRefusalKind::Canceled);assert_eq!(requested,released,"{label} canceled backing release");assert!(requested<=c.owned_bytes()+diagnostic);if materialized{assert!(requested>diagnostic);}else{assert_eq!(c.owned_bytes(),0);assert_eq!(requested,diagnostic);}drop(c);assert!(reached);}
 }
}
