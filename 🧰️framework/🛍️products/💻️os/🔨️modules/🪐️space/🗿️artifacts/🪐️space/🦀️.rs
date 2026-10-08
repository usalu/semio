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

/// 🔺️ Sparse space-manifest delta: the scalar fields are present slots, and every member collection is a positional row delta
/// (`removed`/`inserted`/`moved`/`patched`, see `protocol::list_delta`) so any number of users, collections, programs and extensions change in one diff.
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
    pub name: Option<String>,
    pub avatar: Option<SpaceOptionalAvatar>,
    pub role: Option<SpaceRole>,
}

/// 🩹 Field patch of one collection reference.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceCollectionPatch {
    pub name: Option<String>,
    pub document_id: Option<String>,
}


/// 🩹 Field patch of one installed extension.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceExtensionPatch {
    pub version: Option<String>,
    pub source_uri: Option<String>,
    pub package_hash: Option<String>,
    pub enabled: Option<bool>,
}

protocol::list_delta! {
    /// 🧩️ Positional row delta of the members.
    #[derive(Serialize, Deserialize)]
    pub SpaceUsersDelta { removal: SpaceUserRemoval, insertion: SpaceUserInsertion, relocation: SpaceUserRelocation, modification: SpaceUsersModification, row: SpaceUser, patch: SpaceUserPatch, key: id, values_only }
}

protocol::list_delta! {
    /// 🧩️ Positional row delta of the collections.
    #[derive(Serialize, Deserialize)]
    pub SpaceCollectionsDelta { removal: SpaceCollectionRemoval, insertion: SpaceCollectionInsertion, relocation: SpaceCollectionRelocation, modification: SpaceCollectionsModification, row: CollectionRef, patch: SpaceCollectionPatch, key: id, values_only }
}

protocol::plain_list_delta! {
    /// 🧩️ Positional row delta of the installed programs (bare ids: a program is installed or uninstalled, never patched).
    #[derive(Serialize, Deserialize)]
    pub SpaceProgramsDelta { removal: SpaceProgramRemoval, insertion: SpaceProgramInsertion, relocation: SpaceProgramRelocation, row: String }
}

protocol::list_delta! {
    /// 🧩️ Positional row delta of the installed extensions, keyed by extension id.
    #[derive(Serialize, Deserialize)]
    pub SpaceExtensionsDelta { removal: SpaceExtensionRemoval, insertion: SpaceExtensionInsertion, relocation: SpaceExtensionRelocation, modification: SpaceExtensionsModification, row: InstalledExtension, patch: SpaceExtensionPatch, key: extension_id, values_only }
}










impl protocol::list_delta::RowPatch<SpaceUser> for SpaceUserPatch {
    fn commit_into(&self, row: &mut SpaceUser, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(avatar) = &self.avatar {
            row.avatar = avatar.value.clone();
        }
        if let Some(role) = self.role {
            row.role = role;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or(self.name.take());
        self.avatar = later.avatar.or(self.avatar.take());
        self.role = later.role.or(self.role);
    }
    fn inverse(&self, row: &SpaceUser) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), avatar: self.avatar.as_ref().map(|_| SpaceOptionalAvatar { value: row.avatar.clone() }), role: self.role.map(|_| row.role) }
    }
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.avatar.is_none() && self.role.is_none()
    }
}

impl protocol::list_delta::RowPatch<CollectionRef> for SpaceCollectionPatch {
    fn commit_into(&self, row: &mut CollectionRef, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(document_id) = &self.document_id {
            row.document_id = document_id.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or(self.name.take());
        self.document_id = later.document_id.or(self.document_id.take());
    }
    fn inverse(&self, row: &CollectionRef) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), document_id: self.document_id.as_ref().map(|_| row.document_id.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.document_id.is_none()
    }
}

impl protocol::list_delta::RowPatch<InstalledExtension> for SpaceExtensionPatch {
    fn commit_into(&self, row: &mut InstalledExtension, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(version) = &self.version {
            row.version = version.clone();
        }
        if let Some(source_uri) = &self.source_uri {
            row.source_uri = source_uri.clone();
        }
        if let Some(package_hash) = &self.package_hash {
            row.package_hash = package_hash.clone();
        }
        if let Some(enabled) = self.enabled {
            row.enabled = enabled;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.version = later.version.or(self.version.take());
        self.source_uri = later.source_uri.or(self.source_uri.take());
        self.package_hash = later.package_hash.or(self.package_hash.take());
        self.enabled = later.enabled.or(self.enabled);
    }
    fn inverse(&self, row: &InstalledExtension) -> Self {
        Self {
                        version: self.version.as_ref().map(|_| row.version.clone()),
            source_uri: self.source_uri.as_ref().map(|_| row.source_uri.clone()),
            package_hash: self.package_hash.as_ref().map(|_| row.package_hash.clone()),
            enabled: self.enabled.map(|_| row.enabled),
        }
    }
    fn is_empty(&self) -> bool {
        self.version.is_none() && self.source_uri.is_none() && self.package_hash.is_none() && self.enabled.is_none()
    }
}

impl protocol::MutationDiff<SpaceSnapshot> for SpaceDiff {
    fn apply(&self, base: &SpaceSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SpaceSnapshot> {
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
            next.users = delta.commit_onto(&next.users, capability).map_err(|error| error.under(["users"]))?;
        }
        if let Some(delta) = &self.collections {
            next.collections = delta.commit_onto(&next.collections, capability).map_err(|error| error.under(["collections"]))?;
        }
        if let Some(delta) = &self.programs {
            next.programs = delta.commit_onto(&next.programs, capability).map_err(|error| error.under(["programs"]))?;
        }
        if let Some(delta) = &self.extensions {
            next.extensions = delta.commit_onto(&next.extensions, capability).map_err(|error| error.under(["extensions"]))?;
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
                    (Some(mut first), Some(later)) => {
                        first.absorb(later);
                        Some(first)
                    }
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
            users: self.users.as_ref().map(|delta| delta.inverse(&base.users)),
            collections: self.collections.as_ref().map(|delta| delta.inverse(&base.collections)),
            programs: self.programs.as_ref().map(|delta| delta.inverse(&base.programs)),
            extensions: self.extensions.as_ref().map(|delta| delta.inverse(&base.extensions)),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_none() && self.kind.is_none() && self.visibility.is_none() && self.users.as_ref().is_none_or(SpaceUsersDelta::is_empty) && self.collections.as_ref().is_none_or(SpaceCollectionsDelta::is_empty) && self.programs.as_ref().is_none_or(SpaceProgramsDelta::is_empty) && self.extensions.as_ref().is_none_or(SpaceExtensionsDelta::is_empty)
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
                    Some(existing) => SpaceUsersDelta::modification(user.id.clone(), SpaceUserPatch { name: (existing.name != user.name).then(|| user.name.clone()), avatar: (existing.avatar != user.avatar).then(|| SpaceOptionalAvatar { value: user.avatar.clone() }), role: (existing.role != user.role).then_some(user.role) }),
                    None => SpaceUsersDelta::insertion(index.map_or(base.users.len(), |at| (*at as usize).min(base.users.len())), user.clone()),
                };
                protocol::MutationOutcome::new(SpaceDiff { users: Some(delta), ..Default::default() })
            }
            SpaceMutation::RemoveUser { user_id } => match base.users.iter().position(|user| &user.id == user_id) {
                Some(at) => protocol::MutationOutcome::new(SpaceDiff { users: Some(SpaceUsersDelta::removal(&base.users, at)), ..Default::default() }),
                None => missing("User", user_id),
            },
            SpaceMutation::AddCollection { collection, .. } if base.collections.iter().any(|existing| existing.id == collection.id) => {
                protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Collection {} already exists.", collection.id), [collection.id.clone()])
            }
            SpaceMutation::AddCollection { collection, index } => protocol::MutationOutcome::new(SpaceDiff {
                collections: Some(SpaceCollectionsDelta::insertion(index.map_or(base.collections.len(), |at| (*at as usize).min(base.collections.len())), collection.clone())),
                ..Default::default()
            }),
            SpaceMutation::RemoveCollection { collection_id } => match base.collections.iter().position(|collection| &collection.id == collection_id) {
                Some(at) => protocol::MutationOutcome::new(SpaceDiff { collections: Some(SpaceCollectionsDelta::removal(&base.collections, at)), ..Default::default() }),
                None => missing("Collection", collection_id),
            },
            SpaceMutation::RenameCollection { collection_id, name } if base.collections.iter().any(|collection| &collection.id == collection_id) => protocol::MutationOutcome::new(SpaceDiff {
                collections: Some(SpaceCollectionsDelta::modification(collection_id.clone(), SpaceCollectionPatch { name: Some(name.clone()), ..Default::default() })),
                ..Default::default()
            }),
            SpaceMutation::RenameCollection { collection_id, .. } => missing("Collection", collection_id),
            SpaceMutation::InstallProgram { plugin_id, .. } if base.programs.contains(plugin_id) => protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Program {plugin_id} is already installed."), [plugin_id.clone()]),
            SpaceMutation::InstallProgram { plugin_id, index } => protocol::MutationOutcome::new(SpaceDiff {
                programs: Some(SpaceProgramsDelta::insertion(index.map_or(base.programs.len(), |at| (*at as usize).min(base.programs.len())), plugin_id.clone())),
                ..Default::default()
            }),
            SpaceMutation::UninstallProgram { plugin_id } => match base.programs.iter().position(|existing| existing == plugin_id) {
                Some(at) => protocol::MutationOutcome::new(SpaceDiff { programs: Some(SpaceProgramsDelta::removal(&base.programs, at)), ..Default::default() }),
                None => missing("Program", plugin_id),
            },
            SpaceMutation::InstallExtension { extension_id, version, source_uri, package_hash, enabled, index } => {
                let delta = match base.extensions.iter().find(|existing| &existing.extension_id == extension_id) {
                    Some(existing) => SpaceExtensionsDelta::modification(extension_id.clone(), SpaceExtensionPatch { version: (existing.version != *version).then(|| version.clone()), source_uri: (existing.source_uri != *source_uri).then(|| source_uri.clone()), package_hash: (existing.package_hash != *package_hash).then(|| package_hash.clone()), enabled: (existing.enabled != *enabled).then_some(*enabled) }),
                    None => SpaceExtensionsDelta::insertion(
                        index.map_or(base.extensions.len(), |at| (*at as usize).min(base.extensions.len())),
                        InstalledExtension { extension_id: extension_id.clone(), version: version.clone(), source_uri: source_uri.clone(), package_hash: package_hash.clone(), enabled: *enabled },
                    ),
                };
                protocol::MutationOutcome::new(SpaceDiff { extensions: Some(delta), ..Default::default() })
            }
            SpaceMutation::UninstallExtension { extension_id } => match base.extensions.iter().position(|existing| &existing.extension_id == extension_id) {
                Some(at) => protocol::MutationOutcome::new(SpaceDiff { extensions: Some(SpaceExtensionsDelta::removal(&base.extensions, at)), ..Default::default() }),
                None => missing("Extension", extension_id),
            },
            SpaceMutation::SetExtensionEnabled { extension_id, enabled } if base.extensions.iter().any(|existing| &existing.extension_id == extension_id) => protocol::MutationOutcome::new(SpaceDiff {
                extensions: Some(SpaceExtensionsDelta::modification(extension_id.clone(), SpaceExtensionPatch { enabled: Some(*enabled), ..Default::default() })),
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

#[path = "🧬️schema/📦️package/🦀️.rs"]
pub mod package;
pub use package::{admit_space_package_declaration, package_descriptor, SpaceArtifactPackage, SpacePackageDeclaration, SpacePackageError};

#[cfg(test)]
#[path = "🧪️tests/🪐️space/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_baseline;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
