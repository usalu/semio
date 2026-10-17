/// 🎞️ App frame laws: the AppFrame round trips, the child pack and head commands and frames, the transaction frames, the merge report, conflicts and the UI patch and snapshot frames.

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
    protocol::write_varint_u64(&mut encoded, 2);
    for _ in 0..5 {
        protocol::write_bytes(&mut encoded, &[]);
    }
    protocol::write_bytes(&mut encoded, &vec![0; INVOCATION_RESULT_PACK_MAXIMUM_BYTES + 1]);
    protocol::write_bytes(&mut encoded, &[]);
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
        status: DocumentArchiveLoadStatus { operation: 10, state: DocumentArchiveLoadState::Running, completed: 3, total: 9, ahead: 4, fault: Vec::new() },
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/🪟️window-config.json")).expect("window config fixture parses");
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
    assert_frame_round_trips(&AppFrame::Emit { in_reply_to: 14, document_ops: vec![1], config_ops: vec![2], draft_ops: vec![3], output: vec![4], diagnostics: vec![5], child_ops: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Emit { in_reply_to: 15, document_ops: Vec::new(), config_ops: Vec::new(), draft_ops: Vec::new(), output: vec![4], diagnostics: Vec::new(), child_ops: vec![6, 7, 8] }).await;
}

#[semio_framework_async_macros::async_test]
async fn app_frame_draft_round_trips() {
    assert_frame_round_trips(&AppFrame::Draft { in_reply_to: 15, pack: vec![1], spr: vec![2], ops: "d".to_string() }).await;
    assert_frame_round_trips(&AppFrame::Children { in_reply_to: 16, entries: sample_child_entries().await }).await;
    assert_frame_round_trips(&AppFrame::Children { in_reply_to: 17, entries: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![9, 9], tool_run: Vec::new(), history_edit: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: Vec::new(), tool_run: vec![5, 6, 7], history_edit: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: Vec::new(), presence_generation: 0, transient_generation: 0, interaction: Vec::new(), tool_run: Vec::new(), history_edit: vec![5, 101, 45, 50, 35, 48, 2, 1] }).await;
}

//#region 🔖️Children
/// 🧸️ Two children in different slots, one of them with an empty pack (a genesis child whose
/// envelope has not been printed yet), so the list codec is exercised at both extremes.
async fn sample_child_entries() -> Vec<ChildPackEntry> {
    vec![
        ChildPackEntry { slot: "mesh".to_string(), child_id: "child-1".to_string(), dialect: "s.stdio.mesh@1/*".to_string(), envelope_pack: vec![7, 8, 9], owner: String::new() },
        ChildPackEntry { slot: "brep".to_string(), child_id: "child-2".to_string(), dialect: "s.stdio.brep@1/*".to_string(), envelope_pack: Vec::new(), owner: "content/forms-1".to_string() },
    ]
}

#[semio_framework_async_macros::async_test]
async fn child_pack_commands_round_trip() {
    assert_command_round_trips(&AppCommand::LoadChildren { seq: 19, entries: sample_child_entries().await }).await;
    assert_command_round_trips(&AppCommand::LoadChildren { seq: 20, entries: Vec::new() }).await;
    assert_command_round_trips(&AppCommand::ReadChildren { seq: 21 }).await;
    assert_command_round_trips(&AppCommand::ReadHistory { seq: 22 }).await;
}

/// 🪆️ `ReadChildHeads` → `ChildHeads` round-trips through both the flat codec and the guest's paged ingress, a head list keeps
/// every entry (an empty head pack included), and a declared count past the member authority is refused before it reserves.
#[semio_framework_async_macros::async_test]
async fn child_head_commands_and_frames_round_trip() {
    assert_command_round_trips(&AppCommand::ReadChildHeads { seq: 23 }).await;
    assert_paged_route_admits(&AppCommand::ReadChildHeads { seq: 24 }).await;
    let entries = vec![
        ChildHeadPackEntry { slot: "mesh".to_string(), child_id: "child-1".to_string(), dialect: "s.stdio.mesh@1/*".to_string(), head_pack: vec![7, 8, 9], owner: String::new() },
        ChildHeadPackEntry { slot: "brep".to_string(), child_id: "child-2".to_string(), dialect: "s.stdio.brep@1/*".to_string(), head_pack: Vec::new(), owner: "content/forms-1".to_string() },
    ];
    assert_frame_round_trips(&AppFrame::ChildHeads { in_reply_to: 23, entries }).await;
    assert_frame_round_trips(&AppFrame::ChildHeads { in_reply_to: 24, entries: Vec::new() }).await;
    let mut oversized = vec![32, 1];
    protocol::write_varint_u64(&mut oversized, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 + 1);
    assert!(matches!(decode_app_frame(&oversized).await, Err(protocol::ProtocolError::Malformed { what: "channel child heads", .. })));
}
//#endregion 🔖️Children

//#region 🔖️Transaction
#[semio_framework_async_macros::async_test]
async fn app_frame_transaction_proposal_round_trips() {
    assert_frame_round_trips(&AppFrame::TransactionProposal { in_reply_to: 1, proposal_id: "p".to_string(), local_ops: vec![vec![1]], foreign: Vec::new() }).await;
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/🏁️app-frame-operation-completed.json")).expect("operation completion fixture parses");
    let frame = AppFrame::OperationCompleted { operation: 7, revision: 5, ui_scope: vec![1], history_patch: vec![2] };
    assert_eq!(hex_encode(&encode_app_frame(&frame).await).await, fixture["OperationCompleted"].as_str().expect("frame fixture hex"));
    let wide = AppFrame::OperationCompleted { operation: 7, revision: 0xfedc_ba98_7654_3210, ui_scope: vec![1], history_patch: vec![2] };
    assert_eq!(hex_encode(&encode_app_frame(&wide).await).await, fixture["OperationCompletedWideRevision"].as_str().expect("wide frame fixture hex"));
    let wide_operation = AppFrame::OperationCompleted { operation: u64::MAX, revision: 5, ui_scope: vec![1], history_patch: vec![2] };
    assert_eq!(hex_encode(&encode_app_frame(&wide_operation).await).await, fixture["OperationCompletedWideOperation"].as_str().expect("wide operation fixture hex"));
}
//#endregion 🔖️UiPatch
//#endregion 🔖️AppFrame
