//! 📥️ Deserialize `s.stdio.semio/v1/mesh` from `s.stdio.las/1.0/*` — LAS is a PURE point cloud
//! format: it has no face/edge connectivity concept at all. Maps as ONE `SemioMesh` with ONE
//! `Points`-topology `SemioPrimitive`, `indices` empty (point clouds have nothing to index into —
//! same "no connectivity" honesty as the sibling PLY leaf's no-`face`-element case, just without
//! the possibility of ever having one). `LasPoint.x/y/z` are already real-world-scaled coordinates
//! (`engine::decode_las`'s own `decode_point` applies `header.{x,y,z}_scale/offset` before this
//! leaf ever sees them), so positions map 1:1, no additional scaling here.
//!
//! 🔖 Documented lossiness (real, deliberate — LAS's per-point richness has no `SemioMesh`
//! counterpart beyond position + color):
//! - `intensity`, `return_number`/`number_of_returns`, `scan_direction_flag`,
//!   `edge_of_flight_line`, `classification`, `scan_angle_rank`, `user_data`, `point_source_id`,
//!   `gps_time` — every LAS per-point attribute beyond `x`/`y`/`z`/`rgb` — is dropped.
//! - `LasHeader` (system identifier, generating software, creation date, bounding box,
//!   points-by-return histogram, scale/offset) is dropped entirely — `SemioMeshSnapshot` has no
//!   file-metadata slot.
//! - `vlrs` (Variable Length Records, third-party-proprietary payloads) are dropped.
//! - `rgb`, when present on EVERY point, maps to `colors` (`u16` `0..=65535` -> `[0,1]`, alpha
//!   `1.0`); if only SOME points carry `rgb` (a real but unusual LAS shape — point format is
//!   normally uniform across a whole file), `colors` stays empty rather than a mix of real and
//!   fabricated values.
//! - No normals/uvs/materials/textures — LAS has none of these concepts.

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_las::LasSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

pub struct SemioMeshFromLas;

impl ArtifactDeserializer for SemioMeshFromLas {
    type From = LasSnapshot;
    type Into = SemioMeshSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let positions: Vec<SemioPoint3> = from.points.iter().map(|p| SemioPoint3 { x: p.x, y: p.y, z: p.z }).collect();

        let colors: Vec<SemioRgba> = if !from.points.is_empty() && from.points.iter().all(|p| p.rgb.is_some()) {
            from.points
                .iter()
                .map(|p| {
                    let (r, g, b) = p.rgb.expect("checked all-Some above");
                    SemioRgba { r: r as f32 / 65535.0, g: g as f32 / 65535.0, b: b as f32 / 65535.0, a: 1.0 }
                })
                .collect()
        } else {
            Vec::new()
        };

        let primitive = SemioPrimitive { id: "mesh-0-prim-0".to_string(), topology: SemioTopology::Points, positions, normals: Vec::new(), uvs: Vec::new(), colors, indices: Vec::new(), material_id: None };
        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: vec![SemioMesh { id: "mesh-0".to_string(), primitives: vec![primitive] }], materials: Vec::new(), textures: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
