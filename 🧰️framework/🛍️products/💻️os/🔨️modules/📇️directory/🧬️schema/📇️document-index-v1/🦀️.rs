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
    pub fn validate(&self) -> bool {
        let identity = |value: &str| !value.is_empty() && value.len() <= 256 && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"._:/-".contains(&byte)) && value.as_bytes()[0].is_ascii_alphanumeric();
        !self.name.is_empty()
            && self.name.chars().count() <= 128
            && self.name.trim_matches(' ') == self.name
            && !self.name.chars().any(char::is_control)
            && identity(&self.dialect.artifact_kind)
            && identity(&self.dialect.standard)
            && identity(&self.dialect.subset)
    }
}

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
