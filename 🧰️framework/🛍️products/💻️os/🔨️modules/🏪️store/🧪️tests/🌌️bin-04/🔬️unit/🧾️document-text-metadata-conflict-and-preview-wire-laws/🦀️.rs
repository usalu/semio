/// 🧾️ Document text metadata, conflict and preview wire laws: the authoritative metadata round trips, the ops log json, the missing metadata rejections, the persisted conflicts, the quarantined accept and the preview wire reports.

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_authoritative_metadata_messages_conflicts_and_cursor() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "severity-text", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], transaction: None }, &mut identity).await.expect("apply warning");
    let edit = store.envelope().vcs.edits.last().expect("one durable edit").clone();
    let messages = store.envelope().edit_messages.last().expect("durable outcome ledger").messages.clone();
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
    let mutation_ids = stable_mutation_ids_for_edit(&edit).await.expect("stable operation identity");
    let timestamp = edit.mutation_meta.first().expect("metadata timestamp").timestamp;
    let document_id = ArtifactId(store.envelope().id.clone());
    store.0.log.envelope.conflicts.push(semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages,
        actors: vec![ActorId(edit.actor.clone().expect("edit actor"))],
        timestamp,
    });
    let files = print_document_text(store.envelope()).await.expect("print full-fidelity text");
    for record in ["inverse ", "metadata ", "message ", "conflict "] {
        assert!(files.ops.lines().any(|line| line.starts_with(record)), "missing {record:?} record: {}", files.ops);
    }
    let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse full-fidelity text");
    assert_eq!(&parsed.envelope, store.envelope());
    assert_eq!(parsed.snapshot, store.snapshot().expect("live snapshot"));
    test_support::retire_parsed_document(parsed);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🔬️ `print_ops_log`'s `metadata `/`message `/`conflict ` records now build their `data`
/// payload with `crate::os_pack::json::to_json_string` instead of `serde_json::to_string`
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02) — direct proof the two
/// encode the same ordered first-party values byte-identically, exercising both the
/// sparse (every `Option`/default field omitted) and dense (every optional field populated,
/// including `MutationOrigin`'s two non-default enum-variant-with-fields shapes) ends of each
/// type's `skip_serializing_if` surface.
#[test]
fn ops_log_records_to_json_string_match_serde_json_byte_for_byte() {
    let sparse_meta = MutationMeta {
        mutation_id: None,
        dependencies: Vec::new(),
        base_version: 1,
        author_id: None,
        timestamp: HybridLogicalTimestamp::new(1, 100),
        undo_policy: UndoPolicy::ExactBaseOnly,
        payload_hash: None,
        semantic_kind: None,
        label: None,
        group_id: None,
        origin: semio_framework_os_spr::MutationOrigin::Owner,
        transaction: None,
    };
    let dense_meta = MutationMeta {
        mutation_id: Some(MutationId("mutation-1".into())),
        dependencies: vec![MutationId("mutation-0".into())],
        base_version: 7,
        author_id: Some(ActorId("actor-1".into())),
        timestamp: HybridLogicalTimestamp::new(9, 300),
        undo_policy: UndoPolicy::TransformAgainstConcurrent,
        payload_hash: Some(semio_framework_os_spr::PayloadHash([7u8; 32])),
        semantic_kind: Some(SchemaId("demo/v1#set-n".into())),
        label: Some("set n".into()),
        group_id: Some("group-1".into()),
        origin: semio_framework_os_spr::MutationOrigin::Contributed { plugin_id: "plugin-a".into(), mutation_id: SchemaId("mutation-src".into()), payload_hash: semio_framework_os_spr::PayloadHash([3u8; 32]) },
        transaction: None,
    };
    let transaction_meta =
        MutationMeta { origin: semio_framework_os_spr::MutationOrigin::Transaction { initiator: semio_framework_os_spr::ForeignTarget { artifact_id: "artifact-1".into(), artifact_kind: "demo".into(), dialect: Some("v1".into()) } }, ..dense_meta.clone() };
    for meta in [&sparse_meta, &dense_meta, &transaction_meta] {
        let mine = semio_framework_pack_json::to_json_string(meta);
        let theirs = serde_json::to_string(&test_support::SerdeValue(&meta.to_value())).expect("serde_json encodes MutationMeta fields");
        assert_eq!(mine, theirs, "MutationMeta's ToValue/pack::json bridge diverged from serde_json for origin={:?}", meta.origin);
    }

    let sparse_message = semio_framework_os_spr::MutationMessage { level: semio_framework_diagnostic::Severity::Warning, code: semio_framework_diagnostic::FaultCode("mutation.no-op".into()), message: "no-op".into(), target: Vec::new(), op_index: None };
    let dense_message = semio_framework_os_spr::MutationMessage {
        level: semio_framework_diagnostic::Severity::Fatal,
        code: semio_framework_diagnostic::FaultCode("mutation.invariant".into()),
        message: "n invariant violated".into(),
        target: vec!["n".into()],
        op_index: Some(2),
    };
    for message in [&sparse_message, &dense_message] {
        let mine = semio_framework_pack_json::to_json_string(message);
        let theirs = serde_json::to_string(&test_support::SerdeValue(&message.to_value())).expect("serde_json encodes MutationMessage fields");
        assert_eq!(mine, theirs, "MutationMessage's ToValue/pack::json bridge diverged from serde_json for op_index={:?}", message.op_index);
    }

    let conflict = semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId("conflict-1".into()),
        kind: semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec!["edit-1".into(), "edit-2".into()] },
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: vec![sparse_message, dense_message],
        actors: vec![ActorId("actor-1".into()), ActorId("actor-2".into())],
        timestamp: HybridLogicalTimestamp::new(9, 300),
    };
    let mine = semio_framework_pack_json::to_json_string(&conflict);
    let theirs = serde_json::to_string(&test_support::SerdeValue(&conflict.to_value())).expect("serde_json encodes Conflict fields");
    assert_eq!(mine, theirs, "Conflict's ToValue/pack::json bridge diverged from serde_json");
}

#[semio_framework_async_macros::async_test]
async fn document_text_rejects_missing_metadata_and_dangling_transitions_without_synthesis() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-text", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], transaction: None }, &mut identity).await.expect("apply warning");
    let files = print_document_text(store.envelope()).await.expect("print strict text");
    let missing_metadata = files.ops.lines().filter(|line| !line.starts_with("metadata ")).collect::<Vec<_>>().join("\n");
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &missing_metadata, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("no metadata records")));

    let dangling = semio_framework_os_spr::history_transition_envelope(
        &semio_framework_os_spr::HistoryTransition::Revert { mutation_ids: vec![MutationId("unknown-edit".into())] },
        &ArtifactId("strict-text".into()),
        &ActorId("local".into()),
        Vec::new(),
        HybridLogicalTimestamp::new(0, 1),
    );
    let dangling_text = format!("{}\n{}", files.ops.trim_end(), ops_line_from_transition(&dangling).expect("transition line").print_op());
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &dangling_text, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("unknown operation unknown-edit")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn document_text_preserves_non_contiguous_edit_sequences_and_rejects_invalid_ones() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-sequence", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetN(SeveritySetN { n: 1 })], transaction: None }, &mut identity).await.expect("first edit");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetN(SeveritySetN { n: 2 })], transaction: None }, &mut identity).await.expect("second edit");
    let mut renumbered = owned_test_envelope(&store).await;
    renumbered.vcs.edits[0].sequence_number = 4;
    renumbered.vcs.edits[1].sequence_number = 17;

    let files = print_document_text(&renumbered).await.expect("print strict text");
    retire_demo_envelope(renumbered);
    let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse strict text").into_envelope();
    assert_eq!(parsed.vcs.edits.iter().map(|edit| edit.sequence_number).collect::<Vec<_>>(), vec![4, 17]);
    retire_demo_envelope(parsed);

    let invalid = files.ops.replacen("sequence=4", "sequence=-1", 1);
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &invalid, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("invalid edit sequence -1")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn persisted_conflict_requires_a_global_owned_operation_index() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "conflict-index", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], transaction: None }, &mut identity).await.expect("apply warning");
    let edit = store.envelope().vcs.edits.last().expect("one edit").clone();
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
    let mutation_ids = stable_mutation_ids_for_edit(&edit).await.expect("stable mutation ids");
    let timestamp = edit.mutation_meta.first().expect("timestamp").timestamp;
    let mut messages = store.envelope().edit_messages.last().expect("outcome message").messages.clone();
    messages[0].op_index = Some(1);
    let document_id = ArtifactId(store.envelope().id.clone());
    store.0.log.envelope.conflicts.push(semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages,
        actors: vec![ActorId(edit.actor.clone().expect("actor"))],
        timestamp,
    });
    assert!(matches!(validate_persisted_conflicts(store.envelope()).await, Err(VcsError::ValidationFailed(message)) if message.contains("operation index")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn conflict_generation_and_validation_canonicalize_repeated_actors() {
    let document_id = "repeated-conflict-actor";
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let clean = severity_mutation_envelope_at(document_id, "same-peer", "same-clean", SeverityMutation::SetN(SeveritySetN { n: 1 }), HybridLogicalTimestamp::new(1, 100));
    let mut fatal = severity_mutation_envelope_at(document_id, "same-peer", "same-fatal", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    fatal.dependencies = vec![clean.mutation_id.clone()];

    store.ingest_remote(fatal).await.expect("dependent fatal is buffered");
    let report = store.ingest_remote(clean).await.expect("ready batch is reported");
    let conflict_id = report.conflict.expect("fatal batch is quarantined");
    assert!(!report.accepted);
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("durable conflict");
    assert_eq!(conflict.actors, vec![ActorId("same-peer".into())]);
    validate_persisted_conflicts(store.envelope()).await.expect("generated actors use the canonical unique participant set");

    let mut malformed = owned_test_envelope(&store).await;
    malformed.conflicts[0].actors.push(ActorId("same-peer".into()));
    assert!(matches!(validate_persisted_conflicts(&malformed).await, Err(VcsError::ValidationFailed(message)) if message.contains("malformed actor identities")));
    retire_demo_envelope(malformed);
}

#[semio_framework_async_macros::async_test]
async fn quarantined_accept_is_atomic_when_a_later_envelope_remains_fatal() {
    let document_id = "severity-atomic";
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let clean = severity_mutation_envelope_at(document_id, "clean-peer", "clean-op", SeverityMutation::SetN(SeveritySetN { n: 1 }), HybridLogicalTimestamp::new(1, 100));
    let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    let kind = semio_framework_os_spr::ConflictKind::Quarantined { envelopes: vec![clean.clone(), fatal.clone()] };
    let timestamp = HybridLogicalTimestamp::new(9, 300);
    let mutation_ids = vec![clean.mutation_id.clone(), fatal.mutation_id.clone()];
    let conflict_id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(document_id.to_string()), &mutation_ids, &timestamp).await;
    store.0.log.envelope.conflicts.push(semio_framework_os_spr::Conflict {
        id: conflict_id.clone(),
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: vec![semio_framework_os_spr::MutationMessage {
            level: semio_framework_diagnostic::Severity::Fatal,
            code: semio_framework_diagnostic::FaultCode("mutation.invariant".to_string()),
            message: "n invariant violated".to_string(),
            target: vec!["n".to_string()],
            op_index: Some(0),
        }],
        actors: vec![clean.actor.clone(), fatal.actor.clone()],
        timestamp,
    });
    validate_persisted_conflicts(store.envelope()).await.expect("well-formed quarantine fixture");
    let before = owned_test_envelope(&store).await;
    let generation = store.generation();

    let report = store.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Accept).await.expect("fatal outcome is reported, not an infrastructure error");

    assert!(!report.accepted);
    assert_eq!(report.replayed.len(), 2, "the aggregate report must retain the clean and fatal replay reports");
    assert_eq!(store.envelope(), &before, "the candidate's earlier clean mutation must never leak through a later fatal rejection");
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(0) });
    assert_eq!(store.generation(), generation, "a rejected candidate does not invalidate the source store");
    assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict retained").status, semio_framework_os_spr::ConflictStatus::Open);
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn a_remote_event_rejected_by_policy_stays_quarantined_out_of_history() {
    let document_id = "severity-events";
    let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    let mut local = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let report = local.ingest_remote(fatal).await.expect("a policy rejection is a report");
    assert!(!report.accepted);
    assert_eq!(local.snapshot().expect("local content remains unchanged"), DemoSnapshot { n: Some(0) });
    assert!(local.envelope().vcs.edits.is_empty(), "rejected remote edits are never adopted into local history");
    assert_eq!(local.conflicts().len(), 1);
    assert_eq!(local.conflicts()[0].status, semio_framework_os_spr::ConflictStatus::Open);
}

/// 🧪️ `preview_wire`'s headline law (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
/// FIRST-CLASS-CONFLICTS` §C6): its output is exactly what independently folding the same ops
/// through the diff engine (`apply_mutation`, the same primitive `replay_mutations` itself calls
/// on the real apply path) computes, stamped with `op_index`, and the live store is byte-
/// identical before and after — a pure dry run all the way through, never applying anything even
/// though it threads state forward internally to preview op `i` against `0..i`'s outcome.
///
/// 🧮️ Independently folds the SAME ops through the exact primitive the real apply path
/// (`replay_mutations`) itself calls (`apply_mutation`) — the ground truth `preview_wire`
/// must match. `replay_mutations` does not yet surface its own messages (lane 1-A's pending
/// C6 work, see `📓️w1-e-report.md`), so this recomputation is the closest available proof.
#[semio_framework_async_macros::async_test]
async fn preview_wire_reports_the_same_messages_the_real_apply_would_produce_and_changes_nothing() {
    let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let before = owned_test_envelope(&store).await;

    let ops: Vec<Vec<u8>> = vec![
        SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode clean"),
        SeverityMutation::SetWarningN(SetWarningN { n: 2 }).encode_op().expect("encode warn"),
        SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode error"),
        SeverityMutation::SetFatalN(SetFatalN { n: 99 }).encode_op().expect("encode fatal"),
    ];

    let messages = store.preview_wire(&ops).await;

    let mut expected = Vec::new();
    let mut running = DemoSnapshot { n: Some(0) };
    for (index, op) in ops.iter().enumerate() {
        let mutation = SeverityMutation::decode_op(op).expect("decode");
        let (next, op_messages) = apply_mutation(&running, &mutation).expect("preview fixture diff applies");
        for message in op_messages {
            expected.push(message.at_op(index as u32));
        }
        running = next;
    }
    assert_eq!(messages, expected, "preview_wire must report exactly the messages the real diff engine computes");
    assert_eq!(messages.len(), 3, "one message each for warn/error/fatal; the clean op is silent");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning);
    assert_eq!(messages[0].op_index, Some(1));
    assert_eq!(messages[1].level, semio_framework_diagnostic::Severity::Error);
    assert_eq!(messages[1].op_index, Some(2));
    assert_eq!(messages[2].level, semio_framework_diagnostic::Severity::Fatal);
    assert_eq!(messages[2].op_index, Some(3));

    assert_eq!(store.envelope(), &before, "preview_wire is a pure dry run: the live store is byte-identical afterward");
    assert_eq!(store.snapshot().expect("snapshot unaffected"), DemoSnapshot { n: Some(0) }, "preview_wire never advances the live cursor");
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn preview_wire_reports_a_fatal_message_for_undecodable_op_bytes_and_stops_there() {
    let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-malformed", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let ops: Vec<Vec<u8>> = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode"), vec![0xff, 0xff, 0xff]];

    let messages = store.preview_wire(&ops).await;

    assert_eq!(messages.len(), 1, "the clean op is silent; the malformed op reports one structural message and stops");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Fatal);
    assert_eq!(messages[0].code.0, "mutation.invariant");
    assert_eq!(messages[0].op_index, Some(1));
}

#[semio_framework_async_macros::async_test]
async fn spr_round_trip_preserves_edit_messages_and_conflicts() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initial = DemoSnapshot { n: Some(0) };
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "durable-outcomes", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let receipt = store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 2 })], transaction: None }, &mut identity).await.expect("warning is accepted");
    let edit_id = receipt.edit_ids.first().expect("one durable edit").clone();
    let messages = store.messages_for_edit(&edit_id).to_vec();
    let edit = store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id).expect("durable edit");
    let mutation_ids = stable_mutation_ids_for_edit(edit).await.expect("durable edit carries operation identity");
    let actors = vec![ActorId(edit.actor.clone().expect("durable edit has an actor"))];
    let timestamp = edit.mutation_meta.first().expect("durable edit carries timestamp").timestamp;
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit_id.clone()] };
    let artifact_id = ArtifactId(store.envelope().id.clone());
    let conflict_id = semio_framework_os_spr::ConflictId::new(&kind, &artifact_id, &mutation_ids, &timestamp).await;
    store.0.log.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: conflict_id, kind, status: semio_framework_os_spr::ConflictStatus::Open, messages: messages.clone(), actors, timestamp });

    let pack = initial.encode_pack();
    let spr = print_document_spr(store.envelope()).await.expect("outcome history encodes");
    let parsed = parse_document_spr::<DemoSnapshot, SeverityMutation>(&pack, &spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("outcome history decodes");
    assert_eq!(parsed.envelope.edit_messages, store.envelope().edit_messages);
    assert_eq!(parsed.envelope.conflicts, store.envelope().conflicts);

    let restored = ArtifactStore::new(parsed.envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(restored.messages_for_edit(&edit_id), messages);
    assert_eq!(restored.conflicts(), store.conflicts());
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}
