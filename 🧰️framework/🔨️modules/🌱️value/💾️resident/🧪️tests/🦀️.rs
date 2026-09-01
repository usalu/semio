use super::{ResidentCapacity, ResidentFault, ResidentResources, RESIDENT_MAXIMUM_COUNT};

//#region 🔎️AllocationObservation
struct ObservedAllocator;

thread_local! {
    static COUNT_ALLOCATIONS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ALLOCATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ALLOCATION_PHASE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ALLOCATION_LAYOUTS: std::cell::Cell<[(usize, usize, usize); 8]> = const { std::cell::Cell::new([(0, 0, 0); 8]) };
    static DEALLOCATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static DEALLOCATION_LAYOUTS: std::cell::Cell<[(usize, usize); 8]> = const { std::cell::Cell::new([(0, 0); 8]) };
    static FAIL_NEXT_ALLOCATION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        primary_recovery::observe_allocator_enter(layout);
        COUNT_ALLOCATIONS.with(|enabled| { if enabled.get() {
            ALLOCATIONS.with(|count| {
                let index = count.get();
                if index < 8 { ALLOCATION_LAYOUTS.with(|events| { let mut values = events.get(); values[index] = (ALLOCATION_PHASE.with(std::cell::Cell::get), layout.size(), layout.align()); events.set(values); }); }
                count.set(index + 1);
            });
        } });
        let failed = FAIL_NEXT_ALLOCATION.with(|value| value.replace(false));
        let pointer = if failed { std::ptr::null_mut() } else { unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) } };
        primary_recovery::observe_allocator_return(layout, !failed, pointer);
        pointer
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        let _ = COUNT_ALLOCATIONS.try_with(|enabled| { if enabled.get() { DEALLOCATIONS.with(|count| {
            let index = count.get();
            if index < 8 { DEALLOCATION_LAYOUTS.with(|events| { let mut values = events.get(); values[index] = (layout.size(), layout.align()); events.set(values); }); }
            count.set(index + 1);
        }); } });
        unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System, ptr, layout); }
        release_baseline::observe_system_dealloc_returned(layout);
        release_phases::observe_system_dealloc_returned(layout);
        primary_recovery::observe_system_dealloc_returned(layout);
    }
}

#[global_allocator]
static ALLOCATOR: ObservedAllocator = ObservedAllocator;
//#endregion 🔎️AllocationObservation

//#region 🧪️Capacity
fn resources(value: &serde_json::Value) -> Result<ResidentResources, ResidentFault> {
    ResidentResources::new(value["bytes"].as_u64().ok_or(ResidentFault::Count)?, value["slots"].as_u64().ok_or(ResidentFault::Count)?, value["owners"].as_u64().ok_or(ResidentFault::Count)?)
}

fn capacity(value: &serde_json::Value) -> Result<ResidentCapacity, ResidentFault> {
    ResidentCapacity::new(resources(value)?, resources(&value["control"])?)
}

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture.json")).unwrap() }

#[test]
fn resident_capacity_consumes_the_shared_capacity_and_invalid_vectors() {
    let fixture = fixture();
    let value = capacity(&fixture["capacity"]).unwrap();
    let total = resources(&fixture["capacity"]).unwrap();
    let control = resources(&fixture["capacity"]["control"]).unwrap();
    assert_eq!(value.total(), total);
    assert_eq!(value.control(), control);
    assert_eq!(value.data().checked_add(control).unwrap(), total);
    for vector in fixture["invalidCapacities"].as_array().unwrap() { assert!(capacity(&vector["value"]).is_err(), "{}", vector["name"]); }
    assert_eq!(RESIDENT_MAXIMUM_COUNT, 9_007_199_254_740_991);
    assert!(!std::mem::needs_drop::<ResidentCapacity>());
}

#[test]
fn resident_capacity_all_axes_refuse_before_safe_integer_overflow() {
    let fixture = fixture();
    let maximum = fixture["overflow"]["capacity"].as_u64().unwrap();
    let used = fixture["overflow"]["used"].as_u64().unwrap();
    let request = fixture["overflow"]["request"].as_u64().unwrap();
    for axis in 0..3 {
        let make = |count| ResidentResources::new(if axis == 0 { count } else { 0 }, if axis == 1 { count } else { 0 }, if axis == 2 { count } else { 0 }).unwrap();
        let limit = make(maximum);
        let occupied = make(used);
        let incoming = make(request);
        assert_eq!(incoming.fits_within(limit.checked_sub(occupied).unwrap()), (used as u128 + request as u128) <= maximum as u128);
        assert!(!incoming.fits_within(limit.checked_sub(occupied).unwrap()));
        assert_eq!(occupied.checked_add(incoming), Err(ResidentFault::Count));
        assert_eq!(make(0).checked_sub(make(1)), Err(ResidentFault::Capacity));
    }
    for invalid in [RESIDENT_MAXIMUM_COUNT + 1, u64::MAX] {
        assert_eq!(ResidentResources::new(invalid, 0, 0), Err(ResidentFault::Count));
        assert_eq!(ResidentResources::new(0, invalid, 0), Err(ResidentFault::Count));
        assert_eq!(ResidentResources::new(0, 0, invalid), Err(ResidentFault::Count));
    }
}

#[test]
fn resident_capacity_data_and_control_are_disjoint_and_never_defaulted() {
    let zero = ResidentResources::new(0, 0, 0).unwrap();
    for resources in [zero, ResidentResources::new(1, 1, 1).unwrap(), ResidentResources::new(RESIDENT_MAXIMUM_COUNT, RESIDENT_MAXIMUM_COUNT, RESIDENT_MAXIMUM_COUNT).unwrap()] {
        let all_control = ResidentCapacity::new(resources, resources).unwrap();
        assert_eq!(all_control.data(), zero);
        assert_eq!(all_control.control(), resources);
        let all_data = ResidentCapacity::new(resources, zero).unwrap();
        assert_eq!(all_data.data(), resources);
        assert_eq!(all_data.control(), zero);
    }
    let split = capacity(&fixture()["capacity"]).unwrap();
    let data_remaining = split.data().checked_sub(split.data()).unwrap();
    let control = split.control();
    assert_eq!(data_remaining, zero);
    assert!(control.fits_within(split.control()));
    assert!(!control.fits_within(data_remaining));
    assert_eq!(split.data().checked_add(split.control()).unwrap(), split.total());
}

#[test]
fn resident_capacity_constructor_owns_no_heap_backing() {
    let fixture = fixture();
    let total = resources(&fixture["capacity"]).unwrap();
    let control = resources(&fixture["capacity"]["control"]).unwrap();
    ALLOCATIONS.with(|count| count.set(0));
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(true));
    let value = ResidentCapacity::new(total, control);
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(false));
    let allocations = ALLOCATIONS.with(std::cell::Cell::get);
    assert!(value.is_ok());
    assert_eq!(allocations, 0, "capacity vocabulary must not allocate a hidden ledger or backing");
    eprintln!("[DEBUG] native resident capacity header={} allocations={allocations} permit_mounted=false", std::mem::size_of::<ResidentCapacity>());
}
//#endregion 🧪️Capacity

//#region 🧪️AdmissionOwnership
use super::{ResidentConsumer, ResidentGrant, ResidentLedgerRoot, ResidentPartition, ResidentStep, ResidentStepKind};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

struct ResidentDropProbe {
    payload: Option<Box<[u8; 32]>>,
    drops: Arc<AtomicUsize>,
}

impl ResidentDropProbe {
    fn new(drops: &Arc<AtomicUsize>) -> Self { Self { payload: Some(Box::new([73; 32])), drops: drops.clone() } }
    fn pointer(&self) -> *const u8 { self.payload.as_ref().unwrap().as_ptr() }
    fn retire_payload(&mut self, grant: ResidentGrant) -> bool {
        if grant.max_items() == 0 || grant.max_bytes() < 32 { return false; }
        drop(self.payload.take());
        true
    }
}

impl Drop for ResidentDropProbe {
    fn drop(&mut self) {
        assert!(self.payload.is_none(), "the concrete parent must retire the original payload before its shell destructor");
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

fn admission_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../📨️admission/🧪️fixture.json")).unwrap() }
fn admission_grant(bytes: u64) -> ResidentGrant { ResidentGrant::new(1, bytes).unwrap() }
fn full_admission_grant() -> ResidentGrant { admission_grant(admission_fixture()["grants"][2]["maxBytes"].as_u64().unwrap()) }
fn admission_capacity() -> ResidentCapacity { capacity(&fixture()["capacity"]).unwrap() }
fn assert_admission_step(step: ResidentStep, grant: ResidentGrant) { assert!(step.items <= grant.max_items()); assert!(step.bytes <= grant.max_bytes()); }

fn admitted_consumer<'root, C: Send + 'static>(root: &'root ResidentLedgerRoot, source: &mut Option<C>) -> ResidentConsumer<'root, C> {
    let grant = full_admission_grant();
    for _ in 0..8 { if root.prepare_consumer::<C>(ResidentPartition::Data, grant).unwrap().kind == ResidentStepKind::Ready { break; } }
    let consumer = root.prepared_consumer::<C>().unwrap().unwrap();
    if source.is_some() { assert_eq!(consumer.install(source, grant).unwrap().kind, ResidentStepKind::Ready); }
    consumer
}

fn close_admission_root(root: &ResidentLedgerRoot) {
    let grant = full_admission_grant();
    assert!(root.begin_close().unwrap());
    for _ in 0..128 {
        if root.terminal_is_empty() { break; }
        assert_admission_step(root.close_step(grant).unwrap(), grant);
    }
    assert!(root.terminal_is_empty());
    assert_eq!(root.allocated_bytes().unwrap(), 0);
    assert_eq!(root.usage(ResidentPartition::Data).unwrap(), ResidentResources::new(0, 0, 0).unwrap());
}

fn retire_parent_probe(source: &mut Option<ResidentDropProbe>) {
    assert!(source.as_mut().unwrap().retire_payload(full_admission_grant()));
    drop(source.take());
}

#[test]
fn resident_admission_native_layout_has_one_fixed_root_and_separate_move_costs() {
    let capacity = admission_capacity();
    ALLOCATIONS.with(|count| count.set(0));
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(true));
    let root = ResidentLedgerRoot::new(capacity);
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(false));
    assert_eq!(ALLOCATIONS.with(std::cell::Cell::get), 0, "the retained bootstrap root is inline, not a hidden shared heap directory");
    let layout = root.native_layout::<ResidentDropProbe, ResidentDropProbe>();
    assert_eq!(layout.root_bytes, std::mem::size_of::<ResidentLedgerRoot>() as u64);
    assert!(layout.admission_page_bytes > 0 && layout.record_page_bytes > 0);
    assert!(layout.consumer_move_bytes >= std::mem::size_of::<Option<ResidentDropProbe>>() as u64);
    assert!(layout.shell_move_bytes >= std::mem::size_of::<Option<ResidentDropProbe>>() as u64);
    assert!(layout.descriptor_move_bytes >= std::mem::size_of::<Vec<u8>>() as u64);
    assert_eq!(layout.final_root_bytes, layout.root_bytes);
    for bytes in [layout.admission_page_bytes, layout.consumer_page_bytes, layout.record_page_bytes, layout.consumer_move_bytes, layout.shell_move_bytes, layout.descriptor_move_bytes, layout.final_root_bytes] { assert!(bytes <= full_admission_grant().max_bytes()); }
    assert_eq!(admission_fixture()["nativeOwnership"]["layout"].as_array().unwrap().len(), 8);
    eprintln!("[DEBUG] resident root={} admission={} record={} consumerMove={} shellMove={} descriptorMove={} finalRoot={} bootstrapHeap=0", layout.root_bytes, layout.admission_page_bytes, layout.record_page_bytes, layout.consumer_move_bytes, layout.shell_move_bytes, layout.descriptor_move_bytes, layout.final_root_bytes);
    close_admission_root(&root);
}

#[test]
fn resident_admission_short_and_foreign_refusals_preserve_live_consumer() {
    let drops = Arc::new(AtomicUsize::new(0));
    let root = ResidentLedgerRoot::new(admission_capacity());
    let other = ResidentLedgerRoot::new(admission_capacity());
    let consumer = admitted_consumer(&root, &mut Some(ResidentDropProbe::new(&drops)));
    let original = consumer.read().unwrap().unwrap().pointer();
    let foreign = admitted_consumer(&other, &mut None::<ResidentDropProbe>);
    let original_backing = root.allocated_bytes().unwrap();
    let layout = root.native_layout::<ResidentDropProbe, ResidentDropProbe>();
    let ledger = root.ledger();
    let mut refusal_allocations = [0usize; 2];
    for (index, grant) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layout.admission_page_bytes - 1)].into_iter().enumerate() {
        ALLOCATIONS.with(|count| count.set(0)); COUNT_ALLOCATIONS.with(|enabled| enabled.set(true));
        let step = ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap();
        COUNT_ALLOCATIONS.with(|enabled| enabled.set(false));
        refusal_allocations[index] = ALLOCATIONS.with(std::cell::Cell::get);
        assert_eq!(step.kind, ResidentStepKind::Blocked);
        assert_eq!(root.allocated_bytes().unwrap(), original_backing); assert!(ledger.prepared_admission(&consumer).unwrap().is_none());
    }
    let grant = full_admission_grant();
    for _ in 0..8 { if ledger.prepared_admission(&consumer).unwrap().is_some() { break; } assert_admission_step(ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(), grant); }
    let cell = ledger.prepared_admission(&consumer).unwrap().unwrap();
    let charged = root.allocated_bytes().unwrap();
    assert!(charged >= layout.admission_page_bytes);
    assert!(ledger.prepared_admission(&foreign).unwrap().is_none());
    assert_eq!(ledger.claim_admission(&foreign, &cell, grant).unwrap().kind, ResidentStepKind::Rejected);
    assert_eq!(other.ledger().claim_admission(&consumer, &cell, grant).unwrap().kind, ResidentStepKind::Rejected);
    assert_eq!(root.allocated_bytes().unwrap(), charged);
    assert_eq!(ledger.claim_admission(&consumer, &cell, grant).unwrap().kind, ResidentStepKind::Ready);
    assert!(root.begin_close().unwrap());
    for _ in 0..4 { assert_admission_step(root.close_step(grant).unwrap(), grant); }
    assert!(!root.terminal_is_empty()); assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert_eq!(consumer.read_for_close().unwrap().unwrap().pointer(), original);
    let mut parent = None;
    assert_eq!(cell.handoff_consumer_into(&consumer, &mut parent, admission_grant(layout.consumer_move_bytes - 1)).unwrap().kind, ResidentStepKind::Blocked);
    assert!(parent.is_none()); assert_eq!(consumer.read_for_close().unwrap().unwrap().pointer(), original);
    assert_eq!(cell.handoff_consumer_into(&consumer, &mut parent, grant).unwrap().kind, ResidentStepKind::Pending);
    assert_eq!(parent.as_ref().unwrap().pointer(), original); assert!(consumer.read_for_close().unwrap().is_none());
    retire_parent_probe(&mut parent); assert_eq!(drops.load(Ordering::SeqCst), 1);
    drop(cell); drop(ledger); drop(consumer); drop(foreign); close_admission_root(&root); close_admission_root(&other);
    assert_eq!(refusal_allocations, [0, 0], "refused admission must not allocate; original consumer was retired before this assertion");
}

#[test]
fn resident_admission_caller_loss_and_parent_move_preserve_original_page_and_consumer() {
    let drops = Arc::new(AtomicUsize::new(0));
    let root = ResidentLedgerRoot::new(admission_capacity());
    let consumer = admitted_consumer(&root, &mut Some(ResidentDropProbe::new(&drops)));
    let original = consumer.read().unwrap().unwrap().pointer();
    let grant = full_admission_grant();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let ledger = root.ledger();
        for _ in 0..8 { if ledger.prepared_admission(&consumer).unwrap().is_some() { break; } ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
        let ordinary_return = ledger.prepared_admission(&consumer).unwrap().unwrap();
        drop(ordinary_return);
        panic!("[DEBUG] after actual preparation and lost ordinary return");
    }));
    assert!(result.is_err()); drop(result);
    let retained = root.allocated_bytes().unwrap(); assert!(retained > 0); assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(consumer);
    let moved_parent = (73u8, root);
    assert!(moved_parent.1.begin_close().unwrap());
    assert_eq!(moved_parent.1.close_step(grant).unwrap().kind, ResidentStepKind::Blocked);
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    let ledger = moved_parent.1.ledger();
    let consumer = moved_parent.1.recover_consumer_for_close::<ResidentDropProbe>().unwrap().unwrap();
    let cell = ledger.recover_admission_for_close(&consumer).unwrap().unwrap();
    assert_eq!(moved_parent.1.allocated_bytes().unwrap(), retained);
    assert_eq!(consumer.read_for_close().unwrap().unwrap().pointer(), original);
    let mut parent = None;
    cell.handoff_consumer_into(&consumer, &mut parent, grant).unwrap();
    assert_eq!(parent.as_ref().unwrap().pointer(), original); retire_parent_probe(&mut parent);
    drop(cell); drop(ledger); drop(consumer); close_admission_root(&moved_parent.1);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn resident_admission_record_mutation_unwind_and_exact_parent_handoffs_never_cold_drop() {
    let consumer_drops = Arc::new(AtomicUsize::new(0)); let shell_drops = Arc::new(AtomicUsize::new(0));
    let root = ResidentLedgerRoot::new(admission_capacity());
    let consumer = admitted_consumer(&root, &mut Some(ResidentDropProbe::new(&consumer_drops)));
    let consumer_pointer = consumer.read().unwrap().unwrap().pointer();
    let ledger = root.ledger(); let grant = full_admission_grant();
    for _ in 0..8 { if ledger.prepared_admission(&consumer).unwrap().is_some() { break; } ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
    let cell = ledger.prepared_admission(&consumer).unwrap().unwrap(); ledger.claim_admission(&consumer, &cell, grant).unwrap();
    let envelope = ResidentResources::new(32, 1, 1).unwrap();
    for _ in 0..8 { if cell.record::<ResidentDropProbe>().unwrap().is_some() { break; } assert_admission_step(ledger.reserve_record::<ResidentDropProbe, ResidentDropProbe>(&cell, envelope, grant).unwrap(), grant); }
    let record = cell.record::<ResidentDropProbe>().unwrap().unwrap();
    let mut source = Some(ResidentDropProbe::new(&shell_drops)); let shell_pointer = source.as_ref().unwrap().pointer();
    let layout = root.native_layout::<ResidentDropProbe, ResidentDropProbe>();
    assert_eq!(record.install(&mut source, admission_grant(layout.shell_move_bytes - 1)).unwrap().kind, ResidentStepKind::Blocked);
    assert_eq!(source.as_ref().unwrap().pointer(), shell_pointer);
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_eq!(record.install(&mut source, grant).unwrap().kind, ResidentStepKind::Ready);
        panic!("[DEBUG] after actual typed shell placement into retained record");
    }));
    assert!(outcome.is_err()); drop(outcome); assert!(source.is_none());
    assert_eq!(shell_drops.load(Ordering::SeqCst), 0); assert_eq!(consumer_drops.load(Ordering::SeqCst), 0);
    let mut shell_parent = None; let mut consumer_parent = None;
    assert!(root.begin_close().unwrap()); assert_admission_step(root.close_step(grant).unwrap(), grant); assert!(!root.terminal_is_empty());
    assert_eq!(record.handoff_into(&mut shell_parent, admission_grant(layout.shell_move_bytes - 1)).unwrap().kind, ResidentStepKind::Blocked);
    assert!(shell_parent.is_none());
    let transferred = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_eq!(record.handoff_into(&mut shell_parent, grant).unwrap().kind, ResidentStepKind::Pending);
        panic!("[DEBUG] after exact record-to-parent handoff");
    }));
    assert!(transferred.is_err()); drop(transferred); assert_eq!(shell_parent.as_ref().unwrap().pointer(), shell_pointer);
    for _ in 0..4 { assert_admission_step(root.close_step(grant).unwrap(), grant); }
    assert!(!root.terminal_is_empty(), "an empty S slot is not proof that the original C can be destroyed");
    assert_eq!(consumer_drops.load(Ordering::SeqCst), 0); assert_eq!(shell_drops.load(Ordering::SeqCst), 0);
    assert_eq!(cell.handoff_consumer_into(&consumer, &mut shell_parent, grant).unwrap().kind, ResidentStepKind::Rejected);
    assert_eq!(consumer.read_for_close().unwrap().unwrap().pointer(), consumer_pointer);
    cell.handoff_consumer_into(&consumer, &mut consumer_parent, grant).unwrap();
    assert_eq!(consumer_parent.as_ref().unwrap().pointer(), consumer_pointer);
    retire_parent_probe(&mut shell_parent); retire_parent_probe(&mut consumer_parent);
    drop(record); drop(cell); drop(ledger); drop(consumer); close_admission_root(&root);
    let final_row = admission_fixture()["nativeOwnership"]["releaseTrace"].as_array().unwrap().last().unwrap().clone();
    assert_eq!(shell_drops.load(Ordering::SeqCst) as u64, final_row["shellDrops"].as_u64().unwrap());
    assert_eq!(consumer_drops.load(Ordering::SeqCst) as u64, final_row["consumerDrops"].as_u64().unwrap());
    assert_eq!(root.terminal_is_empty(), final_row["terminal"].as_bool().unwrap());
}

#[test]
fn resident_admission_final_original_root_release_requires_its_own_grant() {
    let root = ResidentLedgerRoot::new(admission_capacity());
    let layout = root.native_layout::<ResidentDropProbe, ResidentDropProbe>(); assert!(root.begin_close().unwrap());
    for grant in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layout.final_root_bytes - 1)] {
        assert_eq!(root.close_step(grant).unwrap().kind, ResidentStepKind::Blocked);
        assert!(!root.terminal_is_empty()); assert_eq!(root.allocated_bytes().unwrap(), 0);
    }
    assert_eq!(root.close_step(admission_grant(layout.final_root_bytes)).unwrap().kind, ResidentStepKind::Complete);
    assert!(root.terminal_is_empty());
    assert_eq!(admission_fixture()["nativeOwnership"]["unknownFaultFinalDisposal"], false);
}

#[test]
fn resident_admission_first_access_refusal_allocation_boundary() {
    let capacity = admission_capacity();
    let short = ResidentGrant::new(0, 4096).unwrap();
    ALLOCATIONS.with(|value| value.set(0)); ALLOCATION_LAYOUTS.with(|value| value.set([(0, 0, 0); 8]));
    COUNT_ALLOCATIONS.with(|value| value.set(true)); ALLOCATION_PHASE.with(|value| value.set(1));
    let root = ResidentLedgerRoot::new(capacity);
    let after_root = ALLOCATIONS.with(std::cell::Cell::get);
    ALLOCATION_PHASE.with(|value| value.set(2));
    let first = root.prepare_consumer::<u64>(ResidentPartition::Data, short);
    let after_first = ALLOCATIONS.with(std::cell::Cell::get);
    ALLOCATION_PHASE.with(|value| value.set(3));
    let second = root.prepare_consumer::<u64>(ResidentPartition::Data, short);
    let after_second = ALLOCATIONS.with(std::cell::Cell::get);
    COUNT_ALLOCATIONS.with(|value| value.set(false));
    let events = ALLOCATION_LAYOUTS.with(std::cell::Cell::get);
    close_admission_root(&root);
    eprintln!("[DEBUG] resident first-access root={after_root} first={} second={} layouts={events:?}", after_first - after_root, after_second - after_first);
    assert_eq!(first.unwrap().kind, ResidentStepKind::Blocked); assert_eq!(second.unwrap().kind, ResidentStepKind::Blocked);
    let expected = admission_fixture()["nativeOwnership"]["firstAccessAllocations"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect::<Vec<_>>();
    assert_eq!([after_root, after_first - after_root, after_second - after_first].as_slice(), expected.as_slice());
}

#[test]
fn resident_admission_busy_access_preserves_original_root_without_allocation() {
    let root = ResidentLedgerRoot::new(admission_capacity());
    let held = root.state.try_lock().ok().unwrap();
    ALLOCATIONS.with(|value| value.set(0)); COUNT_ALLOCATIONS.with(|value| value.set(true));
    let prepare = root.prepare_consumer::<u64>(ResidentPartition::Data, ResidentGrant::new(1, 4096).unwrap());
    let close = root.begin_close(); let observed = root.allocated_bytes();
    COUNT_ALLOCATIONS.with(|value| value.set(false));
    let allocations = ALLOCATIONS.with(std::cell::Cell::get); drop(held);
    close_admission_root(&root);
    assert_eq!(prepare.unwrap().kind, ResidentStepKind::Blocked); assert_eq!(close.unwrap(), false);
    assert_eq!(observed, Err(ResidentFault::Busy)); assert_eq!(allocations, 0);
}

#[test]
fn resident_admission_inline_gate_keeps_poison_sticky_after_callback_unwind() {
    let gate = super::ResidentAccess::new(73u64);
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut held = gate.try_lock().ok().unwrap(); *held = 91;
        panic!("[DEBUG] actual inline gate mutation before callback unwind");
    }));
    assert!(outcome.is_err()); drop(outcome);
    assert!(matches!(gate.try_lock(), Err(super::ResidentAccessError::Poisoned)));
    assert!(matches!(gate.try_lock(), Err(super::ResidentAccessError::Poisoned)));
    assert_eq!(unsafe { *gate.value.get() }, 91);
    assert!(!gate.held.load(Ordering::Acquire));
}

struct RepopulatedConsumer { drops: Arc<AtomicUsize>, payload: Box<[u8; 32]> }

impl Drop for RepopulatedConsumer {
    fn drop(&mut self) { self.drops.fetch_add(1, Ordering::SeqCst); }
}

#[test]
fn resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop() {
    let drops = Arc::new(AtomicUsize::new(0));
    let (observed, wait_for_empty) = std::sync::mpsc::sync_channel(1);
    let (resume, resumed) = std::sync::mpsc::sync_channel(1);
    let (after_release, released) = std::sync::mpsc::sync_channel(1);
    let root = ResidentLedgerRoot::new(admission_capacity()); let grant = full_admission_grant();
    let consumer = admitted_consumer(&root, &mut None::<RepopulatedConsumer>);
    {
        let ledger = root.ledger();
        for _ in 0..8 { if ledger.prepared_admission(&consumer).unwrap().is_some() { break; } ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
        let cell = ledger.prepared_admission(&consumer).unwrap().unwrap(); ledger.claim_admission(&consumer, &cell, grant).unwrap();
    }
    root.state.try_lock().ok().unwrap().consumer_release_interlock = Some(super::ConsumerReleaseInterlock { observed, resume: resumed });
    let mut attempted = Some(RepopulatedConsumer { drops: drops.clone(), payload: Box::new([73; 32]) });
    let original = attempted.as_ref().unwrap().payload.as_ptr() as usize;
    let (during, after, source_pointer) = std::thread::scope(|scope| {
        let foreign = &consumer; let source = &mut attempted;
        let writer = scope.spawn(move || {
            wait_for_empty.recv_timeout(std::time::Duration::from_secs(1)).unwrap();
            let during = foreign.install(source, grant).unwrap().kind;
            resume.send(()).unwrap();
            released.recv_timeout(std::time::Duration::from_secs(1)).unwrap();
            let after = foreign.install(source, grant).unwrap().kind;
            (during, after, source.as_ref().map(|value| value.payload.as_ptr() as usize))
        });
        assert!(root.begin_close().unwrap());
        for _ in 0..16 {
            assert_admission_step(root.close_step(grant).unwrap(), grant);
            if root.state.try_lock().ok().unwrap().consumer_release_interlock.is_none() { break; }
        }
        after_release.send(()).unwrap();
        writer.join().unwrap()
    });
    let forward_alias_refused = matches!(root.prepared_consumer::<RepopulatedConsumer>(), Err(ResidentFault::Closed));
    let forward_read_refused = matches!(consumer.read(), Err(ResidentFault::Closed));
    let close_read_empty = consumer.read_for_close().unwrap().is_none();
    let accepted = after == ResidentStepKind::Ready;
    drop(consumer);
    close_admission_root(&root);
    let actual_drops = drops.load(Ordering::SeqCst);
    drop(attempted.take());
    eprintln!("[DEBUG] resident foreign repopulation accepted={accepted} consumerDropsDuringRelease={actual_drops} originalRootTerminal={}", root.terminal_is_empty());
    let expected = &admission_fixture()["nativeOwnership"]["foreignRepopulation"];
    assert_eq!(actual_drops as u64, expected["consumerDropsDuringRelease"].as_u64().unwrap());
    assert_eq!(accepted, expected["accepted"].as_bool().unwrap());
    assert_eq!(during, ResidentStepKind::Blocked); assert_eq!(after, ResidentStepKind::Rejected);
    assert_eq!(source_pointer, Some(original)); assert!(forward_alias_refused && forward_read_refused && close_read_empty);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[repr(align(64))]
struct AlignedResident(u8);

#[test]
fn resident_admission_exact_layout_and_partial_consumer_cancel_release_once() {
    let fixture = admission_fixture(); let grant = full_admission_grant();
    let layout = std::alloc::Layout::new::<super::ConsumerNode<AlignedResident>>();
    assert_eq!(layout.align() as u64, fixture["nativeOwnership"]["exactAllocation"]["alignment"].as_u64().unwrap());
    for (frontier, name) in fixture["nativeOwnership"]["exactAllocation"]["cancelFrontiers"].as_array().unwrap().iter().enumerate() {
        let root = ResidentLedgerRoot::new(admission_capacity());
        ALLOCATIONS.with(|value| value.set(0)); DEALLOCATIONS.with(|value| value.set(0));
        ALLOCATION_LAYOUTS.with(|value| value.set([(0, 0, 0); 8])); DEALLOCATION_LAYOUTS.with(|value| value.set([(0, 0); 8]));
        COUNT_ALLOCATIONS.with(|value| value.set(true));
        for _ in 0..=frontier { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
        let allocated = root.allocated_bytes().unwrap(); let used = root.usage(ResidentPartition::Data).unwrap();
        let aligned = root.state.try_lock().ok().map(|state| {
            state.pending_consumer.as_ref().or(state.consumers.as_ref()).unwrap().pointer.is_none_or(|pointer| pointer.as_ptr() as usize % layout.align() == 0)
        }).unwrap();
        root.begin_close().unwrap();
        for _ in 0..16 { if root.terminal_is_empty() { break; } root.close_step(grant).unwrap(); }
        COUNT_ALLOCATIONS.with(|value| value.set(false));
        let allocations = ALLOCATIONS.with(std::cell::Cell::get); let deallocations = DEALLOCATIONS.with(std::cell::Cell::get);
        let allocated_layouts = ALLOCATION_LAYOUTS.with(std::cell::Cell::get); let freed_layouts = DEALLOCATION_LAYOUTS.with(std::cell::Cell::get);
        close_admission_root(&root);
        assert!(aligned, "{name}"); assert_eq!(used, ResidentResources::new(layout.size() as u64, 1, 1).unwrap());
        let expected = fixture["nativeOwnership"]["exactAllocation"]["cancelAllocations"][frontier].as_u64().unwrap() as usize;
        let expected_frees = fixture["nativeOwnership"]["exactAllocation"]["cancelDeallocations"][frontier].as_u64().unwrap() as usize;
        assert_eq!((allocations, deallocations), (expected, expected_frees), "{name}");
        assert_eq!(allocated, (expected * layout.size()) as u64, "{name}");
        if expected == 1 { assert_eq!((allocated_layouts[0].1, allocated_layouts[0].2), (layout.size(), layout.align())); assert_eq!(freed_layouts[0], (layout.size(), layout.align())); }
    }
}

#[test]
fn resident_admission_injected_allocation_failure_preserves_original_reservations() {
    let fixture = admission_fixture(); let grant = full_admission_grant();
    assert_eq!(fixture["nativeOwnership"]["exactAllocation"]["failure"], "injected-null");
    let root = ResidentLedgerRoot::new(admission_capacity());
    root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap();
    let consumer_charge = root.usage(ResidentPartition::Data).unwrap();
    FAIL_NEXT_ALLOCATION.with(|value| value.set(true));
    let consumer_failed = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant);
    let consumer_retained = (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
    close_admission_root(&root);
    assert_eq!(consumer_failed, Err(ResidentFault::Allocation)); assert_eq!(consumer_retained, (consumer_charge, 0));
    for record_failure in [false, true] {
        let drops = Arc::new(AtomicUsize::new(0));
        let root = ResidentLedgerRoot::new(admission_capacity()); let consumer = admitted_consumer(&root, &mut Some(ResidentDropProbe::new(&drops)));
        let original = consumer.read().unwrap().unwrap().pointer();
        let ledger = root.ledger();
        ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap();
        let mut cell = None;
        if record_failure {
            for _ in 0..8 { if ledger.prepared_admission(&consumer).unwrap().is_some() { break; } ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
            cell = ledger.prepared_admission(&consumer).unwrap(); ledger.claim_admission(&consumer, cell.as_ref().unwrap(), grant).unwrap();
        }
        let before = (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
        FAIL_NEXT_ALLOCATION.with(|value| value.set(true));
        let failure = if record_failure { ledger.reserve_record::<ResidentDropProbe, AlignedResident>(cell.as_ref().unwrap(), ResidentResources::new(64, 1, 1).unwrap(), grant) } else { ledger.prepare_admission(&consumer, ResidentPartition::Data, grant) };
        let after = (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
        let same_source = consumer.read().unwrap().unwrap().pointer() == original;
        let drops_before_recovery = drops.load(Ordering::SeqCst);
        root.begin_close().unwrap(); let mut recovered = None; consumer.handoff_for_close_into(&mut recovered, grant).unwrap();
        let recovered_original = recovered.as_ref().unwrap().pointer() == original; retire_parent_probe(&mut recovered);
        drop(cell); drop(ledger); drop(consumer); close_admission_root(&root);
        assert_eq!(failure, Err(ResidentFault::Allocation)); assert_eq!(after, before); assert!(same_source && recovered_original);
        assert_eq!(drops_before_recovery, 0); assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn resident_admission_private_consumer_phase_qualifies_access_and_record_aliases() {
    let fixture = admission_fixture(); let grant = full_admission_grant();
    for vector in fixture["nativeOwnership"]["phaseAccess"].as_array().unwrap() {
        let root = ResidentLedgerRoot::new(admission_capacity()); let consumer = admitted_consumer(&root, &mut Some(AlignedResident(73)));
        let ledger = root.ledger();
        for _ in 0..8 { if ledger.prepared_admission(&consumer).unwrap().is_some() { break; } ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
        let cell = ledger.prepared_admission(&consumer).unwrap().unwrap(); ledger.claim_admission(&consumer, &cell, grant).unwrap();
        let envelope = ResidentResources::new(64, 1, 1).unwrap();
        for _ in 0..8 { if cell.record::<AlignedResident>().unwrap().is_some() { break; } ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, grant).unwrap(); }
        let record = cell.record::<AlignedResident>().unwrap().unwrap();
        match vector["phase"].as_str().unwrap() { "consumer-closing" => { consumer.begin_close(grant).unwrap(); }, "root-closing" => { root.begin_close().unwrap(); }, "open" => {}, _ => unreachable!() }
        let expected = vector["forward"].as_bool().unwrap(); let recovery = vector["recovery"].as_bool().unwrap();
        let forwards = (root.prepared_consumer::<AlignedResident>().is_ok(), consumer.read().is_ok(), cell.record::<AlignedResident>().is_ok());
        let recoveries = (root.recover_consumer_for_close::<AlignedResident>().is_ok(), consumer.read_for_close().is_ok(), cell.recover_record_for_close::<AlignedResident>().is_ok());
        let mut attempted = Some(AlignedResident(91)); let install = record.install(&mut attempted, grant).unwrap().kind;
        root.begin_close().unwrap(); let mut shell = None;
        if attempted.is_none() { record.handoff_into(&mut shell, grant).unwrap(); }
        let mut parent = None; consumer.handoff_for_close_into(&mut parent, grant).unwrap();
        drop(record); drop(cell); drop(ledger); drop(consumer); close_admission_root(&root);
        assert_eq!(forwards, (expected, expected, expected)); assert_eq!(recoveries, (recovery, recovery, recovery));
        assert_eq!(install, if expected { ResidentStepKind::Ready } else { ResidentStepKind::Rejected });
        assert_eq!(parent.unwrap().0, 73); assert_eq!(attempted.or(shell).unwrap().0, 91);
    }
}

#[test]
fn resident_admission_all_three_page_backings_use_exact_layout_and_short_grants() {
    let root = ResidentLedgerRoot::new(admission_capacity()); let grant = full_admission_grant();
    let layouts = [std::alloc::Layout::new::<super::ConsumerNode<AlignedResident>>(), std::alloc::Layout::new::<super::AdmissionNode>(), std::alloc::Layout::new::<super::RecordNode<AlignedResident>>()];
    let envelope = ResidentResources::new(64, 1, 1).unwrap();
    let reserve_work = [std::mem::size_of::<Option<super::ConsumerPage>>(), std::mem::size_of::<u64>(), std::mem::size_of::<ResidentResources>(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
    let allocate_work = [layouts[0].size(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>(), std::mem::size_of::<u64>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
    assert!(reserve_work > 0 && allocate_work > 0 && reserve_work <= grant.max_bytes() && allocate_work <= grant.max_bytes());
    ALLOCATIONS.with(|value| value.set(0)); DEALLOCATIONS.with(|value| value.set(0));
    ALLOCATION_LAYOUTS.with(|value| value.set([(0, 0, 0); 8])); DEALLOCATION_LAYOUTS.with(|value| value.set([(0, 0); 8]));
    COUNT_ALLOCATIONS.with(|value| value.set(true));
    let mut refused = [ResidentStepKind::Pending; 8];
    let mut short_consumer_allocations = 0;
    let mut consumer_unchanged = true;
    let mut exact_consumer_work = true;
    for (phase, bytes) in [reserve_work, allocate_work].into_iter().enumerate() {
        let before = (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
        for (index, short) in [ResidentGrant::new(0, bytes).unwrap(), ResidentGrant::new(1, 0).unwrap(), admission_grant(bytes.checked_sub(1).unwrap())].into_iter().enumerate() {
            refused[phase * 3 + index] = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, short).unwrap().kind;
            consumer_unchanged &= before == (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
            short_consumer_allocations = ALLOCATIONS.with(std::cell::Cell::get);
        }
        let exact = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, admission_grant(bytes)).unwrap();
        exact_consumer_work &= bytes <= grant.max_bytes() && exact.items == 1 && exact.bytes == bytes && exact.kind == ResidentStepKind::Pending;
    }
    for _ in 0..2 { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
    let consumer = root.prepared_consumer::<AlignedResident>().unwrap().unwrap(); let ledger = root.ledger();
    for _ in 0..3 { ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
    let cell = ledger.prepared_admission(&consumer).unwrap().unwrap(); ledger.claim_admission(&consumer, &cell, grant).unwrap();
    let before_record = ALLOCATIONS.with(std::cell::Cell::get);
    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[2].size() as u64 - 1)].into_iter().enumerate() { refused[index + 6] = ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, short).unwrap().kind; }
    let after_short_record = ALLOCATIONS.with(std::cell::Cell::get);
    for _ in 0..2 { ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, grant).unwrap(); }
    let record = cell.record::<AlignedResident>().unwrap().unwrap();
    let record_aligned = record.pointer.as_ptr() as usize % layouts[2].align() == 0;
    let usage = root.usage(ResidentPartition::Data).unwrap(); let allocated_bytes = root.allocated_bytes().unwrap();
    drop(record); drop(cell); drop(ledger); drop(consumer); root.begin_close().unwrap();
    for _ in 0..32 { if root.terminal_is_empty() { break; } root.close_step(grant).unwrap(); }
    COUNT_ALLOCATIONS.with(|value| value.set(false));
    let allocations = ALLOCATIONS.with(std::cell::Cell::get); let deallocations = DEALLOCATIONS.with(std::cell::Cell::get);
    let allocation_layouts = ALLOCATION_LAYOUTS.with(std::cell::Cell::get); let deallocation_layouts = DEALLOCATION_LAYOUTS.with(std::cell::Cell::get);
    close_admission_root(&root);
    assert_eq!(refused, [ResidentStepKind::Blocked; 8]); assert_eq!(short_consumer_allocations, 0); assert!(consumer_unchanged && exact_consumer_work); assert_eq!(before_record, after_short_record);
    assert!(record_aligned); assert_eq!((allocations, deallocations), (3, 3));
    for index in 0..3 { assert_eq!((allocation_layouts[index].1, allocation_layouts[index].2), (layouts[index].size(), layouts[index].align())); }
    for (index, layout) in [layouts[2], layouts[1], layouts[0]].into_iter().enumerate() { assert_eq!(deallocation_layouts[index], (layout.size(), layout.align())); }
    let expected_bytes = layouts.iter().map(|layout| layout.size() as u64).sum::<u64>();
    assert_eq!(allocated_bytes, expected_bytes); assert_eq!(usage, ResidentResources::new(expected_bytes, 3, 3).unwrap().checked_add(envelope).unwrap());
}
//#endregion 🧪️AdmissionOwnership

//#region 🧪️ReleaseBaseline
#[path = "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🧪️baseline/🦀️.rs"]
mod release_baseline;
//#endregion 🧪️ReleaseBaseline

//#region 🧪️ReleasePhases
#[path = "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs"]
mod release_phases;

pub(super) fn observe_release_destroy_returned() {
    release_phases::observe_destroy_returned();
    primary_recovery::observe_destroy_returned();
}
//#endregion 🧪️ReleasePhases

//#region 🧪️PrimaryRecovery
#[path = "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs"]
mod primary_recovery;

pub(super) fn observe_primary_recovery_pointer_load(registration: u64) {
    primary_recovery::observe_recovery_pointer_load(registration);
}
//#endregion 🧪️PrimaryRecovery
