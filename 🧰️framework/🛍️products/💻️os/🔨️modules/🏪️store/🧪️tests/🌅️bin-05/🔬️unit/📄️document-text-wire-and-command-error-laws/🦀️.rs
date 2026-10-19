/// 📄️ Document text, wire and command error laws: the canonical committed wire witnesses, the ops header line round trips and rejections, the document text round trips, the shared history validation, the rejected empty apply, undo and redo, the unknown checkpoint and alternative and the text and binary command dispatch.

/// 🧾️ Every committed wire witness of a store fixture aggregate decodes through the aggregate's `FromValue` and re-encodes to
/// exactly the committed JSON: the witness is the canonical Rust wire of that leaf.
#[test]
fn committed_wire_witnesses_are_the_canonical_wire() {
    use semio_framework_os_store_test_support::assert_wire_witness as assert_witness;
    assert_witness::<DemoMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<DemoMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🗑️delete-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<DemoMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/➕️add-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<DemoMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<TimestampedMutation>(include_str!("../../../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<TimestampedMutation>(include_str!("../../../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/⚠️set-warning-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🚫️set-error-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🛑️set-fatal-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<ValidatedMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🛂️validated/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<ValidatedMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🛂️validated/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<LossyMutation>(include_str!("../../../../🧪️testing/🧬️mutations/🪤️lossy/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../../🧫️fixtures/🧬️mutations/📌️commit-space-checkpoint/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../../🧫️fixtures/🧬️mutations/🌿️create-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../../🧫️fixtures/🧬️mutations/🔀️switch-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../../🧫️fixtures/🧬️mutations/🗑️remove-space-checkpoint/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../../🧫️fixtures/🧬️mutations/🧹️remove-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../../🧫️fixtures/🧬️mutations/🎯️set-active-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
}


#[semio_framework_async_macros::async_test]
async fn ops_header_line_edit_round_trips_including_a_quoted_description() {
    let header = OpsHeaderLine::Edit { id: "e1".to_string(), sequence: 42, started: "1".to_string(), actor: None, finished: None, verb: None, line: Vec::new() };
    let printed = header.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    assert!(!printed.contains("actor="), "an absent optional field must be omitted: {printed}");
    let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, header, "OpsHeaderLine::Edit round trip diverged for {printed:?}");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_revert_round_trips_its_operations_and_dependencies() {
    let header = OpsHeaderLine::Revert { id: "transition-2".to_string(), actor: "actor-a".into(), clock: "21.3.0".to_string(), after: vec!["e1".to_string()], operations: vec!["e1".to_string(), "e3".to_string()] };
    let printed = header.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, header, "OpsHeaderLine::Revert round trip diverged for {printed:?}");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_parse_op_rejects_a_line_with_no_known_keyword() {
    let error = OpsHeaderLine::parse_op("not a structural line").unwrap_err();
    assert!(error.message.contains("unknown operation line"), "got {error:?}");
}

#[semio_framework_async_macros::async_test]
async fn parse_document_text_rejects_a_header_line_missing_its_required_positional_id() {
    let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "checkout\n".to_string() };
    let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_err();
    assert!(error.message.contains("expected Text"), "got {error:?}");
    assert_eq!(error.span.line, 1);
}

#[semio_framework_async_macros::async_test]
async fn parse_document_text_rejects_an_unknown_header_line_keyword() {
    let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nbogus id=x\n".to_string() };
    let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_err();
    assert!(error.message.contains("unknown operation line"), "got {error:?}");
    assert_eq!(error.span.line, 2);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_with_an_active_alternative_and_a_quoted_description() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "branch \"a\"".into() }, &mut identity).await.expect("create alternative (auto-commits and activates it)");
    assert!(store.envelope().active_alternative_id.is_some(), "precondition: an alternative is active");
    let files = print_document_text(store.envelope()).await.expect("print document text");
    assert!(files.ops.lines().any(|line| line.starts_with("branch ")), "an alternative must print as a `branch` transition line: {}", files.ops);
    test_support::assert_document_pack_round_trip(&store).await;
    let parsed = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse document text");
    assert!(parsed.envelope.active_alternative_id.is_none(), "the ops text is the shared log and hydrates to the trunk");
    assert_eq!(parsed.snapshot.n, store.snapshot().expect("snapshot").n);
    test_support::retire_parsed_document(parsed);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🔁️ E1 is withdrawn by a revert transition and stays withdrawn once the same author's e2 lands:
/// the text states the events, the reload folds them into the same position.
#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_a_cursor_after_undo_then_apply_interleaving() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply e1");
    store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo e1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply e2");
    assert_eq!(store.applied_edit_ids().len(), 1, "only e2 is applied");
    let files = print_document_text(store.envelope()).await.expect("print document text");
    assert!(files.ops.lines().any(|line| line.starts_with("revert ")), "the undo must print as a `revert` transition line: {}", files.ops);
    let parsed = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
    assert_eq!(parsed.envelope.cursor, store.envelope().cursor.clone(), "cursor diverged across a print/parse round trip");
    assert_eq!(parsed.snapshot.n, Some(2), "restored snapshot must reflect only the applied edit (e2), not both");
    test_support::retire_parsed_document(parsed);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🔐️ The save→load→undo proof (contract's runtime-behavior requirement): a store's
/// undo/redo position survives a full pack+spr save/load cycle, not just its snapshot value.
///
/// Save: print_document_pack persists pack (initial snapshot) + spr (real inverse/meta,
/// AND the cursor reflecting exactly "e1 applied, e2 in redo").
///
/// Load: a FRESH store built only from persisted bytes — no access to the original `store`.
///
/// Redo restores e2 — proving the redo stack (not just applied_edit_ids) survived.
///
/// Undo twice from here reaches the true initial state.
#[semio_framework_async_macros::async_test]
async fn save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply e1");
    let post_e1 = store.snapshot().expect("post-e1 snapshot");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply e2");
    let post_e2 = store.snapshot().expect("post-e2 snapshot");
    store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo e2");
    assert_eq!(store.snapshot().expect("live snapshot"), post_e1, "precondition: live store is back at post-e1");
    test_support::assert_live_equals_replay(&store).await;

    let pack_files = print_document_pack(store.envelope()).await.expect("print document pack");
    assert!(!pack_files.spr.is_empty(), "spr bytes must be non-empty once an edit exists");

    let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&pack_files.pack, &pack_files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
    assert_eq!(parsed.snapshot, post_e1, "loaded snapshot must equal post-e1, proving undo position survived the save");
    let mut reloaded = ArtifactStore::new(parsed.envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.snapshot().expect("reloaded snapshot"), post_e1, "ArtifactStore::new must seed live state from the persisted cursor");
    assert_eq!(reloaded.applied_edit_ids(), store.applied_edit_ids(), "applied_edit_ids must survive the round trip");
    test_support::assert_live_equals_replay(&reloaded).await;

    reloaded.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("redo e2 after reload");
    assert_eq!(reloaded.snapshot().expect("post-redo snapshot"), post_e2);
    test_support::assert_live_equals_replay(&reloaded).await;

    reloaded.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo e2 again");
    reloaded.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo e1");
    assert_eq!(reloaded.snapshot().expect("final snapshot"), DemoSnapshot { n: Some(0) });
    test_support::assert_live_equals_replay(&reloaded).await;
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn document_codecs_share_complete_authoritative_history_validation() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope = create_document_envelope("demo/v1", "codec-validation", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }, &mut identity).await.expect("commit");
    let text = print_document_text(store.envelope()).await.expect("text fixture");
    let duplicate_commit = text.ops.lines().find(|line| line.starts_with("commit ")).expect("one persisted commit transition");
    let malformed_text = format!("{}\n{duplicate_commit}", text.ops);
    assert!(matches!(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &malformed_text, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("repeats transition")));

    let pack = print_document_pack(store.envelope()).await.expect("binary fixture");
    let mut history = semio_framework_os_spr::decode_history(&pack.spr, &semio_framework_os_spr::DecodeOptions::default()).await.expect("decode history");
    history.transitions.push(history.transitions.first().expect("one persisted commit transition").clone());
    let malformed_spr = semio_framework_os_spr::encode_history(&history, &semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..semio_framework_os_spr::EncodeOptions::default() }).await.expect("encode malformed history");
    assert!(matches!(parse_document_spr::<DemoSnapshot, DemoMutation>(&pack.pack, &malformed_spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("repeats transition")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


//#region 🔖️CommandErrorPaths
#[semio_framework_async_macros::async_test]
async fn apply_with_no_mutations_is_rejected() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let error = store.dispatch(ArtifactCommand::Apply { mutations: Vec::new(), transaction: None }, &mut identity).await.unwrap_err();
    assert_eq!(error, VcsError::EmptyApply);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn undo_with_nothing_applied_is_rejected() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(store.dispatch(ArtifactCommand::Undo, &mut identity).await.unwrap_err(), VcsError::NothingToUndo);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn redo_with_nothing_undone_is_rejected() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(store.dispatch(ArtifactCommand::Redo, &mut identity).await.unwrap_err(), VcsError::NothingToRedo);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn checkout_of_an_unknown_checkpoint_is_rejected() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let error = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "nope".into() }, &mut identity).await.unwrap_err();
    assert_eq!(error, VcsError::UnknownChange("nope".into()));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn switch_to_an_unknown_alternative_is_rejected() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let error = store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: "nope".into() }, &mut identity).await.unwrap_err();
    assert_eq!(error, VcsError::UnknownAlternative("nope".into()));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn malformed_alternative_checkpoint_pin_is_rejected_at_construction() {
    let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.vcs.alternatives.try_push(Alternative { id: "alt-dangling".into(), name: "dangling".into(), checkpoint_ids: vec!["checkpoint-that-was-never-recorded".into()] }).expect("test alternative fits the fixed history ledger");
    let error = match semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await {
        Ok(_) => panic!("the alternative's pinned checkpoint id must actually exist"),
        Err(error) => error,
    };
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("alt-dangling") && message.contains("checkpoint-that-was-never-recorded")));
}

#[semio_framework_async_macros::async_test]
async fn create_alternative_with_no_edits_and_no_checkpoints_is_rejected() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let error = store.dispatch(ArtifactCommand::CreateAlternative { name: "x".into() }, &mut identity).await.unwrap_err();
    assert_eq!(error, VcsError::NoCheckpoint, "the auto-commit has nothing pending, so there is still no checkpoint to branch from");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn compensating_undo_without_a_semantic_command_is_rejected() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    let error = store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: None }, &mut identity).await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn materialize_document_snapshot_rejects_an_unknown_edit_id() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let error = materialize_document_snapshot(&envelope, &["missing-edit".to_string()]).await.unwrap_err();
    assert_eq!(error, VcsError::UnknownEdit("missing-edit".into()));
    retire_demo_envelope(envelope);
}

#[semio_framework_async_macros::async_test]
async fn dispatch_text_applies_a_command_block_and_snapshot_json_reflects_it() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let command_text = print_command(&ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }).await.expect("print command");
    store.dispatch_text(&command_text, &mut identity).await.expect("dispatch text");
    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());

    let error = store.dispatch_text("not a command", &mut identity).await.unwrap_err();
    assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn dispatch_binary_applies_an_encoded_command_and_rejects_wrong_format() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let command_bytes = ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }.encode_command().expect("encode command");
    store.dispatch_binary(&command_bytes, &mut identity).await.expect("dispatch binary");
    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());

    let mut wrong_format = command_bytes.clone();
    wrong_format[0] = 9;
    let error = store.dispatch_binary(&wrong_format, &mut identity).await.unwrap_err();
    assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn command_text_binary_equivalence_holds_for_every_document_command_variant() {
    let commands: Vec<ArtifactCommand<DemoMutation>> = vec![
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None },
        ArtifactCommand::Undo,
        ArtifactCommand::Redo,
        ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None },
        ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: None },
        ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: Some(Box::new(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 0 })], transaction: None })) },
        ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".to_string()), authors: vec![Author { id: "u1".to_string(), name: "Ueli Saluz".to_string(), avatar: None }] },
        ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() },
        ArtifactCommand::CreateAlternative { name: "branch".to_string() },
        ArtifactCommand::SwitchAlternative { alternative_id: "alt-1".to_string() },
        ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "ck-1".to_string() },
        ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], lane: HistoryLane::Interaction, transaction: None },
        ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], lane: HistoryLane::Document, transaction: None },
        ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction },
        ArtifactCommand::RedoInLane { lane: HistoryLane::Interaction },
    ];
    for command in &commands {
        test_support::assert_command_text_binary_equivalence::<DemoSnapshot, _>(command).await;
    }
}

//#endregion 🔖️CommandErrorPaths
