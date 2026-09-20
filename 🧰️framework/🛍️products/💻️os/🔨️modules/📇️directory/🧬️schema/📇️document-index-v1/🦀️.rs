//! 📇️ Directory-owned presentation index, separate from immutable document codec authority.

use semio_framework_value_derive::{FromValue, ToValue};

/// 🏷️ One server-selected name and dialect at document genesis.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentIndexEntryV1 {
    pub name: String,
    pub dialect: crate::os_io::ArtifactDialect,
}

impl DocumentIndexEntryV1 {
    /// 🛡️ Validates bounded presentation fields without introducing executable authority.
    ///
    /// 🌟 `subset` additionally admits the ONE canonical any-subset coordinate
    /// (`semio_framework::io_schema::SubsetId::ANY`, spelled `*`): it is the dialect grammar's own
    /// wildcard, the value every dialect that declares no narrowed subset carries — including the
    /// `parent_dialect` the Directory's own document-open admission returns — so rejecting it here
    /// would drop the presentation row of practically every indexed document. It stays a bounded,
    /// non-executable literal: exactly `*`, never a pattern embedded in a longer identity.
    ///
    /// 🪢 `dialect.artifact_kind` is the owning app's `Dialect` coordinate (`s.note.note`,
    /// `s.gis.gismap`), NOT the manifest `ArtifactKindSpec::id` its descriptor carries (`2d.note`,
    /// `stdio.json`) — two id spaces that coincide for `gis` alone. This law therefore bounds the
    /// field by its own grammar and never against the descriptor: an equality between the two
    /// indexes exactly one plugin's documents. It is the single gate both sides read — the client
    /// read-model fold and `validate_directory_event_page_event`, and through the latter the hub's
    /// `document_index_projection_v1` — so the grammar lives here and is mirrored nowhere else.
    pub fn validate(&self) -> bool {
        let identity = |value: &str| !value.is_empty() && value.len() <= 256 && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"._:/-".contains(&byte)) && value.as_bytes()[0].is_ascii_alphanumeric();
        let subset_identity = |value: &str| value == ANY_SUBSET || identity(value);
        !self.name.is_empty()
            && self.name.chars().count() <= 128
            && self.name.trim_matches(' ') == self.name
            && !self.name.chars().any(char::is_control)
            && canonical_dialect_artifact_kind(&self.dialect.artifact_kind)
            && identity(&self.dialect.standard)
            && subset_identity(&self.dialect.subset)
    }
}

/// 🔡️ One canonical dialect artifact-kind coordinate: `s.` then one or two lowercase-kebab
/// segments (`s.stdio`, `s.note.note`, `s.puzzle.5d`), mirroring
/// `semio_framework::io_schema::is_canonical_artifact_kind` widened by the bare `s.<plugin>` IO
/// dialect form that `🔌️plugin/🛂️describe/🦀️.rs` documents. It is mirrored here rather than
/// imported for the same reason `ANY_SUBSET` is: this module must not depend on the io crate.
///
/// 🌿 The plugin segment is deliberately compared to nothing. One package legitimately ships an app
/// whose `Dialect` belongs to another plugin — `demonstrator` alone carries `s.gis.gismap`,
/// `s.cad.cad`, `s.puzzle.puzzle3d`, `s.process.process3d`, `s.sourcing.curation` and
/// `s.procedural.generation3d` among the 127 app dialects of the shipped descriptors — so an
/// ownership predicate here would refuse real documents.
fn canonical_dialect_artifact_kind(kind: &str) -> bool {
    let mut segments = kind.split('.');
    if kind.len() > 256 || segments.next() != Some("s") {
        return false;
    }
    let mut count = 0usize;
    for segment in segments {
        if segment.is_empty() || segment.starts_with('-') || segment.ends_with('-') || segment.contains("--") || !segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') {
            return false;
        }
        count += 1;
    }
    (1..=2).contains(&count)
}

/// 🌟 The dialect grammar's any-subset coordinate, mirrored from `io_schema::SubsetId::ANY` (this
/// module must not depend on the io crate).
const ANY_SUBSET: &str = "*";

/// 🧾️ Read-only row derived from a descriptor followed by its indexed Directory event.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryIndexedDocumentViewV1 {
    pub descriptor: super::DocumentDescriptor,
    pub descriptor_digest_v1: super::ArtifactHash,
    pub entry: DocumentIndexEntryV1,
    pub created_at_ms: i64,
    pub created_by: String,
}
