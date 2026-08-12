//! imperative -> md
use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;
use semio_s_plugin_stdio::artifacts::md::engine::parse_markdown_blocks;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the paired import leaf's doc comment) — wraps the printed DSL text as
/// md's own block model via stdio's own `parse_markdown_blocks`, mirroring `🔱️jack`'s own fix.
pub fn serialize(snapshot: &ImperativeSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        blocks: parse_markdown_blocks(&<ImperativeSnapshot as store::ArtifactDsl>::print_dsl(snapshot)),
    })
}

pub fn serialize_bytes(snapshot: &ImperativeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<MdSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
