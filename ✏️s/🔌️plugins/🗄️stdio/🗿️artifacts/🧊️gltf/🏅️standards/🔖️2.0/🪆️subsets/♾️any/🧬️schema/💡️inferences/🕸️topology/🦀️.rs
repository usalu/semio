//! 🕸 GLTF topology indicators.

#[path = "➰️boundary-loops/🦀️.rs"]
pub mod boundary_loops;
#[path = "🧮️euler-characteristic/🦀️.rs"]
pub mod euler_characteristic;
#[path = "🔢️genus/🦀️.rs"]
pub mod genus;
#[path = "🥯️handles/🦀️.rs"]
pub mod handles;
#[path = "🕳️holes/🦀️.rs"]
pub mod holes;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::GltfGeometryContext;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { holes: holes::infer(context), handles: handles::infer(context), boundary_loops: boundary_loops::infer(context), euler_characteristic: euler_characteristic::infer(context), genus: genus::infer(context) }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            holes: holes::unavailable_measure(diagnostic_ids),
            handles: handles::unavailable_measure(diagnostic_ids),
            boundary_loops: boundary_loops::unavailable_measure(diagnostic_ids),
            euler_characteristic: euler_characteristic::unavailable_measure(diagnostic_ids),
            genus: genus::unavailable_measure(diagnostic_ids),
        }
    }
}

#[cfg(test)]
mod canonical_vectors {
    use super::*;

    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Context {
        points: Vec<[f64; 3]>,
        triangles: Vec<[usize; 3]>,
        valid: bool,
    }

    #[derive(value_derive::FromValue)]
    struct Vector {
        context: Context,
        value: Option<i64>,
        availability: String,
    }

    #[derive(value_derive::FromValue)]
    struct Contract {
        vectors: Vec<Vector>,
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_unsigned(source: &str, infer_leaf: for<'a> fn(&GltfGeometryContext<'a>) -> GltfMeasure<u64>, unavailable_leaf: fn(&[String]) -> GltfMeasure<u64>) {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_signed(source: &str, infer_leaf: for<'a> fn(&GltfGeometryContext<'a>) -> GltfMeasure<i64>, unavailable_leaf: fn(&[String]) -> GltfMeasure<i64>) {
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

    #[test]
    fn every_topology_leaf_executes_its_shared_vectors() {
        assert_unsigned(include_str!("➰️boundary-loops/🧪️contract/🔣️.json"), boundary_loops::infer, boundary_loops::unavailable_measure);
        assert_signed(include_str!("🧮️euler-characteristic/🧪️contract/🔣️.json"), euler_characteristic::infer, euler_characteristic::unavailable_measure);
        assert_unsigned(include_str!("🔢️genus/🧪️contract/🔣️.json"), genus::infer, genus::unavailable_measure);
        assert_unsigned(include_str!("🥯️handles/🧪️contract/🔣️.json"), handles::infer, handles::unavailable_measure);
        assert_unsigned(include_str!("🕳️holes/🧪️contract/🔣️.json"), holes::infer, holes::unavailable_measure);
    }
}
