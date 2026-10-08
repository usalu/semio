//! 🎟️ Original native scaffolds and pending replay frames close under distinct actual grants.
use super::*;
use semio_framework_value::retirement::{OwnedValueRetirementFactory,controlled::ControlledRetirement};
use semio_framework_trace::observe_heap_allocations_on_this_thread;

fn grant(owner:&dyn ErasedSnapshotRetirement)->RetainedCloneGrant{let copy=owner.next_copy_byte_demand().unwrap();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}}
fn observe(work:impl FnOnce()->Result<RetainedCloneStep,ValueError>,grant:RetainedCloneGrant)->RetainedCloneProgress{let(result,heap)=observe_heap_allocations_on_this_thread(work);let progress=result.unwrap().progress();assert!(progress.fits(grant));assert!(progress.copied_bytes+progress.retained_capacity_bytes<=4096);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));progress}

#[test]
fn cooperative_replay_capacity_release_obeys_the_neutral_law(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧫️fixtures/🔣️.json")).unwrap();let law=&law["capacityRelease"];
    let mut owners=Vec::<u8>::new();owners.try_reserve_exact(law["emptySlots"].as_u64().unwrap()as usize).unwrap();let capacity=owners.capacity();let body=law["bodyBytes"].as_u64().unwrap()as usize;
    let(mut owner,heap)=observe_heap_allocations_on_this_thread(||ControlledRetirement::new(owners).unwrap_or_else(|(error,_)|panic!("original vector refused: {error}")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let mut exact=false;
    for turn in 0..10000{
        if owner.terminal_is_empty(){break;}let current=grant(&owner);
        assert_eq!(observe(||owner.step(RetainedCloneGrant{maximum_items:0,..current}),RetainedCloneGrant{maximum_items:0,..current}),Default::default());
        if current.maximum_release_bytes==capacity{
            let ordinary=RetainedCloneGrant{maximum_release_bytes:body,..current};assert_eq!(observe(||owner.step(ordinary),ordinary)!=Default::default(),law["ordinaryGrantAccepted"].as_bool().unwrap());
            let short=RetainedCloneGrant{maximum_release_bytes:capacity-1,..current};assert_eq!(observe(||owner.step(short),short),Default::default());
            assert_eq!(observe(||owner.step(current),current).released_bytes,capacity);exact=true;
        }else{observe(||owner.step(current),current);}
        assert!(turn<9999);
    }
    assert!(exact&&owner.terminal_is_empty());assert!(law["exactGrantAccepted"].as_bool().unwrap());let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    println!("[DEBUG] replay original empty Vec capacity={capacity} ordinaryBody={body} exact physical release=true");
}

#[test]
fn cooperative_replay_paged_inverse_preserves_semantic_array_and_releases_each_page(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧫️fixtures/🔣️.json")).unwrap();let law=&law["pagedInverse"];let count=law["rows"].as_u64().unwrap()as usize;let maximum=law["maximumTurnBytes"].as_u64().unwrap()as usize;
    let expected:Vec<u8>=(0..count).map(|index|(index%256)as u8).collect();let owners=PagedList::<u8,{usize::MAX}>::try_from_iter(expected.iter().copied()).unwrap();assert_eq!(serde_json::to_value(&owners).unwrap(),serde_json::to_value(&expected).unwrap());
    let factory=Arc::new(OwnedValueRetirementFactory::<u8>::default());let(mut owner,heap)=observe_heap_allocations_on_this_thread(||ReplayMutationsRetirement::new(owners,factory));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut releases=0;
    for turn in 0..100000{if owner.terminal_is_empty(){break;}let current=grant(&owner);assert!(current.maximum_copy_bytes+current.maximum_capacity_bytes<=maximum&&current.maximum_release_bytes<=maximum);let progress=observe(||owner.close_step(current),current);releases+=usize::from(progress.released_bytes>0);assert!(turn<99999);}
    assert!(owner.terminal_is_empty());assert!(releases>=law["minimumBackingReleases"].as_u64().unwrap()as usize);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] native paged inverse rows={count} admitted physical releases={releases}");
}

#[test]
fn cooperative_replay_pending_native_lanes_birth_and_release_each_actual_child(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for vector in law["cases"].as_array().unwrap(){
        let factory=registered_replay_retirement_factory(Arc::new(OwnedValueRetirementFactory::<u8>::default()),Arc::new(OwnedValueRetirementFactory::<u8>::default()));
        let(mut owner,heap)=observe_heap_allocations_on_this_thread(||ReplayRetirement::<u8,u8>::new(factory));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        if vector["snapshot"].as_bool().unwrap(){owner.stage_snapshot(Arc::new(7));}if vector["inverse"].as_bool().unwrap(){owner.stage_mutations(PagedList::try_from_iter(0u8..64).unwrap());}
        for turn in 0..10000{if owner.terminal_is_empty(){break;}let((copy,capacity,release,depth),heap)=observe_heap_allocations_on_this_thread(||{let copy=owner.next_copy_byte_demand().unwrap();(copy,owner.next_capacity_byte_demand(copy).unwrap(),owner.next_release_byte_demand().unwrap(),owner.next_depth_demand().unwrap())});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let current=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};assert!(copy+capacity<=4096);
            for short in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_depth:depth.saturating_sub(1),..current},RetainedCloneGrant{maximum_copy_bytes:copy.saturating_sub(1),..current},RetainedCloneGrant{maximum_capacity_bytes:capacity.saturating_sub(1),..current},RetainedCloneGrant{maximum_release_bytes:release.saturating_sub(1),..current}]{if short!=current{assert_eq!(observe(||owner.close_step(short),short),Default::default());}}
            observe(||owner.close_step(current),current);assert!(turn<9999);
        }
        assert!(owner.terminal_is_empty());let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] native replay two-lane original custody closed vector={vector}");
    }
}
