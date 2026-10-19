//! 🏛️ The Store's history ledger retirement returns original read sources and waits for live projection leases.
use super::ArtifactStoreHistoryLedgerRetirement;
use semio_framework_os_vcs::ArtifactHistoryLedger;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryAuthority,FactoryRetirement,RetirementDemand,retirement::OwnedValueRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedOwnedProjection};
use std::sync::Arc;

#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

fn grant(demand:RetirementDemand)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn observed<T>(call:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(call);assert!(!heap.overflowed);(value,heap.requested_bytes,heap.released_bytes)}
fn receipt(progress:RetainedCloneProgress,grant:RetainedCloneGrant,allocated:usize,released:usize){assert!(progress.fits(grant));assert!(allocated<=progress.retained_capacity_bytes);assert!(released<=progress.released_bytes);}
fn close_projection(projection:&mut RetainedOwnedProjection<String>){for _ in 0..10000 {if projection.terminal_is_empty(){return;}let funded=grant(projection.close_demand(4096).unwrap());let(step,allocated,released)=observed(||projection.close_step(funded).unwrap());receipt(step.progress(),funded,allocated,released);}panic!("original operation projection did not close");}
fn close_factory(factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>){let factory:Arc<dyn FactoryRetirement>=factory;let mut owner=FactoryAuthority::new(factory);for _ in 0..10000 {if owner.terminal_is_empty(){return;}let funded=grant(owner.demands(4096).unwrap());let(step,allocated,released)=observed(||owner.step(funded).unwrap());receipt(step.progress(),funded,allocated,released);}panic!("captured original history issuer did not close");}
#[test]
fn original_history_store_retirement_returns_read_sources_and_waits_for_live_leases(){
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<String>>>=Arc::new(OwnedValueRetirementFactory::<Vec<String>>::default());
 for cutoff in 0..3 {for keep_live in [false,true]{let mut ledger=ArtifactHistoryLedger::new();ledger.try_push(vec!["original-stored-forward".repeat(1024)]).unwrap();let key=ledger.retained_read_key_at(0).unwrap();for _ in 0..cutoff {let funded=grant(ledger.retained_read_demands(key).unwrap());let(step,allocated,released)=observed(||ledger.advance_retained_read(key,&factory,funded).unwrap());receipt(step.progress(),funded,allocated,released);if matches!(step,RetainedCloneStep::Complete(_)){break;}}
  let mut projection=if keep_live&&cutoff==2{let source=ledger.retained_read(key).unwrap().project(1,|rows|&rows[0]);let funded=grant(RetainedOwnedProjection::<String>::constructor_demand());Some(source.retain_projection(funded).unwrap().0)}else{None};
  let mut owner=ArtifactStoreHistoryLedgerRetirement::new(ledger,Arc::clone(&factory));let mut saw_wait=projection.is_none();let mut finished=false;
  for _ in 0..100000 {if owner.terminal_is_empty(){finished=true;break;}let demand=owner.next_demand(4096).unwrap();let funded=grant(demand);
   for axis in 0..5 {let mut denied=funded;match axis {0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if demand.release_bytes>0=>denied.maximum_release_bytes-=1,4 if demand.depth>0=>denied.maximum_depth-=1,_=>continue};let(result,allocated,released)=observed(||owner.close_step(denied));assert_eq!((allocated,released),(0,0));match result {Ok(step)=>assert_eq!(step.progress(),RetainedCloneProgress::default()),Err(error)=>assert_eq!(error.retained_progress(),RetainedCloneProgress::default())};assert_eq!(owner.next_demand(4096).unwrap(),demand);}
   let(step,allocated,released)=observed(||owner.close_step(funded).unwrap());receipt(step.progress(),funded,allocated,released);if step.progress()==RetainedCloneProgress::default()&&projection.is_some(){assert_eq!(projection.as_ref().unwrap().borrow().unwrap().get().len(),"original-stored-forward".len()*1024);close_projection(projection.as_mut().unwrap());projection=None;saw_wait=true;}
  }
  assert!(finished&&saw_wait);assert!(projection.is_none());
 }}close_factory(factory);
}
