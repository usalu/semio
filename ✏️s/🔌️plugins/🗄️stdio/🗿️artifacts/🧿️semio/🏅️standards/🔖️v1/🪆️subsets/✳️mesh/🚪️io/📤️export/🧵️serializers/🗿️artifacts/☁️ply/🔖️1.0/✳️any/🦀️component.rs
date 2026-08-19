//! 📤️ Serialize `s.stdio.semio/v1/mesh` into `s.stdio.ply/1.0/*` — mirror of the sibling
//! deserializer leaf. All meshes/primitives merge into ONE `vertex` element (a real shared index
//! pool — `SemioPrimitive.indices`, when present, map straight across since PLY face indices are
//! already single-indexed against the vertex element, exactly like `SemioPrimitive`) plus ONE
//! `face` element for every `Triangles` primitive's triangles; `Points` primitives contribute
//! vertex rows with no face rows (a real, valid PLY point cloud within a mesh-shaped file).
//!
//! 🔖 Documented lossiness:
//! - `Lines`/`LineStrip`/`TriangleStrip`/`TriangleFan` topology has no PLY face-list counterpart
//!   this codec defines -> hard `Err` (never silently coerced to `Triangles`).
//! - PLY's single `vertex` element has ONE shared column set for every row — if some primitives
//!   carry `normals`/`uvs`/`colors` and others don't, there is no honest per-primitive column to
//!   emit; export requires uniform presence (all-or-none) of each attribute across every
//!   primitive and hard-errors on a real mismatch rather than zero-filling (which would look like
//!   fabricated geometry data to a downstream reader).
//! - `material_id` is dropped (PLY has no material/name-reference concept in this schema, unlike
//!   OBJ's `usemtl`).
//! - Colors are written as `uchar` `red`/`green`/`blue`[/`alpha`] (`round(channel * 255)`,
//!   clamped `[0,255]`) — the near-universal real-world PLY color convention.

use crate::artifacts::ply::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType, PlyValue};
use crate::artifacts::ply::PlySnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioTopology};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId::ANY };

//#region 🔖️UniformPresence
/// 🧭️ PLY's single `vertex` element needs one shared column set; a real, honest mismatch (some
/// primitives populate an attribute, others don't) is a hard error, never a silent zero-fill.
fn check_uniform_presence(meshes: &[SemioMesh]) -> Result<(bool, bool, bool), String> {
    let mut normals: Option<bool> = None;
    let mut uvs: Option<bool> = None;
    let mut colors: Option<bool> = None;
    for mesh in meshes {
        for prim in &mesh.primitives {
            let (n, u, c) = (!prim.normals.is_empty(), !prim.uvs.is_empty(), !prim.colors.is_empty());
            for (seen, now, label) in [(&mut normals, n, "normals"), (&mut uvs, u, "uvs"), (&mut colors, c, "colors")] {
                match seen {
                    Some(existing) if *existing != now => return Err(format!("primitive {:?} disagrees with an earlier primitive on whether {label} is populated -- PLY's vertex element needs a uniform column set", prim.id)),
                    Some(_) => {}
                    None => *seen = Some(now),
                }
            }
        }
    }
    Ok((normals.unwrap_or(false), uvs.unwrap_or(false), colors.unwrap_or(false)))
}

fn clamp_u8(v: f32) -> u8 {
    (v * 255.0).round().clamp(0.0, 255.0) as u8
}
//#endregion 🔖️UniformPresence

pub struct SemioMeshToPly;

impl ArtifactSerializer for SemioMeshToPly {
    type From = SemioMeshSnapshot;
    type Into = PlySnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let (has_normals, has_uvs, has_colors) = check_uniform_presence(&from.meshes).map_err(|e| store::PackError::Schema(format!("SemioMeshToPly: {e}")))?;

        let mut properties = vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float }];
        if has_normals {
            properties.push(PlyProperty::Scalar { name: "nx".into(), kind: PlyScalarType::Float });
            properties.push(PlyProperty::Scalar { name: "ny".into(), kind: PlyScalarType::Float });
            properties.push(PlyProperty::Scalar { name: "nz".into(), kind: PlyScalarType::Float });
        }
        if has_colors {
            properties.push(PlyProperty::Scalar { name: "red".into(), kind: PlyScalarType::UChar });
            properties.push(PlyProperty::Scalar { name: "green".into(), kind: PlyScalarType::UChar });
            properties.push(PlyProperty::Scalar { name: "blue".into(), kind: PlyScalarType::UChar });
            properties.push(PlyProperty::Scalar { name: "alpha".into(), kind: PlyScalarType::UChar });
        }
        if has_uvs {
            properties.push(PlyProperty::Scalar { name: "u".into(), kind: PlyScalarType::Float });
            properties.push(PlyProperty::Scalar { name: "v".into(), kind: PlyScalarType::Float });
        }

        let mut vertex_rows: Vec<PlyRow> = Vec::new();
        let mut face_rows: Vec<PlyRow> = Vec::new();

        for mesh in &from.meshes {
            for prim in &mesh.primitives {
                if !matches!(prim.topology, SemioTopology::Triangles | SemioTopology::Points) {
                    return Err(store::PackError::Schema(format!("SemioMeshToPly: primitive {:?} has topology {:?}; this codec only exports Triangles/Points", prim.id, prim.topology)));
                }
                if prim.positions.is_empty() {
                    return Err(store::PackError::Schema(format!("SemioMeshToPly: primitive {:?} has no positions", prim.id)));
                }
                let base = vertex_rows.len() as u32;
                for (i, p) in prim.positions.iter().enumerate() {
                    let mut values = vec![PlyValue::Float(p.x as f32), PlyValue::Float(p.y as f32), PlyValue::Float(p.z as f32)];
                    if has_normals {
                        let n = prim.normals.get(i).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToPly: primitive {:?} missing normal at index {i}", prim.id)))?;
                        values.push(PlyValue::Float(n.x as f32));
                        values.push(PlyValue::Float(n.y as f32));
                        values.push(PlyValue::Float(n.z as f32));
                    }
                    if has_colors {
                        let c = prim.colors.get(i).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToPly: primitive {:?} missing color at index {i}", prim.id)))?;
                        values.push(PlyValue::UChar(clamp_u8(c.r)));
                        values.push(PlyValue::UChar(clamp_u8(c.g)));
                        values.push(PlyValue::UChar(clamp_u8(c.b)));
                        values.push(PlyValue::UChar(clamp_u8(c.a)));
                    }
                    if has_uvs {
                        let uv = prim.uvs.get(i).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToPly: primitive {:?} missing uv at index {i}", prim.id)))?;
                        values.push(PlyValue::Float(uv.u as f32));
                        values.push(PlyValue::Float(uv.v as f32));
                    }
                    vertex_rows.push(PlyRow { values });
                }

                if prim.topology == SemioTopology::Triangles {
                    let corner_indices: Vec<u32> = if !prim.indices.is_empty() {
                        if prim.indices.len() % 3 != 0 {
                            return Err(store::PackError::Schema(format!("SemioMeshToPly: primitive {:?} indices length {} is not a multiple of 3", prim.id, prim.indices.len())));
                        }
                        prim.indices.clone()
                    } else {
                        if prim.positions.len() % 3 != 0 {
                            return Err(store::PackError::Schema(format!("SemioMeshToPly: non-indexed primitive {:?} has {} positions, not a multiple of 3", prim.id, prim.positions.len())));
                        }
                        (0..prim.positions.len() as u32).collect()
                    };
                    for tri in corner_indices.chunks(3) {
                        let list = tri.iter().map(|&i| PlyValue::Int((base + i) as i32)).collect();
                        face_rows.push(PlyRow { values: vec![PlyValue::List(list)] });
                    }
                }
            }
        }

        let vertex_count = vertex_rows.len();
        let mut elements = vec![PlyElement { name: "vertex".into(), count: vertex_count, properties, rows: vertex_rows }];
        if !face_rows.is_empty() {
            let face_count = face_rows.len();
            elements.push(PlyElement { name: "face".into(), count: face_count, properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }], rows: face_rows });
        }

        Ok(PlySnapshot { schema: "stdio.ply".into(), format: PlyFormat::Ascii, comments: Vec::new(), elements })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba};
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioPrimitive;
    use semio_framework_plugin::ArtifactDeserializer;

    fn sample_semio_mesh() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            schema: "stdio.semio.mesh".into(),
            meshes: vec![SemioMesh {
                id: "quad".into(),
                primitives: vec![SemioPrimitive {
                    id: "quad-prim-0".into(),
                    topology: SemioTopology::Triangles,
                    positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 1.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                    normals: Vec::new(),
                    uvs: Vec::new(),
                    colors: vec![SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, SemioRgba { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }, SemioRgba { r: 1.0, g: 1.0, b: 0.0, a: 1.0 }],
                    indices: vec![0, 1, 2, 0, 2, 3],
                    material_id: None,
                }],
            }],
            materials: Vec::new(),
            textures: Vec::new(),
        }
    }

    #[test]
    fn serialize_then_deserialize_round_trips_at_the_semio_level() {
        let original = sample_semio_mesh();
        let ply = semio_framework_plugin::resolve_ready(SemioMeshToPly::serialize(&original)).expect("serialize");
        assert_eq!(ply.elements[0].name, "vertex");
        assert_eq!(ply.elements[0].rows.len(), 4);
        assert_eq!(ply.elements[1].name, "face");
        assert_eq!(ply.elements[1].rows.len(), 2);
        let round_tripped = SemioMeshFromPly::deserialize(&ply).expect("deserialize");
        assert_eq!(original.meshes[0].primitives[0].positions, round_tripped.meshes[0].primitives[0].positions);
        assert_eq!(original.meshes[0].primitives[0].colors, round_tripped.meshes[0].primitives[0].colors);
        assert_eq!(original.meshes[0].primitives[0].indices, round_tripped.meshes[0].primitives[0].indices);
    }

    #[test]
    fn non_uniform_color_presence_is_a_hard_error() {
        let mut semio = sample_semio_mesh();
        semio.meshes[0].primitives.push(SemioPrimitive {
            id: "prim-no-color".into(),
            topology: SemioTopology::Points,
            positions: vec![SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }],
            normals: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
            material_id: None,
        });
        let err = semio_framework_plugin::resolve_ready(SemioMeshToPly::serialize(&semio)).expect_err("mixed color presence must error");
        assert!(format!("{err:?}").contains("colors"), "got {err:?}");
    }

    #[test]
    fn non_triangle_non_points_topology_is_a_hard_error() {
        let mut semio = sample_semio_mesh();
        semio.meshes[0].primitives[0].topology = SemioTopology::LineStrip;
        let err = semio_framework_plugin::resolve_ready(SemioMeshToPly::serialize(&semio)).expect_err("LineStrip must error");
        assert!(format!("{err:?}").contains("Triangles/Points"), "got {err:?}");
    }
}
//#endregion 🔖️Tests
