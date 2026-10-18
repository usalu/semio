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
    for _ in 0..128 {
        match host.advance(grant).expect("mounted assembly commit turn") {
            DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_) => assert!(host.acknowledge()),
            DurableOwnedThreeStoreCommitAdvanceV1::Complete => break,
            DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked => {}
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
    let (parent, drawing, value) = owned_three_stores().await;
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let (mut assembly, _) = map_assembly(parent, drawing, value, None, FakeJournalResolution::Commit, state.clone());
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    while assembly.phase() != DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing {
        assembly.advance(grant).expect("pre-cancellation assembly turn");
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
    for _ in 0..128 {
        match host.advance(grant) {
            Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                uncertain = true;
                let snapshot = host.capture_snapshot().expect("uncertain journal retains one all-old snapshot");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
                assert_eq!(state.lock().unwrap().begins, 1);
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_)) => assert!(host.acknowledge()),
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
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
