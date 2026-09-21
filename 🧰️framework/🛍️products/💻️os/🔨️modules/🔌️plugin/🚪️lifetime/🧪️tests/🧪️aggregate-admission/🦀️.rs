//#region 🧪️NativeAggregateBackingAdmission
use super::*;

#[test]
fn native_aggregate_registry_does_not_allocate_backing_before_admission() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛂️aggregate-admission.json")).unwrap();
    assert_eq!(PLUGIN_RUNTIME_INSTANCE_SLOTS as u64, fixture["registry"]["logicalSlots"].as_u64().unwrap());
    let mut registry = RuntimeInstanceRegistry::<RuntimeActorAuthority>::new();
    let initialized_slots = registry.slots.len();
    let retained_bytes = size_of_val(registry.slots.as_ref());
    let occupied = !registry.is_empty();
    let admitted = registry.allocation_admitted;
    assert!(!occupied, "a fresh runtime registry holds no instance");
    assert_eq!(initialized_slots, 0, "the original runtime registry needs caller-granted backing before slot initialization");
    assert_eq!(retained_bytes, 0, "an unadmitted registry retains no slot bytes at all");
    assert!(!admitted, "allocator success cannot mint the upstream allocation permit");

    assert!(registry.can_insert(7), "the first admission gate is what takes the backing");
    assert!(registry.allocation_admitted, "an admitted gate leaves the backing granted");
    assert_eq!(registry.slots.len() as u64, fixture["registry"]["logicalSlots"].as_u64().unwrap(), "backing, once taken, is the whole fixed slot table");
    assert_eq!(size_of_val(registry.slots.as_ref()), PLUGIN_RUNTIME_INSTANCE_SLOTS * size_of::<std::mem::MaybeUninit<(u32, RuntimeActorAuthority)>>(), "the deferred reservation is the exact table the eager one used to take up front");
    assert!(registry.is_empty(), "taking backing admits no instance");
    drop(registry);
}
//#endregion 🧪️NativeAggregateBackingAdmission
