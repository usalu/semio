//! 💡️ reentrant-volume atomic glTF inference leaf.
use crate::standards::v2_0::subsets::any::schema::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::GltfGeometryContext;

pub struct GltfReentrantVolumeInference;

impl GltfInferenceLeaf for GltfReentrantVolumeInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.reentrant-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reentrant-volume.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfReentrantVolumeInference::DESCRIPTOR
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn from_raw(context: &GltfGeometryContext<'_>, raw: &crate::standards::v2_0::subsets::any::schema::inferences::concavity::GltfConcavityRaw) -> GltfMeasure<f64> {
    raw.hull_volume.filter(|volume| *volume > 0.0).map_or_else(
        || unavailable(GltfUnit::CubicMetre, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology)),
        |volume| {
            if context.solid.is_some() {
                estimate((volume - context.volume).max(0.0), GltfUnit::CubicMetre, context.sample_count, Some(context.topology))
            } else {
                unavailable(GltfUnit::CubicMetre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))
            }
        },
    )
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
        id: "s.stdio.gltf.inference.reentrant-volume.v1",
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


