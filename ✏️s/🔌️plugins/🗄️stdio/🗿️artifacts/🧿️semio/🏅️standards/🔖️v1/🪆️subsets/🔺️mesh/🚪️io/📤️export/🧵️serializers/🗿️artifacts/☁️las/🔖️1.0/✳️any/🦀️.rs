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

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_las::schema::snapshot::{LasHeader, LasPoint};
use semio_s_artifact_stdio_las::LasSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };

use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
