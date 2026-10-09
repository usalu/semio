use super::*;

define_id!(TestId, "test");
#[path="../🎟️growth/🦀️.rs"]
mod admitted_growth;

/// 🧱️ Every original slot keeps its borrowed address when the original arena source grows.
#[test]
fn original_arena_growth_preserves_borrowed_original_slot_addresses(){
    use crate::brep::queries::tessellation::tests::observe_tessellation_system as observe;
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🕸️topology/🧫️fixtures/🎟️reachability/🔣️.json")).unwrap();let row=&law["arenaGrowth"];
    let mut store=Store::<u64,TestId>::new();let mut originals:[Option<(TestId,*const u64)>;1337]=[None;1337];let(mut born,mut freed)=(0,0);
    for slot in 0..row["slots"].as_u64().unwrap() as usize{let(id,heap)=observe(||store.insert(slot as u64));born+=heap.0;freed+=heap.1;originals[slot]=Some((id,store.get(id).unwrap() as *const u64));for (id,pointer) in originals[..slot].iter().flatten(){assert_eq!(store.get(*id).unwrap() as *const u64,*pointer,"original arena growth at slot {slot} moved a prior original slot; actualSystem={heap:?}");}}
    let mut owner=ControlledRetirement::new(store).unwrap_or_else(|_|panic!("original arena growth requires typed closure"));let mut turns=0;
    while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:row["itemsPerTurn"].as_u64().unwrap() as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);}assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,row["terminalDropBytes"].as_u64().unwrap() as usize));eprintln!("[DEBUG] Original arena growth slots=1337 sameOriginalSlots=true physical={freed} turns={turns} terminalDrop=0");
}

/// 🗄️ Original arena removal and LIFO reuse preserve the exact slot backing without an unfunded directory copy.
#[test]
fn original_arena_removal_reuses_same_slots_without_backing_effects(){
    use crate::brep::queries::tessellation::tests::observe_tessellation_system as observe;
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🕸️topology/🧫️fixtures/🎟️reachability/🔣️.json")).unwrap();let row=&law["arenaRemoval"];
    for copy in [1,3,64]{
        let((mut store,mut ids),source)=observe(||{let mut store=Store::<u64,TestId>::new();let ids:[TestId;64]=std::array::from_fn(|slot|store.insert(slot as u64));(store,ids)});let original=&store.slots[0] as *const _;let(mut born,mut freed)=(source.0,source.1);
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
        for generation in 1..=row["cycles"].as_u64().unwrap(){
            let mut stale=[ids[0];6];for(index,slot)in row["removeOrder"].as_array().unwrap().iter().enumerate(){let slot=slot.as_u64().unwrap()as usize;let id=ids[slot];stale[index]=id;assert_eq!(store.remove_backing_demand(id),(0,0));
                let(result,heap)=observe(||store.remove_granted(id,RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert!(result.0.is_none());assert_eq!(result.1,Default::default());assert_eq!(heap,(0,0));assert_eq!(store.get(id),Some(&(slot as u64)));
                let(result,heap)=observe(||store.remove_granted(id,grant).unwrap());assert_eq!(result.0,Some(slot as u64));assert_eq!(result.1.retained_capacity_bytes,0);assert_eq!(result.1.released_bytes,0);assert_eq!(result.1.copied_bytes,0);assert_eq!(result.1.copied_items,1);assert_eq!(heap,(0,0));assert_eq!(&store.slots[0] as *const _,original);
            }
            for slot in row["reuseOrder"].as_array().unwrap(){let slot=slot.as_u64().unwrap()as usize;let(id,heap)=observe(||store.insert(slot as u64));assert_eq!(heap,(0,0));assert_eq!(id.raw_index(),slot as u32);assert_eq!(id.raw_generation(),generation as u32);ids[slot]=id;assert_eq!(&store.slots[0] as *const _,original);}for id in stale{assert!(store.get(id).is_none());}
        }
        let(mut owner,heap)=observe(||ControlledRetirement::new(store).unwrap_or_else(|(error,_)|panic!("original arena close: {error}")));assert_eq!(heap,(0,0));let mut turns=0;while !owner.terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);}assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));eprintln!("[DEBUG] Original arena removal copy={copy} cycles=32 sameOriginalSlots=true removalBackingEffects=0 physical={freed} turns={turns} terminalDrop=0");
    }
}

#[semio_framework_async_macros::async_test]
async fn insert_and_get_round_trips() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(42);
    assert_eq!(store.get(id), Some(&42));
}

#[semio_framework_async_macros::async_test]
async fn remove_then_get_returns_none() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(1);
    assert_eq!(store.remove(id), Some(1));
    assert_eq!(store.get(id), None);
}

#[semio_framework_async_macros::async_test]
async fn stale_handle_after_reuse_returns_none() {
    let mut store: Store<i32, TestId> = Store::new();
    let a = store.insert(1);
    store.remove(a);
    let b = store.insert(2);
    assert_eq!(b.raw_index(), a.raw_index(), "the freed slot should be reused (LIFO free list)");
    assert_ne!(b.raw_generation(), a.raw_generation());
    assert_eq!(store.get(a), None, "the stale handle must not alias the new value");
    assert_eq!(store.get(b), Some(&2));
}

#[semio_framework_async_macros::async_test]
async fn iteration_is_index_ordered_and_skips_removed_slots() {
    let mut store: Store<i32, TestId> = Store::new();
    let a = store.insert(10);
    let _b = store.insert(20);
    let c = store.insert(30);
    store.remove(a);
    let collected: Vec<(TestId, i32)> = store.iter().map(|(id, v)| (id, *v)).collect();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].1, 20);
    assert_eq!(collected[1].1, 30);
    assert!(collected[1].0.raw_index() == c.raw_index());
}

#[semio_framework_async_macros::async_test]
async fn len_reflects_only_live_entries() {
    let mut store: Store<i32, TestId> = Store::new();
    let a = store.insert(1);
    store.insert(2);
    assert_eq!(store.len(), 2);
    store.remove(a);
    assert_eq!(store.len(), 1);
    assert!(!store.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn free_bumps_generation_and_is_live_reflects_it() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(7);
    assert!(store.is_live(id));
    assert!(store.free(id));
    assert!(!store.is_live(id));
    assert!(!store.free(id), "freeing an already-stale id is a no-op, not a panic");
}

#[semio_framework_async_macros::async_test]
async fn display_uses_readable_tag_index_generation_format() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(1);
    assert_eq!(id.to_string(), format!("test-{}-{}", id.raw_index(), id.raw_generation()));
}

#[semio_framework_async_macros::async_test]
async fn serde_round_trips_an_id() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(1);
    let json = semio_framework_pack_json::to_json_string(&id);
    let back: TestId = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(back, id);
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn random_insert_remove_sequence_never_aliases_a_removed_id() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(83);
        let mut store: Store<u64, TestId> = Store::new();
        let mut live: Vec<(TestId, u64)> = Vec::new();
        let mut removed: Vec<TestId> = Vec::new();
        for i in 0..2000u64 {
            if !live.is_empty() && rng.next_bool(0.4) {
                let idx = rng.next_range(0, live.len() as u64) as usize;
                let (id, _) = live.remove(idx);
                store.remove(id);
                removed.push(id);
            } else {
                let id = store.insert(i);
                live.push((id, i));
            }
        }
        for (id, value) in &live {
            assert_eq!(store.get(*id), Some(value));
        }
        for id in &removed {
            if !live.iter().any(|(lid, _)| lid == id) {
                assert_eq!(store.get(*id), None, "removed id {id:?} must not resolve");
            }
        }
    }
}
