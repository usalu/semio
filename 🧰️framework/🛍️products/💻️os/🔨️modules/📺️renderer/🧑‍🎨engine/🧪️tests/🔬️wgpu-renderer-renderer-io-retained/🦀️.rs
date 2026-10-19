use super::*;

static RENDERER_IO_TEST_OWNER: Mutex<()> = Mutex::new(());


/// 🚪️ A process entry point completes and retires mounted page I/O without a window pump.
#[test]
fn headless_entrypoint_drives_retained_io_and_matches_the_json_oracle() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());let _source=crate::original_native_source_test_guard();
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️wgpu/⌨️native-entrypoint/🚪️headless/🔣️.json")).expect("headless contract");
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path = root.join(format!("headless-entrypoint-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let actual=crate::native_entrypoint::drive_native_entrypoint(async {
        let bytes=read_renderer_source_bytes(&path,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES as u64).await.expect("headless original source bytes");
        let parsed:serde_json::Value=serde_json::from_slice(bytes.bytes()).expect("renderer JSON");
        drop(bytes);
        parsed
    });
    assert_eq!(actual,oracle);
    assert_eq!(usize::from(renderer_source_io_pending()),contract["terminalIoOwners"].as_u64().expect("terminal owners") as usize);
    std::fs::remove_file(path).expect("remove test output");
}

/// 🪟️ Window handles are captured synchronously before GPU device preparation is delegated.
#[test]
fn native_surface_capture_precedes_worker_device_preparation() {
    fn assert_factory<F: std::future::Future<Output = Result<GpuContext, String>> + Send>(_: impl FnOnce(ui_gpu::platform::NativeSurfaceCapture) -> F) {}
    assert_factory(ui_gpu::platform::NativeSurfaceCapture::prepare);
    let _: fn(Arc<ui_gpu::platform::NativeWindow>) -> Result<ui_gpu::platform::NativeSurfaceCapture, String> = ui_gpu::platform::NativeSurfaceCapture::capture;
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️wgpu/⌨️native-entrypoint/🪟️surface/🔣️.json")).expect("surface contract");
    let source = semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
    let resumed = &source[source.find("fn resumed(").expect("window callback")..];
    let mut phases = vec![(resumed.find("event_loop.create_window(").expect("create window"), "window"), (resumed.find("ui_gpu::platform::NativeSurfaceCapture::capture(").expect("capture surface"), "surface"), (resumed.find("crate::spawn_app_task(").expect("delegate device preparation"), "worker")];
    phases.sort_by_key(|row| row.0);
    assert_eq!(serde_json::to_value(phases.iter().map(|row| row.1).collect::<Vec<_>>()).expect("phase order"), contract["hostPhases"]);
}

/// 🧑‍🏭️ Native application boot completes retained I/O before any presentation host exists.
#[test]
fn native_app_worker_completes_io_without_a_presentation_host() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());let _source=crate::original_native_source_test_guard();
    let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️wgpu/⌨️native-entrypoint/🚪️headless/🔣️.json")).expect("worker I/O contract");
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path=root.join(format!("application-worker-io-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let (sent,received)=std::sync::mpsc::channel();
    let request_path=path.clone();
    spawn_app_task(async move {
        let bytes=read_renderer_source_bytes(&request_path,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES as u64).await.expect("worker original source bytes");
        let actual:serde_json::Value=serde_json::from_slice(bytes.bytes()).expect("renderer JSON");
        drop(bytes);
        sent.send(actual).expect("worker result");
    });
    let actual=received.recv_timeout(std::time::Duration::from_secs(5)).expect("application worker must advance I/O without a presentation callback");
    assert_eq!(actual,oracle);
    crate::native_entrypoint::drive_native_entrypoint(std::future::ready(()));
    assert!(!renderer_source_io_pending());
    std::fs::remove_file(path).expect("remove test output");
}

/// 🧑‍🏭️ Native application boot completes retained I/O before any presentation host exists.
#[test]
fn native_kernel_worker_completes_io_without_a_presentation_host() {
    let _owner = RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error| error.into_inner());let _source=crate::original_native_source_test_guard();
    let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️wgpu/⌨️native-entrypoint/🚪️headless/🔣️.json")).expect("worker I/O contract");
    let root=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&root).expect("test output root");
    let path=root.join(format!("kernel-worker-io-{}.json",std::process::id()));
    std::fs::write(&path,serde_json::to_vec(&contract["document"]).expect("oracle input")).expect("test document");
    let oracle:serde_json::Value=serde_json::from_slice(&std::fs::read(&path).expect("independent file read")).expect("independent JSON parser");
    let (sent,received)=std::sync::mpsc::channel();
    let request_path=path.clone();
    let _task=kernel_runtime::KernelPoolFuture::spawn(renderer_worker_pool(),semio_framework_async::Lane::Interactive,async move {
        let bytes=read_renderer_source_bytes(&request_path,semio_framework_job::JOB_PAYLOAD_PAGE_BYTES as u64).await.expect("worker original source bytes");
        let actual:serde_json::Value=serde_json::from_slice(bytes.bytes()).expect("renderer JSON");
        drop(bytes);
        sent.send(actual).expect("worker result");
    });
    let actual=received.recv_timeout(std::time::Duration::from_secs(5)).expect("kernel worker must advance I/O without a presentation callback");
    assert_eq!(actual,oracle);
    crate::native_entrypoint::drive_native_entrypoint(std::future::ready(()));
    assert!(!renderer_source_io_pending());
    std::fs::remove_file(path).expect("remove test output");
}

/// 📦️ Local development components use the producer bound and preserve their exact page bytes.
#[test]
fn native_component_reader_admits_local_dev_artifacts_above_catalog_network_bound() {
    use std::io::Write;
    let _owner=RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error|error.into_inner());let _source=crate::original_native_source_test_guard();
    let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️wgpu/⌨️native-entrypoint/🚪️headless/🔣️.json")).expect("component input contract");
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
    let admission=format!("{{\"version\":1,\"declaration\":{},\"scriptOwner\":\"fixture:renderer-original-io\",\"scriptDeadlineEpochMilliseconds\":null}}",include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🎟️launch/🔣️.json"));crate::native_launch::install_native_launch_fixture(&admission).expect("actual declared startup source");
    crate::native_entrypoint::drive_native_entrypoint(async{
        let actual=kernel_runtime::read_native_component(&path).await.expect("local development component pages");
        assert_eq!(actual.bytes().len(),length);
        assert_eq!(&actual.bytes()[..content.len()],content.as_slice());
        assert!(actual.bytes()[content.len()..].iter().all(|byte|*byte==b' '));
        assert_eq!(serde_json::from_slice::<serde_json::Value>(actual.bytes()).expect("native JSON pages"),oracle);
        drop(actual);
    });
    let maximum=contract["localComponentMaxBytes"].as_u64().expect("local component ceiling");
    std::fs::File::create(&path).expect("oversized local input").set_len(maximum+1).expect("bounded sparse input");
    crate::native_entrypoint::drive_native_entrypoint(async{
        let refusal=kernel_runtime::read_native_component(&path).await.expect_err("maximum plus one local byte must be refused");
        assert_eq!(refusal.kind,crate::RendererSourceIoFailureKind::Extent(semio_framework_os_services::NativeIoStreamRefusal{length:maximum+1,maximum_bytes:maximum}));
        drop(refusal);
    });
    std::fs::remove_file(path).expect("remove test output");
}

/// 🚨️ A selected plugin without an application cannot report a successful native smoke boot.
#[test]
fn native_smoke_without_a_selected_application_returns_failure() {
    let _owner=RENDERER_IO_TEST_OWNER.lock().unwrap_or_else(|error|error.into_inner());let _source=crate::original_native_source_test_guard();
    let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️wgpu/⌨️native-entrypoint/🚪️headless/🔣️.json")).expect("smoke contract");
    let fixture=&contract["emptySmoke"];
    let registry:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json")).expect("current descriptor contract");
    assert_eq!(fixture["descriptor"]["executionProtocol"]["appChannelVersion"],registry["$defs"]["PackageDescriptorV1"]["properties"]["executionProtocol"]["properties"]["appChannelVersion"]["const"],"the smoke input uses the current host channel");
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
