//! 🕸 GLTF topology indicators.

#[path = "boundary-loops/🦀️component.rs"]
pub mod boundary_loops;
#[path = "euler-characteristic/🦀️component.rs"]
pub mod euler_characteristic;
#[path = "genus/🦀️component.rs"]
pub mod genus;
#[path = "handles/🦀️component.rs"]
pub mod handles;
#[path = "holes/🦀️component.rs"]
pub mod holes;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::GltfGeometryContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfTopologyIndicators {
    pub holes: GltfMeasure<u64>,
    pub handles: GltfMeasure<u64>,
    pub boundary_loops: GltfMeasure<u64>,
    pub euler_characteristic: GltfMeasure<i64>,
    pub genus: GltfMeasure<u64>,
}

pub struct GltfTopologyInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfTopologyInference {
    type Output = GltfTopologyIndicators;

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { holes: holes::infer(context).await, handles: handles::infer(context).await, boundary_loops: boundary_loops::infer(context).await, euler_characteristic: euler_characteristic::infer(context).await, genus: genus::infer(context).await }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            holes: holes::unavailable_measure(diagnostic_ids).await,
            handles: handles::unavailable_measure(diagnostic_ids).await,
            boundary_loops: boundary_loops::unavailable_measure(diagnostic_ids).await,
            euler_characteristic: euler_characteristic::unavailable_measure(diagnostic_ids).await,
            genus: genus::unavailable_measure(diagnostic_ids).await,
        }
    }
}

#[cfg(test)]
mod canonical_vectors {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Context {
        points: Vec<[f64; 3]>,
        triangles: Vec<[usize; 3]>,
        valid: bool,
    }

    #[derive(Deserialize)]
    struct Vector {
        context: Context,
        value: Option<i64>,
        availability: String,
    }

    #[derive(Deserialize)]
    struct Contract {
        vectors: Vec<Vector>,
    }

    async fn assert_unsigned(source: &str, infer_leaf: for<'a> fn(&GltfGeometryContext<'a>) -> GltfMeasure<u64>, unavailable_leaf: fn(&[String]) -> GltfMeasure<u64>) {
        let contract: Contract = serde_json::from_str(source).unwrap();
        for vector in contract.vectors {
            let result = if vector.context.valid {
                let policy = super::super::geometry_core::policy();
                let context = GltfGeometryContext::new(&vector.context.points, &vector.context.triangles, &policy).unwrap();
                infer_leaf(&context)
            } else {
                unavailable_leaf(&["missing-position".into()])
            };
            assert_eq!(result.value.map(|value| value as i64), vector.value);
            assert_eq!(format!("{:?}", result.availability).to_ascii_lowercase(), vector.availability);
        }
    }

    async fn assert_signed(source: &str, infer_leaf: for<'a> fn(&GltfGeometryContext<'a>) -> GltfMeasure<i64>, unavailable_leaf: fn(&[String]) -> GltfMeasure<i64>) {
        let contract: Contract = serde_json::from_str(source).unwrap();
        for vector in contract.vectors {
            let result = if vector.context.valid {
                let policy = super::super::geometry_core::policy();
                let context = GltfGeometryContext::new(&vector.context.points, &vector.context.triangles, &policy).unwrap();
                infer_leaf(&context)
            } else {
                unavailable_leaf(&["missing-position".into()])
            };
            assert_eq!(result.value, vector.value);
            assert_eq!(format!("{:?}", result.availability).to_ascii_lowercase(), vector.availability);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn every_topology_leaf_executes_its_shared_vectors() {
        assert_unsigned(include_str!("boundary-loops/🧪️contract/🔣️component.json"), boundary_loops::infer, boundary_loops::unavailable_measure);
        assert_signed(include_str!("euler-characteristic/🧪️contract/🔣️component.json"), euler_characteristic::infer, euler_characteristic::unavailable_measure);
        assert_unsigned(include_str!("genus/🧪️contract/🔣️component.json"), genus::infer, genus::unavailable_measure);
        assert_unsigned(include_str!("handles/🧪️contract/🔣️component.json"), handles::infer, handles::unavailable_measure);
        assert_unsigned(include_str!("holes/🧪️contract/🔣️component.json"), holes::infer, holes::unavailable_measure);
    }
}
