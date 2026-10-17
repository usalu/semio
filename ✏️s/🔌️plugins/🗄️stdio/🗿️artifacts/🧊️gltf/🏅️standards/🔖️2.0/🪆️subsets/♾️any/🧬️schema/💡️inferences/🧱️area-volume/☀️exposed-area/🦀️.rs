//! 💡️ exposed-area atomic glTF inference leaf.
use crate::standards::v2_0::subsets::any::schema::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::GltfGeometryContext;

pub struct GltfExposedAreaInference;

impl GltfInferenceLeaf for GltfExposedAreaInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.exposed-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.exposed-area.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfExposedAreaInference::DESCRIPTOR
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn from_assembly(surface_area: &GltfMeasure<f64>, part_count: usize, contact_area: f64, complete: bool, sample_count: usize, topology: Topology) -> GltfMeasure<f64> {
    if part_count <= 1 {
        return surface_area.clone();
    }
    if complete {
        return estimate((surface_area.value.unwrap_or(0.0) - 2.0 * contact_area).max(0.0), GltfUnit::SquareMetre, sample_count, Some(topology));
    }
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), sample_count, Some(topology))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(measure: &GltfMeasure<f64>) -> semio_framework_value::DslValue {
    semio_framework_value::ToValue::to_value(measure)
}

//#region 🧾️InferenceDescriptor
/// 🧾️ Registry descriptor of this inference leaf; its companion documents are embedded from this leaf's own folder.
pub fn inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.gltf.inference.exposed-area.v1",
        inference: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🧾️InferenceDescriptor


