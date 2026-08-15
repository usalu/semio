//! 📥️ Deserialize `s.stdio.semio/v1/mesh` from `s.stdio.ply/1.0/*` — geometry-only, but unlike
//! OBJ/STL, PLY's `face` element indices genuinely reference a SHARED `vertex` element pool
//! (single-indexed, exactly matching `SemioPrimitive`'s own shape) — so this leaf produces a REAL
//! indexed mesh, not a flattened triangle soup.
//!
//! Reads the conventional (real-world, not spec-mandated but near-universal — Stanford/PLY
//! reference tooling) `vertex` element columns `x`/`y`/`z` (mandatory), `nx`/`ny`/`nz` (normals,
//! all-or-nothing), `red`/`green`/`blue`[/`alpha`] (colors, all-or-nothing; integer scalar kinds
//! are range-normalized to `[0,1]`, e.g. `uchar` `/255`), `u`/`v` or `s`/`t` (texcoords,
//! all-or-nothing — either spelling accepted); and the `face` element's list property named
//! `vertex_indices` or `vertex_index` (both real-world spellings).
//!
//! 🔖 Documented lossiness:
//! - No `face` element -> the file is a point cloud: topology `Points`, `indices` stays empty
//!   (same "empty means sequential" convention the gltf/stl/obj sibling leaves use).
//! - `face` element present -> topology `Triangles`; n-gon face rows (>3 indices) are
//!   fan-triangulated the same way the OBJ sibling leaf does.
//! - Any OTHER `element` (PLY's generic element/property system allows arbitrary named blocks) is
//!   dropped — `SemioMeshSnapshot` has no place for unmodeled per-element data.
//! - No color/PBR/texture-byte concept beyond flat per-vertex RGBA -> `materials`/`textures` stay
//!   empty, `material_id` stays `None`.

use crate::artifacts::ply::schema::snapshot::{PlyProperty, PlyScalarType, PlyValue};
use crate::artifacts::ply::PlySnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

//#region 🔖️ValueHelpers
fn property_index(properties: &[PlyProperty], name: &str) -> Option<usize> {
    properties.iter().position(|p| p.name() == name)
}

fn scalar_kind_of(properties: &[PlyProperty], idx: usize) -> Option<PlyScalarType> {
    match &properties[idx] {
        PlyProperty::Scalar { kind, .. } => Some(*kind),
        PlyProperty::List { .. } => None,
    }
}

fn value_as_f64(v: &PlyValue) -> Result<f64, String> {
    match v {
        PlyValue::Char(x) => Ok(*x as f64),
        PlyValue::UChar(x) => Ok(*x as f64),
        PlyValue::Short(x) => Ok(*x as f64),
        PlyValue::UShort(x) => Ok(*x as f64),
        PlyValue::Int(x) => Ok(*x as f64),
        PlyValue::UInt(x) => Ok(*x as f64),
        PlyValue::Float(x) => Ok(*x as f64),
        PlyValue::Double(x) => Ok(*x),
        PlyValue::List(_) => Err("expected a scalar PlyValue, found a List".into()),
    }
}

/// 🎨️ Range-normalizes an integer color channel to `[0,1]`; float/double channels pass through
/// (already-normalized, the near-universal real-world convention).
fn normalize_color_channel(v: f64, kind: PlyScalarType) -> f64 {
    match kind {
        PlyScalarType::Char => (v / 127.0).max(-1.0),
        PlyScalarType::UChar => v / 255.0,
        PlyScalarType::Short => (v / 32767.0).max(-1.0),
        PlyScalarType::UShort => v / 65535.0,
        PlyScalarType::Int | PlyScalarType::UInt | PlyScalarType::Float | PlyScalarType::Double => v,
    }
}
//#endregion 🔖️ValueHelpers

pub struct SemioMeshFromPly;

impl ArtifactDeserializer for SemioMeshFromPly {
    type From = PlySnapshot;
    type Into = SemioMeshSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let vertex_el = from.elements.iter().find(|e| e.name == "vertex").ok_or_else(|| store::PackError::Schema("SemioMeshFromPly: file has no 'vertex' element".to_string()))?;

        let x_idx = property_index(&vertex_el.properties, "x").ok_or_else(|| store::PackError::Schema("SemioMeshFromPly: 'vertex' element has no 'x' property".to_string()))?;
        let y_idx = property_index(&vertex_el.properties, "y").ok_or_else(|| store::PackError::Schema("SemioMeshFromPly: 'vertex' element has no 'y' property".to_string()))?;
        let z_idx = property_index(&vertex_el.properties, "z").ok_or_else(|| store::PackError::Schema("SemioMeshFromPly: 'vertex' element has no 'z' property".to_string()))?;

        let normal_idx = ["nx", "ny", "nz"].iter().map(|n| property_index(&vertex_el.properties, n)).collect::<Option<Vec<_>>>();
        let color_idx: Option<Vec<usize>> = ["red", "green", "blue"].iter().map(|n| property_index(&vertex_el.properties, n)).collect();
        let alpha_idx = property_index(&vertex_el.properties, "alpha");
        let uv_idx = ["u", "v"].iter().map(|n| property_index(&vertex_el.properties, n)).collect::<Option<Vec<_>>>().or_else(|| ["s", "t"].iter().map(|n| property_index(&vertex_el.properties, n)).collect::<Option<Vec<_>>>());

        let read =
            |row_idx: usize, prop_idx: usize| -> Result<f64, store::PackError> { value_as_f64(&vertex_el.rows[row_idx].values[prop_idx]).map_err(|e| store::PackError::Schema(format!("SemioMeshFromPly: row {row_idx} property {prop_idx}: {e}"))) };

        let mut positions = Vec::with_capacity(vertex_el.rows.len());
        let mut normals = Vec::new();
        let mut colors = Vec::new();
        let mut uvs = Vec::new();
        for i in 0..vertex_el.rows.len() {
            positions.push(SemioPoint3 { x: read(i, x_idx)?, y: read(i, y_idx)?, z: read(i, z_idx)? });
            if let Some(ns) = &normal_idx {
                normals.push(SemioPoint3 { x: read(i, ns[0])?, y: read(i, ns[1])?, z: read(i, ns[2])? });
            }
            if let Some(cs) = &color_idx {
                let kind = scalar_kind_of(&vertex_el.properties, cs[0]).unwrap_or(PlyScalarType::Float);
                let a = match alpha_idx {
                    Some(ai) => normalize_color_channel(read(i, ai)?, scalar_kind_of(&vertex_el.properties, ai).unwrap_or(PlyScalarType::Float)) as f32,
                    None => 1.0,
                };
                colors.push(SemioRgba { r: normalize_color_channel(read(i, cs[0])?, kind) as f32, g: normalize_color_channel(read(i, cs[1])?, kind) as f32, b: normalize_color_channel(read(i, cs[2])?, kind) as f32, a });
            }
            if let Some(us) = &uv_idx {
                uvs.push(SemioUv { u: read(i, us[0])?, v: read(i, us[1])? });
            }
        }

        let face_el = from.elements.iter().find(|e| e.name == "face");
        let (topology, indices) = match face_el {
            None => (SemioTopology::Points, Vec::new()),
            Some(face_el) => {
                let list_idx = property_index(&face_el.properties, "vertex_indices")
                    .or_else(|| property_index(&face_el.properties, "vertex_index"))
                    .ok_or_else(|| store::PackError::Schema("SemioMeshFromPly: 'face' element has no 'vertex_indices'/'vertex_index' list property".to_string()))?;
                let mut indices = Vec::new();
                for (ri, row) in face_el.rows.iter().enumerate() {
                    let list = match &row.values[list_idx] {
                        PlyValue::List(items) => items,
                        _ => return Err(store::PackError::Schema(format!("SemioMeshFromPly: face row {ri} 'vertex_indices' is not a List"))),
                    };
                    if list.len() < 3 {
                        return Err(store::PackError::Schema(format!("SemioMeshFromPly: face row {ri} has {} indices, need at least 3", list.len())));
                    }
                    let face_indices: Vec<u32> = list.iter().map(|v| value_as_f64(v).map(|f| f as u32)).collect::<Result<_, _>>().map_err(|e| store::PackError::Schema(format!("SemioMeshFromPly: face row {ri}: {e}")))?;
                    for i in 1..face_indices.len() - 1 {
                        indices.push(face_indices[0]);
                        indices.push(face_indices[i]);
                        indices.push(face_indices[i + 1]);
                    }
                }
                (SemioTopology::Triangles, indices)
            }
        };

        let primitive = SemioPrimitive { id: "mesh-0-prim-0".to_string(), topology, positions, normals, uvs, colors, indices, material_id: None };
        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: vec![SemioMesh { id: "mesh-0".to_string(), primitives: vec![primitive] }], materials: Vec::new(), textures: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::ply::schema::snapshot::{PlyElement, PlyFormat, PlyRow};

    fn sample_ply() -> PlySnapshot {
        let vertex = PlyElement {
            name: "vertex".into(),
            count: 4,
            properties: vec![
                PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float },
                PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float },
                PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float },
                PlyProperty::Scalar { name: "red".into(), kind: PlyScalarType::UChar },
                PlyProperty::Scalar { name: "green".into(), kind: PlyScalarType::UChar },
                PlyProperty::Scalar { name: "blue".into(), kind: PlyScalarType::UChar },
            ],
            rows: vec![
                PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::UChar(255), PlyValue::UChar(0), PlyValue::UChar(0)] },
                PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::UChar(0), PlyValue::UChar(255), PlyValue::UChar(0)] },
                PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0), PlyValue::Float(0.0), PlyValue::UChar(0), PlyValue::UChar(0), PlyValue::UChar(255)] },
                PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(1.0), PlyValue::Float(0.0), PlyValue::UChar(255), PlyValue::UChar(255), PlyValue::UChar(0)] },
            ],
        };
        let face = PlyElement {
            name: "face".into(),
            count: 1,
            properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
            rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2), PlyValue::Int(3)])] }],
        };
        PlySnapshot { schema: "stdio.ply".into(), format: PlyFormat::Ascii, comments: Vec::new(), elements: vec![vertex, face] }
    }

    #[test]
    fn deserialize_builds_a_real_indexed_mesh_with_colors() {
        let semio = SemioMeshFromPly::deserialize(&sample_ply()).expect("deserialize");
        let prim = &semio.meshes[0].primitives[0];
        assert_eq!(prim.topology, SemioTopology::Triangles);
        assert_eq!(prim.positions.len(), 4, "vertex pool stays 4 entries -- a real shared index space");
        assert_eq!(prim.colors.len(), 4);
        assert_eq!(prim.colors[0], SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
        assert_eq!(prim.indices, vec![0, 1, 2, 0, 2, 3], "quad face fan-triangulated, referencing the shared vertex pool");
        assert!(prim.normals.is_empty() && prim.uvs.is_empty());
    }

    #[test]
    fn no_face_element_yields_a_points_primitive() {
        let mut ply = sample_ply();
        ply.elements.retain(|e| e.name != "face");
        let semio = SemioMeshFromPly::deserialize(&ply).expect("deserialize");
        let prim = &semio.meshes[0].primitives[0];
        assert_eq!(prim.topology, SemioTopology::Points);
        assert!(prim.indices.is_empty());
        assert_eq!(prim.positions.len(), 4);
    }
}
//#endregion 🔖️Tests
