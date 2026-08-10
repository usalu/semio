//! note -> dxf
use crate::artifacts::note::engine::flatten_blocks;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::{write_dxf_text, DxfLine};
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn serialize(snapshot: &NoteSnapshot) -> Result<DxfSnapshot, String> {
    let mut lines = Vec::new();
    for block in flatten_blocks(&snapshot.blocks) {
        if let NoteBlockNode::Ink { points, .. } = block {
            for pair in points.windows(2) {
                lines.push(DxfLine { x1: pair[0][0], y1: pair[0][1], z1: 0.0, x2: pair[1][0], y2: pair[1][1], z2: 0.0 });
            }
        }
    }
    Ok(DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    let snap = serialize(snapshot)?;
    Ok(write_dxf_text(&snap.lines).into_bytes())
}
