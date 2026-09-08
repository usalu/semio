//! 🪐️ Composable persisted space manifest artifact.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

#[path = "♻️retirement/🦀️.rs"]
mod retirement;

use serde::{Deserialize, Serialize};

//#region 🔖️Roles
/// 🏛️ A space's collaboration shape: `Atelier` (single-writer personal, reconcile-enforced exactly
/// one `Author`), `Studio` (multi-writer group, any number of `Author`s), `Archive` (frozen, nobody
/// writes).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
pub enum SpaceKind {
    Atelier,
    Studio,
    Archive,
}

/// 👁️ Whether a space is discoverable/readable by an anonymous visitor (`Public`, implicit anonymous
/// spectator — wired at the hub layer in W4) or membership-gated (`Private`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
pub enum SpaceVisibility {
    Private,
    Public,
}

/// 🧑️‍🤝️‍🧑️ A space member's permission level: `Author` (read-write) or `Spectator` (read-only). The
/// hub directory (`🌎️hub/🔨️modules/📇️directory`) re-declares this enum string-identically
/// (`"author"`/`"spectator"`, see `as_str`/`parse`) since it cannot depend on this wasm-facing crate —
/// keep the two in lockstep by hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[dsl(id = "os.space")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
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

//#region 🔖️HandcraftedOpCodecs
/// 🧬️ Encodes and parses space mutations as operation text.
impl protocol::OpText for SpaceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslDiff)]
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
    fn apply(&self, base: &SpaceSnapshot) -> protocol::MutationApplyResult<SpaceSnapshot> {
        let mut next = base.clone();
        if self.rename_collection_id.is_some() != self.rename_collection_name.is_some() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "collection rename requires both id and name").at(["collections"]));
        }
        if self.set_extension_enabled_id.is_some() != self.set_extension_enabled.is_some() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "extension enablement requires both id and value").at(["extensions"]));
        }
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
            if !next.users.iter().any(|user| &user.id == user_id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", format!("user {user_id} does not exist")).at(["users", user_id.as_str()]));
            }
            next.users.retain(|user| &user.id != user_id);
        }
        if let Some(collection) = &self.add_collection {
            if next.collections.iter().any(|existing| existing.id == collection.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", format!("collection {} already exists", collection.id)).at(["collections", collection.id.as_str()]));
            }
            next.collections.push(collection.clone());
        }
        if let Some(collection_id) = &self.remove_collection_id {
            if !next.collections.iter().any(|collection| &collection.id == collection_id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", format!("collection {collection_id} does not exist")).at(["collections", collection_id.as_str()]));
            }
            next.collections.retain(|collection| &collection.id != collection_id);
        }
        if let Some(collection_id) = &self.rename_collection_id {
            if let Some(name) = &self.rename_collection_name {
                let collection = next
                    .collections
                    .iter_mut()
                    .find(|collection| &collection.id == collection_id)
                    .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", format!("collection {collection_id} does not exist")).at(["collections", collection_id.as_str()]))?;
                collection.name = name.clone();
            }
        }
        if let Some(plugin_id) = &self.install_program {
            if next.programs.contains(plugin_id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", format!("program {plugin_id} is already installed")).at(["programs", plugin_id.as_str()]));
            }
            next.programs.push(plugin_id.clone());
        }
        if let Some(plugin_id) = &self.uninstall_program {
            if !next.programs.contains(plugin_id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", format!("program {plugin_id} is not installed")).at(["programs", plugin_id.as_str()]));
            }
            next.programs.retain(|installed| installed != plugin_id);
        }
        if let Some(extension) = &self.install_extension {
            next.extensions.retain(|existing| existing.extension_id != extension.extension_id);
            next.extensions.push(extension.clone());
        }
        if let Some(extension_id) = &self.uninstall_extension_id {
            if !next.extensions.iter().any(|existing| &existing.extension_id == extension_id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", format!("extension {extension_id} is not installed")).at(["extensions", extension_id.as_str()]));
            }
            next.extensions.retain(|existing| &existing.extension_id != extension_id);
        }
        if let Some(extension_id) = &self.set_extension_enabled_id {
            if let Some(enabled) = self.set_extension_enabled {
                let extension = next
                    .extensions
                    .iter_mut()
                    .find(|extension| &extension.extension_id == extension_id)
                    .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", format!("extension {extension_id} is not installed")).at(["extensions", extension_id.as_str()]))?;
                extension.enabled = enabled;
            }
        }
        Ok(next)
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
    fn diff(&self, _base: &SpaceSnapshot) -> protocol::MutationOutcome<SpaceDiff> {
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
                diff.install_extension = Some(InstalledExtension { extension_id: extension_id.clone(), version: version.clone(), source_uri: source_uri.clone(), package_hash: package_hash.clone(), enabled: *enabled });
            }
            SpaceMutation::UninstallExtension { extension_id } => diff.uninstall_extension_id = Some(extension_id.clone()),
            SpaceMutation::SetExtensionEnabled { extension_id, enabled } => {
                diff.set_extension_enabled_id = Some(extension_id.clone());
                diff.set_extension_enabled = Some(*enabled);
            }
        }
        protocol::MutationOutcome::new(diff)
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
            SpaceMutation::RemoveCollection { collection_id } => base.collections.iter().find(|collection| &collection.id == collection_id).map(|collection| vec![SpaceMutation::AddCollection { collection: collection.clone() }]).unwrap_or_default(),
            SpaceMutation::RenameCollection { collection_id, .. } => {
                base.collections.iter().find(|collection| &collection.id == collection_id).map(|collection| vec![SpaceMutation::RenameCollection { collection_id: collection_id.clone(), name: collection.name.clone() }]).unwrap_or_default()
            }
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
            SpaceMutation::SetExtensionEnabled { extension_id, .. } => {
                base.extensions.iter().find(|existing| &existing.extension_id == extension_id).map(|existing| vec![SpaceMutation::SetExtensionEnabled { extension_id: extension_id.clone(), enabled: existing.enabled }]).unwrap_or_default()
            }
        }
    }
}
//#endregion 🔖️SpaceMutation
//#endregion 🔖️Space
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
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for SpaceSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
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
            messages.push(protocol::MutationMessage::warn("mutation.clamped", format!("atelier space retains a single author ({keep}); demoted the rest to spectator")).at(vec!["space/atelier-multi-author".to_string(), keep]));
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
    let parsed = store::os_pack::json::from_json_str::<SpacePackageSource>(source).map_err(|error| SpacePackageSchemaError(error.to_string()))?;
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
