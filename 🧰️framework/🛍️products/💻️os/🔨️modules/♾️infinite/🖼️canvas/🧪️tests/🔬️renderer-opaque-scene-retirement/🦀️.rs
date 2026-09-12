mod opaque_scene_retirement_tests {
    use super::*;

    #[test]
    fn fixed_quarantine_saturates_before_scene_ownership_transfer() {
        let mut registry = OpaqueSceneRetirementRegistry::default();
        for _ in 0..OPAQUE_SCENE_RETIREMENT_CAPACITY {
            let token = registry.reserve().expect("fixed quarantine credit");
            registry.publish(token, Scene::new());
        }
        assert!(registry.reserve().is_none());
        assert!(registry.faulted);
        assert_eq!(registry.active(), OPAQUE_SCENE_RETIREMENT_CAPACITY);
    }

    #[test]
    fn late_opaque_scene_token_is_rejected_before_owner_publication() {
        let mut registry = OpaqueSceneRetirementRegistry::default();
        let token = registry.reserve().expect("fixed quarantine credit");
        registry.publish(token, Scene::new());
        assert!(registry.token_is_current(token));
        assert_eq!(registry.advance(token, 1, 1), OpaqueSceneRetirementStep::Complete { released_items: 1, credited_bytes: 0, released_bytes: 0 });
        assert!(!registry.token_is_current(token));
        assert_eq!(registry.advance(token, 1, 1), OpaqueSceneRetirementStep::Fault);
    }

    #[test]
    fn terminal_opaque_scene_slots_are_reused_after_exact_cursor_drain() {
        let mut registry = OpaqueSceneRetirementRegistry::default();
        for _ in 0..=OPAQUE_SCENE_RETIREMENT_CAPACITY {
            let token = registry.reserve().expect("terminal opaque scene slot is reusable");
            let mut scene = Scene::new();
            scene.pop_layer();
            registry.publish(token, scene);
            assert_eq!(registry.advance(token, 1, usize::MAX), OpaqueSceneRetirementStep::Pending { released_items: 0, credited_bytes: 0, released_bytes: 0 });
            assert_eq!(registry.advance(token, 1, usize::MAX), OpaqueSceneRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 });
            assert!(matches!(registry.advance(token, 1, usize::MAX), OpaqueSceneRetirementStep::Complete { released_items: 1, .. }));
            assert_eq!(registry.active(), 0);
        }
        assert!(!registry.faulted);
    }
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
fn opaque_scene_retirement_slot_table_is_heap_first_and_fits_a_bounded_thread_stack() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_async::BOXED_FIXED_SLOTS_FIXTURE).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "infinite::canvas")
        .map(|table| semio_framework_async::FixedSlotTableBudget::new(table["owner"].as_str().expect("owner"), table["capacity"].as_u64().expect("capacity") as usize, table["elementSizeBytes"].as_u64().expect("element bytes") as usize, table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize))
        .collect();
    let measured = vec![
        semio_framework_async::FixedSlotTableBudget::new("canvas::renderer::OpaqueSceneRetirementRegistry", OPAQUE_SCENE_RETIREMENT_CAPACITY, size_of::<OpaqueSceneRetirementSlot>(), size_of::<OpaqueSceneRetirementRegistry>()),
    ];
    semio_framework_async::assert_fixed_slot_tables(
        "infinite::canvas",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(OpaqueSceneRetirementRegistry::default());
        },
    );
}
