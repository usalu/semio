/// 📨️ App command laws: the fixture helpers, the AppCommand round trips, the transaction forms, the paged route admission laws, the opening and the merge commands.

//#region 🧸️Fixtures
async fn sample_envelope(id: &str) -> protocol::causal::MutationEnvelope {
    protocol::causal::MutationEnvelope {
        mutation_id: protocol::ids::MutationId(id.to_string()),
        document_id: protocol::ids::ArtifactId("document-1".to_string()),
        actor: protocol::ids::ActorId("actor-1".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: protocol::causal::ArtifactDiff { schema: protocol::ids::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
        inverse: protocol::causal::InverseMutation { schema: protocol::ids::SchemaId("diff.v1".to_string()), payload: Vec::new() },
        timestamp: protocol::ids::HybridLogicalTimestamp::new(1, 0),
        transaction: None, verb: None, line: None,
    }
}

fn sample_document_archive() -> DocumentArchivePack {
    let root = DocumentArchiveArtifactRef { artifact_id: "root-1".into(), artifact_kind: "s.test.root".into(), standard: "1".into(), subset: "*".into() };
    let child = DocumentArchiveArtifactRef { artifact_id: "child-1".into(), artifact_kind: "s.test.child".into(), standard: "1".into(), subset: "native".into() };
    let grandchild = DocumentArchiveArtifactRef { artifact_id: "grandchild-1".into(), artifact_kind: "s.test.child".into(), standard: "1".into(), subset: "native".into() };
    DocumentArchivePack {
        parent_pack: vec![1, 2],
        parent_spr: vec![3],
        members: vec![
            OwnedDocumentMemberPackEntry { ordinal: 0, reference: child.clone(), owner: DocumentArchiveOwnerRef { parent: root, slot: "children".into(), child_id: child.artifact_id.clone() }, envelope_pack: vec![4, 5] },
            OwnedDocumentMemberPackEntry { ordinal: 1, reference: grandchild.clone(), owner: DocumentArchiveOwnerRef { parent: child, slot: "nested".into(), child_id: grandchild.artifact_id.clone() }, envelope_pack: vec![6] },
        ],
    }
}

/// #️⃣ Tiny hand-rolled `&[u8] -> String` hex encoder for this crate's own fixture-corpus
/// tests — mirrors `db_engine`'s `write!("{byte:02x}")` idiom (no `hex` crate dependency exists
/// anywhere in `framework/product/os`, so this crate does not introduce one either).
async fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}
//#endregion 🧸️Fixtures

//#region 🔖️AppCommand
async fn assert_command_round_trips(command: &AppCommand) {
    let encoded = encode_app_command(command).await.expect("encode must succeed");
    assert_eq!(encoded.page_len(), 1, "round-trip fixture is one page");
    let bytes = encoded.front_page().expect("round-trip fixture has one page").as_slice();
    let decoded = decode_app_command(bytes).await.expect("decode must succeed");
    assert_eq!(&decoded, command);
}

async fn encode_fixture_command(command: &AppCommand) -> Vec<u8> {
    let encoded = encode_app_command(command).await.expect("fixture encode must succeed");
    assert_eq!(encoded.page_len(), 1, "fixture command is one page");
    encoded.front_page().expect("fixture command has one page").as_slice().to_vec()
}

#[semio_framework_async_macros::async_test]
async fn app_command_config_command_round_trips() {
    assert_command_round_trips(&AppCommand::ConfigCommand { seq: 1, command: vec![9, 9] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_command_round_trips() {
    assert_command_round_trips(&AppCommand::Command { seq: 2, command: vec![1, 2], view_state: vec![] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_command_text_round_trips() {
    assert_command_round_trips(&AppCommand::CommandText { seq: 3, line: "set foo = 1".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_context_menu_round_trips() {
    assert_command_round_trips(&AppCommand::ContextMenu { seq: 5, request: vec![7] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_document_command_round_trips() {
    assert_command_round_trips(&AppCommand::ArtifactCommand { seq: 6, command: vec![8, 8] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_apply_envelopes_round_trips() {
    assert_command_round_trips(&AppCommand::ApplyEnvelopes { seq: 7, envelopes: vec![sample_envelope("op-1").await, sample_envelope("op-2").await] }).await;
}

/// 🕳️ Tag 6 (the retired whole-document `LoadDocument`) stays unassigned: a frame that still carries it is refused as an
/// unknown tag by both decoders, never misread as another command (design §20.7, `📓️api-stepped-document-load.md` §5).
#[semio_framework_async_macros::async_test]
async fn app_command_tag_six_stays_unassigned() {
    let err = decode_app_command(&[6, 8, 1, 1, 1, 2]).await.unwrap_err();
    assert!(matches!(err, protocol::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    let mut pages = CommandPageSet::try_new(1).expect("one fixed page");
    pages.try_push(FixedCommandPage::try_copy_from(&[6, 8, 1, 1, 1, 2]).expect("retired tag page")).unwrap_or_else(|(fault, _page)| panic!("admit retired tag page: {fault:?}"));
    let command = PagedCommand::try_from_pages(pages).unwrap_or_else(|(fault, _pages)| panic!("admit retired tag command: {fault:?}"));
    let fault = PagedAppCommandDecodeCursor::new(command).step().expect_err("the retired tag is refused");
    assert_eq!(fault.code.0, "plugin.command-route-state-machine-required");
}

#[semio_framework_async_macros::async_test]
async fn app_command_read_artifact_round_trips() {
    assert_command_round_trips(&AppCommand::ReadDocument { seq: 9 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_recursive_document_archive_round_trips() {
    assert_command_round_trips(&AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }).await;
    assert_command_round_trips(&AppCommand::MergeDocumentArchive { seq: 13, archive: sample_document_archive() }).await;
    assert_command_round_trips(&AppCommand::ReadDocumentArchive { seq: 11 }).await;
    assert_command_round_trips(&AppCommand::PollDocumentArchiveLoad { seq: 12, operation: 10 }).await;
    assert_command_round_trips(&AppCommand::CancelDocumentArchiveLoad { seq: 13, operation: 10 }).await;
    assert_command_round_trips(&AppCommand::AcknowledgeDocumentArchiveLoad { seq: 14, operation: 10 }).await;
}

#[semio_framework_async_macros::async_test]
async fn document_archive_persisted_bytes_round_trip_exactly() {
    let archive = sample_document_archive();
    let bytes = encode_document_archive_bytes(&archive).expect("encode persisted document archive");
    assert_eq!(decode_document_archive_bytes(&bytes).await.expect("decode persisted document archive"), archive);
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_document_archive_bytes(&trailing).await.is_err());
    assert!(decode_document_archive_bytes(&[2]).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn app_command_load_config_round_trips() {
    assert_command_round_trips(&AppCommand::LoadConfig { seq: 10, pack: vec![1], spr: vec![2] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_read_config_round_trips() {
    assert_command_round_trips(&AppCommand::ReadConfig { seq: 11 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_window_config_round_trips() {
    assert_command_round_trips(&AppCommand::LoadWindowConfig {
        seq: 12,
        entry: WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![1, 2] },
    })
    .await;
    assert_command_round_trips(&AppCommand::ReadWindowConfigs { seq: 13 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_media_in_round_trips() {
    assert_command_round_trips(&AppCommand::MediaIn { seq: 14, port: "camera".to_string(), descriptor: vec![1], data: vec![2, 3] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_media_out_round_trips() {
    assert_command_round_trips(&AppCommand::MediaOut { seq: 15, port: "speaker".to_string(), request: vec![4] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_media_fingerprint_round_trips() {
    assert_command_round_trips(&AppCommand::MediaFingerprint { seq: 16, port: "camera".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_pure_command_round_trips() {
    assert_command_round_trips(&AppCommand::PureCommand { seq: 18, command: vec![1], head: vec![2] }).await;
    assert_command_round_trips(&AppCommand::PureCommand { seq: 19, command: vec![1], head: Vec::new() }).await;
}

//#region 🔖️Transaction
#[semio_framework_async_macros::async_test]
async fn app_command_transaction_prepare_round_trips_owner_and_preplanned_forms() {
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), origin: Vec::new(), prepared_child_ops: Vec::new() }).await;
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], origin: vec![9], prepared_child_ops: Vec::new() }).await;
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 7, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1]], origin: vec![9], prepared_child_ops: vec![5, 6] }).await;
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 8, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: Vec::new(), origin: vec![9], prepared_child_ops: vec![5, 6] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_transaction_commit_round_trips() {
    assert_command_round_trips(&AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_transaction_rollback_round_trips() {
    assert_command_round_trips(&AppCommand::TransactionRollback { seq: 4, txn_id: "t".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_transaction_undo_round_trips() {
    assert_command_round_trips(&AppCommand::TransactionUndo { seq: 5, group_id: "g".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_transaction_redo_round_trips() {
    assert_command_round_trips(&AppCommand::TransactionRedo { seq: 6, group_id: "g".to_string() }).await;
}
//#endregion 🔖️Transaction

//#region 🔖️PagedRouteAdmission
/// 🧵️ Drives the GUEST's own retained ingress — `PagedAppCommandDecodeCursor`, the decoder compiled
/// into every component — rather than the flat host-side `decode_app_command` every round-trip law
/// above uses. A route the flat decoder reads and the paged cursor refuses is exactly the gap this
/// admission law exists to catch (`plugin.command-route-state-machine-required`).
async fn assert_paged_route_admits(command: &AppCommand) {
    let encoded = encode_app_command(command).await.expect("route command encodes");
    let mut cursor = PagedAppCommandDecodeCursor::new(encoded);
    for _ in 0..1024 {
        match cursor.step().expect("paged route decode step") {
            Some(decoded) => {
                assert_eq!(&decoded, command, "paged ingress decodes the route byte for byte");
                return;
            }
            None => continue,
        }
    }
    panic!("paged ingress never completed a route command within its step ceiling");
}

#[semio_framework_async_macros::async_test]
async fn paged_ingress_admits_every_media_route() {
    assert_paged_route_admits(&AppCommand::MediaIn { seq: 14, port: "camera".to_string(), descriptor: vec![1], data: vec![2, 3] }).await;
    assert_paged_route_admits(&AppCommand::MediaOut { seq: 15, port: "vector:out".to_string(), request: Vec::new() }).await;
    assert_paged_route_admits(&AppCommand::MediaFingerprint { seq: 16, port: "camera".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn paged_ingress_admits_every_transaction_route() {
    assert_paged_route_admits(&AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), origin: Vec::new(), prepared_child_ops: Vec::new() }).await;
    assert_paged_route_admits(&AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], origin: vec![9], prepared_child_ops: Vec::new() }).await;
    assert_paged_route_admits(&AppCommand::TransactionPrepare { seq: 7, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1]], origin: vec![9], prepared_child_ops: vec![5, 6] }).await;
    assert_paged_route_admits(&AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }).await;
    assert_paged_route_admits(&AppCommand::TransactionRollback { seq: 4, txn_id: "t".to_string() }).await;
    assert_paged_route_admits(&AppCommand::TransactionUndo { seq: 5, group_id: "g".to_string() }).await;
    assert_paged_route_admits(&AppCommand::TransactionRedo { seq: 6, group_id: "g".to_string() }).await;
}

/// 🚪️ The routes that still require their own reserved ingress authority stay refused BY NAME, so
/// "admitted" never becomes "everything is admitted": `Presence` (tag 28) owns
/// `PresenceCommandCursor`'s one-exact-page-per-peer admission, and every tag no route declares is
/// still a typed refusal rather than a silent drop.
#[semio_framework_async_macros::async_test]
async fn paged_ingress_still_refuses_an_undeclared_route_by_name() {
    let mut pages = CommandPageSet::try_new(1).expect("one fixed page");
    pages.try_push(FixedCommandPage::try_copy_from(&[22, 1, 0, 0, 0, 0]).expect("undeclared route page")).unwrap_or_else(|(fault, _page)| panic!("admit undeclared route page: {fault:?}"));
    let command = PagedCommand::try_from_pages(pages).unwrap_or_else(|(fault, _pages)| panic!("admit undeclared route command: {fault:?}"));
    let mut cursor = PagedAppCommandDecodeCursor::new(command);
    let fault = cursor.step().expect_err("an undeclared route is refused");
    assert_eq!(fault.code.0, "plugin.command-route-state-machine-required");
}

/// 🧹️ A refused route releases what it has already accumulated ONE field at a time under the
/// caller's own grant — the same bounded-retirement contract `LoadDocumentArchive` keeps — so a
/// faulted media command can never free a whole payload in one step.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_media_route_releases_one_field_at_a_time() {
    let command = AppCommand::MediaIn { seq: 7, port: "camera".to_string(), descriptor: vec![1; 64], data: vec![2; 64] };
    let encoded = encode_app_command(&command).await.expect("media route encodes");
    let mut cursor = PagedAppCommandDecodeCursor::new(encoded);
    cursor.step().expect("port field");
    cursor.step().expect("descriptor field");
    let mut releases = 0;
    for _ in 0..1024 {
        let (empty, released) = cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES);
        if released != 0 {
            releases += 1;
        }
        if empty {
            break;
        }
    }
    assert!(releases >= 2, "a cancelled media route released {releases} field(s), not one per step");
    assert!(cursor.terminal_is_empty(), "a cancelled media route retires empty");
}
//#endregion 🔖️PagedRouteAdmission

//#region 🔖️Opening
#[semio_framework_async_macros::async_test]
async fn app_command_open_artifact_round_trips_resolved_and_explicit_forms() {
    assert_command_round_trips(&AppCommand::OpenArtifact { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer".to_string(), role: 0, plugin_id: String::new(), app_id: String::new() }).await;
    assert_command_round_trips(&AppCommand::OpenArtifact { seq: 2, artifact_ref: "s.cad.cad@1/*#editor".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_set_default_app_round_trips() {
    assert_command_round_trips(&AppCommand::SetDefaultApp { seq: 3, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() })
        .await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_clear_default_app_round_trips() {
    assert_command_round_trips(&AppCommand::ClearDefaultApp { seq: 4, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 0 }).await;
}
//#endregion 🔖️Opening

//#region 🔖️Merge
#[semio_framework_async_macros::async_test]
async fn app_command_set_merge_policy_round_trips() {
    assert_command_round_trips(&AppCommand::SetMergePolicy { seq: 5, policy: 0 }).await;
    assert_command_round_trips(&AppCommand::SetMergePolicy { seq: 6, policy: 2 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_resolve_conflict_round_trips() {
    assert_command_round_trips(&AppCommand::ResolveConflict { seq: 7, conflict_id: "c-1".to_string(), resolution: 0 }).await;
    assert_command_round_trips(&AppCommand::ResolveConflict { seq: 8, conflict_id: "c-1".to_string(), resolution: 1 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_read_conflicts_round_trips() {
    assert_command_round_trips(&AppCommand::ReadConflicts { seq: 9 }).await;
}
//#endregion 🔖️Merge
//#endregion 🔖️AppCommand
