use super::*;

use crate::os_store::component::presence_test_retirement::{Factory, CLOSE_GRANT, observed_step, observed_box_close, finish_box};

fn close_publication(original: &mut PresencePeersPublication<i32>) -> usize {
    let mut released = 0;
    for _ in 0..4096 {
        let step = observed_step(CLOSE_GRANT, || original.close_step(CLOSE_GRANT));
        released += step.progress().released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(original.terminal_is_empty()); return released; }
    }
    panic!("original Presence candidate exceeded bounded close turns")
}

fn admit_rejected(original: PresencePeerAdmissionRejected<i32>) -> Option<Box<dyn ErasedSnapshotRetirement>> {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original.into_retirement(CLOSE_GRANT));
    let (owner, receipt) = result.unwrap_or_else(|_| panic!("original rejection requires supplied constructor grant"));
    assert!(receipt.fits(CLOSE_GRANT));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
    Some(owner)
}

#[test]
fn retained_presence_peer_rejection_keeps_its_minting_factory_after_source_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🛂️peer-admission.json")).unwrap();
    let law = &fixture["factoryBinding"];
    let original = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let foreign = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let original_factory: Arc<dyn SnapshotRetirementFactory<i32>> = Arc::new(Factory(original.clone()));
    let foreign_factory: Arc<dyn SnapshotRetirementFactory<i32>> = Arc::new(Factory(foreign.clone()));
    let mut source = PresencePeersPublication::<i32>::new(&Arc::new(PresencePeersRoot::empty()), original_factory.clone());
    let mut other = PresencePeersPublication::<i32>::new(&Arc::new(PresencePeersRoot::empty()), foreign_factory.clone());
    let rejected = source.adopt("exact-owner".into(), 41, 0).err().unwrap();
    assert!(Arc::ptr_eq(rejected.factory.as_ref().unwrap(), &original_factory));
    assert!(!Arc::ptr_eq(rejected.factory.as_ref().unwrap(), &foreign_factory));
    close_publication(&mut source);
    assert!(source.terminal_is_empty());
    drop(source);
    drop(original_factory);
    let mut rejected = admit_rejected(rejected);
    finish_box(&mut rejected);
    assert!(rejected.is_none());
    assert_eq!(serde_json::json!(original.load(std::sync::atomic::Ordering::Relaxed)), law["expectedOriginalRetirements"]);
    assert_eq!(serde_json::json!(foreign.load(std::sync::atomic::Ordering::Relaxed)), law["expectedForeignRetirements"]);
    close_publication(&mut other);
    assert!(other.terminal_is_empty());
}

#[test]
fn retained_presence_peer_admission_preserves_rejected_actor_allocation_and_payload() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🛂️peer-admission.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory = Arc::new(Factory(count.clone()));
        let mut publication = PresencePeersPublication::<i32>::new(&Arc::new(PresencePeersRoot::empty()), factory.clone());
        let state = case["state"].as_str().unwrap();
        if state != "pruning" {
            while publication.prune_one(|_| true).unwrap() {}
        }
        if state == "transferred" {
            let mut transferred = publication.take_commit().unwrap().into_retirement();
            for _ in 0..4096 {
                if matches!(crate::os_store::component::presence_test_retirement::observed_close(&mut transferred, CLOSE_GRANT), RetainedCloneStep::Complete(_)) {
                    break;
                }
            }
            assert!(transferred.terminal_is_empty() && publication.terminal_is_empty());
        }
        let mut seeded_bytes = 0;
        let mut seeded_capacity = 0;
        if matches!(state, "full" | "created-full") {
            for index in 0..PRESENCE_PEER_SLOTS {
                let actor = format!("seed-{index:02}");
                seeded_bytes += actor.len();
                seeded_capacity += actor.capacity();
                assert!(publication.adopt(actor, 0, 0).is_ok());
            }
        }
        let text = case["actor"]["unit"].as_str().unwrap().repeat(case["actor"]["repeat"].as_u64().unwrap() as usize);
        let minimum_capacity = case["actor"]["minimumCapacity"].as_u64().unwrap() as usize;
        let mut actor = String::with_capacity(minimum_capacity);
        actor.push_str(&text);
        let pointer = actor.as_ptr();
        let capacity = actor.capacity();
        assert!(capacity >= minimum_capacity && capacity > fixture["maximumBytes"].as_u64().unwrap() as usize);
        let expected_bytes = case["expectedActorBytes"].as_u64().unwrap() as usize;
        assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&text).unwrap()).unwrap().len(), expected_bytes);
        let mut bytes = 0;
        match publication.adopt(actor, 41, 0) {
            Ok(()) => assert!(case["accepted"].as_bool().unwrap()),
            Err(mut rejected) => {
                assert!(!case["accepted"].as_bool().unwrap());
                assert_eq!(rejected.actor(), text);
                assert_eq!(rejected.actor.as_ptr(), pointer);
                assert_eq!(rejected.actor.capacity(), capacity);
                assert_eq!(*rejected.presence(), 41);
                assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
                let birth = rejected.retirement_birth_demand();
                for currency in fixture["retirementBirthRefusals"].as_array().unwrap() {
                    let denied = match currency.as_str().unwrap() {
                        "items" => RetainedCloneGrant { maximum_items: 0, ..CLOSE_GRANT },
                        "capacity" => RetainedCloneGrant { maximum_capacity_bytes: birth.capacity_bytes - 1, ..CLOSE_GRANT },
                        "depth" => RetainedCloneGrant { maximum_depth: 0, ..CLOSE_GRANT },
                        _ => unreachable!(),
                    };
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| rejected.into_retirement(denied));
                    let (_, original) = result.err().unwrap();
                    rejected = original;
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    assert_eq!(rejected.actor.as_ptr(), pointer);
                    assert_eq!(rejected.actor.capacity(), capacity);
                    assert_eq!(*rejected.presence(), 41);
                    assert!(Arc::ptr_eq(rejected.factory.as_ref().unwrap(), &(factory.clone() as Arc<dyn SnapshotRetirementFactory<i32>>)));
                }
                let mut rejected = admit_rejected(rejected);
                assert_eq!(observed_box_close(&mut rejected, RetainedCloneGrant { maximum_items: 0, ..CLOSE_GRANT }), RetainedCloneStep::Progress(Default::default()));
                let release_demand = rejected.as_ref().unwrap().next_release_byte_demand().unwrap();
                assert_eq!(release_demand, capacity);
                assert_eq!(observed_box_close(&mut rejected, RetainedCloneGrant { maximum_release_bytes: 4096, ..CLOSE_GRANT }), RetainedCloneStep::Progress(Default::default()));
                assert_eq!(rejected.as_ref().unwrap().next_release_byte_demand().unwrap(), capacity);
                assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
                bytes += finish_box(&mut rejected);
                assert!(rejected.is_none());
                assert!(bytes >= capacity);
            }
        }
        bytes += close_publication(&mut publication);
        assert!(publication.terminal_is_empty());
        assert!(bytes >= capacity + seeded_capacity);
        assert_eq!(text.len(), expected_bytes);
        assert_eq!(seeded_bytes, if matches!(state, "full" | "created-full") { PRESENCE_PEER_SLOTS * "seed-00".len() } else { 0 });
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 1);
        eprintln!("[DEBUG] peer admission case={} retained actor capacity={capacity}, semantic UTF8 bytes={expected_bytes}, physical close receipt={bytes}, exact target snapshots=1", case["name"]);
    }
}
