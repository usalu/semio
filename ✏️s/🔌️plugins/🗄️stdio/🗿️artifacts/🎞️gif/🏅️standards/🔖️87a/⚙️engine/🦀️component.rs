//! ⚙️ GifEngine — GIF87a header + image descriptor.

use crate::artifacts::gif::{schema::snapshot::RasterImage, GifArtifact, GifDiff, GifMutation, GifSnapshot, STDIO_GIF_DOCUMENT_SCHEMA};

pub fn encode_gif(snap: &GifSnapshot) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    if img.width == 0 || img.height == 0 { return Err("empty image".into()); }
    if img.rgba.len() != (img.width as usize) * (img.height as usize) * 4 {
        return Err("rgba length mismatch".into());
    }
    let w = img.width.min(0xFFFF) as u16;
    let h = img.height.min(0xFFFF) as u16;
    let mut out = b"GIF87a".to_vec();
    out.extend_from_slice(&w.to_le_bytes());
    out.extend_from_slice(&h.to_le_bytes());
    out.push(0xF0);
    out.push(0);
    out.push(0);
    out.extend_from_slice(&[0, 0, 0, 0xFF, 0xFF, 0xFF]);
    out.push(0x2C);
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&w.to_le_bytes());
    out.extend_from_slice(&h.to_le_bytes());
    out.push(0);
    out.push(2);
    let mut indices = Vec::with_capacity((w as usize) * (h as usize));
    for px in img.rgba.chunks(4) {
        indices.push(if px[0] > 127 { 1 } else { 0 });
    }
    out.push(2);
    let mut sub = Vec::new();
    sub.push(1);
    sub.push(0);
    for &idx in &indices {
        sub.push(idx);
        if sub.len() == 255 {
            out.push(255);
            out.extend_from_slice(&sub);
            sub.clear();
        }
    }
    if !sub.is_empty() {
        out.push(sub.len() as u8);
        out.extend_from_slice(&sub);
    }
    out.push(0);
    out.push(0x3B);
    Ok(out)
}

pub fn decode_gif(data: &[u8]) -> Result<GifSnapshot, String> {
    if data.len() < 13 || &data[0..6] != b"GIF87a" && &data[0..6] != b"GIF89a" {
        return Err("not gif".into());
    }
    let w = u16::from_le_bytes([data[6], data[7]]) as u32;
    let h = u16::from_le_bytes([data[8], data[9]]) as u32;
    let mut rgba = vec![0u8; (w as usize) * (h as usize) * 4];
    for px in rgba.chunks_mut(4) {
        px[0] = 0;
        px[1] = 0;
        px[2] = 0;
        px[3] = 255;
    }
    Ok(GifSnapshot { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba } })
}

pub fn empty_gif_snapshot() -> GifSnapshot { GifSnapshot::default() }

pub fn register() {
    crate::artifacts::gif::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::gif::schema::gif_artifact_schema_descriptor());
    store::register_document_codec(store::DocumentCodec::of::<GifSnapshot, GifMutation>(STDIO_GIF_DOCUMENT_SCHEMA));
}

pub struct GifEngine { artifact_state: GifArtifact, snapshot_state: GifSnapshot }
impl GifEngine {
    pub fn new(snapshot: GifSnapshot) -> Self {
        Self { artifact_state: GifArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for GifEngine {
    type Artifact = GifArtifact; type Snapshot = GifSnapshot; type Mutation = GifMutation; type Diff = GifDiff;
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
