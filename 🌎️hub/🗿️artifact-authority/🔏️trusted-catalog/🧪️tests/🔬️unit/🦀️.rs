use super::*;
use crate::artifact_authority::adapters::AUTHORITY_MAX_DIAGNOSTIC_BYTES;
#[cfg(feature = "native-artifact-execution")]
use crate::artifact_authority::native_openable_provider::NativeCodecProviderSetV1;
use crate::artifact_authority::trusted_catalog::schema::{TrustedBundleBrowserActorV1, TrustedBundleCodecV1, TrustedBundleComponentV1, TrustedBundlePluginModuleV1, TrustedBundleProfileOpenTargetV1};
use crate::artifact_authority::{AuthorityLimits, AuthorityOperationControl};
use directory::os_store::{ArtifactPackFiles, ArtifactTextFiles, VcsError, document_codec};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static FIXTURE_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// 🧊️ The residency every hub applies is exactly the `const` values `TrustedCatalogGuestResidencyV1`
/// declares, and a catalog load never verifies more guests at once than the declared bound.
#[test]
fn guest_residency_bounds_are_the_declared_schema_values() {
    let module: serde_json::Value = serde_json::from_str(schema::TRUSTED_CATALOG_SCHEMA_JSON).unwrap();
    let residency = &module["$defs"]["TrustedCatalogGuestResidencyV1"];
    let declared = &residency["properties"];
    assert_eq!(declared["residentComponentBytesMaximum"]["const"].as_u64(), Some(TRUSTED_CATALOG_GUEST_RESIDENCY.resident_component_bytes_maximum));
    assert_eq!(declared["concurrentVerifications"]["const"].as_u64(), Some(TRUSTED_CATALOG_GUEST_RESIDENCY.concurrent_verifications as u64));
    assert_eq!(residency["required"].as_array().unwrap().len(), declared.as_object().unwrap().len(), "every declared bound is required");
    assert!(guest_verification_concurrency() >= 1 && guest_verification_concurrency() <= TRUSTED_CATALOG_GUEST_RESIDENCY.concurrent_verifications);
}

/// 🗃️ A remembered verification answers only for exactly its component, schema and engine; a record
/// edited to claim another hash, another engine or another component is a miss, never a verdict.
#[tokio::test]
async fn a_guest_codec_verification_is_recalled_only_for_its_exact_key() {
    let root = std::env::temp_dir().join(format!("guest-codec-verifications-{}-{}", std::process::id(), FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst)));
    let cache = GuestCodecVerificationCacheV1::at(root.clone(), "owned-engine-v1:aa:fuel-1");
    let component = [7u8; 32];
    let observed = [9u8; 32];
    assert_eq!(cache.recall(&component, "note.document").await, None, "a cold cache recalls nothing");
    assert!(cache.remember(&component, "note.document", &observed).await);
    assert_eq!(cache.recall(&component, "note.document").await, Some(observed));
    assert_eq!(cache.recall(&component, "draw.document").await, None, "another schema is another key");
    assert_eq!(cache.recall(&[8u8; 32], "note.document").await, None, "another component is another key");
    assert_eq!(GuestCodecVerificationCacheV1::at(root.clone(), "owned-engine-v1:ab:fuel-1").recall(&component, "note.document").await, None, "a changed engine verifies afresh");
    assert_eq!(GuestCodecVerificationCacheV1::disabled().recall(&component, "note.document").await, None);
    let file = std::fs::read_dir(&root).unwrap().next().unwrap().unwrap().path();
    let module: serde_json::Value = serde_json::from_str(schema::TRUSTED_CATALOG_SCHEMA_JSON).unwrap();
    let stored: serde_json::Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    let required: Vec<&str> = module["$defs"]["GuestCodecVerificationV1"]["required"].as_array().unwrap().iter().map(|field| field.as_str().unwrap()).collect();
    assert_eq!(stored.as_object().unwrap().keys().map(String::as_str).collect::<BTreeSet<_>>(), required.iter().copied().collect(), "the record is exactly the schema's fields");
    let forged = String::from_utf8(std::fs::read(&file).unwrap()).unwrap().replace(&"09".repeat(32), &"0a".repeat(32));
    std::fs::write(&file, forged).unwrap();
    assert_eq!(cache.recall(&component, "note.document").await, Some([10u8; 32]), "a record is only a memory; the loader compares it with the trust record");
    std::fs::write(&file, b"{not json").unwrap();
    assert_eq!(cache.recall(&component, "note.document").await, None, "an unreadable record is a miss");
    std::fs::remove_dir_all(&root).unwrap();
}

/// 🪪️ A hub's verification memory is keyed by the owned engine and lives inside its trusted catalog:
/// any build of the same engine at the same data root — another path, a copy, a re-signed binary —
/// recalls what one verified, and copying `trusted-catalog/` carries the memory with it.
#[tokio::test]
async fn a_guest_codec_verification_is_keyed_by_the_engine_and_travels_with_the_catalog() {
    let data = std::env::temp_dir().join(format!("guest-codec-engine-{}-{}", std::process::id(), FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst)));
    let engine = guest_codec_engine_identity();
    assert!(engine.starts_with("owned-engine-v1:") && engine.ends_with(&format!(":fuel-{}", GUEST_CODEC_BUDGET.fuel)), "{engine}");
    assert_eq!(engine, guest_codec_engine_identity(), "the identity is a pure function of the engine");
    let component = [3u8; 32];
    let observed = [5u8; 32];
    assert!(GuestCodecVerificationCacheV1::beside(&data).remember(&component, "note.document", &observed).await);
    assert!(data.join("trusted-catalog").join("guest-codec-verifications").is_dir());
    assert_eq!(GuestCodecVerificationCacheV1::beside(&data).recall(&component, "note.document").await, Some(observed));
    let copied = data.with_extension("copy");
    std::fs::create_dir_all(copied.join("trusted-catalog")).unwrap();
    std::fs::rename(data.join("trusted-catalog").join("guest-codec-verifications"), copied.join("trusted-catalog").join("guest-codec-verifications")).unwrap();
    assert_eq!(GuestCodecVerificationCacheV1::beside(&copied).recall(&component, "note.document").await, Some(observed), "the memory travels with the catalog tree");
    assert_eq!(GuestCodecVerificationCacheV1::beside(&data).recall(&component, "note.document").await, None);
    std::fs::remove_dir_all(&data).unwrap();
    std::fs::remove_dir_all(&copied).unwrap();
}

/// 🧊️ Compiled guests stay resident within the ledger's byte budget: a new compile releases the least
/// recently used guests no call holds, a held guest is never released, idleness alone releases nothing,
/// and a released guest compiles again on its next use.
#[tokio::test]
async fn compiled_guests_are_released_least_recently_used_by_capacity_never_by_idleness() {
    let compiles = AtomicUsize::new(0);
    let compile = |value: u8| {
        let compiles = &compiles;
        move || async move {
            compiles.fetch_add(1, Ordering::SeqCst);
            Ok::<_, AuthorityError>(vec![value; 16])
        }
    };
    let ledger = GuestResidencyLedgerV1::<Vec<u8>>::new(2_000);
    let (a, b, c) = (ledger.register(1_000), ledger.register(1_000), ledger.register(1_000));
    assert_eq!(*a.acquire(compile(1)).await.unwrap(), vec![1; 16]);
    assert_eq!(*b.acquire(compile(2)).await.unwrap(), vec![2; 16]);
    assert_eq!(*a.acquire(compile(1)).await.unwrap(), vec![1; 16]);
    assert_eq!(compiles.load(Ordering::SeqCst), 2, "a resident guest is reused");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    assert!(a.is_resident() && b.is_resident(), "idleness alone releases nothing");
    assert_eq!(ledger.state(), GuestResidencyStateV1 { resident: 2, resident_bytes: 2_000, maximum_bytes: 2_000, released: 0 });
    assert_eq!(*c.acquire(compile(3)).await.unwrap(), vec![3; 16]);
    assert!(!b.is_resident() && a.is_resident() && c.is_resident(), "the least recently used unheld guest makes room");
    assert_eq!(ledger.state(), GuestResidencyStateV1 { resident: 2, resident_bytes: 2_000, maximum_bytes: 2_000, released: 1 });
    let held_a = a.acquire(compile(1)).await.unwrap();
    let held_c = c.acquire(compile(3)).await.unwrap();
    assert_eq!(*b.acquire(compile(2)).await.unwrap(), vec![2; 16]);
    assert_eq!(compiles.load(Ordering::SeqCst), 4, "the released guest compiles again on its next use");
    assert!(a.is_resident() && c.is_resident() && b.is_resident(), "guests running calls hold are never released, even over budget");
    drop((held_a, held_c));
    assert_eq!(*b.acquire(compile(2)).await.unwrap(), vec![2; 16]);
    assert_eq!(compiles.load(Ordering::SeqCst), 4);
    let d = ledger.register(1_000);
    assert_eq!(*d.acquire(compile(4)).await.unwrap(), vec![4; 16]);
    assert!(!a.is_resident() && !c.is_resident() && b.is_resident() && d.is_resident(), "the next compile brings the ledger back under budget, oldest first");
    assert_eq!(ledger.state().resident_bytes, 2_000);
    let failing = ledger.register(1_000);
    assert!(failing.acquire(|| async { Err::<Vec<u8>, _>(AuthorityError::Catalog("refused".into())) }).await.is_err());
    assert!(!failing.is_resident(), "a failed compile leaves nothing resident");
}

/// 🧵️ A guest codec call interprets on the blocking pool, never on the worker awaiting it: another
/// task on a one-thread runtime keeps running for the whole call, every fuel observation reaches the
/// caller's context in order, and a cancelled caller is released at its next observation.
#[test]
fn guest_codec_calls_run_off_the_async_worker_and_relay_their_fuel() {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        let control = TestControl::new();
        let context = control.context();
        let ticks = Arc::new(AtomicUsize::new(0));
        let ticker = {
            let ticks = ticks.clone();
            tokio::spawn(async move {
                loop {
                    ticks.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
            })
        };
        let answer = interpret_off_worker(&context, |_handle, progress| {
            for fuel in [10, 20, 30] {
                std::thread::sleep(std::time::Duration::from_millis(100));
                progress(fuel);
            }
            Ok(42u32)
        })
        .await;
        assert!(matches!(answer, Ok(Ok(42))), "{answer:?}");
        assert!(ticks.load(Ordering::SeqCst) >= 20, "the awaiting worker kept running other tasks: {} ticks", ticks.load(Ordering::SeqCst));
        let relayed: Vec<u64> = control.progress.lock().unwrap().iter().filter(|progress| progress.stage == AuthorityProgressStage::GuestCodecExecuting).map(|progress| progress.completed_units).collect();
        assert_eq!(relayed, vec![10, 20, 30]);
        control.cancelled.store(true, Ordering::SeqCst);
        let started = std::time::Instant::now();
        let cancelled = interpret_off_worker(&context, |_handle, progress| {
            progress(1);
            std::thread::sleep(std::time::Duration::from_millis(400));
            Ok(0u32)
        })
        .await;
        assert!(matches!(cancelled, Err(AuthorityError::Cancelled)), "{cancelled:?}");
        assert!(started.elapsed() < std::time::Duration::from_millis(300), "a cancelled caller is released at its next observation, not when the call ends");
        ticker.abort();
    });
}

#[test]
fn trusted_descriptor_wire_materialization_matches_neutral_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧫️fixtures/🧮️wire-materialization/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let encoded = row["rawHex"].as_str().unwrap();
        let bytes: Vec<_> = (0..encoded.len()).step_by(2).map(|index| u8::from_str_radix(&encoded[index..index + 2], 16).unwrap()).collect();
        let limits = &row["limits"];
        let mut options = os_store::PackDecodeOptions::default();
        options.limits.max_file_len = limits["maxFileLen"].as_u64().unwrap();
        options.limits.max_segment_len = limits["maxSegmentLen"].as_u64().unwrap();
        options.limits.max_symbols = u32::try_from(limits["maxSymbols"].as_u64().unwrap()).unwrap();
        options.limits.max_depth = u16::try_from(limits["maxDepth"].as_u64().unwrap()).unwrap();
        options.limits.max_items = limits["maxItems"].as_u64().unwrap();
        options.limits.max_total_alloc = limits["maxTotalAlloc"].as_u64().unwrap();
        let decoded = os_store::pack_rt::decode_wire_value_with_options(&bytes, &options);
        match row["expect"]["outcome"].as_str().unwrap() {
            "accepted" => assert_eq!(serde_json::to_value(decoded.expect("admitted bounded value")).unwrap(), row["expect"]["value"], "{}", row["id"]),
            "limit" => assert!(matches!(decoded, Err(os_store::PackError::LimitExceeded(_))), "{}: {decoded:?}", row["id"]),
            "malformed" => {
                assert!(matches!(decoded, Err(os_store::PackError::Malformed { .. })), "{}: {decoded:?}", row["id"]);
                assert!(matches!(os_store::pack_rt::decode_wire_value(&bytes), Err(os_store::PackError::Malformed { .. })), "default wire facade: {}", row["id"]);
            }
            outcome => panic!("unknown bounded wire outcome {outcome}"),
        }
        eprintln!("[DEBUG] bounded-descriptor-wire case={}", row["id"]);
    }
    let mut amplified = Vec::new();
    os_store::pack_rt::write_varint_u64(&mut amplified, 1);
    os_store::pack_rt::write_varint_u64(&mut amplified, 2 * 1024 * 1024);
    amplified.resize(amplified.len() + 2 * 1024 * 1024, b'x');
    amplified.extend_from_slice(&[1, 1, 0x11, 0x0c, 32]);
    for _ in 0..32 {
        amplified.extend_from_slice(&[0x06, 0]);
    }
    assert!((amplified.len() as u64) < TRUSTED_DESCRIPTOR_MAX_BYTES);
    let result = decode_package_descriptor(&amplified);
    assert!(matches!(&result, Err(AuthorityError::Catalog(message)) if message.contains("max_total_alloc")), "actual Hub materialization fence: {result:?}");
}

include!("../📤️publication/🦀️.rs");

struct TestControl {
    cancelled: AtomicBool,
    progress: Mutex<Vec<AuthorityProgress>>,
}

impl TestControl {
    fn new() -> Self {
        Self { cancelled: AtomicBool::new(false), progress: Mutex::new(Vec::new()) }
    }

    fn context(&self) -> OperationContext<'_> {
        OperationContext::new(u64::MAX, AuthorityLimits::maximum(), self)
    }
}

impl AuthorityOperationControl for TestControl {
    fn now_ms(&self) -> u64 {
        0
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn report(&self, progress: AuthorityProgress) {
        self.progress.lock().expect("progress lock").push(progress);
    }
}

struct FixtureProviderSource<'a> {
    bindings: Vec<NativeCodecBinding>,
    calls: Mutex<Vec<String>>,
    failure: Option<&'static str>,
    cancel_after_preview: Option<&'a TestControl>,
    hostile: Option<&'static str>,
    exact_pool: bool,
}

impl FixtureProviderSource<'_> {
    fn new(bindings: Vec<NativeCodecBinding>) -> Self {
        Self { bindings, calls: Mutex::new(Vec::new()), failure: None, cancel_after_preview: None, hostile: None, exact_pool: true }
    }
}

impl NativeCodecProviderSourceV1 for FixtureProviderSource<'_> {
    fn preflight_selection(&self, selected: &[NativeCodecProviderRequirementV1<'_>]) -> Result<(), AuthorityError> {
        if self.exact_pool {
            let bindings = validate_native_bindings(&self.bindings)?;
            if bindings.keys().any(|key| {
                !selected.iter().any(|required| required.package.plugin_id == key.plugin_id && required.package.package_id == key.package_id && required.artifact_kind == key.artifact_kind && required.artifact_schema == key.artifact_schema)
            }) {
                return Err(catalog("no explicit native codec binding matches the selected closure"));
            }
        }
        Ok(())
    }

    fn preview(&self, package: NativeCodecProviderPackageV1<'_>, descriptor: &PackageDescriptor, _context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
        assert_eq!(descriptor.package_id, package.package_id);
        assert_eq!(descriptor.manifest.plugin_id, package.plugin_id);
        assert_eq!(descriptor.manifest.version, package.version);
        self.calls.lock().expect("provider calls").push(package.package_id.to_owned());
        if self.failure == Some(package.package_id) {
            return Err(catalog("selected fixture provider failed"));
        }
        let mut bindings = self.bindings.iter().filter(|binding| binding.plugin_id == package.plugin_id && binding.package_id == package.package_id).cloned().collect::<Vec<_>>();
        if package.plugin_id == "fixture.editor" {
            match self.hostile {
                Some("foreign") => bindings[0].package_id = "semio:unselected".into(),
                Some("missing") => bindings.clear(),
                Some("duplicate") => bindings.push(bindings[0].clone()),
                Some("zero") => bindings[0].codec.pack_schema_hash = [0; 32],
                Some("hash") => bindings[0].codec.pack_schema_hash = [0x22; 32],
                Some("extra") => {
                    let mut extra = bindings[0].clone();
                    extra.artifact_kind = "fixture.extra".into();
                    bindings.push(extra);
                }
                _ => {}
            }
        }
        if let Some(control) = self.cancel_after_preview {
            control.cancelled.store(true, Ordering::SeqCst);
        }
        Ok(bindings)
    }
}

struct FixtureDirectory {
    root: PathBuf,
    bundle_path: PathBuf,
    bundle: serde_json::Value,
    schema: String,
}

impl FixtureDirectory {
    fn persist_bundle(&self) {
        std::fs::write(&self.bundle_path, serde_json::to_vec_pretty(&self.bundle).expect("bundle json")).expect("write bundle");
    }

    fn refresh_profile_generation(&mut self) {
        let bundle: TrustedBundleV1 = serde_json::from_value(self.bundle.clone()).expect("fixture bundle");
        self.bundle["profiles"][0]["selectedClosureSha256"] = hex_lower(&selected_closure_digest(&bundle.profiles[0].selected_closure).expect("fixture closure digest")).into();
        self.bundle["profiles"][0]["generationId"] = trusted_profile_generation(&bundle, &bundle.profiles[0]).expect("fixture generation").into();
    }

    fn component_path(&self, package: usize) -> PathBuf {
        self.root.join(self.bundle["packages"][package]["component"]["path"].as_str().expect("component path"))
    }

    fn binding(&self) -> NativeCodecBinding {
        NativeCodecBinding::new("fixture.editor", "semio:fixture-editor", "s.fixture.document", fixture_codec(&self.schema, [0x11; 32]))
    }

    fn rewrite_descriptor(&mut self, index: usize, schema: Option<&str>, dependency: Option<(&str, &str)>) {
        let record = &mut self.bundle["packages"][index];
        let bytes = descriptor_bytes(
            record["pluginId"].as_str().expect("plugin"),
            record["packageId"].as_str().expect("package"),
            record["version"].as_str().expect("version"),
            record["component"]["sha256"].as_str().expect("component hash"),
            schema,
            dependency,
        );
        record["descriptor"]["byteLength"] = bytes.len().into();
        record["descriptor"]["sha256"] = hex_lower(&Sha256::digest(&bytes)).into();
        if record["browserActor"]["kind"] == "closed-browser-actor" {
            record["browserActor"]["sourceDescriptorByteSha256"] = record["descriptor"]["sha256"].clone();
        }
        attach_fixture_plugin_module(&self.root, record, &bytes);
        std::fs::write(self.root.join(record["descriptor"]["path"].as_str().expect("descriptor path")), bytes).expect("replace descriptor");
        self.refresh_profile_generation();
        self.persist_bundle();
    }

    fn make_two_codec_bindings(&mut self) -> Vec<NativeCodecBinding> {
        let schema = format!("{}.base", self.schema);
        self.bundle["packages"][1]["nativeCodecs"] = serde_json::json!([{ "artifactKind": "s.fixture.document", "artifactSchema": schema, "packSchemaHash": "11".repeat(32) }]);
        self.rewrite_descriptor(1, Some(&schema), None);
        vec![NativeCodecBinding::new("fixture.base", "semio:fixture-base", "s.fixture.document", fixture_codec(&schema, [0x11; 32])), self.binding()]
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// 🧩️ (Re)writes one fixture record's plugin module for its current descriptor bytes.
fn attach_fixture_plugin_module(root: &Path, record: &mut serde_json::Value, descriptor: &[u8]) {
    record["pluginModule"] = write_fixture_plugin_module(root, record["pluginId"].as_str().unwrap(), record["packageId"].as_str().unwrap(), record["version"].as_str().unwrap(), record["component"]["sha256"].as_str().unwrap(), descriptor).expect("fixture plugin module");
}

fn fixture_json() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/👥️two-package/🔣️.json")).expect("trusted-catalog fixture")
}

fn synthetic_browser_actor(component: &str, descriptor: &str, path: &str) -> serde_json::Value {
    let mut fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌐️browser-actor/🔣️.json")).unwrap();
    let mut actor = fixture["closed"].take();
    actor["sourceComponentSha256"] = component.into();
    actor["sourceDescriptorByteSha256"] = descriptor.into();
    actor["path"] = path.into();
    actor
}

fn local_stdio_gis_profile_bundle() -> TrustedBundleV1 {
    let version = "0.1.0".to_owned();
    let map_hash = "a1".repeat(32);
    let target = TrustedBundleOpenTargetV1 {
        artifact_kind: "s.gis.gismap".into(),
        artifact_schema: "gis.map".into(),
        pack_schema_hash: map_hash.clone(),
        surface_id: "s.gis.gismap@1/*#editor".into(),
        app_id: "s.gis.gismap@1/*#editor".into(),
        window_kind_id: "gis2d-main".into(),
        role: TrustedBundleOpenRole::Editor,
        renderer_target: TrustedBundleRendererTarget::Wasm,
        parent_dialect: semio_framework::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() },
        grant: TrustedBundleGrantV1 { read: true, write: true, observe: true },
    };
    let viewer = TrustedBundleOpenTargetV1 {
        surface_id: "s.gis.gismap@1/*#viewer".into(),
        app_id: "s.gis.gismap@1/*#viewer".into(),
        window_kind_id: "gis2d-view-map".into(),
        role: TrustedBundleOpenRole::Viewer,
        grant: TrustedBundleGrantV1 { read: true, write: false, observe: true },
        ..target.clone()
    };
    let gis_identity = TrustedBundleIdentityV1 { plugin_id: "gis".into(), package_id: "semio:gis".into(), version: version.clone() };
    let stdio_identity = TrustedBundleIdentityV1 { plugin_id: "stdio".into(), package_id: "semio:stdio".into(), version: version.clone() };
    let stdio_codecs = (0u8..26).map(|index| TrustedBundleCodecV1 { artifact_kind: format!("s.stdio.fixture{index:02}"), artifact_schema: format!("stdio.fixture{index:02}"), pack_schema_hash: format!("{:02x}", index + 1).repeat(32) }).collect();
    let packages = vec![
        TrustedBundlePackageV1 {
            plugin_id: "gis".into(),
            package_id: "semio:gis".into(),
            version: version.clone(),
            role: TrustedBundlePackageRole::Plugin,
            execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
            dependencies: vec![stdio_identity.clone()],
            component: TrustedBundleComponentV1 { path: "packages/gis/component.wasm".into(), byte_length: 1, sha256: "11".repeat(32), blake3: "12".repeat(32) },
            descriptor: TrustedBundleFileV1 { path: "packages/gis/descriptor.semio".into(), byte_length: 1, sha256: "13".repeat(32) },
            browser_actor: serde_json::from_value(synthetic_browser_actor(&"11".repeat(32), &"13".repeat(32), "packages/gis/browser/closed-actor.mjs")).unwrap(),
            plugin_module: TrustedBundlePluginModuleV1 { path: "packages/gis/plugin-module.json".into(), byte_length: 1, sha256: "14".repeat(32), blake3: "15".repeat(32) },
            native_codecs: vec![
                TrustedBundleCodecV1 { artifact_kind: "s.gis.gismap".into(), artifact_schema: "gis.map".into(), pack_schema_hash: map_hash },
                TrustedBundleCodecV1 { artifact_kind: "s.gis.gisterrain".into(), artifact_schema: "gis.terrain".into(), pack_schema_hash: "a2".repeat(32) },
            ],
            open_targets: vec![target.clone(), viewer.clone()],
        },
        TrustedBundlePackageV1 {
            plugin_id: "stdio".into(),
            package_id: "semio:stdio".into(),
            version: version.clone(),
            role: TrustedBundlePackageRole::Plugin,
            execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
            dependencies: vec![],
            component: TrustedBundleComponentV1 { path: "packages/stdio/component.wasm".into(), byte_length: 1, sha256: "21".repeat(32), blake3: "22".repeat(32) },
            descriptor: TrustedBundleFileV1 { path: "packages/stdio/descriptor.semio".into(), byte_length: 1, sha256: "23".repeat(32) },
            browser_actor: TrustedBundleBrowserActorV1::None {},
            plugin_module: TrustedBundlePluginModuleV1 { path: "packages/stdio/plugin-module.json".into(), byte_length: 1, sha256: "24".repeat(32), blake3: "25".repeat(32) },
            native_codecs: stdio_codecs,
            open_targets: vec![],
        },
    ];
    let selected_closure = vec![gis_identity.clone(), stdio_identity];
    let mut bundle = TrustedBundleV1 {
        schema_version: 3,
        profiles: vec![TrustedBundleProfileV1 {
            id: "local-stdio-gis-open-v1".into(),
            selected_closure,
            selected_closure_sha256: "01".repeat(32),
            open_targets: vec![TrustedBundleProfileOpenTargetV1 { package: gis_identity.clone(), target }, TrustedBundleProfileOpenTargetV1 { package: gis_identity, target: viewer }],
            generation_id: "02".repeat(32),
        }],
        packages,
    };
    bundle.profiles[0].selected_closure_sha256 = hex_lower(&selected_closure_digest(&bundle.profiles[0].selected_closure).expect("closure digest"));
    bundle.profiles[0].generation_id = trusted_profile_generation(&bundle, &bundle.profiles[0]).expect("profile generation");
    bundle
}

fn descriptor_bytes(plugin_id: &str, package_id: &str, version: &str, component_sha256: &str, schema: Option<&str>, dependency: Option<(&str, &str)>) -> Vec<u8> {
    let artifact_kinds = schema.map_or_else(Vec::new, |schema| {
        vec![serde_json::json!({
            "id": "s.fixture.document",
            "name": "Fixture Document",
            "sourceFormat": "fixture",
            "componentKind": "document",
            "dimension": "data",
            "mediaCapability": "meshOnly",
            "mediaType": { "class": "data", "form": "value" },
            "schema": schema,
            "exportFormats": [],
            "importFormats": [],
            "exportStdioKinds": [],
            "importStdioKinds": []
        })]
    });
    let apps = schema.map_or_else(Vec::new, |_| {
        [semio_framework::AppRole::Editor, semio_framework::AppRole::Viewer]
            .into_iter()
            .map(|role| {
                let role = role.as_str();
                serde_json::from_value::<semio_framework::AppDefinition>(serde_json::json!({
                    "id": format!("s.fixture.document@1/*#{role}"),
                    "role": role,
                    "dialect": { "artifactKind": "s.fixture.document", "standard": "1", "subset": "*" },
                    "label": {
                        "native": { "de": format!("Fixture {role}"), "en": format!("Fixture {role}") },
                        "reuse": { "de": format!("Fixture {role}"), "en": format!("Fixture {role}") }
                    },
                    "breadcrumb": ["semio", "fixture", role],
                    "controllerId": format!("fixture-{role}"),
                    "modes": [{
                        "id": if role == "editor" { "edit" } else { "view" },
                        "label": {
                            "native": { "de": if role == "editor" { "Bearbeiten" } else { "Ansehen" }, "en": if role == "editor" { "Edit" } else { "View" } },
                            "reuse": { "de": if role == "editor" { "Bearbeiten" } else { "Ansehen" }, "en": if role == "editor" { "Edit" } else { "View" } }
                        },
                        "iconId": if role == "editor" { "pencil" } else { "eye" },
                        "tools": [],
                        "commands": []
                    }],
                    "defaultModeId": if role == "editor" { "edit" } else { "view" },
                    "windowKinds": [{
                        "id": "fixture-main",
                        "label": {
                            "native": { "de": "Fixture", "en": "Fixture" },
                            "reuse": { "de": "Fixture", "en": "Fixture" }
                        },
                        "bodyKey": "fixture.main",
                        "surfaceKind": "canvas-2d",
                        "iconId": "app-window",
                        "actions": [],
                        "utilities": [],
                        "interactions": [],
                        "capabilities": []
                    }],
                    "panelTabs": [],
                    "keybindings": [],
                    "utilities": [],
                    "tools": [],
                    "commands": [],
                    "interactions": [],
                    "namedLayouts": [],
                    "terminologies": [],
                    "terminologyBreadcrumbs": {},
                    "tutorials": [],
                    "dialogs": [],
                    "mediaInputs": [],
                    "mediaOutputs": [],
                    "artifactKinds": [],
                    "config": { "fields": [] },
                    "commandGrammar": { "variants": [] },
                    "io": {
                        "artifactSchema": "fixture.document",
                        "artifactMediaType": { "class": "data", "form": "value" },
                        "ports": [],
                        "exportFormats": [],
                        "importFormats": [],
                        "artifact": { "id": "s.fixture.document", "name": "Fixture Document", "dimension": "data", "componentKind": "document" }
                    }
                }))
                .expect("handcrafted fixture app")
            })
            .collect()
    });
    let dependencies = dependency.map_or_else(Vec::new, |(plugin_id, version)| vec![serde_json::json!({ "pluginId": plugin_id, "version": format!("={version}") })]);
    let json = serde_json::json!({
        "descriptorVersion": 1,
        "packageId": package_id,
        "role": "plugin",
        "manifest": {
            "pluginId": plugin_id,
            "label": plugin_id,
            "version": version,
            "apps": apps,
            "examples": [],
            "artifactKinds": artifact_kinds,
            "dependencies": dependencies
        },
        "execution": "isolated",
        "executionProtocol": { "appChannelVersion": directory::os_spr::CHANNEL_VERSION },
        "hashes": {
            "wasmSha256": component_sha256,
            "coreWasmSha256": "22".repeat(32),
            "descriptorSha256": "33".repeat(32)
        }
    });
    let descriptor: PackageDescriptor = serde_json::from_value(json).expect("package descriptor");
    os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("project descriptor"))
}

fn prepared_fixture() -> FixtureDirectory {
    let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst);
    let schema = format!("fixture.document.catalog.{}.{}@1", std::process::id(), sequence);
    let root = crate::test_artifact_root::test_artifact_root().join(format!("semio-hub-trusted-catalog-{}-{sequence}", std::process::id()));
    std::fs::create_dir_all(root.join("components")).expect("component directory");
    std::fs::create_dir_all(root.join("descriptors")).expect("descriptor directory");
    std::fs::create_dir_all(root.join("browser")).expect("actor directory");
    std::fs::write(root.join("browser/closed-actor.mjs"), b"abc").expect("synthetic actor, never executed");
    let mut fixture = fixture_json();
    let mut bundle = fixture["bundle"].take();
    bundle["packages"][0]["nativeCodecs"][0]["artifactSchema"] = schema.clone().into();
    for target in bundle["packages"][0]["openTargets"].as_array_mut().expect("open targets") {
        target["artifactSchema"] = schema.clone().into();
    }
    for entry in bundle["profiles"][0]["openTargets"].as_array_mut().expect("profile open targets") {
        entry["target"]["artifactSchema"] = schema.clone().into();
    }
    let component = [b'a', b'b', b'c'];
    for index in 0..2 {
        let component_path = root.join(bundle["packages"][index]["component"]["path"].as_str().expect("component path"));
        std::fs::write(component_path, component).expect("write component");
    }
    let root_descriptor = descriptor_bytes("fixture.editor", "semio:fixture-editor", "1.2.3", fixture["componentSha256"].as_str().expect("sha256"), Some(&schema), Some(("fixture.base", "1.0.0")));
    let base_descriptor = descriptor_bytes("fixture.base", "semio:fixture-base", "1.0.0", fixture["componentSha256"].as_str().expect("sha256"), None, None);
    for (index, bytes) in [(0, root_descriptor), (1, base_descriptor)] {
        bundle["packages"][index]["descriptor"]["byteLength"] = bytes.len().into();
        bundle["packages"][index]["descriptor"]["sha256"] = hex_lower(&Sha256::digest(&bytes)).into();
        if index == 0 {
            bundle["packages"][index]["browserActor"]["sourceDescriptorByteSha256"] = bundle["packages"][index]["descriptor"]["sha256"].clone();
        }
        attach_fixture_plugin_module(&root, &mut bundle["packages"][index], &bytes);
        let path = root.join(bundle["packages"][index]["descriptor"]["path"].as_str().expect("descriptor path"));
        std::fs::write(path, bytes).expect("write descriptor");
    }
    let bundle_path = root.join("trusted-catalog.json");
    let mut fixture = FixtureDirectory { root, bundle_path, bundle, schema };
    fixture.refresh_profile_generation();
    fixture.persist_bundle();
    fixture
}

#[cfg(unix)]
fn fixture_file_link(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).expect("create fixture file symlink");
}

#[cfg(windows)]
fn fixture_file_link(target: &Path, link: &Path) {
    std::os::windows::fs::symlink_file(target, link).expect("create fixture file reparse point");
}

#[cfg(unix)]
fn fixture_directory_link(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).expect("create fixture directory symlink");
}

#[cfg(windows)]
fn fixture_directory_link(target: &Path, link: &Path) {
    std::os::windows::fs::symlink_dir(target, link).expect("create fixture directory reparse point");
}

async fn assert_linked_fixture_denied(fixture: &FixtureDirectory) {
    let provider = FixtureProviderSource::new(vec![fixture.binding()]);
    let result = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &TestControl::new().context()).await;
    assert!(result.is_err(), "linked trusted closure unexpectedly loaded");
    assert!(provider.calls.lock().expect("provider calls").is_empty(), "provider observed linked bytes");
    assert!(document_codec(&fixture.schema).await.expect("codec registry").is_none(), "linked bytes published a codec");
}

/// 🧪️ Loads real GIS assembly metadata and native receipts around synthetic component bytes; component execution is outside this fixture.
#[cfg(feature = "native-artifact-execution")]
async fn prepared_gis_binding_fixture(viewer: bool, foreign_service: bool) -> FixtureDirectory {
    let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new();
    semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_s_plugin_gis::plugin().expect("GIS assembly"));
    let emitted = semio_framework_plugin::describe::describe_plugin(&runtime).await;
    let mut descriptor = decode_package_descriptor(&emitted).expect("actual native GIS descriptor");
    semio_s_plugin_stdio::registry::validate_native_artifact_catalog_dependency(&descriptor.manifest.dependencies).expect("actual GIS compiled Stdio dependency");
    semio_s_plugin_stdio::registry::validate_native_artifact_catalog_contributions(&descriptor.manifest.topic_contributions).expect("actual GIS compiled Stdio catalog");
    let component = b"synthetic-gis-component-for-catalog-binding-test";
    let component_sha256 = hex_lower(&Sha256::digest(component));
    let mut component_blake3 = Hasher::new();
    component_blake3.update(component);
    descriptor.hashes.wasm_sha256 = component_sha256.clone();
    descriptor.hashes.core_wasm_sha256 = component_sha256.clone();
    descriptor.hashes.descriptor_sha256.clear();
    if foreign_service {
        descriptor.contributions.inference_services.iter_mut().find(|service| service.inference_schema == "s.gis.gismap.inference").expect("actual GIS inference declaration").contributor = "foreign".into();
    }
    descriptor.hashes.descriptor_sha256 = hex_lower(&Sha256::digest(&os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("GIS descriptor self-hash projection"))));
    let bytes = os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("project GIS descriptor"));
    let native_codecs: Vec<_> = semio_s_plugin_gis::native_codecs::native_codec_factory_receipts()
        .expect("actual GIS codec receipts")
        .into_iter()
        .map(|receipt| {
            let identity = receipt.identity();
            serde_json::json!({ "artifactKind": identity.artifact_kind, "artifactSchema": identity.schema, "packSchemaHash": hex_lower(&identity.pack_schema_hash) })
        })
        .collect();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json")).expect("neutral frozen binding corpus");
    let binding = &corpus["binding"];
    let package = serde_json::json!({ "pluginId": descriptor.manifest.plugin_id, "packageId": descriptor.package_id, "version": descriptor.manifest.version });
    let mut target = serde_json::json!({
        "artifactKind": binding["artifact"]["kind"], "artifactSchema": binding["artifact"]["schema"],
        "packSchemaHash": native_codecs.iter().find(|codec| codec["artifactKind"] == binding["artifact"]["kind"]).expect("Map native receipt")["packSchemaHash"],
        "surfaceId": binding["surface"]["surfaceId"], "appId": binding["surface"]["appId"], "windowKindId": binding["surface"]["windowKindId"],
        "role": binding["surface"]["role"], "rendererTarget": binding["surface"]["rendererTarget"],
        "parentDialect": binding["parentDialect"], "grant": binding["grant"]
    });
    if viewer {
        let app = descriptor.manifest.apps.iter().find(|app| app.role == semio_framework::AppRole::Viewer && app.dialect.artifact_kind == "s.gis.gismap").expect("actual Map viewer");
        target["surfaceId"] = app.id.clone().into();
        target["appId"] = app.id.clone().into();
        target["windowKindId"] = app.window_kinds.first().id.clone().into();
        target["role"] = "viewer".into();
        target["grant"]["write"] = false.into();
    }
    let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst);
    let root = crate::test_artifact_root::test_artifact_root().join(format!("gis-binding-catalog-{}-{sequence}", std::process::id()));
    std::fs::create_dir(&root).expect("exclusive GIS binding fixture directory");
    std::fs::write(root.join("component.wasm"), component).expect("write synthetic GIS component");
    std::fs::write(root.join("descriptor.semio"), &bytes).expect("write actual GIS descriptor");
    std::fs::write(root.join("closed-actor.mjs"), b"abc").expect("synthetic actor, never executed");
    let (stdio_identity, stdio_record) = headless_stdio_fixture_package(&root).expect("headless Stdio dependency package");
    let mut bundle = serde_json::json!({
        "schemaVersion": 3,
        "profiles": [{ "id": "frozen-gis-test", "selectedClosure": [package.clone(), stdio_identity.clone()], "selectedClosureSha256": "01".repeat(32),
            "openTargets": [{ "package": package.clone(), "target": target.clone() }], "generationId": "02".repeat(32) }],
        "packages": [{ "pluginId": package["pluginId"], "packageId": package["packageId"], "version": package["version"], "role": "plugin", "dependencies": [stdio_identity],
            "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
            "component": { "path": "component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
            "descriptor": { "path": "descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
            "browserActor": synthetic_browser_actor(&component_sha256, &hex_lower(&Sha256::digest(&bytes)), "closed-actor.mjs"),
            "nativeCodecs": native_codecs, "openTargets": [target] }, stdio_record]
    });
    attach_fixture_plugin_module(&root, &mut bundle["packages"][0], &bytes);
    let mut fixture = FixtureDirectory { bundle_path: root.join("trusted-catalog.json"), root, bundle, schema: "gis.map".into() };
    fixture.refresh_profile_generation();
    fixture.persist_bundle();
    fixture
}

#[cfg(feature = "native-artifact-execution")]
struct RecordingLinkedProvider {
    previews: Mutex<Vec<String>>,
    fail_gis: bool,
}

#[cfg(feature = "native-artifact-execution")]
impl NativeCodecProviderSourceV1 for RecordingLinkedProvider {
    fn preview(&self, package: NativeCodecProviderPackageV1<'_>, descriptor: &PackageDescriptor, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
        if self.fail_gis && package.plugin_id == "gis" {
            return Err(catalog("selected GIS provider fixture failure"));
        }
        let bindings = NativeCodecProviderSourceV1::preview(&NativeCodecProviderSetV1::linked(), package, descriptor, context)?;
        self.previews.lock().expect("successful private previews").push(package.plugin_id.to_owned());
        Ok(bindings)
    }
}

fn fixture_compile<'a>(_dsl: &'a str, _ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>> {
    Box::pin(async { Err(VcsError::Deserialize("fixture compile is not exercised".to_string())) })
}

fn fixture_print<'a>(_pack: &'a [u8], _spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>> {
    Box::pin(async { Ok(ArtifactTextFiles { dsl: String::new(), ops: String::new() }) })
}

fn fixture_edit<'a>(_envelope: &'a directory::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>> {
    Box::pin(async { Ok(String::new()) })
}

fn fixture_apply<'a>(pack: &'a [u8], spr: &'a [u8], _operations: &'a [u8]) -> directory::os_store::ArtifactCodecApplyFuture<'a> {
    Box::pin(async move { Ok((pack.to_vec(), spr.to_vec(), String::new())) })
}

fn fixture_codec(schema: &str, pack_schema_hash: [u8; 32]) -> ArtifactCodec {
    ArtifactCodec { schema: schema.to_string(), extension: "fixture", pack_schema_hash, compile_dsl: fixture_compile, print_mirror: fixture_print, edit_text_from_envelope: fixture_edit, apply_ops_binary: fixture_apply, replay_envelopes: fixture_apply }
}

async fn expect_load_error(fixture: &FixtureDirectory, bindings: &[NativeCodecBinding], control: &TestControl) -> AuthorityError {
    match load_fixture(fixture, bindings, &control.context()).await {
        Ok(_) => panic!("trusted catalog load unexpectedly succeeded"),
        Err(error) => error,
    }
}

async fn load_fixture(fixture: &FixtureDirectory, bindings: &[NativeCodecBinding], context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
    TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &FixtureProviderSource::new(bindings.to_vec()), context).await
}

#[tokio::test]
async fn selected_native_providers_are_descriptor_verified_dependency_first_and_only_selected() {
    let mut single = prepared_fixture();
    single.bundle["packages"][0]["dependencies"] = serde_json::json!([]);
    single.bundle["profiles"][0]["selectedClosure"] = serde_json::json!([
        { "pluginId": "fixture.editor", "packageId": "semio:fixture-editor", "version": "1.2.3" }
    ]);
    single.rewrite_descriptor(0, Some(&single.schema.clone()), None);
    let unselected = NativeCodecBinding::new("unselected", "semio:unselected", "fixture.unselected", fixture_codec(&format!("{}.unselected", single.schema), [0x11; 32]));
    let mut source = FixtureProviderSource::new(vec![single.binding(), unselected]);
    source.exact_pool = false;
    let catalog = TrustedCatalogLoader::load_fixture(&single.bundle_path, "fixture", &source, &TestControl::new().context()).await.expect("selected-only catalog");
    assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-editor"]);
    assert_eq!(catalog.packages().len(), 1);
    assert_eq!(catalog.codec_count(), 1);
    assert!(document_codec(&format!("{}.unselected", single.schema)).await.expect("registry").is_none());

    let mut pair = prepared_fixture();
    let source = FixtureProviderSource::new(pair.make_two_codec_bindings());
    let catalog = TrustedCatalogLoader::load_fixture(&pair.bundle_path, "fixture", &source, &TestControl::new().context()).await.expect("complete selected closure");
    assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-base", "semio:fixture-editor"]);
    assert_eq!(catalog.packages().len(), 2);
    assert_eq!(catalog.codec_count(), 2);
    assert!(document_codec(&pair.schema).await.expect("registry").is_some());
    assert!(document_codec(&format!("{}.base", pair.schema)).await.expect("registry").is_some());
}

#[cfg(feature = "native-artifact-execution")]
#[tokio::test]
async fn gis_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication() {
    struct SelectionControl {
        cancelled: bool,
        now_ms: u64,
    }
    impl AuthorityOperationControl for SelectionControl {
        fn now_ms(&self) -> u64 {
            self.now_ms
        }
        fn is_cancelled(&self) -> bool {
            self.cancelled
        }
        fn report(&self, _progress: AuthorityProgress) {}
    }
    let _registry = crate::artifact_authority::REAL_LINKED_CODEC_REGISTRY.lock().await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📇️native-openable-provider/🧫️fixtures/🌍️gis-v1/🔣️.json")).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json")).unwrap();
    assert_eq!(fixture["packageVersion"], expected["packageVersion"]);
    let providers = NativeCodecProviderSetV1::linked();
    let mut published = Vec::new();
    for row in expected["receipts"].as_array().unwrap() {
        published.push(document_codec(row["schema"].as_str().unwrap()).await.unwrap().map(|codec| codec.pack_schema_hash));
    }
    for case in fixture["cases"].as_array().unwrap() {
        let control = SelectionControl { cancelled: case["cancelled"].as_bool().unwrap(), now_ms: case["nowMs"].as_u64().unwrap() };
        let context = OperationContext::new(case["deadlineMs"].as_u64().unwrap(), AuthorityLimits::maximum(), &control);
        let result = providers.preview(case["pluginId"].as_str().unwrap(), case["packageId"].as_str().unwrap(), case["version"].as_str().unwrap(), &context);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
        if let Ok(bindings) = result {
            assert_eq!(bindings.len(), case["bindings"].as_u64().unwrap() as usize, "{}", case["name"]);
            assert_eq!(bindings.is_empty(), case["code"] == "unlinked-package", "{}", case["name"]);
            for (binding, row) in bindings.iter().zip(expected["receipts"].as_array().unwrap()) {
                assert_eq!(binding.plugin_id, expected["pluginId"]);
                assert_eq!(binding.package_id, expected["packageId"]);
                assert_eq!(binding.artifact_kind, row["kind"]);
                assert_eq!(binding.codec.schema, row["schema"]);
                assert_eq!(binding.codec.extension, row["extension"]);
                let receipt = semio_s_plugin_gis::native_codecs::native_codec_factory_receipts().unwrap().into_iter().find(|receipt| receipt.identity().schema == binding.codec.schema).unwrap();
                assert_eq!(hex_lower(&receipt.identity().protocol_sha256), row["protocolSha256"]);
                assert_eq!(binding.codec.pack_schema_hash, receipt.into_codec().unwrap().pack_schema_hash);
            }
        }
        for (row, prior) in expected["receipts"].as_array().unwrap().iter().zip(&published) {
            assert_eq!(&document_codec(row["schema"].as_str().unwrap()).await.unwrap().map(|codec| codec.pack_schema_hash), prior, "{} must not publish even a partial codec closure", case["name"]);
        }
    }
}

#[tokio::test]
async fn selected_native_provider_failure_substitution_and_conflict_publish_no_partial_closure() {
    for hostile in ["foreign", "missing", "duplicate", "zero", "hash", "extra", "provider-failure", "registry-conflict"] {
        let mut fixture = prepared_fixture();
        let mut source = FixtureProviderSource::new(fixture.make_two_codec_bindings());
        source.hostile = Some(hostile);
        if hostile == "provider-failure" {
            source.failure = Some("semio:fixture-editor");
        }
        if hostile == "registry-conflict" {
            os_store::register_document_codec(fixture_codec(&fixture.schema, [0x22; 32])).expect("prior immutable owner");
        }
        assert!(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &source, &TestControl::new().context()).await.is_err(), "{hostile}");
        assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-base", "semio:fixture-editor"], "{hostile}");
        assert!(document_codec(&format!("{}.base", fixture.schema)).await.expect("registry").is_none(), "{hostile} published first provider");
        let existing = document_codec(&fixture.schema).await.expect("registry");
        if hostile == "registry-conflict" {
            assert_eq!(existing.expect("prior owner retained").pack_schema_hash, [0x22; 32]);
        } else {
            assert!(existing.is_none(), "{hostile} published second provider");
        }
    }
    #[cfg(feature = "native-artifact-execution")]
    {
        let fixture = prepared_fixture();
        assert!(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &NativeCodecProviderSetV1::linked(), &TestControl::new().context()).await.is_err());
        assert!(document_codec(&fixture.schema).await.expect("registry").is_none());
    }
}

#[tokio::test]
async fn selected_native_provider_descriptor_and_cancellation_fences_precede_publication() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
    for case in cases["descriptorPreviewCases"].as_array().unwrap() {
        let mut invalid = prepared_fixture();
        let source = FixtureProviderSource::new(invalid.make_two_codec_bindings());
        let index = case["packageIndex"].as_u64().unwrap() as usize;
        let descriptor_path = invalid.root.join(invalid.bundle["packages"][index]["descriptor"]["path"].as_str().expect("descriptor path"));
        std::fs::write(descriptor_path, b"invalid descriptor").expect("hostile descriptor bytes");
        assert!(TrustedCatalogLoader::load_fixture(&invalid.bundle_path, "fixture", &source, &TestControl::new().context()).await.is_err());
        assert_eq!(serde_json::to_value(source.calls.lock().expect("calls").clone()).unwrap(), case["previews"], "{}", case["id"]);
        assert!(document_codec(&invalid.schema).await.expect("registry").is_none());
        assert!(document_codec(&format!("{}.base", invalid.schema)).await.expect("registry").is_none());
    }

    for before in [true, false] {
        let mut fixture = prepared_fixture();
        let control = TestControl::new();
        control.cancelled.store(before, Ordering::SeqCst);
        let mut source = FixtureProviderSource::new(fixture.make_two_codec_bindings());
        source.cancel_after_preview = Some(&control);
        let result = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &source, &control.context()).await;
        assert!(matches!(result, Err(AuthorityError::Cancelled)));
        let calls = source.calls.lock().expect("calls").clone();
        assert_eq!(calls, if before { Vec::<String>::new() } else { vec!["semio:fixture-base".into()] });
        assert!(document_codec(&fixture.schema).await.expect("registry").is_none());
        assert!(document_codec(&format!("{}.base", fixture.schema)).await.expect("registry").is_none());
    }
}

#[tokio::test]
async fn neutral_fixture_proves_dependency_order_hash_oracles_and_exact_limit_edges() {
    let fixture = fixture_json();
    let bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle shape");
    assert_eq!(validate_bundle(&bundle, "fixture").expect("valid closure").package_indices, vec![1, 0]);
    let bytes = fixture["componentHex"].as_str().expect("component hex").as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), 16).expect("hex byte")).collect::<Vec<_>>();
    let control = TestControl::new();
    let (sha, internal_blake3) = dual_hash(&bytes, &control.context()).await.expect("dual hash");
    assert_eq!(hex_lower(&sha), fixture["componentSha256"]);
    assert_eq!(hex_lower(&internal_blake3), fixture["componentBlake3"]);
    assert_eq!(blake3::hash(&bytes).to_hex().as_str(), fixture["componentBlake3"]);

    let limits = &fixture["limits"];
    assert_eq!(limits["componentBytesMax"], TRUSTED_COMPONENT_MAX_BYTES);
    assert_eq!(limits["componentBytesMaxPlusOne"], TRUSTED_COMPONENT_MAX_BYTES + 1);
    assert!(validate_file(&TrustedBundleFileV1 { path: "component.wasm".to_string(), byte_length: TRUSTED_COMPONENT_MAX_BYTES, sha256: "11".repeat(32) }, TRUSTED_COMPONENT_MAX_BYTES).is_ok());
    assert!(validate_file(&TrustedBundleFileV1 { path: "component.wasm".to_string(), byte_length: TRUSTED_COMPONENT_MAX_BYTES + 1, sha256: "11".repeat(32) }, TRUSTED_COMPONENT_MAX_BYTES).is_err());
    assert_eq!(limits["descriptorBytesMax"], TRUSTED_DESCRIPTOR_MAX_BYTES);
    assert!(validate_file(&TrustedBundleFileV1 { path: "descriptor.semio".to_string(), byte_length: TRUSTED_DESCRIPTOR_MAX_BYTES, sha256: "11".repeat(32) }, TRUSTED_DESCRIPTOR_MAX_BYTES).is_ok());
    assert!(validate_file(&TrustedBundleFileV1 { path: "descriptor.semio".to_string(), byte_length: TRUSTED_DESCRIPTOR_MAX_BYTES + 1, sha256: "11".repeat(32) }, TRUSTED_DESCRIPTOR_MAX_BYTES).is_err());
    assert!(valid_identity(&"a".repeat(TRUSTED_IDENTITY_MAX_BYTES)));
    assert!(!valid_identity(&"a".repeat(TRUSTED_IDENTITY_MAX_BYTES + 1)));
    assert!(ensure_count(TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_CODECS, "trusted codec count").is_ok());
    assert!(ensure_count(TRUSTED_CATALOG_MAX_CODECS + 1, TRUSTED_CATALOG_MAX_CODECS, "trusted codec count").is_err());
}

#[tokio::test]
async fn loader_retains_exact_bytes_and_independent_identities_before_atomic_codec_activation() {
    let fixture = prepared_fixture();
    let binding = fixture.binding();
    let control = TestControl::new();
    let catalog = load_fixture(&fixture, &[binding], &control.context()).await.expect("verified catalog");
    assert_eq!(catalog.packages().iter().map(VerifiedTrustedPackage::plugin_id).collect::<Vec<_>>(), vec!["fixture.base", "fixture.editor"]);
    let editor = &catalog.packages()[1];
    assert_eq!(editor.package_ref().package.0, "semio:fixture-editor");
    assert_eq!(hex_lower(&editor.package_ref().hash.0), fixture_json()["componentBlake3"]);
    assert_ne!(editor.plugin_id(), editor.package_ref().package.0);
    assert_eq!(editor.component().byte_length(), 3);
    assert_eq!(&*editor.component().read(&control.context()).await.expect("retained component rereads"), b"abc");
    assert_eq!(hex_lower(editor.component_sha256()), fixture_json()["componentSha256"]);
    assert_eq!(editor.descriptor().manifest.plugin_id, editor.plugin_id());
    assert_eq!(editor.descriptor().manifest.version, editor.version());
    assert_eq!(Sha256::digest(editor.descriptor_bytes()), *editor.descriptor_sha256());
    assert_eq!(catalog.codec_count(), 1);
    assert_eq!(catalog.open_target_count(), 1);
    assert_eq!(catalog.generation_id().len(), 64);
    let descriptor = DocumentDescriptor {
        space_id: "space".into(),
        document_id: "document".into(),
        artifact_kind: "s.fixture.document".into(),
        artifact_schema: fixture.schema.clone(),
        owner: directory::os_directory::DocumentOwner {
            plugin_id: "fixture.editor".into(),
            package_id: "semio:fixture-editor".into(),
            version: "1.2.3".into(),
            package_hash: fixture_json()["componentSha256"].as_str().expect("component sha256").into(),
        },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    };
    assert!(catalog.resolve(&TrustedArtifactIdentity::from_descriptor(&descriptor)).await.is_ok(), "the codec and open plan use the same descriptor SHA-256 owner identity");
    let editor = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("exact editor target");
    assert_eq!(editor.surface.role, DocumentOpenSurfaceRoleV1::Editor);
    assert!(editor.grant.write);
    assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), false).is_none());
    assert!(catalog.resolve_document_open(&descriptor, None, true).is_some());
    assert!(document_codec(&fixture.schema).await.expect("codec registry").is_some());
    let progress = control.progress.lock().expect("progress lock");
    assert_eq!(progress.first().map(|entry| entry.stage), Some(AuthorityProgressStage::Preflight));
    assert_eq!(progress.last().map(|entry| entry.stage), Some(AuthorityProgressStage::CatalogResolved));
    assert!(progress.iter().all(|entry| entry.completed_units <= entry.total_units));
}

/// 🧱 The exact-selection asset accessor is bound to the current generation and to every
/// selected digest: it answers only for the exact descriptor/role/surface the catalog itself
/// resolves, only while the caller-observed generation is still this catalog's own, and the
/// bytes it returns are the very bytes whose SHA-256/BLAKE3 the selection projects.
#[tokio::test]
async fn selected_execution_target_assets_are_generation_and_digest_bound() {
    let fixture = prepared_fixture();
    let control = TestControl::new();
    let catalog = load_fixture(&fixture, &[fixture.binding()], &control.context()).await.expect("verified catalog");
    let descriptor = DocumentDescriptor {
        space_id: "space".into(),
        document_id: "document".into(),
        artifact_kind: "s.fixture.document".into(),
        artifact_schema: fixture.schema.clone(),
        owner: directory::os_directory::DocumentOwner {
            plugin_id: "fixture.editor".into(),
            package_id: "semio:fixture-editor".into(),
            version: "1.2.3".into(),
            package_hash: fixture_json()["componentSha256"].as_str().expect("component sha256").into(),
        },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    };
    let generation = catalog.generation_id().to_string();
    let assets = catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), true, &generation).expect("selected assets");
    let selection = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("selection");
    assert_eq!(assets.selection, selection);
    let component = assets.component.read(&control.context()).await.expect("selected component rereads");
    assert!(!component.is_empty() && !assets.descriptor.is_empty());
    assert_eq!(hex_lower(&Sha256::digest(&component)), assets.selection.package.component_sha256);
    assert_eq!(semio_framework_hash::hash_bytes(&component), assets.selection.package.component_blake3);
    assert_eq!(hex_lower(&Sha256::digest(&assets.descriptor)), assets.selection.package.descriptor_byte_sha256);
    let actor = assets.browser_actor.as_ref().expect("Wasm selection retains its actor").read(&control.context()).await.expect("selected actor rereads");
    let retained = catalog.packages.iter().find(|package| package.plugin_id == selection.package.plugin_id).unwrap();
    assert!(Arc::ptr_eq(&actor, &retained.browser_actor_asset.as_ref().unwrap().read(&control.context()).await.expect("resident actor")));
    assert_eq!(actor.as_ref(), b"abc");
    assert_eq!(assets.selection.browser_actor, retained.browser_actor);
    assert!(assets.component.byte_length() <= TRUSTED_COMPONENT_MAX_BYTES && assets.descriptor.len() as u64 <= TRUSTED_DESCRIPTOR_MAX_BYTES);
    // 🔁 A rotated (or merely guessed) generation is never served, and no role, surface or
    // descriptor substitution reaches bytes.
    assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), true, &"ab".repeat(32)).is_none());
    assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), false, &generation).is_none());
    assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#viewer"), true, &generation).is_none());
    assert!(catalog.assets_for_current_selection(&descriptor, Some("foreign"), true, &generation).is_none());
    for change in ["plugin", "package", "version", "hash", "kind", "schema", "pack"] {
        let mut candidate = descriptor.clone();
        match change {
            "plugin" => candidate.owner.plugin_id.push_str(".foreign"),
            "package" => candidate.owner.package_id.push_str(".foreign"),
            "version" => candidate.owner.version.push_str("-foreign"),
            "hash" => candidate.owner.package_hash = "ab".repeat(32),
            "kind" => candidate.artifact_kind.push_str(".foreign"),
            "schema" => candidate.artifact_schema.push_str("-foreign"),
            "pack" => candidate.pack_schema_hash = "ab".repeat(32),
            _ => unreachable!(),
        }
        assert!(catalog.assets_for_current_selection(&candidate, Some("s.fixture.document@1/*#editor"), true, &generation).is_none(), "descriptor {change} reached selected bytes");
    }
}

#[tokio::test]
async fn verified_trusted_catalog_document_open_generation_and_resolution_are_exact() {
    let fixture = prepared_fixture();
    let catalog = load_fixture(&fixture, &[fixture.binding()], &TestControl::new().context()).await.expect("verified catalog");
    let descriptor = DocumentDescriptor {
        space_id: "space".into(),
        document_id: "document".into(),
        artifact_kind: "s.fixture.document".into(),
        artifact_schema: fixture.schema.clone(),
        owner: directory::os_directory::DocumentOwner {
            plugin_id: "fixture.editor".into(),
            package_id: "semio:fixture-editor".into(),
            version: "1.2.3".into(),
            package_hash: fixture_json()["componentSha256"].as_str().expect("component sha256").into(),
        },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    };
    assert_eq!(catalog.open_target_count(), 1);
    assert_eq!(catalog.generation_id().len(), 64);
    assert!(catalog.generation_id().bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')));
    let editor = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("editor");
    assert_eq!(editor.surface.role, DocumentOpenSurfaceRoleV1::Editor);
    assert_eq!(editor.grant, DocumentOpenGrantV1 { read: true, write: true, observe: true });
    assert_eq!(editor.parent_dialect, semio_framework::ArtifactDialect { artifact_kind: "s.fixture.document".into(), standard: "1".into(), subset: "*".into() });
    assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), false).is_none());
    for field in ["artifactKind", "standard", "subset"] {
        let mut changed: TrustedBundleV1 = serde_json::from_value(fixture.bundle.clone()).expect("bundle");
        let dialect = &mut changed.profiles[0].open_targets[0].target.parent_dialect;
        match field {
            "artifactKind" => dialect.artifact_kind.push_str(".foreign"),
            "standard" => dialect.standard.push_str("-foreign"),
            "subset" => dialect.subset.push_str("-foreign"),
            _ => unreachable!(),
        }
        let changed_generation = trusted_profile_generation(&changed, &changed.profiles[0]).unwrap();
        assert_ne!(changed_generation, catalog.generation_id(), "catalog binds parent {field}");
    }
    eprintln!("[DEBUG] trusted open catalog retained verified editor/viewer parent dialect;3 field changes alter generation");
    assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), true).is_none());
    assert!(catalog.resolve_document_open(&descriptor, Some("foreign"), false).is_none());
    let roles: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️identity-roles/🔣️.json")).unwrap();
    for case in roles["cases"].as_array().unwrap() {
        let mut candidate = descriptor.clone();
        let mut surface = "s.fixture.document@1/*#editor".to_owned();
        match case["change"].as_str().unwrap() {
            "none" => {}
            "blake3-owner" => candidate.owner.package_hash = fixture_json()["componentBlake3"].as_str().unwrap().to_owned(),
            "descriptor-owner" => candidate.owner.package_hash = hex_lower(catalog.packages()[1].descriptor_sha256()),
            "zero-owner" => candidate.owner.package_hash = "00".repeat(32),
            "bare-kind" => candidate.artifact_kind = "fixture.document".to_owned(),
            "bare-surface" => surface = "fixture.document@1/*#editor".to_owned(),
            _ => panic!("unknown catalog identity role vector"),
        }
        assert_eq!(catalog.resolve(&TrustedArtifactIdentity::from_descriptor(&candidate)).await.is_ok(), case["codec"].as_bool().unwrap(), "codec {}", case["change"]);
        assert_eq!(catalog.resolve_document_open(&candidate, Some(&surface), true).is_some(), case["open"].as_bool().unwrap(), "open {}", case["change"]);
    }
}

#[tokio::test]
async fn trusted_browser_actor_loader_verifies_retains_and_cancels_before_publication() {
    struct ActorControl {
        control: TestControl,
        cancel_after_descriptor: bool,
    }
    impl AuthorityOperationControl for ActorControl {
        fn now_ms(&self) -> u64 {
            0
        }
        fn is_cancelled(&self) -> bool {
            self.control.is_cancelled()
        }
        fn report(&self, progress: AuthorityProgress) {
            if self.cancel_after_descriptor && progress.stage == AuthorityProgressStage::CatalogLoading && progress.completed_units == 6 {
                self.control.cancelled.store(true, Ordering::SeqCst);
            }
            self.control.report(progress);
        }
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌐️browser-actor/🔣️.json")).unwrap();
    for law in corpus["loadCases"].as_array().unwrap() {
        let mut fixture = prepared_fixture();
        let path = fixture.root.join(fixture.bundle["packages"][0]["browserActor"]["path"].as_str().unwrap());
        if let Some(hex) = law["bodyHex"].as_str() {
            let bytes: Vec<u8> = hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
            std::fs::write(&path, bytes).unwrap();
        } else {
            std::fs::remove_file(&path).unwrap();
        }
        fixture.bundle["packages"][0]["browserActor"]["byteLength"] = law["byteLength"].clone();
        fixture.refresh_profile_generation();
        fixture.persist_bundle();
        let control = ActorControl { control: TestControl::new(), cancel_after_descriptor: law["cancelAfterDescriptor"].as_bool().unwrap() };
        let provider = FixtureProviderSource::new(vec![fixture.binding()]);
        let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
        let result = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &context).await;
        let accepted = law["accepted"].as_bool().unwrap();
        assert_eq!(result.is_ok(), accepted, "{}", law["id"]);
        assert_eq!(document_codec(&fixture.schema).await.unwrap().is_some(), accepted, "{} codec publication", law["id"]);
        let calls = provider.calls.lock().unwrap();
        assert_eq!(calls.iter().any(|package| package == "semio:fixture-editor"), accepted, "{} provider preview", law["id"]);
        if let Ok(catalog) = result {
            let package = catalog.packages.iter().find(|package| package.plugin_id == "fixture.editor").unwrap();
            let asset = package.browser_actor_asset.as_ref().unwrap();
            let bytes = asset.read(&context).await.expect("verified actor rereads");
            assert_eq!(bytes.as_ref(), b"abc");
            let selected = catalog.selected_document_open().unwrap();
            assert_eq!(selected.browser_actor, package.browser_actor);
            std::fs::write(&path, b"untrusted replacement").unwrap();
            assert_eq!(bytes.as_ref(), b"abc");
            let DocumentOpenBrowserActorV1::ClosedBrowserActor { sha256, source_component_sha256, source_descriptor_byte_sha256, .. } = &selected.browser_actor else { panic!("required actor") };
            assert_eq!(*sha256, hex_lower(&Sha256::digest(&bytes)));
            assert_eq!(*source_component_sha256, selected.package.component_sha256);
            assert_eq!(*source_descriptor_byte_sha256, selected.package.descriptor_byte_sha256);
            assert!(Arc::ptr_eq(&bytes, &asset.read(&context).await.expect("held actor stays resident")));
            drop(bytes);
            assert!(asset.read(&context).await.is_err(), "a replaced actor file is refused once no reader holds the verified bytes");
        } else if control.cancel_after_descriptor {
            assert!(matches!(result, Err(AuthorityError::Cancelled)));
        }
    }
    eprintln!("[DEBUG] trusted browser actor loader:8 neutral body/cancellation vectors; retained synthetic bytes never executed");
}

#[tokio::test]
async fn trusted_catalog_opened_root_rejects_linked_roots_leaves_intermediates_and_actors() {
    let leaf = prepared_fixture();
    let leaf_path = leaf.component_path(0);
    let leaf_target = leaf.root.join("components/retained-component.wasm");
    std::fs::write(&leaf_target, std::fs::read(&leaf_path).expect("read component")).expect("write retained component");
    std::fs::remove_file(&leaf_path).expect("remove component leaf");
    fixture_file_link(&leaf_target, &leaf_path);
    assert_linked_fixture_denied(&leaf).await;

    let intermediate = prepared_fixture();
    let components = intermediate.root.join("components");
    let retained_components = intermediate.root.join("retained-components");
    std::fs::rename(&components, &retained_components).expect("retain component directory");
    fixture_directory_link(&retained_components, &components);
    assert_linked_fixture_denied(&intermediate).await;

    let actor = prepared_fixture();
    let actor_path = actor.root.join("browser/closed-actor.mjs");
    let actor_target = actor.root.join("browser/retained-actor.mjs");
    std::fs::write(&actor_target, std::fs::read(&actor_path).expect("read actor")).expect("write retained actor");
    std::fs::remove_file(&actor_path).expect("remove actor leaf");
    fixture_file_link(&actor_target, &actor_path);
    assert_linked_fixture_denied(&actor).await;

    let owner_fixture = prepared_fixture();
    let actual_data = owner_fixture.root.join("owned-data");
    std::fs::create_dir(&actual_data).expect("create owned data root");
    let linked_data = owner_fixture.root.join("linked-data");
    fixture_directory_link(&actual_data, &linked_data);
    let canonical_owner_root = std::fs::canonicalize(&owner_fixture.root).expect("canonical fixture-owned parent root");
    assert!(TrustedCatalogDataRoot::open_server_owned(&canonical_owner_root.join("linked-data")).is_err(), "linked initial data root was admitted");

    let trusted = actual_data.join("trusted-catalog");
    std::fs::create_dir(&trusted).expect("create trusted root");
    let current_target = actual_data.join("retained-current.json");
    std::fs::write(&current_target, b"{}\n").expect("write current target");
    fixture_file_link(&current_target, &trusted.join("current.json"));
    let canonical_data = std::fs::canonicalize(&actual_data).expect("canonical fixture-owned data root");
    let data_root = TrustedCatalogDataRoot::open_server_owned(&canonical_data).expect("open fixture-owned data root");
    assert!(data_root.open_current().is_err(), "linked current pointer was admitted");

    let generations = trusted.join("generations");
    std::fs::create_dir(&generations).expect("create generations root");
    let retained_generation = trusted.join("retained-generation");
    std::fs::create_dir(&retained_generation).expect("create retained generation");
    let generation_id = "ab".repeat(32);
    fixture_directory_link(&retained_generation, &generations.join(&generation_id));
    assert!(data_root.open_generation(&generation_id).is_err(), "linked generation root was admitted");
}

#[tokio::test]
async fn trusted_catalog_opened_handle_is_swap_stable_bounded_and_cancel_safe() {
    let fixture = prepared_fixture();
    let canonical_root = std::fs::canonicalize(&fixture.root).expect("canonical fixture-owned generation root");
    let root = TrustedCatalogGenerationRoot::open_fixture_owned(&canonical_root).expect("opened generation root");
    let relative = TrustedCatalogRelativePathV1::parse(fixture.bundle["packages"][0]["component"]["path"].as_str().expect("component path")).expect("relative component path");
    let opened = root.open_regular(&relative).expect("open retained component handle");
    let component_path = fixture.component_path(0);
    std::fs::rename(&component_path, fixture.root.join("original-component.wasm")).expect("retain opened component inode");
    std::fs::write(&component_path, b"xyz").expect("write substituted component path");
    assert_eq!(opened.read_bounded(3, &TestControl::new().context()).await.expect("read retained handle"), b"abc");
    let provider = FixtureProviderSource::new(vec![fixture.binding()]);
    assert!(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &TestControl::new().context()).await.is_err(), "substituted component digest loaded");
    assert!(provider.calls.lock().expect("provider calls").is_empty());
    assert!(document_codec(&fixture.schema).await.expect("codec registry").is_none());

    let zero_path = fixture.root.join("zero.bin");
    std::fs::write(&zero_path, []).expect("write zero file");
    let zero = root.open_regular(&TrustedCatalogRelativePathV1::parse("zero.bin").expect("zero path")).expect("open zero file");
    assert!(zero.read_bounded(1, &TestControl::new().context()).await.is_err());

    let over_path = fixture.root.join("over.bin");
    std::fs::write(&over_path, b"abcd").expect("write over-bound file");
    let over = root.open_regular(&TrustedCatalogRelativePathV1::parse("over.bin").expect("over path")).expect("open over-bound file");
    assert!(over.read_bounded(3, &TestControl::new().context()).await.is_err());

    let growth_path = fixture.root.join("growth.bin");
    std::fs::write(&growth_path, b"abc").expect("write growth file");
    let growth = root.open_regular(&TrustedCatalogRelativePathV1::parse("growth.bin").expect("growth path")).expect("open growth file");
    std::fs::OpenOptions::new().append(true).open(&growth_path).expect("open growth writer").write_all(b"d").expect("grow opened file");
    assert!(growth.read_bounded(4, &TestControl::new().context()).await.is_err(), "opened handle admitted growth within the caller maximum");

    #[cfg(unix)]
    {
        let fifo_path = fixture.root.join("special.fifo");
        create_fifo_fixture(&fifo_path).expect("create FIFO fixture");
        assert!(root.open_regular(&TrustedCatalogRelativePathV1::parse("special.fifo").expect("FIFO path")).is_err(), "opened root admitted a blocking special-file leaf");
    }

    struct CancelAfterTwoCheckpoints(AtomicUsize);
    impl AuthorityOperationControl for CancelAfterTwoCheckpoints {
        fn now_ms(&self) -> u64 {
            0
        }
        fn is_cancelled(&self) -> bool {
            self.0.fetch_add(1, Ordering::SeqCst) >= 2
        }
        fn report(&self, _progress: AuthorityProgress) {}
    }
    let large_path = fixture.root.join("large.bin");
    std::fs::write(&large_path, vec![7u8; 64 * 1024 + 1]).expect("write multi-chunk file");
    let large = root.open_regular(&TrustedCatalogRelativePathV1::parse("large.bin").expect("large path")).expect("open multi-chunk file");
    let cancel = CancelAfterTwoCheckpoints(AtomicUsize::new(0));
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &cancel);
    assert!(matches!(large.read_bounded(64 * 1024 + 1, &context).await, Err(AuthorityError::Cancelled)));
}

#[test]
fn trusted_catalog_relative_paths_match_the_neutral_no_link_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛡️opened-root/🔣️.json")).expect("opened-root corpus");
    let rows = corpus["relativePaths"].as_array().expect("relative path rows");
    assert_eq!(rows.len(), 15);
    for row in rows {
        let value = row["value"].as_str().expect("relative path value");
        assert_eq!(TrustedCatalogRelativePathV1::parse(value).is_ok(), row["accepted"].as_bool().expect("relative path acceptance"), "{}", row["id"]);
    }
}

#[test]
fn descriptor_projection_rejects_package_conflicts_unknown_fields_and_duplicate_fields() {
    let fixture = fixture_json();
    let bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    let component_sha256 = fixture["componentSha256"].as_str().expect("sha256");
    let canonical = descriptor_bytes("fixture.editor", "semio:fixture-editor", "1.2.3", component_sha256, Some("fixture.document@1"), Some(("fixture.base", "1.0.0")));
    decode_package_descriptor(&canonical).unwrap_or_else(|error| panic!("canonical descriptor must decode: {error}"));

    let conflicting = decode_package_descriptor(&descriptor_bytes("fixture.editor", "semio:other-package", "1.2.3", component_sha256, Some("fixture.document@1"), Some(("fixture.base", "1.0.0")))).expect("structurally valid conflicting descriptor");
    assert!(validate_descriptor(&bundle.packages[0], &conflicting, &bundle.packages).expect_err("package mismatch").to_string().contains("identity"));

    let mut unknown = os_store::pack_rt::decode_wire_value(&canonical).expect("canonical value");
    let DslValue::Object(fields) = &mut unknown else { panic!("descriptor object") };
    fields.push(("unexpected".to_owned(), DslValue::Null));
    let error = decode_package_descriptor(&os_store::pack_rt::encode_wire_value(&unknown)).expect_err("unknown descriptor field");
    assert!(error.to_string().contains("unknown field") || error.to_string().contains("canonical schema projection"));

    let mut duplicate = os_store::pack_rt::decode_wire_value(&canonical).expect("canonical value");
    let DslValue::Object(fields) = &mut duplicate else { panic!("descriptor object") };
    fields.push(("packageId".to_owned(), DslValue::String("semio:fixture-editor".to_owned())));
    assert!(decode_package_descriptor(&os_store::pack_rt::encode_wire_value(&duplicate)).expect_err("duplicate descriptor field").to_string().contains("duplicate object field"));
    assert!(valid_package_id("semio:fixture-editor"));
    assert!(!valid_package_id("fixture.editor.native"));
    assert!(!valid_package_id("semio:Fixture"));
}

#[tokio::test]
async fn all_trust_failures_precede_activation_and_have_bounded_diagnostics() {
    let mut mutated = prepared_fixture();
    std::fs::write(mutated.component_path(0), b"abd").expect("mutate component");
    let control = TestControl::new();
    let error = expect_load_error(&mutated, &[mutated.binding()], &control).await;
    assert!(error.to_string().contains("digest"));
    assert!(document_codec(&mutated.schema).await.expect("codec registry").is_none());

    let mut descriptor_mutation = prepared_fixture();
    let descriptor_path = descriptor_mutation.root.join(descriptor_mutation.bundle["packages"][0]["descriptor"]["path"].as_str().expect("descriptor path"));
    let mut bytes = std::fs::read(&descriptor_path).expect("descriptor bytes");
    let last = bytes.last_mut().expect("descriptor byte");
    *last ^= 1;
    std::fs::write(descriptor_path, bytes).expect("mutate descriptor");
    let error = expect_load_error(&descriptor_mutation, &[descriptor_mutation.binding()], &control).await;
    assert!(error.to_string().contains("digest"));
    assert!(document_codec(&descriptor_mutation.schema).await.expect("codec registry").is_none());

    let missing = prepared_fixture();
    let error = expect_load_error(&missing, &[], &control).await;
    assert!(error.to_string().contains(&missing.schema), "a declared codec row no provider answers for is pinned against the component and names its own schema: {error}");
    assert!(document_codec(&missing.schema).await.expect("codec registry").is_none());

    let wrong_package = prepared_fixture();
    let lossy = NativeCodecBinding::new("fixture.editor", "semio:fixture-wrong", "s.fixture.document", fixture_codec(&wrong_package.schema, [0x11; 32]));
    let error = expect_load_error(&wrong_package, &[lossy], &control).await;
    assert!(error.to_string().contains("no explicit native codec"));
    assert!(document_codec(&wrong_package.schema).await.expect("codec registry").is_none());

    let mut zero = prepared_fixture();
    zero.bundle["packages"][0]["nativeCodecs"][0]["packSchemaHash"] = "00".repeat(32).into();
    zero.persist_bundle();
    let error = expect_load_error(&zero, &[zero.binding()], &control).await;
    assert!(error.to_string().contains("zero"));
    assert!(document_codec(&zero.schema).await.expect("codec registry").is_none());

    let mut detached_open_target = prepared_fixture();
    detached_open_target.bundle["packages"][0]["openTargets"][0]["packSchemaHash"] = "12".repeat(32).into();
    detached_open_target.persist_bundle();
    let error = expect_load_error(&detached_open_target, &[detached_open_target.binding()], &control).await;
    assert!(error.to_string().contains("open target"));
    assert!(document_codec(&detached_open_target.schema).await.expect("codec registry").is_none());

    let mismatch = prepared_fixture();
    let binding = NativeCodecBinding::new("fixture.editor", "semio:fixture-editor", "s.fixture.document", fixture_codec(&mismatch.schema, [0x12; 32]));
    let error = expect_load_error(&mismatch, &[binding], &control).await;
    assert!(error.to_string().contains("mismatched"));
    assert!(document_codec(&mismatch.schema).await.expect("codec registry").is_none());

    let cancelled = prepared_fixture();
    let control = TestControl::new();
    control.cancelled.store(true, Ordering::SeqCst);
    assert_eq!(expect_load_error(&cancelled, &[cancelled.binding()], &control).await, AuthorityError::Cancelled);
    assert!(document_codec(&cancelled.schema).await.expect("codec registry").is_none());
    assert!(catalog_error("x".repeat(AUTHORITY_MAX_DIAGNOSTIC_BYTES * 2)).to_string().len() <= AUTHORITY_MAX_DIAGNOSTIC_BYTES + 40);
}

#[tokio::test]
async fn descriptor_owned_surface_is_required_before_any_catalog_or_codec_publication() {
    for (field, value) in [
        ("artifactKind", serde_json::json!("fixture.document")),
        ("surfaceId", serde_json::json!("s.fixture.document@1/*#foreign")),
        ("appId", serde_json::json!("s.fixture.document@1/*#foreign")),
        ("windowKindId", serde_json::json!("foreign.window")),
        ("role", serde_json::json!("viewer")),
        ("rendererTarget", serde_json::json!("wgpu")),
    ] {
        let mut fixture = prepared_fixture();
        fixture.bundle["packages"][0]["openTargets"][0][field] = value.clone();
        fixture.bundle["profiles"][0]["openTargets"][0]["target"][field] = value;
        fixture.persist_bundle();
        let error = expect_load_error(&fixture, &[fixture.binding()], &TestControl::new()).await;
        assert!(error.to_string().contains("document-open target"), "{field}: {error}");
        assert!(document_codec(&fixture.schema).await.expect("codec registry").is_none(), "{field} published a codec");
    }
}

#[test]
fn descriptor_open_targets_follow_the_one_pairing_rule_and_validate_as_published() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️descriptor-open-targets/🔣️.json")).unwrap();
    let schema = "fixture.document@1";
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut descriptor = decode_package_descriptor(&descriptor_bytes("fixture.editor", "semio:fixture-editor", "1.0.0", &"11".repeat(32), Some(schema), None)).expect("fixture descriptor");
        descriptor.execution = serde_json::from_value(case["execution"].clone()).expect("execution mode");
        descriptor.manifest.artifact_kinds[0].id = case["kindId"].as_str().unwrap().to_string();
        if case["declaredOn"] == "editor" {
            let kind = descriptor.manifest.artifact_kinds.remove(0);
            descriptor.manifest.apps.iter_mut().find(|app| app.role == semio_framework::AppRole::Editor).expect("editor app").artifact_kinds.push(kind);
        }
        if case["declaredOn"] == "plugin-and-editor" {
            let kind = descriptor.manifest.artifact_kinds[0].clone();
            descriptor.manifest.apps.iter_mut().find(|app| app.role == semio_framework::AppRole::Editor).expect("editor app").artifact_kinds.push(kind);
        }
        let bytes = os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("project descriptor"));
        let answer: serde_json::Value = serde_json::from_slice(&descriptor_open_targets_answer(&bytes).expect("open-target answer")).expect("answer json");
        assert_eq!(answer["schema"], fixture["answerSchema"], "{name}");
        let targets = answer["targets"].as_array().unwrap();
        let expected = case["targets"].as_array().unwrap();
        assert_eq!(targets.len(), expected.len(), "{name}: {answer}");
        for (target, expected) in targets.iter().zip(expected) {
            assert_eq!(target["role"], expected["role"], "{name}");
            assert_eq!(target["surfaceId"], expected["surfaceId"], "{name}");
            assert_eq!(target["appId"], expected["surfaceId"], "{name}");
            assert_eq!(target["windowKindId"], fixture["windowKindId"], "{name}");
            assert_eq!(target["artifactKind"], case["kindId"], "{name}");
            assert_eq!(target["artifactSchema"], schema, "{name}");
            assert_eq!(target["rendererTarget"], "wasm", "{name}");
            assert_eq!(target["grant"], serde_json::json!({ "read": true, "write": expected["write"], "observe": true }), "{name}");
            let mut published = target.as_object().unwrap().clone();
            published.insert("packSchemaHash".into(), serde_json::json!("44".repeat(32)));
            let published: TrustedBundleOpenTargetV1 = serde_json::from_value(serde_json::Value::Object(published)).expect("bundle open target");
            assert_eq!(validate_descriptor_open_target(&descriptor, &published).expect("an answered target validates").artifact_kind, "s.fixture.document", "{name}");
        }
    }
    assert!(descriptor_open_targets_answer(&[]).is_err());
}

#[test]
fn bundle_rejects_incomplete_duplicate_conflicting_and_escaping_declarations() {
    let fixture = fixture_json();
    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.packages.pop();
    assert!(validate_bundle(&bundle, "fixture").expect_err("incomplete closure").to_string().contains("incomplete"));

    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.packages[1].package_id = bundle.packages[0].package_id.clone();
    assert!(validate_bundle(&bundle, "fixture").expect_err("duplicate package identity").to_string().contains("duplicated"));

    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.profiles[0].selected_closure[0].version = "9.9.9".to_string();
    assert!(validate_bundle(&bundle, "fixture").expect_err("conflicting closure identity").to_string().contains("conflicts"));

    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.packages[0].component.path = "../escape.wasm".to_string();
    let error = TrustedCatalogRelativePathV1::parse(&bundle.packages[0].component.path).expect_err("escaping path");
    assert!(error.to_string().contains("relative path"));
}

#[test]
fn trusted_profile_generation_binds_zero_target_package_and_every_codec_row() {
    let dependency_fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
    for row in dependency_fixture["encodingCases"].as_array().unwrap() {
        let identities: Vec<TrustedBundleIdentityV1> = serde_json::from_value(row["identities"].clone()).unwrap();
        let mut encoded = Vec::new();
        append_trusted_profile_dependencies(&mut encoded, &identities).unwrap();
        assert_eq!(hex_lower(&encoded), row["hex"].as_str().unwrap(), "{}", row["name"]);
    }
    let fixture = fixture_json();
    let bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    let original = trusted_profile_generation(&bundle, &bundle.profiles[0]).expect("generation");

    let mut component: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("component bundle");
    component.packages[1].component.sha256 = "31".repeat(32);
    assert_ne!(trusted_profile_generation(&component, &component.profiles[0]).expect("component generation"), original, "zero-target component SHA-256 must rotate the profile");

    let mut descriptor: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("descriptor bundle");
    descriptor.packages[1].descriptor.sha256 = "32".repeat(32);
    assert_ne!(trusted_profile_generation(&descriptor, &descriptor.profiles[0]).expect("descriptor generation"), original, "zero-target descriptor SHA-256 must rotate the profile");

    let mut codec: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("codec bundle");
    codec.packages[0].native_codecs[0].pack_schema_hash = "33".repeat(32);
    assert_ne!(trusted_profile_generation(&codec, &codec.profiles[0]).expect("codec generation"), original, "every codec row must rotate the profile");
}

#[test]
fn local_stdio_gis_profile_is_exact_two_packages_twenty_eight_codecs_and_one_map_editor_and_viewer() {
    let bundle = local_stdio_gis_profile_bundle();
    let selected = validate_bundle(&bundle, "local-stdio-gis-open-v1").expect("closed stdio+GIS profile");
    assert_eq!(selected.package_indices.len(), 2);
    assert_eq!(selected.package_indices, vec![1, 0]);
    assert_eq!(bundle.packages.iter().map(|package| package.native_codecs.len()).sum::<usize>(), 28);
    assert_eq!(bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>(), 2);
    let mut writable_viewer = local_stdio_gis_profile_bundle();
    writable_viewer.packages[0].open_targets[1].grant.write = true;
    writable_viewer.profiles[0].open_targets[1].target.grant.write = true;
    assert!(validate_bundle(&writable_viewer, "local-stdio-gis-open-v1").is_err(), "a viewer target never carries a write grant");
    let mut missing_dependency = local_stdio_gis_profile_bundle();
    missing_dependency.packages[0].dependencies.clear();
    assert!(validate_bundle(&missing_dependency, "local-stdio-gis-open-v1").expect_err("missing compiled Stdio dependency").to_string().contains("exact closed"));
    assert_ne!(trusted_profile_generation(&missing_dependency, &missing_dependency.profiles[0]).unwrap(), bundle.profiles[0].generation_id);

    let mut missing_terrain = local_stdio_gis_profile_bundle();
    missing_terrain.packages[0].native_codecs.pop();
    assert!(validate_bundle(&missing_terrain, "local-stdio-gis-open-v1").expect_err("missing Terrain").to_string().contains("exact closed"));

    let mut terrain_target = local_stdio_gis_profile_bundle();
    let mut target = terrain_target.packages[0].open_targets[0].clone();
    target.artifact_kind = "s.gis.gisterrain".into();
    target.artifact_schema = "gis.terrain".into();
    target.pack_schema_hash = "a2".repeat(32);
    target.surface_id = "s.gis.gisterrain@1/*#editor".into();
    target.app_id = target.surface_id.clone();
    target.parent_dialect.artifact_kind = target.artifact_kind.clone();
    terrain_target.packages[0].open_targets.push(target);
    assert!(validate_bundle(&terrain_target, "local-stdio-gis-open-v1").expect_err("Terrain target").to_string().contains("exact closed"));

    let mut reordered = local_stdio_gis_profile_bundle();
    reordered.profiles[0].selected_closure.reverse();
    assert!(validate_bundle(&reordered, "local-stdio-gis-open-v1").expect_err("noncanonical closure").to_string().contains("canonical"));
}

mod long {
    use super::*;

    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn gis_map_binding_constructs_from_loaded_catalog_and_refuses_tampered_retained_bytes() {
        let _registry = crate::artifact_authority::REAL_LINKED_CODEC_REGISTRY.lock().await;
        for (viewer, foreign_service) in [(false, false), (true, false), (false, true)] {
            let fixture = prepared_gis_binding_fixture(viewer, foreign_service).await;
            let control = TestControl::new();
            let catalog = Arc::new(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "frozen-gis-test", &NativeCodecProviderSetV1::linked(), &control.context()).await.expect("catalog loaded with real GIS receipts"));
            let result = crate::inference::verified_gis_map_binding(catalog.clone());
            if foreign_service {
                assert!(matches!(result, Err(crate::inference::InferenceErrorV1::Denied)));
            } else if viewer {
                assert!(result.expect("viewer profile is admissible").is_none());
            } else {
                let binding = result.expect("verified editor binding").expect("GIS Map editor is bound");
                assert!(Arc::ptr_eq(binding.catalog(), &catalog));
                assert_eq!(binding.selection(), catalog.selected_document_open().expect("sole selection"));
                assert_eq!(binding.service().executable_identity(), semio_s_artifact_gis_gismap::gis_map_inference_service().executable_identity());
                let retained = catalog.packages().iter().find(|package| package.plugin_id() == "gis").expect("verified GIS package").component().read(&control.context()).await.expect("verified GIS component rereads").to_vec();
                let digest = binding.digest().to_owned();
                std::fs::write(fixture.component_path(0), b"tampered").expect("mutate fixture backing component");
                assert!(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "frozen-gis-test", &NativeCodecProviderSetV1::linked(), &control.context()).await.is_err());
                drop(catalog);
                let tampered = binding.catalog().packages().iter().find(|package| package.plugin_id() == "gis").expect("retained GIS package").component().read(&control.context()).await;
                assert!(tampered.is_err() && !retained.is_empty(), "a retained component whose backing file changed must be refused, never served");
                assert_eq!(binding.digest(), digest);
            }
        }
    }

    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure() {
        let _registry = crate::artifact_authority::REAL_LINKED_CODEC_REGISTRY.lock().await;
        let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
        let mut fixture = prepared_gis_binding_fixture(false, false).await;
        for receipt in semio_s_plugin_gis::native_codecs::native_codec_factory_receipts().unwrap() {
            let native = receipt.into_codec().unwrap();
            let declared = document_codec(&native.schema).await.unwrap().expect("actual assembled GIS declaration owns its codec");
            assert_eq!(native.schema, declared.schema);
            assert_eq!(native.extension, declared.extension);
            assert_eq!(native.pack_schema_hash, declared.pack_schema_hash);
            assert!(std::ptr::fn_addr_eq(native.compile_dsl, declared.compile_dsl));
            assert!(std::ptr::fn_addr_eq(native.print_mirror, declared.print_mirror));
            assert!(std::ptr::fn_addr_eq(native.edit_text_from_envelope, declared.edit_text_from_envelope));
            assert!(std::ptr::fn_addr_eq(native.apply_ops_binary, declared.apply_ops_binary));
        }
        let baseline = fixture.bundle.clone();
        let originals: Vec<_> = baseline["packages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|record| {
                let path = fixture.root.join(record["descriptor"]["path"].as_str().unwrap());
                (path.clone(), std::fs::read(path).expect("original full descriptor"))
            })
            .collect();
        let schemas: Vec<_> = baseline["packages"].as_array().unwrap().iter().flat_map(|record| record["nativeCodecs"].as_array().unwrap().iter().map(|codec| codec["artifactSchema"].as_str().unwrap().to_owned())).collect();
        assert_eq!(schemas.len(), 28);
        let mut before = Vec::with_capacity(schemas.len());
        for schema in &schemas {
            before.push(document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash)));
        }
        eprintln!("[DEBUG] linked-catalog initial-public-codecs={}", before.iter().filter(|codec| codec.is_some()).count());
        for row in corpus["atomicCases"].as_array().unwrap() {
            fixture.bundle = baseline.clone();
            for (index, (path, bytes)) in originals.iter().enumerate() {
                std::fs::write(path, bytes).expect("restore exact descriptor bytes");
                attach_fixture_plugin_module(&fixture.root, &mut fixture.bundle["packages"][index], bytes);
            }
            let change = row["change"].as_str().unwrap();
            let package = if change == "stdio-catalog" { "stdio" } else { "gis" };
            let index = fixture.bundle["packages"].as_array().unwrap().iter().position(|record| record["pluginId"] == package).unwrap();
            if !matches!(change, "exact" | "gis-provider") {
                let mut descriptor = decode_package_descriptor(&originals[index].1).expect("original authoritative descriptor");
                match change {
                    "stdio-catalog" | "gis-catalog" => descriptor.manifest.topic_contributions.retain(|entry| entry.topic != "stdio.artifact-catalog.v1"),
                    "gis-dependency" => descriptor.manifest.dependencies.clear(),
                    "gis-trailing-byte" | "gis-duplicate-field" => {}
                    _ => panic!("unknown atomic fixture change {change}"),
                }
                descriptor.hashes.descriptor_sha256.clear();
                descriptor.hashes.descriptor_sha256 = hex_lower(&Sha256::digest(&os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).unwrap())));
                let mut value = to_dsl_value(&descriptor).unwrap();
                if change == "gis-duplicate-field" {
                    let DslValue::Object(fields) = &mut value else { panic!("full descriptor object") };
                    let duplicate = fields.iter().find(|(key, _)| key == "manifest").unwrap().clone();
                    fields.push(duplicate);
                }
                let mut bytes = os_store::pack_rt::encode_wire_value(&value);
                if change == "gis-trailing-byte" {
                    bytes.push(0);
                }
                let raw_invalid = matches!(change, "gis-trailing-byte" | "gis-duplicate-field");
                assert_eq!(decode_package_descriptor(&bytes).is_err(), raw_invalid, "full descriptor raw gate: {change}");
                let record = &mut fixture.bundle["packages"][index];
                record["descriptor"]["byteLength"] = bytes.len().into();
                record["descriptor"]["sha256"] = hex_lower(&Sha256::digest(&bytes)).into();
                if record["browserActor"]["kind"] == "closed-browser-actor" {
                    record["browserActor"]["sourceDescriptorByteSha256"] = record["descriptor"]["sha256"].clone();
                }
                attach_fixture_plugin_module(&fixture.root, record, &bytes);
                std::fs::write(&originals[index].0, bytes).expect("write resealed complete candidate descriptor");
            }
            fixture.refresh_profile_generation();
            fixture.persist_bundle();
            let providers = RecordingLinkedProvider { previews: Mutex::new(Vec::new()), fail_gis: change == "gis-provider" };
            let control = TestControl::new();
            let result = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "frozen-gis-test", &providers, &control.context()).await;
            assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "loader result: {change}: {:?}", result.as_ref().err());
            let previews: Vec<String> = serde_json::from_value(row["previews"].clone()).unwrap();
            assert_eq!(*providers.previews.lock().unwrap(), previews, "successful private preview frontier: {change}");
            if let Ok(catalog) = result {
                assert_eq!(catalog.codec_count(), 28);
                assert_eq!(catalog.packages().iter().map(VerifiedTrustedPackage::plugin_id).collect::<Vec<_>>(), vec!["stdio", "gis"]);
                assert_eq!(catalog.open_target_count(), 1);
                assert_eq!(catalog.selected_document_open().unwrap().package.plugin_id, "gis");
                for schema in &schemas {
                    assert!(document_codec(schema).await.unwrap().is_some(), "complete public codec: {schema}");
                }
            } else {
                for (schema, prior) in schemas.iter().zip(&before) {
                    assert_eq!(&document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash)), prior, "partial public codec after {change}: {schema}");
                }
            }
            eprintln!("[DEBUG] linked-catalog atomic-case={change} successful-private-previews={}", previews.len());
        }
    }
}

#[tokio::test]
async fn a_loaded_catalog_indexes_and_serves_every_verified_plugin_module_file_and_refuses_a_tampered_one() {
    let fixture = prepared_fixture();
    let provider = FixtureProviderSource::new(vec![fixture.binding()]);
    let control = TestControl::new();
    let catalog = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &control.context()).await.expect("fixture catalog");
    let index = catalog.plugin_module_index();
    plugin_module::validate_plugin_module_index(&index).expect("canonical index");
    assert_eq!(index.generation_id, catalog.generation_id());
    assert_eq!(index.modules.iter().map(|entry| entry.plugin_id.as_str()).collect::<Vec<_>>(), vec!["fixture.base", "fixture.editor"]);
    assert_eq!(index.modules[1].dependencies, vec!["fixture.base".to_owned()]);
    assert_eq!(index.modules[1].dialect_artifact_kinds, vec!["s.fixture.document".to_owned()]);
    assert!(index.modules[0].dialect_artifact_kinds.is_empty(), "a package without apps opens no dialect");
    assert!(index.modules.iter().all(|entry| entry.extends_plugin_id.is_none()), "plugin packages extend nothing");
    for entry in &index.modules {
        let module = catalog.plugin_module(&entry.bundle_sha256).expect("indexed module");
        assert_eq!(hex_lower(&Sha256::digest(module.manifest_bytes())), entry.bundle_sha256);
        assert_eq!(module.manifest_bytes().len() as u64, entry.bundle_byte_length);
        assert_eq!(module.bundle().entry, entry.entry);
        for file in &module.bundle().files {
            let bytes = module.file(&file.path).expect("listed file").read(&control.context()).await.expect("verified file");
            assert_eq!(hex_lower(&Sha256::digest(&bytes)), file.sha256);
        }
    }
    assert!(catalog.plugin_module(&"00".repeat(32)).is_none());
    let editor = catalog.plugin_module(&index.modules[1].bundle_sha256).expect("editor module");
    let bridge = editor.bundle().files.iter().find(|file| file.path.ends_with("🌉️bridge.js")).expect("editor entry");
    assert!(editor.file("fixture.editor/unlisted.js").is_none());
    std::fs::write(fixture.root.join(plugin_module::plugin_module_blob_path(&bridge.sha256)), b"tampered").expect("tamper entry");
    assert!(editor.file(&bridge.path).expect("editor entry").read(&control.context()).await.is_err(), "a module file tampered after verification is never served");
    assert!(drain_stream(editor.file(&bridge.path).expect("editor entry")).await.1.is_some(), "a module file tampered after verification never completes its stream");
}

/// 🚰️ Drains one asset stream: the bytes it released and the error that ended it, if any.
async fn drain_stream(asset: &TrustedCatalogAsset) -> (Vec<u8>, Option<AuthorityError>) {
    let mut stream = match asset.stream() {
        Ok(stream) => stream,
        Err(error) => return (Vec::new(), Some(error)),
    };
    let mut released = Vec::new();
    while let Some(chunk) = stream.next_chunk().await {
        match chunk {
            Ok(bytes) => released.extend_from_slice(&bytes),
            Err(error) => return (released, Some(error)),
        }
    }
    (released, None)
}

/// ⚖️ LAW: a verified file is served as a chunked stream with NO request deadline that releases exactly the verified bytes,
/// and a file changed on disk after verification — same length, one byte flipped anywhere, or truncated — ends the stream in
/// an error before its final chunk, so a reader holding the declared length never completes it. The per-request reread
/// under the 8 s execution-target deadline refused every large core module with 500 once a burst loaded the debug hub
/// (S15, session 12: all 7 core modules ≥ 17 MB of catalog B answered 500 at 8.0–8.2 s, `wp-s15/s15-module-burst.ts`).
#[tokio::test]
async fn a_verified_file_streams_its_exact_bytes_without_a_deadline_and_withholds_its_last_chunk_once_tampered() {
    let fixture = prepared_fixture();
    let canonical_root = std::fs::canonicalize(&fixture.root).expect("canonical fixture-owned generation root");
    let root = Arc::new(TrustedCatalogGenerationRoot::open_fixture_owned(&canonical_root).expect("opened generation root"));
    let bytes: Vec<u8> = (0..(3 * TRUSTED_READ_CHUNK_BYTES + 17)).map(|index| (index % 251) as u8).collect();
    let retained = |name: &str| {
        std::fs::write(fixture.root.join(name), &bytes).expect("write multi-chunk module file");
        TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(&root), TrustedCatalogRelativePathV1::parse(name).expect("relative path"), bytes.len() as u64, Sha256::digest(&bytes), Some(*Hasher::new().update(&bytes).finalize().as_bytes())))
    };

    let intact = retained("intact.bin");
    assert_eq!(intact.stream().expect("stream").byte_length(), bytes.len() as u64);
    let (released, error) = drain_stream(&intact).await;
    assert!(error.is_none(), "an intact verified file streams to its end: {error:?}");
    assert_eq!(released, bytes, "the stream releases exactly the verified bytes");

    for (name, offset) in [("flip-first.bin", 0), ("flip-middle.bin", TRUSTED_READ_CHUNK_BYTES + 5), ("flip-last.bin", bytes.len() - 1)] {
        let asset = retained(name);
        let mut tampered = bytes.clone();
        tampered[offset] ^= 0xff;
        std::fs::write(fixture.root.join(name), &tampered).expect("tamper after verification");
        let (released, error) = drain_stream(&asset).await;
        assert!(error.is_some(), "{name}: a file changed after verification ends its stream in an error");
        assert!(released.len() < bytes.len(), "{name}: the final chunk is withheld, so the declared length is never completed ({} of {})", released.len(), bytes.len());
    }

    let truncated = retained("truncated.bin");
    std::fs::write(fixture.root.join("truncated.bin"), &bytes[..bytes.len() - 1]).expect("truncate after verification");
    assert!(drain_stream(&truncated).await.1.is_some(), "a truncated file is refused before it streams");

    let (released, error) = drain_stream(&TrustedCatalogAsset::resident(Arc::from(bytes.clone()))).await;
    assert!(error.is_none() && released == bytes, "a resident asset streams the bytes the catalog verified");
}
