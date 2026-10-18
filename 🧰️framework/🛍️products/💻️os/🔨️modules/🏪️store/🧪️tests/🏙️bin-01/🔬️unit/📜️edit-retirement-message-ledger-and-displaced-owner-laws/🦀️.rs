/// 📜️ Edit retirement, message ledger and displaced owner laws: the interrupted and rejected edit retirement, the history metadata retirement, the fixed edit message ledger, the unit owned retirement and the displaced owner reservations.

#[test]
fn artifact_store_edit_retirement_is_interruptible_and_terminal_empty_after_deep_strings() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["editRetirement"];
    let strings = law["strings"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize)).collect::<Vec<_>>();
    let mut inverse = semio_framework_value::list::PagedList::new(); inverse.try_push(DemoMutation::SetN(SetN { n: 7 })).unwrap(); drop(inverse.pop());
    assert!(!inverse.terminal_is_empty());
    let forwards = Vec::with_capacity(law["forwardBackingSlots"].as_u64().unwrap() as usize); let forward_pointer = forwards.as_ptr();
    let edit = Edit::<DemoMutation> { line: None, id: strings[0].clone(), actor: Some(strings[1].clone().into()), forwards, inverse, mutation_meta: Vec::with_capacity(law["metadataBackingSlots"].as_u64().unwrap() as usize), verb: None, sequence_number: 1, started_at: strings[2].clone(), finished_at: Some(strings[3].clone()) };
    let payload_bytes = strings.iter().map(String::len).sum::<usize>();
    let (result, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| ArtifactStoreEditRetirement::new(edit));
    let mut retirement = result.unwrap_or_else(|_| panic!("empty original mutation lanes admitted"));
    assert_eq!((allocated, released), (0, law["constructorDropBytes"].as_u64().unwrap() as usize));
    assert_eq!(retirement.forwards.as_ref().unwrap().as_ptr(), forward_pointer);
    let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize; let mut physical_release = 0;
    for _ in 0..10_000 {
        if retirement.terminal_is_empty() {
            assert!(physical_release >= payload_bytes);
            let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(retirement));
            assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
             return;
        }
        let demand = retirement.demands(copy).unwrap();
        let grant = RetainedCloneGrant { maximum_copy_bytes: copy.max(demand.copy_bytes), ..physical_test_close_grant() };
        assert!(demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth);
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| retirement.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
        assert_eq!((allocated, released), (0, 0)); assert_eq!(step.progress(), RetainedCloneProgress::default()); assert_eq!(retirement.demands(copy).unwrap(), demand);
        if demand.release_bytes != 0 {
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| retirement.close_step(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }).unwrap());
            assert_eq!((allocated, released), (0, 0)); assert_eq!(step.progress(), RetainedCloneProgress::default()); assert_eq!(retirement.demands(copy).unwrap(), demand);
        }
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| retirement.close_step(grant).unwrap());
        assert!(step.progress().fits(grant)); assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes)); physical_release += released;
    }
    panic!("original edit retirement must make finite progress across interruptions");
}

#[test]
fn artifact_store_edit_retirement_rejects_and_returns_the_exact_nonempty_mutation_owner() {
    let edit = Edit::<DemoMutation> {
        line: None,
        id: "edit-owner".into(),
        actor: None,
        forwards: vec![DemoMutation::SetN(SetN { n: 7 })],
        inverse: Vec::new().into(),
        mutation_meta: Vec::new(),
        verb: None,
        sequence_number: 1,
        started_at: "now".into(),
        finished_at: None,
    };
    let rejected = match ArtifactStoreEditRetirement::new(edit) {
        Err(rejected) => rejected,
        Ok(_) => panic!("a nonempty mutation vector must remain with its exact domain owner"),
    };
    assert_eq!(rejected.forwards, vec![DemoMutation::SetN(SetN { n: 7 })]);
}

#[test]
fn artifact_store_history_metadata_retirement_cursors_authors_pins_and_ids_under_each_grant() {
    let checkpoint = Checkpoint {
        id: "checkpoint".repeat(64),
        change_ids: vec!["change-a".repeat(64), "change-b".repeat(64)],
        parent_id: Some("parent".repeat(64)),
        authors: vec![Author { id: "author".repeat(64), name: "name".repeat(64), avatar: Some("avatar".repeat(64)) }],
        message: Some("message".repeat(64)),
        timestamp: "timestamp".repeat(64),
        composition_pins: vec![semio_framework_os_vcs::CompositionPin {
            child_ref: semio_framework_artifact_reference::ArtifactRef { artifact_id: "child".repeat(64), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "kind".repeat(64), standard: "standard".repeat(64), subset: "subset".repeat(64) } },
            checkpoint_id: "child-checkpoint".repeat(64),
        }],
    };
    let mut retirement = ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint);
    let grant=RetainedCloneGrant { maximum_copy_bytes:11,..physical_test_close_grant() };
    for _ in 0..10_000 {
        if retirement.terminal_is_empty() { return; }
        let demand=retirement.next_release_byte_demand().unwrap();
        if demand>11 {
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirement.close_step(RetainedCloneGrant { maximum_release_bytes:11,..grant }).unwrap());
            assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(retirement.next_release_byte_demand().unwrap(),demand);
        }
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirement.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        assert!(retirement.terminal_is_empty() || step.progress()!=RetainedCloneProgress::default(),"funded owned metadata makes genuine progress without an external wait");
    }
    panic!("history metadata must terminate across interrupted supplied grants");
}

#[test]
fn fixed_edit_message_ledger_resolves_probe_collisions_without_a_parallel_runtime_index() {
    let mut first_by_slot = BTreeMap::new();
    let mut collision = None;
    for index in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY * 2 {
        let id = format!("edit-collision-{index}");
        let slot = ArtifactEditMessageLedger::hash(&id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        if let Some(first) = first_by_slot.insert(slot, id.clone()) {
            collision = Some((first, id));
            break;
        }
    }
    let (first, second) = collision.expect("pigeonhole probe collision exists");
    let entries = vec![
        semio_framework_os_spr::EditMessages { edit_id: first.clone(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "first")] },
        semio_framework_os_spr::EditMessages { edit_id: second.clone(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "second")] },
    ];
    let mut ledger = ArtifactEditMessageLedger::try_from_entries(entries).unwrap_or_else(|_| panic!("colliding fixed entries probe deterministically"));
    assert_eq!(ledger.get_by_id(&first).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("first"));
    assert_eq!(ledger.get_by_id(&second).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("second"));
    while let Some(entry) = ledger.pop() {
        let mut retirement = ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages);
        while !retirement.terminal_is_empty() {
            let grant=physical_test_close_grant();
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirement.close_step(grant).expect("colliding owner declared close"));
            assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        }
    }
}

fn drain_rejected_edit_message_ledger(mut rejected: ArtifactEditMessageLedgerRejected) {
    let grant=physical_test_close_grant();
    for _ in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY*32 {
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||rejected.close_step(grant).expect("rejected ledger declared close"));
        assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        match step {
            RetainedCloneStep::Complete(_) => { assert!(rejected.terminal_is_empty());return; }
            RetainedCloneStep::Progress(progress) => assert!(progress!=RetainedCloneProgress::default(),"original rejected ledger has no external wait; declared policy must advance its original owner"),
        }
    }
    panic!("rejected ledger close must terminate under repeated supplied grants");
}

#[test]
fn fixed_edit_message_ledger_rejects_duplicate_and_capacity_plus_one_with_exact_retained_owners() {
    let duplicate = vec![
        semio_framework_os_spr::EditMessages { edit_id: "duplicate".into(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "first")] },
        semio_framework_os_spr::EditMessages { edit_id: "duplicate".into(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "second")] },
    ];
    let duplicate = match ArtifactEditMessageLedger::try_from_entries(duplicate) {
        Err(rejected) => rejected,
        Ok(mut ledger) => {
            while ledger.pop().is_some() {}
            panic!("duplicate identities must fail before payload adoption")
        }
    };
    assert_eq!(duplicate.retained_entries().len(), 2, "duplicate rejection retains the exact owner batch");
    drain_rejected_edit_message_ledger(duplicate);

    let plus_one = (0..=ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).map(|index| semio_framework_os_spr::EditMessages { edit_id: format!("edit-{index}"), messages: Vec::new() }).collect();
    let plus_one = match ArtifactEditMessageLedger::try_from_entries(plus_one) {
        Err(rejected) => rejected,
        Ok(mut ledger) => {
            while ledger.pop().is_some() {}
            panic!("capacity plus one must fail before payload adoption")
        }
    };
    assert_eq!(plus_one.retained_entries().len(), ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY + 1, "capacity rejection retains every exact owner");
    drain_rejected_edit_message_ledger(plus_one);
}

#[test]
fn fixed_edit_message_ledger_preserves_order_and_rejects_stale_generation_after_slot_reuse() {
    let entries =
        vec![semio_framework_os_spr::EditMessages { edit_id: "first".into(), messages: Vec::new() }, semio_framework_os_spr::EditMessages { edit_id: "middle".into(), messages: Vec::new() }, semio_framework_os_spr::EditMessages { edit_id: "last".into(), messages: Vec::new() }];
    let mut ledger = match ArtifactEditMessageLedger::try_from_entries(entries) {
        Ok(ledger) => ledger,
        Err(rejected) => {
            drain_rejected_edit_message_ledger(rejected);
            panic!("valid fixed ledger must be admitted")
        }
    };
    let middle = ledger.find_ticket("middle").expect("middle ticket");
    let removed = ledger.remove(middle).expect("middle owner detaches in O(1)");
    assert_eq!(removed.edit_id, "middle");
    let reused = ledger.try_push(semio_framework_os_spr::EditMessages { edit_id: "replacement".into(), messages: Vec::new() }).unwrap_or_else(|_| panic!("released slot is reusable"));
    assert_eq!(reused.slot, middle.slot);
    assert_ne!(reused.generation, middle.generation);
    assert!(ledger.get(middle).is_none(), "stale generation cannot resolve the reused slot");
    assert_eq!(ledger.iter().map(|entry| entry.edit_id.as_str()).collect::<Vec<_>>(), vec!["first", "last", "replacement"]);
    let removed = ArtifactStoreMessageLedgerRetirement::new(removed.edit_id, removed.messages);
    let mut removed = removed;
    while !removed.terminal_is_empty() {
        let grant=RetainedCloneGrant {maximum_copy_bytes:ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES,..physical_test_close_grant()};let step=removed.close_step(grant).expect("original message owner full-policy close");assert!(step.progress().fits(grant));
    }
    while let Some(entry) = ledger.pop() {
        let mut retirement = ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages);
        while !retirement.terminal_is_empty() {
                let grant=RetainedCloneGrant {maximum_copy_bytes:ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES,..physical_test_close_grant()};let step=retirement.close_step(grant).expect("original message owner full-policy close");assert!(step.progress().fits(grant));
        }
    }
    assert!(ledger.terminal_is_empty());
}

struct UnitOwnedRetirement {
    value: Option<()>,
}

impl ErasedSnapshotRetirement for UnitOwnedRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original unit payload requires supplied depth")); }
        self.value.take();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,..Default::default() }))
    }
    fn terminal_is_empty(&self) -> bool { self.value.is_none() }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.value.is_some()))}
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct UnitOwnedRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<()> for UnitOwnedRetirementFactory {
    fn retirement_birth_bytes(&self,_:&())->usize { std::mem::size_of::<UnitOwnedRetirement>() }
    fn retire_owned(&self, value: (), grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,())> {
        admit_artifact_retirement(value,grant,|original|UnitOwnedRetirement { value: Some(original) })
    }
}

#[test]
fn displaced_owner_reservations_preserve_capacity_generation_and_interrupted_close() {
    let mut retirements = ArtifactStoreDisplacedRetirements::new();
    let large = retirements.reserve_owner_slots(ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY - 1).expect("exact capacity minus one is reserved");
    let mut final_slot = retirements.reserve_owner_slots(1).expect("the final exact retirement slot is independently reserved");
    assert!(retirements.reserve_owner_slots(1).is_err(), "capacity +1 cannot consume an owner");
    retirements.release_owner_slots(large).expect("unused large reservation returns atomically");
    let grant=physical_test_close_grant();
    let(owner,birth)=UnitOwnedRetirementFactory.retire_owned((),grant).expect("original unit cursor admission is funded");
    assert_eq!(birth.retained_capacity_bytes,std::mem::size_of::<UnitOwnedRetirement>());
    retirements.push_owner_reserved(&mut final_slot, owner).unwrap_or_else(|_| panic!("the exact final-slot reservation retains its owner"));
    retirements.release_owner_slots(final_slot).expect("consumed reservation has no live credit");
    assert_eq!(retirements.close_step(RetainedCloneGrant { maximum_items: 0,..grant }).unwrap().progress(),RetainedCloneProgress::default());
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.close_step(grant).unwrap());
    assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress { copied_items: 1,..Default::default() });
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.close_step(RetainedCloneGrant { maximum_release_bytes: 0,..grant }).unwrap());
    assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.close_step(grant).unwrap());
    assert_eq!((allocated,released),(0,std::mem::size_of::<UnitOwnedRetirement>()));assert_eq!(step.progress(),RetainedCloneProgress { copied_items: 1,released_bytes: released,..Default::default() });
    assert_eq!(retirements.close_step(grant).unwrap(),RetainedCloneStep::Complete(Default::default()));
    assert!(retirements.terminal_is_empty());
    let expected=retirements.backing_release_byte_demand().unwrap();
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.release_empty_backing_step(grant).unwrap());
    assert_eq!((allocated,released),(0,expected));assert_eq!(step.progress().released_bytes,expected);

    let mut reservations = Vec::new();
    for _ in 0..ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY {
        reservations.push(retirements.reserve_owner_slots(1).expect("fixed reservation registry admits its exact capacity"));
    }
    assert!(retirements.reserve_owner_slots(1).is_err(), "reservation-registry +1 fails before owner construction");
    for reservation in reservations {
        retirements.release_owner_slots(reservation).expect("each exact generation-qualified reservation returns once");
    }
    assert!(retirements.terminal_is_empty());
}
