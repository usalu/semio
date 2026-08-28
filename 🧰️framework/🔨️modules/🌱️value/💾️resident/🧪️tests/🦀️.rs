use super::{ResidentCapacity, ResidentFault, ResidentResources, RESIDENT_MAXIMUM_COUNT};

//#region 🔎️AllocationObservation
struct ObservedAllocator;

thread_local! {
    static COUNT_ALLOCATIONS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ALLOCATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        COUNT_ALLOCATIONS.with(|enabled| { if enabled.get() { ALLOCATIONS.with(|count| count.set(count.get() + 1)); } });
        unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) { unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System, ptr, layout) } }
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
