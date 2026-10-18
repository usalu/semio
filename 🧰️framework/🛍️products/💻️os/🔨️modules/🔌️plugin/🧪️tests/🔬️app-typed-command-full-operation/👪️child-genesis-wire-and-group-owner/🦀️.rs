    /// 👪️ Child genesis, wire and group owner: the revision change between preparation turns, private child genesis, nested owner retirement and the one retained async group owner.

    #[test]
    fn document_revision_change_between_ephemeral_preparation_turns_closes_without_publishing_the_lane_root() {
        let canonical_revision = [3; 32];
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(93), semio_framework_job::RevisionId(u64::from_be_bytes([3; 8])), semio_framework_job::Generation(0), 17);
        let mut presence = store::PresenceStore::<PublicationPresence, PublicationPresenceMutation>::new(PublicationPresence::default());
        let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
        presence.install_local_retirement_factory(root_factory.clone()).unwrap();
        let initial_root = presence.local_read().unwrap();
        let factory = TwoTurnPublicationPresencePreparationFactory;
        let mut publication = presence.begin_publish_one(operation.operation, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&factory), Some(root_factory.clone())).expect("two-turn presence publication admits");
        let grant = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 64, maximum_release_bytes: 64, maximum_depth: 128 };
        assert!(matches!(presence.advance_publish_one(&mut publication, grant), Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(_))));
        assert!(std::ptr::eq(initial_root.get(), presence.local()));
        assert_eq!(presence.generation_now(), 0);

        let changed_document_revision = [4; 32];
        assert!(!typed_operation_document_is_fresh(&operation, canonical_revision, changed_document_revision, 0));
        publication.begin_close();
        for _ in 0..4 {
            if matches!(publication.close_step(grant).expect("stale pending publication closes"),RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(publication.terminal_is_empty());
        assert!(std::ptr::eq(initial_root.get(), presence.local()));
        assert_eq!(presence.generation_now(), 0);
        drop(initial_root);
        let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
        for _ in 0..2048 {
            if matches!(close.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64}).unwrap(),RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(close.terminal_is_empty());
    }

    #[test]
    fn private_child_genesis_wire_retains_exact_identity_and_indivisible_backing() {
        let fixture: Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json")).expect("neutral private genesis contract");
        let source=&fixture["source"];
        let reference=&source["reference"];
        let genesis=ChildEmitGenesis {
            reference: ArtifactRef { artifact_id:reference["artifactId"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind:reference["dialect"]["artifactKind"].as_str().unwrap().into(), standard:reference["dialect"]["standard"].as_str().unwrap().into(), subset:reference["dialect"]["subset"].as_str().unwrap().into() } },
            initial_pack: serde_json::to_vec(&fixture["initial"]["brep"]).unwrap(),
        };
        let mut child=ChildEmit::open(source["slot"].as_str().unwrap(),source["childId"].as_str().unwrap(),0);
        child.genesis=Some(genesis);
        assert_ne!(child.child_id,child.genesis.as_ref().unwrap().reference.artifact_id);
        let independent=serde_json::to_value(&child).expect("independent exact genesis projection");
        assert_eq!(independent["genesis"]["reference"],source["reference"]);
        assert_eq!(Value::from(semio_framework_value::ToValue::to_value(&child)),independent);
        let wire=ChildEmit::encode_groups(std::slice::from_ref(&child));
        let mut decoded=ChildEmit::decode_groups(&wire).expect("exact genesis wire roundtrip");
        assert_eq!(decoded[0],child);
        let pointer=child.genesis.as_ref().unwrap().initial_pack.as_ptr();
        let bytes=child.retirement_demands().unwrap().release_bytes;
        assert!(bytes>0);
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let yielded=RetainedCloneStep::Progress(RetainedCloneProgress::default());
        for _ in 0..4 {
            assert_eq!(child.close_one(RetainedCloneGrant::one_release_turn(bytes-1,1)).unwrap(),yielded);
            assert_eq!(child.genesis.as_ref().unwrap().initial_pack.as_ptr(),pointer);
            assert_eq!(child.genesis.as_ref().unwrap().reference.artifact_id,reference["artifactId"].as_str().unwrap());
        }
        assert_eq!(child.close_one(RetainedCloneGrant{maximum_items:0,..RetainedCloneGrant::one_release_turn(bytes,1)}).unwrap(),yielded);
        assert_eq!(child.close_one(RetainedCloneGrant::one_release_turn(bytes,1)).unwrap(),RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}));
        for child in std::iter::once(&mut child).chain(decoded.iter_mut()) {
            for _ in 0..32 {
                let grant=RetainedCloneGrant::one_release_turn(child.retirement_demands().unwrap().release_bytes,1);
                match child.close_one(grant).unwrap() {
                    RetainedCloneStep::Progress(progress)=>assert!(progress.fits(grant)),
                    RetainedCloneStep::Complete(_)=>break,
                }
            }
            assert!(child.genesis.is_none());
            assert_eq!(child.retirement_demands().unwrap().release_bytes,0);
        }
    }

    #[test]
    fn retained_child_wire_rejection_retires_nested_owners_under_the_production_grant() {
        let fixture: Value = serde_json::from_str(include_str!("../../../../🏪️store/🧪️tests/🧫️fixtures/📢️member-publication.json")).expect("retained child fixture");
        let row = &fixture["orderedMembers"][0];
        let mut wire = row["wire"].as_str().unwrap().as_bytes().to_vec();
        wire.resize(wire.len() + row["paddingBytes"].as_u64().unwrap() as usize, b' ');
        let wire_bytes = wire.len();
        let mut child = ChildEmit {
            genesis: None,
            owner: String::new(),
            slot: "slot".into(),
            child_id: "child".into(),
            ops: vec![wire],
            op_schema: SchemaId("demo.member.json-number".into()),
            labels: vec![LocalizedLabel::native("ä🧩", "ß🎯")],
        };
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let page = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        assert_eq!(child.close_one(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..page }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        assert_eq!(child.ops[0].len(), wire_bytes);
        let mut bytes = 0;
        let mut complete = false;
        for _ in 0..wire_bytes + 128 {
            match child.close_one(page).unwrap() {
                RetainedCloneStep::Progress(progress) => {
                    assert!(progress.fits(page));
                    bytes += progress.released_bytes;
                }
                RetainedCloneStep::Complete(_) => {
                    complete = true;
                    break;
                }
            }
        }
        assert!(complete);
        assert!(bytes >= wire_bytes);
        assert!(child.ops.is_empty() && child.labels.is_empty() && child.slot.is_empty() && child.child_id.is_empty() && child.op_schema.0.is_empty());
    }

    #[test]
    fn typed_child_publication_has_one_retained_async_group_owner_and_never_clone_publishes() {
        let source = instance_source();
        let child_start = source.rfind("async fn publish_mounted_typed_child_operation_unit(").expect("retained child publisher");
        let child_end = source[child_start..].find("fn publish_mounted_typed_operation_unit(").map(|offset| child_start + offset).expect("retained child publisher end");
        let child_publisher = &source[child_start..child_end];
        for retained_seam in
            ["PendingChildGroupPublicationPhase::Ready", "captured_child_content_generation", "captured.identity_digest()", "try_claim_publication", "begin_dispatch", "&pending.child_emits", "dispatch_emit_group(", "TypedOperationResultLane::Child"]
        {
            assert!(child_publisher.contains(retained_seam), "retained Child publication lost its exact owner seam: {retained_seam}");
        }
        for forbidden in ["child_emits.last().cloned()", "vec![child.clone()]", "for child in pending.child_emits.clone()"] {
            assert!(!child_publisher.contains(forbidden), "retained Child publication restored clone-then-publish: {forbidden}");
        }
        let publisher_start = child_end;
        let publisher_end = source[publisher_start..].find("fn require_tool_operation_authority(").map(|offset| publisher_start + offset).expect("typed publisher end");
        let publisher = &source[publisher_start..publisher_end];
        let parent_guard = publisher.find("if emit.child_emits.is_empty()").expect("parent publication group guard");
        let retained_install = publisher.find("PendingChildGroupPublication::new(").expect("retained child owner install");
        assert!(parent_guard < retained_install, "parent mutations must remain group-owned whenever Child output is present");
        assert!(source.contains("ArtifactToolPublicationLane::Child => None"));
    }
