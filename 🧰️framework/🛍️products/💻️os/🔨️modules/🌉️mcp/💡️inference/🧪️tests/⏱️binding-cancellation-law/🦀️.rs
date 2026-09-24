//! ⏱️ Replays `💡️inference/🧫️fixtures/⏱️binding-cancellation-law.json`, then
//! `🗿️artifact/🧫️fixtures/⏱️create-cancellation-law.json`, against a real staged `🀄️wfc` component, in a process of its own so every binding phase is genuinely reached: the compiled-code
//! map starts empty and the persisted component identities live under a private `SEMIO_HOME` whose
//! `.cwasm` store points at the shared one, so the component is read and hashed here while its
//! compiled code still loads from the cache instead of a cold compile.
use semio_framework_os_mcp::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/⏱️binding-cancellation-law.json")).expect("binding-cancellation law fixture")
}

fn create_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🗿️artifact/🧫️fixtures/⏱️create-cancellation-law.json")).expect("create-cancellation law fixture")
}

fn phase(id: &str) -> ActivationPhase {
    *ActivationPhase::ALL.iter().find(|phase| phase.id() == id).unwrap_or_else(|| panic!("unknown binding phase {id}"))
}

fn private_semio_home() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME");
    let shared = Path::new(&home).join(".semio").join("cache").join("wasmtime");
    let private = std::env::temp_dir().join(format!("semio-binding-law-{}", std::process::id()));
    let cache = private.join(".semio").join("cache").join("wasmtime");
    std::fs::create_dir_all(&cache).expect("private cache root");
    for entry in std::fs::read_dir(&shared).into_iter().flatten().flatten() {
        if entry.file_name() == "component-identities" {
            continue;
        }
        #[cfg(unix)]
        let _ = std::os::unix::fs::symlink(entry.path(), cache.join(entry.file_name()));
        #[cfg(windows)]
        let _ = std::os::windows::fs::symlink_dir(entry.path(), cache.join(entry.file_name()));
    }
    private
}

/// 🧾️ What one law case observed: the phases its binding reported, and when its cancel fired.
struct Case {
    reported: Mutex<Vec<ActivationPhase>>,
    cancelled_at: Mutex<Option<Instant>>,
    fired: AtomicBool,
}

enum Target<'a> {
    Session(&'a str),
    Artifact(&'a str),
}

fn run(workspace: &Arc<HeadlessWorkspace>, target: Target<'_>, cancel_at: Option<ActivationPhase>) -> (Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError>, Arc<Case>, Instant) {
    let case = Arc::new(Case { reported: Mutex::new(Vec::new()), cancelled_at: Mutex::new(None), fired: AtomicBool::new(false) });
    let token = semio_framework_async::CancelToken::root_now();
    let observer = Arc::clone(&case);
    let cancel = token.clone();
    let scope = ActivationScope::new(token, move |reported, _| {
        observer.reported.lock().unwrap().push(reported);
        if Some(reported) == cancel_at && !observer.fired.swap(true, Ordering::SeqCst) {
            *observer.cancelled_at.lock().unwrap() = Some(Instant::now());
            cancel.cancel_now();
        }
    });
    let result = match target {
        Target::Session(plugin_id) => workspace.activate_plugin_session_cancellably(plugin_id, scope).map(|()| None),
        Target::Artifact(artifact_id) => workspace.bind_artifact_document_cancellably(artifact_id, scope),
    };
    (result, case, Instant::now())
}

#[test]
fn a_cancel_in_any_binding_phase_is_answered_within_the_ceiling_and_leaves_no_half_open_guest() {
    let law = fixture();
    let rust_rows: Vec<(String, f64)> = ActivationPhase::ALL.iter().map(|phase| (phase.id().to_string(), phase.position())).collect();
    let fixture_rows: Vec<(String, f64)> = law["phases"].as_array().unwrap().iter().map(|row| (row["phase"].as_str().unwrap().to_string(), row["position"].as_f64().unwrap())).collect();
    assert_eq!(rust_rows, fixture_rows, "the Rust activation phases are the schema's, in order");
    let Ok(repo_root) = find_repo_root() else {
        panic!("the binding law needs the repo tree for its real wfc component");
    };
    let registry = load_plugin_registry(&repo_root).expect("generated plugin registry");
    resolve_plugin_wasm_path(&repo_root, find_plugin_entry(&registry, "wfc").expect("wfc in the registry")).expect("a staged wfc component");
    std::env::set_var("SEMIO_HOME", private_semio_home());
    set_component_compile_worker(PathBuf::from(env!("CARGO_BIN_EXE_semio-os-mcp")));
    let folder = std::env::temp_dir().join(format!("semio-binding-law-folder-{}", std::process::id()));
    let catalog = Arc::new(build_catalog());
    let workspace = Arc::new(HeadlessWorkspace::open_folder(folder, "agent:binding-law".to_string(), Vec::new(), Arc::clone(&catalog)).expect("folder workspace"));
    let channel = Box::new(ArtifactChannels::Routing(workspace.open_routing_channel()));
    let actions = Arc::new(ActionAdapter::new(channel, Arc::new(HandleTable::new()), Arc::new(IdempotencyStore::new()), Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), AutoApprovePolicy::Never, ClientInfo { name: "binding-law".to_string(), version: "1".to_string() }));
    workspace.bind_root_action_adapter(Arc::clone(&actions));
    let kind = workspace.installed_artifact_kinds().expect("installed kinds").into_iter().find(|kind| kind.schema == "s.wfc.bitmap").expect("wfc bitmap kind installed");
    let ceiling = Duration::from_millis(law["cancelAnswerCeilingMs"].as_u64().unwrap());
    let mut created = None;
    let mut bound = None;
    for row in law["cases"].as_array().unwrap() {
        let cancel_at = row["cancelAt"].as_str().map(phase);
        let component_phase = cancel_at.is_some_and(|phase| phase != ActivationPhase::ReadingDocument);
        if !component_phase && created.is_none() {
            created = Some(workspace.create_plugin_artifact("binding-law-wfc", &kind, &ActivationScope::detached()).expect("genesis artifact"));
        }
        let target = if component_phase { Target::Session("wfc") } else { Target::Artifact("binding-law-wfc") };
        let (result, case, answered) = run(&workspace, target, cancel_at);
        let reported = case.reported.lock().unwrap().clone();
        eprintln!("binding-law case {:?}: reported {:?}", cancel_at.map(ActivationPhase::id), reported.iter().map(|phase| phase.id()).collect::<Vec<_>>());
        match (row["expected"].as_str().unwrap(), cancel_at) {
            ("CANCELLED", Some(target)) => {
                assert!(reported.contains(&target), "the case never reached `{}` — reported {:?}", target.id(), reported);
                let error = result.expect_err("a cancelled binding answers an error");
                assert_eq!(error.code, GatewayErrorCode::Cancelled, "{}", error.message);
                let waited = answered.duration_since(case.cancelled_at.lock().unwrap().expect("the cancel fired"));
                eprintln!("binding-law case {}: cancel answered in {} µs", target.id(), waited.as_micros());
                assert!(waited < ceiling, "the cancel in `{}` was answered after {waited:?}", target.id());
            }
            ("BOUND", None) => {
                let (pack, spr) = result.expect("an uncancelled binding binds").expect("the artifact is readable");
                assert_eq!(Some((pack.len(), spr.len())), created, "the bound document is the genesis document the artifact was created with");
                bound = Some((pack, spr));
            }
            other => panic!("unsupported law row {other:?}"),
        }
    }
    let (again, _, _) = run(&workspace, Target::Artifact("binding-law-wfc"), None);
    let bound = bound.expect("the law binds once uncancelled");
    assert_eq!(again.expect("a warm rebinding binds").expect("readable"), bound, "rebinding answers byte-identical bytes");

    let create_law = create_fixture();
    let create_ceiling = Duration::from_millis(create_law["cancelAnswerCeilingMs"].as_u64().unwrap());
    for (index, row) in create_law["cases"].as_array().unwrap().iter().enumerate() {
        let cancel_at = row["cancelAt"].as_str().map(phase);
        let artifact_id = format!("create-law-wfc-{index}");
        let (result, case, answered) = create(&workspace, &artifact_id, &kind, cancel_at);
        let reported = case.reported.lock().unwrap().clone();
        eprintln!("create-law case {:?}: reported {:?}", cancel_at.map(ActivationPhase::id), reported.iter().map(|phase| phase.id()).collect::<Vec<_>>());
        let exists = workspace.workspace_artifact_ids().expect("artifact ids").contains(&artifact_id);
        match (row["expected"].as_str().unwrap(), cancel_at) {
            ("CANCELLED", Some(target)) => {
                assert!(reported.contains(&target), "the create never reached `{}` — reported {:?}", target.id(), reported);
                assert_eq!(result.expect_err("a cancelled create answers an error").code, GatewayErrorCode::Cancelled);
                let waited = answered.duration_since(case.cancelled_at.lock().unwrap().expect("the cancel fired"));
                eprintln!("create-law case {}: cancel answered in {} µs", target.id(), waited.as_micros());
                assert!(waited < create_ceiling, "the create cancel in `{}` was answered after {waited:?}", target.id());
                assert!(!exists, "a cancelled create persisted `{artifact_id}`");
            }
            ("SUCCEEDED", None) => {
                assert_eq!(result.expect("an uncancelled create persists"), (bound.0.len(), bound.1.len()), "the created document is the guest's genesis document");
                assert!(exists, "an uncancelled create persisted nothing");
            }
            other => panic!("unsupported create law row {other:?}"),
        }
    }
}

fn create(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str, kind: &InstalledArtifactKind, cancel_at: Option<ActivationPhase>) -> (Result<(usize, usize), GatewayError>, Arc<Case>, Instant) {
    let case = Arc::new(Case { reported: Mutex::new(Vec::new()), cancelled_at: Mutex::new(None), fired: AtomicBool::new(false) });
    let token = semio_framework_async::CancelToken::root_now();
    let observer = Arc::clone(&case);
    let cancel = token.clone();
    let scope = ActivationScope::new(token, move |reported, _| {
        observer.reported.lock().unwrap().push(reported);
        if Some(reported) == cancel_at && !observer.fired.swap(true, Ordering::SeqCst) {
            *observer.cancelled_at.lock().unwrap() = Some(Instant::now());
            cancel.cancel_now();
        }
    });
    let result = workspace.create_plugin_artifact_cancellably(artifact_id, kind, scope);
    (result, case, Instant::now())
}
