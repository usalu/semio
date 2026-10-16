//! 📂️ `shell-doc-open` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🔖️IdentityPure
/// 🔗️ ticket §2 — `attach_sync_backbone`/`open_document`'s default binding decision: `[Hub, Folder]`
/// when an identity AND a space both exist, `[Folder]` with a data dir but no identity, `[]`
/// otherwise. Pure and free-standing so the decision is unit-testable without a live `ShellState`.
pub fn default_persistence_bindings(identity: Option<&Identity>, space_id: Option<&str>, data_dir: Option<&std::path::Path>, surface: Option<&str>) -> Vec<PersistenceBinding> {
    let folder = match (space_id, data_dir) {
        (Some(space_id), Some(data_dir)) => Some(PersistenceBinding::Folder { path: data_dir.join("spaces").join(space_id) }),
        _ => None,
    };
    match (identity, space_id) {
        (Some(identity), Some(space_id)) => {
            let mut bindings = vec![PersistenceBinding::Hub { base_url: identity.hub_base_url.clone(), space_id: space_id.to_string(), surface: surface.map(str::to_string) }];
            bindings.extend(folder);
            bindings
        }
        _ => folder.into_iter().collect(),
    }
}

/// 📂️ Parses the schema-first opening relay shared with the React shell. String and numeric
/// role forms normalize identically; a surface-suffixed artifact ref must agree with the role; app
/// and document coordinates are all-or-nothing pairs.
#[derive(Debug)]
pub struct OpenArtifactRelayTarget {
    pub artifact_ref: String,
    pub dialect: semio_framework_artifact_reference::ArtifactDialect,
    pub role: semio_framework::AppRole,
    pub plugin_id: Option<String>,
    pub app_id: Option<String>,
    pub document_id: Option<String>,
    pub space_id: Option<String>,
    pub schema: Option<String>,
}

/// 🚪️ Where the one frame-pumped document open stands: a hub document's component is being resolved by
/// the serving catalog generation, the guest's app instance is being created, the document's canonical
/// checkpoint pair is being fetched and loaded into it, or the open settled as cancelled or failed — a
/// settled record stays until the user closes its band, like a settled plugin install.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellDocumentOpenPhase {
    Resolving,
    Instantiating,
    Seeding,
    Cancelled,
    Failed(String),
}

/// 📨️ What one detached open step answered: a hub document's resolved owner — the plugin and app its lease
/// names, the program to mount (`None` when the local one already is the lease's component) and the
/// checkpoint the lease authorizes — an app instance, or a seeded document with the canonical pair its
/// guest now holds (`None` for a document bound to no hub).
pub enum ShellDocumentOpenAnswer {
    #[cfg(not(target_arch = "wasm32"))]
    Resolved {
        plugin_id: String,
        app_id: String,
        program: Option<ProgramBridgeEntry>,
        checkpoint: semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1,
    },
    Instantiated(u32),
    Seeded(Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>),
}

/// ⏳️ The resolution step a hub document open is on, as its band reads it ([`ShellDocumentOpening::resolve_step`]).
#[cfg(not(target_arch = "wasm32"))]
pub fn document_open_resolve_step_code(step: semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep) -> u8 {
    use semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep;
    match step {
        ExecutionTargetModuleStep::Lease => 0,
        ExecutionTargetModuleStep::Component => 1,
        ExecutionTargetModuleStep::Descriptor => 2,
        ExecutionTargetModuleStep::Verified => 3,
    }
}

/// 🎯️ The document half of an open: what the relay asked for, and — once a hub resolution read the
/// document's lease — the checkpoint that lease authorizes.
pub struct ShellDocumentOpenTarget {
    pub document_id: String,
    pub schema: String,
    pub checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
}

/// 🚪️ The one document open this shell runs at a time (`os.open-artifact`), advanced by the frame pump
/// ([`ShellState::advance_document_opening`]). Every guest turn it waits on — the app instance, the
/// genesis load — runs detached ([`ShellDetached`]), so the frame loop keeps building frames, painting
/// this open's band (phase, step, elapsed, cancel) and taking input while the guest works; before, the
/// whole open held the shell for ~17 s in a debug build (ticket 26/09/23 slice WG8).
pub struct ShellDocumentOpening {
    pub label: String,
    pub phase: ShellDocumentOpenPhase,
    pub cancel_requested: bool,
    pub started_at_ms: f64,
    pub plugin_id: String,
    pub app_id: String,
    pub document: Option<ShellDocumentOpenTarget>,
    pub pending: Option<ShellDetached<Result<ShellDocumentOpenAnswer, semio_framework_os_renderer_wgpu_program_bridge::ProgramFault>>>,
    pub prepared: Option<ShellPreparedDocumentOpen>,
    /// 🛑️ The token the detached hub resolution's requests check; cancelling the open cancels it.
    pub cancel: Option<CancelToken>,
    /// ⏳️ The resolution step the detached hub resolution last reported (lease, component, descriptor, verified).
    pub resolve_step: std::sync::Arc<std::sync::atomic::AtomicU8>,
}

impl ShellDocumentOpening {
    /// 🚪️ An open whose first detached step is `pending`, in `phase`.
    pub fn new(label: String, phase: ShellDocumentOpenPhase, plugin_id: String, app_id: String, document: Option<ShellDocumentOpenTarget>, pending: Option<ShellDetached<Result<ShellDocumentOpenAnswer, semio_framework_os_renderer_wgpu_program_bridge::ProgramFault>>>) -> Self {
        Self { label, phase, cancel_requested: false, started_at_ms: chrome_now_ms(), plugin_id, app_id, document, pending, prepared: None, cancel: None, resolve_step: std::sync::Arc::default() }
    }

    /// ⏳️ Whether a step is still out; a settled record only waits for its band to be closed.
    pub fn running(&self) -> bool {
        matches!(self.phase, ShellDocumentOpenPhase::Resolving | ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding)
    }
}

/// 🎯 Resolves the descriptor-bound canonical surface id this local selection may request. It is a
/// preference, never an authority: a local WGPU selection verifies no execution-target bytes and so
/// can never mint a `DocumentExecutionTargetLeaseFieldsV1`.
pub fn document_socket_surface_from_descriptor(plugin_id: &str, package_id: Option<&str>, manifest: &semio_framework::PluginManifest, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {
    let _package_id = package_id.filter(|value| !value.is_empty()).ok_or_else(|| "document open requires a verified package descriptor".to_string())?;
    if plugin_id != manifest.plugin_id || manifest.version.is_empty() || !manifest.apps.iter().any(|candidate| candidate.id == app.id) || !app.window_kinds.iter().any(|window| window.id == window_kind_id) {
        return Err("document open local plugin selection is not descriptor-bound".into());
    }
    Ok(semio_framework::manifest::surface_app_id(&app.dialect, app.role))
}

/// 🪪️ Admits a hub-selected execution-target lease only for the package this shell actually mounted:
/// the same plugin, the same package, the same component digest, the document's schema and the
/// surface the shell requested. Anything else would lend the document actor a kind identity for
/// code that is not running here.
pub fn document_execution_target_admitted(
    lease: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1,
    plugin_id: &str,
    package_id: Option<&str>,
    component_sha256: Option<&str>,
    artifact_schema: &str,
    surface_id: &str,
) -> Result<(), &'static str> {
    if lease.package.plugin_id != plugin_id {
        return Err("document-execution-target.plugin-mismatch");
    }
    if package_id != Some(lease.package.package_id.as_str()) {
        return Err("document-execution-target.package-mismatch");
    }
    if component_sha256 != Some(lease.component.sha256.as_str()) {
        return Err("document-execution-target.component-mismatch");
    }
    if lease.artifact.schema != artifact_schema {
        return Err("document-execution-target.schema-mismatch");
    }
    if lease.surface.surface_id != surface_id {
        return Err("document-execution-target.surface-mismatch");
    }
    Ok(())
}

pub fn wgpu_document_socket_surface(program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {
    document_socket_surface_from_descriptor(&program.plugin_id, program.package_id.as_deref(), &program.manifest, app, window_kind_id)
}

//#endregion 🔖️IdentityPure

