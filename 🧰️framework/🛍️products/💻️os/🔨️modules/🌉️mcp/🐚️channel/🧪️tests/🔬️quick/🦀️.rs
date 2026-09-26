use super::*;
use crate::bridge::{BridgeFlags, BridgeInstanceRef, ShellToGateway};

fn fixtures() -> Vec<serde_json::Value> {
    serde_json::from_str(include_str!("../../🧫️fixtures/🗿️app-payloads.json")).expect("the shared shell-channel payload fixture parses")
}

fn fixture(kind: &str) -> serde_json::Value {
    fixtures().into_iter().find(|entry| entry.get("kind").and_then(serde_json::Value::as_str) == Some(kind)).unwrap_or_else(|| panic!("fixture `{kind}` exists")).get("payload").cloned().expect("every fixture row carries a payload")
}

fn empty_catalog() -> std::sync::Arc<crate::catalog::Catalog> {
    std::sync::Arc::new(crate::catalog::Catalog { hash: "empty".to_string(), entries: Vec::new() })
}

fn agent_origin() -> MutationOrigin {
    MutationOrigin::Agent { principal: "agent:local".to_string(), invocation_id: "inv_1".to_string() }
}

#[test]
fn base64_round_trips_every_length_class() {
    for length in 0..=64usize {
        let bytes: Vec<u8> = (0..length).map(|index| (index * 7 % 251) as u8).collect();
        let text = encode_base64(&bytes);
        assert_eq!(decode_base64(&text).as_deref(), Some(bytes.as_slice()), "round trip at length {length}");
    }
}

#[test]
fn base64_refuses_malformed_text_instead_of_guessing() {
    assert_eq!(decode_base64("AQI"), None, "a length that is not a multiple of four");
    assert_eq!(decode_base64("A==="), None, "padding that starts before the third symbol");
    assert_eq!(decode_base64("AQ=D"), None, "a symbol after padding");
    assert_eq!(decode_base64("AQ!D"), None, "a symbol outside the alphabet");
}

#[test]
fn every_gateway_to_shell_command_encodes_to_its_shared_fixture() {
    assert_eq!(encode_app_command(&AppCommand::ReadHistory), fixture("readHistory"));
    assert_eq!(encode_app_command(&AppCommand::ReadArtifact), fixture("readArtifact"));
    assert_eq!(encode_app_command(&AppCommand::PureCommand { capability_id: "note.note.appendParagraph".to_string(), input: serde_json::json!({ "text": "hello" }) }), fixture("pureCommand"));
    assert_eq!(
        encode_app_command(&AppCommand::TransactionPrepare { txn_id: "txn_1".to_string(), ops: PreparedOps { document: vec![vec![1, 2, 3]], config: Vec::new(), draft: Vec::new() }, label: "append paragraph".to_string(), origin: agent_origin() }),
        fixture("transactionPrepare")
    );
    assert_eq!(encode_app_command(&AppCommand::TransactionCommit { txn_id: "txn_1".to_string() }), fixture("transactionCommit"));
    assert_eq!(encode_app_command(&AppCommand::TransactionRollback { txn_id: "txn_1".to_string() }), fixture("transactionRollback"));
    assert_eq!(encode_app_command(&AppCommand::TransactionUndo { group_id: "edit_7".to_string() }), fixture("transactionUndo"));
    assert_eq!(encode_app_command(&AppCommand::TransactionRedo { group_id: "edit_7".to_string() }), fixture("transactionRedo"));
    assert_eq!(encode_app_command(&AppCommand::ExportMedia { port: "pdf".to_string(), document: vec![1, 2, 3, 4], document_spr: Vec::new() }), fixture("exportMedia"));
}

#[test]
fn every_shell_to_gateway_frame_decodes_from_its_shared_fixture() {
    assert_eq!(decode_app_frame(&fixture("historySnapshot")), Ok(AppFrame::HistorySnapshot(RevisionStamp { artifact_id: "note-1".to_string(), head_edit_id: "edit_7".to_string(), cursor: "7".to_string() })));
    assert_eq!(decode_app_frame(&fixture("emit")), Ok(AppFrame::Emit { ops: PreparedOps { document: vec![vec![1, 2, 3]], config: Vec::new(), draft: Vec::new() }, warnings: Vec::new() }));
    assert_eq!(decode_app_frame(&fixture("transactionPrepared")), Ok(AppFrame::TransactionPrepared { txn_id: "txn_1".to_string() }));
    assert_eq!(decode_app_frame(&fixture("transactionCommitted")), Ok(AppFrame::TransactionCommitted { txn_id: "txn_1".to_string(), edit_id: "edit_8".to_string(), relay: None }));
    assert_eq!(decode_app_frame(&fixture("transactionRolledBack")), Ok(AppFrame::TransactionRolledBack { txn_id: "txn_1".to_string() }));
    assert_eq!(decode_app_frame(&fixture("transactionUndone")), Ok(AppFrame::TransactionUndone { group_id: "edit_8".to_string() }));
    assert_eq!(decode_app_frame(&fixture("transactionRedone")), Ok(AppFrame::TransactionRedone { group_id: "edit_8".to_string() }));
    assert_eq!(decode_app_frame(&fixture("artifact")), Ok(AppFrame::Artifact { pack: vec![1, 2, 3, 4], spr: vec![5, 6] }));
    assert_eq!(decode_app_frame(&fixture("exported")), Ok(AppFrame::Exported { port: "pdf".to_string(), descriptor: vec![1], data: vec![2, 3] }));
    assert_eq!(decode_app_frame(&fixture("error")), Ok(AppFrame::Error(Fault { code: "mutation.rejected".to_string(), message: "the document is read-only for this actor".to_string() })));
}

#[test]
fn the_fixture_covers_every_variant_both_banks_must_speak() {
    let rows = fixtures();
    let outbound: std::collections::BTreeSet<String> = rows.iter().filter(|row| row.get("direction").and_then(serde_json::Value::as_str) == Some("gateway_to_shell")).filter_map(|row| row.get("kind").and_then(serde_json::Value::as_str).map(str::to_string)).collect();
    let inbound: std::collections::BTreeSet<String> = rows.iter().filter(|row| row.get("direction").and_then(serde_json::Value::as_str) == Some("shell_to_gateway")).filter_map(|row| row.get("kind").and_then(serde_json::Value::as_str).map(str::to_string)).collect();
    assert_eq!(outbound.len(), 9, "every AppCommand variant the shell route carries (Infer is refused before encoding)");
    assert_eq!(inbound.len(), 10, "every AppFrame variant the shell may answer");
    assert!(rows.iter().all(|row| row.get("payload").and_then(|payload| payload.get("version")).and_then(serde_json::Value::as_u64) == Some(u64::from(SHELL_CHANNEL_PAYLOAD_VERSION))), "every payload declares this codec's version");
}

#[test]
fn a_payload_from_another_codec_version_is_refused_not_guessed() {
    let mut payload = fixture("transactionCommitted");
    payload.as_object_mut().unwrap().insert("version".to_string(), serde_json::json!(99));
    let error = decode_app_frame(&payload).expect_err("a future codec version is refused");
    assert!(error.contains("99") && error.contains(&SHELL_CHANNEL_PAYLOAD_VERSION.to_string()), "the refusal names both versions: {error}");
}

#[test]
fn an_unknown_frame_kind_and_a_missing_field_both_answer_a_named_error() {
    let mut unknown = fixture("transactionCommitted");
    unknown.as_object_mut().unwrap().insert("kind".to_string(), serde_json::json!("teleport"));
    assert!(decode_app_frame(&unknown).expect_err("unknown kind").contains("teleport"));
    let mut missing = fixture("transactionCommitted");
    missing.as_object_mut().unwrap().remove("editId");
    assert!(decode_app_frame(&missing).expect_err("missing field").contains("editId"));
}

#[test]
fn a_session_with_no_bridge_resolves_headless_and_stays_headless() {
    let binding = SessionChannelBinding::new(None);
    assert_eq!(binding.decided(), None, "no decision is taken until one is asked for");
    assert_eq!(binding.resolve(), ChannelKind::Headless);
    assert_eq!(binding.decided(), Some(ChannelKind::Headless));
    assert_eq!(binding.resolve(), ChannelKind::Headless, "the decision is sticky");
    assert_eq!(ChannelKind::Headless.label(), "headless");
    assert_eq!(ChannelKind::Shell.label(), "shell");
}

#[test]
fn a_shell_that_does_not_declare_the_relay_is_not_a_shell_route() {
    let handle = std::sync::Arc::new(BridgeHandle::new());
    let (id, _receiver) = handle.register();
    handle.record_hello(id, BridgeFlags::NONE);
    let slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    slot.set(std::sync::Arc::clone(&handle)).ok();
    let binding = SessionChannelBinding::new(Some(slot));
    assert!(binding.relay_connection().is_none(), "a shell that never claimed relayAppCommands cannot answer an AppCommand");
    assert_eq!(binding.resolve(), ChannelKind::Headless);
}

#[test]
fn a_relaying_shell_is_selected_and_reported_as_the_shell_channel() {
    let handle = std::sync::Arc::new(BridgeHandle::new());
    let (id, _receiver) = handle.register();
    handle.record_hello(id, BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: false });
    assert!(handle.shell_flags(id).relay_app_commands);
    let slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    slot.set(std::sync::Arc::clone(&handle)).ok();
    let binding = SessionChannelBinding::new(Some(slot));
    assert_eq!(binding.resolve(), ChannelKind::Shell);
    assert_eq!(binding.resolve().label(), "shell");
}

#[test]
fn an_app_frames_reply_is_recorded_against_its_correlation_id() {
    let handle = BridgeHandle::new();
    let (id, _receiver) = handle.register();
    assert_eq!(handle.last_app_frames(id), None);
    handle.record(id, ShellToGateway::AppFrames { in_reply_to: 41, instance_id: "inst-1".to_string(), frames: vec![b"{}".to_vec()] });
    assert_eq!(handle.last_app_frames(id), Some((41, "inst-1".to_string(), vec![b"{}".to_vec()])));
}

#[test]
fn a_session_that_resolved_shell_refuses_rather_than_editing_a_different_document() {
    let handle = std::sync::Arc::new(BridgeHandle::new());
    let (id, _receiver) = handle.register();
    handle.record_hello(id, BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: false });
    let slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    slot.set(std::sync::Arc::clone(&handle)).ok();
    let binding = std::sync::Arc::new(SessionChannelBinding::new(Some(slot)));
    assert_eq!(binding.resolve(), ChannelKind::Shell);
    let mut channel = ShellArtifactChannel::new(std::sync::Arc::clone(&binding), empty_catalog());
    handle.unregister(id);
    let fault = channel.exchange(0, vec![AppCommand::ReadHistory]).expect_err("the shell is gone");
    assert_eq!(fault.code, "plugin.unavailable");
    assert!(fault.message.contains("no longer attached"), "{}", fault.message);
}

#[test]
fn a_shell_with_no_open_instances_names_what_the_human_must_do() {
    let handle = std::sync::Arc::new(BridgeHandle::new());
    let (id, _receiver) = handle.register();
    handle.record_hello(id, BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: false });
    let slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    slot.set(std::sync::Arc::clone(&handle)).ok();
    let binding = std::sync::Arc::new(SessionChannelBinding::new(Some(slot)));
    let mut channel = ShellArtifactChannel::new(binding, empty_catalog());
    let fault = channel.exchange(0, vec![AppCommand::ReadHistory]).expect_err("no instances");
    assert_eq!(fault.code, "plugin.unavailable");
    assert!(fault.message.contains("open plugin instances"), "{}", fault.message);
}

#[test]
fn a_silent_shell_times_out_into_a_retryable_budget_fault_and_never_hangs() {
    let handle = std::sync::Arc::new(BridgeHandle::new());
    let (id, _receiver) = handle.register();
    handle.record_hello(id, BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: false });
    handle.record(id, ShellToGateway::Instances { entries: vec![BridgeInstanceRef { plugin_id: "note".to_string(), app_id: "note".to_string(), instance_id: "inst-1".to_string(), artifact_ref: "note-1".to_string(), window_ids: vec!["w1".to_string()] }] });
    let slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    slot.set(std::sync::Arc::clone(&handle)).ok();
    let binding = std::sync::Arc::new(SessionChannelBinding::new(Some(slot)));
    let mut channel = ShellArtifactChannel::new(binding, empty_catalog()).with_timeout(Duration::from_millis(60));
    let started = Instant::now();
    let fault = channel.exchange(0, vec![AppCommand::ReadHistory]).expect_err("nothing ever answers");
    assert_eq!(fault.code, "budget.exceeded");
    assert!(started.elapsed() < Duration::from_secs(5), "the wait is bounded, not forever");
}

/// 🎯️ An artifact handle names its own plugin, and that is what a capability-less command resolves
/// an instance from. Measured inside `s` before this rule existed: with a spawned `note` editor open
/// the shell reported 3 instances and `ReadArtifact` refused outright, cascading (f1)…(f8) of the
/// live agent gate (ticket 26/09/18, S6 §5.4).
#[test]
fn a_capability_less_command_resolves_the_one_open_instance_of_its_own_plugin() {
    let handle = std::sync::Arc::new(BridgeHandle::new());
    let (id, _receiver) = handle.register();
    handle.record_hello(id, BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: false });
    handle.record(id, ShellToGateway::Instances {
        entries: vec![
            BridgeInstanceRef { plugin_id: "space".to_string(), app_id: "s.space.home".to_string(), instance_id: "2".to_string(), artifact_ref: "space:s.space.home:2".to_string(), window_ids: vec!["s-home-main".to_string()] },
            BridgeInstanceRef { plugin_id: "note".to_string(), app_id: "s.note.note".to_string(), instance_id: "4".to_string(), artifact_ref: "note:s.note.note:4".to_string(), window_ids: vec!["note-4::note-composite".to_string()] },
        ],
    });
    let slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    slot.set(std::sync::Arc::clone(&handle)).ok();
    let binding = std::sync::Arc::new(SessionChannelBinding::new(Some(slot)));
    let mut channel = ShellArtifactChannel::new(binding, empty_catalog()).for_plugin("note").with_timeout(Duration::from_millis(60));
    // 🎯️ Nothing answers, so the exchange still times out — but it times out having ADDRESSED an
    // instance, which is the whole statement: before the rule it refused `plugin.unavailable` here.
    let fault = channel.exchange(0, vec![AppCommand::ReadArtifact]).expect_err("nothing answers");
    assert_eq!(fault.code, "budget.exceeded", "{}", fault.message);
}

/// 🎯️ Several open instances of the artifact's own plugin is genuinely ambiguous, and an ambiguous
/// handle is refused BY NAME rather than resolved silently — picking one would edit a document the
/// agent never named.
#[test]
fn an_ambiguous_artifact_handle_is_refused_by_name_with_its_candidates() {
    let handle = std::sync::Arc::new(BridgeHandle::new());
    let (id, _receiver) = handle.register();
    handle.record_hello(id, BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: false });
    handle.record(id, ShellToGateway::Instances {
        entries: vec![
            BridgeInstanceRef { plugin_id: "note".to_string(), app_id: "s.note.note".to_string(), instance_id: "4".to_string(), artifact_ref: "note:s.note.note:4".to_string(), window_ids: vec![] },
            BridgeInstanceRef { plugin_id: "note".to_string(), app_id: "s.note.note".to_string(), instance_id: "9".to_string(), artifact_ref: "note:s.note.note:9".to_string(), window_ids: vec![] },
        ],
    });
    let slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    slot.set(std::sync::Arc::clone(&handle)).ok();
    let binding = std::sync::Arc::new(SessionChannelBinding::new(Some(slot)));
    let mut channel = ShellArtifactChannel::new(binding, empty_catalog()).for_plugin("note");
    let fault = channel.exchange(0, vec![AppCommand::ReadArtifact]).expect_err("two candidates");
    assert_eq!(fault.code, "plugin.unavailable");
    assert!(fault.message.contains("note:s.note.note:4"), "{}", fault.message);
    assert!(fault.message.contains("note:s.note.note:9"), "{}", fault.message);
}

#[test]
fn inference_never_travels_the_shell_route() {
    let binding = std::sync::Arc::new(SessionChannelBinding::new(None));
    let mut channel = ShellArtifactChannel::new(binding, empty_catalog());
    let fault = channel.exchange(0, vec![AppCommand::Infer(crate::actions::InferCommand::default())]).expect_err("inference is refused");
    assert_eq!(fault.code, "plugin.unavailable");
    assert!(fault.message.contains("inference"), "{}", fault.message);
}

#[test]
fn the_port_contract_of_one_command_per_exchange_is_enforced_here_too() {
    let binding = std::sync::Arc::new(SessionChannelBinding::new(None));
    let mut channel = ShellArtifactChannel::new(binding, empty_catalog());
    let fault = channel.exchange(0, vec![AppCommand::ReadHistory, AppCommand::ReadArtifact]).expect_err("two commands");
    assert_eq!(fault.code, "channel.not-wired");
    assert!(fault.message.contains("exactly one command"), "{}", fault.message);
}
