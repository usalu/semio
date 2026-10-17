/// 📚️ Fixture corpus laws: the golden command and frame corpus, the channel version pin and the shared cross-language vectors of the transaction, opening, merge, media export and document identity fixtures.

//#region 🔖️Corpus
// Cross-language drift fixture: a sibling TypeScript work package duplicates these exact hex
// strings in a vitest suite, so `AppCommand`/`AppFrame` and the TS-side codec they hand-port
// stay byte-exact. Every entry is `(variant label, value)`; `channel_command_fixture_hex`/
// `channel_frame_fixture_hex` below are this codec's own committed golden hex per label —
// sourced from `encode_app_command`/`encode_app_frame`'s actual output, not hand-computed.

/// 🧾️ Named `AppCommand` fixture corpus, one entry per variant.
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
        ("ReadDocument", AppCommand::ReadDocument { seq: 1 }),
        ("LoadConfig", AppCommand::LoadConfig { seq: 1, pack: vec![1], spr: vec![2] }),
        ("ReadConfig", AppCommand::ReadConfig { seq: 1 }),
        ("LoadWindowConfig", AppCommand::LoadWindowConfig { seq: 1, entry: WindowConfigPackEntry { window_id: "w1".into(), window_kind_id: "graph".into(), envelope_pack: vec![1, 2] } }),
        ("ReadWindowConfigs", AppCommand::ReadWindowConfigs { seq: 1 }),
        ("MediaIn", AppCommand::MediaIn { seq: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
        ("MediaOut", AppCommand::MediaOut { seq: 1, port: "p".to_string(), request: vec![1] }),
        ("MediaFingerprint", AppCommand::MediaFingerprint { seq: 1, port: "p".to_string() }),
        ("PureCommand", AppCommand::PureCommand { seq: 1, command: vec![1], head: vec![2] }),
        ("LoadChildren", AppCommand::LoadChildren { seq: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1], owner: String::new() }] }),
        ("LoadChildrenOfAMember", AppCommand::LoadChildren { seq: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1], owner: "content/forms-1".to_string() }] }),
        ("ReadChildren", AppCommand::ReadChildren { seq: 1 }),
        ("ReadChildHeads", AppCommand::ReadChildHeads { seq: 1 }),
        ("ReadHistory", AppCommand::ReadHistory { seq: 1 }),
        ("TransactionPrepareOwner", AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), origin: Vec::new(), prepared_child_ops: Vec::new() }),
        ("TransactionPreparePrePlanned", AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], origin: vec![9], prepared_child_ops: Vec::new() }),
        ("TransactionPreparePrePlannedChildren", AppCommand::TransactionPrepare { seq: 7, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1]], origin: vec![9], prepared_child_ops: vec![5, 6] }),
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

/// 🧾️ Named `AppFrame` fixture corpus, one entry per variant.
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
        ("Emit", AppFrame::Emit { in_reply_to: 1, document_ops: vec![1], config_ops: vec![], draft_ops: vec![], output: vec![2], diagnostics: vec![], child_ops: vec![] }),
        ("Draft", AppFrame::Draft { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "d".to_string() }),
        ("Children", AppFrame::Children { in_reply_to: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1], owner: String::new() }] }),
        ("ChildrenOfAMember", AppFrame::Children { in_reply_to: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1], owner: "content/forms-1".to_string() }] }),
        ("Ephemeral", AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![7], tool_run: vec![8], history_edit: vec![9] }),
        ("HistorySnapshot", AppFrame::HistorySnapshot { in_reply_to: 1, history_patch: vec![1] }),
        ("TransactionProposal", AppFrame::TransactionProposal { in_reply_to: 1, proposal_id: "p".to_string(), local_ops: vec![vec![1]], foreign: Vec::new() }),
        ("TransactionPrepared", AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: vec![vec![1]], rejection: Vec::new() }),
        ("TransactionCommitted", AppFrame::TransactionCommitted { txn_id: "t".to_string(), edit_id: "e".to_string() }),
        ("TransactionRolledBack", AppFrame::TransactionRolledBack { txn_id: "t".to_string() }),
        ("MergeReport", AppFrame::MergeReport { in_reply_to: Some(1), report: vec![1] }),
        ("Conflicts", AppFrame::Conflicts { in_reply_to: None, conflicts: vec![2] }),
        ("UiPatch", AppFrame::UiPatch { in_reply_to: Some(1), surface: "1:body".to_string(), kind: "window".to_string(), revision: 2, base_revision: 1, ops: vec![3] }),
        ("UiSnapshotEnd", AppFrame::UiSnapshotEnd { revision: 4 }),
        ("OperationCompleted", AppFrame::OperationCompleted { operation: 7, revision: 5, ui_scope: vec![1], history_patch: vec![2] }),
        ("ChildHeads", AppFrame::ChildHeads { in_reply_to: 1, entries: vec![ChildHeadPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), head_pack: vec![1], owner: String::new() }] }),
        ("ChildHeadsOfAMember", AppFrame::ChildHeads { in_reply_to: 1, entries: vec![ChildHeadPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), head_pack: vec![1], owner: "content/forms-1".to_string() }] }),
    ]
}

/// 🔒️ Golden hex per `AppCommand` fixture-corpus label — sourced by actually running
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
        "ReadDocument" => "0701",
        "LoadConfig" => "080101010102",
        "ReadConfig" => "0901",
        "LoadWindowConfig" => "1e01027731056772617068020102",
        "ReadWindowConfigs" => "1f01",
        "MediaIn" => "0a01017001010102",
        "MediaOut" => "0b0101700101",
        "MediaFingerprint" => "0c010170",
        "PureCommand" => "0d0101010102",
        "LoadChildren" => "0e0101017301630164010100",
        "LoadChildrenOfAMember" => "0e010101730163016401010f636f6e74656e742f666f726d732d31",
        "ReadChildren" => "0f01",
        "ReadChildHeads" => "2a01",
        "ReadHistory" => "1001",
        "TransactionPrepareOwner" => "11010174016d0109000000",
        "TransactionPreparePrePlanned" => "110201740000020101020202010900",
        "TransactionPreparePrePlannedChildren" => "1107017400000101010109020506",
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

/// 🔒️ Golden hex per `AppFrame` fixture-corpus label — see
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
        "Emit" => "0a010101000001020000",
        "Draft" => "0b01010101020164",
        "Children" => "0c0101017301630164010100",
        "ChildrenOfAMember" => "0c010101730163016401010f636f6e74656e742f666f726d732d31",
        "Ephemeral" => "0d0201020304010701080109",
        "HistorySnapshot" => "0e010101",
        "TransactionProposal" => "0f01017001010100",
        "TransactionPrepared" => "10017401010100",
        "TransactionCommitted" => "1101740165",
        "TransactionRolledBack" => "120174",
        "MergeReport" => "1301010101",
        "Conflicts" => "14000102",
        "UiPatch" => "15010106313a626f64790677696e646f7702010103",
        "UiSnapshotEnd" => "1604",
        "OperationCompleted" => "19070501010102",
        "ChildHeads" => "200101017301630164010100",
        "ChildHeadsOfAMember" => "20010101730163016401010f636f6e74656e742f666f726d732d31",
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
                while !cursor.terminal_is_empty() { let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:1};let step=cursor.close_step(grant);assert!(step.progress().fits(grant));if matches!(step,semio_framework_value::RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;} }
            }
            assert!(entries.iter().map(FixedCommandPage::as_slice).eq(peers.iter()), "Presence retained cursor must preserve exact entry order");
            while !cursor.terminal_is_empty() { let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:1};let step=cursor.close_step(grant);assert!(step.progress().fits(grant));if matches!(step,semio_framework_value::RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;} }
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

/// 📡️ The wire version is owned by `🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json`, not by
/// either language's constant, so a bump that updates only one host fails here instead of at
/// runtime — the drift this guard was added for was a live `APP_CHANNEL_VERSION = 8` in
/// TypeScript against `CHANNEL_VERSION = 10` in Rust. The TS twin asserts the same file.
#[semio_framework_async_macros::async_test]
async fn channel_version_matches_the_shared_cross_language_pin() {
    let json = include_str!("../../../../../🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json");
    let pin: serde_json::Value = serde_json::from_str(json).expect("🔖️channel-version.json must parse");
    let pinned = pin.get("channelVersion").and_then(serde_json::Value::as_u64).expect("🔖️channel-version.json must carry channelVersion");
    assert_eq!(u64::from(CHANNEL_VERSION), pinned, "CHANNEL_VERSION and the shared cross-language pin disagree — bump both, plus APP_CHANNEL_VERSION in 🟦️.ts");
}

/// 🤝️ LAW (`🧫️fixtures/🧫️channel-handshake/🔣️.json`): a host admits a guest only at its own app-channel version — older and
/// newer guests are refused with `plugin.channel-mismatch`, naming both versions as notice placeholders. The TS twin
/// (`admitGuestChannelVersion`) replays the same corpus.
#[semio_framework_async_macros::async_test]
async fn a_host_admits_only_a_guest_of_its_own_channel_version() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️channel-handshake/🔣️.json")).unwrap();
    assert_eq!(law["code"].as_str(), Some(CHANNEL_MISMATCH_CODE));
    for case in law["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (guest_offset, host_offset) = (case["guestOffset"].as_i64().unwrap(), case["hostOffset"].as_i64().unwrap_or(0));
        assert_eq!(case["admitted"].as_bool().unwrap(), guest_offset == host_offset, "{name}: the corpus admits exactly a guest of its host's version");
        let guest = u32::try_from(i64::from(CHANNEL_VERSION) + guest_offset).unwrap();
        let host = u32::try_from(i64::from(CHANNEL_VERSION) + host_offset).unwrap();
        match admit_guest_channel_version(guest, host) {
            Ok(()) => assert!(case["admitted"].as_bool().unwrap(), "{name}: admitted a guest the law refuses"),
            Err(fault) => {
                assert!(!case["admitted"].as_bool().unwrap(), "{name}: refused a guest the law admits");
                assert_eq!(fault.code.0, CHANNEL_MISMATCH_CODE, "{name}");
                assert_eq!(fault.param("guest"), Some(guest.to_string().as_str()), "{name}");
                assert_eq!(fault.param("host"), Some(host.to_string().as_str()), "{name}");
            }
        }
    }
}

/// 🔗️ Cross-language drift guard for the M2 transaction variants (tags 17-21/15-18): the
/// two JSON files under `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS
/// twin (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too —
/// a change to either side's encode/decode that shifts these bytes fails on exactly one side.
#[semio_framework_async_macros::async_test]
async fn channel_transaction_fixtures_match_shared_cross_language_json_vectors() {
    let command_json = include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/🧾️app-command-transaction.json");
    let frame_json = include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/📨️app-frame-transaction.json");
    let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("🧾️app-command-transaction.json must parse");
    let frame_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(frame_json).expect("📨️app-frame-transaction.json must parse");
    assert_eq!(command_vectors.len(), 7, "🧾️app-command-transaction.json vector count changed");
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

/// 🔗️ Cross-language drift guard for the C3 opening variants (tags 22-24): the JSON file
/// under `🧫️fixtures/📡️channel/` is the single source of truth this codec's TS twin
/// (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — no
/// `AppFrame` variants were added for opening, so only the command-side vector file exists.
#[semio_framework_async_macros::async_test]
async fn channel_opening_fixtures_match_shared_cross_language_json_vectors() {
    let command_json = include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/🚪️app-command-opening.json");
    let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("🚪️app-command-opening.json must parse");
    assert_eq!(command_vectors.len(), 4, "🚪️app-command-opening.json vector count changed");

    for (label, value) in channel_command_fixture_corpus().await {
        if let Some(expected) = command_vectors.get(label) {
            let actual = hex_encode(&encode_fixture_command(&value).await).await;
            assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
        }
    }
}

/// 🔗️ Cross-language drift guard for the C8 merge-policy/conflict variants (tags 25-27,
/// 19-20) plus the extended `Invocation`/`Error` frames: the two JSON files under
/// `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS twin
/// (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — see
/// contract-freeze.md §C8 of
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
#[semio_framework_async_macros::async_test]
async fn channel_merge_fixtures_match_shared_cross_language_json_vectors() {
    let command_json = include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/🔀️app-command-merge.json");
    let frame_json = include_str!("../../../../../../🧪️tests/🧫️fixtures/📡️channel/📢️app-frame-merge.json");
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

#[semio_framework_async_macros::async_test]
async fn media_export_wire_matches_the_language_neutral_fixture_above_number_safe_range() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎬️media-export-wire/🔣️.json")).expect("media export wire fixture parses");
    assert_eq!(fixture["channelVersion"].as_u64().unwrap(), CHANNEL_VERSION as u64);
    let handle = MediaExportHandleWire {
        app_instance_id: 42,
        parent_document_id: "doc-α".into(),
        operation_id: 9_007_199_254_740_995,
        base_revision: 9_007_199_254_740_993,
        generation: u64::MAX,
    };
    let commands = [
        ("submit", AppCommand::SubmitMediaExport { seq: 9_007_199_254_740_993, port: "playback:out".into(), expected_parent_document_id: "doc-α".into(), expected_base_revision: 9_007_199_254_740_993 }),
        ("poll", AppCommand::PollMediaExport { seq: 9_007_199_254_740_994, handle: handle.clone() }),
        ("cancel", AppCommand::CancelMediaExport { seq: 9_007_199_254_740_995, handle: handle.clone() }),
        ("takeChunk", AppCommand::TakeMediaExportChunk { seq: 9_007_199_254_740_996, handle: handle.clone() }),
    ];
    for (name, command) in commands {
        let encoded = encode_app_command(&command).await.expect("media export command encodes");
        let bytes = encoded.front_page().expect("fixture command has one page").as_slice();
        assert_eq!(hex_encode(bytes).await, fixture["commands"][name]["hex"].as_str().expect("command hex"));
        assert_eq!(decode_app_command(bytes).await.expect("media export command decodes"), command);
        let mut paged = PagedAppCommandDecodeCursor::new(encoded);
        let decoded = loop {
            if let Some(decoded) = paged.step().expect("media export paged decode step") {
                break decoded;
            }
        };
        assert_eq!(decoded, command);
    }
    let frames = [
        ("submitted", AppFrame::MediaExportSubmitted { in_reply_to: 9_007_199_254_740_993, handle: handle.clone() }),
        ("status", AppFrame::MediaExportStatus { in_reply_to: 9_007_199_254_740_994, handle: handle.clone(), state: MediaExportStateWire::Complete, applied_progress: 4096, checkpoint_available: true, mime_type: "video/mp4".into(), total_bytes: 9_007_199_254_740_997, detail: String::new() }),
        ("chunk", AppFrame::MediaExportChunk { in_reply_to: 9_007_199_254_740_996, handle: handle.clone(), data: vec![0, 255, 1, 128], terminal: false }),
        ("terminalChunk", AppFrame::MediaExportChunk { in_reply_to: 9_007_199_254_740_997, handle, data: Vec::new(), terminal: true }),
    ];
    for (name, frame) in frames {
        let bytes = encode_app_frame(&frame).await;
        assert_eq!(hex_encode(&bytes).await, fixture["frames"][name]["hex"].as_str().expect("frame hex"));
        assert_eq!(decode_app_frame(&bytes).await.expect("media export frame decodes"), frame);
    }
}


/// 🪪️ The scalar owner query roundtrips the shared wire vectors without a document archive.
#[semio_framework_async_macros::async_test]
async fn document_identity_wire_matches_the_language_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-identity-wire/🔣️.json")).unwrap();
    assert_eq!(fixture["channelVersion"].as_u64().unwrap(), CHANNEL_VERSION as u64);
    assert!(serde_json::from_value::<AppDocumentIdentity>(serde_json::json!({ "appInstanceId": 17 })).is_err());
    assert!(serde_json::from_value::<AppDocumentIdentity>(serde_json::json!({ "appInstanceId": 17, "parentDocumentId": null, "claimedControllerId": "foreign" })).is_err());
    for row in fixture["cases"].as_array().unwrap() {
        let seq = row["seq"].as_str().unwrap().parse::<u64>().unwrap();
        let identity: AppDocumentIdentity = serde_json::from_value(row["identity"].clone()).unwrap();
        let command = AppCommand::ReadDocumentIdentity { seq };
        let encoded = encode_app_command(&command).await.unwrap();
        let command_bytes: Vec<u8> = serde_json::from_value(row["commandBytes"].clone()).unwrap();
        assert_eq!(encoded.front_page().unwrap().as_slice(), command_bytes);
        assert_eq!(decode_app_command(&command_bytes).await.unwrap(), command);
        let mut paged = PagedAppCommandDecodeCursor::new(encoded);
        let decoded = loop { if let Some(decoded) = paged.step().unwrap() { break decoded; } };
        assert_eq!(decoded, command);
        let frame = AppFrame::DocumentIdentity { in_reply_to: seq, identity };
        let frame_bytes: Vec<u8> = serde_json::from_value(row["frameBytes"].clone()).unwrap();
        assert_eq!(encode_app_frame(&frame).await, frame_bytes);
        assert_eq!(decode_app_frame(&frame_bytes).await.unwrap(), frame);
        let mut trailing = frame_bytes.clone();
        trailing.push(0);
        assert!(decode_app_frame(&trailing).await.is_err());
    }
    for bytes in [vec![31, 0, 0, 2], vec![31, 0, 0, 1, 0], vec![31, 0, 255, 255, 255, 255, 31, 0]] {
        assert!(decode_app_frame(&bytes).await.is_err());
    }
    let oversized = AppFrame::DocumentIdentity { in_reply_to: 0, identity: AppDocumentIdentity { app_instance_id: 0, parent_document_id: Some("x".repeat(513)) } };
    assert!(decode_app_frame(&encode_app_frame(&oversized).await).await.is_err());
}
