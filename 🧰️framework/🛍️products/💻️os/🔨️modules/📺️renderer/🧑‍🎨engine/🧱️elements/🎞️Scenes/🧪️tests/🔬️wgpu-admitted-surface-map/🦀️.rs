
use super::*;

#[test]
fn preserves_admission_order_with_constant_index_access() {
    let mut surfaces = AdmittedSurfaceMap::default();
    surfaces.try_insert("third".to_string(), 3usize).unwrap();
    surfaces.try_insert("first".to_string(), 1usize).unwrap();
    surfaces.try_insert("third".to_string(), 30usize).unwrap();
    assert_eq!(surfaces.id_at(0), Some("third"));
    assert_eq!(surfaces.id_at(1), Some("first"));
    assert_eq!(surfaces.id_at(2), None);
    assert_eq!(surfaces.get("third"), Some(&30));
}

#[test]
fn rejects_the_257th_surface_before_map_ownership() {
    let mut surfaces = AdmittedSurfaceMap::default();
    for index in 0..SCENE_SURFACE_CAPACITY {
        surfaces.try_insert(format!("surface-{index}"), index).unwrap();
    }
    let rejected = surfaces.try_insert("overflow".to_string(), SCENE_SURFACE_CAPACITY).expect_err("exact capacity owner");
    assert_eq!((rejected.id.as_str(), rejected.value), ("overflow", SCENE_SURFACE_CAPACITY));
    surfaces.retain_rejected(rejected).unwrap();
    assert_eq!(surfaces.len(), SCENE_SURFACE_CAPACITY);
    assert_eq!(surfaces.id_at(SCENE_SURFACE_CAPACITY), None);
    assert_eq!(surfaces.take_fault(), Some("scene surface item credits exceeded"));
}

#[test]
fn replacement_removal_and_clear_preserve_order_invariants() {
    let mut surfaces = AdmittedSurfaceMap::default();
    surfaces.try_insert("a".to_string(), 1usize).unwrap();
    surfaces.try_insert("b".to_string(), 2usize).unwrap();
    surfaces.try_insert("a".to_string(), 3usize).unwrap();
    assert_eq!(surfaces.id_at(0), Some("a"));
    assert_eq!(surfaces.id_at(1), Some("b"));
    assert_eq!(surfaces.remove("a"), Some(3));
    assert_eq!(surfaces.id_at(0), Some("b"));
    assert_eq!(surfaces.id_at(1), None);
    surfaces.begin_close();
    let mut closed = Vec::new();
    while let Some(owner) = surfaces.close_step() {
        closed.push((owner.id, owner.value));
    }
    assert!(surfaces.is_empty());
    assert!(surfaces.terminal_is_empty());
    assert_eq!(surfaces.id_at(0), None);
    assert_eq!(surfaces.take_fault(), None);
}

#[test]
fn replacement_and_slot_reuse_invalidate_surface_aba_tokens() {
    let mut surfaces = AdmittedSurfaceMap::default();
    surfaces.try_insert("surface".to_string(), 1usize).unwrap();
    let first = surfaces.token("surface").unwrap();
    surfaces.try_insert("surface".to_string(), 2usize).unwrap();
    let second = surfaces.token("surface").unwrap();
    assert_ne!(first, second);
    assert_eq!(surfaces.get_token(first), None);
    assert_eq!(surfaces.get_token(second), Some(&2));
    assert_eq!(surfaces.remove("surface"), Some(2));
    surfaces.try_insert("replacement".to_string(), 3usize).unwrap();
    assert_eq!(surfaces.get_token(second), None);
}

#[test]
fn production_surface_authority_has_no_hash_map_or_structural_deref() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    let authority = source.split("#[cfg(test)]\nmod admitted_surface_map_tests").next().unwrap();
    assert!(!authority.contains("values: HashMap<String, T>"));
    assert!(!authority.contains("DerefMut"));
    assert!(authority.contains("slots: Box<[Option<AdmittedSurfaceEntry<T>>; SCENE_SURFACE_CAPACITY]>"));
}

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`semio_framework_async::BOXED_FIXED_SLOTS_FIXTURE`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack() {
    let fixture: Value = serde_json::from_str(semio_framework_async::BOXED_FIXED_SLOTS_FIXTURE).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "renderer::scenes")
        .map(|table| semio_framework_async::FixedSlotTableBudget::new(table["owner"].as_str().expect("owner"), table["capacity"].as_u64().expect("capacity") as usize, table["elementSizeBytes"].as_u64().expect("element bytes") as usize, table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize))
        .collect();
    let measured = vec![
        semio_framework_async::FixedSlotTableBudget::new("scenes::AdmittedSurfaceMap<World3dState>", SCENE_SURFACE_CAPACITY, size_of::<Option<AdmittedSurfaceEntry<infinite_world::world::World3dState>>>(), size_of::<AdmittedSurfaceMap<infinite_world::world::World3dState>>()),
    ];
    semio_framework_async::assert_fixed_slot_tables(
        "renderer::scenes",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
        drop(AdmittedSurfaceMap::<infinite_world::world::World3dState>::default());
        },
    );
}
