//#region 🧪️NativeAggregateBackingAdmission
use super::*;

#[test]
fn native_aggregate_registry_does_not_allocate_backing_before_admission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🛂️aggregate-admission.json")).unwrap();
    assert_eq!(PLUGIN_RUNTIME_INSTANCE_SLOTS as u64, fixture["registry"]["logicalSlots"].as_u64().unwrap());
    let registry = RuntimeInstanceRegistry::<RuntimeActorAuthority>::new();
    let initialized_slots = registry.slots.len();
    let retained_bytes = std::mem::size_of_val(registry.slots.as_ref());
    let occupied = !registry.is_empty();
    let admitted = registry.allocation_admitted;
    drop(registry);
    eprintln!("[DEBUG] native aggregate actor registry initialized_slots={initialized_slots} retained_bytes={retained_bytes} occupied={occupied} admitted={admitted}");
    assert!(!occupied);
    assert_eq!(initialized_slots, 0, "the original runtime registry needs caller-granted backing before slot initialization");
    assert_eq!(retained_bytes, 0);
    assert!(!admitted, "allocator success cannot mint the upstream allocation permit");
}
//#endregion 🧪️NativeAggregateBackingAdmission
