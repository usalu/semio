//! 🔬️ Native socket probe: probe snapshots and diffs, the native runners, the smoke runner and the icon atlas upload.
use super::*;
use semio_framework_os_store_testing_doubles::store;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned)]
pub struct NativeSocketProbeSnapshot(pub String);


#[cfg(not(target_arch = "wasm32"))]
#[path="../🪶️sqlite/🦀️.rs"]
pub mod native_socket_sqlite;

#[cfg(not(target_arch = "wasm32"))]
impl pack::snapshot::ArtifactDsl for NativeSocketProbeSnapshot {
    const EXTENSION: &'static str = "native-socket-probe";

    fn parse_dsl(text: &str) -> Result<Self, store::os_store::TextError> {
        Ok(Self(text.to_string()))
    }

    fn print_dsl(&self) -> String {
        self.0.clone()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl pack::snapshot::ArtifactPack for NativeSocketProbeSnapshot {
    /// 🪪️ Declares this probe's real production IO identity independently from editor factories.
    

    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, _options: &store::os_store::PackEncodeOptions) -> Result<Vec<u8>, store::os_store::PackError> {
        Ok(self.0.as_bytes().to_vec())
    }

    fn decode_pack_with(bytes: &[u8], _options: &store::os_store::PackDecodeOptions) -> Result<Self, store::os_store::PackError> {
        String::from_utf8(bytes.to_vec()).map(Self).map_err(|error| store::os_store::PackError::from(semio_framework_value::ValueError::from(error)))
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(semio_framework_dsl_record::RecordSpec::new(Some("native-socket-probe"), semio_framework_dsl_record::RecordLayout::Inline, vec![semio_framework_dsl_record::FieldSpec::new(0, "value", semio_framework_dsl_record::Shape::Value)]))
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for NativeSocketProbeSnapshot {
fn native_snapshot_registration()->Option<(semio_framework_artifact_reference::Dialect,store::os_store::serialization::ArtifactCodec)>{
        Some((native_socket_sqlite::DIALECT,store::os_store::artifact_codec::<Self,NativeSocketProbeMutation>(native_socket_sqlite::SCHEMA)))
    }
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

/// 🌉️ Hand-written, not derived: this newtype wraps a bare `String`, so it encodes transparently as
/// that string — exactly what `#[derive(ToValue)]` and `serde` emit for a newtype struct.
///
/// 🪲️ NEVER route this through `store::os_store::to_dsl_value`/`from_dsl_value`: since
/// `🧰️framework/🔨️modules/🌱️value/🦀️.rs:313`'s serde-elimination those are `Ok(value.to_value())` /
/// `T::from_value(value)`, so an impl that calls them is a two-frame infinite recursion that aborts
/// the process with `fatal runtime error: stack overflow` the first time a probe document is
/// committed — no stack size survives it. See `📓️fable-mcp-artifact-quick-recursion.md`.
#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::ToValue for NativeSocketProbeSnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(self.0.clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::FromValue for NativeSocketProbeSnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(text) => Ok(Self(text)),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned)]
pub struct NativeSocketProbeDiff(pub String);

#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::command::DiffAlgebra<NativeSocketProbeSnapshot> for NativeSocketProbeDiff {


    fn is_empty(&self) -> bool {
        false
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::command::ColdDiffAlgebra<NativeSocketProbeSnapshot> for NativeSocketProbeDiff {
fn inverse(&self, base: &NativeSocketProbeSnapshot) -> Self {
        NativeSocketProbeDiff(base.0.clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::command::MutationDiff<NativeSocketProbeSnapshot> for NativeSocketProbeDiff {}
#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::command::ColdMutationDiff<NativeSocketProbeSnapshot> for NativeSocketProbeDiff {
    fn apply(&self, base: &mut NativeSocketProbeSnapshot, capability: store::os_spr::command::ApplyCapability) -> store::os_spr::command::MutationApplyResult<()> {
        store::os_spr::command::PreparedField::new(NativeSocketProbeSnapshot(self.0.clone()),drop,capability).install(base);Ok(())
    }

    fn absorb(&mut self, other: Self) {
        self.0 = other.0;
    }
}

/// 🌉️ Hand-written — same transparent-newtype shape, and the same no-`to_dsl_value` recursion rule,
/// as [`NativeSocketProbeSnapshot`]'s impl above.
#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::ToValue for NativeSocketProbeDiff {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(self.0.clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::FromValue for NativeSocketProbeDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(text) => Ok(Self(text)),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned)]
pub enum NativeSocketProbeMutation {
    Set(String),
}

#[cfg(not(target_arch = "wasm32"))]
const NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR: store::os_spr::command::MutationLeafDescriptor = store::os_spr::command::MutationLeafDescriptor {
    schema_version: 1,
    owner: "framework/os/renderer/wgpu/native-socket-probe",
    semantic_kind: "set",
    display_name: "Set",
    emoji: "🧪",
    aggregate_variant: "Set",
    payload_schema: "native.socket-grant.probe/v1",
    text_opcode: None,
    binary_tag: None,
    invertibility: store::os_spr::command::MutationInvertibility::ExplicitMutation,
    diff_participation: store::os_spr::command::MutationDiffParticipation::Detect,
    outcome_classes: &[store::os_spr::command::MutationOutcomeClass::Applied],
    composition: store::os_spr::command::MutationComposition::Atomic,
    required_language_surfaces: &[store::os_spr::command::MutationLanguageSurface::Rust],
};

#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::command::Mutation<NativeSocketProbeSnapshot> for NativeSocketProbeMutation {
    type Diff = NativeSocketProbeDiff;
    const DESCRIPTORS: &'static [store::os_spr::command::MutationLeafDescriptor] = &[NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR];

    fn descriptor(&self) -> &'static store::os_spr::command::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }




}
#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::command::ColdMutation<NativeSocketProbeSnapshot> for NativeSocketProbeMutation {
type ReferenceDiff= NativeSocketProbeDiff;
fn diff(&self, _base: &NativeSocketProbeSnapshot) -> store::os_spr::command::MutationOutcome<NativeSocketProbeDiff> {
        let Self::Set(value) = self;
        store::os_spr::command::MutationOutcome::new(NativeSocketProbeDiff(value.clone()))
    }
fn inverse(&self, base: &NativeSocketProbeSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::Set(base.0.clone())]
    
    })())
}
}

/// 🌉️ Hand-written — externally tagged as `{"Set": "<payload>"}`, byte-identical to what
/// `#[derive(ToValue)]` and `serde` emit for a tagless single-unnamed-field variant, and bound to
/// the one descriptor's own `aggregate_variant` rather than a second copy of the string. Same
/// no-`to_dsl_value` recursion rule as [`NativeSocketProbeSnapshot`]'s impl above.
#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::ToValue for NativeSocketProbeMutation {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let Self::Set(value) = self;
        semio_framework_value::DslValue::object([(NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR.aggregate_variant.to_string(), semio_framework_value::DslValue::String(value.clone()))])
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::FromValue for NativeSocketProbeMutation {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a one-key `{}` object, found {value:?}", NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR.aggregate_variant)));
        };
        match <[(String, semio_framework_value::DslValue); 1]>::try_from(entries) {
            Ok([(key, semio_framework_value::DslValue::String(text))]) if key == NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR.aggregate_variant => Ok(Self::Set(text)),
            Ok([(key, payload)]) => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected variant `{}` with a string payload, found `{key}` with {payload:?}", NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR.aggregate_variant))),
            Err(entries) => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected exactly one variant key, found {}", entries.len()))),
        }
    }
}

/// 🪲️ Regression law for the two-frame `to_value` → `to_dsl_value` → `to_value` recursion that made
/// every `NativeSocketProbe*` encode abort the process with `fatal runtime error: stack overflow`:
/// reaching an assertion at all proves the cycle is gone, and the fixture pins the exact wire shape
/// so the cure cannot silently change the encoding. `serde_json` is the independent third-party
/// oracle — the first-party `ToValue` tree must equal what it serializes for the same values.

#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::OpText for NativeSocketProbeMutation {
    fn print_op(&self) -> String {
        let Self::Set(value) = self;
        value.clone()
    }

    fn parse_op(line: &str) -> Result<Self, store::os_store::TextError> {
        Ok(Self::Set(line.to_string()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_spr::OpBinary for NativeSocketProbeMutation {
    fn encode_op(&self) -> Result<Vec<u8>, store::os_spr::ProtocolError> {
        let Self::Set(value) = self;
        Ok(value.as_bytes().to_vec())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, store::os_spr::ProtocolError> {
        String::from_utf8(bytes.to_vec()).map(Self::Set).map_err(|error| store::os_spr::ProtocolError::Malformed { what: "native-socket-probe-op", offset: 0, detail: error.to_string() })
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn run_socket_grant_probe() -> i32 {
    use dsl::os_directory::client::{native::NativeDirectoryTransport, DirectoryClient};
    use dsl::os_directory::identity::claimed_local_hub_credential;
    use dsl::os_spr::{ActorId, ArtifactDiff, ArtifactId, HybridLogicalTimestamp, InverseMutation, MutationEnvelope, MutationId, SchemaId};
    use dsl::os_store::sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactDocumentKey, ArtifactEvent, ArtifactHost, CommandAckOutcome, PersistenceBinding};
    use semio_framework_actor::{ActorId as DirectoryActorId, PackageId as DirectoryPackageId};
    use semio_framework_async::ScopeOwner;
    use semio_framework_os_services::{ComputePool, TokioHostRuntime};

    const PROBE_SCHEMA: &str = native_socket_sqlite::SCHEMA;
    let Some(credential) = claimed_local_hub_credential("native") else { return 1 };
    if store::io::publish_native_snapshot::<NativeSocketProbeSnapshot>().is_err() { return 1; }
    let pool = renderer_worker_pool();
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("native_socket_grant_probe"), None);
    let compute = Arc::new(ComputePool::with_pool(2, pool.clone()));
    let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime, scope, compute, 1_000_000, 2, DirectoryPackageId("native.socket-grant-probe".into()), DirectoryActorId(0));
    let client = Arc::new(DirectoryClient::authenticated(transport, credential.clone()));
    let host = ArtifactHost::new(Arc::new(pool));
    if host.local_hub_ready() {
        return 1;
    }
    host.set_local_hub_credential(credential.clone());
    if host.local_hub_ready() {
        return 1;
    }
    host.set_hub_socket_grant_source(client);
    if !host.local_hub_ready() {
        return 1;
    }
    let document_id = "probe-document";
    let document_key = ArtifactDocumentKey::hub("probe-space", document_id);
    let _ = &document_key;
    let channels = host
        .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
            document_id: document_id.into(),
            schema: PROBE_SCHEMA.into(),
            bindings: vec![PersistenceBinding::Hub { base_url: credential.hub_origin().into(), space_id: "probe-space".into(), surface: Some("native.socket-grant.probe.editor".into()) }],
            watch_external: false,
            actor: "untrusted-local-probe".into(),
        })
        .await;
    let mut events = host.subscribe_key(&channels.document_key).await;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(12);
    let mut sessions = 0u8;
    let mut acknowledgements = 0u8;
    let envelope = |sequence: u8| MutationEnvelope {
        mutation_id: MutationId(format!("probe-{sequence}")),
        document_id: ArtifactId(document_id.into()),
        actor: ActorId("untrusted-local-probe".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: ArtifactDiff { schema: SchemaId(PROBE_SCHEMA.into()), payload: vec![sequence] },
        inverse: InverseMutation { schema: SchemaId(PROBE_SCHEMA.into()), payload: Vec::new() },
        timestamp: HybridLogicalTimestamp::new(0, 0),
        transaction: None,
        verb: None,
        line: None,
    };
    if channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: vec![envelope(1)] }).is_err() {
        host.close_key(&channels.document_key);
        return 1;
    }
    while std::time::Instant::now() < deadline {
        match events.try_recv() {
            Ok(ArtifactEvent::Session { .. }) => {
                sessions += 1;
                if sessions == 2 && channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: vec![envelope(2)] }).is_err() {
                    break;
                }
            }
            Ok(ArtifactEvent::CommandOutcome { outcome: CommandAckOutcome::Accepted, .. }) => {
                acknowledgements += 1;
                if acknowledgements == 2 {
                    host.close_key(&channels.document_key);
                    return 0;
                }
            }
            Ok(_) | Err(tokio::sync::broadcast::error::TryRecvError::Empty) | Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => {}
            Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    host.close_key(&channels.document_key);
    1
}

/// 🧪️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END — headless smoke mode: boots
/// `ShellState` against a live hub (`S_HUB_URL`/`S_USER`/`S_DATA_DIR`) with NO GPU/window at all —
/// `GpuContext`/`winit`/`AppRuntime` are the only GPU-coupled pieces in this crate and this mode never
/// touches any of them, since `ShellState` itself is renderer-agnostic (chrome painting is a separate
/// concern layered on top by `AppRuntime::frame`). Boots, waits (bounded) for identity to mint/restore
/// and the initial directory fold to land, then dumps the Home window's widget tree + a small identity/
/// session summary as JSON to stdout and returns an exit code. An honest, explicit substitute for
/// driving a real window when this environment cannot open one (lane 3-D's brief proposed exactly this
/// shape). Returns `0` on a clean boot+dump, `1` on any hard failure along the way.
///
/// 🪪️ Identity mint/restore runs on a background OS thread (contract §C3: never blocks
/// `boot()` itself) — poll the same every-frame pump the real render loop uses (drains the
/// identity bootstrap channel + the directory stream + folds any pending events) for up to 5s
/// so a real hub round trip has time to land before the dump.
#[cfg(not(target_arch = "wasm32"))]
pub async fn run_smoke(plugin_filter: &str, plugin_modules_root: std::path::PathBuf,services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>) -> i32 {
    let axes = match semio_framework_os_renderer_wgpu_shell_prefs::shell_language_axes() {
        Ok(axes) => axes,
        Err(error) => { eprintln!("smoke: {error}"); return 1; }
    };
    run_smoke_with_axes(plugin_filter,plugin_modules_root,services,axes).await
}

/// 🧭️ Boots the exact selected native plugin under caller-declared presentation axes.
#[cfg(not(target_arch = "wasm32"))]
pub async fn run_smoke_with_axes(plugin_filter: &str, plugin_modules_root: std::path::PathBuf,services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>,axes:(semio_framework_ui_locale::Locale,semio_framework_ui_locale::Terminology)) -> i32 {
    runtime_assets::install_native_runtime_assets();
    let loaded = match load_wasm_plugins(plugin_filter, &plugin_modules_root).await {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("smoke: load_wasm_plugins failed: {error}");
            return 1;
        }
    };
    let entries = filter_plugins(loaded, plugin_filter);
    let (locale, terminology) = axes;
    let mut shell = ShellState::new(entries, plugin_filter.to_string(), locale, terminology);
    shell.opening.install_document_services(services, &mut shell.directory);
    if let Err(error) = semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx::of(&mut shell).boot().await {
        eprintln!("smoke: shell.boot() failed: {error}");
        return 1;
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while std::time::Instant::now() < deadline {
        semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx::of(&mut shell).pump_sync_events().await;
        if shell.directory.identity.is_some() || shell.directory.identity_env.is_none() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let _ = semio_framework_os_renderer_wgpu_shell_window_actions::WindowActionsCtx::of(&mut shell).refresh_ui(semio_framework::kernel::UiDirtyScope::Full).await;
    let identity_summary = shell.directory.identity.as_ref().map(|identity| serde_json::json!({ "userId": identity.user_id, "email": identity.email, "hubBaseUrl": identity.hub_base_url }));
    let window_documents: Vec<_> = shell.docs.window_ui.iter().map(|(window, document)| serde_json::json!({ "window": window, "generation": document.generation() })).collect();
    let booted = plugin_filter.is_empty() || shell.docs.session.is_some();
    let report = serde_json::json!({
        "booted": booted,
        "identity": identity_summary,
        "identityOffline": shell.directory.identity_offline,
        "openSpaceId": shell.opening.open_space_id,
        "session": shell.docs.session.as_ref().map(|session| serde_json::json!({ "pluginId": session.plugin_id, "appId": session.app.id, "role": format!("{:?}", session.app.role) })),
        "windowDocuments": window_documents,
    });
    match serde_json::to_string_pretty(&report) {
        Ok(json) => {
            println!("{json}");
            i32::from(!booted)
        }
        Err(error) => {
            eprintln!("smoke: report encode failed: {error}");
            1
        }
    }
}

/// 🐚️ Multi-mount: takes an already-created, already-placed canvas from the caller instead of looking
/// up a hardcoded `#root`/`#semio-wgpu-canvas` and taking it over via `set_inner_html("")` — that
/// single-mount assumption meant a second boot call would wipe the first mount's canvas and collide on
/// the same DOM id. The caller (`bootFrameworkOsWgpu` in `🟦️.ts`) now owns creating and placing
/// the canvas, so N independent mounts can coexist on one page.
///
/// Known gap (not yet done — see the plan's Wave 6 D11 notes), **narrowed but not closed** by ticket
/// 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the independent, uncancellable
/// `start_frame_loop`/`schedule_frame` recursive `requestAnimationFrame` chain this comment used to
/// describe is DELETED — every redraw now goes through `winit`'s own event loop
/// (`event_loop.spawn_app(app)` below → `winit_app::WinitApp`), and `WindowEvent::CloseRequested`
/// already calls `ActiveEventLoop::exit()`. Whether `exit()` alone now fully tears down a wasm mount
/// (winit's own wasm backend's post-`exit()` behaviour) is UNVERIFIED — this crate still does not
/// build clean (U4, `📓️terra-os-host-report.md`), so a real `semioWgpuUnmount` handle remains
/// deferred, but the mechanism it would need to cancel no longer exists in its old shape.
/// The dozen-plus `thread_local!` globals further up this file (`UI_ENGINE`, `ENGINE_SURFACES`,
/// `SCENE_STATE`, tooltip/dialog/tour chrome state, clipboard mocks, prefs, image-fetch caches, …) are
/// also still page-global, not per-mount — two simultaneous wgpu mounts each render on their own
/// independent GPU device/queue/surface (real, working isolation), but would still cross-talk on shared
/// UI chrome auxiliary state (a tooltip or dialog opened in one mount could show in the other).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = uploadIconAtlas)]
pub fn upload_icon_atlas(width: u32, height: u32, pixels: &[u8], entries_json: &str) -> Result<(), JsValue> {
    let entries_map: std::collections::HashMap<String, [f32; 4]> = serde_json::from_str(entries_json).map_err(|err| JsValue::from_str(&format!("icon entries parse: {err}")))?;
    let entries: Vec<(String, [f32; 4])> = entries_map.into_iter().collect();
    ICON_ATLAS_RUNTIME.with(|cell| {
        cell.borrow_mut().replace(IconAtlas::from_packed(width, height, pixels.to_vec(), entries));
    });
    Ok(())
}
