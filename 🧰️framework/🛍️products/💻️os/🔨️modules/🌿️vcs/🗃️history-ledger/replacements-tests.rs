use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn close_authority(mut owner:semio_framework_value::FactoryAuthority){while !owner.terminal_is_empty(){let d=owner.demands(4096).unwrap();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant(d)).unwrap());assert!(step.progress().fits(grant(d)));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}
#[test]
fn original_history_replacements_share_one_decision_and_return_original_owners(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("replacements.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){
  let mut ledger=ArtifactHistoryLedger::<i32>::new();for value in case["before"].as_array().unwrap(){ledger.try_push(value.as_i64().unwrap()as i32).unwrap();}
  let issuance=ledger.retained_issuance();let mut publisher=ArtifactGroupVisibilityOwner::new();let view:std::sync::Arc<dyn semio_framework_value::RetainedPublicationDecision>=publisher.view();let mut keys=Vec::new();
  for row in case["replacements"].as_array().unwrap(){
   let key=ledger.key_at(row[0].as_u64().unwrap()as usize).unwrap();let mut candidate=Some(row[1].as_i64().unwrap()as i32);let mut decision=Some(view.clone());let d=ledger.replacement_stage_demands(key).unwrap();
   for axis in 0..5{let mut denied=grant(d);let amount=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,3=>d.release_bytes,_=>d.depth};if amount==0{continue}match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ledger.stage_replacement(key,&mut candidate,&mut decision,denied).unwrap());assert_eq!(result,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(candidate.is_some()&&decision.is_some());}
   let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ledger.stage_replacement(key,&mut candidate,&mut decision,grant(d)).unwrap());assert!(p.fits(grant(d)));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(candidate.is_none()&&decision.is_none());keys.push(key);
   assert_eq!(serde_json::to_value(ledger.iter().copied().collect::<Vec<_>>()).unwrap(),case["before"]);assert!(ledger.get_key_mut(key).is_none());assert!(ledger.retained_read_key(key).is_none());assert!(ledger.remove_key(key).is_err());
  }
  if case["commit"].as_bool().unwrap(){assert!(publisher.commit())}else{assert!(publisher.abort())}
  assert_eq!(serde_json::to_value(ledger.iter().copied().collect::<Vec<_>>()).unwrap(),case["after"]);assert_eq!(ledger.retained_issuance(),issuance);
  let mut retired=Vec::new();for key in keys{let mut displaced=None;let mut decision=None;let d=ledger.replacement_settlement_demands(key).unwrap();let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ledger.settle_replacement(key,&mut displaced,&mut decision,grant(d)).unwrap());assert!(p.fits(grant(d)));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));retired.push(displaced.unwrap());close_authority(decision.unwrap());assert_eq!(serde_json::to_value(ledger.iter().copied().collect::<Vec<_>>()).unwrap(),case["after"]);}
  assert_eq!(serde_json::to_value(retired).unwrap(),case["displaced"]);drop(view);close_authority(publisher.into_retirement());while ledger.pop().is_some(){}while ledger.retained_terminal_slot_count()>0{let(_,p)=ledger.retire_terminal_slot(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).unwrap();assert_eq!(p.copied_items,1);}while !ledger.backing_is_empty(){let d=ledger.next_empty_page_release_byte_demand().unwrap();ledger.release_empty_page(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:d,maximum_depth:1,..Default::default()}).unwrap();}assert!(ledger.terminal_is_empty());
 }
}
