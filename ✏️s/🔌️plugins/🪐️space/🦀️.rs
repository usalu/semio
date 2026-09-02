//! 🌱️ S Studio plugin — fixtures + document helpers shared by the `home` editor/viewer surfaces AND the
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

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::artifacts::space::S_SPACE_INDEX_DOCUMENT_SCHEMA;
use semio_framework_os::{
    artifact_backbone_uri, collection_backbone_uri, create_backbone_document, decode_backbone_payload, draft_catalog_for, draft_uri, empty_space_snapshot, empty_workflow_snapshot, encode_backbone_payload, export_backbone_pack, export_os_space_pack,
    list_os_space_catalog_entries, load_os_space_document, materialize_backbone_snapshot, register_os_fixture_json, seed_os_space_catalog_if_empty, ArtifactBody, CollectionEntry, CollectionMutation, CollectionSnapshot, DraftCatalog,
    MemoryBackbonePort, OsBackbonePort, OsBackbonePorts, OsSpaceDocument, OsWorkflowArtifactDocument, SpaceBackbonePort, SpaceKind, SpaceMutation, SpaceRole, SpaceSnapshot, SpaceUser, SpaceVisibility, WorkflowMutation, WorkflowSnapshot, space_backbone_uri, OS_SPACE_SCHEMA,
    S_COLLECTION_SCHEMA, S_SPACE_SCHEMA, S_WORKFLOW_SCHEMA,
};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::{document_backbone_ref, VcsError};
use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{app_labels, ExecutionMode, Plugin, PluginApp};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use store::{BackbonePorts, LocalStorageBackbonePort};

//#region 🔖️Constants
pub const DEMO_STUDIO_ID: &str = "demo-studio";
pub const DEMO_STUDIO_NAME: &str = "Demo Studio";
/// 📜️ the demo studio is handcrafted `.s` DSL text (a `WorkflowSnapshot`, see `🔖️DocumentHelpers` —
/// the dissolved `OsProjection`'s successor), not JSON — it is compiled into the binary, so a parse
/// failure here is a bug in the bundled fixture.
pub const DEMO_STUDIO_DSL: &str = include_str!("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.space.studio.dsl.semio");
const OS_BOOT_STUDIO_ID: &str = "default";
//#endregion 🔖️Constants

//#region 🔖️Fixtures
/// 🧵️ Registers the draw/writer fixture documents referenced by the demo space's app instances —
/// shared by the Home editor's catalog seed and the Studio app's media export path, both of which need
/// these fixtures resolvable before they touch a studio document that references them.
pub async fn ensure_space_fixtures_registered() {
    static FIXTURES: LazyLock<()> = LazyLock::new(|| {
        // 🩹️ draw/writer migrated their fixtures from JSON to a handcrafted DSL (`store::ArtifactDsl`);
        // this registry is still JSON-shaped (framework/product/os hasn't migrated yet), so
        // `materialize_os_app_instance_document_json`'s `pack::from_json_str` will fall back to
        // `json!({})` for these two slugs until then. Non-fatal: seed content is a convenience default,
        // not required for correctness.
        register_os_fixture_json("🖍️semio.draw.json", include_str!("../🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));
        register_os_fixture_json("✒️jack.writer.json", include_str!("../✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));
    });
    let _ = &*FIXTURES;
}

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

/// @emoji 🌱️ The demo space's bare `WorkflowSnapshot` — the studio app's `initial_snapshot`, parsed
/// straight out of the packaged fixture (no envelope/runtime wrapper).
pub async fn demo_space_projection() -> WorkflowSnapshot {
    demo_os_document().await.vcs.initial_snapshot
}
//#endregion 🔖️Fixtures

//#region 🔖️DocumentHelpers
/// 🧬️ O1 — enum dispatch, not a trait object: os-host's own `OsBackbonePorts` (the enum its
/// `list_os_space_catalog_entries`/`seed_os_space_catalog_if_empty`/`load_os_space_document` are now
/// closed over, `Store(store::BackbonePorts) | Space(..)`) wraps the `store::BackbonePorts` enum this
/// function actually builds — no `dyn` anywhere, no separate trait-object "view" variable the way the
/// pre-O1 code kept one.
async fn catalog_port_concrete() -> Arc<OsBackbonePorts> {
    ensure_space_fixtures_registered().await;
    // 🧬️ `::default()`, not `::new()`: `LocalStorageBackbonePort::new()` is `async fn` but defined to
    // equal `Default::default()` exactly (store's own impl just forwards); using the sync constructor
    // here avoids a pointless suspension point and keeps this line symmetric with
    // `temp_catalog_port_concrete()` below, whose `OnceLock::get_or_init` closure cannot be async at all.
    // 🧬️ `OsBackbonePorts::Store(..)`: os-host's O1 enum-dispatch closed the catalog-facing fns
    // (`list_os_space_catalog_entries`/`seed_os_space_catalog_if_empty`) over its OWN `OsBackbonePorts`
    // enum, not `store::BackbonePorts` directly — every real transport still routes through the
    // `Store` variant's inner `store::BackbonePorts`.
    let port = Arc::new(OsBackbonePorts::Store(BackbonePorts::LocalStorage(LocalStorageBackbonePort::default())));
    if list_os_space_catalog_entries(port.clone()).map_or(true, |entries| entries.is_empty()) {
        // 🧬️ `parse_demo_space_document` yields a `WorkflowSnapshot` (the dissolved `OsProjection`'s
        // workflow-graph half) — the space CATALOG this boot seed populates needs a `SpaceSnapshot`
        // manifest instead. `demo_name` still comes from the bundled fixture's own name; the manifest
        // itself is a fresh space with no workflow artifact wired in yet (`create_os_space`'s own doc: a
        // space only auto-creates its default collection, never a workflow artifact — that stays a
        // later, explicit user action).
        let demo_name = {
            let demo = parse_demo_space_document().await;
            if demo.name.trim().is_empty() {
                "Demo Studio".into()
            } else {
                demo.name
            }
        };
        let mut projection = empty_space_snapshot(&demo_name, SpaceKind::Atelier, SpaceVisibility::Private);
        // 🪪️ Deliberately NOT threaded to a real session identity (unlike
        // `create_and_register_ephemeral_studio`'s `owner_id`/`owner_name`): this seed runs once, lazily,
        // from a process-global `static`/`LazyLock` at first catalog access, with no `HomeConfig`/
        // `ActionMeta` in scope — there is no user session to attribute this bootstrap fixture to.
        // `"local"` here names the pre-ticket guest sentinel, not a real signed-in user; fabricating one
        // would misattribute ownership of a system-seeded demo space.
        projection.users.push(SpaceUser { id: "local".into(), name: demo_name.clone(), avatar: None, role: SpaceRole::Author });
        let seed: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, OS_BOOT_STUDIO_ID, &demo_name, projection);
        let _ = seed_os_space_catalog_if_empty(seed, port.clone());
    }
    port
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
async fn shared_studio_ports() -> Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>> {
    static REGISTRY: OnceLock<Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Arc::new(Mutex::new(HashMap::new()))).clone()
}

/// 🌉️ The Home editor's `🎮️commands/*`, the Home viewer's read-only render, and the sibling `🪐️space`
/// studio app's own commands all resolve studios through this same catalog port.
pub async fn catalog_port() -> Arc<OsBackbonePorts> {
    catalog_port_concrete().await
}

pub(crate) async fn temp_catalog_port() -> Arc<OsBackbonePorts> {
    temp_catalog_port_concrete().await
}

/// 🔌️ `Arc<BackbonePorts>` — the concrete `store` enum, NOT `Arc<dyn SpaceBackbonePort>`. Every real
/// consumer of this return value — `draft_catalog_for`, `DraftCatalog::list_drafts_sweeping_expired`,
/// `DraftCatalog::discard_draft`, all declared in `🧰️framework/🔨️modules/🪐️space/🦀️.rs` (out
/// of this packet's owned path) — takes `&Arc<store::BackbonePorts>` directly; `SpaceBackbonePort`'s
/// blanket impl over `T: store::BackbonePort` covers this enum for free (`SpaceBackbonePort::read`/
/// `::write`, UFCS-disambiguated below against the sibling `OsBackbonePort` blanket).
pub(crate) async fn draft_backbone_port() -> Arc<BackbonePorts> {
    draft_backbone_port_concrete().await
}

/// 🗄️ The port-keyed `DraftCatalog` for `draft_backbone_port` — every draft studio's bookkeeping (id,
/// kind, TTL) lives here; `draft_catalog_for` guarantees the SAME instance is returned every call since
/// `draft_backbone_port` always clones the SAME `draft_backbone_port_concrete()` allocation.
pub(crate) async fn ephemeral_draft_catalog() -> Arc<DraftCatalog> {
    draft_catalog_for(&draft_backbone_port().await)
}

/// 🕰️ Wall-clock millis, reusing `store::now_iso`'s own wasm-safe implementation (its string is
/// already the millis count as text) rather than duplicating the `cfg(target_arch = "wasm32")`
/// branching this crate has no `js-sys` dependency to replicate directly.
async fn now_ms() -> u64 {
    store::now_iso().parse().unwrap_or(0)
}

pub(crate) async fn register_studio_port(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    if let Ok(mut guard) = shared_studio_ports().await.lock() {
        guard.insert(space_id.into(), port);
    }
}

/// @emoji 🆕️ Mints a fresh draft space manifest (empty, no collections) for the default create path — a
/// `SpaceSnapshot` document registered as a draft (`kind_id = "s.space"`) at `draft_uri(id)` on the
/// ephemeral port, never on the real catalog port, never tracked as a `space://` catalog entry.
/// `owner_id`/`owner_name` carry the real signed-in identity (ticket
/// 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS — `HomeConfig.client_id`/`client_name`,
/// contract §C3); empty strings fall back to the pre-ticket `"local"` guest identity, which is the
/// correct behavior when there is no signed-in session (no hub reachable) — the local-only path this
/// ticket's brief requires stays working unchanged in that case.
pub(crate) async fn create_and_register_ephemeral_studio(name: &str, owner_id: &str, owner_name: &str) -> String {
    let owner = SpaceUser { id: if owner_id.is_empty() { "local".into() } else { owner_id.into() }, name: if owner_name.is_empty() { name.into() } else { owner_name.into() }, avatar: None, role: SpaceRole::Author };
    let mut projection = empty_space_snapshot(name.trim(), SpaceKind::Atelier, SpaceVisibility::Private);
    projection.users.push(owner);
    let draft = ephemeral_draft_catalog().await.create_draft("s.space", S_SPACE_SCHEMA, name.trim(), now_ms().await, None);
    let document: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, &draft.artifact_id, name.trim(), projection);
    if let Ok(payload) = encode_backbone_payload(&document) {
        let draft_port = draft_backbone_port().await;
        let _ = SpaceBackbonePort::write(draft_port.as_ref(), &draft_uri(&draft.artifact_id), &payload);
    }
    draft.artifact_id
}

/// @emoji 📂️ Resolves a studio id against the draft catalog, registered ports, then catalogs.
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
        if let Ok(document) = load_os_space_document(space_id, port) {
            return Some(document);
        }
    }
    None
}

/// @emoji 📦️ Pack+spr bytes for `Effect::LoadDocument` / host `loadAppArtifactPack`.
pub async fn space_document_envelope_pack(document: &OsSpaceDocument) -> Option<store::ArtifactPackFiles> {
    export_os_space_pack(document).ok()
}

//#region 🔖️WorkflowArtifactResolution
/// 🕸️ "Space session -> active workflow artifact" resolution — a space manifest carries no graph of
/// its own anymore, the graph lives in a separate `s.workflow` artifact document addressed via a
/// `CollectionEntry` inside one of the space's collections. Searches every collection the resolved
/// space manifest references, through the SAME port search order `resolve_studio_document` uses, for
/// the first `CollectionEntry` whose body is an `s.workflow` document.
async fn resolve_backbone_bytes(uri: &str) -> Option<Vec<u8>> {
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

/// 🪆️ Reads a space's `s.space` artifact index (document id `index`, contract §C4) and decodes it —
/// `None` when no index document has been written yet (older spaces / test fixtures seeded before this
/// ticket), which is exactly the case `resolve_workflow_artifact_document` falls back on below.
async fn resolve_space_index_snapshot(space_id: &str) -> Option<SSpaceSnapshot> {
    let index_uri = artifact_backbone_uri(space_id, "index");
    let payload = resolve_backbone_bytes(&index_uri).await?;
    let index_document = decode_backbone_payload::<SSpaceSnapshot, SSpaceMutation>(&payload, S_SPACE_INDEX_DOCUMENT_SCHEMA).ok()?;
    materialize_backbone_snapshot(&index_document, &index_document.cursor.applied_edit_ids).ok()
}

/// 🕸️ "Space session -> active workflow artifact" resolution. Index-first (contract §C4: the space's
/// own `s.space` artifact index is the single source of truth for which artifacts live in a space) —
/// projects the index onto the framework's `os.collection` shape via `project_space_index_to_collection`
/// and walks ITS entries first. Falls back to the legacy direct `projection.collections` walk only when
/// no index document exists yet, so existing `⚙️engine` fixtures that seed a collection directly (never
/// an index) keep resolving exactly as before — never a silent behavior loss.
pub async fn resolve_workflow_artifact_document(space_id: &str, space_document: &OsSpaceDocument) -> Option<OsWorkflowArtifactDocument> {
    if let Some(index_snapshot) = resolve_space_index_snapshot(space_id).await {
        let collection_projection = project_space_index_to_collection(&index_snapshot).await;
        if let Some(workflow_snapshot) = find_workflow_snapshot_in_collection(space_id, &collection_projection).await {
            return Some(workflow_snapshot);
        }
    }
    let projection = materialize_backbone_snapshot(space_document, &space_document.cursor.applied_edit_ids).ok()?;
    for collection_ref in &projection.collections {
        let collection_uri = collection_backbone_uri(space_id, &collection_ref.id);
        let Some(collection_payload) = resolve_backbone_bytes(&collection_uri).await else { continue };
        let Ok(collection_document) = decode_backbone_payload::<CollectionSnapshot, CollectionMutation>(&collection_payload, S_COLLECTION_SCHEMA) else { continue };
        let Ok(collection_projection) = materialize_backbone_snapshot(&collection_document, &collection_document.cursor.applied_edit_ids) else { continue };
        if let Some(workflow_snapshot) = find_workflow_snapshot_in_collection(space_id, &collection_projection).await {
            return Some(workflow_snapshot);
        }
    }
    None
}

/// 🔎️ Shared entry-walk: the first `s.workflow`-schema'd entry whose backbone bytes decode cleanly.
async fn find_workflow_snapshot_in_collection(space_id: &str, collection_projection: &CollectionSnapshot) -> Option<OsWorkflowArtifactDocument> {
    for entry in &collection_projection.entries {
        let ArtifactBody::Document { schema, document_id } = entry.body.as_ref() else { continue };
        if schema != S_WORKFLOW_SCHEMA {
            continue;
        }
        let artifact_uri = artifact_backbone_uri(space_id, document_id);
        let Some(artifact_payload) = resolve_backbone_bytes(&artifact_uri).await else { continue };
        if let Ok(workflow_snapshot) = decode_backbone_payload::<WorkflowSnapshot, WorkflowMutation>(&artifact_payload, S_WORKFLOW_SCHEMA) {
            return Some(workflow_snapshot);
        }
    }
    None
}

//#region 🔖️SpaceIndexProjection
/// 🪞️ Projects the space's `s.space` artifact index onto the framework's `os.collection` shape — the
/// SAME `CollectionSnapshot` type `resolve_workflow_artifact_document`'s legacy walk already understood,
/// so the index becomes a drop-in single source of truth without widening either reader's contract.
/// Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4.
pub async fn project_space_index_to_collection(index: &SSpaceSnapshot) -> CollectionSnapshot {
    let entries = index
        .artifacts
        .iter()
        .map(|row| CollectionEntry { id: row.id.clone(), folder_id: None, name: row.name.clone(), kind_id: row.kind_id.clone(), body: Box::new(ArtifactBody::Document { schema: row.schema.clone(), document_id: row.id.clone() }) })
        .collect();
    CollectionSnapshot { schema: S_COLLECTION_SCHEMA.into(), name: index.space_id.clone(), folders: Vec::new(), entries }
}
//#endregion 🔖️SpaceIndexProjection

/// 🆕️ Mints a fresh, valid, empty `s.workflow` artifact document for a space that has none registered
/// yet — the "genuinely new/default space" leg of `resolve_workflow_artifact_document`'s three-way
/// fallback (existing registered artifact / demo fixture / fresh empty document). Not persisted as a
/// `CollectionEntry` (real artifact-registration UI is a later wave) — the studio editor still gets a
/// real, decodable `WorkflowSnapshot` pack instead of a broken placeholder, it just starts from a blank
/// canvas each time until persistence is wired.
pub async fn empty_workflow_artifact_document(space_id: &str, space_name: &str) -> OsWorkflowArtifactDocument {
    create_backbone_document(S_WORKFLOW_SCHEMA, space_id, space_name, empty_workflow_snapshot().await)
}

/// @emoji 📦️ `s.workflow` counterpart of `space_document_envelope_pack` — pack+spr bytes for
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
pub(crate) async fn sync_os_space_document_helper(document: &OsSpaceDocument, backbone_uri: &str, port: &Arc<BackbonePorts>) -> Result<(), VcsError> {
    let mut synced = document.clone();
    synced.backbone = Some(document_backbone_ref(backbone_uri));
    // 🧬️ UFCS, not `.write(..)`: `BackbonePorts` satisfies BOTH `OsBackbonePort` and `SpaceBackbonePort`
    // via their respective blanket impls over `store::BackbonePort` — both traits are `use`d in this
    // file, so a plain method call is ambiguous (E0034). Disambiguate to the same trait this fn's
    // pre-O1 signature used (`OsBackbonePort`).
    OsBackbonePort::write(port.as_ref(), backbone_uri, &encode_backbone_payload(&synced)?)
}

/// 🎯️ The TTL-sweep call site — `list_drafts_sweeping_expired` clears any stale draft bookkeeping (and
/// best-effort tombstones its bytes) BEFORE this listing is built, so Home's VFS never shows a studio
/// draft past its deadline. Mirrors the spirit of os-core's own catalog-listing entry points. `pub(crate)`
/// (not `pub`): only reached from within this crate (Home's editor/viewer main windows).
pub(crate) async fn list_all_space_catalog_entries() -> Vec<semio_framework_os::OsSpaceCatalogEntry> {
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    for port in [catalog_port().await, temp_catalog_port().await] {
        if let Ok(rows) = list_os_space_catalog_entries(port) {
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
        let projection = &document.vcs.initial_snapshot;
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
        origin_hub: native_en "hub", native_de "Hub", reuse_en "hub", reuse_de "Hub";
        origin_local: native_en "local", native_de "lokal", reuse_en "local", reuse_de "lokal";
    }
}

pub struct HomeSpaceRow {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub visibility: String,
    pub members: String,
    pub updated: String,
    pub origin: &'static str,
}

async fn directory_kind_str(kind: store::os_directory::DirectorySpaceKind) -> &'static str {
    match kind {
        store::os_directory::DirectorySpaceKind::Atelier => "atelier",
        store::os_directory::DirectorySpaceKind::Studio => "studio",
        store::os_directory::DirectorySpaceKind::Archive => "archive",
    }
}

async fn directory_visibility_str(visibility: store::os_directory::DirectorySpaceVisibility) -> &'static str {
    match visibility {
        store::os_directory::DirectorySpaceVisibility::Private => "private",
        store::os_directory::DirectorySpaceVisibility::Public => "public",
    }
}

async fn local_kind_str(kind: &SpaceKind) -> &'static str {
    match kind {
        SpaceKind::Atelier => "atelier",
        SpaceKind::Studio => "studio",
        SpaceKind::Archive => "archive",
    }
}

async fn local_visibility_str(visibility: &SpaceVisibility) -> &'static str {
    match visibility {
        SpaceVisibility::Private => "private",
        SpaceVisibility::Public => "public",
    }
}

/// 🪞️ Home table rows: every hub-directory space (`origin: "hub"`) UNIONED with the local-only catalog
/// (`origin: "local"`) — a hub row wins on an id collision (a space promoted from local to hub keeps
/// its hub-confirmed data, never a stale local shadow). Contract §C0 row-id grammar for the e2e is
/// `space:<id>`; callers building the table's `data-row-id` prepend that prefix to `HomeSpaceRow.id`.
pub async fn home_space_rows(directory: &store::os_directory::DirectoryReadModel) -> Vec<HomeSpaceRow> {
    let mut seen = HashSet::new();
    let mut rows = Vec::new();
    for (id, space) in &directory.spaces {
        seen.insert(id.clone());
        rows.push(HomeSpaceRow {
            id: id.clone(),
            name: space.view.name.clone(),
            kind: directory_kind_str(space.view.kind).await.into(),
            visibility: directory_visibility_str(space.view.visibility).await.into(),
            members: space.view.member_count.to_string(),
            updated: space.view.updated_at_ms.to_string(),
            origin: "hub",
        });
    }
    for entry in list_all_space_catalog_entries().await {
        if seen.contains(&entry.id) {
            continue;
        }
        rows.push(HomeSpaceRow {
            id: entry.id.clone(),
            name: entry.name.clone(),
            kind: local_kind_str(&entry.kind).await.into(),
            visibility: local_visibility_str(&entry.visibility).await.into(),
            // 🧑️ The local-only catalog carries no membership roster (single-user by construction);
            // "1" (the implicit owner) is the honest synthesis, not a directory-sourced count.
            members: "1".into(),
            updated: entry.updated_at.clone(),
            origin: "local",
        });
    }
    rows
}
//#endregion 🔖️HomeSpaceRows

//#region 🔌️Registration
/// 🗃️ Closed runtime app fleet for the home, space-index, and studio surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum SpaceApps: PluginApp {
        HomeEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::home::HomeApp>>),
        HomeViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::home::HomeViewer>>),
        SpaceIndexEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::space_index::SpaceIndexEditor>>),
        SpaceIndexViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::space_index::SpaceIndexViewer>>),
        Studio(semio_framework_plugin::VcsArtifactApp<crate::engine::space::SpaceApp>),
    }
}

/// 🔌️ Builds the S Studio plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) builds all artifact, app-schema, and codec
/// contributions as immutable data before the aggregate registration commit. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
/// M6-remaining, `📓️design-abi.md` §3/§6) are this crate's migration proof: one `OnArtifactKind`
/// event per owned kind (`home`/`space`, read live from each's own `artifact_kind().id`), `Isolated`
/// execution (grepped for `.handler(…)`, a `🧩️extensions/` dir, and self-tick loops — none found,
/// despite this crate having the heaviest `Effect` usage in the repo), and one `documents.write` ask
/// covering both editors' persisted mutations. No quota declared — no measured need found.
// 🚫️async: `plugin_exports!`'s `__semio_install_plugin_bundle` calls this from a bare `fn()`
// pointer slot (`PLUGIN_BUNDLE_INSTALLER: OnceLock<fn()>`), which can never itself be `async fn` —
// so this fn stays sync and bridges its still-async `home`/`engine` sub-calls with `resolve_ready`
// (in scope via `plugin_app_close_prelude::*` above), same poll-once guarantee as every other E5
// executor-bridge call site in this crate; the `space` artifact/editor/viewer calls below are
// already sync (no bridge needed).
pub fn plugin() -> Result<Plugin<SpaceApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<SpaceApps>::builder("s")
        .label("S Studio")
        .version("0.1.0")
        .local_backbone_storage()
        .artifact(resolve_ready(crate::artifacts::home::declaration()).map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::home::HomeApp>(resolve_ready(crate::editor::home::create_home_app()))
        .editor_mutation_roster::<crate::editor::home::HomeApp>()
        .viewer::<crate::viewer::home::HomeViewer>(resolve_ready(crate::viewer::home::create_home_viewer()))
        .viewer_mutation_roster::<crate::viewer::home::HomeViewer>()
        .artifact(crate::artifacts::space::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::space_index::SpaceIndexEditor>(crate::editor::space_index::create_space_index_editor())
        .editor_mutation_roster::<crate::editor::space_index::SpaceIndexEditor>()
        .viewer::<crate::viewer::space_index::SpaceIndexViewer>(crate::viewer::space_index::create_space_index_viewer())
        .viewer_mutation_roster::<crate::viewer::space_index::SpaceIndexViewer>()
        .document_app::<crate::engine::space::SpaceApp>(resolve_ready(crate::engine::space::create_space_app()))
        .foreign_document_codec::<crate::engine::space::SpaceApp>(OS_SPACE_SCHEMA)
        .activation(ActivationEvent::OnArtifactKind { kind: resolve_ready(crate::artifacts::home::artifact_kind()).id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::space::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist home/space-index edits to the open document".into(), optional: false })
        .try_build()
}
//#endregion 🔌️Registration

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — the real
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
    //! new_viewer}` (closed by w0-f, gap 2), used directly rather than local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn home_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::home::HomeViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn home_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::home::HomeApp, crate::viewer::home::HomeViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn space_index_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::space_index::SpaceIndexViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn space_index_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::space_index::SpaceIndexEditor, crate::viewer::space_index::SpaceIndexViewer>();
    }
}
//#endregion 🧪️SurfaceTests

//#region 🧪️SpaceIndexProjectionTests
#[cfg(test)]
mod space_index_projection_tests {
    use super::*;
    use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{empty_space_index_snapshot, SpaceArtifactDialect, SpaceArtifactRow};

    #[semio_framework_async_macros::async_test]
    async fn projects_every_row_into_a_root_level_collection_entry() {
        let mut index = empty_space_index_snapshot("space-1");
        index.artifacts.push(SpaceArtifactRow {
            id: "artifact-1".into(),
            name: "First".into(),
            kind_id: "space.sdraw".into(),
            schema: S_WORKFLOW_SCHEMA.into(),
            dialect: SpaceArtifactDialect { artifact_kind: "s.workflow".into(), standard: "1".into(), subset: "*".into() },
            created_at_ms: 1,
            created_by: "user:1".into(),
            updated_at_ms: 1,
            updated_by: "user:1".into(),
        });
        let collection = project_space_index_to_collection(&index).await;
        assert_eq!(collection.name, "space-1");
        assert_eq!(collection.entries.len(), 1);
        let entry = &collection.entries[0];
        assert_eq!(entry.id, "artifact-1");
        assert!(entry.folder_id.is_none());
        let ArtifactBody::Document { schema, document_id } = entry.body.as_ref() else { panic!("expected a document body") };
        assert_eq!(schema, S_WORKFLOW_SCHEMA);
        assert_eq!(document_id, "artifact-1");
    }
}
//#endregion 🧪️SpaceIndexProjectionTests
