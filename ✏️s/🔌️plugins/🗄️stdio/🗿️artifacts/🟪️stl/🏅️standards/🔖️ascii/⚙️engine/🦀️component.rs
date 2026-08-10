//! ⚙️ StlEngine — owns a real `StlArtifact` + the real ASCII/binary STL codec.

use crate::artifacts::stl::schema::snapshot::{MeshTriangle, MeshVertex};
use crate::artifacts::stl::{StlArtifact, StlDiff, StlMutation, StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_stl_snapshot() -> StlSnapshot {
    StlSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Codec
//#region 🔖️Ascii
/// 📥 Parses `solid`/`facet normal`/`outer loop`/`vertex`×3/`endloop`/`endfacet`/`endsolid`
/// ASCII STL. Facet normals are recomputed on encode rather than trusted from the file
/// (the common real-world convention — many writers emit `facet normal 0 0 0` and rely on
/// readers to recompute), so they're intentionally not persisted.
pub fn decode_stl_ascii(text: &str) -> Result<StlSnapshot, String> {
    if !text.trim_start().starts_with("solid") {
        return Err("stl ascii: missing 'solid' header".into());
    }
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut tri: [Option<MeshVertex>; 3] = [None, None, None];
    let mut slot = 0usize;
    let mut in_loop = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("outer loop") {
            in_loop = true;
            slot = 0;
            tri = [None, None, None];
        } else if line.starts_with("endloop") {
            if slot != 3 {
                return Err(format!("stl ascii: facet had {slot} vertices, expected 3"));
            }
            let i0 = vertices.len() as u32;
            for v in tri.iter().flatten() {
                vertices.push(v.clone());
            }
            faces.push(MeshTriangle { i0, i1: i0 + 1, i2: i0 + 2 });
            in_loop = false;
        } else if in_loop && line.starts_with("vertex") {
            let coords: Vec<f32> = line
                .trim_start_matches("vertex")
                .split_whitespace()
                .map(|s| s.parse::<f32>().map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            if coords.len() < 3 {
                return Err("stl ascii: vertex needs 3 coords".into());
            }
            if slot >= 3 {
                return Err("stl ascii: more than 3 vertices in one facet".into());
            }
            tri[slot] = Some(MeshVertex { x: coords[0], y: coords[1], z: coords[2] });
            slot += 1;
        }
    }
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces })
}

fn facet_normal(a: &MeshVertex, b: &MeshVertex, c: &MeshVertex) -> (f32, f32, f32) {
    let (ux, uy, uz) = (b.x - a.x, b.y - a.y, b.z - a.z);
    let (vx, vy, vz) = (c.x - a.x, c.y - a.y, c.z - a.z);
    (uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx)
}

/// 📤 Writes real ASCII STL, recomputing each facet's normal from its vertex winding.
pub fn encode_stl_ascii(snap: &StlSnapshot) -> String {
    let mut out = String::from("solid mesh\n");
    for f in &snap.faces {
        let a = &snap.vertices[f.i0 as usize];
        let b = &snap.vertices[f.i1 as usize];
        let c = &snap.vertices[f.i2 as usize];
        let (nx, ny, nz) = facet_normal(a, b, c);
        out.push_str(&format!("  facet normal {nx} {ny} {nz}\n"));
        out.push_str("    outer loop\n");
        out.push_str(&format!("      vertex {} {} {}\n", a.x, a.y, a.z));
        out.push_str(&format!("      vertex {} {} {}\n", b.x, b.y, b.z));
        out.push_str(&format!("      vertex {} {} {}\n", c.x, c.y, c.z));
        out.push_str("    endloop\n  endfacet\n");
    }
    out.push_str("endsolid mesh\n");
    out
}
//#endregion 🔖️Ascii

//#region 🔖️Binary
/// 📥 Parses binary STL: 80-byte header (ignored) + u32 triangle count + N × (12-byte
/// normal + 3×12-byte vertices + 2-byte attribute-byte-count).
pub fn decode_stl_binary(bytes: &[u8]) -> Result<StlSnapshot, String> {
    if bytes.len() < 84 {
        return Err("stl binary: header too short".into());
    }
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let mut vertices = Vec::with_capacity(count * 3);
    let mut faces = Vec::with_capacity(count);
    let mut off = 84usize;
    for _ in 0..count {
        if off + 50 > bytes.len() {
            return Err("stl binary: truncated facet record".into());
        }
        off += 12; // skip normal, recomputed on encode like the ascii path
        let i0 = vertices.len() as u32;
        for _ in 0..3 {
            let x = f32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            let y = f32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap());
            let z = f32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap());
            off += 12;
            vertices.push(MeshVertex { x, y, z });
        }
        off += 2; // attribute byte count
        faces.push(MeshTriangle { i0, i1: i0 + 1, i2: i0 + 2 });
    }
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces })
}

/// 📤 Writes real binary STL: 80-byte header (unused, left zeroed) + u32 triangle count
/// at offset 80..84 — the count belongs INSIDE the 84-byte header, not appended after it
/// (an 80-byte zero vec here, not 84, is what makes the count land at the right offset).
pub fn encode_stl_binary(snap: &StlSnapshot) -> Vec<u8> {
    let mut out = vec![0u8; 80];
    out.extend_from_slice(&(snap.faces.len() as u32).to_le_bytes());
    for f in &snap.faces {
        let a = &snap.vertices[f.i0 as usize];
        let b = &snap.vertices[f.i1 as usize];
        let c = &snap.vertices[f.i2 as usize];
        let (nx, ny, nz) = facet_normal(a, b, c);
        for v in [nx, ny, nz] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [a, b, c] {
            out.extend_from_slice(&v.x.to_le_bytes());
            out.extend_from_slice(&v.y.to_le_bytes());
            out.extend_from_slice(&v.z.to_le_bytes());
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}
//#endregion 🔖️Binary

//#region 🔖️AutoDetect
/// 🔍 Dispatches on the `solid` ASCII magic; anything else is treated as binary STL.
pub fn decode_stl_auto(bytes: &[u8]) -> Result<StlSnapshot, String> {
    if bytes.len() >= 5 && &bytes[0..5] == b"solid" {
        // A binary STL's 80-byte header can coincidentally start with "solid" too;
        // disambiguate by checking whether the binary triangle-count framing actually
        // matches the file length before trusting the ASCII path.
        if bytes.len() >= 84 {
            let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
            let expected_binary_len = 84 + count * 50;
            if expected_binary_len == bytes.len() {
                return decode_stl_binary(bytes);
            }
        }
        decode_stl_ascii(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)
    } else {
        decode_stl_binary(bytes)
    }
}
//#endregion 🔖️AutoDetect
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::stl::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<StlSnapshot, StlMutation>(STDIO_STL_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.stl",
        extension: Some("stl"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::stl::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::stl::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::stl::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::stl::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.stl"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.stl`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::stl::schema::stl_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.stl` artifact engine.
pub struct StlEngine {
    artifact_state: StlArtifact,
    snapshot_state: StlSnapshot,
}

impl StlEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: StlSnapshot) -> Self {
        let artifact_state = StlArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for StlEngine {
    type Artifact = StlArtifact;
    type Snapshot = StlSnapshot;
    type Mutation = StlMutation;
    type Diff = StlDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }

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
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_stl_snapshot();
        assert_eq!(snapshot.schema, STDIO_STL_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_stl_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <StlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <StlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🔺 A real (non-solid-color-analogue) 4-triangle tetrahedron — enough structure to
    /// catch an off-by-one in facet/vertex slot tracking that a single-triangle fixture would miss.
    fn tetrahedron() -> StlSnapshot {
        let vertices = vec![
            MeshVertex { x: 0.0, y: 0.0, z: 0.0 },
            MeshVertex { x: 1.0, y: 0.0, z: 0.0 },
            MeshVertex { x: 0.0, y: 1.0, z: 0.0 },
            MeshVertex { x: 0.0, y: 0.0, z: 1.0 },
        ];
        // 4 faces, each referencing 3 of the 4 shared apex points, but written out with
        // independent per-facet vertex triples (ascii/binary STL do not share indices).
        let mut expanded_vertices = Vec::new();
        let mut faces = Vec::new();
        for (a, b, c) in [(0usize, 1usize, 2usize), (0, 1, 3), (1, 2, 3), (0, 2, 3)] {
            let i0 = expanded_vertices.len() as u32;
            expanded_vertices.push(vertices[a].clone());
            expanded_vertices.push(vertices[b].clone());
            expanded_vertices.push(vertices[c].clone());
            faces.push(MeshTriangle { i0, i1: i0 + 1, i2: i0 + 2 });
        }
        StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices: expanded_vertices, faces }
    }

    #[test]
    fn ascii_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let text = encode_stl_ascii(&snap);
        assert!(text.starts_with("solid"));
        assert_eq!(text.matches("facet normal").count(), 4);
        let decoded = decode_stl_ascii(&text).expect("decode");
        assert_eq!(decoded.vertices, snap.vertices);
        assert_eq!(decoded.faces, snap.faces);
    }

    #[test]
    fn binary_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_stl_binary(&snap);
        assert_eq!(bytes.len(), 84 + 4 * 50);
        let decoded = decode_stl_binary(&bytes).expect("decode");
        assert_eq!(decoded.vertices, snap.vertices);
        assert_eq!(decoded.faces, snap.faces);
    }

    #[test]
    fn auto_detect_dispatches_ascii_vs_binary() {
        let snap = tetrahedron();
        let ascii_bytes = encode_stl_ascii(&snap).into_bytes();
        let binary_bytes = encode_stl_binary(&snap);
        assert_eq!(decode_stl_auto(&ascii_bytes).expect("ascii").faces.len(), 4);
        assert_eq!(decode_stl_auto(&binary_bytes).expect("binary").faces.len(), 4);
    }
}
//#endregion 🧪️Tests
