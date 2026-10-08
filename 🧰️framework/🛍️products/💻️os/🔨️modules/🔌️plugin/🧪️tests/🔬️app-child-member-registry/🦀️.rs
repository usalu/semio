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
            protocol::ActorId("actor:child-registry-fixture".into()),
        )
        .admit(1)
        .unwrap_or_else(|_| panic!("valid retained member request"));
        OwnedDocumentMemberIngress::try_new(ordinal, reference, owner, request).unwrap_or_else(|_| panic!("exact member ingress"))
    }

    fn close_ingress(mut ingress: OwnedDocumentMemberIngress) {
        for _ in 0..4096 {
            let bytes = ingress.next_close_byte_demand().max(1);
            assert!(bytes <= 262_144);
            match ingress.close_step(1, bytes).expect("bounded ingress close") {
                PluginCloseStep::Complete => {
                    assert!(ingress.terminal_is_empty());
                    drop(ingress);
                    return;
                }
                PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= bytes),
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
            let hash = ChildMemberRegistry::<usize>::hash(MemberKeyRef::root(&key.0, &key.1)).expect("bounded key hash");
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
            let bytes = registry.next_close_byte_demand().max(1);
            assert!(bytes <= 262_144);
            let step = registry.close_step(1, bytes).expect("bounded never-opened request retirement");
            if step == PluginCloseStep::Complete {
                assert!(registry.terminal_is_empty());
                drop(registry);
                return;
            }
            if let PluginCloseStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1 && released_bytes <= bytes);
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
        for _ in 0..4 { let bytes = registry.next_close_byte_demand().max(1); registry.close_step(1, bytes).expect("empty registry close"); if registry.terminal_is_empty() { break; } }
        assert!(registry.terminal_is_empty());
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

    #[test]
    fn prepared_child_content_known_root_alias_releases_zero_and_external_reader_still_blocks() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔗️retained-alias/🔣️.json")).unwrap();
        assert_eq!(fixture["cases"].as_array().unwrap().len(), 6);
        let retained = ChildContentView { root: Some(std::sync::Arc::new(ChildContentRoot::default())) };
        let mut alias = retained.clone();
        let pointer = std::sync::Arc::as_ptr(alias.root.as_ref().unwrap());
        let zero = store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 1 };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| alias.close_prepared_structure_step(&retained, zero).unwrap());
        assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(std::sync::Arc::as_ptr(alias.root.as_ref().unwrap()), pointer);
        let one = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| alias.close_prepared_structure_step(&retained, one).unwrap());
        assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(alias.root.is_none());
        assert_eq!(std::sync::Arc::strong_count(retained.root.as_ref().unwrap()), 1);
        let mut exclusive = retained;
        let external = exclusive.clone();
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| exclusive.close_prepared_structure_step(&ChildContentView::EMPTY, one).unwrap());
        assert_eq!(step, PluginCloseStep::Blocked { reason: "private child root remains externally borrowed" });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(external)).1.released_bytes, 0);
        let bytes = child_content_arc_bytes::<ChildContentRoot>();
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| exclusive.close_prepared_structure_step(&ChildContentView::EMPTY, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes - 1 }).unwrap());
        assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| exclusive.close_prepared_structure_step(&ChildContentView::EMPTY, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap());
        assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, bytes));
        println!("[DEBUG] prepared child exact retained root alias releases0; unrelated reader blocks until return; original root frame{bytes} denies one-below and frees exactly once");
    }

    #[test]
    fn prepared_child_content_metadata_retains_denied_backing_and_reports_each_physical_release() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
        let visibility = vcs::ArtifactGroupVisibilityOwner::new();
        let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
        for row in fixture["cases"].as_array().unwrap() {
            let physical = row["physicalBytes"].as_u64().unwrap() as usize;
            let caller = row["callerBytes"].as_u64().unwrap() as usize;
            let mut text = String::with_capacity(physical);
            text.push('x');
            assert_eq!(text.capacity(), physical);
            let pointer = text.as_ptr();
            let mut owner = PreparedChildContentEntry { entry: std::mem::ManuallyDrop::new(None), metadata: std::mem::ManuallyDrop::new(Some([text, String::new(), String::new(), String::new(), String::new(), String::new(), String::new()])), metadata_cursor: 0, visibility: visibility.view(), generation: 7 };
            assert_eq!(owner.next_close_byte_demand(), physical);
            for grant in [store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: physical }, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: physical - 1 }] {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_metadata_step(grant).unwrap());
                assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(owner.metadata.as_ref().unwrap()[0].as_ptr(), pointer);
                assert_eq!(owner.next_close_byte_demand(), physical);
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_metadata_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: caller }).unwrap());
            let expected = row["releasedBytes"].as_u64().unwrap() as usize;
            assert_eq!(step, PluginCloseStep::Pending { released_items: usize::from(expected != 0), released_bytes: expected });
            assert_eq!((events.requested_bytes, events.released_bytes), (0, expected));
            let mut paid = expected;
            for _ in 0..9 {
                let demand = owner.next_close_byte_demand();
                assert!(demand <= admission);
                assert!(owner.terminal_is_empty() || demand != 0, "every remaining metadata action publishes a usable grant");
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_metadata_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand }).unwrap());
                assert_eq!(events.requested_bytes, 0);
                match step {
                    PluginCloseStep::Pending { released_items, released_bytes } => {
                        assert_eq!(released_items, 1);
                        assert!(released_bytes <= demand);
                        assert_eq!(events.released_bytes, released_bytes);
                        paid += released_bytes;
                    }
                    PluginCloseStep::Complete => { assert_eq!(events.released_bytes, 0); break; }
                    _ => panic!("exclusive prepared metadata must converge"),
                }
            }
            assert!(owner.terminal_is_empty());
            assert_eq!(paid, physical);
            println!("[DEBUG] prepared child metadata physical={physical} denied-grant={} actual-paid-release={paid}", physical - 1);
        }
    }

    #[test]
    fn prepared_child_content_empty_scaffolds_release_whole_arc_frames_in_distinct_turns() {
        let ((mut view, page_bytes, root_bytes), birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            let page = std::sync::Arc::new(ChildContentPage::default());
            let mut root = ChildContentRoot::default();
            root.pages[0] = Some(page);
            (ChildContentView { root: Some(std::sync::Arc::new(root)) }, child_content_arc_bytes::<ChildContentPage>(), child_content_arc_bytes::<ChildContentRoot>())
        });
        assert_eq!(birth.requested_bytes, page_bytes + root_bytes);
        assert_eq!(birth.released_bytes, 0);
        for physical in [page_bytes, root_bytes] {
            assert_eq!(view.prepared_structure_close_byte_demand(), physical);
            let root_pointer = std::sync::Arc::as_ptr(view.root.as_ref().unwrap());
            for grant in [store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: physical }, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: physical - 1 }] {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| view.close_prepared_structure_step(&ChildContentView::EMPTY, grant).unwrap());
                assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(std::sync::Arc::as_ptr(view.root.as_ref().unwrap()), root_pointer);
                assert_eq!(view.prepared_structure_close_byte_demand(), physical);
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| view.close_prepared_structure_step(&ChildContentView::EMPTY, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: physical }).unwrap());
            assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: physical });
            assert_eq!((events.requested_bytes, events.released_bytes), (0, physical));
        }
        assert_eq!(view.prepared_structure_close_byte_demand(), 0);
        assert_eq!(view.close_prepared_structure_step(&ChildContentView::EMPTY, store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 0 }).unwrap(), PluginCloseStep::Complete);
        println!("[DEBUG] prepared child scaffold exact Arc page={page_bytes} root={root_bytes} allocator release matches each whole grant");
    }
    #[test]
    fn owned_document_member_ingress_identity_reports_actual_whole_physical_release() {
        let mut mismatches = 0;
        const NEUTRAL: &str = include_str!("../../../🏪️store/🧩️composition/🚪️open/🏭️operation/📏️birth/🧫️fixtures/🔣️.json");
        let fixture: serde_json::Value = serde_json::from_str(NEUTRAL).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let slot = row["dialect"]["subset"].as_str().unwrap();
            let child = row["artifactId"].as_str().unwrap();
            let mut ingress = member_ingress(0, 18, 19, slot, child);
            let mut request = ingress.take_request().unwrap();
            for _ in 0..4096 {
                let bytes = request.next_close_byte_demand().max(1);
                assert!(bytes <= 262_144);
                if request.close_step(1, bytes).unwrap() == store::SnapshotRetirementStep::Complete { break; }
            }
            assert!(request.terminal_is_empty());
            drop(request);
            let mut paid = 0;
            let mut actual = 0;
            for _ in 0..4096 {
                let bytes = ingress.next_close_byte_demand().max(1);
                assert!(bytes <= 262_144);
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ingress.close_step(1, bytes).unwrap());
                let reported = match step {
                    PluginCloseStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= bytes); released_bytes },
                    PluginCloseStep::Complete => 0,
                    other => panic!("unexpected ingress physical retirement: {other:?}"),
                };
                if (events.requested_bytes, events.released_bytes) != (0, reported) {
                    mismatches += 1;
                    if mismatches <= 8 { println!("[DEBUG] ingress identity physical mismatch born={} freed={} reported={reported}", events.requested_bytes, events.released_bytes); }
                }
                paid += reported;
                actual += events.released_bytes;
                if ingress.terminal_is_empty() { break; }
            }
            assert!(ingress.terminal_is_empty());
            drop(ingress);
            println!("[DEBUG] ingress original identity slot={slot:?} child={child:?} paid={paid} actual={actual}");
        }
        assert_eq!(mismatches, 0, "ingress identity must report physical same-turn frees without close births");
    }

    #[test]
    fn owned_document_member_ingress_identity_preserves_denied_original_capacity_even_when_empty() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🏪️store/🧩️composition/🚪️open/📏️retirement/🧫️fixtures/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let mut ingress = member_ingress(0, 18, 19, "slot", "child");
            let mut request = ingress.take_request().unwrap();
            for _ in 0..4096 { let bytes = request.next_close_byte_demand().max(1); if request.close_step(1, bytes).unwrap() == store::SnapshotRetirementStep::Complete { break; } }
            assert!(request.terminal_is_empty()); drop(request);
            let bytes = row["capacityBytes"].as_u64().unwrap() as usize;
            let mut original = String::with_capacity(bytes); original.push_str(row["text"].as_str().unwrap());
            let old = std::mem::replace(ingress.identity_string_mut().unwrap(), original); drop(old);
            let pointer = ingress.identity_string_mut().unwrap().as_ptr();
            for (items, release) in [(0, bytes), (1, 0), (1, bytes - 1)] {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ingress.close_step(items, release).unwrap());
                assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(ingress.identity_string_mut().unwrap().as_ptr(), pointer);
                assert_eq!(ingress.next_close_byte_demand(), bytes);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ingress.close_step(1, bytes).unwrap());
            assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, bytes));
            close_ingress(ingress);
            println!("[DEBUG] ingress original capacity={} text={} zero-item/one-below preserve pointer; exact physical release once", bytes, row["text"]);
        }
    }

    #[test]
    fn owned_document_ingress_registry_backing_remains_owned_until_exact_release() {
        let mut registry = OwnedDocumentMemberIngressRegistry::try_new(17).unwrap();
        let bytes = registry.next_close_byte_demand();
        assert!(bytes > 0 && bytes <= 262_144);
        let (denied, first) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(0, bytes).unwrap());
        let (under, second) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(1, bytes - 1).unwrap());
        let (funded, third) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(1, bytes).unwrap());
        for _ in 0..8 { let demand = registry.next_close_byte_demand(); registry.close_step(1, demand).unwrap(); if registry.terminal_is_empty() { break; } }
        let empty = registry.terminal_is_empty();
        drop(registry);
        assert_eq!(denied, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(under, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!((first.requested_bytes, first.released_bytes, second.requested_bytes, second.released_bytes), (0, 0, 0, 0));
        assert_eq!(funded, PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        assert_eq!((third.requested_bytes, third.released_bytes), (0, bytes));
        assert!(empty);
        println!("[DEBUG] original empty ingress17 backing={} stays original under denied grants and reports its exact funded same-turn release", bytes);
    }

    #[test]
    fn owned_document_ingress_paging_matches_neutral_frames_and_exact_physical_retirement() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/📦️ingress-paging/🔣️.json")).unwrap();
        let page_slots = fixture["pageSlots"].as_u64().unwrap() as usize;
        let admission = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
        assert_eq!(OWNED_DOCUMENT_INGRESS_PAGE_SLOTS, page_slots);
        assert!(std::mem::size_of::<OwnedDocumentMemberIngressRegistry>() < 1024);
        for row in fixture["cases"].as_array().unwrap() {
            let entries = row["entries"].as_u64().unwrap() as usize;
            let (mut registry, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| OwnedDocumentMemberIngressRegistry::try_new(entries).unwrap());
            let frames: Vec<usize> = registry.pages.iter().map(|page| page.as_ref().unwrap().len()).collect();
            let expected: Vec<usize> = row["frames"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect();
            assert_eq!(frames, expected);
            assert_eq!(registry.pages.capacity(), row["pointerSlots"].as_u64().unwrap() as usize);
            let frame_bytes: Vec<usize> = registry.pages.iter().map(|page| std::mem::size_of_val(page.as_ref().unwrap().as_ref())).collect();
            assert!(frame_bytes.iter().all(|bytes| *bytes <= admission));
            let pointer_bytes = registry.pages.capacity() * std::mem::size_of::<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>();
            let expected_birth = frame_bytes.iter().sum::<usize>() + pointer_bytes;
            assert_eq!((birth.requested_bytes, birth.released_bytes), (expected_birth, 0));
            let mut released = 0;
            for bytes in frame_bytes.into_iter().chain((pointer_bytes != 0).then_some(pointer_bytes)) {
                let pointer = registry.pages.as_ptr();
                let page_pointer = registry.pages.iter().find_map(|page| page.as_ref().map(|page| page.as_ptr()));
                assert_eq!(registry.next_close_byte_demand(), bytes);
                for (items, grant_bytes) in [(0, bytes), (1, bytes - 1)] {
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(items, grant_bytes).unwrap());
                    assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    assert_eq!(registry.pages.as_ptr(), pointer);
                    assert_eq!(registry.pages.iter().find_map(|page| page.as_ref().map(|page| page.as_ptr())), page_pointer);
                    assert_eq!(registry.next_close_byte_demand(), bytes);
                }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(1, bytes).unwrap());
                assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, bytes));
                released += bytes;
            }
            assert!(registry.terminal_is_empty());
            let (step, terminal) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(1, 0).unwrap());
            assert_eq!(step, PluginCloseStep::Complete);
            assert_eq!((terminal.requested_bytes, terminal.released_bytes), (0, 0));
            assert_eq!(released, expected_birth);
            let (_, dropped) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(registry));
            assert_eq!((dropped.requested_bytes, dropped.released_bytes), (0, 0));
            println!("[DEBUG] ingress entries={entries} original8-slot pages and pointer backing birth={expected_birth} release={released}; all denied turns retain original pointers and exact funded frees");
        }
    }

}
