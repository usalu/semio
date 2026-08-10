//! ⚙️ GlbEngine — GLB binary container wrapping glTF JSON.

use crate::artifacts::glb::{{schema::snapshot::GlbPayload, GlbArtifact, GlbDiff, GlbMutation, GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA}};

pub fn encode_glb(snap: &GlbSnapshot) -> Result<Vec<u8>, String> {
    let json = snap.payload.gltf_json.as_bytes();
    let bin = &snap.payload.bin;
    let mut out = Vec::new();
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    let total = 12 + 8 + json.len() + if bin.is_empty() { 0 } else { 8 + bin.len() } + ((4 - (json.len() % 4)) % 4);
    out.extend_from_slice(&(total as u32).to_le_bytes());
    out.extend_from_slice(&(json.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(json);
    let pad = (4 - (json.len() % 4)) % 4;
    out.extend(vec![0x20; pad]);
    if !bin.is_empty() {
        out.extend_from_slice(&(bin.len() as u32).to_le_bytes());
        out.extend_from_slice(b"BIN\0");
        out.extend_from_slice(bin);
        let pad2 = (4 - (bin.len() % 4)) % 4;
        out.extend(vec![0; pad2]);
    }
    Ok(out)
}

pub fn decode_glb(data: &[u8]) -> Result<GlbSnapshot, String> {
    if data.len() < 12 || &data[0..4] != b"glTF" { return Err("not glb".into()); }
    let mut pos = 12usize;
    let mut json = String::new();
    let mut bin = Vec::new();
    while pos + 8 <= data.len() {
        let len = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
        let ty = &data[pos+4..pos+8];
        pos += 8;
        if pos + len > data.len() { break; }
        let chunk = &data[pos..pos+len];
        if ty == b"JSON" {
            json = String::from_utf8_lossy(chunk).trim_matches('\0').trim().to_string();
        } else if ty.starts_with(b"BIN") {
            bin = chunk.to_vec();
        }
        pos += len + (4 - (len % 4)) % 4;
    }
    Ok(GlbSnapshot { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: GlbPayload { gltf_json: json, bin } })
}

pub fn empty_glb_snapshot() -> GlbSnapshot { GlbSnapshot::default() }

pub fn register() {
    crate::artifacts::glb::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::glb::schema::glb_artifact_schema_descriptor());
    store::register_document_codec(store::DocumentCodec::of::<GlbSnapshot, GlbMutation>(STDIO_GLB_DOCUMENT_SCHEMA));
}

pub struct GlbEngine { artifact_state: GlbArtifact, snapshot_state: GlbSnapshot }
impl GlbEngine {
    pub fn new(snapshot: GlbSnapshot) -> Self {
        Self { artifact_state: GlbArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for GlbEngine {
    type Artifact = GlbArtifact; type Snapshot = GlbSnapshot; type Mutation = GlbMutation; type Diff = GlbDiff;
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
