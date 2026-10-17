//! 💡️ overlap-volume atomic glTF inference leaf.
use crate::standards::v2_0::subsets::any::schema::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::{GltfGeometryContext, GltfPairGeometry};

pub struct GltfOverlapVolumeInference;

impl GltfInferenceLeaf for GltfOverlapVolumeInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.overlap-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overlap-volume.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfOverlapVolumeInference::DESCRIPTOR
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
    pair.overlap.map_or_else(|| unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), pair.sample_count, None), |(volume, samples)| estimate(volume, GltfUnit::CubicMetre, samples, None))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn from_assembly(volume: f64, complete: bool, pair_count: usize, sample_count: usize, topology: Topology) -> Option<GltfMeasure<f64>> {
    (pair_count > 0 && complete).then(|| estimate(volume, GltfUnit::CubicMetre, sample_count, Some(topology)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(measure: &GltfMeasure<f64>) -> semio_framework_value::DslValue {
    semio_framework_value::ToValue::to_value(measure)
}

//#region 🧾️InferenceDescriptor
/// 🧾️ Registry descriptor of this inference leaf; its companion documents are embedded from this leaf's own folder.
pub fn inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.gltf.inference.overlap-volume.v1",
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


