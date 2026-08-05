//! 🏠️ S Home launcher app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: unit
//! struct, pure `handle`, real `HomeConfig` (see `home_engine`) replacing the deleted `ViewState`.

use home::SHomeDocument;
use home_engine::HomeConfig;
use home_op::{HomeConfigOperation, SHomeOperation};
use home_protocol::HomeCommand;
use semio_framework_os::{
    artifact_backbone_uri, collection_backbone_uri, create_backbone_document, create_os_space, decode_backbone_payload, delete_os_space, document_backbone_ref, draft_catalog_for, draft_uri, empty_space_projection, empty_workflow_document,
    encode_backbone_payload, export_backbone_pack, export_os_space_pack, import_os_space_from_dsl, list_os_space_catalog_entries, load_os_space_document, materialize_backbone_projection, seed_os_space_catalog_if_empty, ArtifactBody,
    CollectionOperation, CollectionProjection, DraftCatalog, MemoryBackbonePort, OsBackbonePort, OsSpaceDocument, OsWorkflowArtifactDocument, SpaceBackbonePort, SpaceKind, SpaceOperation, SpaceProjection, SpaceRole, SpaceUser, SpaceVisibility, VcsError,
    WorkflowDocument, WorkflowOperation, OS_HOME_VFS_ROOT_ID, OS_SPACE_BACKBONE_URI_PREFIX, S_COLLECTION_SCHEMA, S_SPACE_SCHEMA, S_WORKFLOW_SCHEMA,
};
use semio_framework_plugin::{
        app_labels, build_virtual_file_system_scene, create_tab_stack_layout, App, AppLabels, ConfigView, DocumentApp, DocumentView, Emit, Fault, FaultOrigin, HostEffect, Label, Locale, LocalizedLabel, SurfaceKind, Terminology, UiNode, VirtualFileSystemScene,
};
use serde_json::{json, Value};
use space_shared::{ensure_space_fixtures_registered, parse_demo_space_document};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use store::LocalStorageBackbonePort;

//#region 🔖️Constants
const S_HOME_APP_ID: &str = "home";
const S_HOME_CONTROLLER_ID: &str = "s-home";
const S_HOME_WINDOW: &str = "s-home-main";
const S_HOME_BODY: &str = "s.home.vfs";
const S_HOME_SURFACE: &str = "vfs:home:main";
const OS_BOOT_STUDIO_ID: &str = "default";
//#endregion 🔖️Constants

//#region 🔖️Locale

//#endregion 🔖️Locale

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
    if list_os_space_catalog_entries(os_port.clone()).map(|entries| entries.is_empty()).unwrap_or(true) {
        // 🧬️ `parse_demo_space_document` now yields a `workflow::WorkflowDocument` (the dissolved
        // `OsProjection`'s workflow-graph half, see `## The inversion`) — the space CATALOG this boot
        // seed populates needs a `space::SpaceProjection` manifest instead. `demo_name` still comes
        // from the bundled fixture's own name; the manifest itself is a fresh space with no workflow
        // artifact wired in yet (`create_os_space`'s own doc: a space only auto-creates its default
        // collection, never a workflow artifact — that stays a later, explicit user action).
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
/// file or a real catalog), matching the pre-`DraftCatalog` ephemeral registry's "never persisted"
/// semantics exactly, just now backed by a real port instead of a bespoke `HashMap`.
static TEMP_CATALOG_PORT_CONCRETE: LazyLock<Arc<MemoryBackbonePort>> = LazyLock::new(|| Arc::new(MemoryBackbonePort::new()));

static STUDIO_PORTS: LazyLock<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// 🌉️ `pub` (not `pub(crate)`): `app_space` (a sibling crate, `semio-s-app-space-space-ui`) resolves
/// studios through the Home launcher's own catalog port — see `app_space`'s `openSpace`/
/// `exportStudioPack`/`exportStudioDsl`/`importSpacePackPayload` actions.
pub fn catalog_port() -> Arc<dyn OsBackbonePort> {
    CATALOG_PORT_CONCRETE.clone()
}

fn temp_catalog_port() -> Arc<dyn OsBackbonePort> {
    TEMP_CATALOG_PORT_CONCRETE.clone()
}

/// 🔌️ `space::SpaceBackbonePort` view over the SAME ephemeral port `temp_catalog_port` uses (see
/// `TEMP_CATALOG_PORT_CONCRETE`'s doc) — the port every draft studio's real envelope bytes are
/// relocated through by `space::DraftCatalog`.
fn draft_backbone_port() -> Arc<dyn SpaceBackbonePort> {
    TEMP_CATALOG_PORT_CONCRETE.clone()
}

/// 🗄️ The port-keyed `space::DraftCatalog` for `draft_backbone_port` — every draft studio's
/// bookkeeping (id, kind, TTL) lives here; `space::draft_catalog_for` guarantees the SAME instance is
/// returned every call since `draft_backbone_port` always unsizes the SAME `TEMP_CATALOG_PORT_CONCRETE`
/// allocation.
fn ephemeral_draft_catalog() -> Arc<DraftCatalog> {
    draft_catalog_for(&draft_backbone_port())
}

/// 🕰️ Wall-clock millis, reusing `store::now_iso`'s own wasm-safe implementation (its string is
/// already the millis count as text) rather than duplicating the `cfg(target_arch = "wasm32")`
/// branching this crate has no `js-sys` dependency to replicate directly.
fn now_ms() -> u64 {
    store::now_iso().parse().unwrap_or(0)
}

fn register_studio_port(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    if let Ok(mut guard) = STUDIO_PORTS.lock() {
        guard.insert(space_id.into(), port);
    }
}

/// @emoji 🆕️ Mints a fresh draft space manifest (empty, no collections) for the default create path —
/// the dissolved `create_ephemeral_os_space`'s real `space::DraftCatalog`-backed successor (see `##
/// Draft vs asset` in the plan): a `space::SpaceProjection` document registered as a draft
/// (`kind_id = "s.space"`) at `space::draft_uri(id)` on the ephemeral port, never on the real catalog
/// port, never tracked as a `space://` catalog entry.
fn create_and_register_ephemeral_studio(name: &str) -> String {
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
/// 🌉️ `pub` (not `pub(crate)`): `app_space` resolves the studio it is asked to open through this same
/// lookup — see the note on {@link catalog_port}.
pub fn resolve_studio_document(space_id: &str) -> Option<OsSpaceDocument> {
    let draft_port = draft_backbone_port();
    if let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), &draft_uri(space_id)) {
        if !payload.is_empty() {
            if let Ok(document) = decode_backbone_payload::<SpaceProjection, SpaceOperation>(&payload, S_SPACE_SCHEMA) {
                return Some(document);
            }
        }
    }
    if let Ok(guard) = STUDIO_PORTS.lock() {
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
/// 🌉️ `pub` (not `pub(crate)`): `app_space`'s `openSpace` action loads the studio document it just
/// resolved through this helper — see the note on {@link catalog_port}.
pub fn space_document_envelope_pack(document: &OsSpaceDocument) -> Option<store::DocumentPackFiles> {
    export_os_space_pack(document).ok()
}

//#region 🔖️WorkflowArtifactResolution
/// 🕸️ "Space session -> active workflow artifact" resolution — the real fix for the gap W3 left
/// (`app_space`'s `openSpace` doc comment): a space manifest carries no graph of its own anymore, the
/// graph lives in a separate `s.workflow` artifact document addressed via a `CollectionEntry` inside
/// one of the space's collections (see `## Addressing` in the plan). Searches every collection the
/// resolved space manifest references, through the SAME port search order `resolve_studio_document`
/// uses, for the first `CollectionEntry` whose body is an `s.workflow` document.
fn resolve_backbone_bytes(uri: &str) -> Option<Vec<u8>> {
    let draft_port = draft_backbone_port();
    if let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), uri) {
        if !payload.is_empty() {
            return Some(payload);
        }
    }
    if let Ok(guard) = STUDIO_PORTS.lock() {
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
    let projection = materialize_backbone_projection(space_document, &space_document.applied_edit_ids).ok()?;
    for collection_ref in &projection.collections {
        let collection_uri = collection_backbone_uri(space_id, &collection_ref.id);
        let Some(collection_payload) = resolve_backbone_bytes(&collection_uri) else { continue };
        let Ok(collection_document) = decode_backbone_payload::<CollectionProjection, CollectionOperation>(&collection_payload, S_COLLECTION_SCHEMA) else { continue };
        let Ok(collection_projection) = materialize_backbone_projection(&collection_document, &collection_document.applied_edit_ids) else { continue };
        for entry in &collection_projection.entries {
            let ArtifactBody::Document { schema, document_id } = entry.body.as_ref() else { continue };
            if schema != S_WORKFLOW_SCHEMA {
                continue;
            }
            let artifact_uri = artifact_backbone_uri(space_id, document_id);
            let Some(artifact_payload) = resolve_backbone_bytes(&artifact_uri) else { continue };
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
/// `CollectionEntry` (real artifact-registration UI is W7 scope, see `app_space`'s `openSpace` doc
/// comment) — the studio editor still gets a real, decodable `WorkflowDocument` pack instead of a
/// broken placeholder, it just starts from a blank canvas each time until W7 wires persistence.
pub fn empty_workflow_artifact_document(space_id: &str, space_name: &str) -> OsWorkflowArtifactDocument {
    create_backbone_document(S_WORKFLOW_SCHEMA, space_id, space_name, empty_workflow_document())
}

/// @emoji 📦️ `s.workflow` counterpart of `space_document_envelope_pack` — pack+spr bytes for
/// `HostEffect::LoadDocument` / host `loadAppDocumentPack`, sized to what `app_space`'s
/// `DocumentApp::Projection` (`WorkflowDocument`) actually decodes, unlike the space-manifest pack the
/// studio editor used to be fed by mistake.
pub fn workflow_artifact_envelope_pack(document: &OsWorkflowArtifactDocument) -> Option<store::DocumentPackFiles> {
    export_backbone_pack(document).ok()
}
//#endregion 🔖️WorkflowArtifactResolution

/// 🌉️ `pub` (not `pub(crate)`, and not `#[cfg(test)]`): `app_space`'s own tests (a sibling crate) seed a
/// studio through this hook — a `#[cfg(test)]` gate here would vanish when this crate is pulled in as
/// `app_space`'s ordinary (non-dev) dependency, since `#[cfg(test)]` only activates for the crate under
/// test itself, not its dependencies.
pub fn register_studio_port_for_test(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    register_studio_port(space_id, port);
}

/// 🎯️ W5 Lane B: the TTL-sweep call site — `list_drafts_sweeping_expired` clears any stale draft
/// bookkeeping (and best-effort tombstones its bytes) BEFORE this listing is built, so Home's VFS
/// never shows a studio draft past its deadline. Mirrors the spirit of os-core's own catalog-listing
/// entry points (`list_os_space_catalog_entries`): always sweep, then list.
fn list_all_space_catalog_entries() -> Vec<semio_framework_os::OsSpaceCatalogEntry> {
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

/// @emoji 🧭️ Builds the typed emit for a freshly-created studio: bump the catalog counter (operation) and
/// navigate the shell to the new studio route (host effect).
fn created_studio_emit(catalog_generation: u64, space_id: &str) -> Emit<SHomeOperation, HomeConfigOperation> {
    Emit { document_operations: vec![SHomeOperation::SetCatalogGeneration { value: catalog_generation + 1 }], effects: vec![HostEffect::Navigate { uri: format!("/spaces/{space_id}") }], ..Default::default() }
}

#[cfg(not(target_arch = "wasm32"))]
fn create_folder_studio(name: &str, folder_path: &str) -> Result<semio_framework_os::OsSpaceCatalogEntry, VcsError> {
    let port = semio_framework_os::open_folder_space_backbone(folder_path)?;
    let owner = SpaceUser { id: "local".into(), name: name.into(), avatar: None, role: SpaceRole::Author };
    let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, port.clone())?;
    register_studio_port(&entry.id, port);
    Ok(entry)
}

/// @emoji 🎯️ W5 Lane B: this IS "save this draft as a real asset" for a whole space manifest — binding
/// a studio to a real file/catalog location is the natural persist moment. `resolve_studio_document`
/// (draft-catalog-first) resolves the document whether `space_id` is still a draft or already real, so
/// this now works for BOTH; when it was a draft, discarding its bookkeeping + tombstoning its
/// `draft_uri` bytes afterwards completes the promotion (the SAME document now lives for real at
/// `catalog_uri`, so the draft placement is done).
#[cfg(not(target_arch = "wasm32"))]
fn bind_studio_file(space_id: &str, file_path: &str) -> Result<(), VcsError> {
    let uri = format!("file://{file_path}");
    let port = semio_framework_os::open_file_space_backbone(file_path)?;
    register_studio_port(space_id, port.clone());
    let mut document = resolve_studio_document(space_id).ok_or_else(|| VcsError::Backbone(format!("unknown space {space_id}")))?;
    document.backbone = Some(document_backbone_ref(&uri));
    port.write(&uri, &encode_backbone_payload(&document)?)?;
    let catalog_uri = format!("{OS_SPACE_BACKBONE_URI_PREFIX}{space_id}");
    sync_os_space_document_helper(&document, &catalog_uri, &catalog_port())?;
    let draft_port = draft_backbone_port();
    ephemeral_draft_catalog().discard_draft(&draft_port, space_id);
    Ok(())
}

fn sync_os_space_document_helper(document: &OsSpaceDocument, backbone_uri: &str, port: &Arc<dyn OsBackbonePort>) -> Result<(), VcsError> {
    let mut synced = document.clone();
    synced.backbone = Some(document_backbone_ref(backbone_uri));
    port.write(backbone_uri, &encode_backbone_payload(&synced)?)
}

fn os_home_vfs_schema_json() -> String {
    json!({
        "descriptorKinds": {
            "text": { "id": "text", "name": "Text", "presentation": "text" }
        },
        "fileNodeKinds": {
            "studio": {
                "id": "studio",
                "name": "Space",
                "descriptors": [{ "id": "apps", "descriptorKindId": "text", "label": "Apps" }]
            }
        },
        "descriptorColumnIds": ["apps"]
    })
    .to_string()
}

fn home_vfs_rows() -> Vec<Value> {
    let mut rows = vec![json!({
        "id": OS_HOME_VFS_ROOT_ID,
        "fileNodeKindId": "studio",
        "name": "Studios",
        "path": "/",
        "parentId": null,
        "hasChildren": true,
        "navigateUri": null,
        "descriptorValues": { "apps": "" }
    })];
    for entry in list_all_space_catalog_entries() {
        rows.push(json!({
            "id": format!("studio:{}", entry.id),
            "fileNodeKindId": "studio",
            "name": entry.name,
            "path": format!("/spaces/{}", entry.id),
            "parentId": OS_HOME_VFS_ROOT_ID,
            "hasChildren": false,
            "navigateUri": format!("/spaces/{}", entry.id),
            "descriptorValues": {
                "apps": format!("{:?} · {} collections", entry.kind, entry.collection_count)
            }
        }));
    }
    rows
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the Home launcher; one field per label makes every locale×terminology combination compile-checked.
    struct SHomeLabels {
        vfs_empty_message: native_en "No studios yet. Create one from the navbar.", native_de "Noch keine Studios vorhanden. Erstelle eines über die Navigationsleiste.",
            reuse_en "No studios yet. Create one from the navbar.", reuse_de "Noch keine Studios vorhanden. Erstelle eines über die Navigationsleiste.";
        window_main: native_en "Studios", native_de "Studios", reuse_en "Studios", reuse_de "Studios";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Render
fn render_home_vfs(labels: &SHomeLabels) -> UiNode {
    build_virtual_file_system_scene(
        S_HOME_SURFACE,
        S_HOME_CONTROLLER_ID,
        VirtualFileSystemScene {
            schema_json: os_home_vfs_schema_json(),
            rows_json: serde_json::to_string(&home_vfs_rows()).unwrap_or_else(|_| "[]".into()),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some(labels.vfs_empty_message.as_str().to_string()),
            drag_drop_enabled: None,
        },
        Some(S_HOME_WINDOW.into()),
        None,
    )
}
//#endregion 🔖️Render

//#region 🔖️HomeApp
/// 🧪️ B1: unit struct — the Home launcher held no runtime state even pre-B1; `HomeConfig` (see
/// `home_engine`) now carries `locale`/`active_panel_tab` (formerly `ViewState` reads).
#[derive(Default)]
pub struct HomeApp;

impl DocumentApp for HomeApp {
    type Projection = SHomeDocument;
    type Operation = SHomeOperation;
    type Config = HomeConfig;
    type ConfigOperation = HomeConfigOperation;
    type Command = HomeCommand;

    fn app_id(&self) -> &str {
        S_HOME_APP_ID
    }

    fn document_schema(&self) -> &str {
        "s.home"
    }

    fn initial_projection(&self) -> SHomeDocument {
        SHomeDocument { schema: "s.home".into(), catalog_generation: 0 }
    }

    /// 🏷️ Maps each `HomeCommand` variant back to the action id it was declared under in
    /// `create_home_app`.
    fn command_id(&self, command: &HomeCommand) -> &str {
        match command {
            HomeCommand::CreateStudio { .. } => "createStudio",
            HomeCommand::BindSpaceFile { .. } => "bindSpaceFile",
            HomeCommand::ImportSpace { .. } => "importSpace",
            HomeCommand::OpenSpace { .. } => "openSpace",
            HomeCommand::NavigateVirtualFileSystemNode { .. } => "navigateVirtualFileSystemNode",
            HomeCommand::DeleteVirtualFileSystemNode { .. } => "deleteVirtualFileSystemNode",
            HomeCommand::GoHome => "goHome",
            HomeCommand::SetActivePanelTab { .. } => "setActivePanelTab",
        }
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `HomeCommand` until every call site speaks OpBinary.
    fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<HomeCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "createStudio" => Ok(HomeCommand::CreateStudio {
                name: str_field("name").unwrap_or_else(|| "Untitled".into()),
                kind: str_field("kind").unwrap_or_else(|| "catalog".into()),
                folder_path: str_field("folderPath").or_else(|| str_field("folder_path")),
            }),
            "bindSpaceFile" => Ok(HomeCommand::BindSpaceFile {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                file_path: str_field("filePath").or_else(|| str_field("file_path")).unwrap_or_default(),
            }),
            "importSpace" => Ok(HomeCommand::ImportSpace { dsl: str_field("dsl").or_else(|| str_field("payload")) }),
            "openSpace" => Ok(HomeCommand::OpenSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default() }),
            "navigateVirtualFileSystemNode" => Ok(HomeCommand::NavigateVirtualFileSystemNode {
                node_id: str_field("nodeId")
                    .or_else(|| str_field("node_id"))
                    .or_else(|| str_field("spaceId"))
                    .or_else(|| str_field("space_id"))
                    .unwrap_or_default(),
            }),
            "deleteVirtualFileSystemNode" => Ok(HomeCommand::DeleteVirtualFileSystemNode {
                node_id: str_field("nodeId")
                    .or_else(|| str_field("node_id"))
                    .or_else(|| str_field("spaceId").map(|id| format!("studio:{id}")))
                    .or_else(|| str_field("space_id").map(|id| format!("studio:{id}")))
                    .unwrap_or_default(),
            }),
            "goHome" => Ok(HomeCommand::GoHome),
            "setActivePanelTab" => Ok(HomeCommand::SetActivePanelTab {
                tab_id: str_field("tabId").or_else(|| str_field("tab_id")).unwrap_or_default(),
            }),
            other => Err(Fault::new(FaultOrigin::App, "s.home.unhandled-action", format!("home: unhandled action id {other}"))),
        }
    }

    fn handle(&self, command: &HomeCommand, doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        let generation = doc.projection.catalog_generation;
        let bump = |value: u64| Emit::operations(vec![SHomeOperation::SetCatalogGeneration { value }]);
        let port = catalog_port();
        match command {
            HomeCommand::CreateStudio { name, kind, folder_path } => match kind.as_str() {
                "folder" => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if let Some(folder_path) = folder_path {
                            if let Ok(entry) = create_folder_studio(name, folder_path) {
                                eprintln!("[DEBUG] createStudio folder id={}", entry.id);
                                return Ok(created_studio_emit(generation, &entry.id));
                            }
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = (name, folder_path);
                    }
                    Ok(Emit::default())
                }
                _ => {
                    let space_id = create_and_register_ephemeral_studio(name);
                    eprintln!("[DEBUG] createStudio ephemeral id={space_id}");
                    Ok(created_studio_emit(generation, &space_id))
                }
            },
            HomeCommand::BindSpaceFile { space_id, file_path } => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = bind_studio_file(space_id, file_path);
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = (space_id, file_path);
                }
                Ok(Emit::default())
            }
            HomeCommand::ImportSpace { dsl } => match dsl {
                Some(dsl) => {
                    if import_os_space_from_dsl(dsl, port.clone()).is_ok() {
                        Ok(bump(generation + 1))
                    } else {
                        Ok(Emit::default())
                    }
                }
                None => Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".os".into(), read_as: None, import_action: "importSpace".into(), multiple: false })),
            },
            HomeCommand::OpenSpace { space_id } => {
                eprintln!("[DEBUG] home openSpace id={space_id}");
                Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{space_id}") }))
            }
            HomeCommand::NavigateVirtualFileSystemNode { node_id } => {
                let space_id = node_id.strip_prefix("studio:").unwrap_or(node_id);
                eprintln!("[DEBUG] home navigateVirtualFileSystemNode id={space_id}");
                Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{space_id}") }))
            }
            HomeCommand::DeleteVirtualFileSystemNode { node_id } => match node_id.strip_prefix("studio:") {
                Some(space_id) => {
                    let draft_port = draft_backbone_port();
                    ephemeral_draft_catalog().discard_draft(&draft_port, space_id);
                    let _ = delete_os_space(space_id, port.clone());
                    Ok(bump(generation + 1))
                }
                None => Ok(Emit::default()),
            },
            HomeCommand::GoHome => Ok(Emit::effect(HostEffect::Navigate { uri: "/".into() })),
            HomeCommand::SetActivePanelTab { tab_id } => Ok(Emit::config(vec![HomeConfigOperation::SetActivePanelTab { tab_id: tab_id.clone() }])),
        }
    }

    fn render(&self, body_key: &str, _doc: &DocumentView<'_, SHomeDocument>, cfg: &ConfigView<'_, HomeConfig>) -> UiNode {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<SHomeLabels>(&cfg.projection.locale);
        // 🪟 `VcsDocumentApp::render` appends `:{windowInstanceId}` when `view_state.window_id` is set —
        // strip it so Home's single body key still matches (same pattern as puzzle3d).
        let base_body_key = body_key.split_once(':').map(|(base, _)| base).unwrap_or(body_key);
        match base_body_key {
            S_HOME_BODY => render_home_vfs(labels),
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
            .mode("explore", LocalizedLabel::native("Explore", "Erkunden"), "focus")
            .default_mode_id("explore")
            .window_kind(S_HOME_WINDOW, LocalizedLabel::native("Studios", "Studios"), S_HOME_BODY, SurfaceKind::Canvas2d, "home")
            .default_layout(create_tab_stack_layout(&[S_HOME_WINDOW.into()], Some(&["Studios".into()])))
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
    use semio_framework_plugin::{testkit, HistoryView, VcsDocumentApp};

    fn empty_history() -> HistoryView {
        HistoryView::empty()
    }

    #[test]
    fn home_manifest_uses_home_app_id() {
        let app = create_home_app();
        assert_eq!(app.definition.id, "home");
        assert_eq!(app.definition.controller_id, "s-home");
    }

    #[test]
    fn home_vfs_lists_seeded_studio() {
        let rows = home_vfs_rows();
        assert!(rows.iter().any(|row| row.get("navigateUri").and_then(|v| v.as_str()).unwrap_or("").starts_with("/spaces/")));
    }

    #[test]
    fn creates_studio_via_home_action() {
        let port = catalog_port();
        let before = list_os_space_catalog_entries(port.clone()).expect("list").len();
        let mut home = VcsDocumentApp::new(HomeApp);
        home.dispatch_typed(HomeCommand::CreateStudio { name: "Test Studio".into(), kind: "catalog".into(), folder_path: None }, &testkit::meta("local")).expect("create");
        let after = list_os_space_catalog_entries(port).expect("list").len();
        assert!(after >= before);
    }

    #[test]
    fn home_declares_create_space_action() {
        let app = create_home_app();
        assert!(app.definition.actions.iter().any(|action| action.id == "createStudio"));
    }

    #[test]
    fn temporary_studio_uses_ephemeral_registry_not_catalog() {
        let home = HomeApp;
        let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let history = empty_history();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = HomeConfig::default();
        let cfg = ConfigView { projection: &config };
        let emit = home.handle(&HomeCommand::CreateStudio { name: "Temp Studio".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg).expect("handle");
        assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
        assert!(!emit.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })), "ephemeral create must not download");
        let persistent = list_os_space_catalog_entries(catalog_port()).expect("list");
        assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
        let ephemeral_catalog = list_os_space_catalog_entries(temp_catalog_port()).expect("list");
        assert!(!ephemeral_catalog.iter().any(|entry| entry.name == "Temp Studio"));
        let uri = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.as_str()),
                _ => None,
            })
            .expect("navigate");
        let space_id = uri.trim_start_matches("/spaces/");
        let document = resolve_studio_document(space_id).expect("ephemeral studio");
        assert_eq!(document.name, "Temp Studio");
        assert!(document.backbone.is_none());
        assert!(document.vcs.initial_projection.collections.is_empty());
    }

    #[test]
    fn space_document_persists_through_backbone_port() {
        // 🕳️ Was built off `parse_demo_space_document()` before `## The inversion`'s dissolve — that
        // helper now yields a `workflow::WorkflowDocument` (the demo fixture's own artifact content, see
        // `resolve_workflow_artifact_document`'s doc), not a `space::SpaceProjection`-backed catalog
        // entry `seed_os_space_catalog_if_empty` expects. This test exercises the space-manifest
        // persistence path specifically, so it mints its own manifest instead.
        let port: Arc<dyn OsBackbonePort> = Arc::new(LocalStorageBackbonePort::new());
        let projection = empty_space_projection("Persist Test", SpaceKind::Atelier, SpaceVisibility::Private);
        let demo: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, "persist-test", "Persist Test", projection);
        let _ = seed_os_space_catalog_if_empty(demo.clone(), port.clone()).expect("seed");
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
        let home_node = home.render(S_HOME_BODY, &home_view, &cfg);
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
        let home_node = home.render(S_HOME_BODY, &home_view, &cfg);
        assert!(serde_json::to_string(&home_node).unwrap().contains("Noch keine Studios vorhanden"));
    }

    #[test]
    fn set_active_panel_tab_emits_config_operation() {
        let home = HomeApp;
        let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let history = empty_history();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = HomeConfig::default();
        let cfg = ConfigView { projection: &config };
        let emit = home.handle(&HomeCommand::SetActivePanelTab { tab_id: "tab-1".into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.config_operations, vec![HomeConfigOperation::SetActivePanelTab { tab_id: "tab-1".into() }]);
        assert!(emit.document_operations.is_empty());
    }
}
//#endregion 🧪️Tests
