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
