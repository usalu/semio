//! ⚙️ PngEngine — real png codec.

use crate::artifacts::png::{schema::snapshot::RasterImage, PngArtifact, PngDiff, PngMutation, PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

fn png_crc32(data: &[u8]) -> u32 {
    crate::artifacts::zip::engine::crc32(data)
}

fn write_chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ty);
    out.extend_from_slice(data);
    let mut crc_in = Vec::new();
    crc_in.extend_from_slice(ty);
    crc_in.extend_from_slice(data);
    out.extend_from_slice(&png_crc32(&crc_in).to_be_bytes());
}

pub fn rgba_len(img: &RasterImage) -> Result<usize, String> {
    let n = (img.width as usize).checked_mul(img.height as usize).and_then(|p| p.checked_mul(4))
        .ok_or("dimensions overflow")?;
    if img.rgba.len() != n { return Err("rgba length mismatch".into()); }
    Ok(n)
}

pub fn encode_png(snap: &PngSnapshot) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    rgba_len(img)?;
    let mut idat = Vec::new();
    let row = (img.width as usize) * 4;
    for y in 0..img.height as usize {
        idat.push(0);
        let start = y * row;
        idat.extend_from_slice(&img.rgba[start..start + row]);
    }
    let compressed = crate::artifacts::deflate::engine::zlib_compress(&idat)?;
    let mut out = Vec::new();
    out.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&img.width.to_be_bytes());
    ihdr.extend_from_slice(&img.height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);
    Ok(out)
}

pub fn decode_png(data: &[u8]) -> Result<PngSnapshot, String> {
    if data.len() < 8 || &data[0..8] != [137, 80, 78, 71, 13, 10, 26, 10] {
        return Err("not a png".into());
    }
    let mut pos = 8usize;
    let mut width = 0u32;
    let mut height = 0u32;
    let mut idat = Vec::new();
    while pos + 12 <= data.len() {
        let len = u32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
        let ty = &data[pos+4..pos+8];
        let start = pos + 8;
        let end = start + len;
        if end + 4 > data.len() { break; }
        let chunk = &data[start..end];
        if ty == b"IHDR" && chunk.len() >= 8 {
            width = u32::from_be_bytes([chunk[0],chunk[1],chunk[2],chunk[3]]);
            height = u32::from_be_bytes([chunk[4],chunk[5],chunk[6],chunk[7]]);
        } else if ty == b"IDAT" {
            idat.extend_from_slice(chunk);
        }
        pos = end + 4;
        if ty == b"IEND" { break; }
    }
    let raw = crate::artifacts::deflate::engine::zlib_decompress(&idat)?;
    let row = (width as usize) * 4;
    let mut rgba = Vec::with_capacity(row * height as usize);
    let mut p = 0usize;
    for _ in 0..height {
        if p >= raw.len() { break; }
        p += 1;
        if p + row > raw.len() { return Err("truncated png scanlines".into()); }
        rgba.extend_from_slice(&raw[p..p+row]);
        p += row;
    }
    Ok(PngSnapshot { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), image: RasterImage { width, height, rgba } })
}

pub fn empty_png_snapshot() -> PngSnapshot { PngSnapshot::default() }

pub fn register() {
    crate::artifacts::png::io::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::png::schema::png_artifact_schema_descriptor());
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.png", extension: Some("png"), role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::png::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::png::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::png::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::png::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.png"),
    });
    store::register_document_codec(store::DocumentCodec::of::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA));
}

pub struct PngEngine { artifact_state: PngArtifact, snapshot_state: PngSnapshot }
impl PngEngine {
    pub fn new(snapshot: PngSnapshot) -> Self {
        Self { artifact_state: PngArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for PngEngine {
    type Artifact = PngArtifact; type Snapshot = PngSnapshot; type Mutation = PngMutation; type Diff = PngDiff;
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
