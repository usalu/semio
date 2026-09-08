mod child_member_registry_tests {
    use super::*;

    fn dialect() -> ArtifactDialect {
        ArtifactDialect { artifact_kind: "test-child".into(), standard: "native".into(), subset: "*".into() }
    }

    #[test]
    fn fixed_child_member_registry_admits_exact_capacity_rejects_plus_one_and_cursor_detaches_every_owner() {
        let mut registry = ChildMemberRegistry::new();
        for index in 0..CHILD_CONTENT_SLOTS {
            let key = (format!("slot-{index}"), format!("child-{index}"));
            let admission = registry.admit(&key).expect("exact fixed child capacity");
            registry.insert_admitted(admission, key, (dialect(), index));
        }
        let rejected_key = ("slot-plus-one".to_string(), "child-plus-one".to_string());
        assert!(registry.admit(&rejected_key).is_err(), "capacity plus one is rejected before an owner is transferred");
        for index in 0..CHILD_CONTENT_SLOTS {
            assert!(registry.take_at(index).is_some(), "one exact child owner detaches per cursor step");
        }
        assert!(registry.is_empty());
    }

    #[test]
    fn fixed_child_member_registry_resolves_hash_collisions_without_replacement() {
        let mut first_by_hash: [Option<(String, String)>; CHILD_CONTENT_SLOTS] = std::array::from_fn(|_| None);
        let mut collision = None;
        for index in 0..=CHILD_CONTENT_SLOTS {
            let key = ("slot".to_string(), format!("collision-{index}"));
            let hash = ChildMemberRegistry::<usize>::hash(&key.0, &key.1).expect("bounded key hash");
            if let Some(first) = first_by_hash[hash].take() {
                collision = Some((first, key));
                break;
            }
            first_by_hash[hash] = Some(key);
        }
        let (first, second) = collision.expect("pigeonhole collision across capacity plus one keys");
        let mut registry = ChildMemberRegistry::new();
        let first_admission = registry.admit(&first).expect("first colliding owner");
        registry.insert_admitted(first_admission, first.clone(), (dialect(), 1));
        let second_admission = registry.admit(&second).expect("second colliding owner probes to a distinct fixed slot");
        registry.insert_admitted(second_admission, second.clone(), (dialect(), 2));
        assert_eq!(registry.get(&first).map(|(_, owner)| *owner), Some(1));
        assert_eq!(registry.get(&second).map(|(_, owner)| *owner), Some(2));
        for index in 0..CHILD_CONTENT_SLOTS {
            drop(registry.take_at(index));
        }
    }

    #[test]
    fn stale_child_member_admission_cannot_cancel_a_reused_slot_generation() {
        let key = ("slot".to_string(), "child".to_string());
        let mut registry = ChildMemberRegistry::new();
        let stale = registry.admit(&key).expect("first admission");
        assert!(registry.cancel_admission(&stale));
        let current = registry.admit(&key).expect("same direct slot is reused with a fresh generation");
        assert_ne!(stale.generation, current.generation);
        assert!(!registry.cancel_admission(&stale), "stale generation cannot cancel the reused reservation");
        registry.insert_admitted(current, key, (dialect(), 7));
        for index in 0..CHILD_CONTENT_SLOTS {
            drop(registry.take_at(index));
        }
    }

    #[test]
    fn incomplete_child_member_registry_drop_faults_in_release_instead_of_destroying_nested_owners() {
        let result = std::panic::catch_unwind(|| {
            let key = ("slot".to_string(), "retained".to_string());
            let mut registry = ChildMemberRegistry::new();
            let admission = registry.admit(&key).expect("exact retained admission");
            registry.insert_admitted(admission, key, (dialect(), vec![0u8; 64 * 1024]));
        });
        assert!(result.is_err(), "ordinary incomplete Drop is observably fail-closed and MaybeUninit keeps the nested owner from implicit destruction");
    }
}
