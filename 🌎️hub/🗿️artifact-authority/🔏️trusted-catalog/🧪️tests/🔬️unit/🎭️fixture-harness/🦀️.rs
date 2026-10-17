/// 🎭️ Fixture harness: control and provider doubles, descriptor and bundle builders, linked-fixture guards and prepared fixtures.

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
        if package.plugin_id == "fixture-editor" {
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
        let binding=NativeCodecBinding::new("fixture-editor", "semio:fixture-editor", "s.fixture.document", None, fixture_codec(&self.schema, [0x11; 32]));assert_eq!(binding.factory_id(),None);binding
    }

    fn rewrite_descriptor(&mut self, index: usize, schema: Option<&str>, dependency: Option<(&str, &str)>) {
        let record = &self.bundle["packages"][index];
        let bytes = descriptor_bytes(
            record["pluginId"].as_str().expect("plugin"),
            record["packageId"].as_str().expect("package"),
            record["version"].as_str().expect("version"),
            schema,
            dependency,
        );
        self.replace_descriptor(index, bytes);
    }

    fn replace_descriptor(&mut self, index: usize, bytes: Vec<u8>) {
        let record = &mut self.bundle["packages"][index];
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
        vec![NativeCodecBinding::new("fixture-base", "semio:fixture-base", "s.fixture.document", None, fixture_codec(&schema, [0x11; 32])), self.binding()]
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// 🧩️ (Re)writes one fixture record's shipped receipt and plugin module for its current component and descriptor bytes.
fn attach_fixture_plugin_module(root: &Path, record: &mut serde_json::Value, descriptor: &[u8]) {
    let receipt = fixture_component_receipt(record["pluginId"].as_str().unwrap(), record["packageId"].as_str().unwrap(), record["component"]["sha256"].as_str().unwrap(), record["component"]["byteLength"].as_u64().unwrap(), descriptor);
    record["receipt"] = serde_json::to_value(&receipt).expect("fixture receipt json");
    record["pluginModule"] = write_fixture_plugin_module(root, record["pluginId"].as_str().unwrap(), record["packageId"].as_str().unwrap(), record["version"].as_str().unwrap(), &receipt, descriptor).expect("fixture plugin module");
}

/// 🧱️ The fixture component: the eight-byte WASI component preamble, the smallest bytes a shipped receipt can name.
fn fixture_component_bytes(fixture: &serde_json::Value) -> Vec<u8> {
    fixture["componentHex"].as_str().expect("component hex").as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), 16).expect("hex byte")).collect()
}

fn snapshot_json() -> serde_json::Value {
    serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/👥️two-package/🔣️.json")).expect("trusted-catalog fixture")
}

fn synthetic_browser_actor(component: &str, descriptor: &str, path: &str) -> serde_json::Value {
    let mut fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🌐️browser-actor/🔣️.json")).unwrap();
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
    let stdio_codecs = (0u8..29).map(|index| TrustedBundleCodecV1 { artifact_kind: format!("s.stdio.fixture{index:02}"), artifact_schema: format!("stdio.fixture{index:02}"), pack_schema_hash: format!("{:02x}", index + 1).repeat(32) }).collect();
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
            receipt: ComponentReceiptV1 { wasm_sha256: "11".repeat(32), semantic_descriptor_sha256: "13".repeat(32), ..fixture_component_receipt("gis", "semio:gis", &"11".repeat(32), 8, b"") },
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
            receipt: ComponentReceiptV1 { wasm_sha256: "21".repeat(32), semantic_descriptor_sha256: "23".repeat(32), ..fixture_component_receipt("stdio", "semio:stdio", &"21".repeat(32), 8, b"") },
            browser_actor: TrustedBundleBrowserActorV1::None {},
            plugin_module: TrustedBundlePluginModuleV1 { path: "packages/stdio/plugin-module.json".into(), byte_length: 1, sha256: "24".repeat(32), blake3: "25".repeat(32) },
            native_codecs: stdio_codecs,
            open_targets: vec![],
        },
    ];
    let selected_closure = vec![gis_identity.clone(), stdio_identity];
    let mut bundle = TrustedBundleV1 {
        schema_version: TRUSTED_BUNDLE_SCHEMA_VERSION,
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

fn descriptor_bytes(plugin_id: &str, package_id: &str, version: &str, schema: Option<&str>, dependency: Option<(&str, &str)>) -> Vec<u8> {
    encode_descriptor_json(descriptor_json(plugin_id, package_id, version, schema, dependency))
}

/// 🧾️ Encodes one handcrafted descriptor JSON exactly as the publisher's descriptor bytes are encoded.
fn encode_descriptor_json(json: serde_json::Value) -> Vec<u8> {
    let descriptor: PackageDescriptor = serde_json::from_value(json).expect("package descriptor");
    os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor))
}

/// 🏠️ The fixture editor as a HOST of the base package's kind: it declares no kind, hosts `s.fixture.document` from
/// `fixture-base`, and adds a `strict` subset editor beside its whole-standard `*` editor, both opening the one document schema.
fn hosting_descriptor_bytes(schema: &str) -> Vec<u8> {
    let mut json = descriptor_json("fixture-editor", "semio:fixture-editor", "1.2.3", Some(schema), Some(("fixture-base", "1.0.0")));
    let manifest = &mut json["manifest"];
    manifest["artifactKinds"] = serde_json::json!([]);
    manifest["hostedArtifactKinds"] = serde_json::json!([{ "id": "s.fixture.document", "schema": schema, "owner": "fixture-base" }]);
    let mut strict = manifest["apps"][0].clone();
    strict["id"] = "s.fixture.document@1/strict#editor".into();
    strict["controllerId"] = "fixture-editor-strict".into();
    strict["dialect"]["subset"] = "strict".into();
    manifest["apps"].as_array_mut().expect("fixture apps").push(strict);
    encode_descriptor_json(json)
}

/// 🧾️ One handcrafted fixture descriptor as JSON: a plugin declaring `s.fixture.document` with `schema` (editor and viewer of
/// `@1/*`) or nothing, depending on `dependency`.
fn descriptor_json(plugin_id: &str, package_id: &str, version: &str, schema: Option<&str>, dependency: Option<(&str, &str)>) -> serde_json::Value {
    let artifact_kinds = schema.map_or_else(Vec::new, |schema| {
        vec![serde_json::json!({
            "id": "s.fixture.document",
            "label": { "native": { "en": "Fixture Document", "de": "Fixture-Dokument" }, "reuse": { "en": "Fixture Document", "de": "Fixture-Dokument" } },
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
    serde_json::json!({
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
        "executionProtocol": { "appChannelVersion": directory::os_spr::CHANNEL_VERSION }
    })
}

fn prepared_fixture() -> FixtureDirectory {
    let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst);
    let schema = format!("fixture.document.catalog.{}.{}@1", std::process::id(), sequence);
    let root = semio_hub_testing::test_artifact_root().join(format!("semio-hub-trusted-catalog-{}-{sequence}", std::process::id()));
    std::fs::create_dir_all(root.join("components")).expect("component directory");
    std::fs::create_dir_all(root.join("descriptors")).expect("descriptor directory");
    std::fs::create_dir_all(root.join("browser")).expect("actor directory");
    std::fs::write(root.join("browser/closed-actor.mjs"), b"abc").expect("synthetic actor, never executed");
    let mut fixture = snapshot_json();
    let mut bundle = fixture["bundle"].take();
    bundle["packages"][0]["nativeCodecs"][0]["artifactSchema"] = schema.clone().into();
    for target in bundle["packages"][0]["openTargets"].as_array_mut().expect("open targets") {
        target["artifactSchema"] = schema.clone().into();
    }
    for entry in bundle["profiles"][0]["openTargets"].as_array_mut().expect("profile open targets") {
        entry["target"]["artifactSchema"] = schema.clone().into();
    }
    let component = fixture_component_bytes(&fixture);
    for index in 0..2 {
        let component_path = root.join(bundle["packages"][index]["component"]["path"].as_str().expect("component path"));
        std::fs::write(component_path, &component).expect("write component");
    }
    let root_descriptor = descriptor_bytes("fixture-editor", "semio:fixture-editor", "1.2.3", Some(&schema), Some(("fixture-base", "1.0.0")));
    let base_descriptor = descriptor_bytes("fixture-base", "semio:fixture-base", "1.0.0", None, None);
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
async fn prepared_gis_binding_fixture(viewer: bool, foreign_service: bool) -> FixtureDirectory {
    let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).expect("explicit test mounted owner policy");
    semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_hub_gis::plugin().expect("GIS assembly"));
    let emitted = semio_framework_plugin::describe::describe_plugin(&runtime).await;
    let mut descriptor = decode_package_descriptor(&emitted).expect("actual native GIS descriptor");
    semio_hub_stdio_catalog::validate_native_artifact_catalog_dependency(&descriptor.manifest.dependencies).expect("actual GIS compiled Stdio dependency");
    semio_hub_stdio_catalog::validate_native_artifact_catalog_contributions(&descriptor.manifest.topic_contributions).expect("actual GIS compiled Stdio catalog");
    let component = b"synthetic-gis-component-for-catalog-binding-test";
    let component_sha256 = hex_lower(&Sha256::digest(component));
    let mut component_blake3 = Hasher::new();
    component_blake3.update(component);
    if foreign_service {
        descriptor.contributions.inference_services.iter_mut().find(|service| service.inference_schema == "s.gis.gismap.inference").expect("actual GIS inference declaration").contributor = "foreign".into();
    }
    let bytes = os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor));
    let native_codecs: Vec<_> = semio_hub_gis::native_codecs::native_codec_factory_receipts()
        .expect("actual GIS codec receipts")
        .into_iter()
        .map(|receipt| {
            let identity = receipt.identity();
            serde_json::json!({ "artifactKind": identity.artifact_kind, "artifactSchema": identity.schema, "packSchemaHash": hex_lower(&identity.pack_schema_hash) })
        })
        .collect();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json")).expect("neutral frozen binding corpus");
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
    let root = semio_hub_testing::test_artifact_root().join(format!("gis-binding-catalog-{}-{sequence}", std::process::id()));
    std::fs::create_dir(&root).expect("exclusive GIS binding fixture directory");
    std::fs::write(root.join("component.wasm"), component).expect("write synthetic GIS component");
    std::fs::write(root.join("descriptor.semio"), &bytes).expect("write actual GIS descriptor");
    std::fs::write(root.join("closed-actor.mjs"), b"abc").expect("synthetic actor, never executed");
    let (stdio_identity, stdio_record) = headless_stdio_fixture_package(&root).expect("headless Stdio dependency package");
    let mut bundle = serde_json::json!({
        "schemaVersion": 4,
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

struct RecordingLinkedProvider {
    previews: Mutex<Vec<String>>,
    fail_gis: bool,
}

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

fn fixture_apply<'a>(pack: &'a [u8], spr: &'a [u8], _operations: &'a [u8]) -> directory::os_store::serialization::ArtifactCodecApplyFuture<'a> {
    Box::pin(async move { Ok((pack.to_vec(), spr.to_vec(), String::new())) })
}

fn fixture_codec(schema: &str, pack_schema_hash: [u8; 32]) -> ArtifactCodec {
    ArtifactCodec { schema: schema.to_string(), extension: "fixture", native_identity: directory::os_store::serialization::ArtifactNativeSnapshotIdentity::typed::<Vec<u8>>(), snapshot_sqlite: None, pack_schema_hash, compile_dsl: fixture_compile, print_mirror: fixture_print, edit_text_from_envelope: fixture_edit, apply_ops_binary: fixture_apply, replay_envelopes: fixture_apply }
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
