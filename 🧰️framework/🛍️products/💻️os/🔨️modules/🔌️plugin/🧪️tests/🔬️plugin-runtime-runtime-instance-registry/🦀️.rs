mod runtime_instance_registry_tests {
    use super::*;

    #[test]
    fn sparse_live_instances_receive_successive_round_robin_turns() {
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🧪️fixtures/🔣️.json")).unwrap();
        let mut registry = RuntimeInstanceRegistry::new();
        for id in fixture["instances"].as_array().unwrap() {
            registry.insert_admitted(id.as_u64().unwrap() as u32, ());
        }
        let mut cursor = 0;
        let actual: Vec<_> = fixture["roundRobin"]
            .as_array()
            .unwrap()
            .iter()
            .map(|_| {
                let (index, id, _) = registry.next_entry_from(cursor).expect("next occupied instance");
                cursor = index + 1;
                id
            })
            .collect();
        assert_eq!(serde_json::to_value(actual).unwrap(), fixture["roundRobin"]);
        for id in fixture["instances"].as_array().unwrap() {
            registry.take(id.as_u64().unwrap() as u32);
        }
        assert!(registry.next_entry_from(cursor).is_none());
    }

    #[test]
    fn runtime_instance_registry_has_fixed_capacity_collision_and_reuse() {
        let mut registry = RuntimeInstanceRegistry::new();
        assert!(registry.allocation_admitted);
        for index in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS as u32 {
            assert!(registry.insert(index.saturating_add(100_000), index).is_ok());
        }
        assert_eq!(registry.insert(100_000 + PLUGIN_RUNTIME_INSTANCE_SLOTS as u32, u32::MAX), Err(u32::MAX));
        assert_eq!(registry.take(100_000), Some(0));
        assert!(registry.insert(100_000 + PLUGIN_RUNTIME_INSTANCE_SLOTS as u32, 7).is_ok());
        assert_eq!(registry.get(100_000 + PLUGIN_RUNTIME_INSTANCE_SLOTS as u32), Some(&7));
    }

    #[test]
    fn runtime_instance_close_quarantine_never_implicitly_drops_nested_value() {
        struct DropSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl Drop for DropSentinel {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut live = RuntimeInstanceRegistry::new();
        let mut quarantine = RuntimeInstanceRegistry::new();
        assert!(live.insert(73, DropSentinel(drops.clone())).is_ok());
        let detached = live.take(73).expect("exact live instance");
        assert!(quarantine.insert(73, detached).is_ok());
        assert_eq!(drops.load(Ordering::SeqCst), 0, "the close handoff must not run the nested destructor");
        drop(live);
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(quarantine);
        assert_eq!(drops.load(Ordering::SeqCst), 0, "an incomplete registry shell must fail safe without walking or dropping nested values");
    }

    #[test]
    fn exhausted_close_generation_is_rejected_before_exact_owner_detachment() {
        struct DropSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl Drop for DropSentinel {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut live = RuntimeInstanceRegistry::new();
        live.insert_admitted(91, DropSentinel(drops.clone()));
        assert!(checked_runtime_close_generation(u64::MAX).is_err());
        assert!(live.get(91).is_some(), "generation admission must precede owner detachment");
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(live.take(91));
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn cleanup_queue_saturation_preserves_detached_app_ownership() {
        let instance_id = 700_000u32;
        let exact_owner = std::sync::Arc::new(73usize);
        let mut live = RuntimeInstanceRegistry::new();
        let mut quarantine = RuntimeInstanceRegistry::new();
        live.insert_admitted(instance_id, exact_owner.clone());
        for index in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS as u32 {
            quarantine.insert_admitted(instance_id.saturating_add(index), std::sync::Arc::new(index as usize));
        }
        assert!(!quarantine.can_insert(instance_id), "saturated quarantine must reject before live detachment");
        if quarantine.can_insert(instance_id) {
            let detached = live.take(instance_id).expect("preflight admitted exact owner");
            quarantine.insert_admitted(instance_id, detached);
        }
        let retained = live.get(instance_id).expect("rejected close retains the live owner");
        assert!(std::sync::Arc::ptr_eq(retained, &exact_owner), "saturation hands back the exact owner, not a clone or reconstruction");
        drop(live.take(instance_id));
        for index in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS as u32 {
            drop(quarantine.take(instance_id.saturating_add(index)));
        }
        assert!(quarantine.is_empty());
    }
}
