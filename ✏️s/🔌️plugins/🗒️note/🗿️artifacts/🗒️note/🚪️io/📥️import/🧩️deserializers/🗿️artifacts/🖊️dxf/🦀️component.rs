//! note <- dxf
use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &DxfSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("dxf-import");
    snap.title = Some("Imported DXF".into());
    for (i, line) in from.lines.iter().enumerate() {
        snap.blocks.push(NoteBlockNode::Ink {
            id: format!("dxf-line-{i}"), name: "Line".into(),
            x: line.x1.min(line.x2), y: line.y1.min(line.y2),
            width: (line.x1 - line.x2).abs().max(1.0), height: (line.y1 - line.y2).abs().max(1.0),
            rotation: 0.0, visible: true, locked: false,
            points: vec![[line.x1, line.y1], [line.x2, line.y2]],
            stroke_width: 1.0, color: [0.0, 0.0, 0.0, 1.0],
        });
    }
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let lines = semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::parse_dxf_text(text)?;
    deserialize(&DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
}
