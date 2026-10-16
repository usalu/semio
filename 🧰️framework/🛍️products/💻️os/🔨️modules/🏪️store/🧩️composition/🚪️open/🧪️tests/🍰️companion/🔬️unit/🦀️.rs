//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk M of the cut by subject).

use super::*;


//#region 📬️OneItemPublicationLaws
//#region 🧩️RetainedMemberPublicationLaws
semio_framework_os_kernel::space_members! {
    native_producer:&semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    history_limits:protocol::codec::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum RetainedTestMembers, RetainedTestMembersOpen {
        First("s.test.member", "v1", "first", "demo/v1") => (DemoSnapshot, DemoMutation, semio_framework_os_store_testing_doubles::DemoMemberStoreProducer<DemoMutation>, semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()),
        Second("s.test.member", "v1", "second", "demo/v1") => (DemoSnapshot, DemoMutation, semio_framework_os_store_testing_doubles::DemoMemberStoreProducer<DemoMutation>, semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()),
    }
}

fn member_publication_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/📢️member-publication.json")).expect("language-neutral member publication fixture")
}


fn close_member_dialect_envelope(envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation>) { retire_demo_envelope(envelope); }

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_matches_neutral_admission_corpus() {
    let fixture = member_dialect_fixture();
    let seed = DemoSnapshot { n: Some(7) };
    for row in fixture["cases"].as_array().unwrap() {
        let dialect: semio_framework_artifact_reference::ArtifactDialect = serde_json::from_value(row["requested"].clone()).unwrap();
        let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-1".into(), dialect };
        MEMBER_SNAPSHOT_DECODE_COUNT.set(0);
        let result = if row["operation"] == "create" {
            RetainedTestMembers::create(&expected.artifact_id, &expected.dialect, &seed.encode_pack(), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await
        } else {
            let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>(row["persisted"]["schema"].as_str().unwrap(), "child-1", seed.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
            envelope.dialect = serde_json::from_value(row["persisted"]["dialect"].clone()).unwrap();
            let files = print_document_pack(&envelope).await.unwrap();
            let packed = encode_document_pack_bytes(&files.pack, &files.spr).await;
            close_member_dialect_envelope(envelope);
            RetainedTestMembers::open(&expected, None, &packed, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await
        };
        assert_eq!(MEMBER_SNAPSHOT_DECODE_COUNT.get(), usize::from(row["accepted"].as_bool().unwrap()), "{}: rejected identity must not hydrate a typed snapshot", row["id"]);
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}: {:?}", row["id"], result.as_ref().err());
        if let Ok(mut member) = result {
            let variant = match &member {
                RetainedTestMembers::First(_) => "First",
                RetainedTestMembers::Second(_) => "Second",
            };
            assert_eq!(variant, row["variant"].as_str().unwrap());
            assert_eq!(member.document_id().await, "child-1");
            let restored = DemoSnapshot::decode_pack(&member.document_pack_bytes().await.unwrap()).unwrap();
            assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::to_value(&seed).unwrap());
            assert_eq!(member.artifact_ref().as_ref(), Some(&expected));
            assert_eq!(member.owner_ref(), None);
            assert!(member.child_restore_projection().expect("member exposes its exact typed snapshot projection").is_empty());
            close_member_dialect_fixture(&mut member);
        }
    }
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-1".into(), dialect: serde_json::from_value(fixture["bindings"][0]["dialect"].clone()).unwrap() };
    for bytes in [&[][..], &[1][..], &[0xff, 0xff][..]] {
        MEMBER_SNAPSHOT_DECODE_COUNT.set(0);
        assert!(RetainedTestMembers::open(&expected, None, bytes, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.is_err());
        assert_eq!(MEMBER_SNAPSHOT_DECODE_COUNT.get(), 0);
    }
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_rejects_identity_and_owner_substitution() {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let fixture = member_dialect_fixture();
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-1".into(), dialect: serde_json::from_value(fixture["bindings"][0]["dialect"].clone()).unwrap() };
    let owner = OwnerRef { parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect::parse_coordinate("s.test.parent@v1/*").unwrap() }, slot: "content".into(), child_id: "child-1".into() };
    for row in fixture["identityCases"].as_array().unwrap() {
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", row["persistedId"].as_str().unwrap(), DemoSnapshot { n: Some(7) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        envelope.dialect = Some(expected.dialect.clone());
        if !row["persistedOwner"].is_null() {
            let persisted = &row["persistedOwner"];
            envelope.owner = Some(OwnerRef {
                parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: persisted["parentId"].as_str().unwrap().into(), dialect: semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(persisted["parentDialect"].as_str().unwrap()).unwrap() },
                slot: persisted["slot"].as_str().unwrap().into(),
                child_id: persisted["childId"].as_str().unwrap().into(),
            });
        }
        let files = print_document_pack(&envelope).await.unwrap();
        let packed = encode_document_pack_bytes(&files.pack, &files.spr).await;
        close_member_dialect_envelope(envelope);
        let expected_owner = row["expectedOwned"].as_bool().unwrap().then_some(&owner);
        MEMBER_SNAPSHOT_DECODE_COUNT.set(0);
        let result = RetainedTestMembers::open(&expected, expected_owner, &packed, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await;
        assert_eq!(MEMBER_SNAPSHOT_DECODE_COUNT.get(), usize::from(row["accepted"].as_bool().unwrap()), "{}: rejected owner must not hydrate a typed snapshot", row["id"]);
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}: {:?}", row["id"], result.as_ref().err());
        if let Ok(mut member) = result {
            assert_eq!(member.artifact_ref().as_ref(), Some(&expected));
            assert_eq!(member.owner_ref().as_ref(), expected_owner);
            close_member_dialect_fixture(&mut member);
        }
    }
}


#[semio_framework_async_macros::async_test]
async fn retained_member_publication_preserves_order_group_identity_and_exact_maximum_grant_progress() {
    let fixture = member_publication_fixture();
    let grant = ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize, ..one_item_test_policy() };
    let mut first = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "first", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    let mut second = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "second", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    first.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    second.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let mut members = [RetainedTestMembers::First(Box::new(first.into_inner())), RetainedTestMembers::Second(Box::new(second.into_inner()))];
    let mut observed = Vec::new();
    for (sequence, row) in fixture["orderedMembers"].as_array().unwrap().iter().enumerate() {
        let index = usize::from(row["id"] == "second");
        let member = &mut members[index];
        assert!(member.one_item_wire_publication_supported());
        let (generation, revision) = member.one_item_publication_identity();
        let mut bytes = row["wire"].as_str().unwrap().as_bytes().to_vec();
        bytes.resize(bytes.len() + row["paddingBytes"].as_u64().unwrap() as usize, b' ');
        let oracle: i32 = serde_json::from_slice(&bytes).expect("independent third-party JSON decoder");
        let byte_count = bytes.len();
        let wire_ptr = bytes.as_ptr();
        let request = MemberStoreOneItemWireRequest {
            operation: semio_framework_job::OperationId(700 + sequence as u64),
            expected_generation: generation,
            expected_revision: revision,
            actor: "member-test".into(),
            group_id: Some(fixture["groupId"].as_str().unwrap().into()),
            wire: MemberStoreOneItemWire { schema: fixture["wireSchema"].as_str().unwrap().into(), bytes },
        };
        let mut publication = member.begin_one_item_wire_publication(request).unwrap_or_else(|_| panic!("exact member factory admits owned wire"));
        assert_eq!(publication.progress().completed_bytes, 0);
        assert!(matches!(member.advance_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 4096, ..one_item_test_policy() }), Ok(ArtifactStoreOneItemAdvance::Blocked)));
        let mut published = false;
        let wire_passes = fixture["wirePasses"].as_u64().unwrap() as usize;
        assert_eq!(wire_passes, 2, "one borrowed decode pass and one retained release pass");
        let maximum_turns = byte_count.checked_mul(wire_passes).and_then(|bytes| bytes.checked_add(fixture["wireSchema"].as_str().unwrap().len())).and_then(|bytes| bytes.checked_add(fixture["fixedPublicationTurns"].as_u64().unwrap() as usize)).unwrap();
        for _ in 0..maximum_turns.max(262_144) {
            let before = publication.progress();
            match member.advance_one_item_publication(&mut *publication, grant).expect("bounded retained member unit") {
                ArtifactStoreOneItemAdvance::Published(receipt) => {
                    assert_eq!(receipt.generation_after, generation + 1);
                    published = true;
                    break;
                }
                ArtifactStoreOneItemAdvance::Progress(after) | ArtifactStoreOneItemAdvance::PreparationProgress(after, _) => assert!(after.completed_bytes.saturating_sub(before.completed_bytes) <= grant.maximum_copy_bytes as u64),
                step => panic!("member publication did not progress: {step:?}"),
            }
        }
        assert!(published, "member publication stalled at {:?}, progress {:?}", publication.phase(), publication.progress());
        assert!(publication.retry());
        assert!(matches!(member.advance_one_item_publication(&mut *publication, grant), Ok(ArtifactStoreOneItemAdvance::AwaitingAck(_))));
        assert_eq!(member.one_item_publication_identity().0, generation + 1);
        assert!(publication.acknowledge());
        assert!(!publication.acknowledge());
        close_erased_member_publication(&mut *publication, grant);
        let concrete = member.as_any_mut().await.downcast_mut::<semio_framework_os_store_engine::lifted_store::ArtifactStore<DemoSnapshot, DemoMutation>>().unwrap();
        assert_eq!(concrete.snapshot().unwrap().n, Some(oracle));
        assert_eq!(oracle as i64, row["expected"].as_i64().unwrap());
        assert_eq!(SpaceMember::tail_group_id(concrete).await.as_deref(), fixture["groupId"].as_str());
        observed.push(oracle);
        eprintln!("retained member sequence={sequence} wire={wire_ptr:p} bytes={byte_count} value={oracle}");
    }
    assert_eq!(observed, vec![17, -23, 42]);
    for member in &mut members {
        for _ in 0..4096 {
            if matches!(member.close_owned_step(grant.retained_grant()).unwrap(), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(member.close_owned_terminal_is_empty());
    }
}


#[test]
fn member_open_constructor_requires_borrowed_birth_before_moving_original_request() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🏭️operation/📏️birth/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut denied_births = 0;
    let mut paid_births = 0;
    for row in fixture["cases"].as_array().unwrap() {
        let reference = semio_framework_artifact_reference::ArtifactRef { artifact_id: row["artifactId"].as_str().unwrap().into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: row["dialect"]["artifact_kind"].as_str().unwrap().into(), standard: row["dialect"]["standard"].as_str().unwrap().into(), subset: row["dialect"]["subset"].as_str().unwrap().into() } };
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).unwrap();
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(&[1, 97, 83]).unwrap()).unwrap();
        pages.seal().unwrap();
        let request = MemberOpenRequest::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), 1000, reference, None, pages, ActorId(row["actor"].as_str().unwrap().into())).admit(1).unwrap_or_else(|_| panic!("exact neutral constructor request"));
        let mut caller=OriginalMemberCaller::new();
        let mut admission=MemberOpenSourceAdmission::new(request);
        let mut opened=None;
        let identity = |value: &MemberOpenRequest| { let expected = value.expected(); (expected.artifact_id.as_ptr(), expected.dialect.artifact_kind.as_ptr(), expected.dialect.standard.as_ptr(), expected.dialect.subset.as_ptr(), value.actor().0.as_ptr()) };
        let pointers = identity(admission.request().unwrap());
        let (birth, query_events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| RetainedTestMembers::open_birth_demand(admission.request().unwrap()).unwrap());
        assert_eq!((query_events.requested_bytes, query_events.released_bytes), (0, 0));
        assert_eq!(identity(admission.request().unwrap()), pointers);
        let required = birth.capacity_bytes;
        assert!(birth.depth > 0);
        assert!(required >= 4096 && required <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
        for (items, capacity, depth) in [(0, required, birth.depth), (1, 0, birth.depth), (1, required - 1, birth.depth), (1, required, birth.depth - 1)] {
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: 64, maximum_capacity_bytes: capacity, maximum_release_bytes: 0, maximum_depth: depth };
            let((ready,receipt),events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.run(grant,|parent,cx|RetainedTestMembers::begin_open(&mut admission,&mut opened,cx,parent).unwrap()));
            assert!(!ready&&opened.is_none());assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(events.requested_bytes,events.released_bytes));
            denied_births += events.requested_bytes;
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(identity(admission.request().unwrap()), pointers);
            assert_eq!(admission.request().unwrap().retained_input_bytes(), 3);
        }
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: required, maximum_release_bytes: 0, maximum_depth: birth.depth };
        let((ready,receipt),events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.run(grant,|parent,cx|RetainedTestMembers::begin_open(&mut admission,&mut opened,cx,parent).unwrap()));
        assert!(ready&&admission.terminal_is_empty());assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(events.requested_bytes,events.released_bytes));
        assert_eq!((events.requested_bytes, events.released_bytes), (required, 0));
        let mut open = opened.take().unwrap();
        paid_births += events.requested_bytes;
        let(_,_,turns)=retire_original_member_open(&mut open,&mut caller);
        let(_,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||drop(open));assert_eq!((allocated,released),(0,0));assert!(turns>0);caller.close();
        println!("[DEBUG] member constructor case={} actual-paid-birth={} borrowed-query={required} zero/one-below-births=0 original-identities-retained=true dialect-clones=0 history-page4096=true original-admission262144", row["id"], events.requested_bytes);
    }
    assert_eq!(denied_births, 0);
    assert!(paid_births >= 8192);
}

include!("../../../🏭️operation/📦️catalog/🦀️.rs");
//#endregion 🧩️RetainedMemberPublicationLaws
//#endregion 📬️OneItemPublicationLaws
