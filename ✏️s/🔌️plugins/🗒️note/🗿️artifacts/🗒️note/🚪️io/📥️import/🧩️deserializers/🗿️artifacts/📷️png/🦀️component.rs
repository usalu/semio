//! note <- png
use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::png::engine::encode_png;
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;
use std::collections::BTreeMap;
pub fn register() {}
fn b64(bytes: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if i + 1 < bytes.len() { TABLE[(((b1 & 15) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if i + 2 < bytes.len() { TABLE[(b2 & 63) as usize] as char } else { '=' });
        i += 3;
    }
    out
}
pub fn deserialize(from: &PngSnapshot) -> Result<NoteSnapshot, String> {
    let bytes = encode_png(from)?;
    let key = "png-import".to_string();
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("png-import");
    snap.title = Some("Imported PNG".into());
    let mut assets = BTreeMap::new();
    assets.insert(key.clone(), NoteImageAsset { mime: "image/png".into(), data: format!("data:image/png;base64,{}", b64(&bytes)), width: Some(from.image.width as f64), height: Some(from.image.height as f64) });
    snap.assets = assets;
    snap.blocks.push(NoteBlockNode::Image { id: "png-image-1".into(), name: "PNG".into(), x: 0.0, y: 0.0, width: from.image.width.max(1) as f64, height: from.image.height.max(1) as f64, rotation: 0.0, visible: true, locked: false, image_key: key });
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    deserialize(&semio_s_plugin_stdio::artifacts::png::engine::decode_png(bytes)?)
}
