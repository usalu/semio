use super::*;

static RENDERER_IO_TEST_OWNER: Mutex<()> = Mutex::new(());

struct RendererIoPollWake {
    count: std::sync::atomic::AtomicUsize,
}

impl std::task::Wake for RendererIoPollWake {
    fn wake(self: Arc<Self>) {
        self.count.fetch_add(1, Ordering::AcqRel);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.count.fetch_add(1, Ordering::AcqRel);
    }
}

/// 🪢 A first poll contending with the exact mounted-session owner retains a runnable future.
#[test]
fn a_contended_renderer_io_poll_wakes_itself_until_the_exact_session_can_register() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).expect("native asset contract");
    let contention = &contract["rendererIoContention"];
    assert_eq!(contention["registrationState"].as_str(), Some("checkedOut"));
    assert_eq!(contention["exactGenerationLive"].as_bool(), Some(true));
    assert_eq!(contention["poll"].as_str(), Some("pending"));
    let mut handle = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ProcessResidentBytes).expect("mounted native I/O handle");
    let slot = &RENDERER_IO_SLOTS[handle.slot];
    assert_eq!(slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire), Ok(RENDERER_IO_LIVE));
    let wake = Arc::new(RendererIoPollWake { count: std::sync::atomic::AtomicUsize::new(0) });
    let waker = std::task::Waker::from(wake.clone());
    let mut context = std::task::Context::from_waker(&waker);
    assert!(std::pin::Pin::new(&mut handle).poll(&mut context).is_pending());
    assert_eq!(wake.count.load(Ordering::Acquire), contention["wakeCount"].as_u64().expect("wake count") as usize, "contended registration preserves a runnable owner");
    slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
    drop(handle);
    for _ in 0..16 {
        let _ = pump_renderer_io_sessions(1);
    }
    assert_eq!(RENDERER_IO_SLOTS.iter().filter(|slot| slot.state.load(Ordering::Acquire) == RENDERER_IO_LIVE).count(), contention["terminalOwners"].as_u64().expect("terminal owner count") as usize);
}

#[test]
fn mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());
    let mut handles = Vec::with_capacity(RENDERER_IO_SESSION_SLOTS);
    for index in 0..RENDERER_IO_SESSION_SLOTS {
        let path = std::path::PathBuf::from(format!("/semio-retained-native-io-{index:04}"));
        let request_pointer = path.as_os_str().as_encoded_bytes().as_ptr();
        let handle = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(path)).expect("logical maximum plus one has a mounted rejection owner");
        if index + 1 == RENDERER_IO_SESSION_SLOTS {
            let plus_one_request_pointer = request_pointer;
            let returned_pointer =
                renderer_io_with_node(handle.slot, handle.generation, |node| node.rejected.as_ref().and_then(|rejected| rejected.job().retained_request_backing_identity()).expect("maximum plus one retains the exact rejected request"))
                    .expect("maximum plus one mounted generation");
            assert_eq!(returned_pointer, plus_one_request_pointer);
        }
        handles.push(handle);
    }
    assert_eq!(pump_renderer_io_sessions(0), 0);
    let first_slot = handles[0].slot;
    let first_generation = handles[0].generation;
    drop(handles);
    assert_eq!(pump_renderer_io_sessions(1), 1, "one host turn advances one mounted control opportunity");
    for _ in 0..RENDERER_IO_SESSION_SLOTS * 16 {
        if RENDERER_IO_SLOTS.iter().all(|slot| slot.state.load(Ordering::Acquire) != RENDERER_IO_LIVE) {
            break;
        }
        assert!(pump_renderer_io_sessions(1) <= 1);
    }
    assert!(RENDERER_IO_SLOTS.iter().all(|slot| slot.state.load(Ordering::Acquire) != RENDERER_IO_LIVE));
    assert!(!renderer_io_generation_live(first_slot, first_generation));
    let replacement = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ProcessResidentBytes).expect("closed registry slot is reusable with a new generation");
    assert_eq!(replacement.slot, first_slot);
    assert!(replacement.generation > first_generation);
    drop(replacement);
    for _ in 0..16 {
        let _ = pump_renderer_io_sessions(1);
    }
}

/// 🚪️ A process entry point completes and retires mounted page I/O without a window pump.
#[test]
fn headless_entrypoint_drives_retained_io_and_matches_the_json_oracle() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("headless contract");
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path = root.join(format!("headless-entrypoint-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let actual=crate::native_entrypoint::drive_native_entrypoint(async {
        let value=run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage {path:path.clone(),offset:0,max_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES}).await.expect("headless page read");
        let semio_framework_os_services::NativeIoValue::Page {mut bytes,eof}=value else {panic!("wrong I/O result")};
        assert!(eof);
        let parsed:serde_json::Value=serde_json::from_slice(bytes.page(0).expect("page bytes")).expect("renderer JSON");
        while !matches!(bytes.close_step(1,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES),semio_framework_job::JobPayloadCloseStep::Complete) {}
        parsed
    });
    assert_eq!(actual,oracle);
    assert_eq!(RENDERER_IO_SLOTS.iter().filter(|slot|matches!(slot.state.load(Ordering::Acquire),RENDERER_IO_LIVE|RENDERER_IO_CHECKED_OUT)).count(),contract["terminalIoOwners"].as_u64().expect("terminal owners") as usize);
    std::fs::remove_file(path).expect("remove test output");
}

/// 🪟️ Window handles are captured synchronously before GPU device preparation is delegated.
#[test]
fn native_surface_capture_precedes_worker_device_preparation() {
    fn assert_factory<F: std::future::Future<Output = Result<GpuContext, String>> + Send>(_: impl FnOnce(Arc<winit::window::Window>) -> Result<F, String>) {}
    assert_factory(GpuContext::from_window);
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🪟️surface/🔣️.json")).expect("surface contract");
    let source = include_str!("../../🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
    let resumed = &source[source.find("fn resumed(").expect("window callback")..];
    let mut phases = vec![(resumed.find("event_loop.create_window(").expect("create window"), "window"), (resumed.find("GpuContext::from_window(").expect("capture surface"), "surface"), (resumed.find("crate::spawn_app_task(").expect("delegate device preparation"), "worker")];
    phases.sort_by_key(|row| row.0);
    assert_eq!(serde_json::to_value(phases.iter().map(|row| row.1).collect::<Vec<_>>()).expect("phase order"), contract["hostPhases"]);
}

/// 🧑‍🏭️ Native application boot completes retained I/O before any presentation host exists.
#[test]
fn native_app_worker_completes_io_without_a_presentation_host() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("worker I/O contract");
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path=root.join(format!("application-worker-io-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let (sent,received)=std::sync::mpsc::channel();
    let request_path=path.clone();
    spawn_app_task(async move {
        let value=run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage {path:request_path,offset:0,max_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES}).await.expect("worker page read");
        let semio_framework_os_services::NativeIoValue::Page {mut bytes,eof}=value else {panic!("wrong I/O result")};
        assert!(eof);
        let actual:serde_json::Value=serde_json::from_slice(bytes.page(0).expect("page bytes")).expect("renderer JSON");
        while !matches!(bytes.close_step(1,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES),semio_framework_job::JobPayloadCloseStep::Complete) {}
        sent.send(actual).expect("worker result");
    });
    let actual=received.recv_timeout(std::time::Duration::from_secs(5)).expect("application worker must advance I/O without a presentation callback");
    assert_eq!(actual,oracle);
    std::fs::remove_file(path).expect("remove test output");
}

/// 🧑‍🏭️ Native application boot completes retained I/O before any presentation host exists.
#[test]
fn native_kernel_worker_completes_io_without_a_presentation_host() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("worker I/O contract");
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path=root.join(format!("kernel-worker-io-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let (sent,received)=std::sync::mpsc::channel();
    let request_path=path.clone();
    let _task=kernel_runtime::KernelPoolFuture::spawn(renderer_worker_pool(),semio_framework_async::Lane::Interactive,async move {
        let value=run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage {path:request_path,offset:0,max_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES}).await.expect("worker page read");
        let semio_framework_os_services::NativeIoValue::Page {mut bytes,eof}=value else {panic!("wrong I/O result")};
        assert!(eof);
        let actual:serde_json::Value=serde_json::from_slice(bytes.page(0).expect("page bytes")).expect("renderer JSON");
        while !matches!(bytes.close_step(1,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES),semio_framework_job::JobPayloadCloseStep::Complete) {}
        sent.send(actual).expect("worker result");
    });
    let actual=received.recv_timeout(std::time::Duration::from_secs(5)).expect("kernel worker must advance I/O without a presentation callback");
    assert_eq!(actual,oracle);
    std::fs::remove_file(path).expect("remove test output");
}

/// 📦️ Local development components use the producer bound and preserve their exact page bytes.
#[test]
fn native_component_reader_admits_local_dev_artifacts_above_catalog_network_bound() {
    use std::io::Write;
    let _owner=RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error|error.into_inner());
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("component input contract");
    assert_eq!(semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES,contract["networkComponentMaxBytes"].as_u64().expect("network bound"));
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path=root.join(format!("local-component-pages-{}.json",std::process::id()));
    let content=serde_json::to_vec(&contract["document"]).expect("oracle input");
    let length=contract["localComponentTestBytes"].as_u64().expect("local input length") as usize;
    {
        let mut file=std::fs::File::create(&path).expect("test component");
        file.write_all(&content).expect("document bytes");
        let padding=[b' ';65536];
        let mut remaining=length-content.len();
        while remaining!=0 {let count=remaining.min(padding.len());file.write_all(&padding[..count]).expect("bounded padding");remaining-=count;}
    }
    let oracle:serde_json::Value=serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(&path).expect("independent file read"))).expect("independent JSON parser");
    let actual=crate::native_entrypoint::drive_native_entrypoint(kernel_runtime::read_native_component(&path)).expect("local development component pages");
    assert_eq!(actual.len(),length);
    assert_eq!(&actual[..content.len()],content.as_slice());
    assert!(actual[content.len()..].iter().all(|byte|*byte==b' '));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual).expect("native JSON pages"),oracle);
    drop(actual);
    let maximum=contract["localComponentMaxBytes"].as_u64().expect("local component ceiling");
    std::fs::File::create(&path).expect("oversized local input").set_len(maximum+1).expect("bounded sparse input");
    let refusal=crate::native_entrypoint::drive_native_entrypoint(kernel_runtime::read_native_component(&path)).expect_err("maximum plus one local byte must be refused");
    assert!(refusal.contains(&format!("exceeds {maximum} bytes")));
    std::fs::remove_file(path).expect("remove test output");
}

/// 🚨️ A selected plugin without an application cannot report a successful native smoke boot.
#[test]
fn native_smoke_without_a_selected_application_returns_failure() {
    let _owner=RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error|error.into_inner());
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json")).expect("smoke contract");
    let fixture=&contract["emptySmoke"];
    let registry:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugin/📇️registry/🧬️schema/🔣️.json")).expect("current descriptor contract");
    assert_eq!(fixture["descriptor"]["executionProtocol"]["appChannelVersion"],registry["$defs"]["CatalogDescriptorV1"]["properties"]["executionProtocol"]["properties"]["appChannelVersion"]["const"],"the smoke input uses the current host channel");
    let descriptor:semio_framework::manifest::PackageDescriptor=serde_json::from_value(fixture["descriptor"].clone()).expect("independent descriptor oracle");
    assert!(descriptor.manifest.apps.is_empty());
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("empty-native-smoke-{}",std::process::id()));
    std::fs::create_dir_all(&root).expect("test runtime root");
    std::fs::write(root.join("descriptor.json"),serde_json::to_vec(&descriptor).expect("oracle descriptor bytes")).expect("descriptor input");
    std::fs::write(root.join("component.wasm"),[0,97,115,109,13,0,1,0]).expect("empty component input");
    std::fs::write(root.join("🔣️runtime.json"),serde_json::to_vec(&fixture["runtime"]).expect("runtime bytes")).expect("runtime input");
    let actual=crate::native_entrypoint::drive_native_entrypoint(run_smoke_with_axes("empty-smoke",root.clone(),Vec::new(),(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native)));
    assert_eq!(actual,fixture["exitCode"].as_i64().expect("smoke status") as i32);
    std::fs::remove_dir_all(root).expect("remove test runtime");
}
