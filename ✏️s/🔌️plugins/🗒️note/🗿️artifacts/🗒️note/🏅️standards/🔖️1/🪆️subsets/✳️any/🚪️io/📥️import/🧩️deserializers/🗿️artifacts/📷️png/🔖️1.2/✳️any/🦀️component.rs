//! 🚪️ note <- png — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Wraps the whole PNG as one
//! data-uri image asset/block — real pixel content is carried but not decomposed into note
//! structure, so this hop is `IoFidelity::Lossy`.

use crate::artifacts::note::schema::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NoteSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::io::decode_png;
use std::collections::BTreeMap;

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct PngIntoNote;

/// 🔤️ Minimal, dependency-free base64 encoder (this repo's "no external libraries for runtime
/// purposes" rule) — unchanged from the pre-migration free function.
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

impl Deserializer<NoteSnapshot> for PngIntoNote {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PngIntoNote: expected a binary png payload".to_string(), diagnostics: Vec::new() });
        };
        let png = decode_png(bytes).map_err(|error| IoError { message: format!("PngIntoNote: decode failed: {error}"), diagnostics: Vec::new() })?;
        let key = "png-import".to_string();
        let mut snap = empty_note_snapshot();
        snap.id = create_note_id("png-import");
        snap.title = Some("Imported PNG".into());
        let mut assets = BTreeMap::new();
        assets.insert(key.clone(), NoteImageAsset { mime: "image/png".into(), data: format!("data:image/png;base64,{}", b64(bytes)), width: Some(png.width as f64), height: Some(png.height as f64) });
        snap.assets = assets;
        snap.blocks.push(NoteBlockNode::Image { id: "png-image-1".into(), name: "PNG".into(), x: 0.0, y: 0.0, width: png.width.max(1) as f64, height: png.height.max(1) as f64, rotation: 0.0, visible: true, locked: false, image_key: key });
        Ok(IoOutcome::clean(snap))
    }
}
