/// 🎰️ Map host slot, foreign anchor and carrier laws: the fixed host slot and its cancellation, the foreign anchor refusal and the json and byte carrier preservation.

#[semio_framework_async_macros::async_test]
async fn durable_map_fixed_host_slot_retains_every_live_owner_across_request_error_until_terminal_handoff() {
    let (parent, drawing, value) = owned_three_stores().await;
    let coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained Map coordinator before mounting any request-facing turn");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let sink = Box::new(FakeJournalSink { resolution: FakeJournalResolution::ErrorThenCommit, state: Arc::clone(&state) });
    let mut host = coordinator.mount_map(parent, drawing, value, sink);
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    let mut observed_uncertain_error = false;
    let mut observed_committed = false;
    for _ in 0..128 {
        let request_observation = host.advance(grant);
        match request_observation {
            Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                observed_uncertain_error = true;
                assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::Journal));
                let snapshot = host.capture_snapshot().expect("dropped request observation leaves the fixed host's coherent Map read retained");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt)) => {
                observed_committed = true;
                let snapshot = host.capture_snapshot().expect("mounted committed observation retains one coherent Map read");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(7), Some(11), Some(13)]);
                drop(receipt);
                assert!(host.acknowledge(), "only the receipt retained by the fixed host can unlock terminal handoff");
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
            Err(error) => panic!("mounted Map operation returned an unexpected terminal error: {error}"),
        }
    }
    assert!(observed_uncertain_error && observed_committed);
    let owners = host.take_terminal_owners().expect("terminal Map host returns every live Store and sink owner exactly once");
    assert!(host.terminal_is_empty());
    assert!(host.take_terminal_owners().is_none());
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
    }
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_fixed_host_slot_cancellation_after_uncertain_io_waits_for_trusted_absence() {
    let (parent, drawing, value) = owned_three_stores().await;
    let coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained Map coordinator before mounting any request-facing turn");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let sink = Box::new(FakeJournalSink { resolution: FakeJournalResolution::ErrorThenAbsentAfterCancel, state: Arc::clone(&state) });
    let mut host = coordinator.mount_map(parent, drawing, value, sink);
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    let mut dropped_uncertain_request = false;
    for _ in 0..128 {
        match host.advance(grant) {
            Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                dropped_uncertain_request = true;
                assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::Journal));
                assert!(host.cancel());
                let snapshot = host.capture_snapshot().expect("uncertain cancellation retains one all-old Map snapshot");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_)) => panic!("trusted absence cannot produce a commit acknowledgement"),
            Err(error) => panic!("fixed Map host returned an unexpected cancellation error: {error}"),
        }
    }
    assert!(dropped_uncertain_request);
    let owners = host.take_terminal_owners().expect("trusted absence terminates the fixed host before owner handoff");
    assert!(host.terminal_is_empty());
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
    assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [0, 0, 0]);
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
        assert!(journal.cancelled);
    }
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::WrongAnchorThenAbsentAfterCancel, state: Arc::clone(&state) };
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
    }
    coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("journal first pending turn");
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant), Err(DurableOwnedGroupDecisionError::InvalidHash));
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::Journal);
    let pending = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("foreign receipt cannot flip visibility");
    assert_eq!([pending.parent.n, pending.drawing.n, pending.value.n], [Some(0), Some(0), Some(0)]);
    assert!(coordinator.cancel());
    for _ in 0..64 {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("trusted absence cleanup turn");
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    let journal = state.lock().unwrap();
    assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[test]
fn durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess() {
    let fixture = fixture();
    let numeric = DslValue::Object(vec![("u64".into(), DslValue::uint(u64::MAX)), ("i64".into(), DslValue::int(i64::MIN)), ("float".into(), DslValue::float(1.5))]);
    let encoded = semio_framework_pack_json::to_json_string(&numeric);
    for row in fixture["carrier"]["numericCases"].as_array().unwrap() {
        assert!(encoded.contains(row["canonical"].as_str().unwrap()));
    }
    let decoded: DslValue = parse_canonical_json_value(encoded.as_bytes()).expect("typed canonical JSON preserves numeric tags and extrema");
    assert_eq!(decoded, numeric);
    let clock = HybridLogicalTimestamp { actor: u64::MAX, physical_ms: u64::MAX, logical: u64::MAX };
    let clock_json = semio_framework_pack_json::to_json_string(&clock);
    assert!(clock_json.matches("18446744073709551615").count() == 3);
    assert_eq!(parse_canonical_json_value::<HybridLogicalTimestamp>(clock_json.as_bytes()).unwrap(), clock);

    let control = fixture["carrier"]["escapedControlIdentity"].as_str().unwrap();
    assert!(!valid_identity(control));
    let raw_control = b"\"map\0x\"";
    assert_eq!(validate_json_budget(raw_control), Err(DurableOwnedGroupDecisionError::NonCanonical));
    let over_depth = format!("{}0{}", "[".repeat(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH + 1), "]".repeat(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH + 1));
    assert_eq!(validate_json_budget(over_depth.as_bytes()), Err(DurableOwnedGroupDecisionError::InvalidOutcome));
    let over_items = format!("[{}]", std::iter::repeat_n("0", DURABLE_OWNED_GROUP_JSON_MAX_ITEMS + 1).collect::<Vec<_>>().join(","));
    assert!(over_items.len() <= DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES);
    assert_eq!(validate_json_budget(over_items.as_bytes()), Err(DurableOwnedGroupDecisionError::InvalidOutcome));
}

#[test]
fn durable_decision_rejects_deflate_expansion_before_document_body_allocation() {
    let decision = decision();
    let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).unwrap();
    let mut expanded_record = decision.__dsl_to_record();
    expanded_record.fields.insert(u16::MAX, FieldValue::Bytes64(vec![0; DURABLE_OWNED_GROUP_EVENT_MAX_BYTES + 1]));
    let mut expanded_options = PackEncodeOptions::default();
    expanded_options.chunk_threshold = u64::MAX;
    expanded_options.frame_size = u64::MAX;
    let expanded_inner = pack_rt::encode_document(&DurableOwnedThreeMemberDecisionV1::__dsl_spec(), &expanded_record, &expanded_options).unwrap();
    let expanded = semio_framework_os_semio::wrap_binary(&envelope, &expanded_inner);
    assert!(expanded.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&expanded), Err(DurableOwnedGroupDecisionError::EventTooLarge));

    let mut framed_options = PackEncodeOptions::default();
    framed_options.frame_size = 64;
    let framed_inner = pack_rt::encode_document(&DurableOwnedThreeMemberDecisionV1::__dsl_spec(), &decision.__dsl_to_record(), &framed_options).unwrap();
    let framed = semio_framework_os_semio::wrap_binary(&envelope, &framed_inner);
    assert!(framed.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&framed), Err(DurableOwnedGroupDecisionError::NonCanonical));
}

#[test]
fn durable_byte_carriers_preserve_octets_under_the_declared_ownership_ceiling() {
    let fixture = fixture();
    let contract = &fixture["recoveryOctets"];
    for binding in contract["bindings"].as_array().unwrap() {
        let spec = match binding["record"].as_str().unwrap() {
            "unbound" => DurableUnboundOneItemOutcomeV1::__dsl_spec(),
            "bound" => DurableBoundOneItemOutcomeV1::__dsl_spec(),
            _ => panic!("unknown recovery record"),
        };
        for key in binding["fields"].as_array().unwrap() {
            let field = spec.fields.iter().find(|field| field.key == key.as_str().unwrap()).unwrap();
            assert!(matches!(field.shape, Shape::Bytes64), "{} must retain octets", field.key);
        }
    }
    let options = PackDecodeOptions {
        verification: PackVerificationLevel::Full,
        preserve_unknown: false,
        limits: semio_framework_os_pack::PackLimits {
            max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            max_symbols: 128,
            max_depth: 32,
            max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            max_total_alloc: contract["ownershipBytes"].as_u64().unwrap(),
        },
    };
    for case in contract["cases"].as_array().unwrap() {
        let octets: Vec<u8> = case["octets"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
        let bytes = octets.repeat(case["repeat"].as_u64().unwrap() as usize);
        assert_eq!(bytes.len() as u64, case["length"].as_u64().unwrap());
        assert_eq!(semio_framework_hash::sha256_hex(&bytes), case["sha256"].as_str().unwrap());
        assert_eq!(semio_framework_value::bytes::encode_base64(&bytes), case["base64"].as_str().unwrap());
        let unbound = DurableUnboundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: PARENT_RECOVERY_SCHEMA.into(),
            operation: 101,
            base_generation: 7,
            base_revision: [0; 32],
            base_applied_edit_count: 1,
            next_sequence_number: 11,
            next_clock_canonical_json: bytes.clone(),
            actor: "owner".into(),
            edit_without_group_canonical_json: bytes.clone(),
            post_snapshot_pack: bytes.clone(),
        };
        let bound = DurableBoundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: PARENT_RECOVERY_SCHEMA.into(),
            operation: 101,
            base_generation: 7,
            base_revision: [0; 32],
            base_applied_edit_count: 1,
            next_sequence_number: 11,
            next_clock_canonical_json: bytes.clone(),
            actor: "owner".into(),
            group_id: "0".repeat(64),
            edit_canonical_json: bytes.clone(),
            post_snapshot_pack: bytes.clone(),
            edit_digest: [0; 32],
            post_generation: 8,
            post_revision: [0; 32],
        };
        for (binding, spec, record) in [(&contract["bindings"][0], DurableUnboundOneItemOutcomeV1::__dsl_spec(), unbound.__dsl_to_record()), (&contract["bindings"][1], DurableBoundOneItemOutcomeV1::__dsl_spec(), bound.__dsl_to_record())] {
            for key in binding["fields"].as_array().unwrap() {
                let field = spec.fields.iter().find(|field| field.key == key.as_str().unwrap()).unwrap();
                assert_eq!(record.get(field.id), Some(&FieldValue::Bytes64(bytes.clone())));
            }
        }
        let unbound_pack = unbound.encode_pack();
        let bound_pack = bound.encode_pack();
        let (decoded_unbound, unbound_requested) = semio_framework_trace_memory_testing::observe(|| DurableUnboundOneItemOutcomeV1::decode_pack_with(&unbound_pack, &options).unwrap());
        let (decoded_bound, bound_requested) = semio_framework_trace_memory_testing::observe(|| DurableBoundOneItemOutcomeV1::decode_pack_with(&bound_pack, &options).unwrap());
        assert_eq!(decoded_unbound, unbound);
        assert_eq!(decoded_bound, bound);
        assert!(unbound_requested <= options.limits.max_total_alloc as usize);
        assert!(bound_requested <= options.limits.max_total_alloc as usize);
    }
}
