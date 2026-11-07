use super::*;
use crate::retirement::controlled::ControlledRetirement;
fn grant(d:crate::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn denied(d:crate::RetirementDemand,g:RetainedCloneGrant)->impl Iterator<Item=RetainedCloneGrant>{[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(active,g)|active.then_some(g))}
fn account(p:RetainedCloneProgress,g:RetainedCloneGrant,heap:(usize,usize)){assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes))}
fn close<V:crate::retirement::RetireOwned+Sync>(cursor:LookupCursor<V>){
 let mut owner=ControlledRetirement::new(cursor).unwrap_or_else(|_|panic!("actual lookup declares controlled retirement"));
 for _ in 0..10000{
  if owner.terminal_is_empty(){return}
  let d=owner.selected_demand(4096).unwrap();let g=grant(d);
  for denied in denied(d,g){let(turn,heap)=crate::observe_retirement_allocations(||owner.step(denied));assert_eq!(owner.last_turn_work(),0);assert_eq!(heap,(0,0));match turn{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert_eq!(error.kind,crate::ValueRefusalKind::DepthLimit)}assert_eq!(owner.selected_demand(4096).unwrap(),d)}
  let(turn,heap)=crate::observe_retirement_allocations(||owner.step(g));account(turn.unwrap().progress(),g,heap);
 }panic!("borrowed lookup original root retirement stalled")
}
#[test]
fn original_borrowed_lookup_matches_cold_map_and_sql_goldens_with_exact_receipts(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔣️ordered-map.json")).unwrap();let cases=&corpus["borrowedLookupCases"];
 for case in cases.as_array().unwrap(){for cut in [None,Some(0),Some(1),Some(3),Some(5),Some(15)]{
  let entries:Vec<(String,usize)>=serde_json::from_value(case["entries"].clone()).unwrap();let map:OrderedMap<usize>=entries.into_iter().collect();let query=case["query"].as_str().unwrap().to_owned();let expected:Option<usize>=serde_json::from_value(case["expected"].clone()).unwrap();assert_eq!(map.get(&query).copied(),expected);
  let d=map.lookup_borrowed_constructor_demands();let g=grant(d);
  for denied in denied(d,g){let(turn,heap)=crate::observe_retirement_allocations(||map.admit_lookup_borrowed_original(&query,denied));assert_eq!(turn.work,0);assert!(turn.result.is_err());assert_eq!(heap,(0,0));}
  let(turn,heap)=crate::observe_retirement_allocations(||map.admit_lookup_borrowed_original(&query,g));let(mut cursor,p)=turn.result.unwrap();account(p,g,heap);let mut complete=false;
  for index in 0..10000{
   if cut==Some(index){break}
   let d=cursor.next_borrowed_original_demands(&query).unwrap();let g=grant(d);
   assert!(d.copy_bytes<=1);
   for denied in denied(d,g){let(turn,heap)=crate::observe_retirement_allocations(||cursor.advance_borrowed_original(&query,denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.next_borrowed_original_demands(&query).unwrap(),d)}
   let(turn,heap)=crate::observe_retirement_allocations(||cursor.advance_borrowed_original(&query,g));let step=turn.result.unwrap();account(step.progress(),g,heap);if cursor.is_complete(){complete=true;break}
  }
  if cut.is_none(){assert!(complete);assert_eq!(cursor.result().copied(),expected);assert_eq!(serde_json::to_value(cursor.result_rank()).unwrap(),case["expectedRank"])}close(cursor);assert_eq!(map.get(&query).copied(),expected);retire_cold(map.retire());
 }}
}
#[test]
fn original_borrowed_lookup_refuses_a_replaced_query_and_owned_only_entry(){
 let map:OrderedMap<usize>=[("needle".to_owned(),7)].into();let query="needle".to_owned();let replaced=query.clone();let g=grant(map.lookup_borrowed_constructor_demands());let(mut cursor,_)=map.admit_lookup_borrowed_original(&query,g).result.unwrap();assert_ne!(query.as_ptr(),replaced.as_ptr());
 let d=cursor.next_borrowed_original_demands(&query).unwrap();let g=grant(d);
 let(turn,heap)=crate::observe_retirement_allocations(||cursor.advance_borrowed_original(&replaced,g));assert_eq!(turn.work,0);assert!(turn.result.is_err());assert_eq!(heap,(0,0));assert_eq!(cursor.next_borrowed_original_demands(&query).unwrap(),d);
 assert!(cursor.next_original_demands().is_err());let(turn,heap)=crate::observe_retirement_allocations(||cursor.advance_original(g));assert_eq!(turn.work,0);assert!(turn.result.is_err());assert_eq!(heap,(0,0));close(cursor);retire_cold(map.retire());
}
