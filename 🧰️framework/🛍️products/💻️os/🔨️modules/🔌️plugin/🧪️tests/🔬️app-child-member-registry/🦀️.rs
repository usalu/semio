mod child_member_registry_tests {
    use super::*;

    fn dialect() -> ArtifactDialect {
        ArtifactDialect { artifact_kind: "test-child".into(), standard: "native".into(), subset: "*".into() }
    }

    fn insert<M>(registry: &mut ChildMemberRegistry<M>, key: (String, String), member: M) {
        let admission = registry.admit(&key).expect("exact child admission");
        let reference = ArtifactRef { artifact_id: key.1.clone(), dialect: dialect() };
        let owner = store::OwnerRef {
            parent: ArtifactRef { artifact_id: "parent".into(), dialect: dialect() },
            slot: key.0,
            child_id: key.1,
        };
        registry.insert_admitted(admission, reference, owner, member);
    }

    fn member_ingress(ordinal: usize, operation: u64, generation: u64, slot: &str, child_id: &str) -> OwnedDocumentMemberIngress {
        let reference = ArtifactRef { artifact_id: child_id.into(), dialect: dialect() };
        let owner = store::OwnerRef {
            parent: ArtifactRef { artifact_id: "parent".into(), dialect: dialect() },
            slot: slot.into(),
            child_id: child_id.into(),
        };
        let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).expect("one retained member page credit");
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(&[1, 2, 3]).expect("bounded member page")).unwrap_or_else(|_| panic!("pre-admitted member page"));
        pages.seal().expect("complete member page set");
        let request = store::MemberOpenRequest::new(
            semio_framework_job::OperationId(operation),
            semio_framework_job::Generation(generation),
            999,
            reference.clone(),
            Some(owner.clone()),
            pages,
        )
        .admit(1)
        .unwrap_or_else(|_| panic!("valid retained member request"));
        OwnedDocumentMemberIngress::try_new(ordinal, reference, owner, request).unwrap_or_else(|_| panic!("exact member ingress"))
    }

    fn close_ingress(mut ingress: OwnedDocumentMemberIngress) {
        for _ in 0..4096 {
            match ingress.close_step(1, 1).expect("bounded ingress close") {
                PluginCloseStep::Complete => {
                    assert!(ingress.terminal_is_empty());
                    drop(ingress);
                    return;
                }
                PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 1),
                PluginCloseStep::Blocked { reason } | PluginCloseStep::AwaitingInput { reason } => panic!("unexpected ingress close block: {reason}"),
            }
        }
        panic!("member ingress did not retire within its bounded owner count");
    }

    #[test]
    fn fixed_child_member_registry_admits_exact_capacity_rejects_plus_one_and_cursor_detaches_every_owner() {
        let mut registry = ChildMemberRegistry::new();
        for index in 0..CHILD_CONTENT_SLOTS {
            let key = (format!("slot-{index}"), format!("child-{index}"));
            insert(&mut registry, key, index);
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
        insert(&mut registry, first.clone(), 1);
        insert(&mut registry, second.clone(), 2);
        assert_eq!(registry.get(&first).map(|entry| entry.member), Some(1));
        assert_eq!(registry.get(&second).map(|entry| entry.member), Some(2));
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
        let reference = ArtifactRef { artifact_id: key.1.clone(), dialect: dialect() };
        let owner = store::OwnerRef { parent: ArtifactRef { artifact_id: "parent".into(), dialect: dialect() }, slot: key.0, child_id: key.1 };
        registry.insert_admitted(current, reference, owner, 7);
        for index in 0..CHILD_CONTENT_SLOTS {
            drop(registry.take_at(index));
        }
    }

    #[test]
    fn incomplete_child_member_registry_drop_faults_in_release_instead_of_destroying_nested_owners() {
        let result = std::panic::catch_unwind(|| {
            let key = ("slot".to_string(), "retained".to_string());
            let mut registry = ChildMemberRegistry::new();
            insert(&mut registry, key, vec![0u8; 64 * 1024]);
        });
        assert!(result.is_err(), "ordinary incomplete Drop is observably fail-closed and MaybeUninit keeps the nested owner from implicit destruction");
    }

    #[test]
    fn owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally() {
        assert!(std::mem::size_of::<OwnedDocumentMemberIngressRegistry>() < 1024, "the 1,024 request slots stay behind one heap owner");
        let mut registry = OwnedDocumentMemberIngressRegistry::try_new(2).expect("two exact ingress slots");
        registry.admit(member_ingress(0, 7, 11, "a", "child-a")).unwrap_or_else(|_| panic!("first ordinal"));
        let (_, duplicate) = registry.admit(member_ingress(0, 7, 11, "a", "child-a-duplicate")).expect_err("duplicate ordinal returns its exact owner");
        close_ingress(duplicate);
        assert!(registry.seal().is_err(), "missing ordinal is an exact closure fault");
        assert_eq!(registry.close_step(0, 0).expect("zero grant"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..8192 {
            let step = registry.close_step(1, 1).expect("bounded never-opened request retirement");
            if step == PluginCloseStep::Complete {
                assert!(registry.terminal_is_empty());
                drop(registry);
                return;
            }
            if let PluginCloseStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1 && released_bytes <= 1);
            }
        }
        panic!("never-opened ingress registry did not retire within its bounded owner count");
    }

    #[test]
    fn owned_document_ingress_refuses_extra_ordinal_and_preserves_the_returned_request() {
        let ingress = member_ingress(2, 7, 11, "extra", "child-extra");
        let mut registry = OwnedDocumentMemberIngressRegistry::try_new(2).expect("two exact ingress slots");
        let (_, ingress) = registry.admit(ingress).expect_err("extra ordinal is rejected unchanged");
        close_ingress(ingress);
        assert!(registry.seal().is_err());
        assert_eq!(registry.close_step(1, 1).expect("empty registry close"), PluginCloseStep::Complete);
        drop(registry);
    }

    #[test]
    fn displaced_composition_pins_retire_under_exact_byte_and_item_grants() {
        let mut retirement = CompositionPinsRetirement::new(vec![vcs::CompositionPin {
            child_ref: ArtifactRef { artifact_id: "child-δ".into(), dialect: dialect() },
            checkpoint_id: "checkpoint-α".into(),
        }]);
        assert_eq!(retirement.close_step(0, 0), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(retirement.close_step(1, 1), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..512 {
            let step = retirement.close_step(1, 2);
            if step == PluginCloseStep::Complete {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
            if let PluginCloseStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1 && released_bytes <= 2);
            }
        }
        panic!("composition pin retirement did not reach exact terminal emptiness");
    }
}
