use super::*;
fn denied(d:RetirementDemand,g:RetainedCloneGrant)->impl Iterator<Item=RetainedCloneGrant>{[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(active,g)|active.then_some(g))}
#[test]
fn original_owned_removal_preserves_aliases_finite_stock_denials_and_cancellation(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔣️ordered-map.json")).unwrap();let cases=&corpus["ownedRemovalCases"];let policy:serde_json::Value=serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
 for case in cases.as_array().unwrap(){for cut in policy["cancelAfter"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap())).chain([None]){
  let stock=&policy["stock"];let mut bank=RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize};let mut fuel=stock["fuel"].as_u64().unwrap();
  let base:OrderedMap<u64>=serde_json::from_value::<Vec<(String,u64)>>(case["base"].clone()).unwrap().into_iter().collect();let alias=base.clone();let identity=base.original_root_identity();let key=case["key"].as_str().unwrap().to_owned();
  let(mut work,heap)=crate::observe_retirement_allocations(||OriginalOwnedEntry::remove(base,key));assert_eq!(heap,(0,0));assert_eq!(work.fields.base.as_ref().unwrap().original_root_identity(),identity);let mut output=None;
  for turn in 0..policy["maximumTurns"].as_u64().unwrap(){
   if cut==Some(turn){break}
   let d=work.next_demands(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(retained_clone_grant_admits(bank,d));
   for denied in denied(d,g){let(step,heap)=crate::observe_retirement_allocations(||work.advance(denied));assert_eq!(step.work,0);assert_eq!(heap,(0,0));match step.result{Ok(value)=>assert!(value.is_none()),Err(error)=>assert_eq!(error.retained_progress(),Default::default())}assert_eq!(work.next_demands(4096).unwrap(),d)}
   let(step,heap)=crate::observe_retirement_allocations(||work.advance(g));let p=work.step_progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));post(&mut bank,&mut fuel,p,step.work);output=step.result.unwrap();if output.is_some(){break}
  }
  if cut.is_none(){let actual:Vec<_>=output.as_ref().unwrap().iter().map(|(key,value)|(key.clone(),*value)).collect();let expected:Vec<(String,u64)>=serde_json::from_value(case["expected"].clone()).unwrap();assert_eq!(actual,expected)}
  let actual:Vec<_>=alias.iter().map(|(key,value)|(key.clone(),*value)).collect();let expected:Vec<(String,u64)>=serde_json::from_value(case["base"].clone()).unwrap();assert_eq!(actual,expected);
  work.begin_close();
  for _ in 0..10000{if work.terminal_is_empty(){break}let d=work.next_close_demands(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(retained_clone_grant_admits(bank,d));
   for denied in denied(d,g){let(step,heap)=crate::observe_retirement_allocations(||work.close_step(denied));assert_eq!(step.work,0);assert_eq!(heap,(0,0));match step.result{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert_eq!(error.retained_progress(),Default::default())}assert_eq!(work.next_close_demands(4096).unwrap(),d)}
   let(step,heap)=crate::observe_retirement_allocations(||work.close_step(g));let p=step.result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));post(&mut bank,&mut fuel,p,step.work);
  }assert!(work.terminal_is_empty());drop(work);if let Some(output)=output{close(output,&mut bank,&mut fuel)}close(alias,&mut bank,&mut fuel)
 }}
}
