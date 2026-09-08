//! 🌱️ Pure bounded initial-child identity; this digest grants no creation or publication authority.

use crate::os_directory::schema::DocumentScope;
use crate::os_io::{ArtifactDialect, ArtifactRef};

const INITIAL_CHILD_DOMAIN: &[u8] = b"semio.initial-child.v1\0";
const INITIAL_CHILD_FIELD_BYTES: usize = 256;
const INITIAL_CHILD_ORDINAL_LIMIT: u32 = 64;

/// 🪪️ Hashes agreeing typed scope/parent coordinates, slot, child dialect and ordinal without mutable content.
pub(super) fn initial_child_identity(scope: &DocumentScope, parent: &ArtifactRef, slot: &str, child: &ArtifactDialect, ordinal: u32) -> Result<String, &'static str> {
    if scope.document_id.len() > INITIAL_CHILD_FIELD_BYTES || parent.artifact_id.len() > INITIAL_CHILD_FIELD_BYTES {
        return Err("initial-child-coordinate");
    }
    if scope.document_id != parent.artifact_id {
        return Err("initial-child-document");
    }
    let fields = [
        scope.space_id.as_str(),
        parent.artifact_id.as_str(),
        parent.dialect.artifact_kind.as_str(),
        parent.dialect.standard.as_str(),
        parent.dialect.subset.as_str(),
        slot,
        child.artifact_kind.as_str(),
        child.standard.as_str(),
        child.subset.as_str(),
    ];
    if ordinal >= INITIAL_CHILD_ORDINAL_LIMIT {
        return Err("initial-child-ordinal");
    }
    if fields.iter().any(|field| field.is_empty() || field.len() > INITIAL_CHILD_FIELD_BYTES || field.chars().any(|value| matches!(value, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}'))) {
        return Err("initial-child-coordinate");
    }
    let mut hash = semio_framework_hash::Hasher::new();
    hash.update(INITIAL_CHILD_DOMAIN);
    for field in fields {
        hash.update(&(field.len() as u32).to_le_bytes());
        hash.update(field.as_bytes());
    }
    hash.update(&ordinal.to_le_bytes());
    Ok(format!("initial-child-{}", hash.finalize().to_hex()))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
