use super::*;
use semio_framework_value::{ErasedSnapshotRetirement,FactoryAuthority,FactoryRetirement,factory::OwnedValueRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneClose,RetainedOwnedProjection};
fn grant(demand:RetirementDemand)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn observed<T>(call:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(call);assert!(!heap.overflowed);(value,heap.requested_bytes,heap.released_bytes)}
fn receipt(progress:RetainedCloneProgress,grant:RetainedCloneGrant,allocated:usize,released:usize){assert!(progress.fits(grant));assert!(allocated<=progress.retained_capacity_bytes);assert!(released<=progress.released_bytes);}
fn close_projection<T:?Sized+Sync>(projection:&mut RetainedOwnedProjection<T>){for _ in 0..10000 {if projection.terminal_is_empty(){return;}let funded=grant(projection.close_demand(4096).unwrap());let(step,allocated,released)=observed(||projection.close_step(funded).unwrap());receipt(step.progress(),funded,allocated,released);}panic!("original operation projection did not close");}
fn close_factory(factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>){let factory:Arc<dyn FactoryRetirement>=factory;let mut owner=FactoryAuthority::new(factory);for _ in 0..10000 {if owner.terminal_is_empty(){return;}let funded=grant(owner.demands(4096).unwrap());let(step,allocated,released)=observed(||owner.step(funded).unwrap());receipt(step.progress(),funded,allocated,released);}panic!("captured original history issuer did not close");}
fn close_value<T:semio_framework_value::retirement::RetireOwned>(value:T){let mut original=Some(value);let mut close=RetainedCloneClose::default();for _ in 0..10000 {if original.is_none()&&close.is_empty(){return;}let mut funded=grant(close.next_owner_demands::<T>(original.is_some(),&None,4096).unwrap());funded.maximum_copy_bytes=4096;let(step,allocated,released)=observed(||if let Some(step)=close.begin_granted(&mut original,funded).unwrap(){step}else{close.step_granted(funded).unwrap()});receipt(step.progress(),funded,allocated,released);}panic!("original history value {} did not close: {:?}",std::any::type_name::<T>(),close.next_owner_demands::<T>(original.is_some(),&None,4096));}
fn close_ledger(ledger:&mut ArtifactHistoryLedger<Vec<String>>){while let Some(original)=ledger.pop(){close_value(original);}while ledger.retained_terminal_slot_count()>0{let funded=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let(visibility,progress)=ledger.retire_terminal_slot(funded).unwrap();if let Some(visibility)=visibility{close_value(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(visibility));}assert!(progress.fits(funded));}while !ledger.backing_is_empty(){let funded=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:ledger.next_empty_page_release_byte_demand().unwrap(),maximum_depth:1,..Default::default()};let(step,allocated,released)=observed(||ledger.release_empty_page(funded).unwrap());receipt(step.progress(),funded,allocated,released);}}
fn return_original(ledger:&mut ArtifactHistoryLedger<Vec<String>>,key:ArtifactHistoryReadKey){let mut returned=None;for _ in 0..10000 {let funded=grant(ledger.retained_read_return_demands(key,4096).unwrap());let(step,allocated,released)=observed(||ledger.return_retained_read(key,&mut returned,funded).unwrap());receipt(step.progress(),funded,allocated,released);if matches!(step,RetainedCloneStep::Complete(_)){if let Some(factory)=returned.take(){close_factory(factory);}return;}}panic!("same original history slot did not return");}
fn corpus()->serde_json::Value {serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn original_history_reads_keep_forward_backing_and_refuse_each_denied_currency(){
 let corpus=corpus();let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>=Arc::new(OwnedValueRetirementFactory::<Vec<String>>::default());let mut count=0;
 for case in corpus["cases"].as_array().unwrap(){
  let original:Vec<String>=case["operations"].as_array().unwrap().iter().map(|op|serde_json::to_string(op).unwrap()).collect();let pointer=original.as_ptr();let mut ledger=ArtifactHistoryLedger::new();let slot=ledger.try_push(original).unwrap();let key=ledger.retained_read_key_at(0).unwrap();assert!(ledger.retained_read(key).is_err());
  for _ in 0..10 {
   let demand=ledger.retained_read_demands(key).unwrap();let funded=grant(demand);
   for axis in 0..5 {let mut denied=funded;match axis {0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if demand.release_bytes>0=>denied.maximum_release_bytes-=1,4 if demand.depth>0=>denied.maximum_depth-=1,_=>continue};let(step,allocated,released)=observed(||ledger.advance_retained_read(key,&factory,denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((allocated,released),(0,0));assert_eq!(ledger.get_key(slot).unwrap().as_ptr(),pointer);assert_eq!(ledger.retained_read_demands(key).unwrap(),demand);}
   let(step,allocated,released)=observed(||ledger.advance_retained_read(key,&factory,funded).unwrap());receipt(step.progress(),funded,allocated,released);if matches!(step,RetainedCloneStep::Complete(_)){break;}
  }
  let funded=grant(RetainedOwnedProjection::<Vec<String>>::constructor_demand());let((mut root,progress),allocated,released)=observed(||ledger.retain_read(key,funded).unwrap());receipt(progress,funded,allocated,released);
  let mut projections=Vec::new();for index in case["readOrder"].as_array().unwrap(){let index=index.as_u64().unwrap() as usize;let original_pointer=ledger.get_key(slot).unwrap()[index].as_ptr();let funded=grant(RetainedOwnedProjection::<String>::constructor_demand());let((projection,progress),allocated,released)=observed(||root.project(index+1,|ops|&ops[index],funded).unwrap());receipt(progress,funded,allocated,released);assert_eq!(projection.borrow().unwrap().get().as_ptr(),original_pointer);projections.push(projection);}
  assert!(ledger.get_mut(0).is_none());assert!(ledger.remove_key(slot).is_err());assert_eq!(ledger.iter().count(),1);assert_eq!(ledger.first().unwrap().as_ptr(),pointer);
  if !projections.is_empty(){let mut returned=None;let mut blocked=false;for _ in 0..1000 {let funded=grant(ledger.retained_read_return_demands(key,4096).unwrap());let(result,allocated,released)=observed(||ledger.return_retained_read(key,&mut returned,funded));match result {Ok(step)=>{receipt(step.progress(),funded,allocated,released);assert!(!matches!(step,RetainedCloneStep::Complete(_)));},Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!((allocated,released),(0,0));blocked=true;break;}}}assert!(blocked);assert!(returned.is_none());}
  for projection in &mut projections {close_projection(projection);}while !root.terminal_is_empty(){let funded=grant(root.close_demand(4096).unwrap());let(step,allocated,released)=observed(||root.close_step(funded).unwrap());receipt(step.progress(),funded,allocated,released);}return_original(&mut ledger,key);assert_eq!(ledger.get_key(slot).unwrap().as_ptr(),pointer);assert!(ledger.get_mut(0).is_some());close_ledger(&mut ledger);count+=1;
 }
 close_factory(factory);
}
#[test]
fn original_history_reads_return_every_cancelled_birth_and_reject_foreign_generations(){
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>=Arc::new(OwnedValueRetirementFactory::<Vec<String>>::default());
 for cutoff in 0..4 {let original=vec!["original-operation".repeat(1024)];let pointer=original[0].as_ptr();let mut ledger=ArtifactHistoryLedger::new();let slot=ledger.try_push(original).unwrap();let key=ledger.retained_read_key_at(0).unwrap();let mut foreign=ArtifactHistoryLedger::<Vec<String>>::new();foreign.try_push(vec![]).unwrap();assert!(foreign.retained_read_demands(key).is_err());close_ledger(&mut foreign);
  for _ in 0..cutoff {let funded=grant(ledger.retained_read_demands(key).unwrap());let(step,allocated,released)=observed(||ledger.advance_retained_read(key,&factory,funded).unwrap());receipt(step.progress(),funded,allocated,released);if matches!(step,RetainedCloneStep::Complete(_)){break;}}
  if cutoff>0 {return_original(&mut ledger,key);}assert_eq!(ledger.get_key(slot).unwrap()[0].as_ptr(),pointer);let original=ledger.remove_key(slot).unwrap();close_value(original);ledger.try_push(vec![]).unwrap();assert!(ledger.retained_read_demands(key).is_err());close_ledger(&mut ledger);
 }
 close_factory(factory);
}

#[test]
fn original_history_reads_refuse_unpublished_groups_and_foreign_original_issuers(){
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>=Arc::new(OwnedValueRetirementFactory::<Vec<String>>::default());
 let wrong:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>=Arc::new(OwnedValueRetirementFactory::<Vec<String>>::default());
 let mut ledger=ArtifactHistoryLedger::new();let mut group=super::super::ArtifactGroupVisibilityOwner::new();let visibility=group.view();let reservation=ledger.reserve_group_one(&visibility).unwrap();let slot=ledger.stage_group_reserved(reservation,vec!["actual-original".repeat(1024)],&visibility).unwrap();let key=ArtifactHistoryReadKey{authority:ledger.authority(),issuance:ledger.read_issuance,key:slot};
 assert!(ledger.retained_read_key_at(0).is_none());let(result,allocated,released)=observed(||ledger.retained_read_demands(key));assert!(result.is_err());assert_eq!((allocated,released),(0,0));assert!(group.commit());ledger.adopt_group(&visibility).unwrap();let key=ledger.retained_read_key_at(0).unwrap();
 for _ in 0..10 {let funded=grant(ledger.retained_read_demands(key).unwrap());let(step,allocated,released)=observed(||ledger.advance_retained_read(key,&factory,funded).unwrap());receipt(step.progress(),funded,allocated,released);if matches!(step,RetainedCloneStep::Complete(_)){break;}}
 let funded=grant(ledger.retained_read_demands(key).unwrap());let(result,allocated,released)=observed(||ledger.advance_retained_read(key,&wrong,funded));assert!(result.is_err());assert_eq!((allocated,released),(0,0));return_original(&mut ledger,key);close_ledger(&mut ledger);drop(group);close_value(visibility);close_factory(wrong);close_factory(factory);
}

struct OpaqueProjectedBytes {bytes:Vec<u8>}
struct OriginalProjectedAuthority {original:OpaqueProjectedBytes}
impl semio_framework_value::retirement::RetireOwned for OriginalProjectedAuthority {
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.original.bytes)}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.original.bytes)}
 fn controlled_retirement_supported()->bool{true}
}
#[test]
fn original_projected_source_requires_retirement_only_from_its_actual_authority(){
 use semio_framework_value::retained_clone::RetainedCloneSource;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔗️projection.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let bytes:Vec<u8>=case["bytes"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap() as u8).collect();let pointer=bytes.as_ptr();let mut authority=OriginalProjectedAuthority{original:OpaqueProjectedBytes{bytes}};
  type Source=RetainedCloneSource<OpaqueProjectedBytes,OriginalProjectedAuthority>;
  let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:Source::constructor_copy_bytes(),maximum_capacity_bytes:Source::constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};
  for axis in 0..3{if axis==1&&funded.maximum_copy_bytes==0||axis==2&&funded.maximum_capacity_bytes==0{continue}let mut denied=funded;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,_=>denied.maximum_capacity_bytes-=1};let(result,allocated,released)=observed(||Source::admit_borrowed(authority,|original|&original.original,denied));let(_,returned)=result.err().unwrap();authority=returned;assert_eq!((allocated,released),(0,0));assert_eq!(authority.original.bytes.as_ptr(),pointer);}
  let(result,allocated,released)=observed(||Source::admit_borrowed(authority,|original|&original.original,funded));let(mut source,progress)=result.unwrap_or_else(|_|panic!("original projected source birth"));receipt(progress,funded,allocated,released);assert_eq!(source.borrow().get().bytes.as_ptr(),pointer);assert_eq!(serde_json::to_value(&source.borrow().get().bytes).unwrap(),case["expected"]);
  let mut finished=false;for _ in 0..10000{if source.terminal_is_empty(){finished=true;break;}let funded=grant(source.next_close_demand(4096).unwrap());let(step,allocated,released)=observed(||source.close_step(funded).unwrap());receipt(step.progress(),funded,allocated,released);}assert!(finished);
 }
}
#[test]
fn original_history_reverse_links_borrow_the_same_generation_without_heap_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/⏮️links.json")).unwrap();let mut ledger=ArtifactHistoryLedger::new();let mut keys=Vec::new();let mut pointers=Vec::new();for row in law["rows"].as_array().unwrap(){let original=row.as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect::<Vec<_>>();pointers.push(original.as_ptr());keys.push(ledger.try_push(original).unwrap());}
 let(mut cursor,allocated,released)=observed(||ledger.last_key());assert_eq!((allocated,released),(0,0));for expected in law["expected"].as_array().unwrap(){let index=expected.as_u64().unwrap()as usize;let key=cursor.unwrap();assert_eq!(key,keys[index]);let((pointer,previous),allocated,released)=observed(||(ledger.get_key(key).unwrap().as_ptr(),ledger.previous_key(key)));assert_eq!(pointer,pointers[index]);assert_eq!((allocated,released),(0,0));cursor=previous;}assert!(cursor.is_none());let stale=keys[1];close_value(ledger.remove_key(stale).unwrap());assert!(ledger.previous_key(stale).is_none());assert_eq!(ledger.previous_key(keys[2]),Some(keys[0]));close_ledger(&mut ledger);assert!(ledger.last_key().is_none());
}
