//! 🧪️ Original ranked sources and immutable key roots preserve exact typed custody under every paid frontier.
#![allow(dead_code)]
#[path="../../../🧪️tests/support.rs"]mod support;
#[path="helpers.rs"]mod helpers;
#[path="actual.rs"]mod actual;
use helpers::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_control_contract::RetainedCloneGrant;
use semio_framework_value_edit::{IndexedEditEntry,NamedEditFault,NamedKeyIndexPreparation,NamedRankedSource,PersistentRankedKeys,VecRankedKeys};
use semio_framework_value_retirement::RetirementDemand;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use support::effect;
use std::mem::size_of;
#[test]
fn admitted_named_index_preserves_original_opaque_keys_ranked_sources_and_cancelled_custody(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🔑️named/📇️index/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let keys=row["keys"].as_array().unwrap();for body in fixture["bodyBytes"].as_array().unwrap(){for cut in fixture["cancelCuts"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap())).chain(std::iter::once(None)){
  let mut original=source(keys.iter().map(|key|Row{key:key.as_str().unwrap().to_owned()}).collect::<Vec<_>>());let input=VecRankedKeys{original:original.borrow(),key};let pointers=input.original.get().iter().map(|row|(row.key.as_ptr(),row.key.len())).collect::<Vec<_>>();let mut owner=NamedKeyIndexPreparation::new(keys.len(),grant(NamedKeyIndexPreparation::constructor_demand())).result.unwrap().unwrap();let mut turns=0;let mut born=0;let mut released=0;
  while !owner.output_ready()&&owner.fault().is_none()&&cut.is_none_or(|cut|turns<cut){let d=owner.demand(&input,body.as_u64().unwrap()as usize).unwrap();for denied in denies(d){let(s,bytes)=advance_denied(&mut owner,&input,denied);assert_eq!(s.progress(),Default::default());assert_eq!(bytes,(0,0));assert_eq!(owner.demand(&input,body.as_u64().unwrap()as usize).unwrap(),d);}let(_,b,r)=advance_exact(&mut owner,&input,grant(d));born+=b;released+=r;turns+=1;assert!(turns<1000000);}
  if cut.is_none(){assert_eq!(owner.output_ready(),!row["expected"].is_null());if owner.output_ready(){let d=owner.take_demand().unwrap();let((mut index,_),..)=exact(||owner.take(grant(d)).result.unwrap().unwrap(),|(_,p)|*p);for(i,text)in row["queries"].as_array().unwrap().iter().enumerate(){assert_eq!(lookup(&input,&index,text.as_str().unwrap()).0,row["expected"][i].as_u64().map(|n|n as usize));}let mut foreign=source(keys.iter().map(|key|Row{key:key.as_str().unwrap().to_owned()}).collect::<Vec<_>>());let other=VecRankedKeys{original:foreign.borrow(),key};assert!(index.check_source(&other).is_err());assert!(index.project_demand(&other).is_err());close_source(&mut foreign);let(b,r)=close_index(&mut index);born+=b;released+=r;assert_eq!(heap(||drop(index)).1,(0,0));}else{assert_eq!(owner.fault(),Some(NamedEditFault::DuplicateBase));}}
  assert_eq!(input.original.get().iter().map(|row|(row.key.as_ptr(),row.key.len())).collect::<Vec<_>>(),pointers);close_preparation(&mut owner,born,released);assert_eq!(heap(||drop(owner)).1,(0,0));close_source(&mut original);
 }}}
}
#[test]
fn admitted_named_index_reuses_persistent_originals_without_base_allocation_or_resorting(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🔑️named/📇️index/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for len in fixture["lengths"].as_array().unwrap(){let len=len.as_u64().unwrap()as usize;let mut original=source(persistent((0..len).map(|i|format!("key{:08}",len-i)),len));let input=PersistentRankedKeys{original:original.borrow(),key};let address=input.original.get().get(len-1).unwrap().key.as_ptr();let mut index=admit(&input);for rank in [0,len/2,len-1]{let query=format!("key{:08}",len-rank);let(result,capacity)=lookup(&input,&index,&query);assert_eq!(result,Some(rank));assert!(capacity<=size_of::<usize>()+size_of::<IndexedEditEntry>()*2,"sparse lookup allocated base coordinates");}assert_eq!(input.original.get().get(len-1).unwrap().key.as_ptr(),address);close_index(&mut index);assert_eq!(heap(||drop(index)).1,(0,0));close_source(&mut original);}
}
