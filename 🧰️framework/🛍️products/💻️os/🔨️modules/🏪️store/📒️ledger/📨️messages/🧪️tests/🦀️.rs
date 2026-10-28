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
}


#[test]
fn measured_diagnostics_replace_the_original_slot_and_return_displaced_owner() {
    let mut ledger = ArtifactEditMessageLedger::new();
    let entry = protocol::EditMessages { edit_id: "edit".into(), messages: Vec::new() };
    let pointer = entry.edit_id.as_ptr();
    assert!(!ledger.admits_measured("edit", ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES + 1));
    assert!(ledger.admit_measured(entry, 4).unwrap().is_none());
    let ticket = ledger.find_ticket("edit").unwrap();
    assert_eq!(ledger.get(ticket).unwrap().edit_id.as_ptr(), pointer);
    let replacement = protocol::EditMessages { edit_id: "edit".into(), messages: Vec::new() };
    let replacement_pointer = replacement.edit_id.as_ptr();
    let displaced = ledger.admit_measured(replacement, 4).unwrap().unwrap();
    assert_eq!(displaced.edit_id.as_ptr(), pointer);
    assert_eq!(ledger.find_ticket("edit"), Some(ticket));
    assert_eq!(ledger.get(ticket).unwrap().edit_id.as_ptr(), replacement_pointer);
    assert_eq!(ledger.len(), 1);
    drop(ledger.remove(ticket).unwrap());
    let mut owner = ArtifactEditMessageLedgerRetirement::new(ledger);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 65536, maximum_release_bytes: usize::MAX, maximum_depth: 16 };
    while !owner.terminal_is_empty() { owner.close_step(grant).unwrap(); }
}

#[test]
fn original_message_retirement_birth_keeps_ledger_until_exact_funded_placement() {
    let law: serde_json::Value = serde_json::from_str(include_str!("📥️birth/🔣️.json")).unwrap();
    let policy = &law["policy"];
    let paid = RetainedCloneGrant { maximum_items: policy["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: policy["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: policy["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: policy["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: policy["maximumDepth"].as_u64().unwrap() as usize };
    let demand = ArtifactEditMessageLedgerRetirement::boxed_birth_demands();
    assert!(demand.copy_bytes <= paid.maximum_copy_bytes && demand.capacity_bytes <= paid.maximum_capacity_bytes);
    let (ledger, input_born, input_freed) = semio_framework_trace_memory_testing::observe_backing(ArtifactEditMessageLedger::new);
    assert_eq!(input_freed, 0);
    let mut original = Some(ledger);
    let mut output = None;
    let pointers = |owner: &ArtifactEditMessageLedger| (owner.slots.pages[0].as_ref().unwrap().as_ptr() as usize, owner.buckets.as_ptr() as usize);
    let expected = pointers(original.as_ref().unwrap());
    for denied in [RetainedCloneGrant { maximum_items: 0, ..paid }, RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..paid }, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..paid }, RetainedCloneGrant { maximum_depth: demand.depth - 1, ..paid }] {
        let (progress, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| ArtifactEditMessageLedgerRetirement::admit_boxed(&mut original, &mut output, denied).unwrap());
        assert_eq!(progress, Default::default()); assert_eq!((born, freed), (0, 0));
        assert_eq!(pointers(original.as_ref().unwrap()), expected); assert!(output.is_none());
    }
    let (progress, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| ArtifactEditMessageLedgerRetirement::admit_boxed(&mut original, &mut output, paid).unwrap());
    assert!(original.is_none()); assert!(output.is_some()); assert_eq!((born, freed), (demand.capacity_bytes, 0)); assert_eq!(progress.retained_capacity_bytes, born);
    let mut all_born = input_born + born; let mut all_freed = 0;
    for _ in 0..1024 {
        if output.is_none() { break; }
        let quoted = semio_framework_value::factory_ticket_demands(output.as_ref().unwrap(), 4096).unwrap();
        assert!(quoted.copy_bytes <= paid.maximum_copy_bytes && quoted.capacity_bytes <= paid.maximum_capacity_bytes);
        let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| semio_framework_value::close_factory_ticket(&mut output, paid).unwrap());
        assert_eq!((born, freed), (step.progress().retained_capacity_bytes, step.progress().released_bytes)); all_born += born; all_freed += freed;
    }
    assert!(output.is_none()); assert_eq!(all_born, all_freed);
}
