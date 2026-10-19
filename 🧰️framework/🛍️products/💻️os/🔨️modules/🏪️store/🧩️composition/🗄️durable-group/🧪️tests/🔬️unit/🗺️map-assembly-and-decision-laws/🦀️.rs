/// 🗺️ Map assembly and decision laws: the three store map assembly under exact, late member, cancelled and uncertain grants, the owned group decision, the journal record projection and the prepared outcome derivation.

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_uses_exact_gis_factories_and_binds_one_decision() {
    let (parent, drawing, value) = owned_three_stores().await;
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let (mut assembly, factories) = map_assembly(parent, drawing, value, None, FakeJournalResolution::Commit, state.clone());
    drive_assembly_to_mounted(&mut assembly);
    assert!(factories.iter().all(|factory| Arc::strong_count(factory) == 1), "each explicit role factory leaves the assembly immediately after admission");
    let mut host = assembly.take_mounted_host().expect("valid assembly transfers only the existing fixed host");
    assert!(assembly.terminal_is_empty());
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    for _ in 0..4096 {
        match host.advance(grant).expect("mounted assembly commit turn") {
            DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_) => assert!(host.acknowledge()),
            DurableOwnedThreeStoreCommitAdvanceV1::Complete => break,
            DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::DiagnosticProgress(_, _) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked => {}
        }
    }
    let decision_pack = state.lock().unwrap().decision_pack.clone();
    let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&decision_pack).expect("journal receives one Store-sealed canonical decision");
    for member in [&decision.parent, &decision.drawing, &decision.value] {
        let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack).expect("member retains one bound recovery owner");
        assert_eq!(outcome.group_id, decision.decision_sha256);
    }
    assert_eq!(state.lock().unwrap().begins, 1);
    let owners = host.take_terminal_owners().expect("acknowledged host returns all Stores and sink");
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_late_member_rejection_closes_prior_publications_before_owner_handoff() {
    for role in [DRAWING_ROLE, VALUE_ROLE] {
        let (parent, drawing, value) = owned_three_stores().await;
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let (mut assembly, _) = map_assembly(parent, drawing, value, Some(role), FakeJournalResolution::Commit, state.clone());
        let owners = drain_assembly_rejection(&mut assembly);
        assert!(assembly.terminal_is_empty());
        assert_eq!([owners.parent.generation(), owners.drawing.generation(), owners.value.generation()], [0, 0, 0]);
        assert_eq!(state.lock().unwrap().begins, 0);
        assert!(matches!(owners.failure, DurableOwnedThreeStoreMapAssemblyFailureV1::Admission { role: rejected, .. } if rejected == role));
        assert!(owners.parent_mutation.is_none());
        assert_eq!(owners.drawing_mutation.is_some(), role == DRAWING_ROLE);
        assert!(owners.value_mutation.is_some());
        close_assembly_owners(owners);
    }
}

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_cancellation_before_journal_restores_all_three_frontiers() {
        let mut receiving_fixture=super::super::tests::FixtureStoreReceiving::new(copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES).retained_grant());
let (parent, drawing, value) = owned_three_stores().await;
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let (mut assembly, _) = map_assembly(parent, drawing, value, None, FakeJournalResolution::Commit, state.clone());
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    while assembly.phase() != DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing {
        receiving_fixture.receive(|receiving|assembly.advance(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES,grant,receiving)).expect("pre-cancellation assembly turn");
    }
    assert!(assembly.cancel());
    let owners = drain_assembly_rejection(&mut assembly);
    assert!(matches!(owners.failure, DurableOwnedThreeStoreMapAssemblyFailureV1::Cancelled));
    assert_eq!([owners.parent.generation(), owners.drawing.generation(), owners.value.generation()], [0, 0, 0]);
    assert_eq!(state.lock().unwrap().begins, 0);
    close_assembly_owners(owners);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_uncertain_journal_retains_same_host_until_committed_or_proven_absent() {
    let (parent, drawing, value) = owned_three_stores().await;
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let (mut assembly, _) = map_assembly(parent, drawing, value, None, FakeJournalResolution::ErrorThenCommit, state.clone());
    drive_assembly_to_mounted(&mut assembly);
    let mut host = assembly.take_mounted_host().expect("prepared assembly transfers one fixed host");
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    let mut uncertain = false;
    for _ in 0..4096 {
        match host.advance(grant) {
            Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                uncertain = true;
                let snapshot = host.capture_snapshot().expect("uncertain journal retains one all-old snapshot");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
                assert_eq!(state.lock().unwrap().begins, 1);
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_)) => assert!(host.acknowledge()),
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::DiagnosticProgress(_, _) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
            Err(error) => panic!("unexpected retained assembly journal error: {error}"),
        }
    }
    assert!(uncertain);
    assert_eq!(state.lock().unwrap().begins, 1);
    let owners = host.take_terminal_owners().expect("same host terminates after one commit and ACK");
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[test]
fn durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds() {
    let fixture = fixture();
    let decision = decision();
    assert_eq!(decision.anchor_sha256, fixture["expected"]["anchorSha256"]);
    assert_eq!(decision.decision_sha256, fixture["expected"]["decisionSha256"]);
    assert_eq!(decision.canonical_unsigned_json(), fixture["expected"]["unsignedJson"]);
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 8);
    let json = decision.canonical_json();
    assert_eq!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&json).unwrap(), decision);
    let pack = decision.encode_pack();
    assert!(pack.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&pack).unwrap(), decision);
    let handles = DurableOwnedGroupMapHandlesV1 { drawing: decision.drawing.reference.clone(), value: decision.value.reference.clone() };
    let frontiers = DurableOwnedGroupMapFrontiersV1 {
        parent_generation: decision.parent.expected_generation,
        parent_revision: decision.parent.expected_revision,
        drawing_generation: decision.drawing.expected_generation,
        drawing_revision: decision.drawing.expected_revision,
        value_generation: decision.value.expected_generation,
        value_revision: decision.value.expected_revision,
    };
    decision.admit_map(&handles, &frontiers).unwrap();
    let mut bound_derivations = decision.clone();
    bound_derivations.parent.recovery_pack.push(0);
    bound_derivations.parent.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&bound_derivations.parent.recovery_pack);
    bound_derivations.parent.post_generation += 1;
    bound_derivations.parent.post_revision[0] ^= 1;
    assert_eq!(bound_derivations.canonical_unsigned_json(), decision.canonical_unsigned_json());
    assert_eq!(semio_framework_hash::sha256_hex(bound_derivations.canonical_unsigned_json().as_bytes()), decision.decision_sha256);
    assert_eq!(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES * 3 + DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, 490_096);
    const { assert!(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES * 3 + DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES) };
    let varint = |value: usize| {
        if value < 1 << 7 {
            1
        } else if value < 1 << 14 {
            2
        } else if value < 1 << 21 {
            3
        } else if value < 1 << 28 {
            4
        } else {
            5
        }
    };
    let frame = |payload: usize| varint(payload + 2) + payload + 10;
    let framed_bytes = 129 + frame(8) + frame(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES) + frame(12) + 75;
    assert_eq!(framed_bytes, 491_779);
    assert!(framed_bytes <= 507_904);
}

#[test]
fn durable_group_journal_record_projects_edits_only_after_all_three_bound_outcomes_verify() {
    let decision = decision();
    let canonical_pack = decision.encode_pack();
    let record = DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack.clone()).expect("Store derives identity from the complete canonical decision");
    assert!(matches!(DurableOwnedGroupJournalRecordV1::admit(canonical_pack, &"0".repeat(64)), Err(DurableOwnedGroupDecisionError::InvalidHash)));
    let verified = record.verify_fixed_three_edits::<DslValue, String, DslValue, String, DslValue, String>().expect("all three typed outcomes verify before projection");
    assert_eq!(verified.decision_sha256(), decision.decision_sha256);
    assert_eq!(verified.anchor_sha256(), decision.anchor_sha256);
    assert_eq!(verified.document(), &decision.anchor.parent);
    assert_eq!(record.parent_edit_id(), verified.parent().id);
    assert_eq!(record.parent_post_revision(), decision.parent.post_revision);
    assert_eq!(verified.parent().forwards, ["parent:forward"]);
    assert_eq!(verified.drawing().forwards, ["drawing:forward"]);
    assert_eq!(verified.value().forwards, ["value:forward"]);
    for edit in [verified.parent(), verified.drawing(), verified.value()] {
        assert_eq!(edit.mutation_meta.len(), 1);
        assert_eq!(edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    }
}

#[test]
fn durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes() {
    let fixture = fixture();
    let parent = &fixture["members"]["parent"];
    let (prepared, outcome) = prepared_outcome(PARENT_ROLE, parent["recoverySchema"].as_str().unwrap(), parent["expectedGeneration"].as_u64().unwrap(), revision(parent["expectedRevisionHex"].as_str().unwrap()), 1);
    let verified = outcome.verify_inverse::<DslValue, String>().expect("Store-owned bytes invert to the same typed owners");
    assert_eq!(ValueToValue::to_value(verified.edit.as_ref()), ValueToValue::to_value(prepared.edit()),);
    assert_eq!(verified.post_snapshot.encode_pack(), prepared.post_snapshot().encode_pack(),);
    assert_eq!(outcome.sha256, semio_framework_hash::sha256_hex(&outcome.pack));
    assert_eq!(hex_string(&outcome.pack), parent["unboundOutcomePackHex"]);
    assert_eq!(outcome.sha256, parent["unboundOutcomeSha256"]);

    let mut modified_bytes = outcome.clone();
    *modified_bytes.pack.last_mut().expect("nonempty outcome pack") ^= 1;
    assert!(matches!(modified_bytes.verify_inverse::<DslValue, String>(), Err(DurableOwnedGroupDecisionError::InvalidHash)));

    let mut reordered = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack).expect("canonical outcome");
    let mut reordered_edit: DslValue = semio_framework_pack_json::from_json_str(std::str::from_utf8(&reordered.edit_without_group_canonical_json).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("edit value");
    let DslValue::Object(fields) = &mut reordered_edit else { panic!("edit projection is an object") };
    fields.rotate_left(1);
    reordered.edit_without_group_canonical_json = semio_framework_pack_json::to_json_string(&reordered_edit).into_bytes();
    let reordered_pack = reordered.encode_pack();
    let reordered = DurableStorePreparedOutcomeV1 { recovery_schema: outcome.recovery_schema.clone(), sha256: semio_framework_hash::sha256_hex(&reordered_pack), pack: reordered_pack };
    assert!(matches!(reordered.verify_inverse::<DslValue, String>(), Err(DurableOwnedGroupDecisionError::NonCanonical)));

    let mut retagged = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack).expect("canonical outcome");
    let mut retagged_edit: DslValue = semio_framework_pack_json::from_json_str(std::str::from_utf8(&retagged.edit_without_group_canonical_json).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("edit value");
    let DslValue::Object(fields) = &mut retagged_edit else { panic!("edit projection is an object") };
    let sequence = fields.iter_mut().find(|(name, _)| name == "sequenceNumber").expect("sequence field");
    sequence.1 = DslValue::float(sequence.1.as_i64().expect("signed sequence") as f64);
    retagged.edit_without_group_canonical_json = semio_framework_pack_json::to_json_string(&retagged_edit).into_bytes();
    let retagged_pack = retagged.encode_pack();
    let retagged = DurableStorePreparedOutcomeV1 { recovery_schema: outcome.recovery_schema, sha256: semio_framework_hash::sha256_hex(&retagged_pack), pack: retagged_pack };
    assert!(retagged.verify_inverse::<DslValue, String>().is_err());

    let all = outcomes();
    for (role, outcome) in [(PARENT_ROLE, &all.parent), (DRAWING_ROLE, &all.drawing), (VALUE_ROLE, &all.value)] {
        assert_eq!(hex_string(&outcome.pack), fixture["members"][role]["unboundOutcomePackHex"]);
        assert_eq!(outcome.sha256, fixture["members"][role]["unboundOutcomeSha256"]);
    }
}

#[test]
fn durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity() {
    let decision = decision();
    let mut forged = decision.clone();
    forged.drawing.owner.as_mut().unwrap().slot = "image".into();
    assert_eq!(forged.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
    let mut forged_child = decision.clone();
    forged_child.drawing.reference.artifact_id = "image".into();
    assert_eq!(forged_child.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
    let mut forged_parent = decision.clone();
    forged_parent.value.owner.as_mut().unwrap().parent.artifact_id = "map-b".into();
    assert_eq!(forged_parent.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
    let mut tampered = decision.clone();
    tampered.value.recovery_pack[0] ^= 1;
    assert_eq!(tampered.validate(), Err(DurableOwnedGroupDecisionError::InvalidHash));
    let mut oversized = decision.clone();
    oversized.parent.recovery_pack.resize(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES + 1, 0);
    oversized.parent.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&oversized.parent.recovery_pack);
    assert_eq!(oversized.validate(), Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge));
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&vec![0; DURABLE_OWNED_GROUP_EVENT_MAX_BYTES + 1]), Err(DurableOwnedGroupDecisionError::EventTooLarge));
    let canonical = decision.canonical_json();
    let unknown = format!("{},\"image\":{{}}}}", canonical.strip_suffix('}').unwrap());
    assert!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&unknown).is_err());
    #[derive(serde::Deserialize)]
    struct SchemaMember {
        #[serde(rename = "schema")]
        _schema: String,
    }
    for case in fixture()["carrier"]["jsonMemberCases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let reference = serde_json::from_str::<SchemaMember>(source).err().unwrap();
        let category = match reference.classify() {
            serde_json::error::Category::Data => "data",
            serde_json::error::Category::Eof => "eof",
            other => panic!("unexpected JSON reference category {other:?}"),
        };
        assert_eq!(category, case["serdeCategory"].as_str().unwrap());
        let actual = DurableOwnedThreeMemberDecisionV1::parse_canonical_json(source).unwrap_err();
        match case["expected"].as_str().unwrap() {
            "non-canonical" => assert_eq!(actual, DurableOwnedGroupDecisionError::NonCanonical),
            "codec" => assert!(matches!(actual, DurableOwnedGroupDecisionError::Codec(_))),
            _ => panic!("unknown JSON refusal"),
        }
    }
    let duplicate = canonical.replacen("{\"schema\":", "{\"schema\":\"semio.store.durable-owned-three-member-decision.v1\",\"schema\":", 1);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&duplicate), Err(DurableOwnedGroupDecisionError::NonCanonical));
    let mut stale = DurableOwnedGroupMapFrontiersV1 {
        parent_generation: decision.parent.expected_generation,
        parent_revision: decision.parent.expected_revision,
        drawing_generation: decision.drawing.expected_generation,
        drawing_revision: decision.drawing.expected_revision,
        value_generation: decision.value.expected_generation,
        value_revision: decision.value.expected_revision,
    };
    stale.value_generation += 1;
    let handles = DurableOwnedGroupMapHandlesV1 { drawing: decision.drawing.reference.clone(), value: decision.value.reference.clone() };
    assert_eq!(decision.admit_map(&handles, &stale), Err(DurableOwnedGroupDecisionError::InvalidFrontier));
    let mut noncanonical = decision.encode_pack();
    noncanonical.push(0);
    assert!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&noncanonical).is_err());
}

#[test]
fn durable_semantic_sources_preserve_original_decision_order_without_value_projection() {
    use semio_framework_value::NativeEncodeControl;
    let decision = decision();
    for (source, expected) in [(planning::DurableBorrowedValue::Decision(&decision), decision.canonical_json()), (planning::DurableBorrowedValue::Unsigned(&decision), decision.canonical_unsigned_json())] {
        let mut output = planning::DurableCanonicalOutputV1::default();
        let mut accepted = |_| true;
        let mut native = NativeEncodeControl::new_retained(&mut accepted);
        for _ in 0..100000 {
            if output.ready() { break; }
            let demand = output.next_demand();
            let funded = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (receipt, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| output.advance(&source, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, funded, &mut native).unwrap());
            assert!(receipt.fits(funded));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
        }
        assert!(output.ready());
        let (bytes, _) = output.take(copy_grant(4096).retained_grant()).unwrap();
        assert_eq!(bytes, expected.as_bytes());
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(), serde_json::from_str::<serde_json::Value>(&expected).unwrap());
        assert!(output.terminal_is_empty());
    }
    let (view, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| DslField::projection_view(&decision.anchor, &[1, 1, 0]).unwrap());
    assert!(matches!(view, semio_framework_dsl_record::native_encoding::FieldProjectionView::IntrinsicText("s.gis.gismap")));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    println!("[DEBUG] durable full/unsigned semantic source canonical bytes equal original neutral corpus and Serde; nested Pack identity projection returns original borrowed field with System0");
}

#[test]
#[cfg(feature = "deflate")]
fn durable_pack_planning_borrows_original_and_reports_codec_heap_and_cancellation() {
    use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../📦️prepared/🔗️binding/🧫️fixtures/🔣️.json")).unwrap();let policy=&law["packPlanning"];
    let source=decision();let address=&source as *const _ as usize;let expected=source.encode_pack();
    let prefix=(0..crate::os_store::semio_format::declared_envelope_prefix_len(<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(),crate::os_store::semio_format::Component::Pack,1).unwrap()).map(|offset|crate::os_store::semio_format::declared_binary_prefix_byte(<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(),crate::os_store::semio_format::Component::Pack,1,offset).unwrap().unwrap()).collect::<Vec<_>>();assert_eq!(hex_string(&prefix),policy["prefixHex"].as_str().unwrap());
    for cancel in [false,true] {
        let mut output=planning::DurablePackOutputV1::default();let mut accepted=|_|true;let mut native=NativeEncodeControl::new_retained(&mut accepted);let mut turns=0;
        while !output.ready() {
            let demand=output.next_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};
            for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant},RetainedCloneGrant{maximum_release_bytes:0,..grant}] {
                if denied.maximum_items!=0&&denied.maximum_copy_bytes>=demand.copy_bytes&&denied.maximum_capacity_bytes>=demand.capacity_bytes&&denied.maximum_depth>=demand.depth&&denied.maximum_release_bytes>=demand.release_bytes{continue;}
                let (receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||output.advance(&source,<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(),DURABLE_OWNED_GROUP_EVENT_MAX_BYTES,denied,&mut native).unwrap());assert_eq!(receipt,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
            let (receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||output.advance(&source,<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(),DURABLE_OWNED_GROUP_EVENT_MAX_BYTES,grant,&mut native).unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(&source as *const _ as usize,address);turns+=1;
            if cancel&&turns==policy["cancelTurn"].as_u64().unwrap() {let demand=output.next_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};let mut canceled=|_|false;let mut control=NativeEncodeControl::new_retained(&mut canceled);let(reason,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||output.advance(&source,<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(),DURABLE_OWNED_GROUP_EVENT_MAX_BYTES,grant,&mut control).unwrap_err());assert_eq!(reason.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break;}assert!(turns<policy["maximumTurns"].as_u64().unwrap(),"retained default codec makes progress");
        }
        if !cancel {let demand=output.next_take_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:1,..Default::default()};let(bytes,receipt)=output.take(grant).unwrap();assert!(receipt.fits(grant));assert_eq!(bytes,expected);assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_pack(&bytes).unwrap(),source);}
        while !output.terminal_is_empty() {let demand=output.next_close_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};let(receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||output.close(grant).unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));}
        eprintln!("[DEBUG] durable retained Pack original={address} turns={turns} cancel={cancel} exact-codec1-output=true");
    }
}

#[test]
fn durable_canonical_planning_preserves_source_and_reports_each_physical_turn() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../📦️prepared/🔗️binding/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &law["planningOutput"];
    for source in policy["sources"].as_array().unwrap() {
        let expected = source.as_str().unwrap();
        let original: DslValue = semio_framework_pack_json::from_json_str(expected, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let source = super::super::ArtifactCanonicalTreeSource::view(&original);
        let mut output = planning::DurableCanonicalOutputV1::default();
        let mut accepted = |_| true;
        let mut native = NativeEncodeControl::new_retained(&mut accepted);
        for _ in 0..10000 {
            if output.ready() { break; }
            let demand = output.next_demand();
            let funded = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            for axis in policy["deniedAxes"].as_array().unwrap() {
                let mut denied = funded;
                match axis.as_str().unwrap() {
                    "items" => denied.maximum_items = 0,
                    "copyBytes" if demand.copy_bytes != 0 => denied.maximum_copy_bytes -= 1,
                    "capacityBytes" if demand.capacity_bytes != 0 => denied.maximum_capacity_bytes -= 1,
                    "depth" if demand.depth != 0 => denied.maximum_depth -= 1,
                    _ => continue,
                }
                let (receipt, heap) = observe(|| output.advance(source, policy["maximumBytes"].as_u64().unwrap() as usize, denied, &mut native).unwrap());
                assert_eq!(receipt, Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (receipt, heap) = observe(|| output.advance(source, policy["maximumBytes"].as_u64().unwrap() as usize, funded, &mut native).unwrap());
            assert!(receipt.fits(funded));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
        }
        assert!(output.ready());
        let actual = output.take(copy_grant(4096).retained_grant()).unwrap().0;
        assert_eq!(actual, expected.as_bytes());
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual).unwrap(), serde_json::from_str::<serde_json::Value>(expected).unwrap());
        assert!(output.terminal_is_empty());
        assert_eq!(semio_framework_pack_json::to_json_string(&original), expected);
        for boundary in policy["cancelPhases"].as_array().unwrap() {
            let mut cancelled = planning::DurableCanonicalOutputV1::default();
            let live = std::cell::Cell::new(true);
            let mut observer = |_| live.get();
            let mut native = NativeEncodeControl::new_retained(&mut observer);
            for _ in 0..10000 {
                if cancelled.phase() == boundary.as_str().unwrap() { break; }
                let demand = cancelled.next_demand();
                let funded = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                cancelled.advance(source, policy["maximumBytes"].as_u64().unwrap() as usize, funded, &mut native).unwrap();
            }
            assert_eq!(cancelled.phase(), boundary.as_str().unwrap());
            live.set(false);
            let demand = cancelled.next_demand();
            let funded = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (error, heap) = observe(|| cancelled.advance(source, policy["maximumBytes"].as_u64().unwrap() as usize, funded, &mut native).unwrap_err());
            assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::Canceled);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let demand = cancelled.next_close_demand();
            let close = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (receipt, heap) = observe(|| cancelled.close(close));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            assert!(receipt.fits(close));
            assert!(cancelled.terminal_is_empty());
            assert_eq!(semio_framework_pack_json::to_json_string(&original), expected);
        }
    }
    for row in policy["refusals"].as_array().unwrap(){
        let zero:DslValue=semio_framework_pack_json::from_json_str("0",semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let original=if row["kind"]=="items"{DslValue::Array(vec![zero;row["count"].as_u64().unwrap()as usize])}else{let mut value=zero;for _ in 0..row["count"].as_u64().unwrap(){value=DslValue::Array(vec![value]);}value};
        let expected=semio_framework_pack_json::to_json_string(&original);assert!(serde_json::from_str::<serde_json::Value>(&expected).is_ok());
        let source=super::super::ArtifactCanonicalTreeSource::view(&original);let mut output=planning::DurableCanonicalOutputV1::default();let mut accepted=|_|true;let mut native=NativeEncodeControl::new_retained(&mut accepted);let mut refused=false;
        for _ in 0..1_000_000{let demand=output.next_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(result,heap)=observe(||output.advance(source,policy["maximumBytes"].as_u64().unwrap()as usize,grant,&mut native));match result{Ok(receipt)=>{assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));},Err(reason)=>{assert!(matches!(reason.kind,semio_framework_value::ValueRefusalKind::DepthLimit|semio_framework_value::ValueRefusalKind::OwnershipLimit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));refused=true;break;}}}
        assert!(refused);let demand=output.next_close_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(receipt,heap)=observe(||output.close(grant));assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(output.terminal_is_empty());assert_eq!(semio_framework_pack_json::to_json_string(&original),expected);
    }
    println!("[DEBUG] durable canonical planning preserves original source; each denied/admitted byte/capacity/depth receipt equals actual System allocation; Serde semantic output agrees; declared nesting/item refusals retain original source");
}

#[test]
fn durable_inline_terminal_retirement_requires_its_original_frame_grant(){
    use semio_framework_value::{ErasedSnapshotRetirement,ValueError,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
    struct Terminal(bool);
    impl ErasedSnapshotRetirement for Terminal{
        fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
        fn terminal_is_empty(&self)->bool{self.0}
        fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
        fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
    }
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../📦️prepared/🔗️binding/🧫️fixtures/🔣️.json")).unwrap();
    for row in law["inlineTerminalRetirement"]["cases"].as_array().unwrap(){
        let mut owner=Some(Terminal(true));let original=std::ptr::from_ref(owner.as_ref().unwrap());
        let d=crate::os_store::snapshot_read::artifact_retirement_owner_demands(&owner,512).unwrap();
        assert_eq!(d.copy_bytes,law["inlineTerminalRetirement"]["frameCopyMultiplier"].as_u64().unwrap()as usize*std::mem::size_of::<Option<Terminal>>());
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:if row["funded"].as_bool().unwrap(){d.copy_bytes}else{d.copy_bytes-1},maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:d.depth};
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::os_store::snapshot_read::artifact_retirement_owner_close(&mut owner,grant).unwrap());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(step.progress().fits(grant));assert_eq!(owner.is_some(),row["retained"].as_bool().unwrap());assert_eq!(step.progress()!=RetainedCloneProgress::default(),row["visible"].as_bool().unwrap());
        if owner.is_some(){assert_eq!(std::ptr::from_ref(owner.as_ref().unwrap()),original);crate::os_store::snapshot_read::artifact_retirement_owner_close(&mut owner,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..grant}).unwrap();}assert!(owner.is_none());
    }
    eprintln!("[DEBUG] inline terminal frame denial preserves original; exact funded copy exposes physical progress");
}

#[test]
fn durable_edit_wire_projection_preserves_sequence_without_changing_semantic_seal(){
    use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant};
    let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../📦️prepared/🔗️binding/🧫️fixtures/🔣️.json")).unwrap();
    let original=Edit::<bool>{id:"wire-edit".into(),actor:None,line:None,forwards:vec![true],inverse:Default::default(),mutation_meta:Vec::new(),verb:None,sequence_number:policy["editProjection"]["sequenceNumber"].as_i64().unwrap()as i32,started_at:"now".into(),finished_at:None};
    let address=std::ptr::from_ref(&original)as usize;let wire=::protocol::io::text::canonical::EditWireTree::view(&original);assert_eq!(std::ptr::from_ref(wire)as usize,address);let mut values=Vec::new();
    for source in [crate::os_store::ArtifactCanonicalTreeSource::view(&original)as &dyn super::super::canonical_edit::ArtifactCanonicalJson,crate::os_store::ArtifactCanonicalTreeSource::view(wire)]{
        let mut output=planning::DurableCanonicalOutputV1::default();let mut accepted=|_|true;let mut native=NativeEncodeControl::new_retained(&mut accepted);
        for _ in 0..10000{if output.ready(){break;}let d=output.next_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||output.advance(source,162000,grant,&mut native).unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(std::ptr::from_ref(&original)as usize,address);}
        assert!(output.ready());let bytes=output.take(copy_grant(4096).retained_grant()).unwrap().0;values.push(serde_json::from_slice::<serde_json::Value>(&bytes).unwrap());assert!(output.terminal_is_empty());
    }
    assert!(values[0].get("sequenceNumber").is_none());assert_eq!(values[1]["sequenceNumber"],original.sequence_number);let recovered=semio_framework_pack_json::from_json_str::<Edit<bool>>(&values[1].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(recovered.sequence_number,original.sequence_number);assert_eq!(recovered.forwards,original.forwards);values[1].as_object_mut().unwrap().remove("sequenceNumber");assert_eq!(values[0],values[1]);
    eprintln!("[DEBUG] full recovery wire retains original sequence; semantic seal fields unchanged, original tree address preserved");
}

#[test]
fn durable_binding_plans_before_consumption_and_transfers_one_original_member_per_turn() {
    std::panic::set_hook(Box::new(|information|{use std::io::Write as _;let _=writeln!(std::io::stderr().lock(),"[DEBUG] durable binding panic={information}");}));
    ::semio_framework_async::poll::resolve_ready(async {
        let (mut parent, mut drawing, mut value) = owned_three_stores().await;
        let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../📦️prepared/🔗️binding/🧫️fixtures/🔣️.json")).unwrap();
        for row in law["receiptCases"].as_array().unwrap(){let receipt=semio_framework_value::RetainedCloneProgress{copied_items:row["copiedItems"].as_u64().unwrap()as usize,copied_bytes:row["copiedBytes"].as_u64().unwrap()as usize,retained_capacity_bytes:row["retainedCapacityBytes"].as_u64().unwrap()as usize,released_bytes:row["releasedBytes"].as_u64().unwrap()as usize};assert_eq!(receipt!=Default::default(),row["visible"].as_bool().unwrap());}
        assert_eq!(law["durableExecution"]["retirementIssuer"],"captured-original");assert_eq!(law["durableExecution"]["retirementIssuerOwnership"],"moves-with-candidate");assert_eq!(law["durableExecution"]["currentFactorySlotsDuringRetirement"],"empty");
        for row in law["durableExecution"]["cases"].as_array().unwrap() {
            let started_at = row["oversizedJsonBytes"].as_u64().map(|bytes| "x".repeat(bytes as usize));
            let mut prepared = DurableOwnedThreeStorePreparedV1::from_store_prepared(store_prepared(&parent, 1, 7), store_prepared_with_started_at(&drawing, 2, 11, started_at,row["invalidDependency"].as_str(),row["validDependencyCount"].as_u64().unwrap_or(0)as usize), store_prepared(&value, 3, 13)).unwrap_or_else(|_| panic!("original candidates admitted"));
            let addresses = [prepared.parent.original_addresses(), prepared.drawing.original_addresses(), prepared.value.original_addresses()];
            let stop = row["stopStep"].as_u64().unwrap() as u8;
            {use std::io::Write as _;writeln!(std::io::stderr().lock(),"[DEBUG] durable binding case={} begins",row["id"]).unwrap();}
            for step in 0..stop {
                let grant=copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
                for turn in 0..4_000_000 {
                if prepared.binding_step!=step{break;}
                if turn%10000==0{use std::io::Write as _;writeln!(std::io::stderr().lock(),"[DEBUG] durable binding case={} phase={} turn={}",row["id"],step,turn).unwrap();}
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| prepared.advance_binding(&parent, &drawing, &value, grant));
                let receipt=match result.unwrap(){Some(receipt)=>receipt,None=>{use std::io::Write as _;let frontier=match step{0=>prepared.parent.member_encoding.frontier(),1=>prepared.drawing.member_encoding.frontier(),_=>prepared.value.member_encoding.frontier()};writeln!(std::io::stderr().lock(),"[DEBUG] durable blocked phase={step} turn={turn} frontier={frontier:?} heap={heap:?}").unwrap();
let mut mutation_issuers=[parent.mutation_retirement_factory.take(),drawing.mutation_retirement_factory.take(),value.mutation_retirement_factory.take()];let mut snapshot_issuers=[parent.snapshot_retirement_factory.take(),drawing.snapshot_retirement_factory.take(),value.snapshot_retirement_factory.take()];let mut closed=false;let mut born=0;let mut released=0;
for close_turn in 0..4_000_000{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||prepared.retire_originals_step(&mut parent,&mut drawing,&mut value,grant));let done=result.expect("blocked original's captured issuers must remain usable for controlled cancellation");let receipt=prepared.retirement_progress;assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;if done{closed=true;writeln!(std::io::stderr().lock(),"[DEBUG] blocked durable original custody cancelled turns={close_turn} born={born} released={released} six-current-issuer-slots-empty").unwrap();break;}}
assert!(closed&&prepared.originals_empty());*parent.mutation_retirement_factory=mutation_issuers[0].take();*drawing.mutation_retirement_factory=mutation_issuers[1].take();*value.mutation_retirement_factory=mutation_issuers[2].take();*parent.snapshot_retirement_factory=snapshot_issuers[0].take();*drawing.snapshot_retirement_factory=snapshot_issuers[1].take();*value.snapshot_retirement_factory=snapshot_issuers[2].take();close_demo_artifact_store(&mut parent);close_demo_artifact_store(&mut drawing);close_demo_artifact_store(&mut value);panic!("fully funded durable binding frontier returned Blocked after all original children were cancelled");}};
                assert!(receipt.fits(grant.retained_grant()));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
                if step == 0 {
                    for (owner, store) in [(&prepared.parent, &parent), (&prepared.drawing, &drawing), (&prepared.value, &value)] {
                        assert!(Arc::ptr_eq(owner.mutation_retirement.as_ref().unwrap(), store.mutation_retirement_factory.as_ref().unwrap()));
                        assert!(Arc::ptr_eq(owner.snapshot_retirement.as_ref().unwrap(), store.snapshot_retirement_factory.as_ref().unwrap()));
                    }
                }
                if (8..=10).contains(&step) && prepared.binding_step != step {
                    let (owner,store)=match step {8=>(&prepared.parent,&parent),9=>(&prepared.drawing,&drawing),_=>(&prepared.value,&value)};
                    assert!(owner.mutation_retirement.is_none()&&owner.snapshot_retirement.is_none(),"successful transfer moves captured issuers with their exact candidate");
                    let bound=owner.bound.as_ref().unwrap();assert!(Arc::ptr_eq(&bound.mutation_retirement,store.mutation_retirement_factory.as_ref().unwrap()));assert!(Arc::ptr_eq(&bound.snapshot_retirement,store.snapshot_retirement_factory.as_ref().unwrap()));
                }
                if step < 8 {
                    assert_eq!([prepared.parent.original_addresses(), prepared.drawing.original_addresses(), prepared.value.original_addresses()], addresses);
                    for original in [&prepared.parent, &prepared.drawing, &prepared.value] { let original = original.prepared.as_ref().unwrap(); assert!(original.edit().mutation_meta[0].group_id.is_none()); original.authority().validate_prepared(original).unwrap(); }
                }
                }
                assert_eq!(prepared.binding_step,step+1,"logical phase completes its actual child turns");
            }
            if row["id"] == "output-denied" {
                assert!(prepared.advance_binding(&parent, &drawing, &value, copy_grant(0)).unwrap().is_none());
                assert_eq!(prepared.binding_step, stop);
            }
            if row["id"] == "later-member-refusal" {
                let generation = drawing.generation; drawing.generation += 1;
                assert!(matches!(prepared.advance_binding(&parent, &drawing, &value, copy_grant(4096)), Err(DurableOwnedGroupDecisionError::InvalidFrontier)));
                drawing.generation = generation;
                assert_eq!([prepared.parent.original_addresses(), prepared.drawing.original_addresses(), prepared.value.original_addresses()], addresses);
            }
            if row["id"]=="later-member-metadata-refusal"{let mut refused=false;let mut accepted=0;for _ in 0..1024{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||prepared.advance_binding(&parent,&drawing,&value,copy_grant(4096)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));match result{Err(DurableOwnedGroupDecisionError::InvalidOutcome)=>{refused=true;break;},Ok(Some(receipt))=>{assert!(receipt.fits(copy_grant(4096).retained_grant()));accepted+=1;},other=>panic!("original metadata identity frontier retains exact custody: {other:?}")}}assert!(refused&&accepted>=row["validDependencyCount"].as_u64().unwrap()as usize);assert_eq!([prepared.parent.original_addresses(),prepared.drawing.original_addresses(),prepared.value.original_addresses()],addresses);}
            if row["id"] == "later-member-encoding-refusal" {
                assert!(prepared.parent.unbound.is_some());
                let mut rejected=false;for _ in 0..4_000_000 {match prepared.advance_binding(&parent,&drawing,&value,copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES)){Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge)=>{rejected=true;break;},Ok(Some(receipt))=>assert!(receipt.fits(copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES).retained_grant())),other=>panic!("oversized original retains typed refusal: {other:?}")}}assert!(rejected);
                assert_eq!([prepared.parent.original_addresses(), prepared.drawing.original_addresses(), prepared.value.original_addresses()], addresses);
            }
            if row["id"] == "decision-pack-refusal" {
                prepared.value.plan.as_mut().unwrap().recovery_pack[0] ^= 1;
                let mut rejected=false;for _ in 0..4_000_000 {match prepared.advance_binding(&parent,&drawing,&value,copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES)){Err(DurableOwnedGroupDecisionError::InvalidHash)=>{rejected=true;break;},Ok(Some(receipt))=>assert!(receipt.fits(copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES).retained_grant())),other=>panic!("tampered packet retains its exact typed refusal: {other:?}")}}assert!(rejected);
                assert_eq!([prepared.parent.original_addresses(), prepared.drawing.original_addresses(), prepared.value.original_addresses()], addresses);
            }
            let originals = [&prepared.parent, &prepared.drawing, &prepared.value].iter().filter(|owner| owner.prepared.is_some()).count();
            assert_eq!(originals, row["originalCount"].as_u64().unwrap() as usize);
            assert_eq!(3 - originals, row["transferCount"].as_u64().unwrap() as usize);
            if row["id"] == "begin-commit-refusal" {
                for _ in 0..4_000_000 { if prepared.binding_step == 14 { break; } let grant=copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES); let (receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||prepared.advance_binding(&parent, &drawing, &value, grant).unwrap().unwrap()); assert!(receipt.fits(grant.retained_grant())); assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes)); }
                let mut bound = prepared.take_bound().unwrap();
                let before = [&bound.parent, &bound.drawing, &bound.value].map(|owner| [owner.prepared.edit() as *const Edit<DemoMutation> as usize, Arc::as_ptr(owner.prepared.post_snapshot()) as usize, Arc::as_ptr(owner.prepared.authority()) as usize]);
                let issuers = [Arc::clone(&bound.parent.mutation_retirement), Arc::clone(&bound.drawing.mutation_retirement), Arc::clone(&bound.value.mutation_retirement)];
                bound.decision_pack = None;
                let ((error,bound),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bound.begin_retained_commit().err().expect("missing retained admission returns the complete original bound owner"));
                assert_eq!(error, DurableOwnedGroupDecisionError::InvalidOutcome);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                let after = [&bound.parent, &bound.drawing, &bound.value].map(|owner| [owner.prepared.edit() as *const Edit<DemoMutation> as usize, Arc::as_ptr(owner.prepared.post_snapshot()) as usize, Arc::as_ptr(owner.prepared.authority()) as usize]);
                assert_eq!(after, before);
                for (owner, issuer) in [&bound.parent, &bound.drawing, &bound.value].iter().zip(&issuers) { assert!(Arc::ptr_eq(&owner.mutation_retirement, issuer)); }
                prepared.parent.bound = Some(bound.parent); prepared.drawing.bound = Some(bound.drawing); prepared.value.bound = Some(bound.value); prepared.decision = Some(bound.decision);
            }
            let mut mutation_issuers=[parent.mutation_retirement_factory.take(),drawing.mutation_retirement_factory.take(),value.mutation_retirement_factory.take()];let mut snapshot_issuers=[parent.snapshot_retirement_factory.take(),drawing.snapshot_retirement_factory.take(),value.snapshot_retirement_factory.take()];
            let grant=copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);let mut closed=false;
            for _ in 0..4_000_000 { let before=[prepared.parent.original_addresses(),prepared.drawing.original_addresses(),prepared.value.original_addresses()]; let(_,denied_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||prepared.retire_originals_step(&mut parent,&mut drawing,&mut value,crate::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:0,..grant}).unwrap()); assert_eq!((denied_heap.requested_bytes,denied_heap.released_bytes),(0,0)); assert_eq!(prepared.retirement_progress,Default::default()); assert_eq!([prepared.parent.original_addresses(),prepared.drawing.original_addresses(),prepared.value.original_addresses()],before); let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||prepared.retire_originals_step(&mut parent,&mut drawing,&mut value,grant));let complete=result.unwrap();let receipt=prepared.retirement_progress;assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if complete{closed=true;break;} }
            assert!(closed);
            assert!(prepared.originals_empty());
            for owner in [&prepared.parent,&prepared.drawing,&prepared.value] {assert!(owner.mutation_retirement.is_none()&&owner.snapshot_retirement.is_none(),"retirement transfers every captured issuer to its exact retained candidate owner");}
            *parent.mutation_retirement_factory=mutation_issuers[0].take();*drawing.mutation_retirement_factory=mutation_issuers[1].take();*value.mutation_retirement_factory=mutation_issuers[2].take();*parent.snapshot_retirement_factory=snapshot_issuers[0].take();*drawing.snapshot_retirement_factory=snapshot_issuers[1].take();*value.snapshot_retirement_factory=snapshot_issuers[2].take();
        }
        close_demo_artifact_store(&mut parent); close_demo_artifact_store(&mut drawing); close_demo_artifact_store(&mut value);
        println!("[DEBUG] durable binding neutral cases preserve original addresses through planning; MaxItems1 transfers and denied output retain exact custody; cancellation uses captured issuers while all six current Store factory slots are empty");
    });
}
