use super::*;

#[test]
fn retained_batch_descriptor_has_no_destructor() {
    assert!(!std::mem::needs_drop::<CommandBatchEntry>());
}

//#region 🧸️Fixtures
async fn sample_envelope(id: &str) -> crate::os_spr::causal::MutationEnvelope {
    crate::os_spr::causal::MutationEnvelope {
        mutation_id: crate::os_spr::ids::MutationId(id.to_string()),
        document_id: crate::os_spr::ids::ArtifactId("document-1".to_string()),
        actor: crate::os_spr::ids::ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: crate::os_spr::causal::ArtifactDiff { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
        inverse: crate::os_spr::causal::InverseMutation { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: Vec::new() },
        timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
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

/// @emoji #️⃣ Tiny hand-rolled `&[u8] -> String` hex encoder for this crate's own fixture-corpus
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

#[semio_framework_async_macros::async_test]
async fn app_command_load_document_round_trips() {
    assert_command_round_trips(&AppCommand::LoadDocument { seq: 8, pack: vec![1], spr: vec![2] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_read_artifact_round_trips() {
    assert_command_round_trips(&AppCommand::ReadDocument { seq: 9 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_command_recursive_document_archive_round_trips() {
    assert_command_round_trips(&AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }).await;
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
    assert_command_round_trips(&AppCommand::PureCommand { seq: 18, command: vec![1], document: vec![2], document_spr: vec![3], config: vec![4], config_spr: vec![5], draft: vec![6], draft_spr: vec![7] }).await;
}

//#region 🔖️Transaction
#[semio_framework_async_macros::async_test]
async fn app_command_transaction_prepare_round_trips_owner_and_preplanned_forms() {
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), label: String::new(), origin: Vec::new() }).await;
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9] }).await;
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
    assert_paged_route_admits(&AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), label: String::new(), origin: Vec::new() }).await;
    assert_paged_route_admits(&AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9] }).await;
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

//#region 🔖️AppFrame
async fn assert_frame_round_trips(frame: &AppFrame) {
    let bytes = encode_app_frame(frame);
    let decoded = decode_app_frame(&bytes.await).await.expect("decode must succeed");
    assert_eq!(&decoded, frame);
}

#[semio_framework_async_macros::async_test]
async fn app_frame_done_round_trips() {
    assert_frame_round_trips(&AppFrame::Done { in_reply_to: 1 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_invocation_round_trips() {
    assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2], ui_scope: vec![3], history_patch: vec![4], messages: vec![5], mutations: vec![6], inverse_group: vec![7] }).await;
    assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2], ui_scope: vec![3], history_patch: vec![4], messages: Vec::new(), mutations: Vec::new(), inverse_group: Vec::new() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_invocation_rejects_result_pack_outside_command_transport_authority() {
    let mut encoded = vec![1];
    crate::os_spr::write_varint_u64(&mut encoded, 2);
    for _ in 0..5 {
        crate::os_spr::write_bytes(&mut encoded, &[]);
    }
    crate::os_spr::write_bytes(&mut encoded, &vec![0; INVOCATION_RESULT_PACK_MAXIMUM_BYTES + 1]);
    crate::os_spr::write_bytes(&mut encoded, &[]);
    let error = decode_app_frame(&encoded).await.expect_err("oversized packed mutation authority must fail closed");
    assert!(format!("{error:?}").contains("packed mutations or inverse group exceed command transport authority"));
}

#[semio_framework_async_macros::async_test]
async fn app_frame_document_changed_round_trips() {
    assert_frame_round_trips(&AppFrame::DocumentChanged { envelopes: vec![sample_envelope("op-1").await], origin: "peer-1".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_document_round_trips() {
    assert_frame_round_trips(&AppFrame::Document { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set foo = 1".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_recursive_document_archive_round_trips() {
    assert_frame_round_trips(&AppFrame::DocumentArchive { in_reply_to: 6, archive: sample_document_archive() }).await;
    assert_frame_round_trips(&AppFrame::DocumentArchiveLoad {
        in_reply_to: 7,
        status: DocumentArchiveLoadStatus { operation: 10, state: DocumentArchiveLoadState::Running, completed: 3, total: 9, fault: Vec::new() },
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_config_round_trips() {
    assert_frame_round_trips(&AppFrame::Config { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set cam = 1".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_window_configs_round_trips() {
    assert_frame_round_trips(&AppFrame::WindowConfigs {
        in_reply_to: 6,
        entries: vec![
            WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![1, 2] },
            WindowConfigPackEntry { window_id: "w2".into(), window_kind_id: "graph".into(), envelope_pack: vec![3] },
        ],
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn window_config_transport_matches_shared_cross_language_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/📡️channel/🪟️window-config.json")).expect("window config fixture parses");
    let cases = [
        (
            "loadWindowConfig",
            encode_fixture_command(&AppCommand::LoadWindowConfig {
                seq: 1,
                entry: WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![1, 2] },
            })
            .await,
        ),
        ("readWindowConfigs", encode_fixture_command(&AppCommand::ReadWindowConfigs { seq: 1 }).await),
    ];
    for (key, bytes) in cases {
        assert_eq!(hex_encode(&bytes).await, fixture[key].as_str().expect("command fixture hex"));
    }
    let frame = AppFrame::WindowConfigs {
        in_reply_to: 1,
        entries: vec![
            WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![1, 2] },
            WindowConfigPackEntry { window_id: "w2".into(), window_kind_id: "graph".into(), envelope_pack: vec![3] },
        ],
    };
    assert_eq!(hex_encode(&encode_app_frame(&frame).await).await, fixture["windowConfigs"].as_str().expect("frame fixture hex"));
}

#[semio_framework_async_macros::async_test]
async fn app_frame_config_changed_round_trips() {
    assert_frame_round_trips(&AppFrame::ConfigChanged { envelopes: vec![sample_envelope("cfg-1").await], origin: "peer-1".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_context_menu_round_trips() {
    assert_frame_round_trips(&AppFrame::ContextMenu { in_reply_to: 6, items: vec![1, 2, 3] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_media_round_trips() {
    assert_frame_round_trips(&AppFrame::Media { in_reply_to: 7, port: "camera".to_string(), descriptor: vec![1], data: vec![2] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_media_fingerprint_round_trips() {
    assert_frame_round_trips(&AppFrame::MediaFingerprint { in_reply_to: 8, port: "camera".to_string(), fingerprint: vec![1, 2] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_error_round_trips() {
    assert_frame_round_trips(&AppFrame::Error { in_reply_to: Some(9), fault: b"rejected:bad command".to_vec(), report: vec![1, 2] }).await;
    assert_frame_round_trips(&AppFrame::Error { in_reply_to: None, fault: b"rejected:bad command".to_vec(), report: Vec::new() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_emit_round_trips() {
    assert_frame_round_trips(&AppFrame::Emit { in_reply_to: 14, document_ops: vec![1], config_ops: vec![2], draft_ops: vec![3], output: vec![4], diagnostics: vec![5] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_draft_round_trips() {
    assert_frame_round_trips(&AppFrame::Draft { in_reply_to: 15, pack: vec![1], spr: vec![2], ops: "d".to_string() }).await;
    assert_frame_round_trips(&AppFrame::Children { in_reply_to: 16, entries: sample_child_entries().await }).await;
    assert_frame_round_trips(&AppFrame::Children { in_reply_to: 17, entries: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![9, 9], tool_run: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: Vec::new(), tool_run: vec![5, 6, 7] }).await;
}

//#region 🔖️Children
/// 🧸️ Two children in different slots, one of them with an empty pack (a genesis child whose
/// envelope has not been printed yet), so the list codec is exercised at both extremes.
async fn sample_child_entries() -> Vec<ChildPackEntry> {
    vec![
        ChildPackEntry { slot: "mesh".to_string(), child_id: "child-1".to_string(), dialect: "s.stdio.mesh@1/*".to_string(), envelope_pack: vec![7, 8, 9] },
        ChildPackEntry { slot: "brep".to_string(), child_id: "child-2".to_string(), dialect: "s.stdio.brep@1/*".to_string(), envelope_pack: Vec::new() },
    ]
}

#[semio_framework_async_macros::async_test]
async fn child_pack_commands_round_trip() {
    assert_command_round_trips(&AppCommand::LoadChildren { seq: 19, entries: sample_child_entries().await }).await;
    assert_command_round_trips(&AppCommand::LoadChildren { seq: 20, entries: Vec::new() }).await;
    assert_command_round_trips(&AppCommand::ReadChildren { seq: 21 }).await;
    assert_command_round_trips(&AppCommand::ReadHistory { seq: 22 }).await;
}
//#endregion 🔖️Children

//#region 🔖️Transaction
#[semio_framework_async_macros::async_test]
async fn app_frame_transaction_proposal_round_trips() {
    assert_frame_round_trips(&AppFrame::TransactionProposal { in_reply_to: 1, proposal_id: "p".to_string(), local_ops: vec![vec![1]], description: "d".to_string(), coalesce_key: "k".to_string(), foreign: Vec::new() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_transaction_prepared_round_trips_with_and_without_rejection() {
    assert_frame_round_trips(&AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: vec![vec![1]], rejection: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: Vec::new(), rejection: b"rejected".to_vec() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_transaction_committed_round_trips() {
    assert_frame_round_trips(&AppFrame::TransactionCommitted { txn_id: "t".to_string(), edit_id: "e".to_string() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_transaction_rolled_back_round_trips() {
    assert_frame_round_trips(&AppFrame::TransactionRolledBack { txn_id: "t".to_string() }).await;
}
//#endregion 🔖️Transaction

//#region 🔖️Merge
#[semio_framework_async_macros::async_test]
async fn app_frame_merge_report_round_trips() {
    assert_frame_round_trips(&AppFrame::MergeReport { in_reply_to: Some(1), report: vec![1, 2, 3] }).await;
    assert_frame_round_trips(&AppFrame::MergeReport { in_reply_to: None, report: Vec::new() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_conflicts_round_trips() {
    assert_frame_round_trips(&AppFrame::Conflicts { in_reply_to: Some(2), conflicts: vec![4, 5] }).await;
    assert_frame_round_trips(&AppFrame::Conflicts { in_reply_to: None, conflicts: Vec::new() }).await;
}
//#endregion 🔖️Merge

//#region 🔖️UiPatch
#[semio_framework_async_macros::async_test]
async fn app_frame_ui_patch_round_trips_with_and_without_in_reply_to() {
    assert_frame_round_trips(&AppFrame::UiPatch { in_reply_to: Some(3), surface: "1:body".to_string(), kind: "window".to_string(), revision: 5, base_revision: 4, ops: vec![1, 2, 3] }).await;
    assert_frame_round_trips(&AppFrame::UiPatch { in_reply_to: None, surface: "1:body".to_string(), kind: "window".to_string(), revision: 1, base_revision: 0, ops: Vec::new() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_ui_snapshot_end_round_trips() {
    assert_frame_round_trips(&AppFrame::UiSnapshotEnd { revision: 7 }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_operation_completed_round_trips() {
    assert_frame_round_trips(&AppFrame::OperationCompleted { operation: 7, revision: 5, ui_scope: vec![1], history_patch: vec![2] }).await;
    assert_frame_round_trips(&AppFrame::OperationCompleted { operation: u64::MAX, revision: 0, ui_scope: Vec::new(), history_patch: Vec::new() }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_operation_completed_matches_shared_cross_language_json_vector() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/📡️channel/🏁️app-frame-operation-completed.json")).expect("operation completion fixture parses");
    let frame = AppFrame::OperationCompleted { operation: 7, revision: 5, ui_scope: vec![1], history_patch: vec![2] };
    assert_eq!(hex_encode(&encode_app_frame(&frame).await).await, fixture["OperationCompleted"].as_str().expect("frame fixture hex"));
    let wide = AppFrame::OperationCompleted { operation: 7, revision: 0xfedc_ba98_7654_3210, ui_scope: vec![1], history_patch: vec![2] };
    assert_eq!(hex_encode(&encode_app_frame(&wide).await).await, fixture["OperationCompletedWideRevision"].as_str().expect("wide frame fixture hex"));
}
//#endregion 🔖️UiPatch
//#endregion 🔖️AppFrame

//#region 🔖️Codec
#[semio_framework_async_macros::async_test]
async fn encoding_is_deterministic() {
    let command = AppCommand::ContextMenu { seq: 1, request: vec![1, 2, 3] };
    assert_eq!(encode_app_command(&command).await, encode_app_command(&command).await);

    let frame = AppFrame::Error { in_reply_to: Some(1), fault: b"e:m".to_vec(), report: vec![9] };
    assert_eq!(encode_app_frame(&frame).await, encode_app_frame(&frame).await);
}

#[semio_framework_async_macros::async_test]
async fn decode_app_command_rejects_empty_bytes() {
    let err = decode_app_command(&[]).await.unwrap_err();
    assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-command tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_app_frame_rejects_empty_bytes() {
    let err = decode_app_frame(&[]).await.unwrap_err();
    assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_app_command_rejects_unknown_tag() {
    let err = decode_app_command(&[0xFF]).await.unwrap_err();
    assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-command tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_app_frame_rejects_unknown_tag() {
    let err = decode_app_frame(&[0xFF]).await.unwrap_err();
    assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_app_command_rejects_truncated_field() {
    let encoded = encode_app_command(&AppCommand::CommandText { seq: 1, line: "hello".to_string() }).await.unwrap();
    assert_eq!(encoded.page_len(), 1);
    let bytes = encoded.front_page().unwrap().as_slice();
    let truncated = &bytes[..bytes.len() - 2];
    assert!(decode_app_command(truncated).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn presence_roster_fixed_maximum_plus_one_returns_the_exact_rejected_owner() {
    let mut roster = PresenceRosterWire::empty();
    for index in 0..PRESENCE_ROSTER_MAXIMUM_ITEMS {
        roster.try_push(vec![index as u8]).expect("fixed admitted roster slot");
    }
    let rejected = vec![0xA5; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES];
    let error = roster.try_push(rejected.clone()).expect_err("maximum plus one must fail before growth");
    assert_eq!(error.entry, rejected);
    assert_eq!(roster.len(), PRESENCE_ROSTER_MAXIMUM_ITEMS);
}

#[semio_framework_async_macros::async_test]
async fn presence_cursor_preserves_fifo_and_releases_at_most_one_grant() {
    let mut roster = PresenceRosterWire::empty();
    roster.try_push(vec![1; 17]).unwrap();
    roster.try_push(vec![2; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES]).unwrap();
    let mut cursor = PresenceCommandCursor::admit_page(77, Some(4), 2, FixedCommandPage::try_copy_from(&[1; 17]).unwrap()).map_err(|(error, _)| error).unwrap();
    assert_eq!(cursor.take_next().unwrap().unwrap().as_slice(), &[1; 17]);
    cursor.push_page(1, FixedCommandPage::try_copy_from(&[2; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES]).unwrap()).map_err(|(error, _)| error).unwrap();
    assert_eq!(cursor.take_next().unwrap().unwrap().as_slice(), &[2; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES]);
    assert!(cursor.take_next().unwrap().is_none());
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn generic_decoder_rejects_presence_before_whole_roster_materialization() {
    let mut roster = PresenceRosterWire::empty();
    roster.try_push(vec![7]).unwrap();
    let encoded = encode_app_command(&AppCommand::Presence { seq: 3, own_color: None, peers: roster }).await.unwrap();
    assert_eq!(encoded.kind(), 28);
    let mut cursor = PresenceCommandCursor::admit_page(3, None, 1, FixedCommandPage::try_copy_from(&[7]).unwrap()).map_err(|(error, _)| error).unwrap();
    while !cursor.close_release(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES).0 {}
}

#[semio_framework_async_macros::async_test]
async fn paged_generic_decoder_crosses_a_two_page_field_boundary_without_concatenation() {
    let expected = AppCommand::Command { seq: 1, command: vec![0xA5; COMMAND_PAGE_MAXIMUM_BYTES + 1], view_state: vec![7, 8] };
    let encoded = encode_app_command(&expected).await.unwrap();
    assert_eq!(encoded.page_len(), 2);
    let mut cursor = PagedAppCommandDecodeCursor::new(encoded);
    assert!(cursor.step().unwrap().is_none());
    assert!(cursor.step().unwrap().is_none());
    assert_eq!(cursor.step().unwrap(), Some(expected));
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn paged_generic_decoder_admits_document_config_and_projection_commands_used_during_browser_boot() {
    // 🧮️ Each row carries the EXACT number of bounded cursor turns its paged decode costs: one
    // header turn plus one turn per length-prefixed field. A two-member document archive is
    // 1 + 3 (parent pack, parent spr, member count) + 2 x 12 member phases = 28 — a shape the
    // earlier uniform four-turn budget could never reach, which is why this law read red from the
    // day the archive rows were admitted into it.
    let commands = vec![
        (AppCommand::LoadDocument { seq: 1, pack: vec![1, 2], spr: vec![3] }, 3),
        (AppCommand::ReadDocument { seq: 2 }, 2),
        (AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }, 28),
        (AppCommand::ReadDocumentArchive { seq: 11 }, 2),
        (AppCommand::LoadConfig { seq: 3, pack: vec![4], spr: vec![5, 6] }, 3),
        (AppCommand::ReadConfig { seq: 4 }, 2),
        (AppCommand::ReadChildren { seq: 5 }, 2),
        (AppCommand::ReadHistory { seq: 6 }, 2),
        (AppCommand::ReadConflicts { seq: 7 }, 2),
        (AppCommand::LoadWindowConfig { seq: 8, entry: WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![7, 8] } }, 4),
        (AppCommand::ReadWindowConfigs { seq: 9 }, 2),
    ];
    for (expected, turns) in commands {
        let encoded = encode_app_command(&expected).await.unwrap();
        let mut cursor = PagedAppCommandDecodeCursor::new(encoded);
        for turn in 1..turns {
            assert_eq!(cursor.step().unwrap(), None, "turn {turn} of {expected:?} must not yield the command early");
        }
        assert_eq!(cursor.step().unwrap(), Some(expected));
        assert!(cursor.terminal_is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn paged_recursive_archive_crosses_pages_and_decoded_owner_closes_one_field_per_grant() {
    let mut archive = sample_document_archive();
    archive.members[1].envelope_pack = vec![0xA5; COMMAND_PAGE_MAXIMUM_BYTES + 1];
    let expected = AppCommand::LoadDocumentArchive { seq: 12, archive };
    let encoded = encode_app_command(&expected).await.unwrap();
    assert!(encoded.page_len() > 1);
    let mut cursor = PagedAppCommandDecodeCursor::new(encoded);
    let mut decoded = None;
    for _ in 0..64 {
        decoded = cursor.step().unwrap();
        if decoded.is_some() {
            break;
        }
    }
    assert_eq!(decoded, Some(expected));
    assert!(cursor.terminal_is_empty());

    let mut owner = DecodedAppCommandOwner::new(decoded.unwrap_or_else(|| unreachable!()));
    let mut released_items = 0;
    let mut released_bytes = 0;
    for _ in 0..64 {
        let (empty, items, bytes) = owner.close_step(COMMAND_PAGE_MAXIMUM_BYTES + 1);
        released_items += items;
        released_bytes += bytes;
        if empty {
            break;
        }
    }
    assert!(owner.terminal_is_empty());
    assert!(released_items >= 24);
    assert!(released_bytes > COMMAND_PAGE_MAXIMUM_BYTES);
}

#[semio_framework_async_macros::async_test]
async fn document_archive_decoders_reject_member_counts_beyond_fixed_authority() {
    let mut bytes = vec![32, 1, 0, 0];
    crate::os_spr::write_varint_u64(&mut bytes, (DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS + 1) as u64);
    assert!(decode_app_command(&bytes).await.is_err());
    let page = FixedCommandPage::try_copy_from(&bytes).unwrap();
    let mut pages = CommandPageSet::try_new(1).unwrap();
    pages.try_push(page).unwrap();
    let mut cursor = PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap());
    assert!(cursor.step().unwrap().is_none());
    assert!(cursor.step().unwrap().is_none());
    assert!(cursor.step().unwrap().is_none());
    assert_eq!(cursor.step().unwrap_err().code.0, "plugin.document-archive-members");
    while !cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0 {}
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn paged_generic_decoder_faults_hostile_field_length_and_closes_exact_owner() {
    let page = FixedCommandPage::try_copy_from(&[0, 1, 0x81, 0x80, 0x80, 0x04]).unwrap();
    let mut pages = CommandPageSet::try_new(1).unwrap();
    pages.try_push(page).unwrap();
    let mut cursor = PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap());
    assert!(cursor.step().unwrap().is_none());
    let fault = cursor.step().unwrap_err();
    assert_eq!(fault.code.0, "plugin.command-field-cap");
    assert_eq!(cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 0));
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn paged_generic_decoder_retains_first_field_when_middle_field_is_truncated() {
    let page = FixedCommandPage::try_copy_from(&[1, 1, 1, 9, 5, 7]).unwrap();
    let mut pages = CommandPageSet::try_new(1).unwrap();
    pages.try_push(page).unwrap();
    let mut cursor = PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap());
    assert!(cursor.step().unwrap().is_none());
    assert!(cursor.step().unwrap().is_none());
    assert_eq!(cursor.step().unwrap_err().code.0, "plugin.command-decode-truncated");
    assert_eq!(cursor.close_step(1), (false, 1));
    assert_eq!(cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 0));
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn paged_generic_decoder_retains_terminal_fields_when_trailing_bytes_fault() {
    let page = FixedCommandPage::try_copy_from(&[3, 1, 1, 9, 0xFF]).unwrap();
    let mut pages = CommandPageSet::try_new(1).unwrap();
    pages.try_push(page).unwrap();
    let mut cursor = PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap());
    assert!(cursor.step().unwrap().is_none());
    assert_eq!(cursor.step().unwrap_err().code.0, "plugin.command-decode-trailing");
    assert_eq!(cursor.close_step(1), (false, 1));
    assert_eq!(cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 5));
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn decoded_generic_owner_closes_two_fields_one_grant_at_a_time() {
    let mut owner = DecodedAppCommandOwner::new(AppCommand::Command { seq: 7, command: vec![1; 4_096], view_state: vec![2; 4_096] });
    assert_eq!(owner.close_step(4_096), (false, 1, 4_096));
    assert_eq!(owner.close_step(4_096), (false, 1, 4_096));
    assert_eq!(owner.close_step(4_096), (true, 1, 0));
    assert!(owner.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn decode_app_frame_rejects_truncated_field() {
    let bytes = encode_app_frame(&AppFrame::Error { in_reply_to: Some(1), fault: b"e:message".to_vec(), report: Vec::new() }).await;
    let truncated = &bytes[..bytes.len() - 2];
    assert!(decode_app_frame(truncated).await.is_err());
}

/// 📏️ A frame is exactly its bytes: a payload that merely STARTS like a frame is not one. A shell
/// message that is not a frame — a document-backbone binding receipt, whose pack-value tag 3 reads as
/// `AppFrame::Document` — was taken for a frame by a prefix decode and vanished from the shell's
/// messages (measured on native block2d: "binding returned 0 shell receipts", ticket 26/09/23 slice WG8).
#[semio_framework_async_macros::async_test]
async fn decode_app_frame_refuses_trailing_bytes() {
    let mut bytes = encode_app_frame(&AppFrame::Document { in_reply_to: 42, pack: vec![1, 2], spr: vec![3], ops: "ops".into() }).await;
    assert!(decode_app_frame(&bytes).await.is_ok(), "the exact frame decodes");
    bytes.push(0);
    assert!(decode_app_frame(&bytes).await.is_err(), "one trailing byte makes it no frame");
}

#[semio_framework_async_macros::async_test]
async fn decode_app_command_never_panics_on_arbitrary_short_buffers() {
    for len in 0..8 {
        let buf = vec![0u8; len];
        let _ = decode_app_command(&buf);
    }
}

#[semio_framework_async_macros::async_test]
async fn decode_app_frame_never_panics_on_arbitrary_short_buffers() {
    for len in 0..8 {
        let buf = vec![0u8; len];
        let _ = decode_app_frame(&buf);
    }
}
//#endregion 🔖️Codec

//#region 🔖️Corpus
// Cross-language drift fixture: a sibling TypeScript work package duplicates these exact hex
// strings in a vitest suite, so `AppCommand`/`AppFrame` and the TS-side codec they hand-port
// stay byte-exact. Every entry is `(variant label, value)`; `channel_command_fixture_hex`/
// `channel_frame_fixture_hex` below are this codec's own committed golden hex per label —
// sourced from `encode_app_command`/`encode_app_frame`'s actual output, not hand-computed.

/// @emoji 🧾️ Named `AppCommand` fixture corpus, one entry per variant.
async fn channel_command_fixture_corpus() -> Vec<(&'static str, AppCommand)> {
    let mut presence_roster = PresenceRosterWire::empty();
    presence_roster.try_push(vec![1, 2]).expect("bounded presence fixture entry");
    presence_roster.try_push(vec![9]).expect("bounded presence fixture entry");
    vec![
        ("ConfigCommand", AppCommand::ConfigCommand { seq: 1, command: vec![9] }),
        ("Command", AppCommand::Command { seq: 1, command: vec![1], view_state: vec![] }),
        ("CommandText", AppCommand::CommandText { seq: 1, line: "go".to_string() }),
        ("ContextMenu", AppCommand::ContextMenu { seq: 1, request: vec![1] }),
        ("ArtifactCommand", AppCommand::ArtifactCommand { seq: 1, command: vec![1] }),
        ("ApplyEnvelopes", AppCommand::ApplyEnvelopes { seq: 1, envelopes: Vec::new() }),
        ("LoadDocument", AppCommand::LoadDocument { seq: 1, pack: vec![1], spr: vec![2] }),
        ("ReadDocument", AppCommand::ReadDocument { seq: 1 }),
        ("LoadConfig", AppCommand::LoadConfig { seq: 1, pack: vec![1], spr: vec![2] }),
        ("ReadConfig", AppCommand::ReadConfig { seq: 1 }),
        ("LoadWindowConfig", AppCommand::LoadWindowConfig { seq: 1, entry: WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![1, 2] } }),
        ("ReadWindowConfigs", AppCommand::ReadWindowConfigs { seq: 1 }),
        ("MediaIn", AppCommand::MediaIn { seq: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
        ("MediaOut", AppCommand::MediaOut { seq: 1, port: "p".to_string(), request: vec![1] }),
        ("MediaFingerprint", AppCommand::MediaFingerprint { seq: 1, port: "p".to_string() }),
        ("PureCommand", AppCommand::PureCommand { seq: 1, command: vec![1], document: vec![2], document_spr: vec![3], config: vec![4], config_spr: vec![5], draft: vec![6], draft_spr: vec![7] }),
        ("LoadChildren", AppCommand::LoadChildren { seq: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1] }] }),
        ("ReadChildren", AppCommand::ReadChildren { seq: 1 }),
        ("ReadHistory", AppCommand::ReadHistory { seq: 1 }),
        ("TransactionPrepareOwner", AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), label: String::new(), origin: Vec::new() }),
        ("TransactionPreparePrePlanned", AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9] }),
        ("TransactionCommit", AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }),
        ("TransactionRollback", AppCommand::TransactionRollback { seq: 4, txn_id: "t".to_string() }),
        ("TransactionUndo", AppCommand::TransactionUndo { seq: 5, group_id: "g".to_string() }),
        ("TransactionRedo", AppCommand::TransactionRedo { seq: 6, group_id: "g".to_string() }),
        ("OpenArtifactResolve", AppCommand::OpenArtifact { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer".to_string(), role: 0, plugin_id: String::new(), app_id: String::new() }),
        ("OpenArtifactExplicit", AppCommand::OpenArtifact { seq: 2, artifact_ref: "s.cad.cad@1/*#editor".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }),
        ("SetDefaultApp", AppCommand::SetDefaultApp { seq: 3, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }),
        ("ClearDefaultApp", AppCommand::ClearDefaultApp { seq: 4, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 0 }),
        ("SetMergePolicy", AppCommand::SetMergePolicy { seq: 5, policy: 1 }),
        ("ResolveConflict", AppCommand::ResolveConflict { seq: 6, conflict_id: "conflict-1".to_string(), resolution: 0 }),
        ("ReadConflicts", AppCommand::ReadConflicts { seq: 7 }),
        ("Presence", AppCommand::Presence { seq: 8, own_color: Some(3), peers: presence_roster }),
    ]
}

/// @emoji 🧾️ Named `AppFrame` fixture corpus, one entry per variant.
async fn channel_frame_fixture_corpus() -> Vec<(&'static str, AppFrame)> {
    vec![
        ("Done", AppFrame::Done { in_reply_to: 1 }),
        ("Invocation", AppFrame::Invocation { in_reply_to: 1, output: vec![1], diagnostics: vec![], ui_scope: vec![], history_patch: vec![], messages: vec![9], mutations: vec![10], inverse_group: vec![11] }),
        ("DocumentChanged", AppFrame::DocumentChanged { envelopes: vec![], origin: "o".to_string() }),
        ("Document", AppFrame::Document { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "o".to_string() }),
        ("Config", AppFrame::Config { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "c".to_string() }),
        (
            "WindowConfigs",
            AppFrame::WindowConfigs {
                in_reply_to: 1,
                entries: vec![
                    WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![1, 2] },
                    WindowConfigPackEntry { window_id: "w2".into(), window_kind_id: "graph".into(), envelope_pack: vec![3] },
                ],
            },
        ),
        ("ConfigChanged", AppFrame::ConfigChanged { envelopes: vec![], origin: "o".to_string() }),
        ("ContextMenu", AppFrame::ContextMenu { in_reply_to: 1, items: vec![1] }),
        ("Media", AppFrame::Media { in_reply_to: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
        ("MediaFingerprint", AppFrame::MediaFingerprint { in_reply_to: 1, port: "p".to_string(), fingerprint: vec![1] }),
        ("Error", AppFrame::Error { in_reply_to: None, fault: vec![99], report: vec![7] }),
        ("Emit", AppFrame::Emit { in_reply_to: 1, document_ops: vec![1], config_ops: vec![], draft_ops: vec![], output: vec![2], diagnostics: vec![] }),
        ("Draft", AppFrame::Draft { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "d".to_string() }),
        ("Children", AppFrame::Children { in_reply_to: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1] }] }),
        ("Ephemeral", AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![7], tool_run: vec![8] }),
        ("HistorySnapshot", AppFrame::HistorySnapshot { in_reply_to: 1, history_patch: vec![1] }),
        ("TransactionProposal", AppFrame::TransactionProposal { in_reply_to: 1, proposal_id: "p".to_string(), local_ops: vec![vec![1]], description: "d".to_string(), coalesce_key: "k".to_string(), foreign: Vec::new() }),
        ("TransactionPrepared", AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: vec![vec![1]], rejection: Vec::new() }),
        ("TransactionCommitted", AppFrame::TransactionCommitted { txn_id: "t".to_string(), edit_id: "e".to_string() }),
        ("TransactionRolledBack", AppFrame::TransactionRolledBack { txn_id: "t".to_string() }),
        ("MergeReport", AppFrame::MergeReport { in_reply_to: Some(1), report: vec![1] }),
        ("Conflicts", AppFrame::Conflicts { in_reply_to: None, conflicts: vec![2] }),
        ("UiPatch", AppFrame::UiPatch { in_reply_to: Some(1), surface: "1:body".to_string(), kind: "window".to_string(), revision: 2, base_revision: 1, ops: vec![3] }),
        ("UiSnapshotEnd", AppFrame::UiSnapshotEnd { revision: 4 }),
        ("OperationCompleted", AppFrame::OperationCompleted { operation: 7, revision: 5, ui_scope: vec![1], history_patch: vec![2] }),
    ]
}

/// @emoji 🔒️ Golden hex per `AppCommand` fixture-corpus label — sourced by actually running
/// `encode_app_command` over `channel_command_fixture_corpus()` (never hand-computed), then
/// committed here as the drift guard: any future codec change that shifts these bytes fails
/// this test, forcing a deliberate update of both this table and the TS-side twin (WP-0B).
async fn channel_command_fixture_hex(label: &str) -> &'static str {
    match label {
        "ConfigCommand" => "00010109",
        "Command" => "0101010100",
        "CommandText" => "020102676f",
        "ContextMenu" => "03010101",
        "ArtifactCommand" => "04010101",
        "ApplyEnvelopes" => "050100",
        "LoadDocument" => "060101010102",
        "ReadDocument" => "0701",
        "LoadConfig" => "080101010102",
        "ReadConfig" => "0901",
        "LoadWindowConfig" => "1e01027731056772617068020102",
        "ReadWindowConfigs" => "1f01",
        "MediaIn" => "0a01017001010102",
        "MediaOut" => "0b0101700101",
        "MediaFingerprint" => "0c010170",
        "PureCommand" => "0d010101010201030104010501060107",
        "LoadChildren" => "0e01010173016301640101",
        "ReadChildren" => "0f01",
        "ReadHistory" => "1001",
        "TransactionPrepareOwner" => "11010174016d0109000000",
        "TransactionPreparePrePlanned" => "110201740000020101020202016c0109",
        "TransactionCommit" => "12030174",
        "TransactionRollback" => "13040174",
        "TransactionUndo" => "14050167",
        "TransactionRedo" => "15060167",
        "OpenArtifactResolve" => "160114732e6361642e63616440312f2a23766965776572000000",
        "OpenArtifactExplicit" => "160214732e6361642e63616440312f2a23656469746f72010363616414732e6361642e63616440312f2a23656469746f72",
        "SetDefaultApp" => "170309732e6361642e6361640131012a010363616414732e6361642e63616440312f2a23656469746f72",
        "ClearDefaultApp" => "180409732e6361642e6361640131012a00",
        "SetMergePolicy" => "190501",
        "ResolveConflict" => "1a060a636f6e666c6963742d3100",
        "ReadConflicts" => "1b07",
        "Presence" => "1c080103020201020109",
        other => panic!("channel_command_fixture_hex: no golden hex registered for label {other:?}"),
    }
}

/// @emoji 🔒️ Golden hex per `AppFrame` fixture-corpus label — see
/// `channel_command_fixture_hex`'s docstring for provenance/drift-guard rationale.
async fn channel_frame_fixture_hex(label: &str) -> &'static str {
    match label {
        "Done" => "0001",
        "Invocation" => "010101010000000109010a010b",
        "DocumentChanged" => "0200016f",
        "Document" => "030101010102016f",
        "Config" => "0401010101020163",
        "WindowConfigs" => "1801020277310567726170680201020277320567726170680103",
        "ConfigChanged" => "0500016f",
        "ContextMenu" => "06010101",
        "Media" => "0701017001010102",
        "MediaFingerprint" => "080101700101",
        "Error" => "090001630107",
        "Emit" => "0a0101010000010200",
        "Draft" => "0b01010101020164",
        "Children" => "0c01010173016301640101",
        "Ephemeral" => "0d020102030401070108",
        "HistorySnapshot" => "0e010101",
        "TransactionProposal" => "0f0101700101010164016b00",
        "TransactionPrepared" => "10017401010100",
        "TransactionCommitted" => "1101740165",
        "TransactionRolledBack" => "120174",
        "MergeReport" => "1301010101",
        "Conflicts" => "14000102",
        "UiPatch" => "15010106313a626f64790677696e646f7702010103",
        "UiSnapshotEnd" => "1604",
        "OperationCompleted" => "19070501010102",
        other => panic!("channel_frame_fixture_hex: no golden hex registered for label {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn app_command_fixture_corpus_matches_golden_hex_and_round_trips() {
    for (label, value) in channel_command_fixture_corpus().await {
        if let AppCommand::Presence { seq, own_color, peers } = &value {
            let first = peers.iter().next().map(<[u8]>::to_vec).unwrap_or_default();
            let mut cursor = PresenceCommandCursor::admit_page(*seq, *own_color, peers.len() as u32, FixedCommandPage::try_copy_from(&first).expect("encoded Presence page is fixed-authority"))
                .map_err(|(error, _)| error)
                .expect("Presence uses retained cursor admission");
            assert_eq!(cursor.seq(), *seq);
            assert_eq!(cursor.own_color(), *own_color);
            let mut entries = Vec::new();
            for (index, peer) in peers.iter().enumerate() {
                if index != 0 {
                    cursor.push_page(index as u32, FixedCommandPage::try_copy_from(peer).expect("encoded Presence page is fixed-authority")).map_err(|(error, _)| error).expect("ordered peer page");
                }
                if let Some(entry) = cursor.take_next().expect("bounded Presence entry") {
                    entries.push(entry);
                }
            }
            if peers.is_empty() {
                while !cursor.close_release(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES).0 {}
            }
            assert!(entries.iter().map(FixedCommandPage::as_slice).eq(peers.iter()), "Presence retained cursor must preserve exact entry order");
            while !cursor.close_release(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES).0 {}
            assert!(cursor.terminal_is_empty());
            continue;
        }
        let encoded = encode_app_command(&value).await.unwrap();
        assert_eq!(encoded.page_len(), 1, "fixture is one page");
        let bytes = encoded.front_page().expect("fixture has one page").as_slice();
        let actual = hex_encode(bytes).await;
        assert_eq!(actual, channel_command_fixture_hex(label).await, "{label}'s encoding drifted from its committed golden hex");
        let decoded = decode_app_command(bytes).await.unwrap();
        assert_eq!(decoded, value, "{label} must round-trip");
    }
}

#[semio_framework_async_macros::async_test]
async fn app_frame_fixture_corpus_matches_golden_hex_and_round_trips() {
    for (label, value) in channel_frame_fixture_corpus().await {
        let encoded = encode_app_frame(&value).await;
        let actual = hex_encode(&encoded).await;
        assert_eq!(actual, channel_frame_fixture_hex(label).await, "{label}'s encoding drifted from its committed golden hex");
        let decoded = decode_app_frame(&encode_app_frame(&value).await).await.unwrap();
        assert_eq!(decoded, value, "{label} must round-trip");
    }
}

/// 📡️ The wire version is owned by `🧫️fixtures/📡️channel/🔖️channel-version.json`, not by
/// either language's constant, so a bump that updates only one host fails here instead of at
/// runtime — the drift this guard was added for was a live `APP_CHANNEL_VERSION = 8` in
/// TypeScript against `CHANNEL_VERSION = 10` in Rust. The TS twin asserts the same file.
#[semio_framework_async_macros::async_test]
async fn channel_version_matches_the_shared_cross_language_pin() {
    let json = include_str!("../../../../../🧫️fixtures/📡️channel/🔖️channel-version.json");
    let pin: serde_json::Value = serde_json::from_str(json).expect("🔖️channel-version.json must parse");
    let pinned = pin.get("channelVersion").and_then(serde_json::Value::as_u64).expect("🔖️channel-version.json must carry channelVersion");
    assert_eq!(u64::from(CHANNEL_VERSION), pinned, "CHANNEL_VERSION and the shared cross-language pin disagree — bump both, plus APP_CHANNEL_VERSION in 🟦️.ts");
}

/// @emoji 🔗️ Cross-language drift guard for the M2 transaction variants (tags 17-21/15-18): the
/// two JSON files under `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS
/// twin (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too —
/// a change to either side's encode/decode that shifts these bytes fails on exactly one side.
#[semio_framework_async_macros::async_test]
async fn channel_transaction_fixtures_match_shared_cross_language_json_vectors() {
    let command_json = include_str!("../../../../../🧫️fixtures/📡️channel/🧾️app-command-transaction.json");
    let frame_json = include_str!("../../../../../🧫️fixtures/📡️channel/📨️app-frame-transaction.json");
    let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("🧾️app-command-transaction.json must parse");
    let frame_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(frame_json).expect("📨️app-frame-transaction.json must parse");
    assert_eq!(command_vectors.len(), 6, "🧾️app-command-transaction.json vector count changed");
    assert_eq!(frame_vectors.len(), 4, "📨️app-frame-transaction.json vector count changed");

    for (label, value) in channel_command_fixture_corpus().await {
        if let Some(expected) = command_vectors.get(label) {
            let actual = hex_encode(&encode_fixture_command(&value).await).await;
            assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
        }
    }
    for (label, value) in channel_frame_fixture_corpus().await {
        if let Some(expected) = frame_vectors.get(label) {
            let actual = hex_encode(&encode_app_frame(&value).await).await;
            assert_eq!(&actual, expected, "AppFrame::{label} drifted from the shared cross-language fixture");
        }
    }
}

/// @emoji 🔗️ Cross-language drift guard for the C3 opening variants (tags 22-24): the JSON file
/// under `🧫️fixtures/📡️channel/` is the single source of truth this codec's TS twin
/// (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — no
/// `AppFrame` variants were added for opening, so only the command-side vector file exists.
#[semio_framework_async_macros::async_test]
async fn channel_opening_fixtures_match_shared_cross_language_json_vectors() {
    let command_json = include_str!("../../../../../🧫️fixtures/📡️channel/🚪️app-command-opening.json");
    let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("🚪️app-command-opening.json must parse");
    assert_eq!(command_vectors.len(), 4, "🚪️app-command-opening.json vector count changed");

    for (label, value) in channel_command_fixture_corpus().await {
        if let Some(expected) = command_vectors.get(label) {
            let actual = hex_encode(&encode_fixture_command(&value).await).await;
            assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
        }
    }
}

/// @emoji 🔗️ Cross-language drift guard for the C8 merge-policy/conflict variants (tags 25-27,
/// 19-20) plus the extended `Invocation`/`Error` frames: the two JSON files under
/// `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS twin
/// (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — see
/// contract-freeze.md §C8 of
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
#[semio_framework_async_macros::async_test]
async fn channel_merge_fixtures_match_shared_cross_language_json_vectors() {
    let command_json = include_str!("../../../../../🧫️fixtures/📡️channel/🔀️app-command-merge.json");
    let frame_json = include_str!("../../../../../🧫️fixtures/📡️channel/📢️app-frame-merge.json");
    let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("🔀️app-command-merge.json must parse");
    let frame_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(frame_json).expect("📢️app-frame-merge.json must parse");
    assert_eq!(command_vectors.len(), 3, "🔀️app-command-merge.json vector count changed");
    assert_eq!(frame_vectors.len(), 4, "📢️app-frame-merge.json vector count changed");

    for (label, value) in channel_command_fixture_corpus().await {
        if let Some(expected) = command_vectors.get(label) {
            let actual = hex_encode(&encode_fixture_command(&value).await).await;
            assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
        }
    }
    for (label, value) in channel_frame_fixture_corpus().await {
        if let Some(expected) = frame_vectors.get(label) {
            let actual = hex_encode(&encode_app_frame(&value).await).await;
            assert_eq!(&actual, expected, "AppFrame::{label} drifted from the shared cross-language fixture");
        }
    }
}
//#endregion 🔖️Corpus

//#region 📥️CommandIngressPages
/// 📥️ The cross-language paged command-ingress contract — the same rows the TypeScript host writer's
/// twin reads (`🎭️actor/📮️shard-client/🧪️tests/📥️command-ingress-pages/🟦️.ts`).
#[derive(serde::Deserialize)]
struct CommandIngressPagesFixture {
    #[serde(rename = "pageBytes")]
    page_bytes: usize,
    #[serde(rename = "commandMaximumBytes")]
    command_maximum_bytes: usize,
    #[serde(rename = "maximumPages")]
    maximum_pages: usize,
    rows: Vec<CommandIngressPagesRow>,
    refusals: Vec<CommandIngressPagesRefusal>,
}

#[derive(serde::Deserialize)]
struct CommandIngressPagesRow {
    name: String,
    bytes: usize,
    pages: usize,
}

#[derive(serde::Deserialize)]
struct CommandIngressPagesRefusal {
    name: String,
    bytes: usize,
}

fn command_ingress_pages_fixture() -> CommandIngressPagesFixture {
    serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🧫️fixtures/📥️command-ingress-pages/🔣️.json")).expect("📥️command-ingress-pages fixture parses")
}

/// 🧱️ A deterministic command body — every byte a function of its offset, so a reassembly that
/// duplicates, drops or reorders one page cannot compare equal.
fn command_ingress_body(bytes: usize) -> Vec<u8> {
    (0..bytes).map(|offset| (offset % 251 + 1) as u8).collect()
}

/// ⚖️ LAW: the transport's page and byte authorities are DERIVED from the guest linear-memory budget,
/// and the spine that assembles a ceiling-sized command stays inside one guest growth unit.
///
/// 🧨️ This is the law the 64-page constant failed on both sides at once: it refused a 272 089-byte
/// contributions pack outright, and the reservation it bought was 262 272 contiguous bytes — four
/// times what a fragmented guest can be relied on to serve (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_command_ingress_authorities_are_derived_from_the_guest_memory_budget() {
    let fixture = command_ingress_pages_fixture();
    assert_eq!(COMMAND_PAGE_MAXIMUM_BYTES, fixture.page_bytes);
    assert_eq!(COMMAND_MAXIMUM_BYTES, fixture.command_maximum_bytes);
    assert_eq!(COMMAND_MAXIMUM_PAGES, fixture.maximum_pages);
    assert_eq!(COMMAND_MAXIMUM_BYTES, semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES, "a command is an assembled host answer and is bound by that budget, not by one of its own");
    assert_eq!(COMMAND_MAXIMUM_PAGES, COMMAND_MAXIMUM_BYTES / COMMAND_PAGE_MAXIMUM_BYTES);
    assert!(
        CommandPageSet::reservation_bytes(COMMAND_MAXIMUM_PAGES) <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "a ceiling-sized page authority reserves {} B — over the {} B a routine per-command guest path may ask a fixed linear memory for",
        CommandPageSet::reservation_bytes(COMMAND_MAXIMUM_PAGES),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
    );
    eprintln!(
        "[DEBUG] command ingress authorities: pages={COMMAND_MAXIMUM_PAGES} bytes={COMMAND_MAXIMUM_BYTES} slot={} B spine={} B ceiling={} B",
        size_of::<FixedCommandPage>(),
        CommandPageSet::reservation_bytes(COMMAND_MAXIMUM_PAGES),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
    );
    assert!(
        size_of::<FixedCommandPage>() < COMMAND_PAGE_MAXIMUM_BYTES,
        "a page slot must be a handle to its block, never the block — an inline page is what forced a fixed page ceiling"
    );
}

/// ⚖️ LAW: every fixture row splits into exactly the pages it declares and reassembles to the EXACT
/// bytes it was cut from, with no page of a multi-page command left short of the page extent.
#[test]
fn every_command_ingress_row_reassembles_to_the_exact_bytes_it_was_cut_from() {
    let fixture = command_ingress_pages_fixture();
    for row in &fixture.rows {
        let body = command_ingress_body(row.bytes);
        let cut: Vec<&[u8]> = body.chunks(COMMAND_PAGE_MAXIMUM_BYTES).collect();
        assert_eq!(cut.len(), row.pages, "row {} cuts into {} pages, not the {} it declares", row.name, cut.len(), row.pages);
        let mut pages = CommandPageSet::try_new(row.pages).unwrap_or_else(|fault| panic!("row {} declares a page authority: {fault:?}", row.name));
        assert!(
            CommandPageSet::reservation_bytes(row.pages) <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
            "row {} reserves {} B for its spine",
            row.name,
            CommandPageSet::reservation_bytes(row.pages)
        );
        for (index, chunk) in cut.iter().enumerate() {
            assert!(index + 1 == cut.len() || chunk.len() == COMMAND_PAGE_MAXIMUM_BYTES, "row {} left a nonterminal page short", row.name);
            pages.try_push(FixedCommandPage::try_copy_from(chunk).expect("a cut page")).unwrap_or_else(|(fault, _)| panic!("row {} admits page {index}: {fault:?}", row.name));
        }
        let command = PagedCommand::try_from_pages(pages).unwrap_or_else(|(fault, _)| panic!("row {} assembles: {fault:?}", row.name));
        assert_eq!(command.page_len(), row.pages);
        assert_eq!(command.byte_len(), row.bytes);
        assert_eq!(command.kind(), body[0]);
        let mut reader = PagedCommandReader::new(command);
        let mut read = Vec::with_capacity(row.bytes);
        for offset in 0..row.bytes {
            read.push(reader.read_byte().unwrap_or_else(|fault| panic!("row {} ends inside byte {offset}: {fault:?}", row.name)));
        }
        assert_eq!(read, body, "row {} did not reassemble to its own bytes", row.name);
        assert!(reader.terminal_is_empty(), "row {} left pages behind after its last byte", row.name);
        assert_eq!(reader.read_byte().expect_err("a fully read command has no further byte").code.0, "plugin.command-decode-truncated");
    }
}

/// ⚖️ LAW: an over-declaration and an over-push are refused with a `Fault` the host can display, and
/// the page a saturated authority refuses is handed BACK rather than dropped.
#[test]
fn every_command_ingress_refusal_answers_a_fault_and_keeps_its_page() {
    let fixture = command_ingress_pages_fixture();
    for refusal in &fixture.refusals {
        let declared = refusal.bytes.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES);
        assert_eq!(
            CommandPageSet::try_new(declared).expect_err(&format!("refusal {} has no page authority", refusal.name)).code.0,
            "plugin.command-page-count",
            "refusal {} must be refused by the declaration, before one page is copied",
            refusal.name
        );
    }
    let mut pages = CommandPageSet::try_new(2).expect("a two-page authority");
    for _ in 0..2 {
        pages.try_push(FixedCommandPage::try_copy_from(&[7; COMMAND_PAGE_MAXIMUM_BYTES]).expect("a full page")).expect("the declared pages are admitted");
    }
    let (fault, returned) = pages.try_push(FixedCommandPage::try_copy_from(b"third").expect("a page")).expect_err("a third page is over the declared authority");
    assert_eq!(fault.code.0, "plugin.command-page-count");
    assert_eq!(returned.as_slice(), b"third", "a refused page is handed back, never dropped");
}
//#endregion 📥️CommandIngressPages
