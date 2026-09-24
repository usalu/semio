//! ⏱️ Replays `🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json` against the smallest staged
//! plugin component, in a process of its own with a private, empty `SEMIO_HOME`, so every compile is
//! genuinely cold and runs in a real `semio-os-mcp compile-component` worker.
use semio_framework_os_mcp::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/⏱️compile-cancellation-law.json")).expect("compile-cancellation law fixture")
}

/// 🧩️ The smallest staged plugin component: the cheapest real compile that still runs long enough
/// to be cancelled inside.
fn smallest_staged_component(repo_root: &Path) -> (String, Vec<u8>) {
    let registry = load_plugin_registry(repo_root).expect("generated plugin registry");
    let (plugin_id, path) = registry
        .iter()
        .filter_map(|entry| resolve_plugin_wasm_path(repo_root, entry).ok().map(|path| (entry.plugin_id.clone(), path)))
        .filter_map(|(plugin_id, path)| std::fs::metadata(&path).ok().map(|metadata| (metadata.len(), plugin_id, path)))
        .min_by_key(|(length, _, _)| *length)
        .map(|(_, plugin_id, path)| (plugin_id, path))
        .expect("at least one staged plugin component");
    (plugin_id, std::fs::read(path).expect("staged component bytes"))
}

fn files_under(root: &Path, suffix: &str) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.to_string_lossy().ends_with(suffix) {
                found.push(path);
            }
        }
    }
    found
}

/// 🧾️ One requester: whether it reached the compile, when its cancel fired, when it was answered.
struct Requester {
    token: semio_framework_async::CancelToken,
    compiling: AtomicBool,
    cancelled_at: Mutex<Option<Instant>>,
    answered: Mutex<Option<(Result<(), GatewayErrorCode>, Instant)>>,
}

#[test]
fn a_cancelled_compile_is_killed_when_its_last_requester_leaves_and_shared_by_the_rest() {
    let law = fixture();
    let ceiling = Duration::from_millis(law["cancelAnswerCeilingMs"].as_u64().unwrap());
    let repo_root = find_repo_root().expect("the compile law needs the repo tree for a real component");
    let (plugin_id, bytes) = smallest_staged_component(&repo_root);
    let home = std::env::temp_dir().join(format!("semio-compile-law-{}", std::process::id()));
    std::env::set_var("SEMIO_HOME", &home);
    set_component_compile_worker(PathBuf::from(env!("CARGO_BIN_EXE_semio-os-mcp")));
    let cache = home.join(".semio").join("cache").join("wasmtime");
    eprintln!("compile-law component `{plugin_id}`: {} bytes", bytes.len());
    let bytes = Arc::new(bytes);
    for row in law["cases"].as_array().unwrap() {
        let expected = row["expected"].as_str().unwrap();
        let cancelling: Vec<usize> = row["cancel"].as_array().unwrap().iter().map(|index| index.as_u64().unwrap() as usize).collect();
        let requesters: Vec<Arc<Requester>> = (0..row["requesters"].as_u64().unwrap())
            .map(|_| Arc::new(Requester { token: semio_framework_async::CancelToken::root_now(), compiling: AtomicBool::new(false), cancelled_at: Mutex::new(None), answered: Mutex::new(None) }))
            .collect();
        let workers_before = component_compile_workers_started();
        let threads: Vec<_> = requesters
            .iter()
            .map(|requester| {
                let requester = Arc::clone(requester);
                let bytes = Arc::clone(&bytes);
                let plugin_id = plugin_id.clone();
                std::thread::spawn(move || {
                    let observer = Arc::clone(&requester);
                    let scope = ActivationScope::new(requester.token.clone(), move |phase, _| {
                        if phase == ActivationPhase::CompilingComponent {
                            observer.compiling.store(true, Ordering::SeqCst);
                        }
                    });
                    let result = prepare_plugin_component(&plugin_id, &bytes, &scope).map_err(|error| error.code);
                    *requester.answered.lock().unwrap() = Some((result, Instant::now()));
                })
            })
            .collect();
        if expected != "WARM" {
            let deadline = Instant::now() + Duration::from_secs(60);
            while !requesters.iter().all(|requester| requester.compiling.load(Ordering::SeqCst)) || live_component_compile_workers() != 1 {
                assert!(Instant::now() < deadline, "{expected}: the requesters never joined one running compile");
                std::thread::sleep(Duration::from_millis(5));
            }
            std::thread::sleep(Duration::from_millis(300));
            assert_eq!(live_component_compile_workers(), 1, "{expected}: the compile is still running when the cancels land");
        }
        for index in &cancelling {
            *requesters[*index].cancelled_at.lock().unwrap() = Some(Instant::now());
            requesters[*index].token.cancel_now();
        }
        for thread in threads {
            thread.join().expect("requester thread");
        }
        let started = component_compile_workers_started() - workers_before;
        let compiled = files_under(&cache, ".cwasm");
        let scratch = files_under(&cache, ".scratch");
        for index in &cancelling {
            let (result, answered) = requesters[*index].answered.lock().unwrap().take().expect("answered");
            let waited = answered.duration_since(requesters[*index].cancelled_at.lock().unwrap().expect("cancel fired"));
            eprintln!("compile-law {expected}: requester {index} cancelled, answered in {} µs", waited.as_micros());
            assert_eq!(result, Err(GatewayErrorCode::Cancelled), "{expected}: a cancelling requester answers CANCELLED");
            assert!(waited < ceiling, "{expected}: requester {index} was answered {waited:?} after its cancel");
        }
        let survivors: Vec<Result<(), GatewayErrorCode>> = (0..requesters.len()).filter(|index| !cancelling.contains(index)).map(|index| requesters[index].answered.lock().unwrap().take().expect("answered").0).collect();
        eprintln!("compile-law {expected}: workers started {started}, compiled {compiled:?}, scratch {scratch:?}");
        assert_eq!(live_component_compile_workers(), 0, "{expected}: no worker outlives its requesters");
        assert!(scratch.is_empty(), "{expected}: no unfinished output remains: {scratch:?}");
        match expected {
            "ABORTED" => {
                assert_eq!(started, 1, "one worker per compile");
                assert!(compiled.is_empty(), "a killed compile leaves no compiled code: {compiled:?}");
            }
            "DETACHED" => {
                assert_eq!(started, 1, "the cancelling requester and the rest shared one worker");
                assert!(survivors.iter().all(Result::is_ok), "the remaining requesters received the compiled code: {survivors:?}");
                assert_eq!(compiled.len(), 1, "the shared compile left its compiled code");
            }
            "WARM" => {
                assert_eq!(started, 0, "compiled code is served without a worker");
                assert!(survivors.iter().all(Result::is_ok));
            }
            other => panic!("unsupported law row {other}"),
        }
    }
    let _ = std::fs::remove_dir_all(&home);
}
