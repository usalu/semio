//! note -> dxf
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--report.md`): `DxfSnapshot` was restructured from a flat `lines: Vec<DxfLine>` into the
//! full real R12 model (`header_vars`/`tables`/`other_tables`/`blocks`/`entities`) by a
//! concurrent stdio wave. Fixed as a minimal lagging-call-site update: each ink stroke segment
//! becomes one `DxfEntity::Line` (same per-segment shape the old `DxfLine` chain built), and
//! `print_dxf_document` replaces the old free `write_dxf_text(&[DxfLine])`.
use crate::artifacts::note::engine::flatten_blocks;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::{print_dxf_document, DxfEntity};
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn serialize(snapshot: &NoteSnapshot) -> Result<DxfSnapshot, String> {
    let mut entities = Vec::new();
    for block in flatten_blocks(&snapshot.blocks) {
        if let NoteBlockNode::Ink { points, .. } = block {
            for pair in points.windows(2) {
                entities.push(DxfEntity::Line {
                    start: [pair[0][0], pair[0][1], 0.0],
                    end: [pair[1][0], pair[1][1], 0.0],
                    layer: "0".into(),
                    unknown_group_codes: Vec::new(),
                });
            }
        }
    }
    Ok(DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), entities, ..DxfSnapshot::default() })
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    Ok(print_dxf_document(&serialize(snapshot)?).into_bytes())
}
