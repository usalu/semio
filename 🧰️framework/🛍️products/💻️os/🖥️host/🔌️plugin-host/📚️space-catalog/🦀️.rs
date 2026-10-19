//! 📚️ Space catalog of the plugin host: the space catalog entries and their backbone URI tracking, creation, deletion, import, export, loading and seeding. See [`super`].

use super::*;

// #region host
//#region 🔖️SpaceCatalog
pub const OS_HOME_VFS_ROOT_ID: &str = "os-home-root";

pub const OS_SPACE_BACKBONE_URI_PREFIX: &str = "space://";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsSpaceCatalogEntry {
    pub id: String,
    pub name: String,
    pub backbone_uri: String,
    pub kind: space::SpaceKind,
    pub visibility: space::SpaceVisibility,
    pub collection_count: usize,
    pub updated_at: String,
}

static SPACE_CATALOG_URIS: LazyLock<Mutex<HashMap<usize, HashSet<String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn port_key(port: &Arc<OsBackbonePorts>) -> usize {
    Arc::as_ptr(port) as *const () as usize
}

fn track_os_space_backbone_uri(port: &Arc<OsBackbonePorts>, uri: &str) {
    SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(port_key(port)).or_default().insert(uri.into());
}

fn untrack_os_space_backbone_uri(port: &Arc<OsBackbonePorts>, uri: &str) {
    if let Some(uris) = SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get_mut(&port_key(port)) {
        uris.remove(uri);
    }
}

/// 🔎️ Extracts a bare space id from a `space://<id>` uri — deliberately excludes the
/// `space://<id>/collection/<cid>` form (`space::collection_backbone_uri`), which the catalog
/// tracks alongside a space's manifest uri but must not surface as a catalog row of its own.
fn os_space_id_from_backbone_uri(uri: &str) -> Option<String> {
    let id = uri.strip_prefix(OS_SPACE_BACKBONE_URI_PREFIX)?;
    if id.is_empty() || id.contains('/') {
        return None;
    }
    Some(id.to_string())
}

fn os_space_catalog_entry_from_document(backbone_uri: &str, document: &OsSpaceDocument) -> Result<OsSpaceCatalogEntry, VcsError> {
    let space_id = os_space_id_from_backbone_uri(backbone_uri).unwrap_or_else(|| document.id.clone());
    let snapshot = materialize_backbone_snapshot(document, &[])?;
    let updated_at = document.vcs.changes.last().map_or_else(|| "0".into(), |change| change.saved_at.clone());
    Ok(OsSpaceCatalogEntry { id: space_id, name: document.name.clone(), backbone_uri: backbone_uri.into(), kind: snapshot.kind, visibility: snapshot.visibility, collection_count: snapshot.collections.len(), updated_at })
}

/// 📚️ Lists persisted space manifests from the dev backbone namespace.
pub fn list_os_space_catalog_entries(port: &Arc<OsBackbonePorts>) -> Result<Vec<OsSpaceCatalogEntry>, VcsError> {
    let mut entries = Vec::new();
    let uris: Vec<String> = SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&port_key(port)).cloned().unwrap_or_default().into_iter().collect();
    for uri in uris {
        if os_space_id_from_backbone_uri(&uri).is_none() {
            continue;
        }
        let Ok(payload) = port.read(&uri) else {
            continue;
        };
        if payload.is_empty() {
            continue;
        }
        let document: OsSpaceDocument = decode_backbone_payload(&payload, space::S_SPACE_SCHEMA)?;
        entries.push(os_space_catalog_entry_from_document(&uri, &document)?);
    }
    entries.sort_by(|left, right| right.updated_at.cmp(&left.updated_at).then_with(|| left.name.cmp(&right.name)));
    Ok(entries)
}

/// 🆕️ Creates a space manifest PLUS one default "main" collection on the dev backbone — see
/// `## The inversion`/`Addressing` in the plan: a space no longer auto-creates a workflow artifact
/// (that's an explicit, later user action), only the collection every space needs to hold artifacts
/// in the first place.
pub fn create_os_space(name: &str, kind: space::SpaceKind, visibility: space::SpaceVisibility, owner: space::SpaceUser, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let space_id = create_os_id("space");
    let collection_id = create_os_id("collection");
    let mut space_snapshot = space::empty_space_snapshot(name.trim(), kind, visibility);
    space_snapshot.users.push(owner);
    space_snapshot.collections.push(space::CollectionRef { id: collection_id.clone(), name: "main".into(), document_id: collection_id.clone() });
    let space_document: OsSpaceDocument = create_backbone_document(space::S_SPACE_SCHEMA, &space_id, name.trim(), space_snapshot);
    let collection_document: OsCollectionDocument = create_backbone_document(space::S_COLLECTION_SCHEMA, &collection_id, "main", space::empty_collection_snapshot("main"));

    let space_uri = space::space_backbone_uri(&space_id);
    let collection_uri = space::collection_backbone_uri(&space_id, &collection_id);
    sync_backbone_document(&space_document, &space_uri, port)?;
    sync_backbone_document(&collection_document, &collection_uri, port)?;
    track_os_space_backbone_uri(port, &space_uri);
    track_os_space_backbone_uri(port, &collection_uri);
    os_space_catalog_entry_from_document(&space_uri, &space_document)
}

/// 🗑️ Deletes a space manifest and every collection it references from the dev backbone.
pub fn delete_os_space(space_id: &str, port: &Arc<OsBackbonePorts>) -> Result<(), VcsError> {
    let uri = space::space_backbone_uri(space_id);
    if let Ok(document) = load_os_space_document(space_id, port) {
        if let Ok(snapshot) = materialize_backbone_snapshot(&document, &[]) {
            for collection in &snapshot.collections {
                let collection_uri = space::collection_backbone_uri(space_id, &collection.id);
                untrack_os_space_backbone_uri(port, &collection_uri);
                port.write(&collection_uri, &[])?;
            }
        }
    }
    untrack_os_space_backbone_uri(port, &uri);
    port.write(&uri, &[])
}

/// 🌉️ Admits one space document into the catalog of `port` under its own id and name — the shared tail of
/// `import_os_space_from_dsl`/`import_os_space_from_pack` and of a studio newly bound to a file or folder backbone:
/// mints a fresh id when the source carried none, syncs, and tracks the catalog uri.
pub fn admit_os_space_document(mut document: OsSpaceDocument, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let space_id = if document.id.is_empty() { create_os_id("space") } else { document.id.clone() };
    let backbone_uri = space::space_backbone_uri(&space_id);
    document.id = space_id;
    sync_backbone_document(&document, &backbone_uri, port)?;
    track_os_space_backbone_uri(port, &backbone_uri);
    os_space_catalog_entry_from_document(&backbone_uri, &document)
}

/// 📥️ Imports a space manifest dsl text (`export_os_space_dsl`'s counterpart) onto the dev
/// backbone under the manifest's own name (the catalog lists a studio by its document name). Does not create a collection — a manifest imported this way is expected to already
/// reference its own collections (a fresh, collection-less space only comes from `create_os_space`).
pub fn import_os_space_from_dsl(dsl: &str, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let snapshot = <space::SpaceSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(dsl).map_err(|error| VcsError::Deserialize(error.message))?;
    let name = snapshot.name.trim().to_owned();
    admit_os_space_document(create_backbone_document(space::S_SPACE_SCHEMA, "", &name, snapshot), port)
}

/// 📦️ Pack counterpart of `import_os_space_from_dsl`: admits the pair under its own document id and the head
/// manifest's name (the catalog lists a studio by its document name).
pub fn import_os_space_from_pack(pack: &[u8], spr: &[u8], port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let parsed: store::os_store::text_format::ParsedDocumentText<space::SpaceSnapshot, space::SpaceMutation> = resolve_kernel_future(store::os_store::text_format::parse_document_pack(pack, spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let name = parsed.snapshot.name.trim().to_owned();
    let mut owners = parsed.into_envelope().into_owners();
    let mut edit_messages = Vec::with_capacity(owners.edit_messages.len());
    while let Some(entry) = owners.edit_messages.pop() { edit_messages.push(entry); }
    edit_messages.reverse();
    let document = BackboneDocument {
        schema: owners.schema,
        id: owners.id,
        name,
        vcs: owners.vcs,
        transitions: owners.transitions,
        edit_messages,
        conflicts: owners.conflicts,
        backbone: owners.backbone,
    };
    admit_os_space_document(document, port)
}

/// 📤️ Exports an already-loaded space manifest as pack bytes + ops text.
pub fn export_os_space_pack(document: &OsSpaceDocument) -> Result<store::os_store::serialization::ArtifactPackFiles, VcsError> {
    export_backbone_pack(document)
}

/// 📤️ DSL-text counterpart of `export_os_space_pack`.
pub fn export_os_space_dsl(document: &OsSpaceDocument) -> Result<store::os_store::text_format::ArtifactTextFiles, VcsError> {
    export_backbone_dsl(document)
}

/// 📂️ Loads a space manifest from the dev backbone.
pub fn load_os_space_document(space_id: &str, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceDocument, VcsError> {
    let backbone_uri = space::space_backbone_uri(space_id);
    let payload = port.read(&backbone_uri)?;
    if payload.is_empty() {
        return Err(VcsError::Backbone(format!("unknown os space: {space_id}")));
    }
    decode_backbone_payload(&payload, space::S_SPACE_SCHEMA)
}

/// 🌱️ Seeds the demo space when the catalog is empty.
pub fn seed_os_space_catalog_if_empty(seed_document: OsSpaceDocument, port: &Arc<OsBackbonePorts>) -> Result<Option<OsSpaceCatalogEntry>, VcsError> {
    if !list_os_space_catalog_entries(port)?.is_empty() {
        return Ok(None);
    }
    let space_id = if seed_document.id.is_empty() { "default".into() } else { seed_document.id.clone() };
    let backbone_uri = space::space_backbone_uri(&space_id);
    let mut seeded = seed_document;
    seeded.id = space_id;
    sync_backbone_document(&seeded, &backbone_uri, port)?;
    track_os_space_backbone_uri(port, &backbone_uri);
    Ok(Some(os_space_catalog_entry_from_document(&backbone_uri, &seeded)?))
}
//#endregion 🔖️SpaceCatalog
// #endregion host
