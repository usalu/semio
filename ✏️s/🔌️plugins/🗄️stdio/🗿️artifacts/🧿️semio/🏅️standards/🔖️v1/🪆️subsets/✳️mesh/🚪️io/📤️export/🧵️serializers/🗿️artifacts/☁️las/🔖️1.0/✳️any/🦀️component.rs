//! 📤️ Serialize `s.stdio.semio/v1/mesh` into `s.stdio.las/1.0/*` — mirror of the sibling
//! deserializer leaf. Every mesh/primitive's `positions` flatten into ONE point list (LAS has no
//! face/topology concept at all to preserve, so a `Triangles`/`Lines`/etc. primitive's
//! connectivity is dropped by definition here, not by an arbitrary choice — see the module doc
//! comment's lossiness list).
//!
//! 🔖 Documented lossiness:
//! - Any primitive's `indices`/`topology` beyond `Points` is dropped — only vertex POSITIONS
//!   survive; exporting a triangle mesh to LAS keeps its vertices as a point cloud, exactly the
//!   real semantic difference between a mesh and a point cloud (this is not silently discarding
//!   data unexpectedly — it is what "export a mesh to a point-cloud format" honestly means).
//! - `colors`, when populated with one entry per position, map to LAS `rgb` (`[0,1]` -> `u16`
//!   `round(channel * 65535)`, clamped); `SemioRgba.a` (alpha) has no LAS point-format counterpart
//!   and is dropped. Colors are LAS-format-uniform (`encode_las`'s own `choose_point_format`
//!   requires all-or-nothing `rgb` across every point) — a primitive with a colors/positions
//!   length mismatch exports with NO color for that primitive's points rather than a partial/
//!   fabricated column.
//! - `normals`/`uvs`/`material_id` have no LAS counterpart and are dropped.
//! - LAS coordinates are inherently scaled 32-bit integers (`(x - offset) / scale`, rounded) —
//!   this leaf writes a fine `0.0001`-unit scale/offset-zero header (finer than the format's own
//!   `0.01` default) to keep quantization error far below any input precision this codec's own
//!   `f64` positions realistically carry, but SOME quantization is an inherent, real LAS property,
//!   not an artifact of this codec — never claimed to be bit-exact.

use crate::artifacts::las::schema::snapshot::{LasHeader, LasPoint};
use crate::artifacts::las::LasSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

pub struct SemioMeshToLas;

impl ArtifactSerializer for SemioMeshToLas {
    type From = SemioMeshSnapshot;
    type Into = LasSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut points = Vec::new();
        for mesh in &from.meshes {
            for prim in &mesh.primitives {
                let has_colors = !prim.colors.is_empty() && prim.colors.len() == prim.positions.len();
                for (i, p) in prim.positions.iter().enumerate() {
                    let rgb = if has_colors {
                        let c = prim.colors[i];
                        Some(((c.r as f64 * 65535.0).round().clamp(0.0, 65535.0) as u16, (c.g as f64 * 65535.0).round().clamp(0.0, 65535.0) as u16, (c.b as f64 * 65535.0).round().clamp(0.0, 65535.0) as u16))
                    } else {
                        None
                    };
                    points.push(LasPoint {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                        intensity: 0,
                        return_number: 1,
                        number_of_returns: 1,
                        scan_direction_flag: false,
                        edge_of_flight_line: false,
                        classification: 0,
                        scan_angle_rank: 0,
                        user_data: 0,
                        point_source_id: 0,
                        gps_time: None,
                        rgb,
                    });
                }
            }
        }

        let header = LasHeader { x_scale: 0.0001, y_scale: 0.0001, z_scale: 0.0001, x_offset: 0.0, y_offset: 0.0, z_offset: 0.0, ..LasHeader::default() };
        Ok(LasSnapshot { schema: "stdio.las".into(), header, vlrs: Vec::new(), points })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba};
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive, SemioTopology};
    use semio_framework_plugin::ArtifactDeserializer;

    async fn sample_semio_mesh() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            schema: "stdio.semio.mesh".into(),
            meshes: vec![SemioMesh {
                id: "cloud".into(),
                primitives: vec![SemioPrimitive {
                    id: "cloud-prim-0".into(),
                    topology: SemioTopology::Points,
                    positions: vec![SemioPoint3 { x: 1.23, y: 4.56, z: 7.89 }, SemioPoint3 { x: -2.5, y: 0.0, z: 10.0 }],
                    normals: Vec::new(),
                    uvs: Vec::new(),
                    colors: vec![SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }],
                    indices: Vec::new(),
                    material_id: None,
                }],
            }],
            materials: Vec::new(),
            textures: Vec::new(),
        }
    }

    #[test]
    async fn serialize_then_deserialize_round_trips_positions_and_colors_within_las_quantization() {
        let original = sample_semio_mesh();
        let las = semio_framework_plugin::resolve_ready(SemioMeshToLas::serialize(&original)).expect("serialize");
        assert_eq!(las.points.len(), 2);
        assert_eq!(las.points[0].rgb, Some((65535, 0, 0)));
        let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromLas::deserialize(&las)).expect("deserialize");
        let orig_prim = &original.meshes[0].primitives[0];
        let rt_prim = &round_tripped.meshes[0].primitives[0];
        assert_eq!(rt_prim.topology, SemioTopology::Points);
        assert_eq!(rt_prim.positions.len(), orig_prim.positions.len());
        for (a, b) in orig_prim.positions.iter().zip(&rt_prim.positions) {
            assert!((a.x - b.x).abs() < 1e-6, "x drifted beyond LAS's documented scale quantization: {a:?} vs {b:?}");
            assert!((a.y - b.y).abs() < 1e-6);
            assert!((a.z - b.z).abs() < 1e-6);
        }
        assert_eq!(orig_prim.colors, rt_prim.colors, "0.0/1.0 channel extremes must survive the u16 round trip exactly");
    }

    #[test]
    async fn triangle_primitive_flattens_to_a_point_cloud_no_error() {
        let mut semio = sample_semio_mesh();
        semio.meshes[0].primitives[0].topology = SemioTopology::Triangles;
        semio.meshes[0].primitives[0].indices = vec![0, 1, 0];
        let las = semio_framework_plugin::resolve_ready(SemioMeshToLas::serialize(&semio)).expect("triangle -> point-cloud flatten must succeed, not error");
        assert_eq!(las.points.len(), 2, "connectivity dropped, positions kept");
    }
}
//#endregion 🔖️Tests
