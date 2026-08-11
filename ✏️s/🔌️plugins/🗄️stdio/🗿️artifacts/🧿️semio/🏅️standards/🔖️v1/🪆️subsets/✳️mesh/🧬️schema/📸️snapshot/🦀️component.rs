//! 🧬️ SemioMeshSnapshot — meshes -> primitives{topology, positions/normals/uvs/colors, indices,
//! material} + materials (PBR base_color/metallic/roughness) + textures{mime, bytes}. Informed by
//! gltf 2.0's `GltfMesh`/`GltfPrimitive`/`GltfAccessor`/`GltfMaterial`, per the master plan's
//! "Subset snapshot cores" table. Owned types (w1b-type-ownership.md): `SemioMesh`,
//! `SemioPrimitive`, `SemioMaterial`, `SemioTexture` (`SemioPrimitive` was RESERVED at W1b —
//! this file is where it lands).

use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioRgba, SemioUv};

//#region 🔖️Topology
/// 🔺️ Primitive draw mode — the gltf 2.0 `mode` enumeration, named (never a bare integer tag).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemioTopology {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl Default for SemioTopology {
    fn default() -> Self { Self::Triangles }
}
//#endregion 🔖️Topology

//#region 🔖️Primitive
/// 🔷️ One drawable primitive inside a `SemioMesh` — id-keyed (the strong entity gltf's
/// `mesh.primitives` array lacks; every W2 subset id-keys its repeating structures per the
/// schema-design.md recipe). `positions`/`normals`/`uvs`/`colors`/`indices` are weak, parallel
/// buffer-shaped data — whole-value replaced in diffs, never sub-diffed per vertex.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioPrimitive {
    pub id: String,
    #[serde(default)]
    pub topology: SemioTopology,
    #[serde(default)]
    pub positions: Vec<SemioPoint3>,
    #[serde(default)]
    pub normals: Vec<SemioPoint3>,
    #[serde(default)]
    pub uvs: Vec<SemioUv>,
    #[serde(default)]
    pub colors: Vec<SemioRgba>,
    #[serde(default)]
    pub indices: Vec<u32>,
    #[serde(default)]
    pub material_id: Option<String>,
}
//#endregion 🔖️Primitive

//#region 🔖️Mesh
/// 🕸️ A mesh is an id-keyed collection of `SemioPrimitive`s (gltf's `mesh.primitives`).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioMesh {
    pub id: String,
    #[serde(default)]
    pub primitives: Vec<SemioPrimitive>,
}
//#endregion 🔖️Mesh

//#region 🔖️Material
/// 🎨️ PBR metallic-roughness material (gltf's `material.pbrMetallicRoughness`, the spec-mandated
/// field set per the master plan's row: "materials (PBR base_color/metallic/roughness)").
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioMaterial {
    pub id: String,
    #[serde(default)]
    pub base_color: SemioRgba,
    #[serde(default)]
    pub metallic: f32,
    #[serde(default)]
    pub roughness: f32,
}
//#endregion 🔖️Material

//#region 🔖️Texture
/// 🖼️ Raw texture payload (gltf's `image` + embedded `bufferView`/data-uri collapsed into one
/// typed-raw-retention entity — mime + bytes, per the master plan's row: "textures{mime, bytes}").
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioTexture {
    pub id: String,
    #[serde(default)]
    pub mime: String,
    #[serde(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Texture

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOMESH_DOCUMENT_SCHEMA: &str = "stdio.semio.mesh";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.mesh")]
pub struct SemioMeshSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub meshes: Vec<SemioMesh>,
    #[state(persistent)]
    #[serde(default)]
    pub materials: Vec<SemioMaterial>,
    #[state(persistent)]
    #[serde(default)]
    pub textures: Vec<SemioTexture>,
}

impl Default for SemioMeshSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(),
            meshes: Default::default(),
            materials: Default::default(),
            textures: Default::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 📦️ JSON-pack round trip (genuinely working — not a per-format binary codec, since this
/// subset's snapshot is a NEUTRAL semio type, not an on-disk file format). Wrapped in the same
/// `store::semio_format` envelope every stdio artifact uses.
impl store::ArtifactDsl for SemioMeshSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOMESH_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioMeshSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn populated() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(),
            meshes: vec![SemioMesh {
                id: "mesh-1".into(),
                primitives: vec![SemioPrimitive {
                    id: "prim-1".into(),
                    topology: SemioTopology::Triangles,
                    positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                    normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3],
                    uvs: vec![SemioUv { u: 0.0, v: 0.0 }; 3],
                    colors: vec![SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }; 3],
                    indices: vec![0, 1, 2],
                    material_id: Some("mat-1".into()),
                }],
            }],
            materials: vec![SemioMaterial { id: "mat-1".into(), base_color: SemioRgba { r: 0.8, g: 0.2, b: 0.2, a: 1.0 }, metallic: 0.1, roughness: 0.6 }],
            textures: vec![SemioTexture { id: "tex-1".into(), mime: "image/png".into(), bytes: vec![0x89, 0x50, 0x4e, 0x47] }],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = populated();
        let bytes = <SemioMeshSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = populated();
        let text = <SemioMeshSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn default_snapshot_has_no_meshes_materials_or_textures() {
        let snap = SemioMeshSnapshot::default();
        assert!(snap.meshes.is_empty() && snap.materials.is_empty() && snap.textures.is_empty());
    }
}
//#endregion 🔖️Tests
