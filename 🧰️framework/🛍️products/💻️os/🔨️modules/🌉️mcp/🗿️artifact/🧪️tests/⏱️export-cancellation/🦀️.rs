//! 📤️ Drives the folder workspace's actual Note owner and compares its exported archive to the source pair.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc};
use std::time::Duration;
use semio_os_mcp_capabilities::core_capabilities::build_catalog;
use semio_os_mcp_foundation::errors::GatewayErrorCode;
use semio_os_mcp_plugin_runtime::compiled_components::set_component_compile_worker;
use semio_os_mcp_port::channel_port::ActivationScope;
use semio_os_mcp_shell_channel::shell_channel::decode_base64;
use semio_os_mcp_workspace::headless_workspace_state::HeadlessWorkspace;

#[test]
fn folder_export_cancellation_and_retry_use_actual_note_bytes() {
    set_component_compile_worker(PathBuf::from(env!("CARGO_BIN_EXE_semio-os-mcp")));
    assert!(semio_os_mcp_capabilities::capability_declarations::artifact_capabilities().iter()
        .any(|capability| capability.id.0 == "artifact.export" && capability.execution.cancellable));
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/⏱️export-cancellation-law.json")).expect("neutral export law");
    let dir = semio_os_store_test_support::tempdir().expect("export folder");
    let workspace = Arc::new(HeadlessWorkspace::open_folder(
        dir.path().to_path_buf(), "agent:export-law".into(), Vec::new(), Arc::new(build_catalog()),
        serde_json::from_str(include_str!("../../../🎚️config/🏛️host-driver-policy.json")).expect("host driver policy"),
        serde_json::from_str(include_str!("../../../🎚️config/💡️host-operation-policy.json")).expect("operation policy"),
    ).expect("workspace"));
    let plugin = law["pluginId"].as_str().unwrap();
    let port = law["port"].as_str().unwrap();
    let kind = workspace.installed_artifact_kinds().expect("actual Note descriptor").into_iter()
        .find(|kind| kind.plugin_id == plugin && Some(kind.schema.as_str()) == law["schema"].as_str()).expect("Note owner");
    let artifact_id = "note-seeded-sdk";
    if let Some(seed) = std::env::var_os("SEMIO_MCP_EXPORT_TEST_SEED") {
        let seed = std::fs::read_to_string(seed).expect("canonical Note test pair");
        let (pack, spr) = seed.trim().split_once('\t').expect("Pack and SPR columns");
        let (pack, spr) = (decode_base64(pack).expect("seed Pack"), decode_base64(spr).expect("seed SPR"));
        let storage = semio_os_kernel::sync::FolderEventLogStorage::new(dir.path().to_path_buf());
        assert!(semio_framework_async::block_on(storage.write_if_current(artifact_id, &kind.schema, None, &pack, &spr)).expect("persist seed"));
        workspace.bind_folder_session_document(artifact_id, &pack, &spr).expect("bind seed to Note");
    } else {
        workspace.create_plugin_artifact(artifact_id, &kind, None, &ActivationScope::detached()).expect("actual Note genesis");
    }
    let original = workspace.read_artifact_bytes(artifact_id).unwrap().unwrap();
    let selected = std::env::var("SEMIO_MCP_EXPORT_TEST_CASE").ok();
    let cases = law["cases"].as_array().unwrap();
    if let Some(id) = &selected { assert!(cases.iter().any(|row| row["id"] == id.as_str()), "unknown export case {id}"); }
    let cases = cases.iter().filter(|row| selected.as_deref().is_none_or(|id| row["id"] == id));
    for row in cases {
        let at = row["cancelAt"].as_str().map(str::to_string);
        let after = row["after"].as_f64().unwrap_or_default();
        let fired = Arc::new(AtomicBool::new(false));
        let token = semio_framework_async::CancelToken::root_now();
        let observed = Arc::clone(&fired);
        let cancel = token.clone();
        let (resume, hold) = mpsc::channel();
        let hold = Mutex::new(Some(hold));
        let scope = ActivationScope::new(token, move |phase, fraction| {
            let reached = at.as_deref() == Some(phase.id()) && (phase.id() == "opening-guest" || fraction > after);
            if reached && !observed.swap(true, Ordering::AcqRel) {
                cancel.cancel_now();
                hold.lock().unwrap().take().unwrap().recv_timeout(Duration::from_secs(15)).expect("the cancellable request returns while its worker is suspended");
            }
        });
        let (report, settled) = mpsc::sync_channel(1);
        let outcome = workspace.export_artifact_media_cancellably(plugin, artifact_id, port, scope, move |result| report.send(result).unwrap());
        if row["expected"] == "CANCELLED" {
            assert!(fired.load(Ordering::Acquire), "the real guest reached {}: {:?}", row["id"], outcome.as_ref().err());
            assert_eq!(outcome.expect_err("scoped export was cancelled").code, GatewayErrorCode::Cancelled, "{}", row["id"]);
            assert!(matches!(settled.try_recv(), Err(mpsc::TryRecvError::Empty)), "the halted worker still owns cleanup");
            resume.send(()).unwrap();
            let worker_error = settled.recv_timeout(Duration::from_secs(240)).expect("worker settled").expect_err("no cancelled worker media");
            assert_eq!(worker_error.code, GatewayErrorCode::Cancelled, "{}", worker_error.message);
            eprintln!("[DEBUG] {} returned request cancellation before worker settlement", row["id"]);
        } else {
            let (_, data, answered_port) = outcome.expect("genuine export succeeds after cancellation");
            settled.recv_timeout(Duration::from_secs(1)).expect("successful worker settles before output").unwrap();
            assert_eq!(answered_port, port);
            let media = std::str::from_utf8(&data).unwrap();
            let carrier = decode_base64(media.strip_prefix("pk:").unwrap()).unwrap();
            let archive = semio_framework_async::block_on(semio_os_kernel::os_spr::decode_document_archive_bytes(&carrier)).expect("owned archive decoding");
            assert_eq!((archive.parent_pack, archive.parent_spr), original);
        }
        assert_eq!(workspace.read_artifact_bytes(artifact_id).unwrap().unwrap(), original, "{}", row["id"]);
    }
}
