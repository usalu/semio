//! imperative <- md
use crate::artifacts::procedure::schema::snapshot::ProcedureSnapshot;
use semio_s_plugin_stdio::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the CSV import leaf's doc comment for the wave that caused this) —
/// `MdSnapshot` is now a typed `blocks: Vec<MdBlock>` CommonMark tree, not a raw `body: String`;
/// mirrors `🔱️jack`'s own fix using stdio's own `render_markdown_blocks` to flatten the tree back
/// to text before handing it to this artifact's own DSL parser.
pub fn deserialize(from: &MdSnapshot) -> Result<ProcedureSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(&render_markdown_blocks(&from.blocks))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProcedureSnapshot, store::TextError> {
    let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&md)
}
