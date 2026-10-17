//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 5 of the cut by subject).

use super::*;

/// 🪪️ Runs one authoring call under a fixture caller-owned identity authority and returns the call's own result after the authority's cumulative receipt is settled.
macro_rules! with_fixture_identity {
    (|$identity:ident| $call:expr) => {{
        const FIXTURE_IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
        let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            assert!(progress.owned_bytes <= FIXTURE_IDENTITY_CEILING);
            true
        };
        let mut $identity = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(FIXTURE_IDENTITY_CEILING, &mut observer).expect("declared fixture identity authority");
        let result = $call;
        drop($identity.pause().expect("original cumulative fixture identity receipt"));
        result
    }};
}


/// 🧾️ Every committed wire witness of a store fixture aggregate decodes through the aggregate's `FromValue` and re-encodes to
/// exactly the committed JSON: the witness is the canonical Rust wire of that leaf.
#[test]
fn committed_wire_witnesses_are_the_canonical_wire() {
    use semio_framework_os_store_test_support::assert_wire_witness as assert_witness;
    assert_witness::<DemoMutation>(include_str!("../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<DemoMutation>(include_str!("../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🗑️delete-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<DemoMutation>(include_str!("../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/➕️add-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<DemoMutation>(include_str!("../../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<TimestampedMutation>(include_str!("../../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<TimestampedMutation>(include_str!("../../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/⚠️set-warning-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🚫️set-error-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🛑️set-fatal-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SeverityMutation>(include_str!("../../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<ValidatedMutation>(include_str!("../../../🧪️testing/🧬️mutations/🛂️validated/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<ValidatedMutation>(include_str!("../../../🧪️testing/🧬️mutations/🛂️validated/🧬️mutations/↩️assign-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<LossyMutation>(include_str!("../../../🧪️testing/🧬️mutations/🪤️lossy/🧬️mutations/🔢️set-n/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../🧫️fixtures/🧬️mutations/📌️commit-space-checkpoint/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../🧫️fixtures/🧬️mutations/🌿️create-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../🧫️fixtures/🧬️mutations/🔀️switch-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../🧫️fixtures/🧬️mutations/🗑️remove-space-checkpoint/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../🧫️fixtures/🧬️mutations/🧹️remove-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
    assert_witness::<SpaceHistoryMutation>(include_str!("../../🧫️fixtures/🧬️mutations/🎯️set-active-space-alternative/🧾️wire-witness/🦠️mutation/🔣️.json"));
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
    let malformed_spr = semio_framework_os_spr::encode_history(&history, &semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() }).await.expect("encode malformed history");
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

//#region 🔖️ReconcileAlternative
#[semio_framework_async_macros::async_test]
async fn reconcile_alternative_requires_an_existing_checkpoint() {
    const CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING,&mut observer).unwrap();
    let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let error = reconcile_alternative(&mut envelope, "reconciled", None, Vec::new(), &mut identity).await.unwrap_err();
    assert_eq!(error, VcsError::NoCheckpoint);
    retire_demo_envelope(envelope);
}

#[semio_framework_async_macros::async_test]
async fn reconcile_alternative_pins_the_latest_checkpoint_and_optionally_records_a_reconciliation_checkpoint() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }, &mut identity).await.expect("commit");
    let base_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();

    let mut without_message = owned_test_envelope(&store).await;
    let alt_id = reconcile_alternative(&mut without_message, "no-record", None, Vec::new(), &mut identity).await.expect("reconcile without message");
    assert_eq!(without_message.vcs.alternatives.last().unwrap().checkpoint_ids, vec![base_checkpoint_id.clone()]);
    assert_eq!(without_message.vcs.checkpoints.len(), 1, "no checkpoint_message means no new checkpoint is recorded");
    assert!(!alt_id.is_empty());

    let mut with_message = owned_test_envelope(&store).await;
    let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
    reconcile_alternative(&mut with_message, "recorded", Some("merged concurrent work".into()), authors.clone(), &mut identity).await.expect("reconcile with message");
    assert_eq!(with_message.vcs.checkpoints.len(), 2, "a checkpoint_message appends one reconciliation checkpoint");
    let recorded_checkpoint = with_message.vcs.checkpoints.last().unwrap();
    assert_eq!(recorded_checkpoint.parent_id, Some(base_checkpoint_id));
    assert_eq!(recorded_checkpoint.authors, authors);
    assert_eq!(recorded_checkpoint.message, Some("reconciled".into()), "the reconciliation checkpoint's own message is fixed, distinct from the change description");
    assert_eq!(with_message.vcs.changes.last().unwrap().description, Some("merged concurrent work".into()), "the passed checkpoint_message becomes the change's description");
    retire_demo_envelope(without_message);
    retire_demo_envelope(with_message);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn commit_checkpoint_mints_distinct_content_addressed_ids_for_distinct_commits() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply 1");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("first".into()), authors: Vec::new() }, &mut identity).await.expect("commit 1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply 2");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("second".into()), authors: Vec::new() }, &mut identity).await.expect("commit 2");

    let ids: Vec<&str> = store.envelope().vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()).collect();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1], "two distinct commits must mint two distinct checkpoint ids");
    assert!(ids.iter().all(|id| id.starts_with("ck-")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn merge_base_finds_the_nearest_common_ancestor_across_a_fork() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply root");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }, &mut identity).await.expect("commit root");
    let root_id = store.envelope().vcs.checkpoints[0].id.clone();

    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }, &mut identity).await.expect("create feature-a");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply a");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }, &mut identity).await.expect("commit a1");
    let a1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root_id.clone() }, &mut identity).await.expect("checkout root");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-b".into() }, &mut identity).await.expect("create feature-b");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply b");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }, &mut identity).await.expect("commit b1");
    let b1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

    assert_eq!(merge_base(store.envelope(), &a1_id, &b1_id).await, Some(root_id.clone()), "a1 and b1 forked at root");
    assert_eq!(merge_base(store.envelope(), &a1_id, &root_id).await, Some(root_id.clone()), "root is its own descendant's merge-base");
    assert_eq!(merge_base(store.envelope(), &root_id, &root_id).await, Some(root_id), "a checkpoint is its own merge-base");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn merge_base_is_none_for_a_dangling_unknown_checkpoint_id() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }, &mut identity).await.expect("commit");
    let root_id = store.envelope().vcs.checkpoints[0].id.clone();

    assert_eq!(merge_base(store.envelope(), &root_id, "unknown-checkpoint").await, None, "an id absent from the checkpoint list shares no ancestry with anything");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


//#region 🔖️EventLogCatchUp
#[semio_framework_async_macros::async_test]
async fn event_log_catch_up_adds_only_unknown_events_and_derives_their_records() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = fresh_demo_store().await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("local apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("local".into()), authors: Vec::new() }, &mut identity).await.expect("local commit");

    let mut remote_store = fresh_demo_store().await;
    for event in store.event_log().expect("local log") {
        remote_store.ingest_remote(event).await.expect("remote replica ingests the local log");
    }
    remote_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("remote apply");
    remote_store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("remote".into()), authors: Vec::new() }, &mut identity).await.expect("remote commit");

    let (channel, remote_end) = ChannelBackbone::pair("chan").await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let _ = drain_channel_for_test(&remote_end).expect("drain attach announcement");
    remote_end.push(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&remote_store.event_log().expect("remote log")) }).await.expect("push the whole remote log");
    store.tick().await.expect("tick folds the pushed events");

    assert_eq!(store.envelope().vcs.edits.len(), 2, "the shared original edit is deduplicated, only the new remote edit is added");
    assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "the remote's checkpoint is derived from its commit transition");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "current folds in the new remote edit");
    assert_eq!(store.current_checkpoint_id().await, remote_store.current_checkpoint_id().await);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧾️ Every observable position of a store: payload, applied order, redo stack, checkpoint and alternative.
async fn history_position(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> (DemoSnapshot, usize, usize, Option<String>, Option<String>) {
    (store.snapshot().expect("snapshot"), store.applied_edit_ids().len(), store.redo_edit_ids().len(), store.current_checkpoint_id().await.map(str::to_string), store.envelope().active_alternative_id.clone())
}

/// 🔀️ Undo, redo, checkpoint, alternative and checkout on one replica reach the other as events over a
/// live backbone — the receiver folds them into exactly the author's positions, step by step.
#[semio_framework_async_macros::async_test]
async fn structural_history_replicates_as_events_over_a_backbone() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut a = fresh_demo_store().await;
    let mut b = fresh_demo_store().await;
    let (near, far) = MemoryBackbone::pair("structural-a", "structural-b").await;
    a.attach_backbone(Backbones::Memory(near)).await.expect("attach a");
    b.attach_backbone(Backbones::Memory(far)).await.expect("attach b");
    let steps: Vec<ArtifactCommand<DemoMutation>> = vec![
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None },
        ArtifactCommand::Undo,
        ArtifactCommand::Redo,
        ArtifactCommand::Undo,
        ArtifactCommand::CommitCheckpoint { message: Some("one".into()), authors: vec![Author { id: "u1".into(), name: "Ada".into(), avatar: Some("a.png".into()) }] },
        ArtifactCommand::CreateAlternative { name: "variant".into() },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None },
        ArtifactCommand::CommitCheckpoint { message: Some("two".into()), authors: Vec::new() },
    ];
    let (shared, local) = steps.split_at(6);
    for command in shared {
        a.dispatch(command.clone(), &mut identity).await.expect("a authors the step");
        b.tick().await.expect("b folds the step's events");
        assert_eq!(history_position(&a).await, history_position(&b).await, "shared trunk steps land on both replicas");
    }
    for command in local {
        a.dispatch(command.clone(), &mut identity).await.expect("a authors on the new alternative");
        b.tick().await.expect("b folds the shared events and stays on the trunk");
    }
    assert_eq!(a.snapshot().expect("a"), DemoSnapshot { n: Some(9) });
    assert_eq!(b.snapshot().expect("b"), DemoSnapshot { n: Some(1) }, "b never left the trunk");
    assert!(a.envelope().active_alternative_id.is_some());
    assert!(b.envelope().active_alternative_id.is_none());
    let first = a.envelope().vcs.checkpoints[0].id.clone();
    let transitions = a.envelope().transitions.len();
    a.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: first }, &mut identity).await.expect("a checks out the first checkpoint locally");
    b.tick().await.expect("b has no checkout event to fold");
    assert_eq!(a.envelope().transitions.len(), transitions);
    assert_eq!(a.snapshot().expect("a"), DemoSnapshot { n: Some(1) });
    assert_eq!(b.snapshot().expect("b"), DemoSnapshot { n: Some(1) });
    assert_ne!(a.envelope().active_alternative_id, b.envelope().active_alternative_id);
    assert_eq!(a.envelope().vcs.alternatives, b.envelope().vcs.alternatives, "alternative chains are folded facts on both replicas");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🔀️ A checkout and a concurrent edit converge whichever replica receives the other's events first.
#[semio_framework_async_macros::async_test]
async fn a_concurrent_checkout_and_edit_converge_in_either_arrival_order() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut a = fresh_demo_store().await;
    a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("seed edit");
    a.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }, &mut identity).await.expect("seed checkpoint");
    a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("a's later edit");
    let mut b = fresh_demo_store().await;
    for event in a.event_log().expect("a's log") {
        b.ingest_remote(event).await.expect("b catches up");
    }
    let known: HashSet<MutationId> = a.event_log().expect("a's log").into_iter().map(|event| event.mutation_id).collect();
    let checkpoint_id = a.envelope().vcs.checkpoints[0].id.clone();
    let transitions = a.envelope().transitions.len();
    a.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.clone() }, &mut identity).await.expect("a checks out the checkpoint");
    assert_eq!(a.envelope().transitions.len(), transitions, "a's checkout authors nothing a peer could fold");
    b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }, &mut identity).await.expect("b's concurrent edit");
    let fresh_from = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| store.event_log().expect("log").into_iter().filter(|event| !known.contains(&event.mutation_id)).collect::<Vec<_>>();
    let from_b = fresh_from(&b);
    assert!(fresh_from(&a).is_empty());
    for event in from_b {
        a.ingest_remote(event).await.expect("a folds b's edit");
    }
    assert_eq!(a.snapshot().expect("a"), DemoSnapshot { n: Some(1) }, "a's explicit checkout hides uncommitted edits");
    assert_eq!(b.snapshot().expect("b"), DemoSnapshot { n: Some(7) });
    b.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id }, &mut identity).await.expect("b looks at the same checkpoint");
    assert_eq!(history_position(&a).await, history_position(&b).await, "equal heads over the same events agree");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 💾️ A reload replays the persisted events into the exact positions — through `.pack`+`.spr` and
/// through the `.ops` text mirror alike; no cursor is persisted.
#[semio_framework_async_macros::async_test]
async fn reload_replays_the_event_log_into_the_same_positions() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = fresh_demo_store().await;
    for command in [
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None },
        ArtifactCommand::CommitCheckpoint { message: Some("base".into()), authors: Vec::new() },
        ArtifactCommand::CreateAlternative { name: "variant".into() },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 8 })], transaction: None },
        ArtifactCommand::Undo,
    ] {
        store.dispatch(command, &mut identity).await.expect("history step");
    }
    let expected = history_position(&store).await;
    let files = print_document_pack(store.envelope()).await.expect("print pack");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse pack").envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(history_position(&reloaded).await, expected, ".pack+.spr reload restores every position by folding events");
    let text = print_document_text(store.envelope()).await.expect("print text");
    assert!(text.ops.contains("\ncommit ") && text.ops.contains("\nbranch ") && text.ops.contains("\nrevert "), "the text mirror states the semantic transitions: {}", text.ops);
    let from_text = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse text").envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(from_text.snapshot().expect("text snapshot"), expected.0, "the ops text folds the shared events");
    assert!(from_text.envelope().active_alternative_id.is_none(), "the ops text has no viewer head and hydrates to the trunk");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#endregion 🔖️EventLogCatchUp

//#region 🔖️SpaceMemberCheckoutRouting
#[semio_framework_async_macros::async_test]
async fn space_member_checkout_switches_at_the_alternative_tip_and_falls_back_to_checkout_when_stale() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature".into() }, &mut identity).await.expect("create alternative (auto-commits since no checkpoint existed yet)");
    let alt_id = store.envelope().vcs.alternatives.last().expect("the branched alternative").id.clone();
    let tip = store.envelope().vcs.alternatives.last().expect("the branched alternative").checkpoint_ids.last().expect("alt has a tip").clone();

    SpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout at the tip routes through SwitchAlternative");
    assert_eq!(store.envelope().active_alternative_id, Some(alt_id.clone()), "switching to the tip keeps it active");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply on branch");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }, &mut identity).await.expect("commit c2, advancing the alt's tip past `tip`");

    SpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "restored the old checkpoint's state");
    assert_eq!(store.envelope().active_alternative_id.as_deref(), Some(alt_id.as_str()), "the checkpoint is still on this alternative, so the line stays");
    assert_eq!(store.envelope().viewer_checkpoint_id.as_deref(), Some(tip.as_str()));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#endregion 🔖️SpaceMemberCheckoutRouting

//#region 🔖️BackbonePorts
#[semio_framework_async_macros::async_test]
async fn memory_backbone_port_round_trips_and_reports_a_missing_file() {
    let port = MemoryBackbonePort::new().await;
    let error = port.read("file://nowhere").await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    port.write("file://a", "payload-1").await.expect("write");
    assert_eq!(port.read("file://a").await.expect("read"), "payload-1");
    port.write("file://a", "payload-2").await.expect("overwrite");
    assert_eq!(port.read("file://a").await.expect("read after overwrite"), "payload-2", "write is an upsert");
}

#[semio_framework_async_macros::async_test]
async fn local_storage_backbone_port_falls_back_to_its_in_memory_store() {
    let port = LocalStorageBackbonePort::new().await;
    let error = port.read("local://missing").await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    port.write("local://a", "value").await.expect("write falls back to the in-memory store");
    assert_eq!(port.read("local://a").await.expect("read falls back too"), "value");

    let defaulted = LocalStorageBackbonePort::default();
    assert!(defaulted.read("local://a").await.is_err(), "Default constructs its own independent fallback store");
}

//#endregion 🔖️BackbonePorts

//#region 🔖️PackValueFixtures
async fn pack_value_fixture_corpus() -> Vec<(&'static str, DslValue)> {
    vec![
        ("null", DslValue::Null),
        ("bool_true", DslValue::Bool(true)),
        ("bool_false", DslValue::Bool(false)),
        ("int_zero", DslValue::uint(0)),
        ("int_negative_one", DslValue::int(-1)),
        ("float_pi", DslValue::float(3.14)),
        ("float_whole_number", DslValue::float(2.0)),
        ("string_empty", DslValue::String(String::new())),
        ("string_escapes", DslValue::String("hello\nworld with \"quotes\"".into())),
        ("array_empty", DslValue::Array(vec![])),
        ("array_ints", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2), DslValue::uint(3)])),
        ("object_empty", DslValue::Object(vec![])),
        ("object_mixed", DslValue::object([("a".into(), DslValue::uint(1)), ("b".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null]))])),
        (
            "nested_deep",
            DslValue::object([("a".into(), DslValue::object([("b".into(), DslValue::object([("c".into(), DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2), DslValue::object([("d".into(), DslValue::String("leaf".into()))])]))]))]))]),
        ),
    ]
}

/// 🎯️ Variant-exact `DslValue` equality. `Number`'s own `PartialEq` calls a fitting
/// `UInt`/`Int` pair equal, so it cannot witness that the wire preserved the WRITER's variant —
/// which is exactly what `TAG_UINT` versus `TAG_INT` decides, and what the canonical bytes and
/// every hash over them depend on. `Float` compares by bits so `-0.0` and `NaN` are exact too.
// 🚫️async: E1 pure recursive comparison consumed inside sync `.all()`/`.is_some_and()` closures
// that cannot themselves be async — see R9/R10 residue shape 1
fn dsl_value_variant_exact_eq(a: &DslValue, b: &DslValue) -> bool {
    match (a, b) {
        (DslValue::Number(semio_framework_value::Number::UInt(x)), DslValue::Number(semio_framework_value::Number::UInt(y))) => x == y,
        (DslValue::Number(semio_framework_value::Number::Int(x)), DslValue::Number(semio_framework_value::Number::Int(y))) => x == y,
        (DslValue::Number(semio_framework_value::Number::Float(x)), DslValue::Number(semio_framework_value::Number::Float(y))) => x.to_bits() == y.to_bits(),
        (DslValue::Number(_), DslValue::Number(_)) => false,
        (DslValue::Array(x), DslValue::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| dsl_value_variant_exact_eq(a, b)),
        (DslValue::Object(x), DslValue::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.iter().find(|(ok, _)| ok == k).is_some_and(|(_, ov)| dsl_value_variant_exact_eq(v, ov))),
        _ => a == b,
    }
}

/// 🔣️ Reads one `semio.pack.dynamic-integer/v1` value node — integers arrive as exact
/// decimal STRINGS and floats as little-endian hex, so no fixture number ever passes through a
/// JSON `f64` on its way into the corpus.
// 🚫️async: E1 pure recursive fixture reader consumed inside sync iterator closures — see R9
fn dynamic_integer_fixture_value(node: &serde_json::Value) -> DslValue {
    if let Some(text) = node.get("uint").and_then(|v| v.as_str()) {
        return DslValue::uint(text.parse::<u64>().expect("corpus uint is an exact u64"));
    }
    if let Some(text) = node.get("int").and_then(|v| v.as_str()) {
        return DslValue::int(text.parse::<i64>().expect("corpus int is an exact i64"));
    }
    if let Some(text) = node.get("f64LeHex").and_then(|v| v.as_str()) {
        let mut bits = [0u8; 8];
        for (index, byte) in bits.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("corpus f64LeHex byte");
        }
        return DslValue::Number(semio_framework_value::Number::Float(f64::from_le_bytes(bits)));
    }
    if let Some(items) = node.get("list").and_then(|v| v.as_array()) {
        return DslValue::Array(items.iter().map(dynamic_integer_fixture_value).collect());
    }
    if let Some(entries) = node.get("map").and_then(|v| v.as_array()) {
        return DslValue::Object(entries.iter().map(|entry| (entry[0].as_str().expect("corpus map key").to_string(), dynamic_integer_fixture_value(&entry[1]))).collect());
    }
    panic!("unsupported corpus value node {node}");
}

// 🚫️async: E1 pure hex reader consumed inside sync iterator closures — see R9
fn dynamic_integer_fixture_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len() / 2).map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("corpus wireHex byte")).collect()
}

/// 🧾️ Hex-dumps `pack_rt::encode_pack_value` over a representative `DslValue`
/// corpus — ground truth for `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s TS
/// `PackValueCodec` mirror (`framework/product/os/ts/index.ts`). Run with `--nocapture` to
/// capture the printed `name -> hex` lines; also asserts `decode_pack_value(encode_pack_value(v))
/// == v` for every entry so the corpus is never accidentally out of date with the real codec.
#[semio_framework_async_macros::async_test]
async fn pack_value_fixture_corpus_hex_dump() {
    for (name, value) in pack_value_fixture_corpus().await {
        let bytes = pack_rt::encode_pack_value(&value);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("[pack_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
        let decoded = pack_rt::decode_pack_value(&bytes).expect("decode_pack_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn interaction_state_pack_matches_first_party_value_and_json_oracle() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🎒️interaction-state-pack.json")).unwrap();
    let value = DslValue::from(&oracle);
    let state = <protocol::InteractionState as FromValue>::from_value(value.clone()).unwrap();
    let encoded = <protocol::InteractionState as ArtifactPack>::encode_pack(&state);
    assert_eq!(encoded, <DslValue as ArtifactPack>::encode_pack(&value));
    assert_eq!(encoded, pack_rt::encode_json_value(&oracle));
    let decoded = <protocol::InteractionState as ArtifactPack>::decode_pack(&encoded).unwrap();
    assert_eq!(decoded, state);
    assert_eq!(serde_json::Value::from(&<protocol::InteractionState as ToValue>::to_value(&decoded)), oracle);
}

/// 🪶️ Hex-dumps `pack_rt::encode_wire_value` over the SAME fixture corpus — ground
/// truth for the container-less wire codec mirror in TS.
#[semio_framework_async_macros::async_test]
async fn pack_wire_value_fixture_corpus_hex_dump() {
    for (name, value) in pack_value_fixture_corpus().await {
        let bytes = pack_rt::encode_wire_value(&value);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("[pack_wire_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
    }
}

/// 🔢️ Exact-variant law for the dynamic `Shape::Value` grammar over the shared,
/// language-neutral `semio.pack.dynamic-integer/v1` corpus
/// (`💻️os/🧫️fixtures/🎒️pack-dynamic-integer-v1`, whose `wireHex` is independently generated
/// by a third-party LEB128 oracle). Asserts the emitted TAG, the exact `Number` variant, and
/// byte equality — `DslValue` equality alone cannot witness any of the three, because
/// `Number::PartialEq` calls `UInt(7)` and `Int(7)` equal while their wire tags differ.
#[semio_framework_async_macros::async_test]
async fn pack_wire_value_preserves_integer_variants_at_u64_i64_boundaries() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json")).expect("dynamic integer corpus parses");
    assert_eq!(corpus["schema"].as_str(), Some("semio.pack.dynamic-integer/v1"));
    assert_eq!(corpus["version"].as_u64(), Some(1));
    let accept = corpus["accept"].as_array().expect("accept rows");
    assert_eq!(accept.len(), 6, "the corpus is bounded at six accept rows");

    for row in accept {
        let id = row["id"].as_str().expect("row id");
        let value = dynamic_integer_fixture_value(&row["value"]);
        let expected = dynamic_integer_fixture_bytes(row["wireHex"].as_str().expect("row wireHex"));
        let bytes = pack_rt::encode_wire_value(&value);
        assert_eq!(bytes, expected, "{id}: encode_wire_value must be byte-exact against the neutral corpus");
        let tag = bytes[4];
        let expected_tag = match row["variant"].as_str() {
            Some("uint") => 0x04u8,
            Some("int") => 0x03,
            Some("float") => 0x05,
            Some("map") => 0x10,
            other => panic!("{id}: unsupported corpus variant {other:?}"),
        };
        assert_eq!(tag, expected_tag, "{id}: the dynamic value's leading tag carries the writer's variant");
        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "{id}: {decoded:?} is not variant-exact against {value:?}");
        assert_eq!(pack_rt::encode_wire_value(&decoded), expected, "{id}: re-encoding the decoded value is canonical and idempotent");
    }

    assert!(matches!(pack_rt::decode_wire_value(&dynamic_integer_fixture_bytes("000101110403")).expect("uint 3"), DslValue::Number(semio_framework_value::Number::UInt(3))));
    assert!(matches!(pack_rt::decode_wire_value(&pack_rt::encode_wire_value(&DslValue::int(7))).expect("int 7"), DslValue::Number(semio_framework_value::Number::Int(7))), "a positive Int keeps TAG_INT rather than collapsing onto TAG_UINT");
    assert_ne!(pack_rt::encode_wire_value(&DslValue::int(7)), pack_rt::encode_wire_value(&DslValue::uint(7)), "Int(7) and UInt(7) are equal under Number::PartialEq but are distinct on the wire");
    let beyond_exact_f64 = 9_007_199_254_740_993u64;
    assert_ne!(beyond_exact_f64 as f64 as u64, beyond_exact_f64, "the f64 widening this law replaced is not injective past 2^53");

    for row in corpus["reject"].as_array().expect("reject rows") {
        let id = row["id"].as_str().expect("row id");
        let bytes = dynamic_integer_fixture_bytes(row["wireHex"].as_str().expect("row wireHex"));
        match row["outcome"].as_str() {
            Some("decode-error") => {
                assert!(pack_rt::decode_wire_value(&bytes).is_err(), "{id}: a truncated or overlong varint must fail the reader");
            }
            Some("noncanonical") => {
                let decoded = pack_rt::decode_wire_value(&bytes).expect("the shared permissive varint reader admits redundant encodings");
                assert!(dsl_value_variant_exact_eq(&decoded, &dynamic_integer_fixture_value(&row["decodes"])), "{id}: {decoded:?} is not the documented decode");
                assert_ne!(pack_rt::encode_wire_value(&decoded), bytes, "{id}: a canonical-bytes caller rejects it because encode(decode(bytes)) != bytes");
            }
            other => panic!("{id}: unsupported corpus outcome {other:?}"),
        }
    }
}


#[semio_framework_async_macros::async_test]
async fn artifact_child_dsl_field_round_trips_via_pack_and_value() {
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.mesh".into(), standard: "87a".into(), subset: "mesh".into() } };
    let child: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target);

    let spec = artifact_child_spec();
    let record = artifact_child_to_record(&child);
    let bytes = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
    let (decoded_record, _report) = semio_framework_os_pack::record::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
    let decoded: ArtifactChild<DemoSnapshot> = artifact_child_from_record(&decoded_record).expect("from_record");
    assert_eq!(decoded, child);

    let value = <ArtifactChild<DemoSnapshot> as semio_framework_dsl_record::DslField>::to_value(&child);
    let via_field = <ArtifactChild<DemoSnapshot> as semio_framework_dsl_record::DslField>::from_value(&value).expect("from_value");
    assert_eq!(via_field, child);

    assert_eq!(child.to_child_ref("mesh-slot").await, ChildRef { slot: "mesh-slot".into(), child_id: "child-1".into(), target: child.target.clone() });
}

#[semio_framework_async_macros::async_test]
async fn artifact_child_required_owner_distinguishes_absence_and_wrong_type() {
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-1".into(), dialect: demo_child_dialect() };
    let absent: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target.clone());
    assert_eq!(absent.require_local_owner::<String>(), Err(ArtifactChildMaterializationError::Absent));

    let typed: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target).with_local_owner(Arc::new(String::from("content")));
    assert_eq!(typed.require_local_owner::<u64>(), Err(ArtifactChildMaterializationError::WrongType));
    assert_eq!(typed.require_local_owner::<String>().unwrap().as_str(), "content");
}

#[semio_framework_async_macros::async_test]
async fn owner_ref_dsl_field_round_trips_via_pack() {
    let owner = OwnerRef {
        parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.object".into(), standard: "1".into(), subset: "*".into() } },
        slot: "mesh-slot".into(),
        child_id: "child-1".into(),
    };
    let spec = owner_ref_spec();
    let record = owner_ref_to_record(&owner);
    let bytes = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
    let (decoded_record, _report) = semio_framework_os_pack::record::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
    let decoded = owner_ref_from_record(&decoded_record).expect("from_record");
    assert_eq!(decoded, owner);
}

#[semio_framework_async_macros::async_test]
async fn artifact_link_dsl_field_round_trips_every_link_pin_variant() {
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "linked-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
    let pins = vec![LinkPin::Head, LinkPin::Checkpoint { id: "ck-abc123".into() }, LinkPin::Snapshot { blob: BlobRef { hash: "deadbeef".into(), size: 42, media_type: "image/png".into() } }];
    for pin in pins {
        let link = ArtifactLink { target: target.clone(), pin: pin.clone(), role: "cover-image".into() };
        let semio_framework_dsl_record::Shape::Record(producer)=<ArtifactLink as semio_framework_dsl_record::DslField>::shape()else{unreachable!()};
        let spec=(producer.ordinary)();
        let semio_framework_dsl_record::FieldValue::Record(record)=<ArtifactLink as semio_framework_dsl_record::DslField>::to_value(&link)else{unreachable!()};
        let bytes = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
        let (decoded_record, _report) = semio_framework_os_pack::record::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
        let decoded=<ArtifactLink as semio_framework_dsl_record::DslField>::from_value(&semio_framework_dsl_record::FieldValue::Record(decoded_record)).expect("from_record");
        assert_eq!(decoded, link, "round trip diverged for pin variant {pin:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_refs_defaults_to_empty_for_a_leaf_snapshot() {
    struct LeafSnapshot;
    impl ArtifactRefs for LeafSnapshot {}
    let snapshot = LeafSnapshot;
    assert!(snapshot.child_refs().await.is_empty());
    assert!(snapshot.links().await.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn typed_child_store_factory_round_trips_a_child_through_create_persist_open() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let dialect = demo_child_dialect();

    let mut child = create_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", "child-round-trip", &dialect, &DemoSnapshot { n: Some(7) }.encode_pack(), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await.expect("create");
    assert!(child.owners.snapshot_retirement_factory.is_some(), "generated member creation must install the exact snapshot retirement factory before returning ownership");
    assert!(child.owned_disposer_installed(), "generated member creation must install the exact whole-store bounded disposer before returning ownership");
    child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }, &mut identity).await.expect("apply");
    child.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo");
    child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 11 })], transaction: None }, &mut identity).await.expect("re-apply");

    let files = print_document_pack(child.envelope()).await.expect("print");
    let persisted = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-round-trip".into(), dialect };
    let mut reopened = open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, None, &persisted, child.local_actor_id().clone(), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await.expect("open");

    assert!(reopened.owners.snapshot_retirement_factory.is_some(), "generated member reopen must install the exact snapshot retirement factory before returning ownership");
    assert!(reopened.owned_disposer_installed(), "generated member reopen must install the exact whole-store bounded disposer before returning ownership");
    assert_eq!(reopened.envelope().id, "child-round-trip");
    assert_eq!(reopened.snapshot().expect("head snapshot"), child.snapshot().expect("head snapshot"), "reopened child's live content diverged from the persisted one");
    assert_eq!(reopened.snapshot().expect("head snapshot"), DemoSnapshot { n: Some(11) }, "reopen restored the wrong cursor position");
    close_member_dialect_fixture(&mut reopened);
    close_member_dialect_fixture(&mut child);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🌱️ A derivable child's genesis envelope is minted from bytes alone and opens exactly like a
/// store-printed one: same id, schema, dialect and owner, the initial snapshot as live content, no
/// history — and a mismatched owner, like an empty initial pack, fails closed.
#[semio_framework_async_macros::async_test]
async fn genesis_member_envelope_pack_opens_as_the_owned_child_without_a_live_store() {
    let dialect = demo_child_dialect();
    let parent = semio_framework_artifact_reference::ArtifactRef { artifact_id: "genesis-parent".into(), dialect: dialect.clone() };
    let owner = OwnerRef { parent: parent.clone(), slot: "slot".into(), child_id: "genesis-child".into() };
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "genesis-child".into(), dialect: dialect.clone() };
    let envelope_pack = genesis_member_envelope_pack("demo/v1", &expected, &owner, &DemoSnapshot { n: Some(5) }.encode_pack()).await.expect("genesis envelope");
    let mut opened = open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, Some(&owner), &envelope_pack, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await.expect("open genesis member");
    assert_eq!(opened.envelope().id, "genesis-child");
    assert_eq!(opened.envelope().owner.as_ref(), Some(&owner));
    assert_eq!(opened.envelope().dialect.as_ref(), Some(&dialect));
    assert_eq!(opened.snapshot().expect("head snapshot"), DemoSnapshot { n: Some(5) });
    assert!(opened.envelope().vcs.edits.is_empty(), "a genesis member carries no history");
    close_member_dialect_fixture(&mut opened);
    let other = OwnerRef { parent, slot: "other".into(), child_id: "genesis-child".into() };
    assert!(matches!(open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, Some(&other), &envelope_pack, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await, Err(VcsError::Deserialize(_))), "the stamped owner must match the requested one");
    assert!(matches!(genesis_member_envelope_pack("demo/v1", &expected, &owner, &[]).await, Err(VcsError::Deserialize(_))), "an empty initial pack is rejected, never substituted");
}

#[semio_framework_async_macros::async_test]
async fn member_factory_wrapper_cannot_bypass_exact_create_or_open_owner_catalog() {
    type Wrapped = ArtifactStore<DemoSnapshot, DemoMutation>;
    let dialect = demo_child_dialect();
    let mut created = <Wrapped as MemberFactory>::create("wrapped-member", &dialect, &DemoSnapshot { n: Some(3) }.encode_pack(), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("wrapper create");
    assert!(created.snapshot_retirement_factory.is_some());
    assert!(created.initial_snapshot_retirement_factory.is_some());
    assert!(created.mutation_retirement_factory.is_some());
    assert!(created.owned_disposer_installed());
    let files = print_document_pack(created.envelope()).await.expect("print wrapped member");
    let persisted = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "wrapped-member".into(), dialect };
    let mut reopened = <Wrapped as MemberFactory>::open(&expected, None, &persisted, created.local_actor_id().clone()).await.expect("wrapper open");
    assert!(reopened.owners.snapshot_retirement_factory.is_some());
    assert!(reopened.owners.initial_snapshot_retirement_factory.is_some());
    assert!(reopened.owners.mutation_retirement_factory.is_some());
    assert!(reopened.owned_disposer_installed());
    close_member_dialect_fixture(&mut reopened);
    close_member_dialect_fixture(&mut created);
}

#[semio_framework_async_macros::async_test]
async fn member_close_rejects_missing_owner_and_preserves_the_installed_disposer_until_terminal() {
    let mut missing = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "missing-member-owner", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    let refused = SpaceMember::close_owned_step(&mut missing, physical_test_close_grant());
    assert!(matches!(&refused, Err(reason) if reason.kind == semio_framework_value::ValueRefusalKind::InvariantViolated && reason.message.contains("no installed owner disposer")), "{refused:?}");

    missing.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    missing.closes_on_drop();
    let first = SpaceMember::close_owned_step(&mut missing, physical_test_close_grant());
    assert!(matches!(&first, Ok(RetainedCloneStep::Progress(progress)) if progress.fits(physical_test_close_grant())), "{first:?}");
    assert!(missing.owned_disposer_installed(), "the admitted owner remains retained until terminal close");
    assert!(!SpaceMember::close_owned_terminal_is_empty(&missing));
    let replacement=complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()));
    let(error,mut returned)=missing.install_document_store_owners_exact(replacement).expect_err("the original catalog cannot be replaced");
    assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
    for _ in 0..10_000 { if returned.uninstalled_owners_terminal_is_empty(){break;}let step=returned.close_uninstalled_owners_step(physical_test_close_grant()).expect("returned original catalog closes under the supplied grant");assert!(step.progress().fits(physical_test_close_grant())); }
    assert!(returned.uninstalled_owners_terminal_is_empty());
    close_member_dialect_fixture(&mut missing);
}

/// 🏠️ owner ⇒ dialect: an envelope that is somebody's child but names no dialect cannot be
/// typed by its parent, so `open` must fail closed rather than hand back an untypable member.
#[semio_framework_async_macros::async_test]
async fn typed_child_store_factory_rejects_empty_genesis_and_dialect_less_owned_child() {
    assert!(matches!(create_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", "child-empty", &demo_child_dialect(), &[], ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await, Err(VcsError::Deserialize(_))), "an empty genesis pack must never silently default");

    let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "child-no-dialect", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.owner = Some(OwnerRef { parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent".into(), dialect: demo_child_dialect() }, slot: "mesh".into(), child_id: "child-no-dialect".into() });
    let files = print_document_pack(&envelope).await.expect("print");
    let orphan = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-no-dialect".into(), dialect: demo_child_dialect() };
    assert!(matches!(open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, envelope.owner.as_ref(), &orphan, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await, Err(VcsError::Deserialize(_))), "an owned child with no dialect must fail closed");
    retire_demo_envelope(envelope);
}

#[semio_framework_async_macros::async_test]
async fn pack_at_checkpoint_reads_history_without_moving_the_live_cursor() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "pinned-target", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply");
    let checkpoint = SpaceMember::commit_checkpoint(&mut store, "v1".into(), Vec::new(), &mut identity).await.expect("checkpoint");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply after checkpoint");

    let historical = DemoSnapshot::decode_pack(&store.pack_at_checkpoint(&checkpoint).await.expect("pack at checkpoint")).expect("decode");
    let live = DemoSnapshot::decode_pack(&store.document_pack_bytes().await.expect("head pack")).expect("decode");
    assert_eq!(historical, DemoSnapshot { n: Some(2) }, "checkpoint read did not return the pinned content");
    assert_eq!(live, DemoSnapshot { n: Some(3) }, "reading a checkpoint moved the live cursor");
    assert!(matches!(store.pack_at_checkpoint("no-such-checkpoint").await, Err(VcsError::UnknownChange(_))));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn member_link_resolver_resolves_head_checkpoint_and_degrades_snapshot_pins() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    struct FixtureDirectory {
        member: ArtifactStore<DemoSnapshot, DemoMutation>,
    }
    impl MemberDirectory for FixtureDirectory {
        async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
            if artifact_id == "linked-doc" { Some(self.member.document_pack_bytes().await) } else { None }
        }
        async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
            if artifact_id == "linked-doc" { Some(self.member.pack_at_checkpoint(checkpoint_id).await) } else { None }
        }
    }

    let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "linked-doc", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 42 })], transaction: None }, &mut identity).await.expect("apply");
    let pinned = SpaceMember::commit_checkpoint(&mut member, "pinned".into(), Vec::new(), &mut identity).await.expect("checkpoint");
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 99 })], transaction: None }, &mut identity).await.expect("apply after pin");

    let resolver = MemberLinkResolver::new(FixtureDirectory { member }).await;
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "linked-doc".into(), dialect: demo_child_dialect() };
    let link_of = |pin: LinkPin| ArtifactLink { target: target.clone(), pin, role: "representation".into() };
    let decode = async |state: LinkState| match state {
        LinkState::Resolved { pack_bytes, .. } => DemoSnapshot::decode_pack(&pack_bytes).expect("decode"),
        other => panic!("expected Resolved, found {other:?}"),
    };

    assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Head)).await).await, DemoSnapshot { n: Some(99) }, "a Head pin must follow the target's live tip");
    assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Checkpoint { id: pinned })).await).await, DemoSnapshot { n: Some(42) }, "a Checkpoint pin must keep resolving to the pinned history, not the tip");

    let blob = BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() };
    assert!(matches!(resolver.resolve(&link_of(LinkPin::Snapshot { blob })).await, LinkState::PinnedOnly { .. }), "a snapshot pin with no blob store must degrade to PinnedOnly, never Missing");

    let absent = ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef { artifact_id: "gone".into(), dialect: demo_child_dialect() }, pin: LinkPin::Head, role: "representation".into() };
    assert_eq!(resolver.resolve(&absent).await, LinkState::Missing);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn link_resolver_reports_resolved_missing_and_pinned_only_states() {
    struct DemoResolver;
    impl LinkResolver for DemoResolver {
        async fn resolve(&self, link: &ArtifactLink) -> LinkState {
            match link.role.as_str() {
                "known" => LinkState::Resolved { pack_bytes: vec![1, 2, 3], dialect: link.target.dialect.clone() },
                "pinned" => LinkState::PinnedOnly { blob: BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() } },
                _ => LinkState::Missing,
            }
        }
    }
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "linked-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
    let resolver = DemoResolver;
    assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "known".into() }).await, LinkState::Resolved { .. }));
    assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "pinned".into() }).await, LinkState::PinnedOnly { .. }));
    assert!(matches!(resolver.resolve(&ArtifactLink { target, pin: LinkPin::Head, role: "gone".into() }).await, LinkState::Missing));
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_parent_projection_matches_neutral_corpus() {
    use semio_framework_schema_composition::{ArtifactCompositionFields, ChildRefFields, ChildRefVisitor, ChildSlotSpec};
    struct Parent<'a>(&'a [serde_json::Value]);
    fn fields(row: &serde_json::Value) -> ChildRefFields<'_> {
        ChildRefFields {
            child_id: row["childId"].as_str().unwrap(),
            artifact_id: row["artifactId"].as_str().unwrap(),
            artifact_kind: row["artifactKind"].as_str().unwrap(),
            standard: row["standard"].as_str().unwrap(),
            subset: row["subset"].as_str().unwrap(),
        }
    }
    impl ArtifactCompositionFields for Parent<'_> {
        fn child_slots() -> &'static [ChildSlotSpec] {
            &[ChildSlotSpec { name: "single", selection: semio_framework_schema_composition::ChildSlotSelection::ArtifactKind("s.test.member"), many: false }, ChildSlotSpec { name: "many", selection: semio_framework_schema_composition::ChildSlotSelection::ArtifactKind("s.test.member"), many: true }, ChildSlotSpec { name: "other", selection: semio_framework_schema_composition::ChildSlotSelection::ArtifactKind("s.test.member"), many: false }]
        }
        fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
            for row in self.0 {
                visitor.step()?;
                let slot = match row["slot"].as_str().unwrap() {
                    "single" => "single",
                    "many" => "many",
                    "other" => "other",
                    _ => "unknown",
                };
                visitor.child(slot, fields(row))?;
            }
            Ok(())
        }
    }
    let fixture = member_dialect_fixture();
    let slots = Parent::child_slots();
    for (actual, expected) in slots.iter().zip(fixture["projectionSlots"].as_array().unwrap()) {
        assert_eq!(actual.name, expected["name"].as_str().unwrap());
        let semio_framework_schema_composition::ChildSlotSelection::ArtifactKind(kind)=actual.selection else{panic!("original projection fixture requires artifact kind selection")};
        assert_eq!(kind,expected["kind"].as_str().unwrap());
        assert_eq!(actual.many, expected["many"].as_bool().unwrap());
    }
    let rows = fixture["projectionCases"].as_array().unwrap();
    assert_eq!(rows.len(), 16);
    for row in rows {
        let parent = Parent(row["parent"].as_array().unwrap());
        let projection = ChildRestoreProjection::from_snapshot(&parent);
        assert_eq!(projection.is_ok(), row["projected"].as_bool().unwrap(), "{}", row["id"]);
        let admitted = projection.as_ref().is_ok_and(|projection| projection.admit_complete(row["incoming"].as_array().unwrap().iter().map(|row| (row["slot"].as_str().unwrap(), fields(row)))).is_ok());
        assert_eq!(admitted, row["accepted"].as_bool().unwrap(), "{}", row["id"]);
    }
}

/// 🌲️ Original forest and linked owners require exact physical closure under independent common grants.
#[semio_framework_async_macros::async_test]
async fn composition_graph_retires_with_exact_independent_frontier_grants() {
    let (mut graph, original_bytes, released) = semio_framework_trace_memory_testing::observe_backing(|| {
        let mut graph = CompositionGraph::default();
        graph.owns.insert("s.flow.flow@1/child".into(), ("s.flow.flow@1/parent".into(), "slot".into()));
        graph.links.insert("s.flow.flow@1/parent".into(), ["s.flow.flow@1/linked".into()].into_iter().collect());
        graph
    });
    assert_eq!(released, 0);
    let mut released_bytes = 0;
    for _ in 0..10000 {
        let grant = physical_test_close_grant();
        assert!(graph.next_close_copy_byte_demand().expect("linked graph requires genuine controlled owner") <= grant.maximum_copy_bytes);
        assert!(graph.next_close_capacity_byte_demand(grant.maximum_copy_bytes).expect("original graph capacity") <= grant.maximum_capacity_bytes);
        assert!(graph.next_close_release_byte_demand().expect("original graph release") <= grant.maximum_release_bytes);
        assert!(graph.next_close_depth_demand().expect("original graph depth") <= grant.maximum_depth);
        assert!(grant.maximum_copy_bytes <= 4096);
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| graph.close_step(grant));
        let step = step.expect("every original graph owner closes without fallback");
        assert_eq!(allocated, 0); assert!(step.progress().fits(grant)); assert_eq!(released, step.progress().released_bytes);
        released_bytes += released;
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(graph.terminal_is_empty()); assert_eq!(released_bytes, original_bytes);
    let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(graph)); assert_eq!((allocated, released), (0, 0));
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_graph_admission_matches_neutral_corpus() {
    let fixture = member_dialect_fixture();
    let rows = fixture["graphCases"].as_array().expect("neutral graph rows");
    assert_eq!(rows.len(), 7);
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let mut graph = CompositionGraph::new().await;
        for edge in row["edges"].as_array().unwrap() {
            graph.insert_owns(edge["parent"].as_str().unwrap(), edge["slot"].as_str().unwrap(), edge["child"].as_str().unwrap()).await.expect("valid initial forest");
        }
        let previous = graph.owns.clone();
        let candidate = &row["candidate"];
        let parent = candidate["parent"].as_str().unwrap();
        let slot = candidate["slot"].as_str().unwrap();
        let child = candidate["child"].as_str().unwrap();
        let admission = graph.admit_owns(parent, slot, child).await;
        assert_eq!(admission.is_ok(), row["accepted"].as_bool().unwrap(), "{id}");
        assert_eq!(graph.owns, previous, "{id}: admission is read-only");
        if let Ok(admission) = admission {
            assert_eq!(admission.inserts(), row["inserted"].as_bool().unwrap(), "{id}");
            graph.commit_owns_admitted(admission);
            assert_eq!(graph.owner_of(child).await, Some(parent), "{id}");
            assert_eq!(graph.slot_of(child).await, Some(slot), "{id}");
            assert!(!graph.admit_owns(parent, slot, child).await.unwrap().inserts(), "{id}: exact repeat cannot erase or replace an existing edge");
        }
    }
    let graph = CompositionGraph::new().await;
    let mut foreign = CompositionGraph::new().await;
    let ticket = graph.admit_owns("parent", "slot", "child").await.unwrap();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| foreign.commit_owns_admitted(ticket))).is_err());
    assert!(foreign.owns.is_empty());
    let mut graph = CompositionGraph::new().await;
    let ticket = graph.admit_owns("parent", "slot", "child").await.unwrap();
    graph.insert_owns("child", "ancestor", "parent").await.unwrap();
    let previous = graph.owns.clone();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| graph.commit_owns_admitted(ticket))).is_err());
    assert_eq!(graph.owns, previous);
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection() {
    struct Refs {
        children: Vec<ChildRef>,
        links: Vec<ArtifactLink>,
    }
    impl ArtifactRefs for Refs {
        async fn child_refs(&self) -> Vec<ChildRef> {
            self.children.clone()
        }
        async fn links(&self) -> Vec<ArtifactLink> {
            self.links.clone()
        }
    }
    let child_ref = |slot: &str, child: &str| ChildRef { slot: slot.into(), child_id: child.into(), target: semio_framework_artifact_reference::ArtifactRef { artifact_id: child.into(), dialect: demo_child_dialect() } };
    let link_ref = |target: &str| ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef { artifact_id: target.into(), dialect: demo_child_dialect() }, pin: LinkPin::Head, role: "reference".into() };
    let fixture = member_dialect_fixture();
    let rows = fixture["graphCases"].as_array().unwrap();
    assert_eq!(rows.len(), 7);
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let parent = row["candidate"]["parent"].as_str().unwrap();
        let child = row["candidate"]["child"].as_str().unwrap();
        let slot = row["candidate"]["slot"].as_str().unwrap();
        let mut graph = CompositionGraph::new().await;
        for edge in row["edges"].as_array().unwrap() {
            graph.insert_owns(edge["parent"].as_str().unwrap(), edge["slot"].as_str().unwrap(), edge["child"].as_str().unwrap()).await.unwrap();
        }
        graph.insert_owns(parent, "previous", "retained-prior-child").await.unwrap();
        graph.insert_link(parent, "retained-prior-link").await.unwrap();
        let prior_owns = graph.owns.clone();
        let prior_links = graph.links.clone();
        let prior_generation = graph.owns_generation;
        let refs = Refs { children: vec![child_ref(slot, child)], links: Vec::new() };
        let result = graph.sync_member(parent, &refs).await;
        assert_eq!(result.is_ok(), row["syncAccepted"].as_bool().unwrap(), "{id}");
        if result.is_ok() {
            let mut expected = prior_owns;
            expected.retain(|_, (owner, _)| owner != parent);
            expected.insert(child.into(), (parent.into(), slot.into()));
            assert_eq!(graph.owns, expected, "{id}");
            assert!(graph.links.get(parent).is_none());
        } else {
            assert_eq!(graph.owns, prior_owns, "{id}");
            assert_eq!(graph.links, prior_links, "{id}");
            assert_eq!(graph.owns_generation, prior_generation, "{id}");
        }
    }
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("parent", "previous", "previous-child").await.unwrap();
    graph.insert_link("cycle-source", "parent").await.unwrap();
    graph.insert_link("parent", "previous-link").await.unwrap();
    let prior_owns = graph.owns.clone();
    let prior_links = graph.links.clone();
    for refs in [Refs { children: vec![child_ref("next", "next-child")], links: vec![link_ref("cycle-source")] }, Refs { children: vec![child_ref("first", "duplicated"), child_ref("second", "duplicated")], links: Vec::new() }] {
        assert!(graph.sync_member("parent", &refs).await.is_err());
        assert_eq!(graph.owns, prior_owns);
        assert_eq!(graph.links, prior_links);
    }
}

#[semio_framework_async_macros::async_test]
async fn composition_graph_owns_forest_rejects_second_owner_and_cycle() {
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("parent-a", "slot-1", "child-1").await.expect("first owner ok");
    graph.insert_owns("parent-a", "slot-1", "child-1").await.expect("idempotent re-own ok");

    let error = graph.insert_owns("parent-b", "slot-1", "child-1").await.expect_err("second owner must be rejected");
    assert!(error.contains("already owned"), "unexpected message: {error}");

    graph.insert_owns("child-1", "slot-x", "grandchild-1").await.expect("child owns grandchild ok");
    assert!(graph.would_cycle_owns("grandchild-1", "parent-a").await, "parent-a is an ancestor of grandchild-1 via child-1");
    let cycle_error = graph.insert_owns("grandchild-1", "slot-y", "parent-a").await.expect_err("cycle must be rejected");
    assert!(cycle_error.contains("cycle"), "unexpected message: {cycle_error}");

    assert_eq!(graph.owner_of("child-1").await, Some("parent-a"));
    assert_eq!(graph.slot_of("child-1").await, Some("slot-1"));
    graph.remove_owns("child-1").await;
    assert_eq!(graph.owner_of("child-1").await, None);
}

#[semio_framework_async_macros::async_test]
async fn composition_graph_links_reject_cycle_but_allow_converging_dag_edges() {
    let mut graph = CompositionGraph::new().await;
    graph.insert_link("a", "b").await.expect("a->b ok");
    graph.insert_link("b", "c").await.expect("b->c ok");
    graph.insert_link("a", "c").await.expect("a->c ok, a converging (non-cyclic) edge");

    let error = graph.insert_link("c", "a").await.expect_err("closing the loop must be rejected");
    assert!(error.contains("cycle"), "unexpected message: {error}");

    assert_eq!(HashSet::<String>::from_iter(graph.links_from("a").await), HashSet::from(["b".to_string(), "c".to_string()]));
    graph.remove_link("a", "b").await;
    assert_eq!(graph.links_from("a").await, vec!["c".to_string()]);
}

/// 🧬️ Fingerprints one operation list under a fixture caller-owned identity authority.
fn fixture_ops_fingerprint(ops: &[Vec<u8>]) -> Vec<u8> {
    with_fixture_identity!(|identity| identity.encode(|control| operation_list_fingerprint(ops, control))).expect("fixture operation fingerprint")
}

/// 🧬️ Mints one child id under a fixture caller-owned identity authority.
fn fixture_child_id(parent_id: &str, slot: &str, fingerprint: &[u8], ordinal: u32) -> String {
    with_fixture_identity!(|identity| identity.encode(|control| mint_child_id(parent_id, slot, fingerprint, ordinal, control))).expect("fixture child id")
}

#[semio_framework_async_macros::async_test]
async fn mint_child_id_converges_across_two_replicas_and_varies_by_ordinal_and_slot() {
    const CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= CEILING); true };
    let mut control = semio_framework_value::NativeEncodeControl::new(CEILING, &mut observer);
    let parent_id = "parent-1";
    let slot = "mesh-slot";
    let ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 7 }).encode_op().expect("encode")];
    let fingerprint_replica_1 = semio_framework_os_vcs::io::binary::entity_identity::operation_list_fingerprint(&ops, &mut control).expect("original operation fingerprint caller");
    let fingerprint_replica_2 = semio_framework_os_vcs::io::binary::entity_identity::operation_list_fingerprint(&ops, &mut control).expect("original operation fingerprint caller");
    assert_eq!(fingerprint_replica_1, fingerprint_replica_2, "identical ops must fingerprint identically");

    let id_replica_1 = mint_child_id(parent_id, slot, &fingerprint_replica_1, 0, &mut control).expect("original child identity caller");
    let id_replica_2 = mint_child_id(parent_id, slot, &fingerprint_replica_2, 0, &mut control).expect("original child identity caller");
    assert_eq!(id_replica_1, id_replica_2, "two replicas performing the identical genesis must converge on the identical child id");

    let id_different_ordinal = mint_child_id(parent_id, slot, &fingerprint_replica_1, 1, &mut control).expect("original child identity caller");
    assert_ne!(id_replica_1, id_different_ordinal, "a different ordinal must mint a different id");

    let id_different_slot = mint_child_id(parent_id, "other-slot", &fingerprint_replica_1, 0, &mut control).expect("original child identity caller");
    assert_ne!(id_replica_1, id_different_slot, "a different slot must mint a different id");
    assert!(control.owned_bytes() <= CEILING);
}

/// 🧪️ One fatal preview anywhere ⇒ nothing applied on ANY member, parent included.
///
/// 🎞️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6: an
/// ordinary mutation-level (message-based) rejection now travels as `VcsError::Rejected`,
/// not `ValidationFailed` (reserved for structural failures only — see its own doc
/// comment). The default policy (`Normal`, via `SpaceMember::merge_policy`'s trait
/// default, since neither member's policy is configured here) rejects Fatal.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-atomic-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-atomic-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let good_op = ValidatedMutation::SetN(ValidatedSetN { n: 5 }).encode_op().expect("encode good op");
    let bad_op = ValidatedMutation::SetN(ValidatedSetN { n: -1 }).encode_op().expect("encode bad op");
    let parent_ops = vec![good_op];
    let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: &[bad_op], op_schema: &SchemaId("demo/v1".into()), labels: &[semio_framework_ui_locale::LocalizedLabel::data("bad")] };
    let mut children = [(&mut child_store, child_dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected the group dispatch to fail phase-1 validation, but it succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, semio_framework_os_spr::MergePolicy::Normal);
            assert!(messages.iter().any(|message| message.level == semio_framework_diagnostic::Severity::Fatal));
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a failed group dispatch");
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a failed group dispatch");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's ownership-check law: `dispatch_group` refuses to touch a `ChildDispatch`
/// whose claimed parent the coordinator's own `CompositionGraph` does not currently track —
/// zero side effects, same as any other phase-1 failure.
///
/// Deliberately NOT seeding `coordinator.graph_mut().insert_owns(..)` — the graph has no
/// record that `parent_ref` owns `child_ref`.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_rejects_a_child_the_graph_does_not_track_as_owned() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-unowned-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-unowned-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator = CompositionCoordinator::new();

    let op = ValidatedMutation::SetN(ValidatedSetN { n: 1 }).encode_op().expect("encode");
    let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: &[op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, child_dispatch)];

    let mut coordinator = coordinator.await;
    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected an OwnershipViolation, but the dispatch succeeded"),
        Err(VcsError::OwnershipViolation(_)) => {}
        Err(other) => panic!("expected OwnershipViolation, got a different VcsError: {other}"),
    }
    assert!(child_store.envelope().vcs.edits.is_empty());
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ Directly exercises `CompositionCoordinator::compensate` (private, visible to this
/// nested test module) — the reverse-order rollback `dispatch_group`'s phase 2 falls back to on
/// a late failure. Proves the order (parent first, then children in reverse dispatch order) and
/// that a clean rollback restores every member's pre-group snapshot.
#[semio_framework_async_macros::async_test]
async fn compensate_undoes_applied_members_in_reverse_order() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-comp-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_a_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-comp-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let child_b_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-comp-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply parent");
    let parent_edit_id = parent_store.envelope().vcs.edits.last().expect("parent edit").id.clone();

    let mut child_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply a");
    let child_a_edit_id = child_a.envelope().vcs.edits.last().expect("a edit").id.clone();

    let mut child_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply b");
    let child_b_edit_id = child_b.envelope().vcs.edits.last().expect("b edit").id.clone();

    let dispatch_a = ChildDispatch { child: child_a_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let dispatch_b = ChildDispatch { child: child_b_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_a, dispatch_a), (&mut child_b, dispatch_b)];
    let applied_children = vec![(0usize, child_a_edit_id), (1usize, child_b_edit_id)];

    let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some(&parent_edit_id), &mut identity).await;

    assert!(report.skipped.is_empty(), "every member should have undone cleanly");
    assert_eq!(report.undone.len(), 3);
    assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id, "parent undone first");
    assert_eq!(report.undone[1].0.artifact_id, child_b_ref.artifact_id, "then the LAST-dispatched child");
    assert_eq!(report.undone[2].0.artifact_id, child_a_ref.artifact_id, "then the first-dispatched child");

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(0), "parent's edit was undone");
    assert_eq!(child_a.snapshot().expect("a snapshot").n, Some(0), "child a's edit was undone");
    assert_eq!(child_b.snapshot().expect("b snapshot").n, Some(0), "child b's edit was undone");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's "if compensation itself fails" law: a member whose own rollback errors is
/// recorded in `GroupUndoReport.skipped` (never panics, never aborts compensating the rest),
/// and `fold_compensation_error` upgrades the original failure into `VcsError::CompensationFailed`
/// carrying both facts.
///
/// Parent has NOTHING applied, so `parent.undo()` deterministically fails with
/// `NothingToUndo` — simulating a member whose own rollback errors mid-compensation.
#[semio_framework_async_macros::async_test]
async fn compensate_reports_skipped_when_a_members_own_undo_fails_and_folds_to_compensation_failed() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-comp-fail-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-comp-fail-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], transaction: None }, &mut identity).await.expect("apply child");
    let child_edit_id = child_store.envelope().vcs.edits.last().expect("child edit").id.clone();

    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];
    let applied_children = vec![(0usize, child_edit_id)];

    let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some("bogus-parent-edit-id"), &mut identity).await;

    assert_eq!(report.skipped.len(), 1, "the parent's own failed undo must be recorded, not panic");
    assert_eq!(report.skipped[0].0.artifact_id, parent_ref.artifact_id);
    assert!(matches!(&report.skipped[0].1, VcsError::NothingToUndo));
    assert_eq!(report.undone.len(), 1, "the child must still be undone despite the parent's rollback failure");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0));

    let folded = fold_compensation_error(VcsError::ValidationFailed("late failure".into()), report).await;
    assert!(matches!(folded, VcsError::CompensationFailed(_)), "a non-empty skipped list must fold into CompensationFailed, got {folded}");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's "deterministic child id minting across two simulated replicas" law,
/// end-to-end through `dispatch_group` itself (not just the bare `mint_child_id` helper): two
/// independent coordinators/parents dispatching the IDENTICAL genesis converge on the identical
/// minted child id and the identical `invocation_id`.
///
/// 🎯️ Empty array literal: with `dispatch_group`'s `Mp`/`Mc` split, `Mc` no longer has
/// `parent`'s type to piggyback inference off of, so an empty `children` list needs an
/// explicit element type — the same local test wrapper `parent_1` uses.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_mints_genesis_child_ids_deterministically_across_replicas() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-genesis-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let genesis = vec![ChildGenesis { slot: "mesh-slot".into(), dialect: demo_child_dialect(), initial_pack: DemoSnapshot { n: Some(0) }.encode_pack() }];
    let parent_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode")];

    let mut parent_1 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator_1 = CompositionCoordinator::new().await;
    let mut children_1: [(&mut ArtifactStore<DemoSnapshot, DemoMutation>, ChildDispatch); 0] = [];
    let receipt_1 = coordinator_1.dispatch_group(&parent_ref, &mut parent_1, &mut children_1, parent_ops.clone(), genesis.clone(), GroupMeta::default(), &mut identity).await.expect("replica 1 dispatch");

    let mut parent_2 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator_2 = CompositionCoordinator::new();
    let mut children_2: [(&mut ArtifactStore<DemoSnapshot, DemoMutation>, ChildDispatch); 0] = [];
    let receipt_2 = coordinator_2.await.dispatch_group(&parent_ref, &mut parent_2, &mut children_2, parent_ops, genesis, GroupMeta::default(), &mut identity).await.expect("replica 2 dispatch");

    assert_eq!(receipt_1.created_children.len(), 1);
    assert_eq!(receipt_2.created_children.len(), 1);
    assert_eq!(receipt_1.created_children[0].0.artifact_id, receipt_2.created_children[0].0.artifact_id, "two replicas performing the identical genesis must mint the identical child id");
    assert_eq!(receipt_1.invocation_id, receipt_2.invocation_id, "two replicas performing the identical composite gesture must converge on the identical invocation id");

    assert_eq!(parent_1.snapshot().expect("parent snapshot").n, Some(1));
    assert_eq!(coordinator_1.graph().await.owner_of(&receipt_1.created_children[0].0.artifact_id).await, Some(parent_ref.artifact_id.as_str()));
    assert_eq!(receipt_1.member_edits.len(), 1, "only the parent got a real edit here — no existing children were dispatched");
    assert_eq!(receipt_1.member_edits[0].0.artifact_id, parent_ref.artifact_id);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's group-undo law: a member whose tail belongs to a DIFFERENT (foreign)
/// group is skipped, never aborting the rest of the group's undo.
#[semio_framework_async_macros::async_test]
async fn undo_group_skips_a_foreign_tail_member_but_still_undoes_the_rest() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let group_id = "group-xyz";
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-undo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-undo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let foreign_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "foreign-undo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply parent");
    parent_store.stamp_tail_group_id(group_id).await.expect("stamp parent");

    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply child");
    child_store.stamp_tail_group_id(group_id).await.expect("stamp child");

    let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply foreign");
    foreign_store.stamp_tail_group_id("some-other-group").await.expect("stamp foreign");

    let mut children = [(&child_ref, &mut child_store), (&foreign_ref, &mut foreign_store)];

    let report = CompositionCoordinator::undo_group(&parent_ref, &mut parent_store, &mut children, group_id, &mut identity).await;

    assert_eq!(report.undone.len(), 2, "parent + child both belong to the group and must be undone");
    assert_eq!(report.skipped.len(), 1, "the foreign member must be skipped, not abort the group");
    assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);
    assert!(matches!(&report.skipped[0].1, VcsError::ForeignEdit(_)));

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(0), "parent's group edit was undone");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0), "child's group edit was undone");
    assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, Some(3), "the foreign member's own edit must be left untouched");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `redo_group`'s mirror of the same law: a foreign-group member's redo stack is left
/// untouched while a matching member is reapplied.
#[semio_framework_async_macros::async_test]
async fn redo_group_skips_a_foreign_tail_member_but_still_redoes_the_rest() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let group_id = "group-redo-1";
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-redo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let foreign_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "foreign-redo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply parent");
    parent_store.stamp_tail_group_id(group_id).await.expect("stamp parent");
    parent_store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo parent, seeding its redo stack");

    let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }, &mut identity).await.expect("apply foreign");
    foreign_store.stamp_tail_group_id("some-other-group").await.expect("stamp foreign");
    foreign_store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo foreign, seeding its redo stack");

    let mut children = [(&foreign_ref, &mut foreign_store)];
    let report = CompositionCoordinator::redo_group(&parent_ref, &mut parent_store, &mut children, group_id, &mut identity).await;

    assert_eq!(report.undone.len(), 1, "only the matching-group member is redone");
    assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id);
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(1), "parent's edit was reapplied");
    assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, Some(0), "the foreign member's redo stack was left untouched");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#region 🔖️TransactionPeerTests
/// 🧪️ W1-C's headline `Peer`-relation law (contract-freeze §5): two artifacts with NO
/// ownership relation commit through `dispatch_peer_group` as ONE atomic transaction — both
/// members end up carrying the SAME `MutationMeta.group_id` (the shared minted
/// `invocation_id`), the peer's tail edit is stamped `MutationOrigin::Transaction { initiator }`,
/// and the initiator's own edit stays at its ordinary `Owner` default (it is not foreign to
/// itself). Deliberately does NOT seed any `CompositionGraph::insert_owns` edge — `Peer` never
/// consults `owner_of` at all.
#[semio_framework_async_macros::async_test]
async fn dispatch_peer_group_commits_both_members_with_one_shared_group_id() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initiator_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-initiator-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-member-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator = TransactionCoordinator::new();

    let initiator_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode initiator op")];
    let peer_op = DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode peer op");
    let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: &[peer_op], op_schema: &SchemaId("demo/v1".into()), labels: &[semio_framework_ui_locale::LocalizedLabel::data("peer")] };
    let mut peers = [(&mut peer_store, peer_dispatch)];

    let receipt = coordinator.await.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default(), &mut identity).await.expect("peer transaction dispatch");

    assert_eq!(receipt.member_edits.len(), 2, "both the initiator and the one peer got a real edit");
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(1));
    assert_eq!(peer_store.snapshot().expect("peer snapshot").n, Some(2));

    let initiator_group_id = initiator_store.tail_group_id().await.expect("initiator tail group id");
    let peer_group_id = peer_store.tail_group_id().await.expect("peer tail group id");
    assert_eq!(initiator_group_id, peer_group_id, "both members share the SAME minted invocation id as their group id");
    assert_eq!(initiator_group_id, receipt.invocation_id);

    let initiator_origin = initiator_store.envelope().vcs.edits.last().expect("initiator edit").mutation_meta.last().expect("initiator meta").origin.clone();
    assert_eq!(initiator_origin, semio_framework_os_spr::MutationOrigin::Owner, "the initiator's own edit is not foreign to itself, so it stays at the ordinary Owner default");

    let peer_origin = peer_store.envelope().vcs.edits.last().expect("peer edit").mutation_meta.last().expect("peer meta").origin.clone();
    match peer_origin {
        semio_framework_os_spr::MutationOrigin::Transaction { initiator } => assert_eq!(initiator.artifact_id, initiator_ref.artifact_id, "the peer's origin names the real initiator"),
        other => panic!("expected MutationOrigin::Transaction on the peer's tail edit, got {other:?}"),
    }
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ W1-C's compensation law for `Peer`: `compensate` is relation-agnostic (see its own
/// doc comment) — exercising it with a two-PEER (no ownership) scenario proves the SAME
/// reverse-order rollback `dispatch_peer_group`'s phase 2 falls back to on a late failure works
/// identically to the `Owned` case `compensate_undoes_applied_members_in_reverse_order` already
/// covers. Peer B is deliberately left with no applied edit at all, modeling "the SECOND
/// member's own dispatch failed" — the exact trigger `dispatch_peer_group`'s real error branch
/// calls `compensate` from, passing only the members that DID get applied (the initiator + peer
/// A) as `applied_children`/`parent_applied`.
#[semio_framework_async_macros::async_test]
async fn compensate_undoes_applied_peer_members_in_reverse_order() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initiator_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-comp-initiator".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_a_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-comp-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let peer_b_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-comp-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    initiator_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply initiator");
    let initiator_edit_id = initiator_store.envelope().vcs.edits.last().expect("initiator edit").id.clone();

    let mut peer_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    peer_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply peer a — the member the transaction already reached");
    let peer_a_edit_id = peer_a.envelope().vcs.edits.last().expect("a edit").id.clone();

    let mut peer_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let dispatch_a = ChildDispatch { child: peer_a_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let dispatch_b = ChildDispatch { child: peer_b_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut peers = [(&mut peer_a, dispatch_a), (&mut peer_b, dispatch_b)];
    let applied_so_far = vec![(0usize, peer_a_edit_id)];

    let report = TransactionCoordinator::compensate(&initiator_ref, &mut initiator_store, &mut peers, &applied_so_far, Some(&initiator_edit_id), &mut identity).await;

    assert!(report.skipped.is_empty(), "every already-applied member should have undone cleanly");
    assert_eq!(report.undone.len(), 2, "the initiator and peer A (the only two that were actually applied) are both rolled back");
    assert_eq!(report.undone[0].0.artifact_id, initiator_ref.artifact_id, "initiator undone first");
    assert_eq!(report.undone[1].0.artifact_id, peer_a_ref.artifact_id, "then the one applied peer");

    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(0), "initiator's edit was compensated");
    assert_eq!(peer_a.snapshot().expect("a snapshot").n, Some(0), "peer A's edit was compensated");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ Task 2's group-undo law, exercised through a REAL `Peer` transaction end-to-end:
/// `undo_group` (unmodified — see its own doc comment on being relation-agnostic) reverses BOTH
/// members of a `dispatch_peer_group` group as ONE, using the exact `invocation_id` that call
/// minted, with no code path specific to `Peer` needed.
#[semio_framework_async_macros::async_test]
async fn undo_group_reverses_both_members_of_a_real_peer_transaction() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initiator_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-undo-initiator".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-undo-member".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator = TransactionCoordinator::new();

    let initiator_ops = vec![DemoMutation::SetN(SetN { n: 5 }).encode_op().expect("encode initiator op")];
    let peer_op = DemoMutation::SetN(SetN { n: 7 }).encode_op().expect("encode peer op");
    let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: &[peer_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let receipt = {
        let mut peers = [(&mut peer_store, peer_dispatch)];
        coordinator.await.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default(), &mut identity).await.expect("peer transaction dispatch")
    };
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(5));
    assert_eq!(peer_store.snapshot().expect("peer snapshot").n, Some(7));

    let mut children = [(&peer_ref, &mut peer_store)];
    let report = TransactionCoordinator::undo_group(&initiator_ref, &mut initiator_store, &mut children, &receipt.invocation_id, &mut identity).await;

    assert!(report.skipped.is_empty(), "both real transaction members must belong to the group");
    assert_eq!(report.undone.len(), 2);
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot after undo").n, Some(0));
    assert_eq!(peer_store.snapshot().expect("peer snapshot after undo").n, Some(0));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `Peer`'s cycle guard law (`MemberRelation::Peer`'s doc comment): a SECOND, separate
/// `dispatch_peer_group` call that would close a link cycle across the coordinator's persisted
/// `Links` graph is rejected — `CompositionGraph::would_cycle_links`, the same primitive
/// `would_cycle_owns` is to `Owned`'s genesis cycle guard. Transaction 1 (A initiates, B is the
/// peer) succeeds and records a `Links` edge A -> B; transaction 2 (B initiates, A is the peer)
/// would close A -> B -> A and is rejected with zero side effects.
#[semio_framework_async_macros::async_test]
async fn dispatch_peer_group_rejects_a_transaction_that_would_close_a_peer_link_cycle() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let artifact_a_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-cycle-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let artifact_b_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-cycle-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut store_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut store_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator = TransactionCoordinator::new().await;

    let op_ab = DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode a->b op");
    let dispatch_ab = ChildDispatch { child: artifact_b_ref.clone(), ops: &[op_ab], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    {
        let mut peers = [(&mut store_b, dispatch_ab)];
        coordinator.dispatch_peer_group(&artifact_a_ref, &mut store_a, &mut peers, Vec::new(), GroupMeta::default(), &mut identity).await.expect("first transaction A -> B");
    }
    assert_eq!(store_b.snapshot().expect("b snapshot").n, Some(1));

    let op_ba = DemoMutation::SetN(SetN { n: 9 }).encode_op().expect("encode b->a op");
    let dispatch_ba = ChildDispatch { child: artifact_a_ref.clone(), ops: &[op_ba], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut peers = [(&mut store_a, dispatch_ba)];
    let result = coordinator.dispatch_peer_group(&artifact_b_ref, &mut store_b, &mut peers, Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected a CompositionCycle rejection, but the dispatch succeeded"),
        Err(VcsError::CompositionCycle(_)) => {}
        Err(other) => panic!("expected CompositionCycle, got a different VcsError: {other}"),
    }
    assert_eq!(store_a.snapshot().expect("a snapshot").n, Some(0), "A must have zero new edits after a rejected cycle");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ W1-C's "Owned reproduces today's behaviour EXACTLY" law: `CompositionCoordinator`
/// (the `TransactionCoordinator` alias) dispatching through the ordinary `Owned`-relation
/// `dispatch_group` produces EXACTLY what it did before `MemberRelation` existed — including
/// that `MutationMeta.origin` is NEVER touched (stays the ordinary `Apply`-assigned `Owner`
/// default), because `stamp_tail_origin` is only ever called on the `Peer` path. Every other
/// `dispatch_group_*`/`compensate_*`/`undo_group_*`/`redo_group_*` test above this region
/// already re-proves the rest of `Owned`'s behaviour (ownership check, genesis, atomicity,
/// compensation order) is untouched, unmodified, still green.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_owned_path_never_stamps_a_transaction_origin() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-owned-origin-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-owned-origin-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity).await.expect("owned dispatch");

    assert_eq!(receipt.member_edits.len(), 2);
    let parent_origin = parent_store.envelope().vcs.edits.last().expect("parent edit").mutation_meta.last().expect("parent meta").origin.clone();
    let child_origin = child_store.envelope().vcs.edits.last().expect("child edit").mutation_meta.last().expect("child meta").origin.clone();
    assert_eq!(parent_origin, semio_framework_os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on the parent");
    assert_eq!(child_origin, semio_framework_os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on an owned child either");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}
//#endregion 🔖️TransactionPeerTests
//#endregion 🔖️PackValueFixtures
//#endregion 🔖️ReconcileAlternative
