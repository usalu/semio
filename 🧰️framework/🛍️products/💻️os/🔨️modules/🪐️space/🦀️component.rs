//! 🪐️ Space/collection/artifact model — `SpaceSnapshot` (atelier/studio/archive manifest: name,
//! kind, visibility, users, collection refs) and `CollectionSnapshot` (flat parent-linked folder
//! tree + artifact entries), the roles/kinds/visibility laws (`space_role_of`/`can_write`, the atelier
//! single-author invariant), a pure path resolver, backbone URI helpers, the draft/asset volatility
//! model (`DraftCatalog`), and IO-free zip import/export. Unifies the target model from
//! `.claude/plans/the-final-goal-for-jolly-spindle.md`'s `## Design rulings` (`Schema lattice`, `The
//! inversion`, `Addressing`, `Roles/kinds/visibility`, `Draft vs asset`) — a peer kernel crate to
//! `workflow` (`🔁️workflow`), consumed by `framework/product/os/core`'s later inversion wave (W3) and
//! not yet wired to any live host/store (that wiring is W3/W4/W5's job — see each region's `🚧️` notes
//! for the exact narrower-interface decisions made here).

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{Cursor, Read as _, Seek, Write as _};
use std::sync::{Arc, LazyLock, Mutex};
use thiserror::Error;

//#region 🔖️Roles
/// 🏛️ A space's collaboration shape: `Atelier` (single-writer personal, reconcile-enforced exactly
/// one `Author`), `Studio` (multi-writer group, any number of `Author`s), `Archive` (frozen, nobody
/// writes).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum SpaceKind {
    Atelier,
    Studio,
    Archive,
}

/// 👁️ Whether a space is discoverable/readable by an anonymous visitor (`Public`, implicit anonymous
/// spectator — wired at the hub layer in W4) or membership-gated (`Private`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum SpaceVisibility {
    Private,
    Public,
}

/// 🧑️‍🤝️‍🧑️ A space member's permission level: `Author` (read-write) or `Spectator` (read-only). The
/// hub directory (`🌎️hub/🔨️modules/📇️directory`) re-declares this enum string-identically
/// (`"author"`/`"spectator"`, see `as_str`/`parse`) since it cannot depend on this wasm-facing crate —
/// keep the two in lockstep by hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum SpaceRole {
    Author,
    Spectator,
}

impl SpaceRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpaceRole::Author => "author",
            SpaceRole::Spectator => "spectator",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "author" => Some(SpaceRole::Author),
            "spectator" => Some(SpaceRole::Spectator),
            _ => None,
        }
    }
}

/// 🧑️ One space member: identity, display name, optional avatar, and their `SpaceRole`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SpaceUser {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub role: SpaceRole,
}

/// 🌉️ Checkpoint authorship (`vcs::Author`) is a distinct concept from space membership — a checkpoint
/// records who authored an edit even after they've left the space or been demoted. This is the one
/// permitted crossing between the two.
impl From<&SpaceUser> for vcs::Author {
    fn from(user: &SpaceUser) -> Self {
        vcs::Author { id: user.id.clone(), name: user.name.clone(), avatar: user.avatar.clone() }
    }
}
//#endregion 🔖️Roles

//#region 🔖️Space
pub const S_SPACE_SCHEMA: &str = "s.space";

/// 🔗️ One entry in `SpaceSnapshot.collections` — the collection's identity, display name, and the
/// `s.collection` document id it addresses (see `🔖️Addressing` in the plan: `CollectionEntry.id ==
/// artifact id == ArtifactEnvelope.id` for document artifacts; a `CollectionRef` follows the same
/// convention one level up).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CollectionRef {
    pub id: String,
    pub name: String,
    pub document_id: String,
}

/// 🏠️ A space's manifest: name, kind, visibility, membership, the collections it hosts, the
/// workflow plugin ids installed into it (`programs`, moved down from os-core's dissolved
/// `OsSnapshot` in W3 — see `## The inversion` in the plan), and the durable extension ledger
/// (`extensions`). Session-only `active_plugin_id`/`active_alternative_id` stay OUT of this document
/// by design (transient UI state, not manifest data) — see os-core's space app glue.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[dsl(id = "s.space")]
pub struct SpaceSnapshot {
    pub schema: String,
    pub name: String,
    pub kind: SpaceKind,
    pub visibility: SpaceVisibility,
    #[dsl(table)]
    pub users: Vec<SpaceUser>,
    #[dsl(table)]
    pub collections: Vec<CollectionRef>,
    #[serde(default)]
    pub programs: Vec<String>,
    #[serde(default)]
    pub extensions: Vec<InstalledExtension>,
}

/// 🧩️ One installed extension recorded in the space ledger — identity, package provenance, and
/// enablement. Distinct from session-only `loadedPlugins` handles; this is what survives reload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct InstalledExtension {
    pub extension_id: String,
    pub version: String,
    pub source_uri: String,
    pub package_hash: String,
    pub enabled: bool,
}

pub fn empty_space_snapshot(name: &str, kind: SpaceKind, visibility: SpaceVisibility) -> SpaceSnapshot {
    SpaceSnapshot {
        schema: S_SPACE_SCHEMA.into(),
        name: name.into(),
        kind,
        visibility,
        users: Vec::new(),
        collections: Vec::new(),
        programs: Vec::new(),
        extensions: Vec::new(),
    }
}

//#region 🔖️SpaceMutation
/// ⚡️ One settled space-manifest mutation. Every variant's op keyword is the auto-derived kebab-case
/// of its own name (`UpsertUser` -> `upsert-user`, ...) — see [`protocol::OpText`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SpaceMutation {
    SetName {
        name: String,
    },
    SetKind {
        kind: SpaceKind,
    },
    SetVisibility {
        visibility: SpaceVisibility,
    },
    UpsertUser {
        #[dsl(block)]
        user: SpaceUser,
    },
    RemoveUser {
        user_id: String,
    },
    AddCollection {
        #[dsl(block)]
        collection: CollectionRef,
    },
    RemoveCollection {
        collection_id: String,
    },
    RenameCollection {
        collection_id: String,
        name: String,
    },
    InstallProgram {
        plugin_id: String,
    },
    UninstallProgram {
        plugin_id: String,
    },
    InstallExtension {
        extension_id: String,
        version: String,
        source_uri: String,
        package_hash: String,
        enabled: bool,
    },
    UninstallExtension {
        extension_id: String,
    },
    SetExtensionEnabled {
        extension_id: String,
        enabled: bool,
    },
}

/// 🧬️ Sparse whole-field diff — mirrors `writer_op::WriterDiff`'s "one `Option<T>` per possible
/// mutation" shape, the pattern every `#[derive(dsl::DslDiff)]` struct uses (the derive only supports
/// structs, never tagged enums — see `dsl_derive::derive_dsl_diff`'s doc comment).

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for SpaceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for SpaceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDiff)]
pub struct SpaceDiff {
    pub name: Option<String>,
    pub kind: Option<SpaceKind>,
    pub visibility: Option<SpaceVisibility>,
    #[dsl(block)]
    pub upsert_user: Option<SpaceUser>,
    pub remove_user_id: Option<String>,
    #[dsl(block)]
    pub add_collection: Option<CollectionRef>,
    pub remove_collection_id: Option<String>,
    pub rename_collection_id: Option<String>,
    pub rename_collection_name: Option<String>,
    pub install_program: Option<String>,
    pub uninstall_program: Option<String>,
    #[dsl(block)]
    pub install_extension: Option<InstalledExtension>,
    pub uninstall_extension_id: Option<String>,
    pub set_extension_enabled_id: Option<String>,
    pub set_extension_enabled: Option<bool>,
}

impl protocol::MutationDiff<SpaceSnapshot> for SpaceDiff {
    fn apply(&self, base: &SpaceSnapshot) -> SpaceSnapshot {
        let mut next = base.clone();
        if let Some(name) = &self.name {
            next.name = name.clone();
        }
        if let Some(kind) = &self.kind {
            next.kind = *kind;
        }
        if let Some(visibility) = &self.visibility {
            next.visibility = *visibility;
        }
        if let Some(user) = &self.upsert_user {
            next.users.retain(|existing| existing.id != user.id);
            next.users.push(user.clone());
        }
        if let Some(user_id) = &self.remove_user_id {
            next.users.retain(|user| &user.id != user_id);
        }
        if let Some(collection) = &self.add_collection {
            next.collections.push(collection.clone());
        }
        if let Some(collection_id) = &self.remove_collection_id {
            next.collections.retain(|collection| &collection.id != collection_id);
        }
        if let Some(collection_id) = &self.rename_collection_id {
            if let Some(name) = &self.rename_collection_name {
                for collection in &mut next.collections {
                    if &collection.id == collection_id {
                        collection.name = name.clone();
                    }
                }
            }
        }
        if let Some(plugin_id) = &self.install_program {
            if !next.programs.contains(plugin_id) {
                next.programs.push(plugin_id.clone());
            }
        }
        if let Some(plugin_id) = &self.uninstall_program {
            next.programs.retain(|installed| installed != plugin_id);
        }
        if let Some(extension) = &self.install_extension {
            next.extensions.retain(|existing| existing.extension_id != extension.extension_id);
            next.extensions.push(extension.clone());
        }
        if let Some(extension_id) = &self.uninstall_extension_id {
            next.extensions.retain(|existing| &existing.extension_id != extension_id);
        }
        if let Some(extension_id) = &self.set_extension_enabled_id {
            if let Some(enabled) = self.set_extension_enabled {
                for extension in &mut next.extensions {
                    if &extension.extension_id == extension_id {
                        extension.enabled = enabled;
                    }
                }
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.visibility.is_some() {
            self.visibility = other.visibility;
        }
        if other.upsert_user.is_some() {
            self.upsert_user = other.upsert_user;
        }
        if other.remove_user_id.is_some() {
            self.remove_user_id = other.remove_user_id;
        }
        if other.add_collection.is_some() {
            self.add_collection = other.add_collection;
        }
        if other.remove_collection_id.is_some() {
            self.remove_collection_id = other.remove_collection_id;
        }
        if other.rename_collection_id.is_some() {
            self.rename_collection_id = other.rename_collection_id;
            self.rename_collection_name = other.rename_collection_name;
        }
        if other.install_program.is_some() {
            self.install_program = other.install_program;
        }
        if other.uninstall_program.is_some() {
            self.uninstall_program = other.uninstall_program;
        }
        if other.install_extension.is_some() {
            self.install_extension = other.install_extension;
        }
        if other.uninstall_extension_id.is_some() {
            self.uninstall_extension_id = other.uninstall_extension_id;
        }
        if other.set_extension_enabled_id.is_some() {
            self.set_extension_enabled_id = other.set_extension_enabled_id;
            self.set_extension_enabled = other.set_extension_enabled;
        }
    }
}

impl protocol::Mutation<SpaceSnapshot> for SpaceMutation {
    type Diff = SpaceDiff;

    fn diff(&self, _base: &SpaceSnapshot) -> SpaceDiff {
        let mut diff = SpaceDiff::default();
        match self {
            SpaceMutation::SetName { name } => diff.name = Some(name.clone()),
            SpaceMutation::SetKind { kind } => diff.kind = Some(*kind),
            SpaceMutation::SetVisibility { visibility } => diff.visibility = Some(*visibility),
            SpaceMutation::UpsertUser { user } => diff.upsert_user = Some(user.clone()),
            SpaceMutation::RemoveUser { user_id } => diff.remove_user_id = Some(user_id.clone()),
            SpaceMutation::AddCollection { collection } => diff.add_collection = Some(collection.clone()),
            SpaceMutation::RemoveCollection { collection_id } => diff.remove_collection_id = Some(collection_id.clone()),
            SpaceMutation::RenameCollection { collection_id, name } => {
                diff.rename_collection_id = Some(collection_id.clone());
                diff.rename_collection_name = Some(name.clone());
            }
            SpaceMutation::InstallProgram { plugin_id } => diff.install_program = Some(plugin_id.clone()),
            SpaceMutation::UninstallProgram { plugin_id } => diff.uninstall_program = Some(plugin_id.clone()),
            SpaceMutation::InstallExtension { extension_id, version, source_uri, package_hash, enabled } => {
                diff.install_extension = Some(InstalledExtension {
                    extension_id: extension_id.clone(),
                    version: version.clone(),
                    source_uri: source_uri.clone(),
                    package_hash: package_hash.clone(),
                    enabled: *enabled,
                });
            }
            SpaceMutation::UninstallExtension { extension_id } => diff.uninstall_extension_id = Some(extension_id.clone()),
            SpaceMutation::SetExtensionEnabled { extension_id, enabled } => {
                diff.set_extension_enabled_id = Some(extension_id.clone());
                diff.set_extension_enabled = Some(*enabled);
            }
        }
        diff
    }

    fn inverse(&self, base: &SpaceSnapshot) -> Vec<Self> {
        match self {
            SpaceMutation::SetName { .. } => vec![SpaceMutation::SetName { name: base.name.clone() }],
            SpaceMutation::SetKind { .. } => vec![SpaceMutation::SetKind { kind: base.kind }],
            SpaceMutation::SetVisibility { .. } => vec![SpaceMutation::SetVisibility { visibility: base.visibility }],
            SpaceMutation::UpsertUser { user } => match base.users.iter().find(|existing| existing.id == user.id) {
                Some(existing) => vec![SpaceMutation::UpsertUser { user: existing.clone() }],
                None => vec![SpaceMutation::RemoveUser { user_id: user.id.clone() }],
            },
            SpaceMutation::RemoveUser { user_id } => base.users.iter().find(|user| &user.id == user_id).map(|user| vec![SpaceMutation::UpsertUser { user: user.clone() }]).unwrap_or_default(),
            SpaceMutation::AddCollection { collection } => vec![SpaceMutation::RemoveCollection { collection_id: collection.id.clone() }],
            SpaceMutation::RemoveCollection { collection_id } => {
                base.collections.iter().find(|collection| &collection.id == collection_id).map(|collection| vec![SpaceMutation::AddCollection { collection: collection.clone() }]).unwrap_or_default()
            }
            SpaceMutation::RenameCollection { collection_id, .. } => base
                .collections
                .iter()
                .find(|collection| &collection.id == collection_id)
                .map(|collection| vec![SpaceMutation::RenameCollection { collection_id: collection_id.clone(), name: collection.name.clone() }])
                .unwrap_or_default(),
            SpaceMutation::InstallProgram { plugin_id } => {
                if base.programs.contains(plugin_id) {
                    Vec::new()
                } else {
                    vec![SpaceMutation::UninstallProgram { plugin_id: plugin_id.clone() }]
                }
            }
            SpaceMutation::UninstallProgram { plugin_id } => {
                if base.programs.contains(plugin_id) {
                    vec![SpaceMutation::InstallProgram { plugin_id: plugin_id.clone() }]
                } else {
                    Vec::new()
                }
            }
            SpaceMutation::InstallExtension { extension_id, .. } => match base.extensions.iter().find(|existing| &existing.extension_id == extension_id) {
                Some(existing) => vec![SpaceMutation::InstallExtension {
                    extension_id: existing.extension_id.clone(),
                    version: existing.version.clone(),
                    source_uri: existing.source_uri.clone(),
                    package_hash: existing.package_hash.clone(),
                    enabled: existing.enabled,
                }],
                None => vec![SpaceMutation::UninstallExtension { extension_id: extension_id.clone() }],
            },
            SpaceMutation::UninstallExtension { extension_id } => base
                .extensions
                .iter()
                .find(|existing| &existing.extension_id == extension_id)
                .map(|existing| {
                    vec![SpaceMutation::InstallExtension {
                        extension_id: existing.extension_id.clone(),
                        version: existing.version.clone(),
                        source_uri: existing.source_uri.clone(),
                        package_hash: existing.package_hash.clone(),
                        enabled: existing.enabled,
                    }]
                })
                .unwrap_or_default(),
            SpaceMutation::SetExtensionEnabled { extension_id, .. } => base
                .extensions
                .iter()
                .find(|existing| &existing.extension_id == extension_id)
                .map(|existing| {
                    vec![SpaceMutation::SetExtensionEnabled {
                        extension_id: extension_id.clone(),
                        enabled: existing.enabled,
                    }]
                })
                .unwrap_or_default(),
        }
    }

    /// 🤝️ Atelier single-author invariant — see `reconcile_space_atelier_invariant` below.
    fn reconcile(&self, snapshot: SpaceSnapshot) -> (SpaceSnapshot, Vec<protocol::ReconcileReport>) {
        reconcile_space_atelier_invariant(snapshot)
    }
}
//#endregion 🔖️SpaceMutation
//#endregion 🔖️Space

//#region 🔖️Collection
pub const S_COLLECTION_SCHEMA: &str = "s.collection";

/// 📁️ One parent-linked folder in a collection's flat tree. `parent_id: None` means root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CollectionFolder {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
}

/// 📦️ What a `CollectionEntry` addresses: either a document artifact (`schema` + the `s.<schema>`
/// document's own id — see `🔖️Addressing`: `CollectionEntry.id == artifact id == ArtifactEnvelope.id`
/// for document artifacts) or a content-addressed blob (files/meshes/breps-as-bytes).
///
/// 🧬️ Hand-crafted `dsl::DslVariants` instead of `#[derive(dsl::DslEnum)]`: the `Blob` variant embeds
/// `store::BlobRef` verbatim (the plan's `Addressing` design ruling pins this exact type), a foreign
/// type this crate cannot implement `dsl::DslField` for under the orphan rule — same reasoning as
/// `workflow::MediaContract`'s hand-crafted `dsl::DslField` impl for its own foreign sub-values. Since
/// `ArtifactBody` itself IS local, hand-writing `DslVariants` bridges `BlobRef`'s three fields
/// (`hash`/`size`/`media_type`) directly to scalar `dsl::FieldValue`s right here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactBody {
    Document { schema: String, document_id: String },
    Blob { blob: store::BlobRef },
}

fn artifact_body_document_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(Some("document"), dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "schema", dsl::Shape::Text), dsl::FieldSpec::new(1, "document-id", dsl::Shape::Text)])
}

fn artifact_body_blob_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        Some("blob"),
        dsl::RecordLayout::Inline,
        vec![dsl::FieldSpec::new(0, "hash", dsl::Shape::Text), dsl::FieldSpec::new(1, "size", dsl::Shape::UInt), dsl::FieldSpec::new(2, "media-type", dsl::Shape::Text)],
    )
}

impl dsl::DslVariants for ArtifactBody {
    fn variants() -> Vec<(String, fn() -> dsl::RecordSpec)> {
        vec![("document".to_string(), artifact_body_document_spec as fn() -> dsl::RecordSpec), ("blob".to_string(), artifact_body_blob_spec as fn() -> dsl::RecordSpec)]
    }

    fn to_named_record(&self) -> (String, dsl::RecordValue) {
        match self {
            ArtifactBody::Document { schema, document_id } => {
                let mut record = dsl::RecordValue::default();
                record.fields.insert(0, dsl::FieldValue::Text(schema.clone()));
                record.fields.insert(1, dsl::FieldValue::Text(document_id.clone()));
                ("document".to_string(), record)
            }
            ArtifactBody::Blob { blob } => {
                let mut record = dsl::RecordValue::default();
                record.fields.insert(0, dsl::FieldValue::Text(blob.hash.clone()));
                record.fields.insert(1, dsl::FieldValue::UInt(blob.size));
                record.fields.insert(2, dsl::FieldValue::Text(blob.media_type.clone()));
                ("blob".to_string(), record)
            }
        }
    }

    fn from_named_record(keyword: &str, record: &dsl::RecordValue) -> Result<Self, dsl::TextError> {
        match keyword {
            "document" => {
                let schema = match record.get(0) {
                    Some(dsl::FieldValue::Text(s)) => s.clone(),
                    other => return Err(dsl::__rt::field_error(format!("expected schema, found {other:?}"))),
                };
                let document_id = match record.get(1) {
                    Some(dsl::FieldValue::Text(s)) => s.clone(),
                    other => return Err(dsl::__rt::field_error(format!("expected document-id, found {other:?}"))),
                };
                Ok(ArtifactBody::Document { schema, document_id })
            }
            "blob" => {
                let hash = match record.get(0) {
                    Some(dsl::FieldValue::Text(s)) => s.clone(),
                    other => return Err(dsl::__rt::field_error(format!("expected hash, found {other:?}"))),
                };
                let size = match record.get(1) {
                    Some(dsl::FieldValue::UInt(v)) => *v,
                    other => return Err(dsl::__rt::field_error(format!("expected size, found {other:?}"))),
                };
                let media_type = match record.get(2) {
                    Some(dsl::FieldValue::Text(s)) => s.clone(),
                    other => return Err(dsl::__rt::field_error(format!("expected media-type, found {other:?}"))),
                };
                Ok(ArtifactBody::Blob { blob: store::BlobRef { hash, size, media_type } })
            }
            other => Err(dsl::__rt::field_error(format!("unknown ArtifactBody keyword '{other}'"))),
        }
    }
}

/// 🧾️ One addressable artifact placed in a collection folder tree. `id == artifact id ==
/// ArtifactEnvelope.id` for document bodies (see `🔖️Addressing`). `folder_id: None` means root-level.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CollectionEntry {
    pub id: String,
    pub folder_id: Option<String>,
    pub name: String,
    pub kind_id: String,
    #[dsl(statements)]
    pub body: Box<ArtifactBody>,
}

/// 🗂️ A collection's flat parent-linked folder tree plus its artifact entries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[dsl(id = "s.collection")]
pub struct CollectionSnapshot {
    pub schema: String,
    pub name: String,
    #[dsl(table)]
    pub folders: Vec<CollectionFolder>,
    // 🧮️ NOT `#[dsl(table)]`: a `#[dsl(statements)]` field (`CollectionEntry.body`) is not a
    // self-delimiting shape, so it cannot be a compact Structure-of-Arrays table COLUMN — only the
    // expanded Array-of-Structs `Shape::List(Record)` form (a full nested record per entry) can carry
    // it. `folders` above has no such field, so it stays compact.
    pub entries: Vec<CollectionEntry>,
}

pub fn empty_collection_snapshot(name: &str) -> CollectionSnapshot {
    CollectionSnapshot { schema: S_COLLECTION_SCHEMA.into(), name: name.into(), folders: Vec::new(), entries: Vec::new() }
}


//#region 🔖️HandcraftedArtifactCodecs
/// 🧬️ P6: `DslArtifact` emits helpers only — ArtifactDsl/ArtifactPack are handcrafted here.
impl store::ArtifactDsl for SpaceSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for SpaceSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactDsl for CollectionSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for CollectionSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️CollectionMutation
/// ⚡️ One settled collection-tree mutation. `Move*`/`Rename*`/`ReplaceEntryBody` diff as the WHOLE
/// post-mutation folder/entry record (see `CollectionDiff` below) rather than a bare field delta —
/// sidesteps the derive engine's lack of nested-`Option` support (a "was this field touched, and to
/// what new *optional* value" diff shape) while staying exactly as replayable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum CollectionMutation {
    SetName {
        name: String,
    },
    AddFolder {
        #[dsl(block)]
        folder: CollectionFolder,
        at: u32,
    },
    RemoveFolder {
        folder_id: String,
    },
    MoveFolder {
        folder_id: String,
        parent_id: Option<String>,
    },
    RenameFolder {
        folder_id: String,
        name: String,
    },
    AddEntry {
        #[dsl(block)]
        entry: CollectionEntry,
        at: u32,
    },
    RemoveEntry {
        entry_id: String,
    },
    MoveEntry {
        entry_id: String,
        folder_id: Option<String>,
    },
    RenameEntry {
        entry_id: String,
        name: String,
    },
    ReplaceEntryBody {
        entry_id: String,
        #[dsl(statements)]
        body: Box<ArtifactBody>,
    },
}


//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for CollectionMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for CollectionMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDiff)]
pub struct CollectionDiff {
    pub name: Option<String>,
    #[dsl(block)]
    pub add_folder: Option<CollectionFolder>,
    /// 🔢️ Companion to `add_folder` — the insertion index (clamped; `>= folders.len()` appends).
    /// Always `Some` exactly when `add_folder` is, kept as a sibling field rather than nested inside
    /// `add_folder` since the derive engine has no first-class "record + position" shape.
    pub add_folder_at: Option<u32>,
    pub remove_folder_id: Option<String>,
    #[dsl(block)]
    pub move_folder: Option<CollectionFolder>,
    #[dsl(block)]
    pub rename_folder: Option<CollectionFolder>,
    #[dsl(block)]
    pub add_entry: Option<CollectionEntry>,
    /// 🔢️ Companion to `add_entry`, same convention as `add_folder_at`.
    pub add_entry_at: Option<u32>,
    pub remove_entry_id: Option<String>,
    #[dsl(block)]
    pub move_entry: Option<CollectionEntry>,
    #[dsl(block)]
    pub rename_entry: Option<CollectionEntry>,
    #[dsl(block)]
    pub replace_entry_body: Option<CollectionEntry>,
}

fn replace_folder(folders: &mut Vec<CollectionFolder>, replacement: CollectionFolder) {
    match folders.iter_mut().find(|folder| folder.id == replacement.id) {
        Some(existing) => *existing = replacement,
        None => folders.push(replacement),
    }
}

fn replace_entry(entries: &mut Vec<CollectionEntry>, replacement: CollectionEntry) {
    match entries.iter_mut().find(|entry| entry.id == replacement.id) {
        Some(existing) => *existing = replacement,
        None => entries.push(replacement),
    }
}

impl protocol::MutationDiff<CollectionSnapshot> for CollectionDiff {
    fn apply(&self, base: &CollectionSnapshot) -> CollectionSnapshot {
        let mut next = base.clone();
        if let Some(name) = &self.name {
            next.name = name.clone();
        }
        if let Some(folder) = &self.add_folder {
            let at = (self.add_folder_at.unwrap_or(u32::MAX) as usize).min(next.folders.len());
            next.folders.insert(at, folder.clone());
        }
        if let Some(folder_id) = &self.remove_folder_id {
            // 🧮️ Mechanical replay only removes the folder itself — a dangling `parent_id`/
            // `folder_id` left pointing at it is `reconcile_collection_integrity`'s job (rules
            // `collection/folder-orphaned`/`collection/entry-folder-missing`), run separately after
            // `apply`, never inline here. Keeping this pure-removal is what makes `RemoveFolder`'s
            // mechanical inverse (`AddFolder`) exactly restore pre-state.
            next.folders.retain(|folder| &folder.id != folder_id);
        }
        if let Some(folder) = &self.move_folder {
            replace_folder(&mut next.folders, folder.clone());
        }
        if let Some(folder) = &self.rename_folder {
            replace_folder(&mut next.folders, folder.clone());
        }
        if let Some(entry) = &self.add_entry {
            let at = (self.add_entry_at.unwrap_or(u32::MAX) as usize).min(next.entries.len());
            next.entries.insert(at, entry.clone());
        }
        if let Some(entry_id) = &self.remove_entry_id {
            next.entries.retain(|entry| &entry.id != entry_id);
        }
        if let Some(entry) = &self.move_entry {
            replace_entry(&mut next.entries, entry.clone());
        }
        if let Some(entry) = &self.rename_entry {
            replace_entry(&mut next.entries, entry.clone());
        }
        if let Some(entry) = &self.replace_entry_body {
            replace_entry(&mut next.entries, entry.clone());
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.add_folder.is_some() {
            self.add_folder = other.add_folder;
            self.add_folder_at = other.add_folder_at;
        }
        if other.remove_folder_id.is_some() {
            self.remove_folder_id = other.remove_folder_id;
        }
        if other.move_folder.is_some() {
            self.move_folder = other.move_folder;
        }
        if other.rename_folder.is_some() {
            self.rename_folder = other.rename_folder;
        }
        if other.add_entry.is_some() {
            self.add_entry = other.add_entry;
            self.add_entry_at = other.add_entry_at;
        }
        if other.remove_entry_id.is_some() {
            self.remove_entry_id = other.remove_entry_id;
        }
        if other.move_entry.is_some() {
            self.move_entry = other.move_entry;
        }
        if other.rename_entry.is_some() {
            self.rename_entry = other.rename_entry;
        }
        if other.replace_entry_body.is_some() {
            self.replace_entry_body = other.replace_entry_body;
        }
    }
}

impl protocol::Mutation<CollectionSnapshot> for CollectionMutation {
    type Diff = CollectionDiff;

    fn diff(&self, base: &CollectionSnapshot) -> CollectionDiff {
        let mut diff = CollectionDiff::default();
        match self {
            CollectionMutation::SetName { name } => diff.name = Some(name.clone()),
            CollectionMutation::AddFolder { folder, at } => {
                diff.add_folder = Some(folder.clone());
                diff.add_folder_at = Some(*at);
            }
            CollectionMutation::RemoveFolder { folder_id } => diff.remove_folder_id = Some(folder_id.clone()),
            CollectionMutation::MoveFolder { folder_id, parent_id } => {
                if let Some(folder) = base.folders.iter().find(|folder| &folder.id == folder_id) {
                    let mut moved = folder.clone();
                    moved.parent_id = parent_id.clone();
                    diff.move_folder = Some(moved);
                }
            }
            CollectionMutation::RenameFolder { folder_id, name } => {
                if let Some(folder) = base.folders.iter().find(|folder| &folder.id == folder_id) {
                    let mut renamed = folder.clone();
                    renamed.name = name.clone();
                    diff.rename_folder = Some(renamed);
                }
            }
            CollectionMutation::AddEntry { entry, at } => {
                diff.add_entry = Some(entry.clone());
                diff.add_entry_at = Some(*at);
            }
            CollectionMutation::RemoveEntry { entry_id } => diff.remove_entry_id = Some(entry_id.clone()),
            CollectionMutation::MoveEntry { entry_id, folder_id } => {
                if let Some(entry) = base.entries.iter().find(|entry| &entry.id == entry_id) {
                    let mut moved = entry.clone();
                    moved.folder_id = folder_id.clone();
                    diff.move_entry = Some(moved);
                }
            }
            CollectionMutation::RenameEntry { entry_id, name } => {
                if let Some(entry) = base.entries.iter().find(|entry| &entry.id == entry_id) {
                    let mut renamed = entry.clone();
                    renamed.name = name.clone();
                    diff.rename_entry = Some(renamed);
                }
            }
            CollectionMutation::ReplaceEntryBody { entry_id, body } => {
                if let Some(entry) = base.entries.iter().find(|entry| &entry.id == entry_id) {
                    let mut replaced = entry.clone();
                    replaced.body = body.clone();
                    diff.replace_entry_body = Some(replaced);
                }
            }
        }
        diff
    }

    fn inverse(&self, base: &CollectionSnapshot) -> Vec<Self> {
        match self {
            CollectionMutation::SetName { .. } => vec![CollectionMutation::SetName { name: base.name.clone() }],
            CollectionMutation::AddFolder { folder, .. } => vec![CollectionMutation::RemoveFolder { folder_id: folder.id.clone() }],
            CollectionMutation::RemoveFolder { folder_id } => base
                .folders
                .iter()
                .position(|folder| &folder.id == folder_id)
                .map(|at| vec![CollectionMutation::AddFolder { folder: base.folders[at].clone(), at: at as u32 }])
                .unwrap_or_default(),
            CollectionMutation::MoveFolder { folder_id, .. } => base
                .folders
                .iter()
                .find(|folder| &folder.id == folder_id)
                .map(|folder| vec![CollectionMutation::MoveFolder { folder_id: folder_id.clone(), parent_id: folder.parent_id.clone() }])
                .unwrap_or_default(),
            CollectionMutation::RenameFolder { folder_id, .. } => {
                base.folders.iter().find(|folder| &folder.id == folder_id).map(|folder| vec![CollectionMutation::RenameFolder { folder_id: folder_id.clone(), name: folder.name.clone() }]).unwrap_or_default()
            }
            CollectionMutation::AddEntry { entry, .. } => vec![CollectionMutation::RemoveEntry { entry_id: entry.id.clone() }],
            CollectionMutation::RemoveEntry { entry_id } => base
                .entries
                .iter()
                .position(|entry| &entry.id == entry_id)
                .map(|at| vec![CollectionMutation::AddEntry { entry: base.entries[at].clone(), at: at as u32 }])
                .unwrap_or_default(),
            CollectionMutation::MoveEntry { entry_id, .. } => {
                base.entries.iter().find(|entry| &entry.id == entry_id).map(|entry| vec![CollectionMutation::MoveEntry { entry_id: entry_id.clone(), folder_id: entry.folder_id.clone() }]).unwrap_or_default()
            }
            CollectionMutation::RenameEntry { entry_id, .. } => {
                base.entries.iter().find(|entry| &entry.id == entry_id).map(|entry| vec![CollectionMutation::RenameEntry { entry_id: entry_id.clone(), name: entry.name.clone() }]).unwrap_or_default()
            }
            CollectionMutation::ReplaceEntryBody { entry_id, .. } => {
                base.entries.iter().find(|entry| &entry.id == entry_id).map(|entry| vec![CollectionMutation::ReplaceEntryBody { entry_id: entry_id.clone(), body: entry.body.clone() }]).unwrap_or_default()
            }
        }
    }

    /// 🤝️ Referential-integrity pass — see `reconcile_collection_integrity` below.
    fn reconcile(&self, snapshot: CollectionSnapshot) -> (CollectionSnapshot, Vec<protocol::ReconcileReport>) {
        reconcile_collection_integrity(snapshot)
    }
}
//#endregion 🔖️CollectionMutation
//#endregion 🔖️Collection

//#region 🔖️Laws
/// 🔎️ Looks up a member's role in a space, if they are one.
pub fn space_role_of(space: &SpaceSnapshot, user_id: &str) -> Option<SpaceRole> {
    space.users.iter().find(|user| user.id == user_id).map(|user| user.role)
}

/// ✍️ Archive spaces never accept writes; atelier/studio spaces accept writes from any `Author`
/// member (the atelier "exactly one author" cardinality is a reconcile-enforced invariant, not a
/// `can_write` distinction — see `reconcile_space_atelier_invariant`).
pub fn can_write(space: &SpaceSnapshot, user_id: &str) -> bool {
    match space.kind {
        SpaceKind::Archive => false,
        SpaceKind::Atelier | SpaceKind::Studio => space_role_of(space, user_id) == Some(SpaceRole::Author),
    }
}

/// 🤝️ Atelier invariant: at most one member holds `Author`. If reconciliation finds more than one
/// (a concurrent membership merge), every author but the lexicographically-smallest-id one is demoted
/// to `Spectator`, deterministically across peers replaying the same operation — surfaced as conflict
/// `"space/atelier-multi-author"`.
pub fn reconcile_space_atelier_invariant(mut snapshot: SpaceSnapshot) -> (SpaceSnapshot, Vec<protocol::ReconcileReport>) {
    let mut reports = Vec::new();
    if snapshot.kind == SpaceKind::Atelier {
        let mut author_ids: Vec<String> = snapshot.users.iter().filter(|user| user.role == SpaceRole::Author).map(|user| user.id.clone()).collect();
        author_ids.sort();
        if author_ids.len() > 1 {
            let keep = author_ids[0].clone();
            for user in &mut snapshot.users {
                if user.role == SpaceRole::Author && user.id != keep {
                    user.role = SpaceRole::Spectator;
                }
            }
            reports.push(protocol::ReconcileReport {
                id: "space/atelier-multi-author".into(),
                message: format!("atelier space retains a single author ({keep}); demoted the rest to spectator"),
                severity: protocol::ReconcileSeverity::Warning,
            });
        }
    }
    (snapshot, reports)
}

/// 🌳️ Which folder ids are cyclic (each folder's own id is in the returned set exactly when walking
/// its `parent_id` chain eventually revisits it).
fn folders_in_cycle(folders: &[CollectionFolder]) -> HashSet<String> {
    let parents: HashMap<&str, Option<&str>> = folders.iter().map(|folder| (folder.id.as_str(), folder.parent_id.as_deref())).collect();
    let mut in_cycle = HashSet::new();
    for folder in folders {
        let mut path: Vec<&str> = Vec::new();
        let mut current = folder.id.as_str();
        loop {
            if let Some(position) = path.iter().position(|id| *id == current) {
                for id in &path[position..] {
                    in_cycle.insert((*id).to_string());
                }
                break;
            }
            path.push(current);
            match parents.get(current).copied().flatten() {
                Some(parent) => current = parent,
                None => break,
            }
            if path.len() > folders.len() + 1 {
                break;
            }
        }
    }
    in_cycle
}

fn dedupe_folder_names(folders: &mut [CollectionFolder], reports: &mut Vec<protocol::ReconcileReport>) {
    let mut seen: HashSet<(Option<String>, String)> = HashSet::new();
    for folder in folders.iter_mut() {
        let mut key = (folder.parent_id.clone(), folder.name.clone());
        if seen.contains(&key) {
            let mut suffix = 2u32;
            let mut candidate = format!("{} ({suffix})", folder.name);
            while seen.contains(&(folder.parent_id.clone(), candidate.clone())) {
                suffix += 1;
                candidate = format!("{} ({suffix})", folder.name);
            }
            reports.push(protocol::ReconcileReport {
                id: "collection/folder-name-collision".into(),
                message: format!("folder {} renamed to '{candidate}' to avoid a sibling name collision", folder.id),
                severity: protocol::ReconcileSeverity::Info,
            });
            folder.name = candidate.clone();
            key = (folder.parent_id.clone(), candidate);
        }
        seen.insert(key);
    }
}

fn dedupe_entry_names(entries: &mut [CollectionEntry], reports: &mut Vec<protocol::ReconcileReport>) {
    let mut seen: HashSet<(Option<String>, String)> = HashSet::new();
    for entry in entries.iter_mut() {
        let mut key = (entry.folder_id.clone(), entry.name.clone());
        if seen.contains(&key) {
            let mut suffix = 2u32;
            let mut candidate = format!("{} ({suffix})", entry.name);
            while seen.contains(&(entry.folder_id.clone(), candidate.clone())) {
                suffix += 1;
                candidate = format!("{} ({suffix})", entry.name);
            }
            reports.push(protocol::ReconcileReport {
                id: "collection/entry-name-collision".into(),
                message: format!("entry {} renamed to '{candidate}' to avoid a sibling name collision", entry.id),
                severity: protocol::ReconcileSeverity::Info,
            });
            entry.name = candidate.clone();
            key = (entry.folder_id.clone(), candidate);
        }
        seen.insert(key);
    }
}

/// 🤝️ Post-materialization collection integrity pass, run in order: (1) a folder whose `parent_id`
/// references a missing folder reparents to root, (2) a folder participating in a parent cycle is cut
/// to root, (3) sibling folders/entries with a name collision (same parent) get a numeric suffix, (4)
/// an entry whose `folder_id` references a missing folder moves to root. Each rule operates on the
/// state the previous one produced — mirrors os-core's `reconcile_os_workflow` ordered-rules style.
pub fn reconcile_collection_integrity(mut snapshot: CollectionSnapshot) -> (CollectionSnapshot, Vec<protocol::ReconcileReport>) {
    let mut reports = Vec::new();

    //#region OrphanFolderReparent
    let folder_ids: HashSet<String> = snapshot.folders.iter().map(|folder| folder.id.clone()).collect();
    for folder in &mut snapshot.folders {
        if let Some(parent_id) = &folder.parent_id {
            if !folder_ids.contains(parent_id) {
                reports.push(protocol::ReconcileReport {
                    id: "collection/folder-orphaned".into(),
                    message: format!("folder {} referenced missing parent {parent_id}; reparented to root", folder.id),
                    severity: protocol::ReconcileSeverity::Warning,
                });
                folder.parent_id = None;
            }
        }
    }
    //#endregion OrphanFolderReparent

    //#region FolderCycleCut
    let cyclic = folders_in_cycle(&snapshot.folders);
    for folder in &mut snapshot.folders {
        if cyclic.contains(&folder.id) {
            reports.push(protocol::ReconcileReport { id: "collection/folder-cycle".into(), message: format!("folder {} participates in a parent cycle; cut to root", folder.id), severity: protocol::ReconcileSeverity::Blocking });
            folder.parent_id = None;
        }
    }
    //#endregion FolderCycleCut

    //#region SiblingNameCollisionSuffix
    dedupe_folder_names(&mut snapshot.folders, &mut reports);
    dedupe_entry_names(&mut snapshot.entries, &mut reports);
    //#endregion SiblingNameCollisionSuffix

    //#region EntryMissingFolderReparent
    let folder_ids: HashSet<String> = snapshot.folders.iter().map(|folder| folder.id.clone()).collect();
    for entry in &mut snapshot.entries {
        if let Some(folder_id) = &entry.folder_id {
            if !folder_ids.contains(folder_id) {
                reports.push(protocol::ReconcileReport {
                    id: "collection/entry-folder-missing".into(),
                    message: format!("entry {} referenced missing folder {folder_id}; moved to root", entry.id),
                    severity: protocol::ReconcileSeverity::Warning,
                });
                entry.folder_id = None;
            }
        }
    }
    //#endregion EntryMissingFolderReparent

    (snapshot, reports)
}

/// 🧵️ Root-to-leaf folder path (slash-joined names), or `None` if `folder_id` is absent or its parent
/// chain is cyclic (a resolver never persists — never trust it over reconciled data with a real cycle).
pub fn folder_path(collection: &CollectionSnapshot, folder_id: &str) -> Option<String> {
    let by_id: HashMap<&str, &CollectionFolder> = collection.folders.iter().map(|folder| (folder.id.as_str(), folder)).collect();
    let mut segments: Vec<String> = Vec::new();
    let mut current = folder_id.to_string();
    let mut guard = 0usize;
    loop {
        let folder = by_id.get(current.as_str())?;
        segments.push(folder.name.clone());
        guard += 1;
        if guard > collection.folders.len() {
            return None;
        }
        match &folder.parent_id {
            Some(parent) => current = parent.clone(),
            None => break,
        }
    }
    segments.reverse();
    Some(segments.join("/"))
}

/// 🧵️ Full slash-joined path to an entry (its folder path plus its own name), or `None` if the entry
/// doesn't exist or its folder chain doesn't resolve.
pub fn entry_path(collection: &CollectionSnapshot, entry_id: &str) -> Option<String> {
    let entry = collection.entries.iter().find(|entry| entry.id == entry_id)?;
    let prefix = match &entry.folder_id {
        Some(folder_id) => folder_path(collection, folder_id)?,
        None => String::new(),
    };
    Some(if prefix.is_empty() { entry.name.clone() } else { format!("{prefix}/{}", entry.name) })
}

/// 🔎️ Resolves a slash-joined path to its `CollectionEntry`, pure over the live snapshot — moves
/// and renames never break a persisted ref because paths are never persisted, only ids (see
/// `🔖️Addressing`).
pub fn resolve_entry_by_path<'a>(collection: &'a CollectionSnapshot, path: &str) -> Option<&'a CollectionEntry> {
    collection.entries.iter().find(|entry| entry_path(collection, &entry.id).as_deref() == Some(path))
}

/// 🔗️ `space://<space_id>` — the space manifest's own backbone URI.
pub fn space_backbone_uri(space_id: &str) -> String {
    format!("space://{space_id}")
}

/// 🔗️ `space://<space_id>/collection/<collection_id>`.
pub fn collection_backbone_uri(space_id: &str, collection_id: &str) -> String {
    format!("space://{space_id}/collection/{collection_id}")
}

/// 🔗️ `space://<space_id>/artifact/<artifact_id>`.
pub fn artifact_backbone_uri(space_id: &str, artifact_id: &str) -> String {
    format!("space://{space_id}/artifact/{artifact_id}")
}
//#endregion 🔖️Laws

//#region 🔖️Drafts
/// 🗄️ Every draft artifact lives at `temp://draft/<id>` while it's a draft — volatility is placement,
/// not an envelope flag (see `## Draft vs asset` in the plan).
pub const DRAFT_URI_PREFIX: &str = "temp://draft/";

/// 🔗️ `temp://draft/<artifact_id>`.
pub fn draft_uri(artifact_id: &str) -> String {
    format!("{DRAFT_URI_PREFIX}{artifact_id}")
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SpaceError {
    #[error("unknown draft: {0}")]
    UnknownDraft(String),
    #[error("draft backbone error: {0}")]
    Backbone(String),
}

//#region 🔖️DraftBackbone
/// 🔌️ Byte-oriented backbone port for draft<->asset envelope relocation — mirrors os-core's
/// `OsBackbonePort`/blanket-bridge pattern exactly (`🧰️framework/🛍️products/💻️os`'s `host::OsBackbonePort`),
/// but declared HERE rather than reused from there: os-core depends on `space` (`pub use space::*;`),
/// so depending back on os-core's trait would cycle the dependency graph. Every real transport this
/// crate needs (`store::MemoryBackbonePort`, `store::LocalStorageBackbonePort`, the host file/folder
/// ports) is already `store::BackbonePort`-shaped (string payloads) — the blanket impl below bridges
/// bytes<->base64 text exactly like os-core's own bridge, so any `Arc<dyn store::BackbonePort>`-backed
/// concrete port a caller already holds satisfies `Arc<dyn SpaceBackbonePort>` for free, and (crucially
/// for `draft_catalog_for`'s per-port keying below) preserves the SAME underlying `Arc` data pointer
/// across both trait-object views since unsizing coercion never reallocates.
pub trait SpaceBackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<Vec<u8>, vcs::VcsError>;
    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), vcs::VcsError>;
}

impl<T: store::BackbonePort> SpaceBackbonePort for T {
    fn read(&self, uri: &str) -> Result<Vec<u8>, vcs::VcsError> {
        use base64::Engine;
        let text = store::BackbonePort::read(self, uri)?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        base64::engine::general_purpose::STANDARD.decode(text).map_err(|error| vcs::VcsError::Deserialize(error.to_string()))
    }

    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), vcs::VcsError> {
        use base64::Engine;
        if payload.is_empty() {
            return store::BackbonePort::write(self, uri, "");
        }
        store::BackbonePort::write(self, uri, &base64::engine::general_purpose::STANDARD.encode(payload))
    }
}
//#endregion 🔖️DraftBackbone

/// 📄️ Bookkeeping for one draft artifact: identity, artifact kind, document schema, display name, and
/// TTL (`expires_at_ms: None` means pinned — the plan's default TTL is 7 days, a caller policy, not a
/// constant this pure catalog hardcodes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DraftEntry {
    pub artifact_id: String,
    pub kind_id: String,
    pub schema: String,
    pub name: String,
    pub created_at_ms: u64,
    pub expires_at_ms: Option<u64>,
}

/// 🗄️ Draft bookkeeping registry (TTL sweep, promote/demote as real operation-sourced byte moves).
///
/// 🎯️ W5 Lane B: promoted from the W2/W4 stub to the real thing. `promote_draft`/`demote_asset` now
/// relocate the draft's actual envelope bytes via an injected `SpaceBackbonePort` — read at
/// `draft_uri`, written at `artifact_backbone_uri`/vice versa, byte-for-byte, no decode/re-encode —
/// while still returning the `CollectionMutation` (`AddEntry`/`RemoveEntry`) that keeps promotion
/// itself operation-sourced. One `DraftCatalog` per distinct backbone port identity lives in the
/// port-keyed global registry below (`draft_catalog_for`), mirroring os-core's `SPACE_CATALOG_URIS`
/// per-port keying — this crate still doesn't reach into os-core's session state directly (that
/// dependency would cycle), it just now offers the SAME per-port-identity registry SHAPE os-core's
/// `list_os_space_catalog_entries`/`create_os_space` already use, so a caller (os-core, or an app like
/// `home`) wires it in by simply calling `draft_catalog_for(&port)` with whatever
/// `Arc<dyn SpaceBackbonePort>` it already holds.
#[derive(Default)]
pub struct DraftCatalog {
    drafts: Mutex<HashMap<String, DraftEntry>>,
}

impl DraftCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌱️ Mints a fresh draft id and registers its bookkeeping. `now_ms`/`ttl_ms` are caller-supplied
    /// (this crate is pure data plus pure functions — no wall-clock reads, matching `vcs`'s own doc
    /// comment convention).
    pub fn create_draft(&self, kind_id: &str, schema: &str, name: &str, now_ms: u64, ttl_ms: Option<u64>) -> DraftEntry {
        static DRAFT_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let seq = DRAFT_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let artifact_id = vcs::content_addressed_entity_id(
            "draft",
            format!("{kind_id}\0{schema}\0{name}\0{now_ms}\0{seq}").as_bytes(),
        );
        let entry = DraftEntry { artifact_id, kind_id: kind_id.into(), schema: schema.into(), name: name.into(), created_at_ms: now_ms, expires_at_ms: ttl_ms.map(|ttl| now_ms + ttl) };
        self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(entry.artifact_id.clone(), entry.clone());
        entry
    }

    pub fn list_drafts(&self) -> Vec<DraftEntry> {
        let mut entries: Vec<DraftEntry> = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).values().cloned().collect();
        entries.sort_by(|a, b| a.artifact_id.cmp(&b.artifact_id));
        entries
    }

    /// ⏰️ Removes every draft whose `expires_at_ms` is at or before `now_ms` from the bookkeeping,
    /// best-effort tombstoning each one's `draft_uri` bytes via `port` (empty-payload write, same
    /// convention as os-core's `delete_os_space`) so an expired draft doesn't leak backbone storage —
    /// bookkeeping removal is still the source of truth (a tombstone write failure doesn't undo it).
    /// Returns the expired ids.
    pub fn expire_drafts(&self, now_ms: u64, port: &Arc<dyn SpaceBackbonePort>) -> Vec<String> {
        let expired: Vec<String> = {
            let mut drafts = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let expired: Vec<String> = drafts.values().filter(|entry| entry.expires_at_ms.is_some_and(|expires_at| expires_at <= now_ms)).map(|entry| entry.artifact_id.clone()).collect();
            for artifact_id in &expired {
                drafts.remove(artifact_id);
            }
            expired
        };
        for artifact_id in &expired {
            let _ = port.write(&draft_uri(artifact_id), &[]);
        }
        expired
    }

    /// 📚️ `list_drafts` preceded by a real `expire_drafts` sweep — the natural "sweep before listing"
    /// call site (mirrors the spirit of os-core's catalog-listing entry points): any caller that lists
    /// drafts for display should always see a freshly-swept set rather than stale expired entries.
    pub fn list_drafts_sweeping_expired(&self, now_ms: u64, port: &Arc<dyn SpaceBackbonePort>) -> Vec<DraftEntry> {
        self.expire_drafts(now_ms, port);
        self.list_drafts()
    }

    /// 🗑️ Discards a draft outright (never promoted) — removes its bookkeeping and best-effort
    /// tombstones its `draft_uri` bytes via `port`. Returns the removed bookkeeping, if any existed.
    pub fn discard_draft(&self, port: &Arc<dyn SpaceBackbonePort>, draft_id: &str) -> Option<DraftEntry> {
        let removed = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(draft_id);
        if removed.is_some() {
            let _ = port.write(&draft_uri(draft_id), &[]);
        }
        removed
    }

    /// ⬆️ REAL promotion: relocates the draft's envelope bytes — whatever opaque blob the caller wrote
    /// at `draft_uri(draft_id)` (a pack+spr snapshot, an `encode_backbone_payload`-framed blob, etc. —
    /// this catalog never decodes it) — to `artifact_backbone_uri(space_id, draft_id)` via `port`,
    /// byte-for-byte (no decode/re-encode anywhere in this path, so the moved bytes are IDENTICAL
    /// before and after, just at a different backbone uri — the plan's exact promotion invariant),
    /// then tombstones the draft uri. Only removes the draft bookkeeping once the byte move fully
    /// succeeds. Returns the draft bookkeeping (removed from this catalog) plus the
    /// `CollectionMutation::AddEntry` the caller applies to their `CollectionSnapshot` — promotion
    /// stays operation-sourced even though it now really touches bytes. The artifact keeps its id
    /// (`entry.id == draft.artifact_id == document id`); a caller who knows the artifact's concrete
    /// `<P, Mutation>` pair can reconstruct a live `store::ArtifactStore<P, Mutation>` from the SAME
    /// moved bytes via `import_document_artifact` (see `🔖️ZipStoreBridge` above) and register it into
    /// their `store::SpaceHost` (`register_member`/`register_space_documents`) — this catalog stays
    /// type-erased on purpose (same reasoning as `ArtifactBody`'s hand-written `DslVariants`) and never
    /// touches `P`/`Mutation` itself, so it never calls `SpaceHost` directly.
    pub fn promote_draft(&self, port: &Arc<dyn SpaceBackbonePort>, space_id: &str, draft_id: &str, folder_id: Option<String>) -> Result<(DraftEntry, CollectionMutation), SpaceError> {
        let draft = {
            let drafts = self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            drafts.get(draft_id).cloned().ok_or_else(|| SpaceError::UnknownDraft(draft_id.to_string()))?
        };

        let source_uri = draft_uri(draft_id);
        let target_uri = artifact_backbone_uri(space_id, draft_id);
        let envelope_bytes = port.read(&source_uri).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&target_uri, &envelope_bytes).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&source_uri, &[]).map_err(|error| SpaceError::Backbone(error.to_string()))?;

        self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(draft_id);

        let entry = CollectionEntry {
            id: draft.artifact_id.clone(),
            folder_id,
            name: draft.name.clone(),
            kind_id: draft.kind_id.clone(),
            body: Box::new(ArtifactBody::Document { schema: draft.schema.clone(), document_id: draft.artifact_id.clone() }),
        };
        Ok((draft, CollectionMutation::AddEntry { entry, at: u32::MAX }))
    }

    /// ⏪️ Demotion's STRUCTURAL inverse — removing the entry from the collection. `CollectionMutation`'s
    /// own `inverse()` on `AddEntry` already produces exactly this (see `## Draft vs asset`'s
    /// "demotion is AddEntry's natural inverse"); kept as a standalone helper since a caller building
    /// a demote flow from scratch (not undoing a specific `AddEntry`) needs the operation without an
    /// `AddEntry` in hand. Does NOT move bytes — see `demote_asset` for the real byte-moving version.
    pub fn demote_operation(entry_id: &str) -> CollectionMutation {
        CollectionMutation::RemoveEntry { entry_id: entry_id.to_string() }
    }

    /// ⏪️ REAL demotion: the byte-moving inverse of `promote_draft` — relocates `entry`'s envelope
    /// bytes back from `artifact_backbone_uri(space_id, entry.id)` to `draft_uri(entry.id)`
    /// byte-for-byte, tombstones the asset uri, and re-registers fresh draft bookkeeping (`now_ms`/
    /// `ttl_ms` are caller-supplied, same convention as `create_draft` — a demoted draft gets a fresh
    /// TTL window, it doesn't inherit whatever deadline it had before its original promotion). Returns
    /// the `CollectionMutation::RemoveEntry` for the caller to apply to their `CollectionSnapshot`
    /// (identical to `demote_operation`'s output — this is the byte-touching sibling of that pure
    /// helper, needed whenever a demotion must actually relocate bytes rather than just undo an
    /// in-hand `AddEntry`).
    pub fn demote_asset(&self, port: &Arc<dyn SpaceBackbonePort>, space_id: &str, entry: &CollectionEntry, kind_id: &str, schema: &str, now_ms: u64, ttl_ms: Option<u64>) -> Result<CollectionMutation, SpaceError> {
        let source_uri = artifact_backbone_uri(space_id, &entry.id);
        let target_uri = draft_uri(&entry.id);
        let envelope_bytes = port.read(&source_uri).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&target_uri, &envelope_bytes).map_err(|error| SpaceError::Backbone(error.to_string()))?;
        port.write(&source_uri, &[]).map_err(|error| SpaceError::Backbone(error.to_string()))?;

        let draft = DraftEntry { artifact_id: entry.id.clone(), kind_id: kind_id.into(), schema: schema.into(), name: entry.name.clone(), created_at_ms: now_ms, expires_at_ms: ttl_ms.map(|ttl| now_ms + ttl) };
        self.drafts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(draft.artifact_id.clone(), draft);
        Ok(Self::demote_operation(&entry.id))
    }
}

//#region 🔖️DraftRegistry
/// 🗄️ Port-keyed global `DraftCatalog` registry — mirrors os-core's `SPACE_CATALOG_URIS`/`port_key`
/// per-port-identity keying exactly (`Arc::as_ptr` truncated to a bare data-pointer `usize`, dropping
/// the vtable so two differently-vtabled trait-object views of the SAME underlying `Arc` allocation
/// still key identically — see `SpaceBackbonePort`'s own doc). One `DraftCatalog` per distinct port
/// identity, shared by every caller holding (a clone of) that port.
static DRAFT_CATALOG_REGISTRY: LazyLock<Mutex<HashMap<usize, Arc<DraftCatalog>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn draft_catalog_port_key(port: &Arc<dyn SpaceBackbonePort>) -> usize {
    Arc::as_ptr(port) as *const () as usize
}

/// 🔎️ Gets (or lazily creates) the `DraftCatalog` for `port`'s identity. Returns a cheap `Arc` clone —
/// every caller sharing the same port shares the same draft bookkeeping, exactly the way
/// `list_os_space_catalog_entries`/`create_os_space` share `SPACE_CATALOG_URIS` per port in os-core.
pub fn draft_catalog_for(port: &Arc<dyn SpaceBackbonePort>) -> Arc<DraftCatalog> {
    DRAFT_CATALOG_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(draft_catalog_port_key(port)).or_insert_with(|| Arc::new(DraftCatalog::new())).clone()
}
//#endregion 🔖️DraftRegistry
//#endregion 🔖️Drafts

//#region 🔖️Zip
/// 📦️ `export_collection_zip`/`import_collection_zip` errors.
#[derive(Debug, Error)]
pub enum SpaceZipError {
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("pack error: {0}")]
    Pack(String),
    #[error("missing path for entry {0}")]
    MissingPath(String),
}

/// 📤️ Everything `import_collection_zip` recovers from a zip byte stream: the collection snapshot
/// itself (plus its own serialized history bytes), and per-entry artifact/blob bytes keyed by the
/// entry they belong to.
pub struct ImportedCollection {
    pub collection: CollectionSnapshot,
    pub collection_spr: Vec<u8>,
    pub artifacts: Vec<(CollectionEntry, Vec<u8>, Vec<u8>)>,
    pub blobs: Vec<(store::BlobRef, Vec<u8>)>,
}

fn zip_file_options() -> zip::write::SimpleFileOptions {
    // 🕰️ Fixed (epoch) timestamp on every entry — the export→import→export byte-stability law
    // depends on nothing time-varying leaking into the zip's central directory.
    zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).last_modified_time(zip::DateTime::default())
}

fn write_zip_file<W: std::io::Write + Seek>(writer: &mut zip::ZipWriter<W>, name: &str, bytes: &[u8], options: zip::write::SimpleFileOptions) -> Result<(), SpaceZipError> {
    writer.start_file(name, options)?;
    writer.write_all(bytes)?;
    Ok(())
}

fn read_zip_entry<R: std::io::Read + Seek>(archive: &mut zip::ZipArchive<R>, name: &str) -> Result<Vec<u8>, SpaceZipError> {
    let mut file = archive.by_name(name)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// 📤️ Exports a collection to a zip byte stream: `collection.collection.pack`/`.spr` at the root, each
/// document artifact at `<folder path>/<name>.pack` + `.spr` (lossless — full VCS history survives via
/// the injected `read_artifact` bytes), each blob raw at its path. IO-free: `read_artifact`/`read_blob`
/// are injected so this crate never touches a live store/filesystem itself — the caller supplies
/// already-serialized bytes from wherever they actually live (a `ArtifactEnvelope`'s pack/spr, a
/// `BlobStore`). Entries are written in id order for determinism (the byte-stability law below).
pub fn export_collection_zip(
    collection: &CollectionSnapshot,
    collection_spr: &[u8],
    read_artifact: &dyn Fn(&str) -> Result<(Vec<u8>, Vec<u8>), SpaceZipError>,
    read_blob: &dyn Fn(&str) -> Result<Vec<u8>, SpaceZipError>,
) -> Result<Vec<u8>, SpaceZipError> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip_file_options();

    write_zip_file(&mut writer, "collection.collection.pack", &store::ArtifactPack::encode_pack(collection), options)?;
    write_zip_file(&mut writer, "collection.collection.spr", collection_spr, options)?;

    let mut entries: Vec<&CollectionEntry> = collection.entries.iter().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    for entry in entries {
        let path = entry_path(collection, &entry.id).ok_or_else(|| SpaceZipError::MissingPath(entry.id.clone()))?;
        match entry.body.as_ref() {
            ArtifactBody::Document { .. } => {
                let (pack_bytes, spr_bytes) = read_artifact(&entry.id)?;
                write_zip_file(&mut writer, &format!("{path}.pack"), &pack_bytes, options)?;
                write_zip_file(&mut writer, &format!("{path}.spr"), &spr_bytes, options)?;
            }
            ArtifactBody::Blob { blob } => {
                let bytes = read_blob(&blob.hash)?;
                write_zip_file(&mut writer, &path, &bytes, options)?;
            }
        }
    }

    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
}

/// 📥️ Inverse of `export_collection_zip`. Parses `collection.collection.pack` first to learn the
/// folder tree/entries, then walks entries in the same id order to know exactly which zip paths to
/// read back — never guesses a layout from the zip's own directory listing.
pub fn import_collection_zip(bytes: &[u8]) -> Result<ImportedCollection, SpaceZipError> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
    let collection_pack = read_zip_entry(&mut archive, "collection.collection.pack")?;
    let collection_spr = read_zip_entry(&mut archive, "collection.collection.spr")?;
    let collection = <CollectionSnapshot as store::ArtifactPack>::decode_pack(&collection_pack).map_err(|error| SpaceZipError::Pack(error.to_string()))?;

    let mut artifacts = Vec::new();
    let mut blobs = Vec::new();
    let mut entries: Vec<&CollectionEntry> = collection.entries.iter().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    for entry in entries {
        let path = entry_path(&collection, &entry.id).ok_or_else(|| SpaceZipError::MissingPath(entry.id.clone()))?;
        match entry.body.as_ref() {
            ArtifactBody::Document { .. } => {
                let pack_bytes = read_zip_entry(&mut archive, &format!("{path}.pack"))?;
                let spr_bytes = read_zip_entry(&mut archive, &format!("{path}.spr"))?;
                artifacts.push((entry.clone(), pack_bytes, spr_bytes));
            }
            ArtifactBody::Blob { blob } => {
                let raw = read_zip_entry(&mut archive, &path)?;
                blobs.push((blob.clone(), raw));
            }
        }
    }

    Ok(ImportedCollection { collection, collection_spr, artifacts, blobs })
}

//#region 🔖️ZipStoreBridge
/// 🌉️ Real (non-mock) reader/writer bridge from `export_collection_zip`/`import_collection_zip`'s
/// injected-callback shape to the actual `store` crate types (`store::ArtifactStore`/
/// `store::ArtifactPackFiles`/`store::BlobStore`). This crate depends on `store` (see this crate's
/// `Cargo.toml`), never the reverse, so the bridge lives HERE rather than inside `store` — the
/// direction that keeps the dependency graph acyclic; `store` itself stays app-agnostic and never
/// names a collection/space type. W2 shipped `export_collection_zip`/`import_collection_zip` IO-free
/// with caller-injected reader closures and only fixture-string-backed unit tests exercising them;
/// this region is what a real caller (a live `store::SpaceHost`'s registered members, W4's storage
/// wave) plugs into those closures so a real collection with real document/blob artifacts actually
/// round-trips through a real `.zip` byte stream.
///
/// 📤️ EXPORT side: a caller snapshots each open document artifact's `store::ArtifactStore<P,
/// Mutation>` via its own `snapshot_pack()` into a `document_id -> store::ArtifactPackFiles` table,
/// then hands `real_artifact_reader`/`real_blob_reader` (closures over that table and a live
/// `store::BlobStore`) straight to `export_collection_zip`.
///
/// 📥️ IMPORT side: `import_document_artifact`/`import_blob` are the inverse — reconstructing a real
/// `store::ArtifactStore<P, Mutation>` from one `ImportedCollection::artifacts` entry's pack+spr
/// bytes (generic over the artifact's own concrete schema, mirroring `store::parse_document_pack`'s
/// own genericity — this crate never knows a document artifact's concrete type, only its `schema`
/// string, so the caller supplies `P`/`Mutation` at the call site), and re-`put`-ing one imported
/// blob's bytes into a live `store::BlobStore`, verifying the freshly computed content hash still
/// matches the `store::BlobRef` recorded in the collection.
pub fn real_artifact_reader(pack_files: &HashMap<String, store::ArtifactPackFiles>) -> impl Fn(&str) -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> + '_ {
    move |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> {
        let files = pack_files.get(entry_id).ok_or_else(|| SpaceZipError::MissingPath(entry_id.to_string()))?;
        Ok((files.pack.clone(), files.spr.clone()))
    }
}

pub fn real_blob_reader(blob_store: &dyn store::BlobStore) -> impl Fn(&str) -> Result<Vec<u8>, SpaceZipError> + '_ {
    move |hash: &str| -> Result<Vec<u8>, SpaceZipError> {
        blob_store.get(hash).map_err(|error| SpaceZipError::Pack(error.to_string()))?.ok_or_else(|| SpaceZipError::MissingPath(hash.to_string()))
    }
}

/// 📥️ Reconstructs a real `store::ArtifactStore<P, Mutation>` from one imported document artifact's
/// pack+spr bytes — the import-side counterpart to snapshotting it for `real_artifact_reader`.
pub fn import_document_artifact<P, Mutation>(pack_bytes: &[u8], spr_bytes: &[u8]) -> Result<store::ArtifactStore<P, Mutation>, SpaceZipError>
where
    P: Clone + Serialize + serde::de::DeserializeOwned + store::ArtifactPack,
    Mutation: Clone + Serialize + serde::de::DeserializeOwned + protocol::Mutation<P> + protocol::OpBinary + protocol::OpText,
{
    let parsed = store::parse_document_pack::<P, Mutation>(pack_bytes, spr_bytes).map_err(|error| SpaceZipError::Pack(error.to_string()))?;
    Ok(store::ArtifactStore::new(parsed.envelope))
}

/// 📥️ Puts one imported blob's bytes into a live `store::BlobStore`, verifying the freshly computed
/// content hash matches the `store::BlobRef` recorded in the collection (a mismatch means the zip was
/// tampered with or corrupted in transit).
pub fn import_blob(blob_store: &dyn store::BlobStore, blob: &store::BlobRef, bytes: Vec<u8>) -> Result<(), SpaceZipError> {
    let stored = blob_store.put(&bytes, &blob.media_type).map_err(|error| SpaceZipError::Pack(error.to_string()))?;
    if stored.hash != blob.hash {
        return Err(SpaceZipError::Pack(format!("blob hash mismatch on import: expected {}, got {}", blob.hash, stored.hash)));
    }
    Ok(())
}
//#endregion 🔖️ZipStoreBridge
//#endregion 🔖️Zip

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::DiffCodec;
    use store::{BlobStore, ArtifactDsl};

    //#region 🧸️Fixtures
    fn demo_user(id: &str, role: SpaceRole) -> SpaceUser {
        SpaceUser { id: id.into(), name: format!("User {id}"), avatar: None, role }
    }

    fn demo_extension(extension_id: &str, enabled: bool) -> InstalledExtension {
        InstalledExtension {
            extension_id: extension_id.into(),
            version: "1.0.0".into(),
            source_uri: format!("https://example.test/{extension_id}.sxt"),
            package_hash: format!("hash-{extension_id}"),
            enabled,
        }
    }

    fn demo_space() -> SpaceSnapshot {
        let mut space = empty_space_snapshot("Atelier Demo", SpaceKind::Atelier, SpaceVisibility::Private);
        space.users.push(demo_user("u1", SpaceRole::Author));
        space.collections.push(CollectionRef { id: "c1".into(), name: "Main".into(), document_id: "doc-c1".into() });
        space
    }

    fn demo_collection() -> CollectionSnapshot {
        let mut collection = empty_collection_snapshot("Main");
        collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: None, name: "Renders".into() });
        collection.entries.push(CollectionEntry {
            id: "e1".into(),
            folder_id: Some("f1".into()),
            name: "sketch".into(),
            kind_id: "puzzle.2d".into(),
            body: Box::new(ArtifactBody::Document { schema: "s.puzzle2d".into(), document_id: "doc-e1".into() }),
        });
        collection.entries.push(CollectionEntry {
            id: "e2".into(),
            folder_id: None,
            name: "reference.png".into(),
            kind_id: "file.blob".into(),
            body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "blake3-deadbeef".into(), size: 42, media_type: "image/png".into() } }),
        });
        collection
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️SpaceDocumentLaws
    #[test]
    fn empty_space_snapshot_matches_schema() {
        let space = empty_space_snapshot("Demo", SpaceKind::Studio, SpaceVisibility::Public);
        assert_eq!(space.schema, S_SPACE_SCHEMA);
        assert!(space.users.is_empty());
        assert!(space.collections.is_empty());
    }

    #[test]
    fn space_snapshot_dsl_pack_round_trips() {
        store::test_support::assert_dsl_pack_equivalence(&demo_space());
    }

    #[test]
    fn space_default_example_dsl_round_trips() {
        let text = include_str!("../../📚️examples/🎬️demo.space");
        let parsed = <SpaceSnapshot as ArtifactDsl>::parse_dsl(text).expect("parse default .space example");
        store::test_support::assert_dsl_round_trip(&parsed);
    }
    //#endregion 🧪️SpaceDocumentLaws

    //#region 🧪️CollectionDocumentLaws
    #[test]
    fn empty_collection_snapshot_matches_schema() {
        let collection = empty_collection_snapshot("Demo");
        assert_eq!(collection.schema, S_COLLECTION_SCHEMA);
        assert!(collection.folders.is_empty());
        assert!(collection.entries.is_empty());
    }

    #[test]
    fn collection_projection_dsl_pack_round_trips() {
        store::test_support::assert_dsl_pack_equivalence(&demo_collection());
    }


    #[test]
    fn collection_envelope_id_is_two_dot_segments() {
        let id = <CollectionSnapshot as store::ArtifactDsl>::envelope_id();
        assert_eq!(id, "s.collection");
        assert_eq!(id.split('.').count(), 2, "SemioEnvelope::from_envelope_id requires plugin.artifact");
        let pack = store::ArtifactPack::encode_pack(&empty_collection_snapshot("test")).expect("encode");
        let decoded = <CollectionSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded.name, "test");
    }

    #[test]
    fn space_envelope_id_is_two_dot_segments() {
        let id = <SpaceSnapshot as store::ArtifactDsl>::envelope_id();
        assert_eq!(id, "s.space");
        assert_eq!(id.split('.').count(), 2, "SemioEnvelope::from_envelope_id requires plugin.artifact");
        let pack = store::ArtifactPack::encode_pack(&empty_space_snapshot("test", SpaceKind::Atelier, SpaceVisibility::Private)).expect("encode");
        let decoded = <SpaceSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded.name, "test");
    }

    #[test]
    fn collection_default_example_dsl_round_trips() {
        let text = include_str!("../../📚️examples/🎬️demo.collection");
        let parsed = <CollectionSnapshot as ArtifactDsl>::parse_dsl(text).expect("parse default .collection example");
        store::test_support::assert_dsl_round_trip(&parsed);
    }
    //#endregion 🧪️CollectionDocumentLaws

    //#region 🧪️SpaceMutationLaws
    #[test]
    fn space_operation_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetName { name: "Renamed".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetKind { kind: SpaceKind::Studio });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetVisibility { visibility: SpaceVisibility::Public });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::UpsertUser { user: demo_user("u2", SpaceRole::Spectator) });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::RemoveUser { user_id: "u2".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::AddCollection { collection: CollectionRef { id: "c2".into(), name: "Extra".into(), document_id: "doc-c2".into() } });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::RemoveCollection { collection_id: "c2".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::RenameCollection { collection_id: "c1".into(), name: "Renamed Collection".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::InstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::UninstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::InstallExtension {
            extension_id: "flow-math".into(),
            version: "1.0.0".into(),
            source_uri: "https://example.test/flow-math.sxt".into(),
            package_hash: "hash-flow-math".into(),
            enabled: true,
        });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::UninstallExtension { extension_id: "flow-math".into() });
        store::test_support::assert_op_line_round_trip(&SpaceMutation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
    }

    #[test]
    fn space_operation_backwards_restores_pre_state() {
        let base = demo_space();
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::SetName { name: "New Name".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::UpsertUser { user: demo_user("u2", SpaceRole::Author) });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::UpsertUser { user: demo_user("u1", SpaceRole::Spectator) });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::RemoveUser { user_id: "u1".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::AddCollection { collection: CollectionRef { id: "c2".into(), name: "Extra".into(), document_id: "doc-c2".into() } });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::RemoveCollection { collection_id: "c1".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::RenameCollection { collection_id: "c1".into(), name: "Renamed".into() });
        store::test_support::assert_operation_round_trip(&base, SpaceMutation::InstallProgram { plugin_id: "cad".into() });
        let mut with_program = base.clone();
        with_program.programs.push("cad".into());
        store::test_support::assert_operation_round_trip(&with_program, SpaceMutation::UninstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_operation_round_trip(
            &base,
            SpaceMutation::InstallExtension {
                extension_id: "flow-math".into(),
                version: "1.0.0".into(),
                source_uri: "https://example.test/flow-math.sxt".into(),
                package_hash: "hash-flow-math".into(),
                enabled: true,
            },
        );
        let mut with_extension = base.clone();
        with_extension.extensions.push(demo_extension("flow-math", true));
        store::test_support::assert_operation_round_trip(&with_extension, SpaceMutation::UninstallExtension { extension_id: "flow-math".into() });
        store::test_support::assert_operation_round_trip(&with_extension, SpaceMutation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
        store::test_support::assert_operation_round_trip(
            &with_extension,
            SpaceMutation::InstallExtension {
                extension_id: "flow-math".into(),
                version: "2.0.0".into(),
                source_uri: "https://example.test/flow-math-v2.sxt".into(),
                package_hash: "hash-flow-math-v2".into(),
                enabled: false,
            },
        );
    }

    #[test]
    fn space_diff_print_parse_and_encode_decode_round_trip() {
        let diffs = vec![
            SpaceDiff { name: Some("Renamed".into()), ..Default::default() },
            SpaceDiff { upsert_user: Some(demo_user("u2", SpaceRole::Author)), ..Default::default() },
            SpaceDiff { install_program: Some("cad".into()), ..Default::default() },
            SpaceDiff { uninstall_program: Some("cad".into()), ..Default::default() },
            SpaceDiff { install_extension: Some(demo_extension("flow-math", true)), ..Default::default() },
            SpaceDiff { uninstall_extension_id: Some("flow-math".into()), ..Default::default() },
            SpaceDiff { set_extension_enabled_id: Some("flow-math".into()), set_extension_enabled: Some(false), ..Default::default() },
            SpaceDiff::default(),
        ];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
            let parsed = SpaceDiff::parse_diff(&printed).unwrap_or_else(|error| panic!("parse_diff failed for {printed:?}: {error}"));
            assert_eq!(parsed, diff);
            let encoded = diff.encode_diff().expect("encode_diff");
            let decoded = SpaceDiff::decode_diff(&encoded).expect("decode_diff");
            assert_eq!(decoded, diff);
        }
    }
    //#endregion 🧪️SpaceMutationLaws

    //#region 🧪️CollectionMutationLaws
    #[test]
    fn collection_operation_op_text_round_trips_every_variant() {
        let folder = CollectionFolder { id: "f2".into(), parent_id: None, name: "Extra".into() };
        let entry = CollectionEntry { id: "e3".into(), folder_id: None, name: "extra".into(), kind_id: "puzzle.2d".into(), body: Box::new(ArtifactBody::Document { schema: "s.puzzle2d".into(), document_id: "doc-e3".into() }) };
        store::test_support::assert_op_line_round_trip(&CollectionMutation::SetName { name: "Renamed".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::AddFolder { folder: folder.clone(), at: 0 });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::RemoveFolder { folder_id: "f1".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveFolder { folder_id: "f1".into(), parent_id: Some("f2".into()) });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveFolder { folder_id: "f1".into(), parent_id: None });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameFolder { folder_id: "f1".into(), name: "Renders 2".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::AddEntry { entry: entry.clone(), at: 0 });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::RemoveEntry { entry_id: "e1".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveEntry { entry_id: "e1".into(), folder_id: None });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameEntry { entry_id: "e1".into(), name: "sketch 2".into() });
        store::test_support::assert_op_line_round_trip(&CollectionMutation::ReplaceEntryBody { entry_id: "e2".into(), body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h2".into(), size: 1, media_type: "image/png".into() } }) });
    }

    #[test]
    fn collection_operation_binary_matches_text() {
        store::test_support::assert_op_text_binary_equivalence(&CollectionMutation::SetName { name: "Renamed".into() });
        let entry = CollectionEntry { id: "e3".into(), folder_id: None, name: "extra".into(), kind_id: "puzzle.2d".into(), body: Box::new(ArtifactBody::Document { schema: "s.puzzle2d".into(), document_id: "doc-e3".into() }) };
        store::test_support::assert_op_text_binary_equivalence(&CollectionMutation::AddEntry { entry, at: 0 });
    }

    #[test]
    fn collection_operation_backwards_restores_pre_state() {
        let base = demo_collection();
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::SetName { name: "Renamed".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::AddFolder { folder: CollectionFolder { id: "f2".into(), parent_id: None, name: "Extra".into() }, at: 0 });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::RemoveFolder { folder_id: "f1".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::MoveFolder { folder_id: "f1".into(), parent_id: None });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameFolder { folder_id: "f1".into(), name: "Renders 2".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::RemoveEntry { entry_id: "e1".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::MoveEntry { entry_id: "e1".into(), folder_id: None });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameEntry { entry_id: "e1".into(), name: "sketch 2".into() });
        store::test_support::assert_operation_round_trip(&base, CollectionMutation::ReplaceEntryBody { entry_id: "e2".into(), body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h2".into(), size: 1, media_type: "image/png".into() } }) });
    }

    #[test]
    fn collection_diff_print_parse_and_encode_decode_round_trip() {
        let diffs = vec![CollectionDiff { name: Some("Renamed".into()), ..Default::default() }, CollectionDiff { remove_entry_id: Some("e1".into()), ..Default::default() }, CollectionDiff::default()];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'));
            let parsed = CollectionDiff::parse_diff(&printed).unwrap_or_else(|error| panic!("parse_diff failed for {printed:?}: {error}"));
            assert_eq!(parsed, diff);
            let encoded = diff.encode_diff().expect("encode_diff");
            let decoded = CollectionDiff::decode_diff(&encoded).expect("decode_diff");
            assert_eq!(decoded, diff);
        }
    }
    //#endregion 🧪️CollectionMutationLaws

    //#region 🧪️RoleLaws
    #[test]
    fn can_write_follows_kind_and_role() {
        let mut archive = empty_space_snapshot("Frozen", SpaceKind::Archive, SpaceVisibility::Public);
        archive.users.push(demo_user("u1", SpaceRole::Author));
        assert!(!can_write(&archive, "u1"), "archive never accepts writes, even from an author");

        let mut studio = empty_space_snapshot("Studio", SpaceKind::Studio, SpaceVisibility::Private);
        studio.users.push(demo_user("u1", SpaceRole::Author));
        studio.users.push(demo_user("u2", SpaceRole::Spectator));
        assert!(can_write(&studio, "u1"));
        assert!(!can_write(&studio, "u2"));
        assert!(!can_write(&studio, "unknown"));
    }

    #[test]
    fn atelier_reconcile_keeps_a_single_author_by_smallest_id() {
        let mut atelier = empty_space_snapshot("Atelier", SpaceKind::Atelier, SpaceVisibility::Private);
        atelier.users.push(demo_user("u2", SpaceRole::Author));
        atelier.users.push(demo_user("u1", SpaceRole::Author));
        let (reconciled, reports) = reconcile_space_atelier_invariant(atelier);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].id, "space/atelier-multi-author");
        assert_eq!(space_role_of(&reconciled, "u1"), Some(SpaceRole::Author));
        assert_eq!(space_role_of(&reconciled, "u2"), Some(SpaceRole::Spectator));
    }

    #[test]
    fn atelier_reconcile_is_a_noop_with_a_single_author() {
        let (_, reports) = reconcile_space_atelier_invariant(demo_space());
        assert!(reports.is_empty());
    }
    //#endregion 🧪️RoleLaws

    //#region 🧪️CollectionReconcileLaws
    #[test]
    fn reconcile_reparents_orphan_folder_to_root() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: Some("missing".into()), name: "Orphan".into() });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.id == "collection/folder-orphaned"));
        assert_eq!(reconciled.folders[0].parent_id, None);
    }

    #[test]
    fn reconcile_cuts_folder_cycle() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.folders.push(CollectionFolder { id: "a".into(), parent_id: Some("b".into()), name: "A".into() });
        collection.folders.push(CollectionFolder { id: "b".into(), parent_id: Some("a".into()), name: "B".into() });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.id == "collection/folder-cycle"));
        assert!(reconciled.folders.iter().all(|f| f.parent_id.is_none()), "both cyclic folders must be cut to root");
    }

    #[test]
    fn reconcile_suffixes_duplicate_sibling_folder_names() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: None, name: "Renders".into() });
        collection.folders.push(CollectionFolder { id: "f2".into(), parent_id: None, name: "Renders".into() });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.id == "collection/folder-name-collision"));
        let names: Vec<&str> = reconciled.folders.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["Renders", "Renders (2)"]);
    }

    #[test]
    fn reconcile_reparents_entry_pointing_at_missing_folder() {
        let mut collection = empty_collection_snapshot("Demo");
        collection.entries.push(CollectionEntry {
            id: "e1".into(),
            folder_id: Some("missing".into()),
            name: "sketch".into(),
            kind_id: "puzzle.2d".into(),
            body: Box::new(ArtifactBody::Document { schema: "s.puzzle2d".into(), document_id: "doc-e1".into() }),
        });
        let (reconciled, reports) = reconcile_collection_integrity(collection);
        assert!(reports.iter().any(|r| r.id == "collection/entry-folder-missing"));
        assert_eq!(reconciled.entries[0].folder_id, None);
    }
    //#endregion 🧪️CollectionReconcileLaws

    //#region 🧪️PathResolverLaws
    #[test]
    fn folder_and_entry_path_round_trip() {
        let collection = demo_collection();
        assert_eq!(folder_path(&collection, "f1"), Some("Renders".into()));
        assert_eq!(entry_path(&collection, "e1"), Some("Renders/sketch".into()));
        assert_eq!(entry_path(&collection, "e2"), Some("reference.png".into()));
        assert_eq!(resolve_entry_by_path(&collection, "Renders/sketch").map(|entry| entry.id.as_str()), Some("e1"));
        assert_eq!(resolve_entry_by_path(&collection, "reference.png").map(|entry| entry.id.as_str()), Some("e2"));
        assert_eq!(resolve_entry_by_path(&collection, "nowhere"), None);
    }

    #[test]
    fn moves_and_renames_never_break_id_based_refs() {
        let mut collection = demo_collection();
        let before_id = collection.entries[0].id.clone();
        // Rename the folder — the path changes, the id-based ref doesn't.
        collection.folders[0].name = "Outputs".into();
        assert_eq!(entry_path(&collection, &before_id), Some("Outputs/sketch".into()));
        assert!(collection.entries.iter().any(|entry| entry.id == before_id));
    }

    #[test]
    fn backbone_uris_are_stable() {
        assert_eq!(space_backbone_uri("space-1"), "space://space-1");
        assert_eq!(collection_backbone_uri("space-1", "col-1"), "space://space-1/collection/col-1");
        assert_eq!(artifact_backbone_uri("space-1", "art-1"), "space://space-1/artifact/art-1");
    }
    //#endregion 🧪️PathResolverLaws

    //#region 🧪️DraftLaws
    fn memory_draft_port() -> Arc<dyn SpaceBackbonePort> {
        Arc::new(store::MemoryBackbonePort::new())
    }

    #[test]
    fn draft_create_list_expire_lifecycle() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft_a = catalog.create_draft("puzzle.2d", "s.puzzle2d", "sketch-a", 1_000, Some(500));
        let draft_b = catalog.create_draft("puzzle.2d", "s.puzzle2d", "sketch-b", 1_000, None);
        assert_eq!(draft_a.expires_at_ms, Some(1_500));
        assert_eq!(draft_b.expires_at_ms, None, "None ttl means pinned");

        let listed = catalog.list_drafts();
        assert_eq!(listed.len(), 2);

        let expired = catalog.expire_drafts(1_400, &port);
        assert!(expired.is_empty(), "not yet expired");
        let expired = catalog.expire_drafts(1_500, &port);
        assert_eq!(expired, vec![draft_a.artifact_id.clone()]);
        assert_eq!(catalog.list_drafts().len(), 1, "the pinned draft survives");
    }

    #[test]
    fn list_drafts_sweeping_expired_removes_stale_entries_first() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "s.puzzle2d", "stale", 0, Some(100));
        assert_eq!(catalog.list_drafts_sweeping_expired(50, &port).len(), 1, "not yet expired");
        assert!(catalog.list_drafts_sweeping_expired(200, &port).is_empty(), "swept before listing");
        assert!(catalog.list_drafts().iter().all(|entry| entry.artifact_id != draft.artifact_id));
    }

    #[test]
    fn discard_draft_removes_bookkeeping_and_tombstones_bytes() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "s.puzzle2d", "scratch", 0, None);
        port.write(&draft_uri(&draft.artifact_id), b"draft-bytes").expect("seed draft bytes");

        let removed = catalog.discard_draft(&port, &draft.artifact_id).expect("discard");
        assert_eq!(removed.artifact_id, draft.artifact_id);
        assert!(catalog.list_drafts().is_empty());
        assert_eq!(port.read(&draft_uri(&draft.artifact_id)).expect("read tombstone"), Vec::<u8>::new());
        assert!(catalog.discard_draft(&port, &draft.artifact_id).is_none(), "already discarded");
    }

    /// 🧪️ The plan's core promotion invariant, proven with REAL bytes (not a fixture string): the
    /// artifact's envelope bytes at `artifact_backbone_uri` after promotion are byte-for-byte IDENTICAL
    /// to what was at `draft_uri` before promotion — just relocated under a different backbone uri, no
    /// decode/re-encode anywhere in the path.
    #[test]
    fn draft_promote_moves_envelope_bytes_byte_identical() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "s.puzzle2d", "sketch", 0, None);
        let original_bytes = b"pretend-pack-plus-spr-envelope-bytes-with-full-vcs-history".to_vec();
        port.write(&draft_uri(&draft.artifact_id), &original_bytes).expect("seed draft bytes");

        let (removed_draft, operation) = catalog.promote_draft(&port, "space-1", &draft.artifact_id, Some("f1".into())).expect("promote");
        assert_eq!(removed_draft.artifact_id, draft.artifact_id);
        let CollectionMutation::AddEntry { entry, .. } = &operation else { panic!("promote_draft must return AddEntry") };
        assert_eq!(entry.id, draft.artifact_id, "promotion preserves the document id");
        assert_eq!(entry.folder_id, Some("f1".into()));
        assert!(catalog.list_drafts().is_empty(), "promoted draft is no longer a draft");

        let moved_bytes = port.read(&artifact_backbone_uri("space-1", &draft.artifact_id)).expect("read promoted bytes");
        assert_eq!(moved_bytes, original_bytes, "promoted envelope bytes are byte-identical, just at a different backbone uri");
        assert_eq!(port.read(&draft_uri(&draft.artifact_id)).expect("read tombstoned draft uri"), Vec::<u8>::new(), "draft uri is tombstoned after promotion");

        let demote = DraftCatalog::demote_operation(&entry.id);
        assert_eq!(demote, CollectionMutation::RemoveEntry { entry_id: entry.id.clone() });

        // Mutation-sourced round trip: AddEntry then its inverse restores the empty collection.
        let empty = empty_collection_snapshot("Demo");
        store::test_support::assert_operation_round_trip(&empty, operation);
    }

    /// 🧪️ `demote_asset` is `promote_draft`'s real byte-moving inverse: the SAME bytes travel back
    /// from the asset uri to the draft uri, byte-identical, and fresh draft bookkeeping reappears.
    #[test]
    fn demote_asset_moves_bytes_back_and_reregisters_draft_bookkeeping() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        let draft = catalog.create_draft("puzzle.2d", "s.puzzle2d", "sketch", 0, None);
        let original_bytes = b"envelope-bytes-round-tripping-through-promote-then-demote".to_vec();
        port.write(&draft_uri(&draft.artifact_id), &original_bytes).expect("seed draft bytes");

        let (_, operation) = catalog.promote_draft(&port, "space-1", &draft.artifact_id, None).expect("promote");
        let CollectionMutation::AddEntry { entry, .. } = operation else { panic!("expected AddEntry") };

        let demote_operation = catalog.demote_asset(&port, "space-1", &entry, "puzzle.2d", "s.puzzle2d", 2_000, Some(1_000)).expect("demote");
        assert_eq!(demote_operation, CollectionMutation::RemoveEntry { entry_id: entry.id.clone() });

        let restored_bytes = port.read(&draft_uri(&entry.id)).expect("read demoted draft bytes");
        assert_eq!(restored_bytes, original_bytes, "demoted envelope bytes are byte-identical to the originally-promoted ones");
        assert_eq!(port.read(&artifact_backbone_uri("space-1", &entry.id)).expect("read tombstoned asset uri"), Vec::<u8>::new());

        let redrafted = catalog.list_drafts();
        assert_eq!(redrafted.len(), 1);
        assert_eq!(redrafted[0].artifact_id, entry.id);
        assert_eq!(redrafted[0].expires_at_ms, Some(3_000), "demotion re-registers a fresh TTL window");
    }

    #[test]
    fn promote_unknown_draft_errors() {
        let catalog = DraftCatalog::new();
        let port = memory_draft_port();
        assert_eq!(catalog.promote_draft(&port, "space-1", "nope", None), Err(SpaceError::UnknownDraft("nope".into())));
    }

    /// 🧪️ `draft_catalog_for` is the port-keyed global registry: the SAME `Arc<dyn SpaceBackbonePort>`
    /// identity always resolves to the SAME `DraftCatalog` instance (so callers sharing a port share
    /// draft bookkeeping), while two DISTINCT port identities never share one.
    #[test]
    fn draft_catalog_for_is_keyed_by_port_identity() {
        let port_a = memory_draft_port();
        let port_b = memory_draft_port();

        let catalog_a1 = draft_catalog_for(&port_a);
        let catalog_a1_created = catalog_a1.create_draft("puzzle.2d", "s.puzzle2d", "shared", 0, None);
        let catalog_a2 = draft_catalog_for(&port_a);
        assert_eq!(catalog_a2.list_drafts().iter().map(|entry| entry.artifact_id.clone()).collect::<Vec<_>>(), vec![catalog_a1_created.artifact_id.clone()], "same port identity shares one catalog");

        let catalog_b = draft_catalog_for(&port_b);
        assert!(catalog_b.list_drafts().is_empty(), "a distinct port identity gets its own catalog");
    }
    //#endregion 🧪️DraftLaws

    //#region 🧪️ZipLaws
    fn zip_fixture_bytes() -> Vec<u8> {
        let collection = demo_collection();
        let read_artifact = |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> { Ok((format!("pack-bytes-for-{entry_id}").into_bytes(), format!("spr-bytes-for-{entry_id}").into_bytes())) };
        let read_blob = |hash: &str| -> Result<Vec<u8>, SpaceZipError> { Ok(format!("blob-bytes-for-{hash}").into_bytes()) };
        export_collection_zip(&collection, b"collection-spr-bytes", &read_artifact, &read_blob).expect("export")
    }

    #[test]
    fn zip_export_import_round_trips_structure_and_bytes() {
        let bytes = zip_fixture_bytes();
        let imported = import_collection_zip(&bytes).expect("import");
        assert_eq!(imported.collection, demo_collection());
        assert_eq!(imported.collection_spr, b"collection-spr-bytes");
        assert_eq!(imported.artifacts.len(), 1, "one document entry");
        let (entry, pack_bytes, spr_bytes) = &imported.artifacts[0];
        assert_eq!(entry.id, "e1");
        assert_eq!(pack_bytes, b"pack-bytes-for-e1");
        assert_eq!(spr_bytes, b"spr-bytes-for-e1");
        assert_eq!(imported.blobs.len(), 1, "one blob entry");
        assert_eq!(imported.blobs[0].0.hash, "blake3-deadbeef");
        assert_eq!(imported.blobs[0].1, b"blob-bytes-for-blake3-deadbeef");
    }

    #[test]
    fn zip_export_import_export_is_byte_stable() {
        let once = zip_fixture_bytes();
        let imported = import_collection_zip(&once).expect("import");
        let read_artifact = |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> {
            let (_, pack_bytes, spr_bytes) = imported.artifacts.iter().find(|(entry, _, _)| entry.id == entry_id).expect("artifact bytes");
            Ok((pack_bytes.clone(), spr_bytes.clone()))
        };
        let read_blob = |hash: &str| -> Result<Vec<u8>, SpaceZipError> {
            let (_, bytes) = imported.blobs.iter().find(|(blob, _)| blob.hash == hash).expect("blob bytes");
            Ok(bytes.clone())
        };
        let twice = export_collection_zip(&imported.collection, &imported.collection_spr, &read_artifact, &read_blob).expect("re-export");
        assert_eq!(once, twice, "export -> import -> export must be byte-stable");
    }
    //#endregion 🧪️ZipLaws

    //#region 🧪️ZipStoreBridgeLaws
    /// 🧪️ Minimal in-memory `store::BlobStore` test double — content-addressed via a fast
    /// non-cryptographic hash (no need for `framework_hash`'s real Blake3, this crate has no such
    /// dependency and a test double only needs internal consistency, not a production hash).
    #[derive(Default)]
    struct TestBlobStore {
        entries: Mutex<HashMap<String, (Vec<u8>, String)>>,
    }

    fn test_blob_hash(bytes: &[u8]) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        format!("test-{:016x}", hasher.finish())
    }

    impl BlobStore for TestBlobStore {
        fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
            let hash = test_blob_hash(bytes);
            self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(hash.clone(), (bytes.to_vec(), media_type.to_string()));
            Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
        }

        fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
            Ok(self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(hash).map(|(bytes, _)| bytes.clone()))
        }

        fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
            Ok(self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(hash))
        }

        fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
            self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(hash);
            Ok(())
        }
    }

    /// 🧪️ End-to-end law with REAL `store` types (not the fixture-string readers `zip_fixture_bytes`
    /// injects above): a real `store::ArtifactStore<SpaceSnapshot, SpaceMutation>` document
    /// artifact (itself an ordinary `#[derive(dsl::DslArtifact)]`/`#[derive(dsl::DslOps)]` document —
    /// exercising exactly the same `ArtifactPack`/`OpBinary`/`OpText` machinery any real app document
    /// would) plus a real blob round-trip through `real_artifact_reader`/`real_blob_reader`/
    /// `import_document_artifact`/`import_blob`, asserting the round trip preserves collection
    /// structure AND artifact envelope bytes byte-for-byte (the plan's "lossless" requirement), and
    /// that export->import->export stays byte-stable with real data too.
    #[test]
    fn zip_export_import_round_trips_real_store_documents_and_blob() {
        let mut nested_space_store = store::ArtifactStore::new(store::create_document_envelope::<SpaceSnapshot, SpaceMutation>(S_SPACE_SCHEMA, "art-nested-space", demo_space(), None));
        nested_space_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![SpaceMutation::SetName { name: "Nested Space".into() }], description: None }).expect("apply");
        nested_space_store.dispatch(store::ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }).expect("commit checkpoint");
        let original_pack_files = nested_space_store.snapshot_pack().expect("snapshot pack");

        let blob_store = TestBlobStore::default();
        let blob_ref = blob_store.put(b"hello blob bytes", "text/plain").expect("put blob");

        let mut collection = empty_collection_snapshot("RealDemo");
        collection.entries.push(CollectionEntry {
            id: "art-nested-space".into(),
            folder_id: None,
            name: "nested-space".into(),
            kind_id: "s.space".into(),
            body: Box::new(ArtifactBody::Document { schema: S_SPACE_SCHEMA.into(), document_id: "art-nested-space".into() }),
        });
        collection.entries.push(CollectionEntry { id: "blob-1".into(), folder_id: None, name: "note.txt".into(), kind_id: "file.blob".into(), body: Box::new(ArtifactBody::Blob { blob: blob_ref.clone() }) });

        let mut pack_files = HashMap::new();
        pack_files.insert("art-nested-space".to_string(), original_pack_files.clone());
        let read_artifact = real_artifact_reader(&pack_files);
        let read_blob = real_blob_reader(&blob_store);
        let collection_spr = b"collection-history-bytes".to_vec();
        let zip_bytes = export_collection_zip(&collection, &collection_spr, &read_artifact, &read_blob).expect("export");

        let imported = import_collection_zip(&zip_bytes).expect("import");
        assert_eq!(imported.collection, collection, "collection structure survives the round trip");
        assert_eq!(imported.collection_spr, collection_spr);

        let (_, imported_pack, imported_spr) = imported.artifacts.iter().find(|(entry, _, _)| entry.id == "art-nested-space").expect("artifact present");
        assert_eq!(imported_pack, &original_pack_files.pack, "artifact pack bytes are byte-identical after the round trip");
        assert_eq!(imported_spr, &original_pack_files.spr, "artifact spr bytes are byte-identical after the round trip");

        let restored_store = import_document_artifact::<SpaceSnapshot, SpaceMutation>(imported_pack, imported_spr).expect("reconstruct store");
        assert_eq!(restored_store.snapshot().expect("projection"), nested_space_store.snapshot().expect("projection"), "reconstructed document projection matches the original exactly");

        let (imported_blob, imported_blob_bytes) = imported.blobs.iter().find(|(blob, _)| blob.hash == blob_ref.hash).expect("blob present");
        assert_eq!(imported_blob_bytes, b"hello blob bytes");
        let fresh_blob_store = TestBlobStore::default();
        import_blob(&fresh_blob_store, imported_blob, imported_blob_bytes.clone()).expect("import blob");
        assert_eq!(fresh_blob_store.get(&blob_ref.hash).expect("get"), Some(b"hello blob bytes".to_vec()));

        // export -> import -> export must stay byte-stable with REAL data too (not just injected
        // fixture strings) — the law `zip_export_import_export_is_byte_stable` proved with mock bytes
        // holds end-to-end.
        let mut reexport_pack_files = HashMap::new();
        reexport_pack_files.insert("art-nested-space".to_string(), store::ArtifactPackFiles { pack: imported_pack.clone(), spr: imported_spr.clone(), ops: String::new() });
        let re_read_artifact = real_artifact_reader(&reexport_pack_files);
        let re_read_blob = real_blob_reader(&fresh_blob_store);
        let twice = export_collection_zip(&imported.collection, &imported.collection_spr, &re_read_artifact, &re_read_blob).expect("re-export");
        assert_eq!(zip_bytes, twice, "export -> import -> export is byte-stable with real store-backed data");
    }
    //#endregion 🧪️ZipStoreBridgeLaws
}
//#endregion 🧪️Tests
