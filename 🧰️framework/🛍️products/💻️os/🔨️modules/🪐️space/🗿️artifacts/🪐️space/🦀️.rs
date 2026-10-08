//! 🪐️ Composable persisted space manifest artifact.

#[cfg(test)]
#[path="../../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

#[path = "♻️retirement/🦀️.rs"]
mod retirement;
pub use io::sqlite::snapshot::{register_sqlite_snapshot,SQLITE_SNAPSHOT_DIALECT};

use serde::{Deserialize, Serialize};

//#region 🔖️Roles
/// 🏛️ A space's collaboration shape: `Atelier` (single-writer personal, reconcile-enforced exactly
/// one `Author`), `Studio` (multi-writer group, any number of `Author`s), `Archive` (frozen, nobody
/// writes).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
pub enum SpaceKind {
    Atelier,
    Studio,
    Archive,
}

/// 👁️ Whether a space is discoverable/readable by an anonymous visitor (`Public`, implicit anonymous
/// spectator — wired at the hub layer in W4) or membership-gated (`Private`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
pub enum SpaceVisibility {
    Private,
    Public,
}

/// 🧑️‍🤝️‍🧑️ A space member's permission level: `Author` (read-write) or `Spectator` (read-only). The
/// hub directory (`🌎️hub/🔨️modules/📇️directory`) re-declares this enum string-identically
/// (`"author"`/`"spectator"`, see `as_str`/`parse`) since it cannot depend on this wasm-facing crate —
/// keep the two in lockstep by hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
pub const S_SPACE_SCHEMA: &str = "os.space";

/// 🔗️ One entry in `SpaceSnapshot.collections` — the collection's identity, display name, and the
/// `os.collection` document id it addresses (see `🔖️Addressing` in the plan: `CollectionEntry.id ==
/// artifact id == ArtifactEnvelope.id` for document artifacts; a `CollectionRef` follows the same
/// convention one level up).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[artifact(id = "os.space")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InstalledExtension {
    pub extension_id: String,
    pub version: String,
    pub source_uri: String,
    pub package_hash: String,
    pub enabled: bool,
}

pub fn empty_space_snapshot(name: &str, kind: SpaceKind, visibility: SpaceVisibility) -> SpaceSnapshot {
    SpaceSnapshot { schema: S_SPACE_SCHEMA.into(), name: name.into(), kind, visibility, users: Vec::new(), collections: Vec::new(), programs: Vec::new(), extensions: Vec::new() }
}

//#region 🔖️SpaceMutation
/// ⚡️ One settled space-manifest mutation. Every variant's op keyword is the auto-derived kebab-case
/// of its own name (`UpsertUser` -> `upsert-user`, ...) — see [`protocol::OpText`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
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
        index: Option<u32>,
    },
    RemoveUser {
        user_id: String,
    },
    AddCollection {
        #[dsl(block)]
        collection: CollectionRef,
        index: Option<u32>,
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
        index: Option<u32>,
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
        index: Option<u32>,
    },
    UninstallExtension {
        extension_id: String,
    },
    SetExtensionEnabled {
        extension_id: String,
        enabled: bool,
    },
}

//#region 🔖️HandcraftedOpCodecs



//#endregion 🔖️HandcraftedOpCodecs

/// 🔺️ Sparse space-manifest delta: the scalar fields are present slots, and every member collection is an id-keyed row delta
/// (`added`/`removed`/`patched`/`reordered`) so any number of users, collections, programs and extensions change in one diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind: Option<SpaceKind>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<SpaceVisibility>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub users: Option<SpaceUsersDelta>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub collections: Option<SpaceCollectionsDelta>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub programs: Option<SpaceProgramsDelta>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<SpaceExtensionsDelta>,
}

/// 🧱️ Carries the optional member avatar as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceOptionalAvatar {
    pub value: Option<String>,
}

/// 🩹 Field patch of one member; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceUserPatch {
    pub id: String,
    pub name: Option<String>,
    pub avatar: Option<SpaceOptionalAvatar>,
    pub role: Option<SpaceRole>,
}

/// 🩹 Field patch of one collection reference.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceCollectionPatch {
    pub id: String,
    pub name: Option<String>,
    pub document_id: Option<String>,
}

/// 🩹 The (always empty) patch of an installed program, which is a bare id.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceProgramPatch {
    pub id: String,
}

/// 🩹 Field patch of one installed extension.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceExtensionPatch {
    pub extension_id: String,
    pub version: Option<String>,
    pub source_uri: Option<String>,
    pub package_hash: Option<String>,
    pub enabled: Option<bool>,
}

/// 🧩️ Id-keyed row delta of the members.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceUsersDelta {
    pub added: Vec<SpaceUser>,
    pub removed: Vec<String>,
    pub patched: Vec<SpaceUserPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩️ Id-keyed row delta of the collections.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceCollectionsDelta {
    pub added: Vec<CollectionRef>,
    pub removed: Vec<String>,
    pub patched: Vec<SpaceCollectionPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩️ Id-keyed row delta of the installed programs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceProgramsDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub patched: Vec<SpaceProgramPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩️ Extension-keyed row delta of the installed extensions.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceExtensionsDelta {
    pub added: Vec<InstalledExtension>,
    pub removed: Vec<String>,
    pub patched: Vec<SpaceExtensionPatch>,
    pub reordered: Option<Vec<String>>,
}

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

macro_rules! keyed_delta_impl {
    ($delta:ident, $row:ty, $patch:ty, $row_key:ident, $patch_key:ident, $fold:expr, $compose:expr, $inverse:expr, $patch_between:expr, $is_empty:expr) => {
        impl KeyedDelta for $delta {
            type Row = $row;
            type Patch = $patch;
            fn added(&self) -> &[$row] {
                &self.added
            }
            fn removed(&self) -> &[String] {
                &self.removed
            }
            fn patched(&self) -> &[$patch] {
                &self.patched
            }
            fn reordered(&self) -> Option<&[String]> {
                self.reordered.as_deref()
            }
            fn assemble(added: Vec<$row>, removed: Vec<String>, patched: Vec<$patch>, reordered: Option<Vec<String>>) -> Self {
                Self { added, removed, patched, reordered }
            }
            fn row_key(row: &$row) -> &str {
                row.$row_key.as_str()
            }
            fn patch_key(patch: &$patch) -> &str {
                patch.$patch_key.as_str()
            }
            fn patch_fold(patch: &$patch, row: &mut $row) -> Result<(), protocol::MutationApplyError> {
                ($fold)(patch, row);
                Ok(())
            }
            fn patch_compose(first: &$patch, later: &$patch) -> $patch {
                ($compose)(first, later)
            }
            fn patch_inverse(patch: &$patch, base: &$row) -> $patch {
                ($inverse)(patch, base)
            }
            fn patch_between(base: &$row, other: &$row) -> Option<$patch> {
                ($patch_between)(base, other)
            }
            fn patch_is_empty(patch: &$patch) -> bool {
                ($is_empty)(patch)
            }
        }
    };
}

keyed_delta_impl!(
    SpaceUsersDelta,
    SpaceUser,
    SpaceUserPatch,
    id,
    id,
    |patch: &SpaceUserPatch, row: &mut SpaceUser| {
        if let Some(name) = &patch.name {
            row.name = name.clone();
        }
        if let Some(avatar) = &patch.avatar {
            row.avatar = avatar.value.clone();
        }
        if let Some(role) = patch.role {
            row.role = role;
        }
    },
    |first: &SpaceUserPatch, later: &SpaceUserPatch| SpaceUserPatch { id: first.id.clone(), name: later.name.clone().or_else(|| first.name.clone()), avatar: later.avatar.clone().or_else(|| first.avatar.clone()), role: later.role.or(first.role) },
    |patch: &SpaceUserPatch, base: &SpaceUser| SpaceUserPatch { id: patch.id.clone(), name: patch.name.as_ref().map(|_| base.name.clone()), avatar: patch.avatar.as_ref().map(|_| SpaceOptionalAvatar { value: base.avatar.clone() }), role: patch.role.map(|_| base.role) },
    |base: &SpaceUser, other: &SpaceUser| {
        let patch = SpaceUserPatch { id: other.id.clone(), name: (base.name != other.name).then(|| other.name.clone()), avatar: (base.avatar != other.avatar).then(|| SpaceOptionalAvatar { value: other.avatar.clone() }), role: (base.role != other.role).then_some(other.role) };
        (patch.name.is_some() || patch.avatar.is_some() || patch.role.is_some()).then_some(patch)
    },
    |patch: &SpaceUserPatch| patch.name.is_none() && patch.avatar.is_none() && patch.role.is_none()
);

keyed_delta_impl!(
    SpaceCollectionsDelta,
    CollectionRef,
    SpaceCollectionPatch,
    id,
    id,
    |patch: &SpaceCollectionPatch, row: &mut CollectionRef| {
        if let Some(name) = &patch.name {
            row.name = name.clone();
        }
        if let Some(document_id) = &patch.document_id {
            row.document_id = document_id.clone();
        }
    },
    |first: &SpaceCollectionPatch, later: &SpaceCollectionPatch| SpaceCollectionPatch { id: first.id.clone(), name: later.name.clone().or_else(|| first.name.clone()), document_id: later.document_id.clone().or_else(|| first.document_id.clone()) },
    |patch: &SpaceCollectionPatch, base: &CollectionRef| SpaceCollectionPatch { id: patch.id.clone(), name: patch.name.as_ref().map(|_| base.name.clone()), document_id: patch.document_id.as_ref().map(|_| base.document_id.clone()) },
    |base: &CollectionRef, other: &CollectionRef| {
        let patch = SpaceCollectionPatch { id: other.id.clone(), name: (base.name != other.name).then(|| other.name.clone()), document_id: (base.document_id != other.document_id).then(|| other.document_id.clone()) };
        (patch.name.is_some() || patch.document_id.is_some()).then_some(patch)
    },
    |patch: &SpaceCollectionPatch| patch.name.is_none() && patch.document_id.is_none()
);

impl KeyedDelta for SpaceProgramsDelta {
    type Row = String;
    type Patch = SpaceProgramPatch;
    fn added(&self) -> &[String] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[SpaceProgramPatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<String>, removed: Vec<String>, patched: Vec<SpaceProgramPatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &String) -> &str {
        row
    }
    fn patch_key(patch: &SpaceProgramPatch) -> &str {
        &patch.id
    }
    fn patch_fold(_patch: &SpaceProgramPatch, _row: &mut String) -> Result<(), protocol::MutationApplyError> {
        Ok(())
    }
    fn patch_compose(first: &SpaceProgramPatch, _later: &SpaceProgramPatch) -> SpaceProgramPatch {
        first.clone()
    }
    fn patch_inverse(patch: &SpaceProgramPatch, _base: &String) -> SpaceProgramPatch {
        patch.clone()
    }
    fn patch_between(_base: &String, _other: &String) -> Option<SpaceProgramPatch> {
        None
    }
    fn patch_is_empty(_patch: &SpaceProgramPatch) -> bool {
        true
    }
}

keyed_delta_impl!(
    SpaceExtensionsDelta,
    InstalledExtension,
    SpaceExtensionPatch,
    extension_id,
    extension_id,
    |patch: &SpaceExtensionPatch, row: &mut InstalledExtension| {
        if let Some(version) = &patch.version {
            row.version = version.clone();
        }
        if let Some(source_uri) = &patch.source_uri {
            row.source_uri = source_uri.clone();
        }
        if let Some(package_hash) = &patch.package_hash {
            row.package_hash = package_hash.clone();
        }
        if let Some(enabled) = patch.enabled {
            row.enabled = enabled;
        }
    },
    |first: &SpaceExtensionPatch, later: &SpaceExtensionPatch| SpaceExtensionPatch {
        extension_id: first.extension_id.clone(),
        version: later.version.clone().or_else(|| first.version.clone()),
        source_uri: later.source_uri.clone().or_else(|| first.source_uri.clone()),
        package_hash: later.package_hash.clone().or_else(|| first.package_hash.clone()),
        enabled: later.enabled.or(first.enabled),
    },
    |patch: &SpaceExtensionPatch, base: &InstalledExtension| SpaceExtensionPatch {
        extension_id: patch.extension_id.clone(),
        version: patch.version.as_ref().map(|_| base.version.clone()),
        source_uri: patch.source_uri.as_ref().map(|_| base.source_uri.clone()),
        package_hash: patch.package_hash.as_ref().map(|_| base.package_hash.clone()),
        enabled: patch.enabled.map(|_| base.enabled),
    },
    |base: &InstalledExtension, other: &InstalledExtension| {
        let patch = SpaceExtensionPatch {
            extension_id: other.extension_id.clone(),
            version: (base.version != other.version).then(|| other.version.clone()),
            source_uri: (base.source_uri != other.source_uri).then(|| other.source_uri.clone()),
            package_hash: (base.package_hash != other.package_hash).then(|| other.package_hash.clone()),
            enabled: (base.enabled != other.enabled).then_some(other.enabled),
        };
        (patch.version.is_some() || patch.source_uri.is_some() || patch.package_hash.is_some() || patch.enabled.is_some()).then_some(patch)
    },
    |patch: &SpaceExtensionPatch| patch.version.is_none() && patch.source_uri.is_none() && patch.package_hash.is_none() && patch.enabled.is_none()
);

/// 📍️ Complete order that places `id` at `index` among `ids`; `None` when the row simply appends.
fn insertion_order<'a>(ids: impl Iterator<Item = &'a str>, id: &str, index: Option<u32>) -> Option<Vec<String>> {
    let mut order: Vec<String> = ids.map(str::to_string).collect();
    let index = index? as usize;
    (index < order.len()).then(|| {
        order.insert(index, id.to_string());
        order
    })
}

impl protocol::MutationDiff<SpaceSnapshot> for SpaceDiff {
    fn apply(&self, base: &SpaceSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SpaceSnapshot> {
        let mut next = base.clone();
        if let Some(name) = &self.name {
            next.name = name.clone();
        }
        if let Some(kind) = self.kind {
            next.kind = kind;
        }
        if let Some(visibility) = self.visibility {
            next.visibility = visibility;
        }
        if let Some(delta) = &self.users {
            next.users = keyed_apply(&next.users, delta).map_err(|error| error.under(["users"]))?;
        }
        if let Some(delta) = &self.collections {
            next.collections = keyed_apply(&next.collections, delta).map_err(|error| error.under(["collections"]))?;
        }
        if let Some(delta) = &self.programs {
            next.programs = keyed_apply(&next.programs, delta).map_err(|error| error.under(["programs"]))?;
        }
        if let Some(delta) = &self.extensions {
            next.extensions = keyed_apply(&next.extensions, delta).map_err(|error| error.under(["extensions"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        macro_rules! compose {
            ($field:ident) => {
                self.$field = match (self.$field.take(), other.$field) {
                    (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
                    (first, later) => later.or(first),
                };
            };
        }
        take!(name);
        take!(kind);
        take!(visibility);
        compose!(users);
        compose!(collections);
        compose!(programs);
        compose!(extensions);
    }
}

impl protocol::DiffAlgebra<SpaceSnapshot> for SpaceDiff {
    fn inverse(&self, base: &SpaceSnapshot) -> Self {
        Self {
            name: self.name.as_ref().map(|_| base.name.clone()),
            kind: self.kind.map(|_| base.kind),
            visibility: self.visibility.map(|_| base.visibility),
            users: self.users.as_ref().map(|delta| keyed_inverse(delta, &base.users)),
            collections: self.collections.as_ref().map(|delta| keyed_inverse(delta, &base.collections)),
            programs: self.programs.as_ref().map(|delta| keyed_inverse(delta, &base.programs)),
            extensions: self.extensions.as_ref().map(|delta| keyed_inverse(delta, &base.extensions)),
        }
    }

    fn between(base: &SpaceSnapshot, other: &SpaceSnapshot) -> Self {
        let users = keyed_between::<SpaceUsersDelta>(&base.users, &other.users);
        let collections = keyed_between::<SpaceCollectionsDelta>(&base.collections, &other.collections);
        let programs = keyed_between::<SpaceProgramsDelta>(&base.programs, &other.programs);
        let extensions = keyed_between::<SpaceExtensionsDelta>(&base.extensions, &other.extensions);
        Self {
            name: (base.name != other.name).then(|| other.name.clone()),
            kind: (base.kind != other.kind).then_some(other.kind),
            visibility: (base.visibility != other.visibility).then_some(other.visibility),
            users: (!keyed_is_empty(&users)).then_some(users),
            collections: (!keyed_is_empty(&collections)).then_some(collections),
            programs: (!keyed_is_empty(&programs)).then_some(programs),
            extensions: (!keyed_is_empty(&extensions)).then_some(extensions),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_none() && self.kind.is_none() && self.visibility.is_none() && self.users.as_ref().is_none_or(keyed_is_empty) && self.collections.as_ref().is_none_or(keyed_is_empty) && self.programs.as_ref().is_none_or(keyed_is_empty) && self.extensions.as_ref().is_none_or(keyed_is_empty)
    }
}

/// 🧷️ Hand-built `protocol::Mutation::DESCRIPTORS` roster for `SpaceMutation`'s 13 leaves — not
/// `#[derive(dsl::Mutations)]` (that derive requires each variant to wrap a `MutationLeaf` payload;
/// `SpaceMutation`'s variants carry inline fields instead, same shape as `🌉️mcp/🏠️workspace`'s
/// hand-built `ProbeMutation` roster). One entry per variant, in declaration order.
const SPACE_MUTATION_DESCRIPTORS: &[protocol::MutationLeafDescriptor] = &[
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/set-name",
        semantic_kind: "set-name",
        display_name: "Set Name",
        emoji: "🏷️",
        aggregate_variant: "SetName",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("set-name"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/set-kind",
        semantic_kind: "set-kind",
        display_name: "Set Kind",
        emoji: "🏛️",
        aggregate_variant: "SetKind",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("set-kind"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/set-visibility",
        semantic_kind: "set-visibility",
        display_name: "Set Visibility",
        emoji: "👁️",
        aggregate_variant: "SetVisibility",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("set-visibility"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/upsert-user",
        semantic_kind: "upsert-user",
        display_name: "Upsert User",
        emoji: "🧑️",
        aggregate_variant: "UpsertUser",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("upsert-user"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/remove-user",
        semantic_kind: "remove-user",
        display_name: "Remove User",
        emoji: "🚪",
        aggregate_variant: "RemoveUser",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("remove-user"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/add-collection",
        semantic_kind: "add-collection",
        display_name: "Add Collection",
        emoji: "🗂️",
        aggregate_variant: "AddCollection",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("add-collection"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/remove-collection",
        semantic_kind: "remove-collection",
        display_name: "Remove Collection",
        emoji: "🗑️",
        aggregate_variant: "RemoveCollection",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("remove-collection"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/rename-collection",
        semantic_kind: "rename-collection",
        display_name: "Rename Collection",
        emoji: "✏️",
        aggregate_variant: "RenameCollection",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("rename-collection"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/install-program",
        semantic_kind: "install-program",
        display_name: "Install Program",
        emoji: "🔌",
        aggregate_variant: "InstallProgram",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("install-program"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/uninstall-program",
        semantic_kind: "uninstall-program",
        display_name: "Uninstall Program",
        emoji: "🔌",
        aggregate_variant: "UninstallProgram",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("uninstall-program"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/install-extension",
        semantic_kind: "install-extension",
        display_name: "Install Extension",
        emoji: "🧩",
        aggregate_variant: "InstallExtension",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("install-extension"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/uninstall-extension",
        semantic_kind: "uninstall-extension",
        display_name: "Uninstall Extension",
        emoji: "🧩",
        aggregate_variant: "UninstallExtension",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("uninstall-extension"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/set-extension-enabled",
        semantic_kind: "set-extension-enabled",
        display_name: "Set Extension Enabled",
        emoji: "🧩",
        aggregate_variant: "SetExtensionEnabled",
        payload_schema: S_SPACE_SCHEMA,
        text_opcode: Some("set-extension-enabled"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
];

impl protocol::Mutation<SpaceSnapshot> for SpaceMutation {
    type Diff = SpaceDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = SPACE_MUTATION_DESCRIPTORS;

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        let index = match self {
            SpaceMutation::SetName { .. } => 0,
            SpaceMutation::SetKind { .. } => 1,
            SpaceMutation::SetVisibility { .. } => 2,
            SpaceMutation::UpsertUser { .. } => 3,
            SpaceMutation::RemoveUser { .. } => 4,
            SpaceMutation::AddCollection { .. } => 5,
            SpaceMutation::RemoveCollection { .. } => 6,
            SpaceMutation::RenameCollection { .. } => 7,
            SpaceMutation::InstallProgram { .. } => 8,
            SpaceMutation::UninstallProgram { .. } => 9,
            SpaceMutation::InstallExtension { .. } => 10,
            SpaceMutation::UninstallExtension { .. } => 11,
            SpaceMutation::SetExtensionEnabled { .. } => 12,
        };
        &Self::DESCRIPTORS[index]
    }

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): no `Error`/`Warning`/`Fatal` messages added here yet — that is the W3
    /// fan-out's job per verb family.
    fn diff(&self, base: &SpaceSnapshot) -> protocol::MutationOutcome<SpaceDiff> {
        let missing = |what: &str, id: &str| protocol::MutationOutcome::error("mutation.target-missing", format!("{what} {id} does not exist."), [id.to_string()]);
        match self {
            SpaceMutation::SetName { name } => protocol::MutationOutcome::new(SpaceDiff { name: Some(name.clone()), ..Default::default() }),
            SpaceMutation::SetKind { kind } => protocol::MutationOutcome::new(SpaceDiff { kind: Some(*kind), ..Default::default() }),
            SpaceMutation::SetVisibility { visibility } => protocol::MutationOutcome::new(SpaceDiff { visibility: Some(*visibility), ..Default::default() }),
            SpaceMutation::UpsertUser { user, index } => {
                let delta = match base.users.iter().find(|existing| existing.id == user.id) {
                    Some(existing) => SpaceUsersDelta {
                        patched: vec![SpaceUserPatch { id: user.id.clone(), name: (existing.name != user.name).then(|| user.name.clone()), avatar: (existing.avatar != user.avatar).then(|| SpaceOptionalAvatar { value: user.avatar.clone() }), role: (existing.role != user.role).then_some(user.role) }],
                        ..Default::default()
                    },
                    None => SpaceUsersDelta { added: vec![user.clone()], reordered: insertion_order(base.users.iter().map(|existing| existing.id.as_str()), &user.id, *index), ..Default::default() },
                };
                protocol::MutationOutcome::new(SpaceDiff { users: Some(delta), ..Default::default() })
            }
            SpaceMutation::RemoveUser { user_id } if base.users.iter().any(|user| &user.id == user_id) => protocol::MutationOutcome::new(SpaceDiff { users: Some(SpaceUsersDelta { removed: vec![user_id.clone()], ..Default::default() }), ..Default::default() }),
            SpaceMutation::RemoveUser { user_id } => missing("User", user_id),
            SpaceMutation::AddCollection { collection, .. } if base.collections.iter().any(|existing| existing.id == collection.id) => {
                protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Collection {} already exists.", collection.id), [collection.id.clone()])
            }
            SpaceMutation::AddCollection { collection, index } => protocol::MutationOutcome::new(SpaceDiff {
                collections: Some(SpaceCollectionsDelta { added: vec![collection.clone()], reordered: insertion_order(base.collections.iter().map(|existing| existing.id.as_str()), &collection.id, *index), ..Default::default() }),
                ..Default::default()
            }),
            SpaceMutation::RemoveCollection { collection_id } if base.collections.iter().any(|collection| &collection.id == collection_id) => {
                protocol::MutationOutcome::new(SpaceDiff { collections: Some(SpaceCollectionsDelta { removed: vec![collection_id.clone()], ..Default::default() }), ..Default::default() })
            }
            SpaceMutation::RemoveCollection { collection_id } => missing("Collection", collection_id),
            SpaceMutation::RenameCollection { collection_id, name } if base.collections.iter().any(|collection| &collection.id == collection_id) => protocol::MutationOutcome::new(SpaceDiff {
                collections: Some(SpaceCollectionsDelta { patched: vec![SpaceCollectionPatch { id: collection_id.clone(), name: Some(name.clone()), ..Default::default() }], ..Default::default() }),
                ..Default::default()
            }),
            SpaceMutation::RenameCollection { collection_id, .. } => missing("Collection", collection_id),
            SpaceMutation::InstallProgram { plugin_id, .. } if base.programs.contains(plugin_id) => protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Program {plugin_id} is already installed."), [plugin_id.clone()]),
            SpaceMutation::InstallProgram { plugin_id, index } => protocol::MutationOutcome::new(SpaceDiff {
                programs: Some(SpaceProgramsDelta { added: vec![plugin_id.clone()], reordered: insertion_order(base.programs.iter().map(String::as_str), plugin_id, *index), ..Default::default() }),
                ..Default::default()
            }),
            SpaceMutation::UninstallProgram { plugin_id } if base.programs.contains(plugin_id) => protocol::MutationOutcome::new(SpaceDiff { programs: Some(SpaceProgramsDelta { removed: vec![plugin_id.clone()], ..Default::default() }), ..Default::default() }),
            SpaceMutation::UninstallProgram { plugin_id } => missing("Program", plugin_id),
            SpaceMutation::InstallExtension { extension_id, version, source_uri, package_hash, enabled, index } => {
                let delta = match base.extensions.iter().find(|existing| &existing.extension_id == extension_id) {
                    Some(existing) => SpaceExtensionsDelta {
                        patched: vec![SpaceExtensionPatch {
                            extension_id: extension_id.clone(),
                            version: (existing.version != *version).then(|| version.clone()),
                            source_uri: (existing.source_uri != *source_uri).then(|| source_uri.clone()),
                            package_hash: (existing.package_hash != *package_hash).then(|| package_hash.clone()),
                            enabled: (existing.enabled != *enabled).then_some(*enabled),
                        }],
                        ..Default::default()
                    },
                    None => SpaceExtensionsDelta {
                        added: vec![InstalledExtension { extension_id: extension_id.clone(), version: version.clone(), source_uri: source_uri.clone(), package_hash: package_hash.clone(), enabled: *enabled }],
                        reordered: insertion_order(base.extensions.iter().map(|existing| existing.extension_id.as_str()), extension_id, *index),
                        ..Default::default()
                    },
                };
                protocol::MutationOutcome::new(SpaceDiff { extensions: Some(delta), ..Default::default() })
            }
            SpaceMutation::UninstallExtension { extension_id } if base.extensions.iter().any(|existing| &existing.extension_id == extension_id) => {
                protocol::MutationOutcome::new(SpaceDiff { extensions: Some(SpaceExtensionsDelta { removed: vec![extension_id.clone()], ..Default::default() }), ..Default::default() })
            }
            SpaceMutation::UninstallExtension { extension_id } => missing("Extension", extension_id),
            SpaceMutation::SetExtensionEnabled { extension_id, enabled } if base.extensions.iter().any(|existing| &existing.extension_id == extension_id) => protocol::MutationOutcome::new(SpaceDiff {
                extensions: Some(SpaceExtensionsDelta { patched: vec![SpaceExtensionPatch { extension_id: extension_id.clone(), enabled: Some(*enabled), ..Default::default() }], ..Default::default() }),
                ..Default::default()
            }),
            SpaceMutation::SetExtensionEnabled { extension_id, .. } => missing("Extension", extension_id),
        }
    }

    fn inverse(&self, base: &SpaceSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        match self {
            SpaceMutation::SetName { .. } => vec![SpaceMutation::SetName { name: base.name.clone() }],
            SpaceMutation::SetKind { .. } => vec![SpaceMutation::SetKind { kind: base.kind }],
            SpaceMutation::SetVisibility { .. } => vec![SpaceMutation::SetVisibility { visibility: base.visibility }],
            SpaceMutation::UpsertUser { user, .. } => match base.users.iter().find(|existing| existing.id == user.id) {
                Some(existing) => vec![SpaceMutation::UpsertUser { user: existing.clone(), index: None }],
                None => vec![SpaceMutation::RemoveUser { user_id: user.id.clone() }],
            },
            SpaceMutation::RemoveUser { user_id } => base.users.iter().position(|user| &user.id == user_id).map(|at| vec![SpaceMutation::UpsertUser { user: base.users[at].clone(), index: Some(at as u32) }]).unwrap_or_default(),
            SpaceMutation::AddCollection { collection, .. } => vec![SpaceMutation::RemoveCollection { collection_id: collection.id.clone() }],
            SpaceMutation::RemoveCollection { collection_id } => base.collections.iter().position(|collection| &collection.id == collection_id).map(|at| vec![SpaceMutation::AddCollection { collection: base.collections[at].clone(), index: Some(at as u32) }]).unwrap_or_default(),
            SpaceMutation::RenameCollection { collection_id, .. } => {
                base.collections.iter().find(|collection| &collection.id == collection_id).map(|collection| vec![SpaceMutation::RenameCollection { collection_id: collection_id.clone(), name: collection.name.clone() }]).unwrap_or_default()
            }
            SpaceMutation::InstallProgram { plugin_id, .. } => {
                if base.programs.contains(plugin_id) {
                    Vec::new()
                } else {
                    vec![SpaceMutation::UninstallProgram { plugin_id: plugin_id.clone() }]
                }
            }
            SpaceMutation::UninstallProgram { plugin_id } => {
                match base.programs.iter().position(|existing| existing == plugin_id) {
                    Some(at) => vec![SpaceMutation::InstallProgram { plugin_id: plugin_id.clone(), index: Some(at as u32) }],
                    None => Vec::new(),
                }
            }
            SpaceMutation::InstallExtension { extension_id, .. } => match base.extensions.iter().find(|existing| &existing.extension_id == extension_id) {
                Some(existing) => vec![SpaceMutation::InstallExtension {
                    extension_id: existing.extension_id.clone(),
                    version: existing.version.clone(),
                    source_uri: existing.source_uri.clone(),
                    package_hash: existing.package_hash.clone(),
                    enabled: existing.enabled,
                    index: None,
                }],
                None => vec![SpaceMutation::UninstallExtension { extension_id: extension_id.clone() }],
            },
            SpaceMutation::UninstallExtension { extension_id } => base
                .extensions
                .iter()
                .position(|existing| &existing.extension_id == extension_id)
                .map(|at| {
                    let existing = &base.extensions[at];
                    vec![SpaceMutation::InstallExtension {
                        extension_id: existing.extension_id.clone(),
                        version: existing.version.clone(),
                        source_uri: existing.source_uri.clone(),
                        package_hash: existing.package_hash.clone(),
                        enabled: existing.enabled,
                        index: Some(at as u32),
                    }]
                })
                .unwrap_or_default(),
            SpaceMutation::SetExtensionEnabled { extension_id, .. } => {
                base.extensions.iter().find(|existing| &existing.extension_id == extension_id).map(|existing| vec![SpaceMutation::SetExtensionEnabled { extension_id: extension_id.clone(), enabled: existing.enabled }]).unwrap_or_default()
            }
        }
    
    })())
}
}
//#endregion 🔖️SpaceMutation
//#endregion 🔖️Space
//#region 🔖️HandcraftedArtifactCodecs



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
/// to `Spectator`, deterministically across peers replaying the same operation — surfaced as a
/// `mutation.clamped` message (§C2's frozen code set has no per-plugin/per-invariant codes; the
/// original free-form `"space/atelier-multi-author"` tag now travels in `target`).
pub fn reconcile_space_atelier_invariant(mut snapshot: SpaceSnapshot) -> (SpaceSnapshot, Vec<protocol::MutationMessage>) {
    let mut messages = Vec::new();
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
            messages.push(protocol::MutationMessage::warning("mutation.clamped", format!("atelier space retains a single author ({keep}); demoted the rest to spectator")).at(vec!["space/atelier-multi-author".to_string(), keep]));
        }
    }
    (snapshot, messages)
}
/// 🔗️ `space://<space_id>` — the space manifest's own backbone URI.
pub fn space_backbone_uri(space_id: &str) -> String {
    format!("space://{space_id}")
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct SpacePackageSource {
    definition_version: u8,
    id: String,
    artifact: String,
    directory: String,
    rust_package: String,
    nx_project: String,
    dependencies: Vec<String>,
}

/// 📦️ Validated package identity for the builtin space artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceArtifactPackage {
    pub id: String,
    pub artifact: String,
    pub directory: String,
    pub rust_package: String,
    pub nx_project: String,
    pub dependencies: Vec<String>,
}

/// 🚫️ Invalid builtin space package declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpacePackageSchemaError(pub String);

impl std::fmt::Display for SpacePackageSchemaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for SpacePackageSchemaError {}

/// 🧬️ Language-neutral package declaration owned by this artifact.
pub const SPACE_ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("🧬️schema/📜️artifact-definition.json");

/// 📦️ Parses and validates the builtin space package declaration.
pub fn space_package_from_schema(source: &str) -> Result<SpaceArtifactPackage, SpacePackageSchemaError> {
    let parsed = semio_framework_pack_json::from_json_str::<SpacePackageSource>(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SpacePackageSchemaError(error.to_string()))?;
    if parsed.definition_version != 1 || parsed.id != "os.space" || parsed.artifact != "space" || parsed.directory != "🪐️space" || parsed.rust_package != "semio-framework-artifact-space-space" || parsed.nx_project != "@semio-tech/framework-space-space-rs" || !parsed.dependencies.is_empty() {
        return Err(SpacePackageSchemaError("builtin space package identity does not match its canonical declaration".into()));
    }
    Ok(SpaceArtifactPackage {
        id: parsed.id,
        artifact: parsed.artifact,
        directory: parsed.directory,
        rust_package: parsed.rust_package,
        nx_project: parsed.nx_project,
        dependencies: parsed.dependencies,
    })
}

/// 📦️ Returns this artifact's validated package declaration.
pub fn package_descriptor() -> Result<SpaceArtifactPackage, SpacePackageSchemaError> {
    space_package_from_schema(SPACE_ARTIFACT_DEFINITION_SCHEMA)
}

#[cfg(test)]
#[path = "🧪️tests/🪐️space/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_baseline;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
