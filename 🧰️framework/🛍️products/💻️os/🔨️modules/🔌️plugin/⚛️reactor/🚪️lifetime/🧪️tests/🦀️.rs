//#region 🧪️GuestLifecycleOwnership
use super::*;

struct Owner { remaining: usize }
impl terminal_owner::Sealed for Owner {}
impl GuestLifetimeOwner for Owner {
    fn terminal_is_empty(&self) -> Result<bool, &'static str> { Ok(self.remaining == 0) }
    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, _maximum_bytes: usize) -> Result<GuestTerminalRelease, &'static str> {
        if maximum_items == 0 { return Ok(GuestTerminalRelease::Pending); }
        if owner.as_ref().is_some_and(|owner| owner.remaining != 0) { return Err("live test owner"); }
        drop(owner.take());
        Ok(GuestTerminalRelease::Released)
    }
}

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }

fn open() -> ActorInstanceOpenRequest {
    let fixture = fixture();
    serde_json::from_value(fixture["open"].clone()).unwrap()
}

fn captured() -> GuestLifecycleCell<Owner> {
    let fixture = fixture();
    let generation = fixture["capturedGuestLifetime"].as_str().unwrap().parse().unwrap();
    let mut cell = GuestLifecycleCell::admit(open(), generation).unwrap();
    cell.install_owner(Owner { remaining: fixture["nativeParticipants"].as_array().unwrap().len() }).unwrap_or_else(|_| panic!("unoccupied structural slot"));
    cell
}

fn ack(cell: &mut GuestLifecycleCell<Owner>) {
    let receipt = cell.retained_receipt().unwrap();
    cell.stage_ack(ActorInstanceLifecycleAck { receipt }).unwrap();
    cell.release_owner_step(1, 4096).unwrap();
    cell.finish_turn(Some(100), || Some(101), true).unwrap();
}

#[test]
fn guest_instance_lifecycle_terminal_release_work_is_measured_and_never_repeated_after_late_clock() {
    struct DropOwner { clock: std::rc::Rc<std::cell::Cell<u64>>, drops: std::rc::Rc<std::cell::Cell<usize>>, work_us: u64 }
    impl terminal_owner::Sealed for DropOwner {}
    impl GuestLifetimeOwner for DropOwner {
        fn terminal_is_empty(&self) -> Result<bool, &'static str> { Ok(true) }
        fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, _maximum_bytes: usize) -> Result<GuestTerminalRelease, &'static str> {
            if maximum_items == 0 { return Ok(GuestTerminalRelease::Pending); }
            drop(owner.take());
            Ok(GuestTerminalRelease::Released)
        }
    }
    impl Drop for DropOwner {
        fn drop(&mut self) { self.clock.set(self.clock.get() + self.work_us); self.drops.set(self.drops.get() + 1); }
    }
    let fixture = fixture();
    let law = &fixture["terminalRelease"];
    let started = law["startedUs"].as_u64().unwrap();
    let clock = std::rc::Rc::new(std::cell::Cell::new(started));
    let drops = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut cell = GuestLifecycleCell::admit(open(), 13).unwrap();
    cell.install_owner(DropOwner { clock: clock.clone(), drops: drops.clone(), work_us: law["destructorWorkUs"].as_u64().unwrap() }).unwrap_or_else(|_| panic!("fresh owner"));
    let captured = cell.retained_receipt().unwrap();
    cell.stage_ack(ActorInstanceLifecycleAck { receipt: captured }).unwrap();
    cell.finish_turn(Some(100), || Some(clock.get()), true).unwrap();
    cell.record_close_admission(ActorInstanceCloseRequest { lifetime: cell.lifetime(), request_sequence: 9 }, 51).unwrap();
    cell.stage_ack(ActorInstanceLifecycleAck { receipt: cell.retained_receipt().unwrap() }).unwrap();
    cell.finish_turn(Some(100), || Some(clock.get()), true).unwrap();
    cell.prepare_retired().unwrap();
    let receipt = cell.retained_receipt().unwrap();
    cell.stage_ack(ActorInstanceLifecycleAck { receipt }).unwrap();
    cell.release_owner_step(0, 4096).unwrap();
    assert_eq!(drops.get(), 0);
    cell.release_owner_step(1, 4096).unwrap();
    assert_eq!(drops.get(), 1);
    assert_eq!(cell.finish_turn(Some(started), || Some(clock.get()), true).is_ok(), law["firstAckAccepted"].as_bool().unwrap());
    assert_eq!(cell.retained_receipt(), Some(receipt));
    assert!(!cell.is_released());
    assert!(cell.owner().is_none());
    cell.release_owner_step(1, 4096).unwrap();
    cell.finish_turn(Some(clock.get()), || Some(clock.get()), true).unwrap();
    assert!(cell.is_released());
    assert_eq!(drops.get(), law["destructionsAfterRetry"].as_u64().unwrap() as usize);
}

fn close(cell: &mut GuestLifecycleCell<Owner>) {
    ack(cell);
    let fixture = fixture();
    let request = ActorInstanceCloseRequest { lifetime: cell.lifetime(), request_sequence: fixture["closeRequestSequence"].as_u64().unwrap() };
    cell.validate_close(request).unwrap();
    cell.record_close_admission(request, fixture["closeGeneration"].as_str().unwrap().parse().unwrap()).unwrap();
    ack(cell);
}

#[test]
fn guest_instance_lifecycle_ack_fault_keeps_exact_receipt_and_owner() {
    for (start, end, succeeded) in [(Some(10), None, true), (None, Some(11), true), (Some(10), Some(9), true), (Some(10), Some(8010), true), (Some(10), Some(11), false)] {
        let mut cell = captured();
        let receipt = cell.retained_receipt().unwrap();
        cell.stage_ack(ActorInstanceLifecycleAck { receipt }).unwrap();
        assert!(cell.finish_turn(start, || end, succeeded).is_err());
        assert_eq!(cell.retained_receipt(), Some(receipt));
        assert!(cell.owner().is_some());
        assert!(!cell.is_live());
        cell.finish_turn(Some(100), || Some(101), true).unwrap();
        assert!(cell.is_live());
        let request = ActorInstanceCloseRequest { lifetime: cell.lifetime(), request_sequence: 9 };
        cell.record_close_admission(request, 51).unwrap();
        ack(&mut cell);
        cell.owner_mut().unwrap().remaining = 0;
        assert!(cell.prepare_retired().unwrap());
        ack(&mut cell);
        assert!(cell.is_released());
    }
}

#[test]
fn guest_instance_lifecycle_partial_close_unwind_never_drops_structural_owner() {
    let mut cell = captured();
    close(&mut cell);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        cell.owner_mut().unwrap().remaining -= 1;
        panic!("injected descendant failure");
    }));
    assert!(result.is_err());
    assert_eq!(cell.owner().unwrap().remaining, 10);
    assert!(!cell.prepare_retired().unwrap());
    while cell.owner().unwrap().remaining != 0 {
        cell.owner_mut().unwrap().remaining -= 1;
        assert_eq!(cell.prepare_retired().unwrap(), cell.owner().unwrap().remaining == 0);
    }
    let retired = cell.retained_receipt().unwrap();
    let ActorInstanceLifecycleReceipt::Retired { lifetime, request_sequence, close_generation } = retired else { panic!("exact terminal receipt") };
    assert!(cell.stage_ack(ActorInstanceLifecycleAck { receipt: ActorInstanceLifecycleReceipt::Retired { lifetime, request_sequence: request_sequence + 1, close_generation } }).is_err());
    assert_eq!(cell.retained_receipt(), Some(retired));
    ack(&mut cell);
    assert!(cell.is_released());
    assert!(cell.owner().is_none());
    cell.stage_ack(ActorInstanceLifecycleAck { receipt: retired }).unwrap();
    assert_eq!(cell.finish_turn(Some(10), || Some(11), true).unwrap(), None);
}

#[test]
fn guest_instance_lifecycle_same_activation_reopen_rejects_old_authority() {
    let mut old = captured();
    let old_lifetime = old.lifetime();
    close(&mut old);
    old.owner_mut().unwrap().remaining = 0;
    old.prepare_retired().unwrap();
    let old_receipt = old.retained_receipt().unwrap();
    ack(&mut old);
    let mut new = GuestLifecycleCell::admit(ActorInstanceOpenRequest { request_sequence: 10, ..open() }, 14).unwrap();
    new.install_owner(Owner { remaining: 0 }).unwrap_or_else(|_| panic!("fresh exact owner"));
    assert!(new.stage_ack(ActorInstanceLifecycleAck { receipt: old_receipt }).is_err());
    assert!(new.validate_close(ActorInstanceCloseRequest { lifetime: old_lifetime, request_sequence: 11 }).is_err());
    assert!(new.owner().is_some());
    ack(&mut new);
    new.record_close_admission(ActorInstanceCloseRequest { lifetime: new.lifetime(), request_sequence: 11 }, 52).unwrap();
    ack(&mut new);
    new.prepare_retired().unwrap();
    ack(&mut new);
    assert!(new.is_released());
    assert!(GuestLifecycleCell::<Owner>::admit(open(), 0).is_err());
    assert!(GuestLifecycleSerial::new(u64::MAX).next().is_err());
}
//#endregion 🧪️GuestLifecycleOwnership
