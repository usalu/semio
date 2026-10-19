use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
#[test]
fn original_registry_birth_keeps_four_kib_quantum_fifo_and_exact_cumulative_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let mut full_case=0;for cut in law["cancelAt"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()as usize).chain(std::iter::repeat_n(usize::MAX,law["queue"].as_array().unwrap().len())){
  let canceled=std::cell::Cell::new(false);let mut callback=|_:semio_framework_value::NativeDecodeProgress|!canceled.get();let mut recipient=semio_framework_value::NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new_retained(&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();let mut owner=SnapshotReadRegistryBirth::new();let(mut born,mut released)=(0,0);
  for turn in 0..50{if turn==cut||owner.ready().unwrap(){break;}let d=owner.demands().unwrap();assert!(d.copy_bytes<=4096);let g=grant(d);
   for axis in 0..4{let mut short=g;match axis{0=>short.maximum_items=0,1 if d.copy_bytes!=0=>short.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes!=0=>short.maximum_capacity_bytes=d.capacity_bytes-1,3=>short.maximum_depth=0,_=>continue}let admitted=control.owned_bytes();let(step,heap)=observe(||owner.advance(short,&mut control).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(control.owned_bytes(),admitted);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let admitted=control.owned_bytes();let(step,heap)=observe(||owner.advance(g,&mut control).unwrap());assert!(step.progress().fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_eq!(control.owned_bytes()-admitted,heap.requested_bytes);born+=heap.requested_bytes;released+=heap.released_bytes;
  }
  if cut==usize::MAX{assert!(owner.ready().unwrap());let identity=owner.registry.as_ref().unwrap().identity();let(registry,p)=owner.take(grant(Default::default())).unwrap().unwrap();assert!(p.fits(grant(Default::default())));assert_eq!(registry.identity(),identity);
   for row in std::iter::once(&law["queue"][full_case]){let source=Arc::new(37u64);let mut leases=Vec::new();for index in 0..row["issue"].as_u64().unwrap()as usize{let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("original funded registry issue"));assert_eq!(lease.index as usize,index);leases.push((lease.index,lease.generation,Some(lease)));}
    for index in row["return"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()as usize){let(slot,generation,lease)=&mut leases[index];drop(lease.take());drop(registry.try_take(*slot,*generation).unwrap());}
    for expected in row["next"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()as usize){let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("original queue return issue"));assert_eq!(lease.index as usize,expected);leases.push((lease.index,lease.generation,Some(lease)));}
    for(index,generation,lease)in leases{if let Some(lease)=lease{drop(lease);drop(registry.try_take(index,generation).unwrap());}}
   }
   full_case+=1;*owner.retirement=Some(SnapshotReadRegistryAliasRetirement::new(registry));owner.taken=false;
  }
  canceled.set(true);owner.begin_close();for _ in 0..100{if owner.terminal_is_empty(){break;}let g=grant(owner.close_demands().unwrap());let admitted=control.owned_bytes();let(step,heap)=observe(||control.with_retirement(g,|loan|owner.close_step(g,loan)).unwrap());assert_eq!(control.owned_bytes()-admitted,heap.requested_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(g));born+=heap.requested_bytes;released+=heap.released_bytes;}assert!(owner.terminal_is_empty());assert_eq!(born,released);assert_eq!(control.owned_bytes(),born);println!("[DEBUG] Original registry 4096-copy cut={cut} born={born} released={released} original-ledger={}",control.ledger_identity());
 }
}
