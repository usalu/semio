//! 🗂️ Plugin manifest registry and OS plugin/artifact catalog.
// #region registry

use semio_framework_artifact_space_collection::S_COLLECTION_SCHEMA;
use semio_framework_artifact_space_space::S_SPACE_SCHEMA;
use semio_framework_artifact_workflow_workflow::S_WORKFLOW_SCHEMA;
use semio_framework_os_host_instance::OsParameterFieldSpec;
use {semio_framework::AppDefinition,semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect,semio_framework::ArtifactKindSpec,semio_framework::ConfigSpec,semio_framework::MediaClass,semio_framework::MediaForm,semio_framework::MediaType,semio_framework::ModeDefinition,semio_framework::OsMediaCapability,semio_framework::PluginManifest,semio_framework::WindowKindDefinition};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use semio_framework_ui_locale::LocalizedLabel;
use ui_wgpu::wgpu::SurfaceKind;

/// 🌉️ Synchronous bridge for the kernel/`semio_framework` calls this crate's already-published
/// sync API (`BackboneDocument` codecs, `OsBackbonePort`'s in-memory/localStorage bridge, media
/// negotiation, the `io_dialects_for`/`io_route`/`io_run`/`io_dispatch` mechanism, workflow node/
/// parameter/validation helpers, `AppIo`/`CommandGrammar` reconstruction) still depend on, now
/// that those crates made them `async fn` for their own pooled-actor substrate. Every caller of
/// this helper is over a documented-immediate operation — pure data transforms and registry
/// lookups, never the file/folder host ports (which stay behind their own genuinely-async
/// host `SpaceBackbonePort` path) — so a single poll always resolves `Ready`. Not a
/// second executor entry point: it holds no runtime, spawns nothing, and never parks — it is one
/// bare `Future::poll`, same shape and same justification as this crate's existing test-only
/// bridge just below (`ExtensionInstall`'s `block_on`), lifted out of `#[cfg(test)]` because this
/// crate's sync surface needs it in production too.
pub fn resolve_kernel_future<F: std::future::Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("resolve_kernel_future: this call site is documented immediate"),
    }
}

pub type OsArtifactKindId = String;

//#region 🔖️ResourceDescriptors
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsArtifactDescriptor {
    pub kind: OsArtifactKindId,
    /// 🗣️ The kind's own localized name — its `ArtifactKindSpec.label`, what a picker shows for it.
    pub label: LocalizedLabel,
    pub source_format: String,
    pub component_kind: String,
    pub dimension: String,
    /// 🧬️ The `MediaType` this resource kind negotiates on the workflow — see
    /// `semio_framework::media_types_compatible`.
    pub media_type: MediaType,
    /// 🔌️ Structured-payload schema id, mirrored from `ArtifactKindSpec::schema` — see
    /// `crate::workflow::negotiate_media_contract`, which prefers a matching schema over a shared
    /// binary format kind id.
    pub schema: String,
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    /// 🗄️ Stdio export target kind ids (`stdio.json` / short `json`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub export_stdio_kinds: Vec<String>,
    /// 🗄️ Stdio import source kind ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import_stdio_kinds: Vec<String>,
    /// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: the real
    /// `ArtifactDialect` this resource kind's `component_kind` was derived into, computed ONCE at
    /// `artifact_kind_entry_from_spec` (never re-formatted ad hoc per call site the way
    /// `native_dialect_kind` used to). Shells (task 3) read this to call the new `io-routes`/
    /// `io-entries` host imports and populate a real "Export as…" list instead of a hard-coded
    /// format table. `standard`/`subset` are placeholder `"1"`/`"*"` until the owning plugin
    /// migrates onto `declare_artifact` and supplies a real dialect (design.md §2) — see
    /// `artifact_kind_entry_from_spec`'s doc comment.
    pub dialect: ArtifactDialect,
}

/// 🗂️ One registered resource kind's full catalog entry — the descriptor plus the media capability
/// its exporters/importers target (kept alongside rather than in `OsArtifactDescriptor` itself since
/// the descriptor is also the wire-facing presentation shape).
struct ArtifactKindEntry {
    descriptor: OsArtifactDescriptor,
    media_capability: OsMediaCapability,
}

/// 🎯️ Builds the one real `ArtifactDialect` a legacy `component_kind` slug derives into —
/// `"s." + component_kind` at standard `"1"`, subset `"*"` (the same coordinate
/// `native_dialect_kind` used to reformat from scratch on every call). Ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: this is now the ONE place that
/// derivation happens — `artifact_kind_entry_from_spec`/`seed_builtin_artifact_kinds` call it once
/// at registration time and store the result, so `native_dialect_kind`, `os_artifact_dialect`, and
/// the io-mechanism export/import path (`registry_export_media`/`registry_import_media`) all read
/// the SAME stored value instead of three independent `format!("s.{...}")` call sites. A plugin
/// that has migrated onto `declare_artifact` (design.md §2) would supply its subsets' own real
/// `ArtifactDialect`s directly instead of this placeholder-derivation path — no such plugin exists
/// yet (W1-D openQuestion #4), so every dialect this catalog holds today is still derived, not
/// declared.
fn dialect_from_component_kind(component_kind: &str) -> ArtifactDialect {
    ArtifactDialect { artifact_kind: format!("s.{component_kind}"), standard: "1".to_string(), subset: "*".to_string() }
}

fn artifact_kind_entry_from_spec(spec: &ArtifactKindSpec) -> ArtifactKindEntry {
    let dialect = dialect_from_component_kind(&spec.component_kind);
    ArtifactKindEntry {
        descriptor: OsArtifactDescriptor {
            kind: spec.id.clone(),
            label: spec.label.clone(),
            source_format: spec.source_format.clone(),
            component_kind: spec.component_kind.clone(),
            dimension: spec.dimension.clone(),
            media_type: spec.media_type,
            schema: spec.schema.clone(),
            export_formats: spec.export_formats.clone(),
            import_formats: spec.import_formats.clone(),
            export_stdio_kinds: spec.export_stdio_kinds.clone(),
            import_stdio_kinds: spec.import_stdio_kinds.clone(),
            dialect,
        },
        media_capability: spec.media_capability,
    }
}

/// 🌱️ `parameter.value` is not one app's document format — every app's parameter fields share it as
/// their port resource kind (see `crate::instance::OsParameterFieldSpec`) — so it is seeded as a
/// framework-level builtin instead of declared via any single app's `AppBuilder::artifact_kind(...)`.
fn seed_builtin_artifact_kinds() -> HashMap<OsArtifactKindId, ArtifactKindEntry> {
    let mut registry = HashMap::new();
    registry.insert(
        "parameter.value".to_string(),
        ArtifactKindEntry {
            descriptor: OsArtifactDescriptor {
                kind: "parameter.value".into(),
                label: LocalizedLabel::native("Parameter", "Parameter"),
                source_format: "parameter.value".into(),
                component_kind: "parameter".into(),
                dimension: "data".into(),
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                schema: "parameter.value".into(),
                export_formats: Vec::new(),
                import_formats: Vec::new(),
                export_stdio_kinds: Vec::new(),
                import_stdio_kinds: Vec::new(),
                dialect: dialect_from_component_kind("parameter"),
            },
            media_capability: OsMediaCapability::MeshOnly,
        },
    );
    // 🌱️ `s.workflow`/`s.space`/`s.collection` are the three backbone-document kinds the os-core
    // dissolve (`## The inversion`) now persists directly — seeded as builtins for the same reason
    // `parameter.value` is: they are not any single app's document format.
    registry.insert(
        S_WORKFLOW_SCHEMA.to_string(),
        ArtifactKindEntry {
            descriptor: OsArtifactDescriptor {
                kind: S_WORKFLOW_SCHEMA.into(),
                label: LocalizedLabel::native("Workflow", "Workflow"),
                source_format: S_WORKFLOW_SCHEMA.into(),
                component_kind: "workflow".into(),
                dimension: "data".into(),
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Document },
                schema: S_WORKFLOW_SCHEMA.into(),
                export_formats: Vec::new(),
                import_formats: Vec::new(),
                export_stdio_kinds: Vec::new(),
                import_stdio_kinds: Vec::new(),
                dialect: dialect_from_component_kind("workflow"),
            },
            media_capability: OsMediaCapability::MeshOnly,
        },
    );
    registry.insert(
        S_SPACE_SCHEMA.to_string(),
        ArtifactKindEntry {
            descriptor: OsArtifactDescriptor {
                kind: S_SPACE_SCHEMA.into(),
                label: LocalizedLabel::native("Space", "Space"),
                source_format: S_SPACE_SCHEMA.into(),
                component_kind: "space".into(),
                dimension: "data".into(),
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Document },
                schema: S_SPACE_SCHEMA.into(),
                export_formats: Vec::new(),
                import_formats: Vec::new(),
                export_stdio_kinds: Vec::new(),
                import_stdio_kinds: Vec::new(),
                dialect: dialect_from_component_kind("space"),
            },
            media_capability: OsMediaCapability::MeshOnly,
        },
    );
    registry.insert(
        S_COLLECTION_SCHEMA.to_string(),
        ArtifactKindEntry {
            descriptor: OsArtifactDescriptor {
                kind: S_COLLECTION_SCHEMA.into(),
                label: LocalizedLabel::native("Collection", "Sammlung"),
                source_format: S_COLLECTION_SCHEMA.into(),
                component_kind: "collection".into(),
                dimension: "data".into(),
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Document },
                schema: S_COLLECTION_SCHEMA.into(),
                export_formats: Vec::new(),
                import_formats: Vec::new(),
                export_stdio_kinds: Vec::new(),
                import_stdio_kinds: Vec::new(),
                dialect: dialect_from_component_kind("collection"),
            },
            media_capability: OsMediaCapability::MeshOnly,
        },
    );
    registry
}

/// 🗂️ Manifest-driven OS artifact catalog, populated at plugin registration time instead of hardcoding
/// the app roster — mirrors the `crate::workflow::export_handlers()` runtime-registry pattern.
static RESOURCE_KIND_REGISTRY: LazyLock<Mutex<HashMap<OsArtifactKindId, ArtifactKindEntry>>> = LazyLock::new(|| Mutex::new(seed_builtin_artifact_kinds()));

/// 📚️ Registers every `ArtifactKindSpec` declared by `manifest`'s apps into the OS resource
/// catalog — call at plugin registration time (`PluginHost::load_plugin`/`hot_swap_plugin`).
pub fn register_artifact_descriptors(manifest: &PluginManifest) {
    let mut registry = RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    for spec in &manifest.artifact_kinds {
        registry.insert(spec.id.clone(), artifact_kind_entry_from_spec(spec));
    }
    for app in &manifest.apps {
        for spec in &app.artifact_kinds {
            registry.insert(spec.id.clone(), artifact_kind_entry_from_spec(spec));
        }
    }
}

/// 🧪️ Registers one resource kind directly, for tests/fixtures that don't build a full
/// `PluginManifest`.
pub fn register_artifact_descriptor(spec: &ArtifactKindSpec) {
    RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(spec.id.clone(), artifact_kind_entry_from_spec(spec));
}

/// 📚️ Lists all registered OS resource descriptors, sorted by kind id for a stable snapshot.
pub fn list_os_artifact_descriptors() -> Vec<OsArtifactDescriptor> {
    let registry = RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut descriptors: Vec<OsArtifactDescriptor> = registry.values().map(|entry| entry.descriptor.clone()).collect();
    descriptors.sort_by(|left, right| left.kind.cmp(&right.kind));
    descriptors
}

/// 📚️ Resolves presentation metadata for one resource kind. An unregistered kind falls back to a
/// bare placeholder built from the kind id itself — dimension is declared by the app, never inferred
/// from an id-prefix convention.
pub fn os_artifact_descriptor(kind: &str) -> OsArtifactDescriptor {
    RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(kind).map_or_else(|| OsArtifactDescriptor {
        kind: kind.into(),
        label: LocalizedLabel::data(kind),
        source_format: kind.into(),
        component_kind: "panel".into(),
        dimension: "unknown".into(),
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: kind.into(),
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        export_stdio_kinds: Vec::new(),
        import_stdio_kinds: Vec::new(),
        dialect: dialect_from_component_kind("panel"),
    }, |entry| entry.descriptor.clone())
}

/// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: the single
/// accessor for a resource kind's real `ArtifactDialect` — reads the SAME stored value
/// `os_artifact_descriptor` returns embedded (`.dialect`), so `native_dialect_kind`, the
/// io-mechanism export/import path, and any shell calling `io-routes`/`io-entries` all resolve
/// one workflow kind id to one identical dialect. Same placeholder-fallback shape as
/// `os_artifact_descriptor` (`"s.panel"`) for an unregistered/synthetic kind.
pub fn os_artifact_dialect(kind: &str) -> ArtifactDialect {
    os_artifact_descriptor(kind).dialect
}

/// 🧬️ Registry lookup for a resource kind's media capability; unregistered kinds default to
/// `MeshOnly` (the lighter, dependency-free representation).
pub fn os_resource_media_capability(kind: &str) -> OsMediaCapability {
    RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(kind).map_or(OsMediaCapability::MeshOnly, |entry| entry.media_capability)
}

/// 🚫️ Fail-closed sibling of `os_artifact_descriptor` for workflow connect-time validation —
/// `None` for an unknown kind instead of a fabricated placeholder (that fallback is still right for
/// best-effort callers like rendering, which keep using `os_artifact_descriptor`).
pub fn try_os_artifact_descriptor(kind: &str) -> Option<OsArtifactDescriptor> {
    RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(kind).map(|entry| entry.descriptor.clone())
}

// 🧷️ `resources_compatible` (`left == right` string equality) deleted — callers now compare real
// `MediaType`s via `semio_framework::media_types_compatible`, or go through
// `crate::workflow::negotiate_media_contract` for a full connect-time decision.
//#endregion 🔖️ResourceDescriptors

//#region 🔖️PluginRegistry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsAppRegistration {
    pub id: String,
    pub role: AppRole,
    pub dialect: ArtifactDialect,
    pub label: LocalizedLabel,
    pub breadcrumb: Vec<String>,
    pub controller_id: String,
    pub inputs: Vec<semio_framework::MediaPortSpec>,
    pub outputs: Vec<semio_framework::MediaPortSpec>,
    pub source_format: String,
    pub component_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_fields: Vec<OsParameterFieldSpec>,
    pub modes: Vec<ModeDefinition>,
    /// 🧮️ The app's declared `AppDefinition.config` — how `host::reconcile_os_workflow` resolves
    /// a `plugin_id`/`app_id` app instance's `ConfigSpec` to type-check/materialize its parameter
    /// bindings (`workflow::validate_workflow_parameter_config_binding`/`instance::build_configure_config`).
    #[serde(default)]
    pub config: ConfigSpec,
}

pub fn os_app_primary_output_kind(registration: &OsAppRegistration) -> OsArtifactKindId {
    registration.outputs.first().and_then(|port| port.kind_id.clone()).unwrap_or_else(|| "graph.dag".into())
}

/// 🗂️ Production app registration store, keyed by `(plugin_id, app_id)` — populated solely by
/// `register_app_io`, called from `PluginHost::load_plugin`/`hot_swap_plugin` beside
/// `register_artifact_descriptors`. Replaces the deleted `BUILTIN_WORKFLOWS`/`EXTENSION_WORKFLOWS`
/// (`OsWorkflowDefinition`-grouped) registry, which only ever existed to route around apps not
/// declaring real `AppIo` ports in production.
static APP_REGISTRATIONS: LazyLock<Mutex<HashMap<(String, String), OsAppRegistration>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// 📚️ Derives an `OsAppRegistration` from `app.io.all_ports()` (the implicit `document:in`/
/// `document:out` ports plus every declared `AppIo.ports` entry) and registers it — call at plugin
/// registration time (`PluginHost::load_plugin`/`hot_swap_plugin`), beside `register_artifact_descriptors`.
pub fn register_app_io(plugin_id: &str, app: &AppDefinition) {
    let ports = resolve_kernel_future(app.io.all_ports());
    let (inputs, outputs): (Vec<_>, Vec<_>) = ports.into_iter().partition(|port| port.direction == semio_framework::MediaPortDirection::In);
    let registration = OsAppRegistration {
        id: app.id.clone(),
        role: app.role,
        dialect: app.dialect.clone(),
        label: app.label.clone(),
        breadcrumb: app.breadcrumb.clone(),
        controller_id: app.controller_id.clone(),
        inputs,
        outputs,
        source_format: app.io.artifact_schema.clone(),
        component_kind: app.io.artifact.component_kind.clone(),
        default_mode_id: Some(app.default_mode_id.clone()),
        parameter_fields: Vec::new(),
        modes: app.modes.iter().cloned().collect(),
        config: app.config.clone(),
    };
    APP_REGISTRATIONS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert((plugin_id.to_string(), app.id.clone()), registration);
}

pub fn os_app_registration(plugin_id: &str, app_id: &str) -> Option<OsAppRegistration> {
    APP_REGISTRATIONS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&(plugin_id.to_string(), app_id.to_string())).cloned()
}

/// 🧩️ Reconstructs an `AppIo` from an `OsAppRegistration`'s already-resolved `inputs`/`outputs` —
/// the document ports' `media_type` is read back off whichever of them is present (so
/// `AppIo::all_ports()` re-derives byte-for-byte the same document ports `register_app_io` stored),
/// every other port becomes a declared `AppIo.ports` entry.
fn app_io_for_registration(registration: &OsAppRegistration) -> semio_framework::AppIo {
    let artifact_media_type = registration
        .inputs
        .iter()
        .find(|port| port.id == "artifact:in")
        .or_else(|| registration.outputs.iter().find(|port| port.id == "artifact:out"))
        .map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |port| port.media_type);
    let declared_ports: Vec<_> = registration.inputs.iter().chain(registration.outputs.iter()).filter(|port| port.id != "artifact:in" && port.id != "artifact:out").cloned().collect();
    let io = resolve_kernel_future(semio_framework::AppIo::from_artifact(
        registration.source_format.clone(),
        artifact_media_type,
        semio_framework::ArtifactPresentation {
            id: registration.id.clone(),
            // 🚧️ No locale context reaches this reconstruction path — resolves native/English
            // pending a documented follow-up (same gap as Menu::action_with_args in the plugin SDK).
            name: registration.label.resolve(Terminology::Native, Locale::En).to_string(),
            dimension: String::new(),
            component_kind: registration.component_kind.clone(),
        },
    ));
    resolve_kernel_future(io.with_ports(declared_ports))
}

/// 🧩️ Resolves the AppDefinition backing an embedded os app instance. Returns `None` if the
/// registration declares zero modes — every app must declare at least one, so an ad hoc "inject a
/// fake edit mode" fallback would just hide a mis-registered app instead of surfacing it. An embedded
/// os app instance renders through exactly one component surface, so this synthesizes the single
/// window kind that represents it rather than leaving `window_kinds` empty (now impossible). `io` is
/// reconstructed via `app_io_for_registration` so `workflow::workflow_node_for_app` derives the same
/// ports `register_app_io` stored — the mechanism `OsStore::add_workflow_node` mints nodes through.
pub fn resolve_os_app_definition(plugin_id: &str, app_id: &str) -> Option<AppDefinition> {
    let registration = os_app_registration(plugin_id, app_id)?;
    let modes = semio_framework::Modes::try_from(registration.modes.clone()).ok()?;
    let default_mode_id = registration.default_mode_id.clone().unwrap_or_else(|| modes.first().id.clone());
    let window_kinds = semio_framework::WindowKinds::one(WindowKindDefinition {
        initial_utility_id: None,
        id: registration.component_kind.clone(),
        label: registration.label.clone(),
        body_key: registration.component_kind.clone(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "app-window".into(),
        options: ui_wgpu::wgpu::WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    });
    let io = app_io_for_registration(&registration);
    Some(AppDefinition {
        id: registration.id,
        role: registration.role,
        dialect: registration.dialect,
        label: registration.label,
        breadcrumb: registration.breadcrumb,
        icon_id: None,
        controller_id: registration.controller_id,
        modes,
        default_mode_id,
        window_kinds,
        actions: Vec::new(),
        panel_tabs: Vec::new(),
        keybindings: Vec::new(),
        utilities: Vec::new(),
        tools: Vec::new(),
        commands: Vec::new(),
        interactions: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        terminology_breadcrumbs: HashMap::new(),
        introduction: None,
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: registration.config,
        command_grammar: resolve_kernel_future(semio_framework::CommandGrammar::empty()),
        io,
        tutorials: Vec::new(),
        fault_notices: Vec::new(),
    })
}

/// 🎨️ One palette entry the browser shell can spawn a workflow node from — a thin,
/// wire-friendly snapshot of `OsAppRegistration` (drops `ConfigSpec`/`ModeDefinition`s the
/// palette doesn't need). `ports` is `app.io.all_ports()` so the palette UI can preview a node's
/// wiring before it's spawned.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPaletteEntry {
    pub plugin_id: String,
    pub app_id: String,
    pub label: LocalizedLabel,
    pub icon_id: String,
    pub ports: Vec<semio_framework::MediaPortSpec>,
}

/// 🎨️ Lists every registered app as a palette entry, sorted by `(plugin_id, app_id)` for a
/// stable snapshot. Replaces `buildSpacePrograms`'s manifest-`workflows`-driven palette (deleted with
/// `WorkflowDefinition`/`PluginManifest.workflows` in WP-0.1) — the browser shell wiring to consume
/// this via a wasm export lands in Wave 3 (WP-3.1); this crate only builds the plain Rust function for
/// now (no `#[cfg(target_arch = "wasm32")]` export region exists in this file to pair it with yet).
pub fn workflow_palette() -> Vec<AppPaletteEntry> {
    let registry = APP_REGISTRATIONS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut entries: Vec<_> = registry
        .iter()
        .map(|((plugin_id, app_id), registration)| AppPaletteEntry {
            plugin_id: plugin_id.clone(),
            app_id: app_id.clone(),
            label: registration.label.clone(),
            icon_id: registration.component_kind.clone(),
            ports: registration.inputs.iter().chain(registration.outputs.iter()).cloned().collect(),
        })
        .collect();
    entries.sort_by(|left, right| (left.plugin_id.as_str(), left.app_id.as_str()).cmp(&(right.plugin_id.as_str(), right.app_id.as_str())));
    entries
}
//#endregion 🔖️PluginRegistry

//#region 🔖️PluginRegistry
// 🚧️ B1 ripple: `WorkflowDefinition` was deleted from framework-core (WP-0.1, concurrent) — its
// `PluginRegistry.workflows` field/`register_workflow`/`find_workflow`/`workflows()` methods (per
// the master plan's explicit deletion list) go with it. Real workflow-palette derivation moves to
// `register_app_io`/`workflow_palette()` (`AppIo`-driven) in Wave 1; no call sites referenced these
// beyond the now-deleted loops in `load_plugin`/`hot_swap_plugin`/`restore`, so nothing else broke.
pub struct PluginRegistry {
    apps: HashMap<String, AppDefinition>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self { apps: HashMap::new() }
    }

    pub fn register_app(&mut self, app: AppDefinition) {
        self.apps.insert(app.id.clone(), app);
    }

    pub fn find_app(&self, app_id: &str) -> Option<&AppDefinition> {
        self.apps.get(app_id)
    }

    pub fn apps(&self) -> Vec<AppDefinition> {
        self.apps.values().cloned().collect()
    }
}
//#endregion 🔖️PluginRegistry

//#region 🧪️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️registry-unit/🦀️.rs");
//#endregion 🧪️Tests
// #endregion registry
