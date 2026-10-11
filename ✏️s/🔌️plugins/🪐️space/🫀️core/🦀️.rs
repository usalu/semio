//! 🌱️ Shared Space runtime — fixtures + document helpers shared by the `home` editor/viewer surfaces AND the
//! `space` studio app. None of the three owns this content alone (see the master ticket's "shared code
//! used by ≥2 apps/surfaces of the plugin" rule), so it lives in this plugin-root `🫀️core` kernel
//! instead of duplicated into any of them.
//!
//! 🕳️ The `//#region 🔖️DocumentHelpers` block below (`catalog_port`, `resolve_studio_document`,
//! `list_all_space_catalog_entries`, …) moved here from `🗿️artifacts/🏠️home/…/✏️editor/🦀️.rs`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET W2 packet P7): once `🏠️home` split into an
//! `✏️editor` and a `👁️viewer`, this catalog-listing code became genuinely needed by THREE call sites
//! (the editor's own commands, the new viewer's read-only render, and studio's own `🎮️commands/*`) and a
//! viewer file can never import through `::editor::` (`policyViewerPurityBreaches`) — so plugin root,
//! reachable as `crate::X` from every module without any role prefix, is the only place all three can
//! reach it from. The vestigial `&HomeApp`/`_for` parameter the pre-split functions carried was dropped
//! in the move: every call site always passed `&HomeApp::default()`, so it never varied and coupling this
//! plugin-root file to `editor::home::HomeApp` for it would have bought nothing.

/// 🗺️ Shared lookup of admitted studio backbone ports.
type SharedStudioPorts = Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>;

use crate::time::{rfc3339_utc_epoch_ms, utc_minute_text};
use semio_framework_artifact_space_space::{empty_space_snapshot, space_backbone_uri, SpaceKind, SpaceMutation, SpaceRole, SpaceSnapshot, SpaceUser, SpaceVisibility, S_SPACE_SCHEMA};
use semio_framework_os::{
    create_backbone_document, decode_backbone_payload, draft_catalog_for, draft_uri, empty_workflow_snapshot, encode_backbone_payload, export_backbone_pack, export_os_space_pack, list_os_space_catalog_entries, load_os_space_document,
    materialize_backbone_snapshot, seed_os_space_catalog_if_empty, DraftCatalog, MemoryBackbonePort, OsBackbonePort, OsBackbonePorts, OsSpaceDocument, OsWorkflowArtifactDocument, SpaceBackbonePort,
    WorkflowMutation, WorkflowSnapshot, S_WORKFLOW_SCHEMA,
};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::{document_backbone_ref, VcsError};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_ui_locale::app_labels;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};
use store::{BackbonePorts, LocalStorageBackbonePort};

//#region 🔖️Constants
pub const DEMO_STUDIO_ID: &str = "demo-studio";
pub const DEMO_STUDIO_NAME: &str = "Demo Studio";
/// 📜️ the demo studio is handcrafted `.s` DSL text (a `WorkflowSnapshot`, see `🔖️DocumentHelpers` —
/// the dissolved `OsProjection`'s successor), not JSON — it is compiled into the binary, so a parse
/// failure here is a bug in the bundled fixture.
pub const DEMO_STUDIO_DSL: &str = include_str!("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️.semio");
const OS_BOOT_STUDIO_ID: &str = "default";
//#endregion 🔖️Constants

//#region 🔖️Fixtures

/// 🌱️ Parses the packaged demo studio fixture into a full `OsWorkflowArtifactDocument` envelope —
/// shared by the Home editor's catalog seed and the Studio app's `initial_snapshot`. The fixture
/// holds only the `WorkflowSnapshot` payload (`DEMO_STUDIO_DSL`); the envelope metadata
/// (schema/id/name, freshly-minted history) is built via `create_backbone_document`.
pub async fn parse_demo_space_document() -> OsWorkflowArtifactDocument {
    let initial_snapshot = <WorkflowSnapshot as store::ArtifactDsl>::parse_dsl(DEMO_STUDIO_DSL).expect("bundled example/✏️demo.s is valid WorkflowSnapshot DSL text");
    create_backbone_document(S_WORKFLOW_SCHEMA, DEMO_STUDIO_ID, DEMO_STUDIO_NAME, initial_snapshot)
}

pub async fn demo_os_document() -> OsWorkflowArtifactDocument {
    parse_demo_space_document().await
}

/// 🌱️ The demo space's bare `WorkflowSnapshot` — the studio app's `initial_snapshot`, parsed
/// straight out of the packaged fixture (no envelope/runtime wrapper).
pub async fn demo_space_projection() -> WorkflowSnapshot {
    demo_os_document().await.vcs.genesis.facts().snapshot().clone()
}
//#endregion 🔖️Fixtures

//#region 🔖️DocumentHelpers
/// 🧬️ The guest's ONE local studio catalog port, minted and seeded once per process. The catalog tracks its studio uris
/// per port identity (`list_os_space_catalog_entries` keys them by the `Arc`) and `LocalStorageBackbonePort` keeps its
/// fallback bytes per instance, so a port minted per call listed nothing a previous call admitted: a studio bound,
/// persisted or imported was gone on the next listing, and every call re-parsed and re-seeded the demo studio.
/// O1 — enum dispatch (`OsBackbonePorts::Store(store::BackbonePorts)`), no `dyn`. The demo seed is a `SpaceSnapshot`
/// manifest named after the bundled demo fixture and owned by the `"local"` guest sentinel: it runs before any user
/// session exists, so there is no signed-in identity to attribute it to.
/// @see ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📎️bind-space-file/🦀️.rs
async fn catalog_port_concrete() -> Arc<OsBackbonePorts> {
    static PORT: OnceLock<Arc<OsBackbonePorts>> = OnceLock::new();
    if let Some(port) = PORT.get() {
        return port.clone();
    }
    let port = Arc::new(OsBackbonePorts::Store(BackbonePorts::LocalStorage(LocalStorageBackbonePort::default())));
    if list_os_space_catalog_entries(&port).map_or(true, |entries| entries.is_empty()) {
        let demo = parse_demo_space_document().await;
        let demo_name = if demo.name.trim().is_empty() { "Demo Studio".to_owned() } else { demo.name };
        let mut projection = empty_space_snapshot(&demo_name, SpaceKind::Atelier, SpaceVisibility::Private);
        projection.users.push(SpaceUser { id: "local".into(), name: demo_name.clone(), avatar: None, role: SpaceRole::Author });
        let seed: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, OS_BOOT_STUDIO_ID, &demo_name, projection);
        let _ = seed_os_space_catalog_if_empty(seed, &port);
    }
    PORT.get_or_init(|| port).clone()
}

/// 🧬️ Session-local, ephemeral (in-memory only) counterpart to `catalog_port_concrete()`, used by the
/// os-catalog-facing fallback reads (`resolve_studio_document`/`resolve_backbone_bytes`/
/// `list_all_space_catalog_entries`) — draft bytes themselves are reached through the SEPARATE
/// `draft_backbone_port_concrete()` singleton below, not this one (see its own doc for why the two
/// can't share one allocation). Same `OsBackbonePorts` wrapping as `catalog_port_concrete()` —
/// `OnceLock::get_or_init`'s closure is plain `FnOnce`, not async, which is the other reason
/// `::default()` (sync) is used over `::new()`.
async fn temp_catalog_port_concrete() -> Arc<OsBackbonePorts> {
    static PORT: OnceLock<Arc<OsBackbonePorts>> = OnceLock::new();
    PORT.get_or_init(|| Arc::new(OsBackbonePorts::Store(BackbonePorts::Memory(MemoryBackbonePort::default())))).clone()
}

/// 🧬️ Independent in-memory singleton for draft byte storage — kept as a bare `Arc<store::BackbonePorts>`
/// (not `Arc<OsBackbonePorts>`) because `draft_catalog_for`/`DraftCatalog::list_drafts_sweeping_expired`/
/// `DraftCatalog::discard_draft` (framework/modules/space) predate `OsBackbonePorts` and can never depend
/// on it (os-host depends on space, not the other way; a back-dependency would cycle). `OsBackbonePorts::
/// Store` owns its inner `store::BackbonePorts` BY VALUE, not by `Arc`, so no wrapper can share this
/// allocation's identity with `temp_catalog_port_concrete()`'s own singleton above — kept deliberately
/// separate rather than faked. Every real caller reaches drafts through THIS port (`draft_uri`-prefixed
/// reads/writes); `temp_catalog_port()`'s fallback-loop reads never see draft entries anyway (they are
/// never `SPACE_CATALOG_URIS`-tracked), so the divergence is inert in practice.
async fn draft_backbone_port_concrete() -> Arc<BackbonePorts> {
    static PORT: OnceLock<Arc<BackbonePorts>> = OnceLock::new();
    PORT.get_or_init(|| Arc::new(BackbonePorts::Memory(MemoryBackbonePort::default()))).clone()
}

/// 🚧️ BLOCKER — this process-global mutable payload registry violates the retained-interaction
/// instance-ownership invariant and keeps Space global-state closure red. `register_studio_port`'s two real callers
/// (`create_folder_studio`/`bind_studio_file` in the Home editor's `create-studio`/`bind-space-file`
/// commands) source `port` from `semio_framework_os::open_folder_space_backbone`/
/// `open_file_space_backbone` — both declared in `🖥️host/🦀️.rs` (out of this packet's owned
/// path) as returning `Arc<dyn OsBackbonePort>` directly, already type-erased before this file ever
/// sees the value; there is no concrete type left to recover into a closed enum variant, and no `Any`
/// bound on `OsBackbonePort` to downcast through even if there were. The correct fix is a host-created,
/// instance-scoped port-catalog service threaded into Home and Studio operation context; moving the
/// same map behind another static would remain invalid. This source-only packet cannot install that
/// host seam, so every route traversing this registry remains fail-closed and the residue is reported.
async fn shared_studio_ports() -> SharedStudioPorts {
    static REGISTRY: OnceLock<SharedStudioPorts> = OnceLock::new();
    REGISTRY.get_or_init(|| Arc::new(Mutex::new(HashMap::new()))).clone()
}

/// 🌉️ The Home editor's `🎮️commands/*`, the Home viewer's read-only render, and the sibling `🪐️space`
/// studio app's own commands all resolve studios through this same catalog port.
pub async fn catalog_port() -> Arc<OsBackbonePorts> {
    catalog_port_concrete().await
}

pub async fn temp_catalog_port() -> Arc<OsBackbonePorts> {
    temp_catalog_port_concrete().await
}

/// 🔌️ `Arc<BackbonePorts>` — the concrete `store` enum, NOT `Arc<dyn SpaceBackbonePort>`. Every real
/// consumer of this return value — `draft_catalog_for`, `DraftCatalog::list_drafts_sweeping_expired`,
/// `DraftCatalog::discard_draft`, all declared in `🧰️framework/🔨️modules/🪐️space/🦀️.rs` (out
/// of this packet's owned path) — takes `&Arc<store::BackbonePorts>` directly; `SpaceBackbonePort`'s
/// blanket impl over `T: store::BackbonePort` covers this enum for free (`SpaceBackbonePort::read`/
/// `::write`, UFCS-disambiguated below against the sibling `OsBackbonePort` blanket).
pub async fn draft_backbone_port() -> Arc<BackbonePorts> {
    draft_backbone_port_concrete().await
}

/// 🗄️ The port-keyed `DraftCatalog` for `draft_backbone_port` — every draft studio's bookkeeping (id,
/// kind, TTL) lives here; `draft_catalog_for` guarantees the SAME instance is returned every call since
/// `draft_backbone_port` always clones the SAME `draft_backbone_port_concrete()` allocation.
pub async fn ephemeral_draft_catalog() -> Arc<DraftCatalog> {
    draft_catalog_for(&draft_backbone_port().await)
}

/// 🕰️ Wall-clock millis, reusing `store::now_iso`'s own wasm-safe implementation (its string is
/// already the millis count as text) rather than duplicating the `cfg(target_arch = "wasm32")`
/// branching this crate has no `js-sys` dependency to replicate directly.
async fn now_ms() -> u64 {
    store::now_iso().parse().unwrap_or(0)
}

pub async fn register_studio_port(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    if let Ok(mut guard) = shared_studio_ports().await.lock() {
        guard.insert(space_id.into(), port);
    }
}

/// 🆕️ Mints a fresh draft space manifest (empty, no collections) for the default create path — a
/// `SpaceSnapshot` document registered as a draft (`kind_id = "s.space"`) at `draft_uri(id)` on the
/// ephemeral port, never on the real catalog port, never tracked as a `space://` catalog entry.
/// `owner_id`/`owner_name` carry the signed-in identity selected by the caller's current host view.
pub async fn create_and_register_ephemeral_studio(name: &str, owner_id: &str, owner_name: &str, identity: &mut store::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, store::VcsError> {
    let draft = ephemeral_draft_catalog().await.create_draft("s.space", S_SPACE_SCHEMA, name.trim(), now_ms().await, None, identity)?;
    let owner = SpaceUser { id: if owner_id.is_empty() { "local".into() } else { owner_id.into() }, name: if owner_name.is_empty() { name.into() } else { owner_name.into() }, avatar: None, role: SpaceRole::Author };
    let mut projection = empty_space_snapshot(name.trim(), SpaceKind::Atelier, SpaceVisibility::Private);
    projection.users.push(owner);
    let document: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, &draft.artifact_id, name.trim(), projection);
    let draft_port = draft_backbone_port().await;
    let result = encode_backbone_payload(&document).and_then(|payload| SpaceBackbonePort::write(draft_port.as_ref(), &draft_uri(&draft.artifact_id), &payload));
    if let Err(error) = result {
        ephemeral_draft_catalog().await.discard_draft(&draft_port, &draft.artifact_id);
        return Err(error);
    }
    Ok(draft.artifact_id)
}

/// 📂️ Resolves a studio id against the draft catalog, registered ports, then catalogs.
pub async fn resolve_studio_document(space_id: &str) -> Option<OsSpaceDocument> {
    let draft_port = draft_backbone_port().await;
    if let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), &draft_uri(space_id)) {
        if !payload.is_empty() {
            if let Ok(document) = decode_backbone_payload::<SpaceSnapshot, SpaceMutation>(&payload, S_SPACE_SCHEMA) {
                return Some(document);
            }
        }
    }
    if let Ok(guard) = shared_studio_ports().await.lock() {
        if let Some(port) = guard.get(space_id) {
            // 🚧️ Same registry blocker as `shared_studio_ports`'s own doc comment: its values are
            // `Arc<dyn OsBackbonePort>`, which cannot recover into the closed `Arc<OsBackbonePorts>`
            // `load_os_space_document` now requires (O1 enum dispatch, no `Any` downcast available) —
            // so this branch reads the manifest bytes straight off the dyn port instead of routing
            // through that helper, matching what `load_os_space_document` does internally.
            if let Ok(payload) = port.read(&space_backbone_uri(space_id)) {
                if !payload.is_empty() {
                    if let Ok(document) = decode_backbone_payload::<SpaceSnapshot, SpaceMutation>(&payload, S_SPACE_SCHEMA) {
                        return Some(document);
                    }
                }
            }
        }
    }
    for port in [temp_catalog_port().await, catalog_port().await] {
        if let Ok(document) = load_os_space_document(space_id, &port) {
            return Some(document);
        }
    }
    None
}

/// 📦️ Pack+spr bytes for `Effect::LoadDocument` / host `loadAppArtifactPack`.
pub async fn space_document_envelope_pack(document: &OsSpaceDocument) -> Option<store::ArtifactPackFiles> {
    export_os_space_pack(document).ok()
}

//#region 🔖️WorkflowArtifactResolution
/// 🕸️ "Space session -> active workflow artifact" resolution — a space manifest carries no graph of
/// its own anymore, the graph lives in a separate `s.workflow` artifact document addressed via a
/// `CollectionEntry` inside one of the space's collections. Searches every collection the resolved
/// space manifest references, through the SAME port search order `resolve_studio_document` uses, for
/// the first `CollectionEntry` whose body is an `s.workflow` document.
pub async fn resolve_backbone_bytes(uri: &str) -> Option<Vec<u8>> {
    let draft_port = draft_backbone_port().await;
    if let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), uri) {
        if !payload.is_empty() {
            return Some(payload);
        }
    }
    if let Ok(guard) = shared_studio_ports().await.lock() {
        for port in guard.values() {
            if let Ok(payload) = port.read(uri) {
                if !payload.is_empty() {
                    return Some(payload);
                }
            }
        }
    }
    for port in [temp_catalog_port().await, catalog_port().await] {
        // 🧬️ `port` is `Arc<OsBackbonePorts>`; the enum's own `impl OsBackbonePort for OsBackbonePorts`
        // (not the `store::BackbonePort` blanket) is the only trait it satisfies, so UFCS needs the
        // `&OsBackbonePorts` the `Arc` derefs to, not the `Arc` itself.
        if let Ok(payload) = OsBackbonePort::read(port.as_ref(), uri) {
            if !payload.is_empty() {
                return Some(payload);
            }
        }
    }
    None
}

/// 🆕️ Mints a fresh, valid, empty `s.workflow` artifact document for a space that has none registered
/// yet — the "genuinely new/default space" leg of `resolve_workflow_artifact_document`'s three-way
/// fallback (existing registered artifact / demo fixture / fresh empty document). Not persisted as a
/// `CollectionEntry` (real artifact-registration UI is a later wave) — the studio editor still gets a
/// real, decodable `WorkflowSnapshot` pack instead of a broken placeholder, it just starts from a blank
/// canvas each time until persistence is wired.
pub async fn empty_workflow_artifact_document(space_id: &str, space_name: &str) -> OsWorkflowArtifactDocument {
    create_backbone_document(S_WORKFLOW_SCHEMA, space_id, space_name, empty_workflow_snapshot().await)
}

/// 📦️ `s.workflow` counterpart of `space_document_envelope_pack` — pack+spr bytes for
/// `Effect::LoadDocument` / host `loadAppArtifactPack`, sized to what the `🪐️space` studio app's
/// `ArtifactApp::Snapshot` (`WorkflowSnapshot`) actually decodes.
pub async fn workflow_artifact_envelope_pack(document: &OsWorkflowArtifactDocument) -> Option<store::ArtifactPackFiles> {
    export_backbone_pack(document).ok()
}
//#endregion 🔖️WorkflowArtifactResolution

/// 🌉️ Not `#[cfg(test)]`: the sibling `🪐️space` studio app's own tests seed a studio through this hook
/// — a `#[cfg(test)]` gate here would vanish when this module is pulled in as `engine::space`'s ordinary
/// (non-dev) dependency, since `#[cfg(test)]` only activates for the crate under test itself, not its
/// dependencies.
pub async fn register_studio_port_for_test(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    register_studio_port(space_id, port).await;
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn sync_os_space_document_helper(document: &OsSpaceDocument, backbone_uri: &str, port: &Arc<OsBackbonePorts>) -> Result<(), VcsError> {
    let mut synced = document.clone();
    synced.backbone = Some(document_backbone_ref(backbone_uri).await);
    OsBackbonePort::write(port.as_ref(), backbone_uri, &encode_backbone_payload(&synced)?)
}

/// 🎯️ The TTL-sweep call site — `list_drafts_sweeping_expired` clears any stale draft bookkeeping (and
/// best-effort tombstones its bytes) BEFORE this listing is built, so Home's VFS never shows a studio
/// draft past its deadline. Mirrors the spirit of os-core's own catalog-listing entry points. `pub`
/// (not `pub`): only reached from within this crate (Home's editor/viewer main windows).
pub async fn list_all_space_catalog_entries() -> Vec<semio_framework_os::OsSpaceCatalogEntry> {
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    for port in [catalog_port().await, temp_catalog_port().await] {
        if let Ok(rows) = list_os_space_catalog_entries(&port) {
            for entry in rows {
                if seen.insert(entry.id.clone()) {
                    entries.push(entry);
                }
            }
        }
    }
    let draft_port = draft_backbone_port().await;
    for draft in ephemeral_draft_catalog().await.list_drafts_sweeping_expired(now_ms().await, &draft_port) {
        if draft.kind_id != "s.space" || !seen.insert(draft.artifact_id.clone()) {
            continue;
        }
        let Ok(payload) = SpaceBackbonePort::read(draft_port.as_ref(), &draft_uri(&draft.artifact_id)) else { continue };
        if payload.is_empty() {
            continue;
        }
        let Ok(document) = decode_backbone_payload::<SpaceSnapshot, SpaceMutation>(&payload, S_SPACE_SCHEMA) else { continue };
        let projection = &document.vcs.genesis.facts().snapshot();
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

//#region 🔖️HomeSpaceRows
/// 🪪️ Returns the current host-owned session identity only when both exact fields satisfy the shared
/// view-context identifier contract.
pub fn home_session_identity(view: &semio_framework_plugin::ViewModel) -> Option<&semio_framework_plugin::ViewSessionIdentity> {
    let identity = view.session_identity.as_ref()?;
    view_session_identity_valid(identity).then_some(identity)
}

/// 🪪️ Applies the shared view-context bounds to an already selected host session identity.
pub fn view_session_identity_valid(identity: &semio_framework_plugin::ViewSessionIdentity) -> bool {
    let admitted = |value: &str| !value.is_empty() && value.chars().count() <= semio_framework::VIEW_CONTEXT_IDENTIFIER_CHARS && !value.chars().any(|character| character.is_control());
    admitted(&identity.user_id) && admitted(&identity.display_name)
}

// 🏠️ One row of the Home overview table — ticket
// 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: replaces the pre-ticket virtual-file-
// system scene with a real table of every space, fed by the event-sourced hub directory read model
// UNIONED with the local-only catalog. Lives at plugin root (not `editor::home`) for the same reason
// `list_all_space_catalog_entries` does: the Home viewer renders the SAME rows and a viewer file can
// never import through `::editor::` (`policyViewerPurityBreaches`).
app_labels! {
    /// 🗣️ Table strings shared by the Home editor's AND viewer's main-window render (both surfaces
    /// render the same 7-column table) — lives here, not in `editor::home::terminology::SHomeLabels`,
    /// for the same reason `home_space_rows` does: a viewer file can never import through `::editor::`.
    pub struct HomeTableLabels {
        empty_message: native_en "No studios yet. Create one from the navbar.", native_de "Noch keine Studios vorhanden. Erstelle eines über die Navigationsleiste.",
            reuse_en "No studios yet. Create one from the navbar.", reuse_de "Noch keine Studios vorhanden. Erstelle eines über die Navigationsleiste.";
        column_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        column_kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        column_visibility: native_en "Visibility", native_de "Sichtbarkeit", reuse_en "Visibility", reuse_de "Sichtbarkeit";
        column_members: native_en "Members", native_de "Mitglieder", reuse_en "Members", reuse_de "Mitglieder";
        column_updated: native_en "Updated", native_de "Aktualisiert", reuse_en "Updated", reuse_de "Aktualisiert";
        column_origin: native_en "Origin", native_de "Herkunft", reuse_en "Origin", reuse_de "Herkunft";
        column_actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        table_name: native_en "Studios", native_de "Studios", reuse_en "Studios", reuse_de "Studios";
        origin_hub: native_en "hub", native_de "Hub", reuse_en "hub", reuse_de "Hub";
        origin_local: native_en "local", native_de "lokal", reuse_en "local", reuse_de "lokal";
        kind_atelier: native_en "Atelier", native_de "Atelier", reuse_en "Atelier", reuse_de "Atelier";
        kind_studio: native_en "Studio", native_de "Studio", reuse_en "Studio", reuse_de "Studio";
        kind_archive: native_en "Archive", native_de "Archiv", reuse_en "Archive", reuse_de "Archiv";
        visibility_private: native_en "private", native_de "privat", reuse_en "private", reuse_de "privat";
        visibility_public: native_en "public", native_de "öffentlich", reuse_en "public", reuse_de "öffentlich";
        updated_never: native_en "never saved", native_de "nie gespeichert", reuse_en "never saved", reuse_de "nie gespeichert";
    }
}

impl HomeTableLabels {
    /// 🏷️ The viewer-language word for a space kind.
    pub fn kind(&self, kind: SpaceKind) -> &str {
        match kind {
            SpaceKind::Atelier => self.kind_atelier.as_str(),
            SpaceKind::Studio => self.kind_studio.as_str(),
            SpaceKind::Archive => self.kind_archive.as_str(),
        }
    }

    /// 🏷️ The viewer-language word for a space visibility.
    pub fn visibility(&self, visibility: SpaceVisibility) -> &str {
        match visibility {
            SpaceVisibility::Private => self.visibility_private.as_str(),
            SpaceVisibility::Public => self.visibility_public.as_str(),
        }
    }

    /// 🕰️ When a row last changed, in the viewer's language — see [`utc_minute_text`]; `never saved` for a
    /// local draft that was never saved.
    pub fn updated(&self, updated_ms: Option<u64>) -> String {
        updated_ms.map_or_else(|| self.updated_never.as_str().to_owned(), |ms| utc_minute_text(ms, self.locale))
    }
}


pub struct HomeSpaceRow {
    pub id: String,
    pub name: String,
    pub kind: SpaceKind,
    pub visibility: SpaceVisibility,
    pub members: String,
    /// 🕰️ When the row last changed (hub-confirmed or last local save); `None` for a never-saved draft.
    pub updated_ms: Option<u64>,
    pub origin: &'static str,
    /// 📂️ Persistence data class — hub=persistedShared; local catalog=persistedLocalOnly;
    /// ephemeral draft studios (empty backbone_uri)=ephemeralLocalOnly.
    pub data_class: &'static str,
    /// 🛂️ The CALLING client's current membership role in this space, as folded from hub-confirmed
    /// directory events — `None` for a space the caller is not a member of (a public row) and for
    /// every local-only catalog row. The Home renderer hides author-only affordances on this and
    /// never on `origin`; the authoritative capability still comes from the administration page.
    pub role: Option<DirectorySpaceRole>,
}

/// 🛂️ The caller's role in one folded space, or `None` when they are not a current member.
pub use store::os_directory::DirectorySpaceRole;

fn caller_role(space: &store::os_directory::DirectorySpace, user_id: &str) -> Option<DirectorySpaceRole> {
    if user_id.is_empty() {
        return None;
    }
    space.members.iter().find(|member| member.user_id == user_id).map(|member| member.role)
}

fn directory_kind(kind: store::os_directory::DirectorySpaceKind) -> SpaceKind {
    match kind {
        store::os_directory::DirectorySpaceKind::Atelier => SpaceKind::Atelier,
        store::os_directory::DirectorySpaceKind::Studio => SpaceKind::Studio,
        store::os_directory::DirectorySpaceKind::Archive => SpaceKind::Archive,
    }
}

fn directory_visibility(visibility: store::os_directory::DirectorySpaceVisibility) -> SpaceVisibility {
    match visibility {
        store::os_directory::DirectorySpaceVisibility::Private => SpaceVisibility::Private,
        store::os_directory::DirectorySpaceVisibility::Public => SpaceVisibility::Public,
    }
}

impl HomeSpaceRow {
    /// 📊️ The row's six cells in `labels`' language, positional to the Home table's columns — shared by the
    /// Home editor's and viewer's main windows.
    pub fn cells(&self, labels: &HomeTableLabels) -> [String; 6] {
        let origin = if self.origin == "hub" { labels.origin_hub.as_str() } else { labels.origin_local.as_str() };
        [self.name.clone(), labels.kind(self.kind).to_owned(), labels.visibility(self.visibility).to_owned(), self.members.clone(), labels.updated(self.updated_ms), origin.to_owned()]
    }
}

/// 🪞️ Home table rows: every hub-directory space (`origin: "hub"`) UNIONED with the local-only catalog
/// (`origin: "local"`) — a hub row wins on an id collision (a space promoted from local to hub keeps
/// its hub-confirmed data, never a stale local shadow). Contract §C0 row-id grammar for the e2e is
/// `space:<id>`; callers building the table's `data-row-id` prepend that prefix to `HomeSpaceRow.id`.
/// `retired_local_studio_ids` (sorted) are the Home config's tombstones: a retired local studio keeps its catalog
/// document and is simply not listed. `hub_spaces` are the folded directory's rows in id order.
pub async fn home_space_rows<'a>(hub_spaces: impl IntoIterator<Item = &'a store::os_directory::DirectorySpace>, user_id: &str, retired_local_studio_ids: &[String]) -> Vec<HomeSpaceRow> {
    let mut seen = HashSet::new();
    let mut rows = Vec::new();
    for space in hub_spaces {
        let id = &space.view.id;
        seen.insert(id.clone());
        rows.push(HomeSpaceRow {
            id: id.clone(),
            name: space.view.name.clone(),
            kind: directory_kind(space.view.kind),
            visibility: directory_visibility(space.view.visibility),
            members: space.view.member_count.to_string(),
            updated_ms: u64::try_from(space.view.updated_at_ms).ok(),
            origin: "hub",
            data_class: "persistedShared",
            role: caller_role(space, user_id),
        });
    }
    for entry in list_all_space_catalog_entries().await {
        if seen.contains(&entry.id) || retired_local_studio_ids.binary_search(&entry.id).is_ok() {
            continue;
        }
        rows.push(HomeSpaceRow {
            id: entry.id.clone(),
            name: entry.name.clone(),
            kind: entry.kind,
            visibility: entry.visibility,
            // 🧑️ The local-only catalog carries no membership roster (single-user by construction);
            // "1" (the implicit owner) is the honest synthesis, not a directory-sourced count.
            members: "1".into(),
            updated_ms: rfc3339_utc_epoch_ms(&entry.updated_at),
            origin: "local",
            data_class: if entry.backbone_uri.is_empty() { "ephemeralLocalOnly" } else { "persistedLocalOnly" },
            // 🏠️ The local-only catalog is single-user by construction and carries no directory
            // membership; a local row therefore never offers a directory-owned affordance.
            role: None,
        });
    }
    rows
}
//#endregion 🔖️HomeSpaceRows

//#region 🧵️RetainedStore

fn space_retained_mutation_bytes<M: ::protocol::OpBinary>(mutation: &M) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "s.space.retained.mutation-encode".to_string())
}

fn admit_space_retained_mutation<P, M: ::protocol::OpBinary + ::protocol::Mutation<P>>(mutation: &M, maximum_bytes: usize) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = space_retained_mutation_bytes(mutation)?;
    if retained_bytes > maximum_bytes {
        return Err("s.space.retained.mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, retained_bytes))
}

fn prepare_space_retained_one_item<P, M>(base: &P, mutation: &M, maximum_bytes: usize) -> Result<(P, Vec<M>), semio_framework_value::ValueError>
where
    M: ::protocol::Mutation<P> + ::protocol::OpBinary,
{
    admit_space_retained_mutation::<P, M>(mutation, maximum_bytes).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "s.space.retained.mutation-envelope"))?;
    let inverse = ::protocol::Mutation::inverse(mutation, base)?;
    let diff = ::protocol::Mutation::diff(mutation, base).into_parts().0;
    let post = ::protocol::apply_diff(&diff, base).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "s.space.retained.diff-apply"))?;
    Ok((post, inverse))
}

fn space_refusal(message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)
}

/// 🏭️ The exact one-item Store preparation authority every migrated `🪐️space` tool needs: a
/// publication lane a tool declares is refused at app construction
/// (`interactive-job.publication-contract`) unless its lane factory exists.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SpaceOneItemPreparationFactory<P, M> {
    prefix: &'static str,
    maximum_bytes: usize,
    lane: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> SpaceOneItemPreparationFactory<P, M> {
    pub const fn new(prefix: &'static str, maximum_bytes: usize) -> Self {
        Self { prefix, maximum_bytes, lane: std::marker::PhantomData }
    }
}

struct SpaceOneItemPreparation<P: 'static, M: 'static> {
    owners: store::OneItemOwners<P, M>,
    maximum_bytes: usize,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for SpaceOneItemPreparationFactory<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: ::protocol::Mutation<P> + ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree + Send + Sync + 'static,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<::protocol::Edit<M>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &M, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("s.space.retained.lane".into());
        }
        admit_space_retained_mutation::<P, M>(mutation, self.maximum_bytes)
    }

    fn begin_demand(&self, _mutation: &M, lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::HistoryLane::Document {
            return Err(space_refusal("s.space.retained.lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<SpaceOneItemPreparation<P, M>>(), depth: 1 })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        let retained_bytes = space_retained_mutation_bytes(&request.mutation).unwrap_or(self.maximum_bytes.saturating_add(1));
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > self.maximum_bytes
        {
            return Err((space_refusal("s.space.retained.request-refused"), request));
        }
        Ok((Box::new(SpaceOneItemPreparation { owners: store::OneItemOwners::from_request(request), maximum_bytes: self.maximum_bytes, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes, cancelled: false }), progress))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for SpaceOneItemPreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: ::protocol::Mutation<P> + ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + 'static,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(space_refusal("s.space.retained.original-refusal-retained"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let base = self.owners.base.as_ref().ok_or_else(|| space_refusal("s.space.retained.base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| space_refusal("s.space.retained.mutation-owner-missing"))?;
        let (post, inverse) = prepare_space_retained_one_item(base.get(), mutation, self.maximum_bytes)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| space_refusal("s.space.retained.authority-missing"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| space_refusal("s.space.retained.mutation-owner-missing"))?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: self.retained_bytes, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

/// 📬️ One lane's `Artifact`/`Config` store override, addressed by its edit-id prefix and byte ceiling.
pub fn space_retained_store_preparation<P, M>(prefix: &'static str, maximum_bytes: usize) -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<P, M>>>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: ::protocol::Mutation<P> + ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree + Send + Sync + 'static,
{
    Some(Arc::new(SpaceOneItemPreparationFactory::<P, M>::new(prefix, maximum_bytes)))
}
//#endregion 🧵️RetainedStore
