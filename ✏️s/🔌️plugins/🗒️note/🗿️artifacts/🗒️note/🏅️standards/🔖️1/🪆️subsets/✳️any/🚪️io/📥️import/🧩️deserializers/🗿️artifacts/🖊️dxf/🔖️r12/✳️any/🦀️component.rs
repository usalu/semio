//! note <- dxf
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired export leaf's doc comment (same wave,
//! `DxfSnapshot`'s flat `lines` -> `entities: Vec<DxfEntity>`). Only `DxfEntity::Line` is mapped
//! back to an ink block (the same narrow scope the old `lines`-only reader covered — this leaf
//! was never a general DXF importer).
use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::{parse_dxf_document, DxfEntity};
use semio_s_plugin_stdio::artifacts::dxf::DxfSnapshot;
pub fn register() {}
pub fn deserialize(from: &DxfSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("dxf-import");
    snap.title = Some("Imported DXF".into());
    let mut i = 0usize;
    for entity in &from.entities {
        if let DxfEntity::Line { start, end, .. } = entity {
            snap.blocks.push(NoteBlockNode::Ink {
                id: format!("dxf-line-{i}"), name: "Line".into(),
                x: start[0].min(end[0]), y: start[1].min(end[1]),
                width: (start[0] - end[0]).abs().max(1.0), height: (start[1] - end[1]).abs().max(1.0),
                rotation: 0.0, visible: true, locked: false,
                points: vec![[start[0], start[1]], [end[0], end[1]]],
                stroke_width: 1.0, color: [0.0, 0.0, 0.0, 1.0],
            });
            i += 1;
        }
    }
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let snapshot = parse_dxf_document(text)?;
    deserialize(&snapshot)
}
