//! 🌱️ Closed host-owned artifact creation intent and progress receipts.

use semio_framework_value_derive::{FromValue, ToValue};

/// 🧯️ Maximum canonical creation request or status bytes.
pub const SPACE_ARTIFACT_CREATION_MAX_BYTES: usize = 4096;

/// 🧮️ Maximum selected-current creation choices disclosed to one Space dialog.
pub const SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS: usize = 64;

/// 🧯️ Maximum canonical selected-current creation catalog bytes.
pub const SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES: usize = 65_536;

fn identity(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256 && value.as_bytes()[0].is_ascii_alphanumeric() && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"._:/-".contains(&byte))
}

/// 🌟 The dialect grammar's any-subset coordinate, mirrored from `io_schema::SubsetId::ANY` exactly
/// as `📇️document-index-v1/🦀️.rs` mirrors it (this module cannot depend on that crate).
const ANY_SUBSET: &str = "*";

/// 🃏️ A dialect subset is an identity **or** the one any-subset coordinate that a dialect writes
/// for a standard's whole subset space (`"s.gis.gismap@1/*"`). Every shipped trusted-catalog open
/// target declares it, so an identity-only predicate here refuses the real catalog and makes
/// artifact creation impossible. It stays a bounded, non-executable literal: exactly `*`, never a
/// pattern embedded in a longer identity.
fn subset(value: &str) -> bool {
    value == ANY_SUBSET || identity(value)
}

fn request_id(value: &str) -> bool {
    value.len() == 32 && value.bytes().any(|byte| byte != b'0') && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().any(|byte| byte != b'0') && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn label(value: &str) -> bool {
    !value.is_empty() && value.chars().count() <= 128 && value.trim_matches(' ') == value && !value.chars().any(char::is_control)
}

/// 🗣️ Both supported locales are explicit; neither is a fallback for the other.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationLabelV1 {
    pub en: String,
    pub de: String,
}

/// 🎯️ One presentation-only choice derived from an exact selected compiled descriptor.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationKindV1 {
    pub kind_id: String,
    pub schema: String,
    pub dialect: SpaceArtifactCreationDialectV1,
    pub label: SpaceArtifactCreationLabelV1,
}

impl SpaceArtifactCreationKindV1 {
    /// 🛡️ Presentation coordinates remain exact and cannot carry executable authority.
    ///
    /// 🪢 `kind_id` and `dialect.artifact_kind` are two DIFFERENT id spaces the product keeps
    /// deliberately distinct, so neither may be validated against the other: `kind_id` is a manifest
    /// `ArtifactKindSpec::id` from the taxonomy space (`2d.note`, `text.document`, `stdio.json`),
    /// while `dialect.artifact_kind` is the owning app's `Dialect` coordinate from the plugin space
    /// (`s.note.note`, `s.writer.writer`, `s.stdio.json`). Every plugin in the repo but one spells
    /// them differently; `gis` alone writes `ArtifactKindSpec { id: GISMAP_DIALECT.artifact_kind }`,
    /// so a string equality here admits `gis` and refuses every other plugin's catalog row, which
    /// made a non-`gis` trusted catalog impossible to present for creation. The binding between the
    /// two spaces is declared by the trusted catalog, where `artifact_creation_catalog` derives BOTH
    /// fields from one `VerifiedDocumentOpenSelectionV1` whose `artifact.kind` is already pinned to a
    /// manifest kind and whose `parent_dialect` is already pinned to that app's dialect by
    /// `validate_descriptor_open_target`; no authority is lost by bounding each field on its own.
    pub fn validate(&self) -> bool {
        identity(&self.kind_id) && identity(&self.schema) && identity(&self.dialect.artifact_kind) && identity(&self.dialect.standard) && subset(&self.dialect.subset) && label(&self.label.en) && label(&self.label.de)
    }
}

/// 🗂️ Bounded selected-current choices for one authenticated Space.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationCatalogV1 {
    pub schema: String,
    pub space_id: String,
    pub catalog_generation_id: String,
    pub kinds: Vec<SpaceArtifactCreationKindV1>,
}

impl SpaceArtifactCreationCatalogV1 {
    /// 🧬️ The list is nonempty, bounded, kind-sorted and duplicate-free.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.space-artifact-creation-catalog/v1"
            && identity(&self.space_id)
            && digest(&self.catalog_generation_id)
            && !self.kinds.is_empty()
            && self.kinds.len() <= SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS
            && self.kinds.iter().all(SpaceArtifactCreationKindV1::validate)
            && self.kinds.windows(2).all(|pair| pair[0].kind_id < pair[1].kind_id)
    }

    /// 📤️ Emits only a canonical bounded response.
    pub fn canonical_json(&self) -> Option<String> {
        if !self.validate() {
            return None;
        }
        let source = crate::os_pack::json::to_json_string(self);
        (source.len() <= SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES).then_some(source)
    }

    /// 🧾️ Rejects reordering, unknown fields, padding and oversized presentation rows.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }
}

/// 📥️ A client chooses only a trusted kind and display name; scope and author come from the route.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreateV1 {
    pub schema: String,
    pub request_id: String,
    pub expected_catalog_generation_id: String,
    pub kind_id: String,
    pub name: String,
}

impl SpaceArtifactCreateV1 {
    /// 🛡️ Validates the schema-owned scalar bounds before catalog or storage access.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.space-artifact-create/v1"
            && request_id(&self.request_id)
            && digest(&self.expected_catalog_generation_id)
            && identity(&self.kind_id)
            && !self.name.is_empty()
            && self.name.chars().count() <= 128
            && self.name.trim_matches(' ') == self.name
            && !self.name.chars().any(char::is_control)
    }

    /// 📦️ Rejects duplicate fields, unknown authority inputs, padding, and noncanonical JSON.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > SPACE_ARTIFACT_CREATION_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }
}

/// 🚦️ Durable creation progress never implies readiness before both publication and indexing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum SpaceArtifactCreationPhaseV1 {
    Accepted,
    Preparing,
    Ready,
    Indeterminate,
    Failed,
    Cancelled,
}

/// 🧭️ The server-selected artifact dialect, without executable authority supplied by a client.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationDialectV1 {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}

/// 🚪️ Only a completed creation carries coordinates for the ordinary document-open flow.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationReadyV1 {
    pub artifact_id: String,
    pub kind_id: String,
    pub artifact_schema: String,
    pub parent_dialect: SpaceArtifactCreationDialectV1,
}

impl SpaceArtifactCreationReadyV1 {
    /// 🧷️ Checks the minted coordinate and each kind/dialect coordinate it carries.
    ///
    /// 🪢 `kind_id` (manifest taxonomy space) and `parent_dialect.artifact_kind` (plugin dialect
    /// space) are the two distinct spaces `SpaceArtifactCreationKindV1::validate` documents; the
    /// receipt copies both out of the accepted intent, which took them from one trusted-catalog
    /// selection, so the relationship is established there and cannot be re-derived from the two
    /// strings alone.
    pub fn validate(&self) -> bool {
        self.artifact_id.strip_prefix("artifact-").is_some_and(request_id)
            && identity(&self.kind_id)
            && identity(&self.artifact_schema)
            && identity(&self.parent_dialect.artifact_kind)
            && identity(&self.parent_dialect.standard)
            && subset(&self.parent_dialect.subset)
    }
}

/// 📣️ A bounded receipt; incomplete operations never leak a usable document coordinate.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationStatusV1 {
    pub schema: String,
    pub request_id: String,
    pub space_id: String,
    pub catalog_generation_id: String,
    pub phase: SpaceArtifactCreationPhaseV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<SpaceArtifactCreationReadyV1>,
}

impl SpaceArtifactCreationStatusV1 {
    /// 🔐️ Only Ready may contain document coordinates, and Ready must contain all of them.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.space-artifact-creation-status/v1"
            && request_id(&self.request_id)
            && identity(&self.space_id)
            && digest(&self.catalog_generation_id)
            && match (&self.phase, &self.ready) {
                (SpaceArtifactCreationPhaseV1::Ready, Some(ready)) => ready.validate(),
                (SpaceArtifactCreationPhaseV1::Ready, None) | (_, Some(_)) => false,
                (_, None) => true,
            }
    }

    /// 🧾️ Reads one exact receipt, withholding malformed or authority-overposted results.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > SPACE_ARTIFACT_CREATION_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }
}
