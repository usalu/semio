/// 🚫️ Codec refusal laws: encoding determinism, the refusals of empty, unknown, truncated and trailing bytes, the presence roster limits and the paged generic decoder laws.

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
    assert!(matches!(err, protocol::ProtocolError::Malformed { what: "channel app-command tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_app_frame_rejects_empty_bytes() {
    let err = decode_app_frame(&[]).await.unwrap_err();
    assert!(matches!(err, protocol::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_app_command_rejects_unknown_tag() {
    let err = decode_app_command(&[0xFF]).await.unwrap_err();
    assert!(matches!(err, protocol::ProtocolError::Malformed { what: "channel app-command tag", .. }));
}

#[semio_framework_async_macros::async_test]
async fn decode_app_frame_rejects_unknown_tag() {
    let err = decode_app_frame(&[0xFF]).await.unwrap_err();
    assert!(matches!(err, protocol::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
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
    while !cursor.terminal_is_empty() { let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:1};let step=cursor.close_step(grant);assert!(step.progress().fits(grant));if matches!(step,semio_framework_value::RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;} }
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
        (AppCommand::ReadDocument { seq: 2 }, 2),
        (AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }, 28),
        (AppCommand::MergeDocumentArchive { seq: 13, archive: sample_document_archive() }, 28),
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

/// 🧲️ `MergeDocumentArchive` (tag 43, CHANNEL_VERSION 22, design §22.22) carries its archive in exactly the
/// `LoadDocumentArchive` encoding: the two commands differ in their tag byte alone, on the flat and on the paged route.
#[semio_framework_async_macros::async_test]
async fn a_merge_archive_command_is_the_load_encoding_under_its_own_tag() {
    let load = encode_fixture_command(&AppCommand::LoadDocumentArchive { seq: 14, archive: sample_document_archive() }).await;
    let merge = encode_fixture_command(&AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() }).await;
    assert_eq!((load[0], merge[0]), (32, 43));
    assert_eq!(load[1..], merge[1..]);
    assert_eq!(decode_app_command(&merge).await.unwrap(), AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() });
    let mut cursor = PagedAppCommandDecodeCursor::new(encode_app_command(&AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() }).await.unwrap());
    let mut decoded = None;
    for _ in 0..64 {
        decoded = cursor.step().unwrap();
        if decoded.is_some() {
            break;
        }
    }
    assert_eq!(decoded, Some(AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() }));
    assert!(cursor.terminal_is_empty());
}

/// ⏭️ The shared cross-language vectors of the merge rider (`🧫️fixtures/📡️channel/🧲️document-archive-merge.json`): both
/// archive commands and the load status with its `ahead` varint after `total`, byte for byte, in both directions.
#[semio_framework_async_macros::async_test]
async fn document_archive_merge_matches_shared_cross_language_json_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/🧲️document-archive-merge.json")).expect("document archive merge fixture parses");
    let bytes = |value: &serde_json::Value| value.as_array().expect("bytes").iter().map(|byte| byte.as_u64().expect("byte") as u8).collect::<Vec<u8>>();
    let seq = fixture["seq"].as_u64().expect("seq");
    let archive = || DocumentArchivePack { parent_pack: bytes(&fixture["archive"]["parent_pack"]), parent_spr: bytes(&fixture["archive"]["parent_spr"]), members: Vec::new() };
    for (name, command) in [("LoadDocumentArchive", AppCommand::LoadDocumentArchive { seq, archive: archive() }), ("MergeDocumentArchive", AppCommand::MergeDocumentArchive { seq, archive: archive() })] {
        let encoded = encode_fixture_command(&command).await;
        assert_eq!(hex_encode(&encoded).await, fixture[name].as_str().expect("command fixture hex"), "{name}");
        assert_eq!(decode_app_command(&encoded).await.expect("fixture command decodes"), command, "{name}");
    }
    let status = &fixture["status"];
    assert_eq!(status["state"], "ready");
    let frame = AppFrame::DocumentArchiveLoad { in_reply_to: status["in_reply_to"].as_u64().expect("in_reply_to"), status: DocumentArchiveLoadStatus { operation: status["operation"].as_u64().expect("operation"), state: DocumentArchiveLoadState::Ready, completed: status["completed"].as_u64().expect("completed"), total: status["total"].as_u64().expect("total"), ahead: status["ahead"].as_u64().expect("ahead"), fault: bytes(&status["fault"]) } };
    assert_eq!(hex_encode(&encode_app_frame(&frame).await).await, fixture["DocumentArchiveLoad"].as_str().expect("frame fixture hex"));
    assert_frame_round_trips(&frame).await;
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
    protocol::write_varint_u64(&mut bytes, (DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS + 1) as u64);
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
