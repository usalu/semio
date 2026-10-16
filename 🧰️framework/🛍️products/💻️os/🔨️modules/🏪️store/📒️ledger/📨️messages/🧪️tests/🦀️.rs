//! 🧪️ Actual original message ledger allocation and paid physical retirement.
use super::*;
#[test]
fn current_message_ledger_retirement_pays_original_pages_and_arrays() {
    let plain: serde_json::Value = serde_json::from_str(include_str!("../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let row = &plain["messageLedgerBirth"]; let supplied = &row["policy"];
    let paid = RetainedCloneGrant { maximum_items: supplied["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: supplied["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: supplied["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: supplied["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: supplied["maximumDepth"].as_u64().unwrap() as usize };
    let (ledger, born, freed) = semio_framework_trace_memory_testing::observe_backing(ArtifactEditMessageLedger::new);
    assert_eq!(freed, 0);
    let expected = ledger.slots.pages.iter().map(|page|std::mem::size_of_val(page.as_ref().unwrap().as_ref())).sum::<usize>()+std::mem::size_of_val(ledger.generations.as_ref())+std::mem::size_of_val(ledger.occupied.as_ref())+std::mem::size_of_val(ledger.next.as_ref())+std::mem::size_of_val(ledger.previous.as_ref())+std::mem::size_of_val(ledger.buckets.as_ref())+std::mem::size_of_val(ledger.free.as_ref());
    assert_eq!(born, expected);
    let pointers = |owner: &ArtifactEditMessageLedgerRetirement| {
        let pages:[usize;ARTIFACT_EDIT_MESSAGE_SLOT_PAGES]=std::array::from_fn(|index|owner.original.slots.pages[index].as_ref().map_or(0,|page|page.as_ptr()as usize));
        (pages,[owner.original.generations.as_ptr()as usize,owner.original.occupied.as_ptr()as usize,owner.original.next.as_ptr()as usize,owner.original.previous.as_ptr()as usize,owner.original.buckets.as_ptr()as usize,owner.original.free.as_ptr()as usize])
    };
    let mut owner = ArtifactEditMessageLedgerRetirement::new(ledger);
    let turns = row["capacitySlots"].as_u64().unwrap() / row["pageSlots"].as_u64().unwrap() + 6;
    let mut total_freed = 0;
    for turn in 0..turns {
        let original = pointers(&owner);
        let demand = owner.demands(0).unwrap();
        assert!(demand.release_bytes > 0 && demand.release_bytes <= paid.maximum_release_bytes);
        for denied in [RetainedCloneGrant { maximum_items: 0, ..paid }, RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..paid }, RetainedCloneGrant { maximum_depth: 0, ..paid }] {
            let (result, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(denied));
            if let Ok(step) = result { assert_eq!(step.progress(), Default::default()); }
            assert_eq!((allocated, released), (0, 0)); assert_eq!(pointers(&owner), original);
        }
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(paid).unwrap());
        assert!(step.progress().fits(paid));
        assert_eq!(allocated, 0); assert_eq!(released, demand.release_bytes);
        assert_eq!(step.progress().released_bytes, released);
        total_freed += released; assert_eq!(owner.terminal_is_empty(), turn == turns - 1);
    }
    assert_eq!(total_freed, expected); assert!(owner.original.backing_is_empty());
    let (_, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| drop(owner)); assert_eq!((born, freed), (0, 0));
    eprintln!("[DEBUG] original message ledger: 128page/6array exact frees, fixed policy, refusal retains every pointer, allocator release={total_freed}");
}

