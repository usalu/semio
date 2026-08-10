//! ⚙️ GlbEngine — GLB binary container wrapping glTF JSON.
//!
//! 🧊️ TRANSITION SHIM (ticket ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2
//! gltf/glb merge step 2 / Decision #4): `🧊️glb` and `🧊️gltf` are not different artifacts, just
//! different serializations of the same glTF 2.0 standard. `🧊️gltf` now owns the real container
//! codec (byte-exact chunk padding/alignment, lenient parsing, typed accessor decode -- see its
//! `⚙️engine`); this file only converts `GlbSnapshot`'s `{gltf_json, bin}` payload shape to/from
//! `GltfSnapshot` and calls straight into it, so both artifacts produce byte-identical `.glb`
//! output for the same input during this transition window. `🧊️glb` itself is deleted (along with
//! this shim) once the 104-file/11-crate fan-out repoints every dependent plugin at `🧊️gltf` --
//! see the plan's D2 "gltf/glb merge sequence", steps 3-5 (a separate, later wave; NOT this one).

use crate::artifacts::glb::{{schema::snapshot::GlbPayload, GlbArtifact, GlbDiff, GlbMutation, GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA}};
use crate::artifacts::gltf::schema::snapshot::GltfSourceForm;
use crate::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

//#region 🔖️GltfDelegation
fn glb_payload_to_gltf_snapshot(payload: &GlbPayload) -> Result<GltfSnapshot, String> {
    let document: serde_json::Value = serde_json::from_str(&payload.gltf_json).map_err(|e| format!("glb payload gltf_json parse error: {e}"))?;
    let buffers = if payload.bin.is_empty() { Vec::new() } else { vec![payload.bin.clone()] };
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Glb })
}

fn gltf_snapshot_to_glb_payload(gltf: &GltfSnapshot) -> GlbPayload {
    let gltf_json = serde_json::to_string(&gltf.document).unwrap_or_else(|_| "{}".into());
    let bin = gltf.buffers.first().cloned().unwrap_or_default();
    GlbPayload { gltf_json, bin }
}

/// 📤️ Encodes `.glb` bytes -- delegates to `gltf::engine::encode_glb` (real chunk padding fix).
pub fn encode_glb(snap: &GlbSnapshot) -> Result<Vec<u8>, String> {
    let gltf_snapshot = glb_payload_to_gltf_snapshot(&snap.payload)?;
    crate::artifacts::gltf::engine::encode_glb(&gltf_snapshot)
}

/// 📥️ Decodes `.glb` bytes -- delegates to `gltf::engine::decode_glb` (lenient parse, no
/// POSITION precondition, real base64/accessor model).
pub fn decode_glb(data: &[u8]) -> Result<GlbSnapshot, String> {
    let gltf_snapshot = crate::artifacts::gltf::engine::decode_glb(data)?;
    Ok(GlbSnapshot { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: gltf_snapshot_to_glb_payload(&gltf_snapshot) })
}
//#endregion 🔖️GltfDelegation

pub fn empty_glb_snapshot() -> GlbSnapshot { GlbSnapshot::default() }

pub fn register() {
    crate::artifacts::glb::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::glb::schema::glb_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<GlbSnapshot, GlbMutation>(STDIO_GLB_DOCUMENT_SCHEMA));
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glb_delegates_to_gltf_codec_and_round_trips() {
        let snap = empty_glb_snapshot();
        let encoded = encode_glb(&snap).expect("encode via gltf delegate");
        assert_eq!(&encoded[0..4], b"glTF");
        let decoded = decode_glb(&encoded).expect("decode via gltf delegate");
        let decoded_doc: serde_json::Value = serde_json::from_str(&decoded.payload.gltf_json).unwrap();
        assert_eq!(decoded_doc["asset"]["version"], "2.0");
    }

    /// 🧪️ Ticket ARTIFACT-SYSTEM-OVERHAUL: glb and gltf must produce byte-identical `.glb` output
    /// for the same logical document during the transition window (Decision #4).
    #[test]
    fn glb_and_gltf_produce_byte_identical_glb_output() {
        let document = serde_json::json!({ "asset": { "version": "2.0" }, "buffers": [{ "byteLength": 4 }] });
        let bin = vec![1u8, 2, 3, 4];
        let glb_snap = GlbSnapshot { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: GlbPayload { gltf_json: serde_json::to_string(&document).unwrap(), bin: bin.clone() } };
        let gltf_snap = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![bin], source_form: GltfSourceForm::Glb };
        let via_glb = encode_glb(&glb_snap).expect("glb encode");
        let via_gltf = crate::artifacts::gltf::engine::encode_glb(&gltf_snap).expect("gltf encode");
        assert_eq!(via_glb, via_gltf);
    }
}
//#endregion 🧪️Tests
