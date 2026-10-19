/// 🔁️ Spr round trip, message vocabulary and text format helper laws: the spr round trips of the state dependency, the persisted message vocabulary, the history parse rejection and the ops author conversion.

/// 🧾️ The two state-dependent `Error` codes of the outcome vocabulary — `mutation.target-referenced` (the target is still
/// referenced) and `mutation.target-mismatch` (the payload contradicts the target's current state) — survive the persisted
/// `.spr` history round trip, and the persisted-message gate refuses either one at any other level.
#[semio_framework_async_macros::async_test]
async fn spr_round_trip_preserves_state_dependent_error_codes() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initial = DemoSnapshot { n: Some(0) };
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "state-dependent-codes", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let receipt = store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 2 })], transaction: None }, &mut identity).await.expect("warning is accepted");
    let edit_id = receipt.edit_ids.first().expect("one durable edit").clone();
    let edit = store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id).expect("durable edit");
    let mutation_ids = stable_mutation_ids_for_edit(edit).await.expect("durable edit carries operation identity");
    let actors = vec![ActorId(edit.actor.clone().expect("durable edit has an actor"))];
    let timestamp = edit.mutation_meta.first().expect("durable edit carries timestamp").timestamp;
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit_id.clone()] };
    let conflict_id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(store.envelope().id.clone()), &mutation_ids, &timestamp).await;
    let messages: Vec<semio_framework_os_spr::MutationMessage> = ["mutation.target-referenced", "mutation.target-mismatch"].into_iter().map(|code| semio_framework_os_spr::MutationMessage::error(code, format!("{code} persisted")).at(["node-1"]).at_op(0)).collect();
    store.0.log.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: conflict_id, kind, status: semio_framework_os_spr::ConflictStatus::Open, messages, actors, timestamp });

    let pack = initial.encode_pack();
    let spr = print_document_spr(store.envelope()).await.expect("state-dependent codes encode");
    let parsed = parse_document_spr::<DemoSnapshot, SeverityMutation>(&pack, &spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("state-dependent codes decode");
    assert_eq!(parsed.envelope.conflicts, store.envelope().conflicts);
    let restored = ArtifactStore::new(parsed.envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(restored.conflicts(), store.conflicts());
    let codes: Vec<&str> = restored.conflicts().last().expect("the conflict survives").messages.iter().map(|message| message.code.0.as_str()).collect();
    assert_eq!(codes, ["mutation.target-referenced", "mutation.target-mismatch"]);

    for message in &mut store.0.log.envelope.conflicts.last_mut().expect("the conflict").messages {
        message.level = semio_framework_diagnostic::Severity::Fatal;
    }
    let error = print_document_spr(store.envelope()).await.expect_err("a state-dependent code is an Error, never Fatal");
    assert!(matches!(&error, VcsError::ValidationFailed(detail) if detail.contains("malformed mutation message")), "{error:?}");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 📖️ The persisted-message gate walks the language-agnostic outcome vocabulary (`📡️replication/🎮️mutation/🧪️tests/🧫️fixtures/
/// 🧫️outcome-code`): every code persists at exactly its level and at no other, the apply family persists `Fatal`, and every
/// code the vocabulary rejects is refused as unknown.
#[semio_framework_async_macros::async_test]
async fn persisted_messages_admit_exactly_the_outcome_vocabulary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧫️fixtures/🧫️outcome-code/🔣️.json")).expect("outcome code fixture parses");
    let levels = [("info", semio_framework_diagnostic::Severity::Info), ("warning", semio_framework_diagnostic::Severity::Warning), ("error", semio_framework_diagnostic::Severity::Error), ("fatal", semio_framework_diagnostic::Severity::Fatal)];
    let level_of = |name: &str| levels.iter().find(|(known, _)| *known == name).map(|(_, level)| *level).expect("fixture level");
    let persisted = |code: &str, level: semio_framework_diagnostic::Severity| semio_framework_os_spr::MutationMessage {
        level,
        code: semio_framework_diagnostic::FaultCode::new(code),
        message: format!("{code} persisted"),
        target: vec!["node-1".to_string()],
        op_index: Some(0),
    };
    let mut admitted: Vec<(String, semio_framework_diagnostic::Severity)> = fixture["codes"].as_array().expect("codes").iter().map(|row| (row["code"].as_str().expect("code").to_string(), level_of(row["level"].as_str().expect("level")))).collect();
    admitted.extend(fixture["apply"]["accepted"].as_array().expect("apply codes").iter().map(|code| (code.as_str().expect("apply code").to_string(), level_of(fixture["apply"]["level"].as_str().expect("apply level")))));
    for (code, level) in &admitted {
        assert_eq!(expected_mutation_message_level(code).await, Some(*level), "{code}");
        validate_persisted_message(&persisted(code, *level), Some(1)).await.unwrap_or_else(|error| panic!("{code} persists at its level: {error:?}"));
        for (_, other) in levels.iter().filter(|(_, other)| other != level) {
            assert!(validate_persisted_message(&persisted(code, *other), Some(1)).await.is_err(), "{code} is refused at {other:?}");
        }
    }
    for code in fixture["rejected"].as_array().expect("rejected codes").iter().map(|code| code.as_str().expect("rejected code")) {
        assert_eq!(expected_mutation_message_level(code).await, None, "{code}");
        assert!(validate_persisted_message(&persisted(code, semio_framework_diagnostic::Severity::Error), Some(1)).await.is_err(), "{code} is refused");
    }
}

#[semio_framework_async_macros::async_test]
async fn spr_parse_rejects_history_without_authoritative_operation_metadata() {
    let initial = DemoSnapshot { n: Some(0) };
    let history = semio_framework_os_spr::HistoryLog {
        doc_id: "metadata-required".to_string(),
        schema: "demo/v1".to_string(),
        edits: [semio_framework_os_spr::HistoryEdit {
            line: None,
            id: "edit-1".to_string(),
            actor: Some("author".into()),
            started_at: String::new(),
            finished_at: None,
            verb: None,
            ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode")) }],
            inverse: Vec::new().into(),
            meta: None,
            lane: None,
        }].into_iter().collect(),
        ..Default::default()
    };
    let spr = semio_framework_os_spr::encode_history(&history, &semio_framework_os_spr::EncodeOptions::default()).await.expect("encode fixture");
    let error = parse_document_spr::<DemoSnapshot, SeverityMutation>(&initial.encode_pack(), &spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect_err("authoritative history must never synthesize operation identity");
    assert!(error.message.contains("authoritative operation metadata"));
}


//#region 🔖️TextFormatHelpers
#[semio_framework_async_macros::async_test]
async fn ops_author_conversion_keeps_the_avatar_a_commit_transition_covers() {
    let author = Author { id: "a1".into(), name: "Alice".into(), avatar: Some("http://example/a1.png".into()) };
    let round_tripped: Author = OpsAuthor::from(&author).into();
    assert_eq!(round_tripped, author, "the avatar is part of the commit payload the transition id covers, so the .ops text must carry it");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_commit_round_trips_including_delimiter_and_quote_characters_in_authors() {
    let header = OpsHeaderLine::Commit {
        id: "transition-1".to_string(),
        actor: "actor-a".into(),
        clock: "18.0.0".to_string(),
        after: vec!["op-1".to_string()],
        checkpoint: "c1".to_string(),
        parent: None,
        change: "ch1".to_string(),
        operations: vec!["op-1".to_string(), "op-2".to_string()],
        description: None,
        saved: "17".to_string(),
        by: vec![OpsAuthor { id: "a:1,x".to_string(), name: "Alice, A. \"the great\"".to_string(), avatar: None }, OpsAuthor { id: "b2".to_string(), name: "Bob".to_string(), avatar: Some("b.png".to_string()) }],
        message: Some("first \"checkpoint\"".to_string()),
        at: "18".to_string(),
        line: Vec::new(),
    };
    let printed = header.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    assert!(!printed.contains("parent="), "an absent optional field must be omitted, not printed as a '-' placeholder: {printed}");
    let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, header, "OpsHeaderLine::Commit round trip diverged for {printed:?}");
}
//#endregion 🔖️TextFormatHelpers
