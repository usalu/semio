//! 🏠️ S Home launcher app — `DocumentApp` impl, command dispatch, manifest (constitutional: ui).
//!
//! WIRING + DISPATCH ONLY: every command's real body lives in its own `🎮️commands/<group>/🦀️component.rs`
//! payload module (see `app_commands!` below); this file holds only the shared catalog/draft/backbone
//! infrastructure used by 2+ command groups (and, `pub`, by `apps::space` — the Studio app resolves/
//! loads studio documents through this Home launcher's own catalog port).

use crate::apps::home::commands::settings::set_active_panel_tab;
use crate::apps::home::commands::studio::{bind_space_file, create_studio, import_space, open_space};
use crate::apps::home::commands::vfs::{delete_virtual_file_system_node, go_home, navigate_virtual_file_system_node};
use crate::apps::home::config::HomeConfig;
use crate::apps::home::terminology::SHomeLabels;
use crate::artifacts::home::SHomeDocument;
use crate::core::{ensure_space_fixtures_registered, parse_demo_space_document};
use semio_framework_os::{
    artifact_backbone_uri, collection_backbone_uri, create_backbone_document, decode_backbone_payload, draft_catalog_for, draft_uri, empty_space_projection, empty_workflow_document, encode_backbone_payload,
    export_backbone_pack, export_os_space_pack, list_os_space_catalog_entries, load_os_space_document, materialize_backbone_projection, seed_os_space_catalog_if_empty, ArtifactBody, CollectionOperation, CollectionProjection, DraftCatalog,
    MemoryBackbonePort, OsBackbonePort, OsSpaceDocument, OsWorkflowArtifactDocument, SpaceBackbonePort, SpaceKind, SpaceOperation, SpaceProjection, SpaceRole, SpaceUser, SpaceVisibility, WorkflowDocument, WorkflowOperation,
    S_COLLECTION_SCHEMA, S_SPACE_SCHEMA, S_WORKFLOW_SCHEMA,
};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::{document_backbone_ref, VcsError};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, app_commands, create_tab_stack_layout, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, FaultOrigin, Label, LocalizedLabel, UiNode};
use store::EngineHandles;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use store::LocalStorageBackbonePort;

//#region 🔖️Constants
pub const S_HOME_APP_ID: &str = "home";
pub const S_HOME_CONTROLLER_ID: &str = "s-home";
const OS_BOOT_STUDIO_ID: &str = "default";
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 🧬️ Kept as its own concrete-typed static (not just `Arc<dyn OsBackbonePort>`) so this module can
/// mint TWO different trait-object views of the SAME underlying allocation: `Arc<dyn OsBackbonePort>`
/// (os-core's byte-via-base64 bridge, used by every existing catalog call) and `Arc<dyn
/// space::SpaceBackbonePort>` (`draft_backbone_port` below, used by the real `space::DraftCatalog`
/// wiring) — both blanket-impl'd over the SAME `store::BackbonePort`, and unsizing coercion never
/// reallocates, so `Arc::as_ptr`-keyed registries (`space::draft_catalog_for`'s port identity key)
/// still line up correctly across both views.
static CATALOG_PORT_CONCRETE: LazyLock<Arc<LocalStorageBackbonePort>> = LazyLock::new(|| {
    ensure_space_fixtures_registered();
    let port = Arc::new(LocalStorageBackbonePort::new());
    let os_port: Arc<dyn OsBackbonePort> = port.clone();
    if list_os_space_catalog_entries(os_port.clone()).map_or(true, |entries| entries.is_empty()) {
        // 🧬️ `parse_demo_space_document` yields a `workflow::WorkflowDocument` (the dissolved
        // `OsProjection`'s workflow-graph half) — the space CATALOG this boot seed populates needs a
        // `space::SpaceProjection` manifest instead. `demo_name` still comes from the bundled fixture's
        // own name; the manifest itself is a fresh space with no workflow artifact wired in yet
        // (`create_os_space`'s own doc: a space only auto-creates its default collection, never a
        // workflow artifact — that stays a later, explicit user action).
        let demo_name = { let demo = parse_demo_space_document(); if demo.name.trim().is_empty() { "Demo Studio".into() } else { demo.name } };
        let mut projection = empty_space_projection(&demo_name, SpaceKind::Atelier, SpaceVisibility::Private);
        projection.users.push(SpaceUser { id: "local".into(), name: demo_name.clone(), avatar: None, role: SpaceRole::Author });
        let seed: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, OS_BOOT_STUDIO_ID, &demo_name, projection);
        let _ = seed_os_space_catalog_if_empty(seed, os_port);
    }
    port
});

/// 🧬️ Session-local, ephemeral (in-memory only) counterpart to `CATALOG_PORT_CONCRETE` — every draft
/// space a user creates from Home lives here at `space::draft_uri(id)` until it's promoted (bound to a
/// file or a real catalog), matching the "never persisted" semantics of a pure ephemeral registry.
static TEMP_CATALOG_PORT_CONCRETE: LazyLock<Arc<MemoryBackbonePort>> = LazyLock::new(|| Arc::new(MemoryBackbonePort::new()));

fn shared_studio_ports() -> Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>> {
    static REGISTRY: OnceLock<Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Arc::new(Mutex::new(HashMap::new()))).clone()
}

/// 🌉️ `pub` (not `pub(crate)`): `apps::space` resolves studios through the Home launcher's own catalog
/// port — see `apps::space`'s `openSpace`/`exportStudioPack`/`exportStudioDsl`/`importSpacePackPayload`
/// commands.
pub fn catalog_port() -> Arc<dyn OsBackbonePort> {
    CATALOG_PORT_CONCRETE.clone()
}

pub(crate) fn temp_catalog_port() -> Arc<dyn OsBackbonePort> {
    TEMP_CATALOG_PORT_CONCRETE.clone()
}

/// 🔌️ `space::SpaceBackbonePort` view over the SAME ephemeral port `temp_catalog_port` uses — the port
/// every draft studio's real envelope bytes are relocated through by `space::DraftCatalog`.
pub(crate) fn draft_backbone_port() -> Arc<dyn SpaceBackbonePort> {
    TEMP_CATALOG_PORT_CONCRETE.clone()
}

/// 🗄️ The port-keyed `space::DraftCatalog` for `draft_backbone_port` — every draft studio's
/// bookkeeping (id, kind, TTL) lives here; `space::draft_catalog_for` guarantees the SAME instance is
/// returned every call since `draft_backbone_port` always unsizes the SAME `TEMP_CATALOG_PORT_CONCRETE`
/// allocation.
pub(crate) fn ephemeral_draft_catalog() -> Arc<DraftCatalog> {
    draft_catalog_for(&draft_backbone_port())
}

/// 🕰️ Wall-clock millis, reusing `store::now_iso`'s own wasm-safe implementation (its string is
/// already the millis count as text) rather than duplicating the `cfg(target_arch = "wasm32")`
/// branching this crate has no `js-sys` dependency to replicate directly.
fn now_ms() -> u64 {
    store::now_iso().parse().unwrap_or(0)
}

pub(crate) fn register_studio_port(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    register_studio_port_for(&HomeApp::default(), space_id, port)
}

pub(crate) fn register_studio_port_for(app: &HomeApp, space_id: &str, port: Arc<dyn OsBackbonePort>) {
    if let Ok(mut guard) = app.studio_ports.lock() {
        guard.insert(space_id.into(), port);
    }
}

/// @emoji 🆕️ Mints a fresh draft space manifest (empty, no collections) for the default create path — a
/// `space::SpaceProjection` document registered as a draft (`kind_id = "s.space"`) at `space::draft_uri(id)`
/// on the ephemeral port, never on the real catalog port, never tracked as a `space://` catalog entry.
pub(crate) fn create_and_register_ephemeral_studio(name: &str) -> String {
    let owner = SpaceUser { id: "local".into(), name: name.into(), avatar: None, role: SpaceRole::Author };
    let mut projection = empty_space_projection(name.trim(), SpaceKind::Atelier, SpaceVisibility::Private);
    projection.users.push(owner);
    let draft = ephemeral_draft_catalog().create_draft("s.space", S_SPACE_SCHEMA, name.trim(), now_ms(), None);
    let document: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, &draft.artifact_id, name.trim(), projection);
    if let Ok(payload) = encode_backbone_payload(&document) {
        let _ = draft_backbone_port().write(&draft_uri(&draft.artifact_id), &payload);
    }
    draft.artifact_id
}

/// @emoji 📂️ Resolves a studio id against the draft catalog, registered ports, then catalogs.
///
/// 🌉️ `pub` (not `pub(crate)`): `apps::space` resolves the studio it is asked to open through this same
/// lookup — see the note on {@link catalog_port}.
pub fn resolve_studio_document(space_id: &str) -> Option<OsSpaceDocument> {
    resolve_studio_document_for(&HomeApp::default(), space_id)
}

pub fn resolve_studio_document_for(app: &HomeApp, space_id: &str) -> Option<OsSpaceDocument> {
    let draft_port = draft_backbone_port();
    if let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), &draft_uri(space_id)) {
        if !payload.is_empty() {
            if let Ok(document) = decode_backbone_payload::<SpaceProjection, SpaceOperation>(&payload, S_SPACE_SCHEMA) {
                return Some(document);
            }
        }
    }
    if let Ok(guard) = app.studio_ports.lock() {
        if let Some(port) = guard.get(space_id) {
            if let Ok(document) = load_os_space_document(space_id, port.clone()) {
                return Some(document);
            }
        }
    }
    for port in [temp_catalog_port(), catalog_port()] {
        if let Ok(document) = load_os_space_document(space_id, port) {
            return Some(document);
        }
    }
    None
}

/// @emoji 📦️ Pack+spr bytes for `HostEffect::LoadDocument` / host `loadAppDocumentPack`.
///
/// 🌉️ `pub` (not `pub(crate)`): `apps::space`'s `openSpace` command loads the studio document it just
/// resolved through this helper — see the note on {@link catalog_port}.
pub fn space_document_envelope_pack(document: &OsSpaceDocument) -> Option<store::DocumentPackFiles> {
    export_os_space_pack(document).ok()
}

//#region 🔖️WorkflowArtifactResolution
/// 🕸️ "Space session -> active workflow artifact" resolution — a space manifest carries no graph of
/// its own anymore, the graph lives in a separate `s.workflow` artifact document addressed via a
/// `CollectionEntry` inside one of the space's collections. Searches every collection the resolved
/// space manifest references, through the SAME port search order `resolve_studio_document` uses, for
/// the first `CollectionEntry` whose body is an `s.workflow` document.
fn resolve_backbone_bytes(app: &HomeApp, uri: &str) -> Option<Vec<u8>> {
    let draft_port = draft_backbone_port();
    if let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), uri) {
        if !payload.is_empty() {
            return Some(payload);
        }
    }
    if let Ok(guard) = app.studio_ports.lock() {
        for port in guard.values() {
            if let Ok(payload) = port.read(uri) {
                if !payload.is_empty() {
                    return Some(payload);
                }
            }
        }
    }
    for port in [temp_catalog_port(), catalog_port()] {
        if let Ok(payload) = port.read(uri) {
            if !payload.is_empty() {
                return Some(payload);
            }
        }
    }
    None
}

pub fn resolve_workflow_artifact_document(space_id: &str, space_document: &OsSpaceDocument) -> Option<OsWorkflowArtifactDocument> {
    resolve_workflow_artifact_document_for(&HomeApp::default(), space_id, space_document)
}

pub fn resolve_workflow_artifact_document_for(app: &HomeApp, space_id: &str, space_document: &OsSpaceDocument) -> Option<OsWorkflowArtifactDocument> {
    let projection = materialize_backbone_projection(space_document, &space_document.applied_edit_ids).ok()?;
    for collection_ref in &projection.collections {
        let collection_uri = collection_backbone_uri(space_id, &collection_ref.id);
        let Some(collection_payload) = resolve_backbone_bytes(app, &collection_uri) else { continue };
        let Ok(collection_document) = decode_backbone_payload::<CollectionProjection, CollectionOperation>(&collection_payload, S_COLLECTION_SCHEMA) else { continue };
        let Ok(collection_projection) = materialize_backbone_projection(&collection_document, &collection_document.applied_edit_ids) else { continue };
        for entry in &collection_projection.entries {
            let ArtifactBody::Document { schema, document_id } = entry.body.as_ref() else { continue };
            if schema != S_WORKFLOW_SCHEMA {
                continue;
            }
            let artifact_uri = artifact_backbone_uri(space_id, document_id);
            let Some(artifact_payload) = resolve_backbone_bytes(app, &artifact_uri) else { continue };
            if let Ok(workflow_document) = decode_backbone_payload::<WorkflowDocument, WorkflowOperation>(&artifact_payload, S_WORKFLOW_SCHEMA) {
                return Some(workflow_document);
            }
        }
    }
    None
}

/// 🆕️ Mints a fresh, valid, empty `s.workflow` artifact document for a space that has none registered
/// yet — the "genuinely new/default space" leg of `resolve_workflow_artifact_document`'s three-way
/// fallback (existing registered artifact / demo fixture / fresh empty document). Not persisted as a
/// `CollectionEntry` (real artifact-registration UI is a later wave) — the studio editor still gets a
/// real, decodable `WorkflowDocument` pack instead of a broken placeholder, it just starts from a blank
/// canvas each time until persistence is wired.
pub fn empty_workflow_artifact_document(space_id: &str, space_name: &str) -> OsWorkflowArtifactDocument {
    create_backbone_document(S_WORKFLOW_SCHEMA, space_id, space_name, empty_workflow_document())
}

/// @emoji 📦️ `s.workflow` counterpart of `space_document_envelope_pack` — pack+spr bytes for
/// `HostEffect::LoadDocument` / host `loadAppDocumentPack`, sized to what `apps::space`'s
/// `DocumentApp::Projection` (`WorkflowDocument`) actually decodes.
pub fn workflow_artifact_envelope_pack(document: &OsWorkflowArtifactDocument) -> Option<store::DocumentPackFiles> {
    export_backbone_pack(document).ok()
}
//#endregion 🔖️WorkflowArtifactResolution

/// 🌉️ `pub` (not `pub(crate)`, and not `#[cfg(test)]`): `apps::space`'s own tests (a sibling module)
/// seed a studio through this hook — a `#[cfg(test)]` gate here would vanish when this module is pulled
/// in as `apps::space`'s ordinary (non-dev) dependency, since `#[cfg(test)]` only activates for the
/// crate under test itself, not its dependencies.
pub fn register_studio_port_for_test(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    register_studio_port_for(&HomeApp::default(), space_id, port);
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn sync_os_space_document_helper(document: &OsSpaceDocument, backbone_uri: &str, port: &Arc<dyn OsBackbonePort>) -> Result<(), VcsError> {
    let mut synced = document.clone();
    synced.backbone = Some(document_backbone_ref(backbone_uri));
    port.write(backbone_uri, &encode_backbone_payload(&synced)?)
}

/// 🎯️ The TTL-sweep call site — `list_drafts_sweeping_expired` clears any stale draft bookkeeping (and
/// best-effort tombstones its bytes) BEFORE this listing is built, so Home's VFS never shows a studio
/// draft past its deadline. Mirrors the spirit of os-core's own catalog-listing entry points.
pub(crate) fn list_all_space_catalog_entries() -> Vec<semio_framework_os::OsSpaceCatalogEntry> {
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    for port in [catalog_port(), temp_catalog_port()] {
        if let Ok(rows) = list_os_space_catalog_entries(port) {
            for entry in rows {
                if seen.insert(entry.id.clone()) {
                    entries.push(entry);
                }
            }
        }
    }
    let draft_port = draft_backbone_port();
    for draft in ephemeral_draft_catalog().list_drafts_sweeping_expired(now_ms(), &draft_port) {
        if draft.kind_id != "s.space" || !seen.insert(draft.artifact_id.clone()) {
            continue;
        }
        let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), &draft_uri(&draft.artifact_id)) else { continue };
        if payload.is_empty() {
            continue;
        }
        let Ok(document) = decode_backbone_payload::<SpaceProjection, SpaceOperation>(&payload, S_SPACE_SCHEMA) else { continue };
        let projection = &document.vcs.initial_projection;
        entries.push(semio_framework_os::OsSpaceCatalogEntry {
            id: draft.artifact_id,
            name: document.name.clone(),
            backbone_uri: String::new(),
            kind: projection.kind,
            visibility: projection.visibility,
            collection_count: projection.collections.len(),
            updated_at: "0".into(),
        });
    }
    entries
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️HomeCommand
app_commands! {
    /// 🎯️ `HomeApp::Command` — the SOLE dispatch surface for the Home launcher's own behavior, one
    /// variant per action declared in `create_home_app`'s manifest.
    pub enum HomeCommand for SHomeDocument, crate::artifacts::home::op::SHomeOperation, HomeConfig, crate::apps::home::config::HomeConfigOperation {
        "createStudio" as "create-studio" => create_studio::CreateStudio,
        "bindSpaceFile" as "bind-space-file" => bind_space_file::BindSpaceFile,
        "importSpace" as "import-space" => import_space::ImportSpace,
        "openSpace" as "open-space" => open_space::OpenSpace,
        "navigateVirtualFileSystemNode" as "navigate-vfs-node" => navigate_virtual_file_system_node::NavigateVirtualFileSystemNode,
        "deleteVirtualFileSystemNode" as "delete-vfs-node" => delete_virtual_file_system_node::DeleteVirtualFileSystemNode,
        "goHome" as "go-home" => go_home::GoHome,
        "setActivePanelTab" as "active-panel-tab" => set_active_panel_tab::SetActivePanelTab,
    }
}
//#endregion 🔖️HomeCommand

//#region 🔖️HomeApp
/// 🧪️ Unit struct — the Home launcher holds catalog bootstrap ports plus per-session studio port
/// bindings for folder/file-backed studios.
pub struct HomeApp {
    studio_ports: Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>,
}

impl Default for HomeApp {
    fn default() -> Self {
        Self { studio_ports: shared_studio_ports() }
    }
}

impl DocumentApp for HomeApp {
    type Projection = SHomeDocument;
    type Operation = crate::artifacts::home::op::SHomeOperation;
    type Config = HomeConfig;
    type ConfigOperation = crate::apps::home::config::HomeConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = HomeCommand;

    const APP_ID: &'static str = S_HOME_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = "s.home";

    fn initial_projection() -> SHomeDocument {
        SHomeDocument { schema: "s.home".into(), catalog_generation: 0 }
    }

    fn command_id(command: &HomeCommand) -> &str {
        command.command_id()
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `HomeCommand` until every call site speaks OpBinary.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<HomeCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "createStudio" => Ok(HomeCommand::CreateStudio(create_studio::CreateStudio {
                name: str_field("name").unwrap_or_else(|| "Untitled".into()),
                kind: str_field("kind").unwrap_or_else(|| "catalog".into()),
                folder_path: str_field("folderPath").or_else(|| str_field("folder_path")),
            })),
            "bindSpaceFile" => Ok(HomeCommand::BindSpaceFile(bind_space_file::BindSpaceFile {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                file_path: str_field("filePath").or_else(|| str_field("file_path")).unwrap_or_default(),
            })),
            "importSpace" => Ok(HomeCommand::ImportSpace(import_space::ImportSpace { dsl: str_field("dsl").or_else(|| str_field("payload")) })),
            "openSpace" => Ok(HomeCommand::OpenSpace(open_space::OpenSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default() })),
            "navigateVirtualFileSystemNode" => Ok(HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode {
                node_id: str_field("nodeId")
                    .or_else(|| str_field("node_id"))
                    .or_else(|| str_field("spaceId"))
                    .or_else(|| str_field("space_id"))
                    .unwrap_or_default(),
            })),
            "deleteVirtualFileSystemNode" => Ok(HomeCommand::DeleteVirtualFileSystemNode(delete_virtual_file_system_node::DeleteVirtualFileSystemNode {
                node_id: str_field("nodeId")
                    .or_else(|| str_field("node_id"))
                    .or_else(|| str_field("spaceId").map(|id| format!("studio:{id}")))
                    .or_else(|| str_field("space_id").map(|id| format!("studio:{id}")))
                    .unwrap_or_default(),
            })),
            "goHome" => Ok(HomeCommand::GoHome(go_home::GoHome {})),
            "setActivePanelTab" => Ok(HomeCommand::SetActivePanelTab(set_active_panel_tab::SetActivePanelTab { tab_id: str_field("tabId").or_else(|| str_field("tab_id")).unwrap_or_default() })),
            other => Err(Fault::new(FaultOrigin::App, "s.home.unhandled-action", format!("home: unhandled action id {other}"))),
        }
    }

    fn handle(command: &HomeCommand, doc: &DocumentView<'_, SHomeDocument>, cfg: &ConfigView<'_, HomeConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<crate::artifacts::home::op::SHomeOperation, crate::apps::home::config::HomeConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, _doc: &DocumentView<'_, SHomeDocument>, cfg: &ConfigView<'_, HomeConfig>) -> UiNode {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<SHomeLabels>(&cfg.projection.locale);
        // 🪟 `VcsDocumentApp::render` appends `:{windowInstanceId}` when `view_state.window_id` is set —
        // strip it so Home's single body key still matches.
        let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
        match base_body_key {
            crate::apps::home::modes::explore::windows::main::S_HOME_BODY => crate::apps::home::modes::explore::windows::main::render(labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️HomeApp

//#region 🔖️HomeManifest
pub fn create_home_app() -> App {
    let mut app = App::from_builder(
        App::builder(S_HOME_APP_ID, LocalizedLabel::native("Home", "Startseite"))
            .document(["semio", "s", "home"])
            .icon_id("home")
            .mode_def(crate::apps::home::modes::explore::definition())
            .default_mode_id("explore")
            .window_kind_def(crate::apps::home::modes::explore::windows::main::definition())
            .default_layout(create_tab_stack_layout(&[crate::apps::home::modes::explore::windows::main::S_HOME_WINDOW.into()], Some(&["Studios".into()])))
            .operation("createStudio", LocalizedLabel::native("Create Studio", "Studio erstellen"))
            .shell_action("bindSpaceFile", LocalizedLabel::native("Bind Studio File", "Studio-Datei verknüpfen"))
            .operation("importSpace", LocalizedLabel::native("Import Studio", "Studio importieren"))
            .shell_action("openSpace", LocalizedLabel::native("Open Studio", "Studio öffnen"))
            .shell_action("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigate File System Node", "Dateisystemknoten navigieren"))
            .operation("deleteVirtualFileSystemNode", LocalizedLabel::native("Delete File System Node", "Dateisystemknoten löschen"))
            .shell_action("goHome", LocalizedLabel::native("Go Home", "Zur Startseite"))
            .view_action("setActivePanelTab", LocalizedLabel::native("Set Active Panel Tab", "Aktiven Panel-Tab festlegen"))
            .keybinding("mod+n", "createStudio")
            .keybinding("mod+o", "importSpace"),
    );
    app.definition.controller_id = S_HOME_CONTROLLER_ID.into();
    app
}
//#endregion 🔖️HomeManifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn empty_history() -> semio_framework_plugin::HistoryView {
        semio_framework_plugin::HistoryView::empty()
    }

    #[test]
    fn home_manifest_uses_home_app_id() {
        let app = create_home_app();
        assert_eq!(app.definition.id, "home");
        assert_eq!(app.definition.controller_id, "s-home");
    }

    #[test]
    fn home_declares_create_space_action() {
        let app = create_home_app();
        assert!(app.definition.actions.iter().any(|action| action.id == "createStudio"));
    }

    #[test]
    fn space_document_persists_through_backbone_port() {
        // 🕳️ `parse_demo_space_document()` yields a `workflow::WorkflowDocument` (the demo fixture's own
        // artifact content), not a `space::SpaceProjection`-backed catalog entry
        // `seed_os_space_catalog_if_empty` expects. This test exercises the space-manifest persistence
        // path specifically, so it mints its own manifest instead.
        let port: Arc<dyn OsBackbonePort> = Arc::new(LocalStorageBackbonePort::new());
        let projection = empty_space_projection("Persist Test", SpaceKind::Atelier, SpaceVisibility::Private);
        let demo: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, "persist-test", "Persist Test", projection);
        let _ = seed_os_space_catalog_if_empty(demo, port.clone()).expect("seed");
        let loaded = load_os_space_document("persist-test", port.clone()).expect("load");
        assert_eq!(loaded.id, "persist-test");
        assert_eq!(loaded.name, "Persist Test");
    }

    #[test]
    fn home_labels_resolve_native_english_by_default() {
        let history = empty_history();
        let home = HomeApp;
        let home_doc = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = DocumentView { projection: &home_doc, history: &history };
        let config = HomeConfig::default();
        let cfg = ConfigView { projection: &config };
        let home_node = home.render(crate::apps::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg);
        assert!(serde_json::to_string(&home_node).unwrap().contains("No studios yet. Create one from the navbar."));
    }

    #[test]
    fn home_labels_resolve_native_german_locale() {
        let history = empty_history();
        let home = HomeApp;
        let home_doc = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = DocumentView { projection: &home_doc, history: &history };
        let config = HomeConfig { locale: "de".into(), ..HomeConfig::default() };
        let cfg = ConfigView { projection: &config };
        let home_node = home.render(crate::apps::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg);
        assert!(serde_json::to_string(&home_node).unwrap().contains("Noch keine Studios vorhanden"));
    }
}
//#endregion 🧪️Tests
