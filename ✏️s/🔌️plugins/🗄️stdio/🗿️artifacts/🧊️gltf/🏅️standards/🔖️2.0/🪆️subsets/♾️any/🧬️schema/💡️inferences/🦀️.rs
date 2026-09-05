//! 💡️ Public glTF inference assembly.

use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::super::modules::measurement_contracts::*;
use super::compactness;
use super::{adjacency::*, area_volume::*, clearance::*, compactness::*, concavity::*, curvature::*, mass_distribution::*, orientation::*, proportion::*, roughness::*, size::*, symmetry::*, thickness::*, topology::*};

//#region 🔖️PublicRecords
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfEntityIndicators {
    pub size: GltfSizeIndicators,
    pub area_volume: GltfAreaVolumeIndicators,
    pub compactness: GltfCompactnessIndicators,
    pub proportion: GltfProportionIndicators,
    pub mass: GltfMassIndicators,
    pub curvature: GltfCurvatureIndicators,
    pub thickness: GltfThicknessIndicators,
    pub concavity: GltfConcavityIndicators,
    pub clearance: GltfClearanceIndicators,
    pub adjacency: GltfAdjacencyIndicators,
    pub orientation: GltfOrientationIndicators,
    pub symmetry: GltfSymmetryIndicators,
    pub roughness: GltfRoughnessIndicators,
    pub topology: GltfTopologyIndicators,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfPartInference {
    pub address: GltfEntityAddress,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub indicators: GltfEntityIndicators,
    pub diagnostic_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfPairInference {
    pub first: GltfEntityAddress,
    pub second: GltfEntityAddress,
    pub minimum_distance: GltfMeasure<f64>,
    pub clearance_distribution: GltfMeasure<GltfStatistics>,
    pub contact_area: GltfMeasure<f64>,
    pub interference_volume: GltfMeasure<f64>,
    pub overlap_volume: GltfMeasure<f64>,
    pub adjacent: GltfMeasure<bool>,
    pub orientation_consistency: GltfMeasure<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfInferenceCounts {
    pub scene_count: u64,
    pub node_instance_count: u64,
    pub mesh_count: u64,
    pub primitive_count: u64,
    pub vertex_count: u64,
    pub triangle_count: u64,
    pub component_count: u64,
    pub surface_region_count: u64,
    pub pair_count: u64,
    pub valid_part_count: u64,
    pub invalid_part_count: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfGeometricInference {
    pub schema: String,
    pub schema_version: u32,
    pub policy: GltfAnalysisPolicy,
    pub counts: GltfInferenceCounts,
    pub overall: GltfEntityIndicators,
    pub parts: Vec<GltfPartInference>,
    pub pairs: Vec<GltfPairInference>,
    pub diagnostics: Vec<GltfDiagnostic>,
    pub validity: GltfValidity,
    pub quality: GltfQuality,
    pub provenance: GltfProvenance,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf.inference")]
pub struct GltfInference {
    #[derived]
    pub geometry: GltfGeometricInference,
}
//#endregion 🔖️PublicRecords

//#region 🧭️LeafDag
pub const GLTF_GEOMETRY_READS: &[&str] = &["document/scene", "document/scenes", "document/nodes", "document/meshes", "document/accessors", "document/bufferViews", "document/buffers", "buffers"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GltfInferenceLeafDescriptor {
    pub id: &'static str,
    pub algorithm_version: u32,
    pub cache_key: &'static str,
    pub reads: &'static [&'static str],
}

pub trait GltfInferenceLeaf {
    const DESCRIPTOR: GltfInferenceLeafDescriptor;
}

#[derive(Clone, Copy)]
pub struct GltfInferenceLeafServiceDescriptor {
    pub id: &'static str,
    pub algorithm_version: u32,
    pub cache_key: &'static str,
    /// 🔤️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: every
    /// leaf's `encode_result` now returns [`dsl::DslValue`] directly via [`dsl::ToValue`] --
    /// infallible (unlike the prior `serde_json::from_str(&pack::to_json_string(...))` round trip
    /// through text, which only ever failed if `pack::to_json_string`'s own output couldn't
    /// re-parse, never a real runtime condition), so there is no `Result`/`Error` to carry.
    pub encode: fn(&GltfEntityIndicators) -> dsl::DslValue,
}

pub const GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS: &[GltfInferenceLeafServiceDescriptor] = &[
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.overall-size.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overall-size.v1:geometry-v2", encode: overall_size::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.axis-aligned-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2", encode: axis_aligned_bounds::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.oriented-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.oriented-bounds.v1:geometry-v2", encode: oriented_bounds::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.bounding-box-dimensions.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.bounding-box-dimensions.v1:geometry-v2", encode: bounding_box_dimensions::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.characteristic-length.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.characteristic-length.v1:geometry-v2", encode: characteristic_length::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.footprint-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.footprint-area.v1:geometry-v2", encode: footprint_area::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.projected-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.projected-area.v1:geometry-v2", encode: projected_area::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.surface-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.surface-area.v1:geometry-v2", encode: surface_area::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.total-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.total-area.v1:geometry-v2", encode: total_area::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.exposed-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.exposed-area.v1:geometry-v2", encode: exposed_area::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.contact-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.contact-area.v1:geometry-v2", encode: contact_area::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.volume.v1:geometry-v2", encode: volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.enclosed-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.enclosed-volume.v1:geometry-v2", encode: enclosed_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.material-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.material-volume.v1:geometry-v2", encode: material_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.void-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.void-volume.v1:geometry-v2", encode: void_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.compactness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.compactness.v1:geometry-v2", encode: compactness::compactness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.surface-to-volume-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.surface-to-volume-ratio.v1:geometry-v2", encode: surface_to_volume_ratio::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.sphericity.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.sphericity.v1:geometry-v2", encode: sphericity::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.compactness-index.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.compactness-index.v1:geometry-v2", encode: compactness_index::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.hull-fill-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.hull-fill-ratio.v1:geometry-v2", encode: hull_fill_ratio::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.aspect-ratios.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.aspect-ratios.v1:geometry-v2", encode: aspect_ratios::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.slenderness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.slenderness.v1:geometry-v2", encode: slenderness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.flatness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.flatness.v1:geometry-v2", encode: flatness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.elongation.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.elongation.v1:geometry-v2", encode: elongation::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.centroid.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.centroid.v1:geometry-v2", encode: centroid::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.principal-frame.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.principal-frame.v1:geometry-v2", encode: principal_frame::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.principal-axes.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.principal-axes.v1:geometry-v2", encode: principal_axes::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.moments-of-inertia.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.moments-of-inertia.v1:geometry-v2", encode: moments_of_inertia::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.inertia-tensor.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.inertia-tensor.v1:geometry-v2", encode: inertia_tensor::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.mean-curvature.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.mean-curvature.v1:geometry-v2", encode: mean_curvature::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.gaussian-curvature.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.gaussian-curvature.v1:geometry-v2", encode: gaussian_curvature::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.curvature-histogram.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.curvature-histogram.v1:geometry-v2", encode: curvature_histogram::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.sharp-feature-proportion.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.sharp-feature-proportion.v1:geometry-v2", encode: sharp_feature_proportion::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.mean-thickness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.mean-thickness.v1:geometry-v2", encode: mean_thickness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.minimum-thickness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.minimum-thickness.v1:geometry-v2", encode: minimum_thickness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.thickness-variability.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-variability.v1:geometry-v2", encode: thickness_variability::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.thickness-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-distribution.v1:geometry-v2", encode: thickness_distribution::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.convex-hull-gap.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.convex-hull-gap.v1:geometry-v2", encode: convex_hull_gap::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.reentrant-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reentrant-area.v1:geometry-v2", encode: reentrant_area::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.reentrant-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reentrant-volume.v1:geometry-v2", encode: reentrant_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.concavity-index.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.concavity-index.v1:geometry-v2", encode: concavity_index::encode_result },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1:geometry-v2",
        encode: minimum_distance_to_neighbors::encode_result,
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.clearance-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.clearance-distribution.v1:geometry-v2", encode: clearance_distribution::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.interference-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.interference-volume.v1:geometry-v2", encode: interference_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.overlap-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overlap-volume.v1:geometry-v2", encode: overlap_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.number-of-contacts.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.number-of-contacts.v1:geometry-v2", encode: number_of_contacts::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.contact-graph-degree.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.contact-graph-degree.v1:geometry-v2", encode: contact_graph_degree::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.connected-components.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.connected-components.v1:geometry-v2", encode: connected_components::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.main-axis-direction.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.main-axis-direction.v1:geometry-v2", encode: main_axis_direction::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.face-normal-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.face-normal-distribution.v1:geometry-v2", encode: face_normal_distribution::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.orientation-consistency.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.orientation-consistency.v1:geometry-v2", encode: orientation_consistency::encode_result },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.reflection-symmetry-score.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.reflection-symmetry-score.v1:geometry-v2",
        encode: reflection_symmetry_score::encode_result,
    },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.rotational-symmetry-score.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.rotational-symmetry-score.v1:geometry-v2",
        encode: rotational_symmetry_score::encode_result,
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.reflection-symmetries.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reflection-symmetries.v1:geometry-v2", encode: reflection_symmetries::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.rotational-symmetries.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.rotational-symmetries.v1:geometry-v2", encode: rotational_symmetries::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.repetition-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.repetition-ratio.v1:geometry-v2", encode: repetition_ratio::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.modularity-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.modularity-ratio.v1:geometry-v2", encode: modularity_ratio::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.deviation-from-ideal.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.deviation-from-ideal.v1:geometry-v2", encode: deviation_from_ideal::encode_result },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1:geometry-v2",
        encode: deviation_from_smoothed_geometry::encode_result,
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.normal-variation.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.normal-variation.v1:geometry-v2", encode: normal_variation::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.surface-waviness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.surface-waviness.v1:geometry-v2", encode: surface_waviness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.irregularity.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.irregularity.v1:geometry-v2", encode: irregularity::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.holes.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.holes.v1:geometry-v2", encode: holes::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.handles.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.handles.v1:geometry-v2", encode: handles::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.boundary-loops.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.boundary-loops.v1:geometry-v2", encode: boundary_loops::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.euler-characteristic.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.euler-characteristic.v1:geometry-v2", encode: euler_characteristic::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.genus.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.genus.v1:geometry-v2", encode: genus::encode_result },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_inference_leaf_service_descriptor(id: &str) -> Option<&'static GltfInferenceLeafServiceDescriptor> {
    GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS.iter().find(|descriptor| descriptor.id == id)
}

pub const GLTF_INFERENCE_FIELDS: &[protocol::InferenceFieldSpec] = &[
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.overall-size.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.axis-aligned-bounds.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.oriented-bounds.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.bounding-box-dimensions.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.characteristic-length.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.footprint-area.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.projected-area.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.surface-area.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.total-area.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.exposed-area.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.contact-area.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.volume.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.enclosed-volume.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.material-volume.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.void-volume.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.compactness.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.surface-to-volume-ratio.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.sphericity.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.compactness-index.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.hull-fill-ratio.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.aspect-ratios.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.slenderness.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.flatness.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.elongation.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.centroid.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.principal-frame.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.principal-axes.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.moments-of-inertia.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.inertia-tensor.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.mean-curvature.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.gaussian-curvature.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.curvature-histogram.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.sharp-feature-proportion.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.mean-thickness.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.minimum-thickness.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.thickness-variability.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.thickness-distribution.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.convex-hull-gap.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.reentrant-area.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.reentrant-volume.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.concavity-index.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.clearance-distribution.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.interference-volume.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.overlap-volume.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.number-of-contacts.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.contact-graph-degree.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.connected-components.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.main-axis-direction.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.face-normal-distribution.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.orientation-consistency.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.reflection-symmetry-score.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.rotational-symmetry-score.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.reflection-symmetries.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.rotational-symmetries.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.repetition-ratio.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.modularity-ratio.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.deviation-from-ideal.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.normal-variation.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.surface-waviness.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.irregularity.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.holes.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.handles.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.boundary-loops.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.euler-characteristic.v1", reads: GLTF_GEOMETRY_READS },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.genus.v1", reads: GLTF_GEOMETRY_READS },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn invalidated_gltf_inference_fields(touched: Option<&protocol::TouchedPaths>) -> Vec<&'static str> {
    GLTF_INFERENCE_FIELDS.iter().filter(|field| touched.is_none_or(|paths| paths.intersects_any(field.reads))).map(|field| field.id).collect()
}
//#endregion 🧭️LeafDag

//#region 🧩️Assembly
pub use super::dag_assembly::compute_gltf_inference;
//#endregion 🧩️Assembly

//#region 🧠️InferenceContract
impl protocol::Inference<GltfSnapshot> for GltfInference {
    fn infer(snapshot: &GltfSnapshot) -> Self {
        Self { geometry: compute_gltf_inference(snapshot) }
    }
}

impl Default for GltfInference {
    fn default() -> Self {
        <Self as protocol::Inference<GltfSnapshot>>::infer(&GltfSnapshot::default())
    }
}

impl protocol::InferenceSpec<GltfSnapshot> for GltfInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.gltf.inference"
    }
    fn schema_version() -> u32 {
        2
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        GLTF_INFERENCE_FIELDS
    }
}

impl ArtifactInferrer for crate::artifacts::gltf::standards::v2_0::subsets::any::schema::GltfBuilder {
    type Snapshot = GltfSnapshot;
    type Inference = GltfInference;
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_artifact_inference_descriptors() -> Vec<schema::ArtifactInferenceDescriptor> {
    vec![
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.overall-size.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/📏️overall-size/🦀️.rs"),
                typescript: include_str!("📦️size/📏️overall-size/🟦️.ts"),
                graphql: include_str!("📦️size/📏️overall-size/🔗️.graphql"),
                json_schema: include_str!("📦️size/📏️overall-size/🔣️.json"),
                proto: include_str!("📦️size/📏️overall-size/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.axis-aligned-bounds.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/🌳️axis-aligned-bounds/🦀️.rs"),
                typescript: include_str!("📦️size/🌳️axis-aligned-bounds/🟦️.ts"),
                graphql: include_str!("📦️size/🌳️axis-aligned-bounds/🔗️.graphql"),
                json_schema: include_str!("📦️size/🌳️axis-aligned-bounds/🔣️.json"),
                proto: include_str!("📦️size/🌳️axis-aligned-bounds/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.oriented-bounds.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/🌹️oriented-bounds/🦀️.rs"),
                typescript: include_str!("📦️size/🌹️oriented-bounds/🟦️.ts"),
                graphql: include_str!("📦️size/🌹️oriented-bounds/🔗️.graphql"),
                json_schema: include_str!("📦️size/🌹️oriented-bounds/🔣️.json"),
                proto: include_str!("📦️size/🌹️oriented-bounds/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.bounding-box-dimensions.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/📐️bounding-box-dimensions/🦀️.rs"),
                typescript: include_str!("📦️size/📐️bounding-box-dimensions/🟦️.ts"),
                graphql: include_str!("📦️size/📐️bounding-box-dimensions/🔗️.graphql"),
                json_schema: include_str!("📦️size/📐️bounding-box-dimensions/🔣️.json"),
                proto: include_str!("📦️size/📐️bounding-box-dimensions/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.characteristic-length.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/🐯️characteristic-length/🦀️.rs"),
                typescript: include_str!("📦️size/🐯️characteristic-length/🟦️.ts"),
                graphql: include_str!("📦️size/🐯️characteristic-length/🔗️.graphql"),
                json_schema: include_str!("📦️size/🐯️characteristic-length/🔣️.json"),
                proto: include_str!("📦️size/🐯️characteristic-length/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.footprint-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/🖨️footprint-area/🦀️.rs"),
                typescript: include_str!("📦️size/🖨️footprint-area/🟦️.ts"),
                graphql: include_str!("📦️size/🖨️footprint-area/🔗️.graphql"),
                json_schema: include_str!("📦️size/🖨️footprint-area/🔣️.json"),
                proto: include_str!("📦️size/🖨️footprint-area/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.projected-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/🎯️projected-area/🦀️.rs"),
                typescript: include_str!("📦️size/🎯️projected-area/🟦️.ts"),
                graphql: include_str!("📦️size/🎯️projected-area/🔗️.graphql"),
                json_schema: include_str!("📦️size/🎯️projected-area/🔣️.json"),
                proto: include_str!("📦️size/🎯️projected-area/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.surface-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/🖱️surface-area/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/🖱️surface-area/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/🖱️surface-area/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/🖱️surface-area/🔣️.json"),
                proto: include_str!("🧱️area-volume/🖱️surface-area/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.total-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/🐨️total-area/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/🐨️total-area/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/🐨️total-area/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/🐨️total-area/🔣️.json"),
                proto: include_str!("🧱️area-volume/🐨️total-area/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.exposed-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/🍐️exposed-area/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/🍐️exposed-area/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/🍐️exposed-area/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/🍐️exposed-area/🔣️.json"),
                proto: include_str!("🧱️area-volume/🍐️exposed-area/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.contact-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/🦅️contact-area/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/🦅️contact-area/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/🦅️contact-area/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/🦅️contact-area/🔣️.json"),
                proto: include_str!("🧱️area-volume/🦅️contact-area/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/🟫️volume/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/🟫️volume/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/🟫️volume/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/🟫️volume/🔣️.json"),
                proto: include_str!("🧱️area-volume/🟫️volume/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.enclosed-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/🐯️enclosed-volume/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/🐯️enclosed-volume/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/🐯️enclosed-volume/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/🐯️enclosed-volume/🔣️.json"),
                proto: include_str!("🧱️area-volume/🐯️enclosed-volume/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.material-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/🧊️material-volume/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/🧊️material-volume/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/🧊️material-volume/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/🧊️material-volume/🔣️.json"),
                proto: include_str!("🧱️area-volume/🧊️material-volume/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.void-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/⚪️void-volume/🦀️.rs"),
                typescript: include_str!("🧱️area-volume/⚪️void-volume/🟦️.ts"),
                graphql: include_str!("🧱️area-volume/⚪️void-volume/🔗️.graphql"),
                json_schema: include_str!("🧱️area-volume/⚪️void-volume/🔣️.json"),
                proto: include_str!("🧱️area-volume/⚪️void-volume/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.compactness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/⚓️compactness/🦀️.rs"),
                typescript: include_str!("⚪️compactness/⚓️compactness/🟦️.ts"),
                graphql: include_str!("⚪️compactness/⚓️compactness/🔗️.graphql"),
                json_schema: include_str!("⚪️compactness/⚓️compactness/🔣️.json"),
                proto: include_str!("⚪️compactness/⚓️compactness/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.surface-to-volume-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/🖱️surface-to-volume-ratio/🦀️.rs"),
                typescript: include_str!("⚪️compactness/🖱️surface-to-volume-ratio/🟦️.ts"),
                graphql: include_str!("⚪️compactness/🖱️surface-to-volume-ratio/🔗️.graphql"),
                json_schema: include_str!("⚪️compactness/🖱️surface-to-volume-ratio/🔣️.json"),
                proto: include_str!("⚪️compactness/🖱️surface-to-volume-ratio/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.sphericity.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/🪁️sphericity/🦀️.rs"),
                typescript: include_str!("⚪️compactness/🪁️sphericity/🟦️.ts"),
                graphql: include_str!("⚪️compactness/🪁️sphericity/🔗️.graphql"),
                json_schema: include_str!("⚪️compactness/🪁️sphericity/🔣️.json"),
                proto: include_str!("⚪️compactness/🪁️sphericity/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.compactness-index.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/🚪️compactness-index/🦀️.rs"),
                typescript: include_str!("⚪️compactness/🚪️compactness-index/🟦️.ts"),
                graphql: include_str!("⚪️compactness/🚪️compactness-index/🔗️.graphql"),
                json_schema: include_str!("⚪️compactness/🚪️compactness-index/🔣️.json"),
                proto: include_str!("⚪️compactness/🚪️compactness-index/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.hull-fill-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/🟤️hull-fill-ratio/🦀️.rs"),
                typescript: include_str!("⚪️compactness/🟤️hull-fill-ratio/🟦️.ts"),
                graphql: include_str!("⚪️compactness/🟤️hull-fill-ratio/🔗️.graphql"),
                json_schema: include_str!("⚪️compactness/🟤️hull-fill-ratio/🔣️.json"),
                proto: include_str!("⚪️compactness/🟤️hull-fill-ratio/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.aspect-ratios.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/🧪️aspect-ratios/🦀️.rs"),
                typescript: include_str!("📏️proportion/🧪️aspect-ratios/🟦️.ts"),
                graphql: include_str!("📏️proportion/🧪️aspect-ratios/🔗️.graphql"),
                json_schema: include_str!("📏️proportion/🧪️aspect-ratios/🔣️.json"),
                proto: include_str!("📏️proportion/🧪️aspect-ratios/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.slenderness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/🌾️slenderness/🦀️.rs"),
                typescript: include_str!("📏️proportion/🌾️slenderness/🟦️.ts"),
                graphql: include_str!("📏️proportion/🌾️slenderness/🔗️.graphql"),
                json_schema: include_str!("📏️proportion/🌾️slenderness/🔣️.json"),
                proto: include_str!("📏️proportion/🌾️slenderness/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.flatness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/🔮️flatness/🦀️.rs"),
                typescript: include_str!("📏️proportion/🔮️flatness/🟦️.ts"),
                graphql: include_str!("📏️proportion/🔮️flatness/🔗️.graphql"),
                json_schema: include_str!("📏️proportion/🔮️flatness/🔣️.json"),
                proto: include_str!("📏️proportion/🔮️flatness/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.elongation.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/🟫️elongation/🦀️.rs"),
                typescript: include_str!("📏️proportion/🟫️elongation/🟦️.ts"),
                graphql: include_str!("📏️proportion/🟫️elongation/🔗️.graphql"),
                json_schema: include_str!("📏️proportion/🟫️elongation/🔣️.json"),
                proto: include_str!("📏️proportion/🟫️elongation/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.centroid.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/🐞️centroid/🦀️.rs"),
                typescript: include_str!("⚖️mass-distribution/🐞️centroid/🟦️.ts"),
                graphql: include_str!("⚖️mass-distribution/🐞️centroid/🔗️.graphql"),
                json_schema: include_str!("⚖️mass-distribution/🐞️centroid/🔣️.json"),
                proto: include_str!("⚖️mass-distribution/🐞️centroid/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.principal-frame.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/🟥️principal-frame/🦀️.rs"),
                typescript: include_str!("⚖️mass-distribution/🟥️principal-frame/🟦️.ts"),
                graphql: include_str!("⚖️mass-distribution/🟥️principal-frame/🔗️.graphql"),
                json_schema: include_str!("⚖️mass-distribution/🟥️principal-frame/🔣️.json"),
                proto: include_str!("⚖️mass-distribution/🟥️principal-frame/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.principal-axes.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/🧭️principal-axes/🦀️.rs"),
                typescript: include_str!("⚖️mass-distribution/🧭️principal-axes/🟦️.ts"),
                graphql: include_str!("⚖️mass-distribution/🧭️principal-axes/🔗️.graphql"),
                json_schema: include_str!("⚖️mass-distribution/🧭️principal-axes/🔣️.json"),
                proto: include_str!("⚖️mass-distribution/🧭️principal-axes/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.moments-of-inertia.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/🍐️moments-of-inertia/🦀️.rs"),
                typescript: include_str!("⚖️mass-distribution/🍐️moments-of-inertia/🟦️.ts"),
                graphql: include_str!("⚖️mass-distribution/🍐️moments-of-inertia/🔗️.graphql"),
                json_schema: include_str!("⚖️mass-distribution/🍐️moments-of-inertia/🔣️.json"),
                proto: include_str!("⚖️mass-distribution/🍐️moments-of-inertia/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.inertia-tensor.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/🐧️inertia-tensor/🦀️.rs"),
                typescript: include_str!("⚖️mass-distribution/🐧️inertia-tensor/🟦️.ts"),
                graphql: include_str!("⚖️mass-distribution/🐧️inertia-tensor/🔗️.graphql"),
                json_schema: include_str!("⚖️mass-distribution/🐧️inertia-tensor/🔣️.json"),
                proto: include_str!("⚖️mass-distribution/🐧️inertia-tensor/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.mean-curvature.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/🐝️mean-curvature/🦀️.rs"),
                typescript: include_str!("🌀️curvature/🐝️mean-curvature/🟦️.ts"),
                graphql: include_str!("🌀️curvature/🐝️mean-curvature/🔗️.graphql"),
                json_schema: include_str!("🌀️curvature/🐝️mean-curvature/🔣️.json"),
                proto: include_str!("🌀️curvature/🐝️mean-curvature/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.gaussian-curvature.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/🟪️gaussian-curvature/🦀️.rs"),
                typescript: include_str!("🌀️curvature/🟪️gaussian-curvature/🟦️.ts"),
                graphql: include_str!("🌀️curvature/🟪️gaussian-curvature/🔗️.graphql"),
                json_schema: include_str!("🌀️curvature/🟪️gaussian-curvature/🔣️.json"),
                proto: include_str!("🌀️curvature/🟪️gaussian-curvature/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.curvature-histogram.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/🟥️curvature-histogram/🦀️.rs"),
                typescript: include_str!("🌀️curvature/🟥️curvature-histogram/🟦️.ts"),
                graphql: include_str!("🌀️curvature/🟥️curvature-histogram/🔗️.graphql"),
                json_schema: include_str!("🌀️curvature/🟥️curvature-histogram/🔣️.json"),
                proto: include_str!("🌀️curvature/🟥️curvature-histogram/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.sharp-feature-proportion.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/🐨️sharp-feature-proportion/🦀️.rs"),
                typescript: include_str!("🌀️curvature/🐨️sharp-feature-proportion/🟦️.ts"),
                graphql: include_str!("🌀️curvature/🐨️sharp-feature-proportion/🔗️.graphql"),
                json_schema: include_str!("🌀️curvature/🐨️sharp-feature-proportion/🔣️.json"),
                proto: include_str!("🌀️curvature/🐨️sharp-feature-proportion/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.mean-thickness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/🦋️mean-thickness/🦀️.rs"),
                typescript: include_str!("↕️thickness/🦋️mean-thickness/🟦️.ts"),
                graphql: include_str!("↕️thickness/🦋️mean-thickness/🔗️.graphql"),
                json_schema: include_str!("↕️thickness/🦋️mean-thickness/🔣️.json"),
                proto: include_str!("↕️thickness/🦋️mean-thickness/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.minimum-thickness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/⚪️minimum-thickness/🦀️.rs"),
                typescript: include_str!("↕️thickness/⚪️minimum-thickness/🟦️.ts"),
                graphql: include_str!("↕️thickness/⚪️minimum-thickness/🔗️.graphql"),
                json_schema: include_str!("↕️thickness/⚪️minimum-thickness/🔣️.json"),
                proto: include_str!("↕️thickness/⚪️minimum-thickness/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.thickness-variability.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/🟤️thickness-variability/🦀️.rs"),
                typescript: include_str!("↕️thickness/🟤️thickness-variability/🟦️.ts"),
                graphql: include_str!("↕️thickness/🟤️thickness-variability/🔗️.graphql"),
                json_schema: include_str!("↕️thickness/🟤️thickness-variability/🔣️.json"),
                proto: include_str!("↕️thickness/🟤️thickness-variability/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.thickness-distribution.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/🛰️thickness-distribution/🦀️.rs"),
                typescript: include_str!("↕️thickness/🛰️thickness-distribution/🟦️.ts"),
                graphql: include_str!("↕️thickness/🛰️thickness-distribution/🔗️.graphql"),
                json_schema: include_str!("↕️thickness/🛰️thickness-distribution/🔣️.json"),
                proto: include_str!("↕️thickness/🛰️thickness-distribution/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.convex-hull-gap.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/🟨️convex-hull-gap/🦀️.rs"),
                typescript: include_str!("🕳️concavity/🟨️convex-hull-gap/🟦️.ts"),
                graphql: include_str!("🕳️concavity/🟨️convex-hull-gap/🔗️.graphql"),
                json_schema: include_str!("🕳️concavity/🟨️convex-hull-gap/🔣️.json"),
                proto: include_str!("🕳️concavity/🟨️convex-hull-gap/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reentrant-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/⚪️reentrant-area/🦀️.rs"),
                typescript: include_str!("🕳️concavity/⚪️reentrant-area/🟦️.ts"),
                graphql: include_str!("🕳️concavity/⚪️reentrant-area/🔗️.graphql"),
                json_schema: include_str!("🕳️concavity/⚪️reentrant-area/🔣️.json"),
                proto: include_str!("🕳️concavity/⚪️reentrant-area/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reentrant-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/🪻️reentrant-volume/🦀️.rs"),
                typescript: include_str!("🕳️concavity/🪻️reentrant-volume/🟦️.ts"),
                graphql: include_str!("🕳️concavity/🪻️reentrant-volume/🔗️.graphql"),
                json_schema: include_str!("🕳️concavity/🪻️reentrant-volume/🔣️.json"),
                proto: include_str!("🕳️concavity/🪻️reentrant-volume/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.concavity-index.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/🚪️concavity-index/🦀️.rs"),
                typescript: include_str!("🕳️concavity/🚪️concavity-index/🟦️.ts"),
                graphql: include_str!("🕳️concavity/🚪️concavity-index/🔗️.graphql"),
                json_schema: include_str!("🕳️concavity/🚪️concavity-index/🔣️.json"),
                proto: include_str!("🕳️concavity/🚪️concavity-index/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/🌾️minimum-distance-to-neighbors/🦀️.rs"),
                typescript: include_str!("↔️clearance/🌾️minimum-distance-to-neighbors/🟦️.ts"),
                graphql: include_str!("↔️clearance/🌾️minimum-distance-to-neighbors/🔗️.graphql"),
                json_schema: include_str!("↔️clearance/🌾️minimum-distance-to-neighbors/🔣️.json"),
                proto: include_str!("↔️clearance/🌾️minimum-distance-to-neighbors/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.clearance-distribution.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/🦁️clearance-distribution/🦀️.rs"),
                typescript: include_str!("↔️clearance/🦁️clearance-distribution/🟦️.ts"),
                graphql: include_str!("↔️clearance/🦁️clearance-distribution/🔗️.graphql"),
                json_schema: include_str!("↔️clearance/🦁️clearance-distribution/🔣️.json"),
                proto: include_str!("↔️clearance/🦁️clearance-distribution/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.interference-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/🟡️interference-volume/🦀️.rs"),
                typescript: include_str!("↔️clearance/🟡️interference-volume/🟦️.ts"),
                graphql: include_str!("↔️clearance/🟡️interference-volume/🔗️.graphql"),
                json_schema: include_str!("↔️clearance/🟡️interference-volume/🔣️.json"),
                proto: include_str!("↔️clearance/🟡️interference-volume/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.overlap-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/🌹️overlap-volume/🦀️.rs"),
                typescript: include_str!("↔️clearance/🌹️overlap-volume/🟦️.ts"),
                graphql: include_str!("↔️clearance/🌹️overlap-volume/🔗️.graphql"),
                json_schema: include_str!("↔️clearance/🌹️overlap-volume/🔣️.json"),
                proto: include_str!("↔️clearance/🌹️overlap-volume/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.number-of-contacts.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🤝️adjacency/🔢️number-of-contacts/🦀️.rs"),
                typescript: include_str!("🤝️adjacency/🔢️number-of-contacts/🟦️.ts"),
                graphql: include_str!("🤝️adjacency/🔢️number-of-contacts/🔗️.graphql"),
                json_schema: include_str!("🤝️adjacency/🔢️number-of-contacts/🔣️.json"),
                proto: include_str!("🤝️adjacency/🔢️number-of-contacts/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.contact-graph-degree.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🤝️adjacency/🌳️contact-graph-degree/🦀️.rs"),
                typescript: include_str!("🤝️adjacency/🌳️contact-graph-degree/🟦️.ts"),
                graphql: include_str!("🤝️adjacency/🌳️contact-graph-degree/🔗️.graphql"),
                json_schema: include_str!("🤝️adjacency/🌳️contact-graph-degree/🔣️.json"),
                proto: include_str!("🤝️adjacency/🌳️contact-graph-degree/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.connected-components.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🤝️adjacency/🧩️connected-components/🦀️.rs"),
                typescript: include_str!("🤝️adjacency/🧩️connected-components/🟦️.ts"),
                graphql: include_str!("🤝️adjacency/🧩️connected-components/🔗️.graphql"),
                json_schema: include_str!("🤝️adjacency/🧩️connected-components/🔣️.json"),
                proto: include_str!("🤝️adjacency/🧩️connected-components/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.main-axis-direction.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧭️orientation/🚪️main-axis-direction/🦀️.rs"),
                typescript: include_str!("🧭️orientation/🚪️main-axis-direction/🟦️.ts"),
                graphql: include_str!("🧭️orientation/🚪️main-axis-direction/🔗️.graphql"),
                json_schema: include_str!("🧭️orientation/🚪️main-axis-direction/🔣️.json"),
                proto: include_str!("🧭️orientation/🚪️main-axis-direction/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.face-normal-distribution.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧭️orientation/📊️face-normal-distribution/🦀️.rs"),
                typescript: include_str!("🧭️orientation/📊️face-normal-distribution/🟦️.ts"),
                graphql: include_str!("🧭️orientation/📊️face-normal-distribution/🔗️.graphql"),
                json_schema: include_str!("🧭️orientation/📊️face-normal-distribution/🔣️.json"),
                proto: include_str!("🧭️orientation/📊️face-normal-distribution/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.orientation-consistency.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧭️orientation/🌻️orientation-consistency/🦀️.rs"),
                typescript: include_str!("🧭️orientation/🌻️orientation-consistency/🟦️.ts"),
                graphql: include_str!("🧭️orientation/🌻️orientation-consistency/🔗️.graphql"),
                json_schema: include_str!("🧭️orientation/🌻️orientation-consistency/🔣️.json"),
                proto: include_str!("🧭️orientation/🌻️orientation-consistency/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reflection-symmetry-score.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/🌴️reflection-symmetry-score/🦀️.rs"),
                typescript: include_str!("🪞️symmetry/🌴️reflection-symmetry-score/🟦️.ts"),
                graphql: include_str!("🪞️symmetry/🌴️reflection-symmetry-score/🔗️.graphql"),
                json_schema: include_str!("🪞️symmetry/🌴️reflection-symmetry-score/🔣️.json"),
                proto: include_str!("🪞️symmetry/🌴️reflection-symmetry-score/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.rotational-symmetry-score.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/🌻️rotational-symmetry-score/🦀️.rs"),
                typescript: include_str!("🪞️symmetry/🌻️rotational-symmetry-score/🟦️.ts"),
                graphql: include_str!("🪞️symmetry/🌻️rotational-symmetry-score/🔗️.graphql"),
                json_schema: include_str!("🪞️symmetry/🌻️rotational-symmetry-score/🔣️.json"),
                proto: include_str!("🪞️symmetry/🌻️rotational-symmetry-score/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reflection-symmetries.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/🐞️reflection-symmetries/🦀️.rs"),
                typescript: include_str!("🪞️symmetry/🐞️reflection-symmetries/🟦️.ts"),
                graphql: include_str!("🪞️symmetry/🐞️reflection-symmetries/🔗️.graphql"),
                json_schema: include_str!("🪞️symmetry/🐞️reflection-symmetries/🔣️.json"),
                proto: include_str!("🪞️symmetry/🐞️reflection-symmetries/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.rotational-symmetries.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/🐯️rotational-symmetries/🦀️.rs"),
                typescript: include_str!("🪞️symmetry/🐯️rotational-symmetries/🟦️.ts"),
                graphql: include_str!("🪞️symmetry/🐯️rotational-symmetries/🔗️.graphql"),
                json_schema: include_str!("🪞️symmetry/🐯️rotational-symmetries/🔣️.json"),
                proto: include_str!("🪞️symmetry/🐯️rotational-symmetries/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.repetition-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/🐸️repetition-ratio/🦀️.rs"),
                typescript: include_str!("🪞️symmetry/🐸️repetition-ratio/🟦️.ts"),
                graphql: include_str!("🪞️symmetry/🐸️repetition-ratio/🔗️.graphql"),
                json_schema: include_str!("🪞️symmetry/🐸️repetition-ratio/🔣️.json"),
                proto: include_str!("🪞️symmetry/🐸️repetition-ratio/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.modularity-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/🍊️modularity-ratio/🦀️.rs"),
                typescript: include_str!("🪞️symmetry/🍊️modularity-ratio/🟦️.ts"),
                graphql: include_str!("🪞️symmetry/🍊️modularity-ratio/🔗️.graphql"),
                json_schema: include_str!("🪞️symmetry/🍊️modularity-ratio/🔣️.json"),
                proto: include_str!("🪞️symmetry/🍊️modularity-ratio/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.deviation-from-ideal.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/🟫️deviation-from-ideal/🦀️.rs"),
                typescript: include_str!("🌊️roughness/🟫️deviation-from-ideal/🟦️.ts"),
                graphql: include_str!("🌊️roughness/🟫️deviation-from-ideal/🔗️.graphql"),
                json_schema: include_str!("🌊️roughness/🟫️deviation-from-ideal/🔣️.json"),
                proto: include_str!("🌊️roughness/🟫️deviation-from-ideal/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/🍎️deviation-from-smoothed-geometry/🦀️.rs"),
                typescript: include_str!("🌊️roughness/🍎️deviation-from-smoothed-geometry/🟦️.ts"),
                graphql: include_str!("🌊️roughness/🍎️deviation-from-smoothed-geometry/🔗️.graphql"),
                json_schema: include_str!("🌊️roughness/🍎️deviation-from-smoothed-geometry/🔣️.json"),
                proto: include_str!("🌊️roughness/🍎️deviation-from-smoothed-geometry/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.normal-variation.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/🎨️normal-variation/🦀️.rs"),
                typescript: include_str!("🌊️roughness/🎨️normal-variation/🟦️.ts"),
                graphql: include_str!("🌊️roughness/🎨️normal-variation/🔗️.graphql"),
                json_schema: include_str!("🌊️roughness/🎨️normal-variation/🔣️.json"),
                proto: include_str!("🌊️roughness/🎨️normal-variation/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.surface-waviness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/🖱️surface-waviness/🦀️.rs"),
                typescript: include_str!("🌊️roughness/🖱️surface-waviness/🟦️.ts"),
                graphql: include_str!("🌊️roughness/🖱️surface-waviness/🔗️.graphql"),
                json_schema: include_str!("🌊️roughness/🖱️surface-waviness/🔣️.json"),
                proto: include_str!("🌊️roughness/🖱️surface-waviness/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.irregularity.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/🦊️irregularity/🦀️.rs"),
                typescript: include_str!("🌊️roughness/🦊️irregularity/🟦️.ts"),
                graphql: include_str!("🌊️roughness/🦊️irregularity/🔗️.graphql"),
                json_schema: include_str!("🌊️roughness/🦊️irregularity/🔣️.json"),
                proto: include_str!("🌊️roughness/🦊️irregularity/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.holes.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/🐼️holes/🦀️.rs"),
                typescript: include_str!("🕸️topology/🐼️holes/🟦️.ts"),
                graphql: include_str!("🕸️topology/🐼️holes/🔗️.graphql"),
                json_schema: include_str!("🕸️topology/🐼️holes/🔣️.json"),
                proto: include_str!("🕸️topology/🐼️holes/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.handles.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/🐙️handles/🦀️.rs"),
                typescript: include_str!("🕸️topology/🐙️handles/🟦️.ts"),
                graphql: include_str!("🕸️topology/🐙️handles/🔗️.graphql"),
                json_schema: include_str!("🕸️topology/🐙️handles/🔣️.json"),
                proto: include_str!("🕸️topology/🐙️handles/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.boundary-loops.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/🎈️boundary-loops/🦀️.rs"),
                typescript: include_str!("🕸️topology/🎈️boundary-loops/🟦️.ts"),
                graphql: include_str!("🕸️topology/🎈️boundary-loops/🔗️.graphql"),
                json_schema: include_str!("🕸️topology/🎈️boundary-loops/🔣️.json"),
                proto: include_str!("🕸️topology/🎈️boundary-loops/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.euler-characteristic.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/🪄️euler-characteristic/🦀️.rs"),
                typescript: include_str!("🕸️topology/🪄️euler-characteristic/🟦️.ts"),
                graphql: include_str!("🕸️topology/🪄️euler-characteristic/🔗️.graphql"),
                json_schema: include_str!("🕸️topology/🪄️euler-characteristic/🔣️.json"),
                proto: include_str!("🕸️topology/🪄️euler-characteristic/🛰️.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.genus.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/🐨️genus/🦀️.rs"),
                typescript: include_str!("🕸️topology/🐨️genus/🟦️.ts"),
                graphql: include_str!("🕸️topology/🐨️genus/🔗️.graphql"),
                json_schema: include_str!("🕸️topology/🐨️genus/🔣️.json"),
                proto: include_str!("🕸️topology/🐨️genus/🛰️.proto"),
            },
        },
    ]
}
//#endregion 🧠️InferenceContract
//#region 🧪️ParityTests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn manifest_requires_exactly_one_fully_faceted_service_per_leaf() {
        let field_ids = GLTF_INFERENCE_FIELDS.iter().map(|field| field.id).collect::<std::collections::BTreeSet<_>>();
        let service_ids = GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS.iter().map(|descriptor| descriptor.id).collect::<std::collections::BTreeSet<_>>();
        let descriptors = gltf_artifact_inference_descriptors();
        let descriptor_ids = descriptors.iter().map(|descriptor| descriptor.id).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(field_ids.len(), 67);
        assert_eq!(service_ids.len(), 67);
        assert_eq!(descriptor_ids.len(), 67);
        assert_eq!(field_ids, service_ids);
        assert_eq!(service_ids, descriptor_ids);
        assert!(GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS.iter().all(|descriptor| descriptor.algorithm_version == 1 && !descriptor.cache_key.is_empty()));
        assert!(!field_ids.iter().any(|id| id.contains("geometric-analysis") || id.ends_with(".bounds.v1")));
    }
}
//#endregion 🧪️ParityTests
