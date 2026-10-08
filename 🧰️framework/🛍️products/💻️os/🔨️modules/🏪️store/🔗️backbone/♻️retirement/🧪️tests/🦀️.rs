use super::*;

#[path = "🎟️admit/🦀️.rs"]
mod admission;

#[test]
fn backbone_retirement_preserves_original_empty_capacities_and_exact_release() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut queue = VecDeque::with_capacity(law["emptyCapacity"].as_u64().unwrap() as usize);
    let mut ids = Vec::with_capacity(113);
    let mut empty_id = String::with_capacity(8192);
    empty_id.push_str("");
    ids.push(empty_id);
    ids.push("雪\0🌍".into());
    queue.push_back(BackboneMessage::Genesis { pack: vec![0; 65536] });
    queue.push_back(BackboneMessage::Ack { op_ids: ids });
    queue.push_back(BackboneMessage::Member { owner: String::with_capacity(97), slot: "tree".into(), child_id: String::with_capacity(71), envelopes: Vec::with_capacity(8192) });
    queue.push_back(BackboneMessage::Retract { mutation_ids: Vec::with_capacity(31) });
    let (mut owner, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::from_queue(queue));
    assert_eq!((birth.requested_bytes, birth.released_bytes), (0, 0));
    let mut physical_release = 0;
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.demands().unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let mut pauses = vec![RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }];
        if demand.release_bytes > 0 { pauses.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
        for pause in pauses {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(pause).unwrap());
            assert_eq!(step.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes);
        assert_eq!(heap.released_bytes, step.progress().released_bytes);
        assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes <= 4096);
        physical_release += heap.released_bytes;
        turns += 1;
        assert!(turns < 100_000);
    }
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert!(physical_release > 65536 + 8192 * size_of::<BackboneMessage>());
    eprintln!("[DEBUG] backbone original empty String/Vec/queue capacities retired turns={turns} released={physical_release}");
}
