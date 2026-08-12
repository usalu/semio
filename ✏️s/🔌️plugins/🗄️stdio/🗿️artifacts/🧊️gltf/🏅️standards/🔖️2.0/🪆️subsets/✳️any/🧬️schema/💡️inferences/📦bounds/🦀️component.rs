//! 📦 `bounds` — one named inference: the spatial bounding box over every mesh primitive's own
//! `POSITION` accessor. glTF 2.0 spec-mandates every `POSITION` accessor to carry its own real
//! per-component `min`/`max` (§5.1, "Implementation note"), so this walk trusts that invariant
//! rather than recomputing an accessor's extent from decoded buffer bytes — a primitive missing a
//! `POSITION` attribute, or whose accessor lacks `min`/`max`, is skipped honestly (never
//! fabricated). `vertexCount` sums every found `POSITION` accessor's own `count` (VEC3 always, so
//! one vertex per component-triple); `meshCount`/`primitiveCount` are direct tallies of
//! `document.meshes`/every mesh's `primitives`. A pure whole-snapshot scalar (one min/max fold) —
//! no `InferredField` needed.

use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Gltf's mesh-primitive-derived spatial bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub vertex_count: u32,
    pub mesh_count: u32,
    pub primitive_count: u32,
}

/// 🩹 Hand-rolled: an empty mesh/primitive set has no honest min/max — `[0,0,0]`/`[0,0,0]` matches
/// what `compute` returns for zero found `POSITION` accessors (the fold's identity value), keeping
/// the inference-default law correct.
impl Default for GltfBounds {
    fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0], vertex_count: 0, mesh_count: 0, primitive_count: 0 }
    }
}

/// 📦️ Computes [`GltfBounds`] over every `document.meshes[].primitives[]`'s own `POSITION`
/// accessor — see module doc comment for the honest-skip rule.
pub fn compute_gltf_bounds(snapshot: &GltfSnapshot) -> GltfBounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;
    let mut vertex_count = 0u32;
    let mesh_count = snapshot.document.meshes.len() as u32;
    let mut primitive_count = 0u32;

    for mesh in &snapshot.document.meshes {
        for primitive in &mesh.primitives {
            primitive_count += 1;

            let Some((_, accessor_index)) = primitive.attributes.iter().find(|(name, _)| name == "POSITION") else {
                continue;
            };
            let Some(accessor) = snapshot.document.accessors.get(*accessor_index) else {
                continue;
            };
            let (Some(accessor_min), Some(accessor_max)) = (&accessor.min, &accessor.max) else {
                continue;
            };
            if accessor_min.len() != 3 || accessor_max.len() != 3 {
                continue;
            }

            for i in 0..3 {
                if seen {
                    min[i] = min[i].min(accessor_min[i]);
                    max[i] = max[i].max(accessor_max[i]);
                } else {
                    min[i] = accessor_min[i];
                    max[i] = accessor_max[i];
                }
            }
            seen = true;
            vertex_count += accessor.count as u32;
        }
    }

    GltfBounds { min, max, vertex_count, mesh_count, primitive_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
    use crate::artifacts::gltf::schema::snapshot::{GltfAccessor, GltfDocument, GltfMesh, GltfPrimitive, GltfSourceForm};
    use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;

    fn accessor(min: [f64; 3], max: [f64; 3], count: usize) -> GltfAccessor {
        GltfAccessor {
            buffer_view: Some(0),
            byte_offset: 0,
            component_type: GltfComponentType::Float,
            normalized: false,
            count,
            kind: GltfAccessorType::Vec3,
            min: Some(min.to_vec()),
            max: Some(max.to_vec()),
            sparse: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }

    #[test]
    fn bounds_matches_hand_built_position_accessor_extent() {
        let mut document = GltfDocument::default();
        document.meshes = vec![
            GltfMesh {
                primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), 0)], ..Default::default() }],
                ..Default::default()
            },
            GltfMesh {
                primitives: vec![
                    GltfPrimitive { attributes: vec![("POSITION".into(), 1)], ..Default::default() },
                    GltfPrimitive { attributes: vec![("NORMAL".into(), 2)], ..Default::default() },
                ],
                ..Default::default()
            },
        ];
        document.accessors = vec![
            accessor([-1.0, 0.0, -2.0], [1.0, 2.0, 2.0], 24),
            accessor([0.0, -5.0, 0.0], [3.0, -1.0, 4.0], 8),
            accessor([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 24),
        ];
        let snapshot = GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document,
            buffers: Vec::new(),
            source_form: GltfSourceForm::Json,
        };

        let bounds = compute_gltf_bounds(&snapshot);
        assert_eq!(bounds.min, [-1.0, -5.0, -2.0]);
        assert_eq!(bounds.max, [3.0, 2.0, 4.0]);
        assert_eq!(bounds.vertex_count, 32);
        assert_eq!(bounds.mesh_count, 2);
        assert_eq!(bounds.primitive_count, 3);
    }

    #[test]
    fn inference_determinism_law() {
        let mut document = GltfDocument::default();
        document.meshes = vec![GltfMesh {
            primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), 0)], ..Default::default() }],
            ..Default::default()
        }];
        document.accessors = vec![accessor([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], 3)];
        let snapshot = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: Vec::new(), source_form: GltfSourceForm::Json };
        assert_eq!(compute_gltf_bounds(&snapshot), compute_gltf_bounds(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_gltf_bounds(&GltfSnapshot::default()), GltfBounds::default());
    }
}
//#endregion 🧪️Tests
