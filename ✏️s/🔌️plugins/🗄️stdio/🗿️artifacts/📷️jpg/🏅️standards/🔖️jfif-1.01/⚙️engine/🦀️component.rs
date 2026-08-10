//! ⚙️ JpgEngine — baseline SOF0/SOS for solid-color round-trip.

use crate::artifacts::jpg::{schema::snapshot::RasterImage, JpgArtifact, JpgDiff, JpgMutation, JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};

pub fn encode_jpg(snap: &JpgSnapshot) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    if img.width == 0 || img.height == 0 { return Err("empty image".into()); }
    if img.rgba.len() != (img.width as usize) * (img.height as usize) * 4 {
        return Err("rgba length mismatch".into());
    }
    let r = img.rgba[0];
    let g = img.rgba[1];
    let b = img.rgba[2];
    for chunk in img.rgba.chunks(4) {
        if chunk[0] != r || chunk[1] != g || chunk[2] != b {
            return Err("jpg codec supports solid-color images only".into());
        }
    }
    let w = img.width.min(255) as u8;
    let h = img.height.min(255) as u8;
    let mut out = vec![0xFF, 0xD8];
    out.extend_from_slice(&[0xFF, 0xE0, 0x00, 0x10, b'J', b'F', b'I', b'F', 0, 1, 1, 0, 0, 1, 0, 1, 0, 0]);
    out.extend_from_slice(&[0xFF, 0xDB, 0x00, 0x43, 0x00]);
    out.extend(vec![8u8; 64]);
    out.extend_from_slice(&[0xFF, 0xC0, 0x00, 0x0B, 0x08, h, w, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01]);
    out.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x0C, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F, 0x00]);
    out.extend_from_slice(&[r, g, b]);
    out.extend_from_slice(&[0xFF, 0xD9]);
    Ok(out)
}

pub fn decode_jpg(data: &[u8]) -> Result<JpgSnapshot, String> {
    if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
        return Err("not jpeg".into());
    }
    let mut width = 1u32;
    let mut height = 1u32;
    let mut i = 2usize;
    let mut rgb = [0u8; 3];
    let mut got_sos = false;
    while i + 1 < data.len() {
        if data[i] != 0xFF { i += 1; continue; }
        let marker = data[i + 1];
        i += 2;
        if marker == 0xD9 { break; }
        if marker == 0xDA {
            got_sos = true;
            if i + 12 <= data.len() { i += 12; } else { break; }
            if i + 3 <= data.len() { rgb = [data[i], data[i + 1], data[i + 2]]; }
            break;
        }
        if i + 2 > data.len() { break; }
        let len = ((data[i] as usize) << 8) | data[i + 1] as usize;
        if marker == 0xC0 && len >= 7 && i + 7 <= data.len() {
            height = data[i + 3] as u32;
            width = data[i + 4] as u32;
        }
        i = i.saturating_add(len);
    }
    if !got_sos { return Err("missing SOS".into()); }
    let n = (width as usize) * (height as usize) * 4;
    let mut rgba = vec![0u8; n];
    for px in rgba.chunks_mut(4) {
        px[0] = rgb[0];
        px[1] = rgb[1];
        px[2] = rgb[2];
        px[3] = 255;
    }
    Ok(JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: RasterImage { width, height, rgba } })
}

pub fn empty_jpg_snapshot() -> JpgSnapshot { JpgSnapshot::default() }

pub fn register() {
    crate::artifacts::jpg::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::jpg::schema::jpg_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<JpgSnapshot, JpgMutation>(STDIO_JPG_DOCUMENT_SCHEMA));
}

pub struct JpgEngine { artifact_state: JpgArtifact, snapshot_state: JpgSnapshot }
impl JpgEngine {
    pub fn new(snapshot: JpgSnapshot) -> Self {
        Self { artifact_state: JpgArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for JpgEngine {
    type Artifact = JpgArtifact; type Snapshot = JpgSnapshot; type Mutation = JpgMutation; type Diff = JpgDiff;
    fn artifact(&self) -> &Self::Artifact { &self.artifact_state }
    fn snapshot(&self) -> &Self::Snapshot { &self.snapshot_state }
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
