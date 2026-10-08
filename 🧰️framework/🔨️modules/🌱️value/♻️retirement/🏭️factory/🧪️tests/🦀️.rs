use super::*;

struct OwnedFactory { payload: Vec<u8> }

#[derive(crate::FactoryPayloadRetirement)]
struct ScalarFactory { tag: u64, name: &'static str }

impl FactoryPayloadRetirement for OwnedFactory {
    type CloseState = Option<Vec<u8>>;
    fn close_state_birth_bytes(&self) -> usize { 0 }
    fn prepare_close_state(&self) -> Self::CloseState { None }
    fn transfer_payload(value: Self, state: &mut Self::CloseState) { *state = Some(value.payload); }
    fn close_state_byte_demand(state: &Self::CloseState) -> usize { state.as_ref().map_or(0, Vec::capacity) }
    fn close_state_step(state: &mut Self::CloseState, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        let Some(owner) = state.as_ref() else { return Ok(SnapshotRetirementStep::Complete); };
        let extent = owner.capacity();
        if items == 0 || bytes < extent { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        drop(state.take());
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent })
    }
    fn close_state_terminal_is_empty(state: &Self::CloseState) -> bool { state.is_none() }
}

#[test]
fn factory_preborn_tickets_consume_concurrent_aliases_and_report_original_physical_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let maximum = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|row| row.as_u64().unwrap() as usize) {
        let original = Arc::new(OwnedFactory { payload: vec![42; extent] });
        let birth = original.factory_retirement_birth_bytes();
        assert_eq!(birth, factory_retirement_frame_bytes::<OwnedFactory>() + original.close_state_birth_bytes());
        let tickets: Vec<_> = (0..fixture["aliasCount"].as_u64().unwrap()).map(|_| {
            let (ticket, events) = crate::observe_retirement_allocations(|| original.clone().preborn_factory_retirement());
            assert_eq!(events, (birth, 0));
            ticket
        }).collect();
        let (_, events) = crate::observe_retirement_allocations(|| drop(original));
        assert_eq!(events, (0, 0));
        let handles: Vec<_> = tickets.into_iter().map(|ticket| std::thread::spawn(move || {
            let mut slot = Some(ticket);
            let mut bytes = 0;
            let mut payload_releases = 0;
            let mut turns = 0;
            while let Some(owner) = slot.as_ref() {
                let demand = factory_ticket_byte_demand(owner);
                assert!(demand <= maximum);
                if demand != 0 {
                    let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, 1, demand - 1));
                    assert_eq!(step.unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    assert_eq!(events, (0, 0));
                }
                let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, 1, demand));
                let reported = match step.unwrap() { SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); released_bytes }, SnapshotRetirementStep::Complete => 0, SnapshotRetirementStep::Blocked => panic!("strong factory aliases have no external wait") };
                assert_eq!(events, (0, reported));
                bytes += reported;
                payload_releases += usize::from(reported == extent);
                turns += 1;
                assert!(turns <= 16);
            }
            (bytes, payload_releases)
        })).collect();
        let receipts: Vec<_> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();
        let released: usize = receipts.iter().map(|row| row.0).sum();
        assert_eq!(receipts.iter().map(|row| row.1).sum::<usize>(), fixture["winnerCount"].as_u64().unwrap() as usize);
        assert_eq!(released, extent + factory_arc_birth_bytes::<OwnedFactory>() + birth * fixture["aliasCount"].as_u64().unwrap() as usize);
        eprintln!("[DEBUG] preborn factory aliases={} original payload={} unique winner=1 physical={} close-birth=0 whole-frame-query={} exact constructor-ticket-birth={}", fixture["aliasCount"], extent, released, arc_extent::<OwnedFactory>(), birth);
    }
}

#[test]
fn factory_preborn_tickets_keep_weak_wait_and_copy_only_body_truthful() {
    let owner = Arc::new(ScalarFactory { tag: 7, name: "factory" });
    assert_eq!((owner.tag, owner.name), (7, "factory"));
    let weak = Arc::downgrade(&owner);
    let mut slot = Some(owner.clone().preborn_factory_retirement());
    drop(owner);
    let demand = factory_ticket_byte_demand(slot.as_ref().unwrap());
    let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, 1, demand));
    assert_eq!(step.unwrap(), SnapshotRetirementStep::Blocked);
    assert_eq!(events, (0, 0));
    assert!(weak.upgrade().is_some());
    drop(weak);
    for _ in 0..8 {
        let Some(ticket) = slot.as_ref() else { break; };
        let demand = factory_ticket_byte_demand(ticket);
        let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, 1, demand));
        let released = match step.unwrap() { SnapshotRetirementStep::Pending { released_bytes, .. } => released_bytes, SnapshotRetirementStep::Complete => 0, SnapshotRetirementStep::Blocked => panic!("factory weak alias has been returned") };
        assert_eq!(events, (0, released));
    }
    assert!(slot.is_none());
    eprintln!("[DEBUG] preborn Copy-field factory external Weak denied with original owner retained; exact frame and ticket turns heap receipts equal");
}

#[derive(crate::FactoryPayloadRetirement)]
struct NestedFactory {
    #[factory_child]
    child: Arc<dyn FactoryRetirement>,
    tag: u64,
}

#[test]
fn factory_preborn_tickets_retire_nested_original_children_without_close_birth() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|row| row.as_u64().unwrap() as usize) {
        let child: Arc<dyn FactoryRetirement> = Arc::new(OwnedFactory { payload: vec![17; extent] });
        let middle: Arc<dyn FactoryRetirement> = Arc::new(NestedFactory { child, tag: 1 });
        let original = Arc::new(NestedFactory { child: middle, tag: 2 });
        assert_eq!(original.tag, fixture["nestedDepth"].as_u64().unwrap());
        let birth = original.factory_retirement_birth_bytes();
        let (ticket, events) = crate::observe_retirement_allocations(|| original.clone().preborn_factory_retirement());
        assert_eq!(events, (birth, 0));
        let (_, events) = crate::observe_retirement_allocations(|| drop(original));
        assert_eq!(events, (0, 0));
        let mut ticket = Some(ticket);
        let mut released = 0;
        for _ in 0..32 {
            let Some(owner) = ticket.as_ref() else { break; };
            let demand = factory_ticket_byte_demand(owner);
            if demand != 0 {
                let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut ticket, 1, demand - 1));
                assert_eq!(step.unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                assert_eq!(events, (0, 0));
            }
            let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut ticket, 1, demand));
            let bytes = match step.unwrap() { SnapshotRetirementStep::Pending { released_bytes, .. } => released_bytes, SnapshotRetirementStep::Complete => 0, SnapshotRetirementStep::Blocked => panic!("nested factory has no external reader") };
            assert_eq!(events, (0, bytes));
            released += bytes;
        }
        assert!(ticket.is_none());
        assert_eq!(released, extent + arc_extent::<OwnedFactory>() + 2 * arc_extent::<NestedFactory>() + birth);
        eprintln!("[DEBUG] nested factory depth=2 original payload={extent} preborn birth={birth} physical={released} close-birth=0");
    }
}

#[derive(crate::FactoryPayloadRetirement)]
struct InlineFactory {
    #[factory_owned]
    payload: OwnedFactory,
    tag: u64,
}

#[test]
fn factory_preborn_tickets_move_inline_original_payload_into_prefunded_state() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["inlineOwnedPayload"], true);
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|row| row.as_u64().unwrap() as usize) {
        let original = Arc::new(InlineFactory { payload: OwnedFactory { payload: vec![31; extent] }, tag: 7 });
        assert_eq!(original.tag, 7);
        let birth = original.factory_retirement_birth_bytes();
        let (ticket, events) = crate::observe_retirement_allocations(|| original.clone().preborn_factory_retirement());
        assert_eq!(events, (birth, 0));
        let (_, events) = crate::observe_retirement_allocations(|| drop(original));
        assert_eq!(events, (0, 0));
        let mut ticket = Some(ticket);
        let mut released = 0;
        for _ in 0..16 {
            let Some(owner) = ticket.as_ref() else { break; };
            let demand = factory_ticket_byte_demand(owner);
            if demand != 0 {
                let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut ticket, 1, demand - 1));
                assert_eq!(step.unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                assert_eq!(events, (0, 0));
            }
            let (step, events) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut ticket, 1, demand));
            let bytes = match step.unwrap() { SnapshotRetirementStep::Pending { released_bytes, .. } => released_bytes, SnapshotRetirementStep::Complete => 0, SnapshotRetirementStep::Blocked => panic!("inline payload has no external reader") };
            assert_eq!(events, (0, bytes));
            released += bytes;
        }
        assert!(ticket.is_none());
        assert_eq!(released, extent + arc_extent::<InlineFactory>() + birth);
        eprintln!("[DEBUG] factory inline original payload={extent} preborn birth={birth} physical={released} close-birth=0");
    }
}
