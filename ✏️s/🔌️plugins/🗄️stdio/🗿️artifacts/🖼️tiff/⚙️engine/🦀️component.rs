//! ⚙️ TiffEngine — classic TIFF IFD + uncompressed RGB strip.

use crate::artifacts::tiff::{schema::snapshot::RasterImage, TiffArtifact, TiffDiff, TiffMutation, TiffSnapshot, STDIO_TIFF_DOCUMENT_SCHEMA};

pub fn encode_tiff(snap: &TiffSnapshot) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    if img.width == 0 || img.height == 0 { return Err("empty image".into()); }
    let pixels = (img.width as usize) * (img.height as usize);
    if img.rgba.len() != pixels * 4 { return Err("rgba length mismatch".into()); }
    let mut rgb = Vec::with_capacity(pixels * 3);
    for px in img.rgba.chunks(4) {
        rgb.extend_from_slice(&px[0..3]);
    }
    let ifd_off = 8u32;
    let strip_off = ifd_off + 2 + 12 * 5 + 4;
    let mut out = Vec::new();
    out.extend_from_slice(b"II");
    out.extend_from_slice(&42u16.to_le_bytes());
    out.extend_from_slice(&ifd_off.to_le_bytes());
    out.extend_from_slice(&5u16.to_le_bytes());
  let entries: [(u16, u16, u32, u32); 5] = [
        (256, 3, 1, img.width),
        (257, 3, 1, img.height),
        (258, 3, 1, 8),
        (262, 3, 1, 2),
        (273, 4, 1, strip_off),
    ];
    for (tag, typ, cnt, val) in entries {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&typ.to_le_bytes());
        out.extend_from_slice(&cnt.to_le_bytes());
        out.extend_from_slice(&val.to_le_bytes());
    }
    out.extend_from_slice(&0u32.to_le_bytes());
    out.resize(strip_off as usize, 0);
    out.extend_from_slice(&rgb);
    Ok(out)
}

pub fn decode_tiff(data: &[u8]) -> Result<TiffSnapshot, String> {
    if data.len() < 8 || &data[0..2] != b"II" { return Err("not little-endian tiff".into()); }
    let ifd = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    if ifd + 2 > data.len() { return Err("bad ifd".into()); }
    let count = u16::from_le_bytes([data[ifd], data[ifd + 1]]) as usize;
    let mut width = 1u32;
    let mut height = 1u32;
    let mut strip_off = 0u32;
    let mut pos = ifd + 2;
    for _ in 0..count {
        if pos + 12 > data.len() { break; }
        let tag = u16::from_le_bytes([data[pos], data[pos + 1]]);
        let val = u32::from_le_bytes([data[pos + 8], data[pos + 9], data[pos + 10], data[pos + 11]]);
        match tag {
            256 => width = val,
            257 => height = val,
            273 => strip_off = val,
            _ => {}
        }
        pos += 12;
    }
    let pixels = (width as usize) * (height as usize);
    let mut rgba = vec![0u8; pixels * 4];
    let start = strip_off as usize;
    for (i, px) in rgba.chunks_mut(4).enumerate() {
        let o = start + i * 3;
        if o + 2 < data.len() {
            px[0] = data[o];
            px[1] = data[o + 1];
            px[2] = data[o + 2];
            px[3] = 255;
        }
    }
    Ok(TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), image: RasterImage { width, height, rgba } })
}

pub fn empty_tiff_snapshot() -> TiffSnapshot { TiffSnapshot::default() }

pub fn register() {
    crate::artifacts::tiff::io::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::tiff::schema::tiff_artifact_schema_descriptor());
    store::register_document_codec(store::DocumentCodec::of::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA));
}

pub struct TiffEngine { artifact_state: TiffArtifact, snapshot_state: TiffSnapshot }
impl TiffEngine {
    pub fn new(snapshot: TiffSnapshot) -> Self {
        Self { artifact_state: TiffArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for TiffEngine {
    type Artifact = TiffArtifact; type Snapshot = TiffSnapshot; type Mutation = TiffMutation; type Diff = TiffDiff;
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
