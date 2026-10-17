//! 💡️ Public glTF inference assembly.

use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use framework_schema::ArtifactSchema;

use crate::standards::v2_0::subsets::any::schema::modules::measurement_contracts::*;
use crate::standards::v2_0::subsets::any::schema::inferences::{adjacency::*, area_volume::*, clearance::*, compactness::*, concavity::*, curvature::*, mass_distribution::*, orientation::*, proportion::*, roughness::*, size::*, symmetry::*, thickness::*, topology::*};

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
    pub encode: fn(&GltfEntityIndicators) -> semio_framework_value::DslValue,
}

pub const GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS: &[GltfInferenceLeafServiceDescriptor] = &[
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.overall-size.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overall-size.v1:geometry-v2", encode: |indicators| overall_size::encode_result(&indicators.size.overall_size) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.axis-aligned-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2", encode: |indicators| axis_aligned_bounds::encode_result(&indicators.size.axis_aligned_bounds) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.oriented-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.oriented-bounds.v1:geometry-v2", encode: |indicators| oriented_bounds::encode_result(&indicators.size.oriented_bounds) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.bounding-box-dimensions.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.bounding-box-dimensions.v1:geometry-v2", encode: |indicators| bounding_box_dimensions::encode_result(&indicators.size.bounding_box_dimensions) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.characteristic-length.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.characteristic-length.v1:geometry-v2", encode: |indicators| characteristic_length::encode_result(&indicators.size.characteristic_length) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.footprint-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.footprint-area.v1:geometry-v2", encode: |indicators| footprint_area::encode_result(&indicators.size.footprint_area) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.projected-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.projected-area.v1:geometry-v2", encode: |indicators| projected_area::encode_result(&indicators.size.projected_area) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.surface-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.surface-area.v1:geometry-v2", encode: |indicators| surface_area::encode_result(&indicators.area_volume.surface_area) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.total-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.total-area.v1:geometry-v2", encode: |indicators| total_area::encode_result(&indicators.area_volume.total_area) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.exposed-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.exposed-area.v1:geometry-v2", encode: |indicators| exposed_area::encode_result(&indicators.area_volume.exposed_area) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.contact-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.contact-area.v1:geometry-v2", encode: |indicators| contact_area::encode_result(&indicators.area_volume.contact_area) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.volume.v1:geometry-v2", encode: |indicators| volume::encode_result(&indicators.area_volume.volume) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.enclosed-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.enclosed-volume.v1:geometry-v2", encode: |indicators| enclosed_volume::encode_result(&indicators.area_volume.enclosed_volume) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.material-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.material-volume.v1:geometry-v2", encode: |indicators| material_volume::encode_result(&indicators.area_volume.material_volume) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.void-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.void-volume.v1:geometry-v2", encode: |indicators| void_volume::encode_result(&indicators.area_volume.void_volume) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.compactness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.compactness.v1:geometry-v2", encode: |indicators| measure::encode_result(&indicators.compactness.compactness) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.surface-to-volume-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.surface-to-volume-ratio.v1:geometry-v2", encode: |indicators| surface_to_volume_ratio::encode_result(&indicators.compactness.surface_to_volume_ratio) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.sphericity.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.sphericity.v1:geometry-v2", encode: |indicators| sphericity::encode_result(&indicators.compactness.sphericity) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.compactness-index.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.compactness-index.v1:geometry-v2", encode: |indicators| compactness_index::encode_result(&indicators.compactness.compactness_index) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.hull-fill-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.hull-fill-ratio.v1:geometry-v2", encode: |indicators| hull_fill_ratio::encode_result(&indicators.compactness.hull_fill_ratio) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.aspect-ratios.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.aspect-ratios.v1:geometry-v2", encode: |indicators| aspect_ratios::encode_result(&indicators.proportion.aspect_ratios) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.slenderness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.slenderness.v1:geometry-v2", encode: |indicators| slenderness::encode_result(&indicators.proportion.slenderness) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.flatness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.flatness.v1:geometry-v2", encode: |indicators| flatness::encode_result(&indicators.proportion.flatness) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.elongation.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.elongation.v1:geometry-v2", encode: |indicators| elongation::encode_result(&indicators.proportion.elongation) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.centroid.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.centroid.v1:geometry-v2", encode: |indicators| centroid::encode_result(&indicators.mass.centroid) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.principal-frame.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.principal-frame.v1:geometry-v2", encode: |indicators| principal_frame::encode_result(&indicators.mass.principal_frame) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.principal-axes.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.principal-axes.v1:geometry-v2", encode: |indicators| principal_axes::encode_result(&indicators.mass.principal_axes) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.moments-of-inertia.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.moments-of-inertia.v1:geometry-v2", encode: |indicators| moments_of_inertia::encode_result(&indicators.mass.moments_of_inertia) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.inertia-tensor.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.inertia-tensor.v1:geometry-v2", encode: |indicators| inertia_tensor::encode_result(&indicators.mass.inertia_tensor) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.mean-curvature.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.mean-curvature.v1:geometry-v2", encode: |indicators| mean_curvature::encode_result(&indicators.curvature.mean_curvature) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.gaussian-curvature.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.gaussian-curvature.v1:geometry-v2", encode: |indicators| gaussian_curvature::encode_result(&indicators.curvature.gaussian_curvature) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.curvature-histogram.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.curvature-histogram.v1:geometry-v2", encode: |indicators| curvature_histogram::encode_result(&indicators.curvature.curvature_histogram) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.sharp-feature-proportion.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.sharp-feature-proportion.v1:geometry-v2", encode: |indicators| sharp_feature_proportion::encode_result(&indicators.curvature.sharp_feature_proportion) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.mean-thickness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.mean-thickness.v1:geometry-v2", encode: |indicators| mean_thickness::encode_result(&indicators.thickness.mean_thickness) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.minimum-thickness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.minimum-thickness.v1:geometry-v2", encode: |indicators| minimum_thickness::encode_result(&indicators.thickness.minimum_thickness) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.thickness-variability.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-variability.v1:geometry-v2", encode: |indicators| thickness_variability::encode_result(&indicators.thickness.thickness_variability) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.thickness-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-distribution.v1:geometry-v2", encode: |indicators| thickness_distribution::encode_result(&indicators.thickness.thickness_distribution) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.convex-hull-gap.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.convex-hull-gap.v1:geometry-v2", encode: |indicators| convex_hull_gap::encode_result(&indicators.concavity.convex_hull_gap) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.reentrant-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reentrant-area.v1:geometry-v2", encode: |indicators| reentrant_area::encode_result(&indicators.concavity.reentrant_area) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.reentrant-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reentrant-volume.v1:geometry-v2", encode: |indicators| reentrant_volume::encode_result(&indicators.concavity.reentrant_volume) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.concavity-index.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.concavity-index.v1:geometry-v2", encode: |indicators| concavity_index::encode_result(&indicators.concavity.concavity_index) },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1:geometry-v2",
        encode: |indicators| minimum_distance_to_neighbors::encode_result(&indicators.clearance.minimum_distance_to_neighbors),
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.clearance-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.clearance-distribution.v1:geometry-v2", encode: |indicators| clearance_distribution::encode_result(&indicators.clearance.clearance_distribution) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.interference-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.interference-volume.v1:geometry-v2", encode: |indicators| interference_volume::encode_result(&indicators.clearance.interference_volume) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.overlap-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overlap-volume.v1:geometry-v2", encode: |indicators| overlap_volume::encode_result(&indicators.clearance.overlap_volume) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.number-of-contacts.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.number-of-contacts.v1:geometry-v2", encode: |indicators| number_of_contacts::encode_result(&indicators.adjacency.number_of_contacts) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.contact-graph-degree.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.contact-graph-degree.v1:geometry-v2", encode: |indicators| contact_graph_degree::encode_result(&indicators.adjacency.contact_graph_degree) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.connected-components.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.connected-components.v1:geometry-v2", encode: |indicators| connected_components::encode_result(&indicators.adjacency.connected_components) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.main-axis-direction.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.main-axis-direction.v1:geometry-v2", encode: |indicators| main_axis_direction::encode_result(&indicators.orientation.main_axis_direction) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.face-normal-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.face-normal-distribution.v1:geometry-v2", encode: |indicators| face_normal_distribution::encode_result(&indicators.orientation.face_normal_distribution) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.orientation-consistency.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.orientation-consistency.v1:geometry-v2", encode: |indicators| orientation_consistency::encode_result(&indicators.orientation.orientation_consistency) },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.reflection-symmetry-score.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.reflection-symmetry-score.v1:geometry-v2",
        encode: |indicators| reflection_symmetry_score::encode_result(&indicators.symmetry.reflection_symmetry_score),
    },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.rotational-symmetry-score.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.rotational-symmetry-score.v1:geometry-v2",
        encode: |indicators| rotational_symmetry_score::encode_result(&indicators.symmetry.rotational_symmetry_score),
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.reflection-symmetries.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reflection-symmetries.v1:geometry-v2", encode: |indicators| reflection_symmetries::encode_result(&indicators.symmetry.reflection_symmetries) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.rotational-symmetries.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.rotational-symmetries.v1:geometry-v2", encode: |indicators| rotational_symmetries::encode_result(&indicators.symmetry.rotational_symmetries) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.repetition-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.repetition-ratio.v1:geometry-v2", encode: |indicators| repetition_ratio::encode_result(&indicators.symmetry.repetition_ratio) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.modularity-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.modularity-ratio.v1:geometry-v2", encode: |indicators| modularity_ratio::encode_result(&indicators.symmetry.modularity_ratio) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.deviation-from-ideal.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.deviation-from-ideal.v1:geometry-v2", encode: |indicators| deviation_from_ideal::encode_result(&indicators.roughness.deviation_from_ideal) },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1:geometry-v2",
        encode: |indicators| deviation_from_smoothed_geometry::encode_result(&indicators.roughness.deviation_from_smoothed_geometry),
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.normal-variation.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.normal-variation.v1:geometry-v2", encode: |indicators| normal_variation::encode_result(&indicators.roughness.normal_variation) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.surface-waviness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.surface-waviness.v1:geometry-v2", encode: |indicators| surface_waviness::encode_result(&indicators.roughness.surface_waviness) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.irregularity.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.irregularity.v1:geometry-v2", encode: |indicators| irregularity::encode_result(&indicators.roughness.irregularity) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.holes.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.holes.v1:geometry-v2", encode: |indicators| holes::encode_result(&indicators.topology.holes) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.handles.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.handles.v1:geometry-v2", encode: |indicators| handles::encode_result(&indicators.topology.handles) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.boundary-loops.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.boundary-loops.v1:geometry-v2", encode: |indicators| boundary_loops::encode_result(&indicators.topology.boundary_loops) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.euler-characteristic.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.euler-characteristic.v1:geometry-v2", encode: |indicators| euler_characteristic::encode_result(&indicators.topology.euler_characteristic) },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.genus.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.genus.v1:geometry-v2", encode: |indicators| genus::encode_result(&indicators.topology.genus) },
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
pub use crate::standards::v2_0::subsets::any::schema::inferences::dag_assembly::compute_gltf_inference;
//#endregion 🧩️Assembly

//#region 🧠️InferenceContract

impl Default for GltfInference {
    fn default() -> Self {
        let snapshot = &GltfSnapshot::default();

        Self { geometry: compute_gltf_inference(&semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfDecodedSnapshot { document:&snapshot.document, accessors:Vec::new(), buffer_fingerprints:Vec::new() }) }
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_artifact_inference_descriptors() -> Vec<semio_framework_schema_registry::ArtifactInferenceDescriptor> {
    vec![
        clearance_distribution::inference_descriptor(),
        interference_volume::inference_descriptor(),
        minimum_distance_to_neighbors::inference_descriptor(),
        overlap_volume::inference_descriptor(),
        mean_thickness::inference_descriptor(),
        minimum_thickness::inference_descriptor(),
        thickness_distribution::inference_descriptor(),
        thickness_variability::inference_descriptor(),
        centroid::inference_descriptor(),
        inertia_tensor::inference_descriptor(),
        moments_of_inertia::inference_descriptor(),
        principal_axes::inference_descriptor(),
        principal_frame::inference_descriptor(),
        curvature_histogram::inference_descriptor(),
        gaussian_curvature::inference_descriptor(),
        mean_curvature::inference_descriptor(),
        sharp_feature_proportion::inference_descriptor(),
        deviation_from_ideal::inference_descriptor(),
        deviation_from_smoothed_geometry::inference_descriptor(),
        irregularity::inference_descriptor(),
        normal_variation::inference_descriptor(),
        surface_waviness::inference_descriptor(),
        aspect_ratios::inference_descriptor(),
        elongation::inference_descriptor(),
        flatness::inference_descriptor(),
        slenderness::inference_descriptor(),
        axis_aligned_bounds::inference_descriptor(),
        bounding_box_dimensions::inference_descriptor(),
        characteristic_length::inference_descriptor(),
        footprint_area::inference_descriptor(),
        oriented_bounds::inference_descriptor(),
        overall_size::inference_descriptor(),
        projected_area::inference_descriptor(),
        concavity_index::inference_descriptor(),
        convex_hull_gap::inference_descriptor(),
        reentrant_area::inference_descriptor(),
        reentrant_volume::inference_descriptor(),
        boundary_loops::inference_descriptor(),
        euler_characteristic::inference_descriptor(),
        genus::inference_descriptor(),
        handles::inference_descriptor(),
        holes::inference_descriptor(),
        compactness_index::inference_descriptor(),
        hull_fill_ratio::inference_descriptor(),
        measure::inference_descriptor(),
        sphericity::inference_descriptor(),
        surface_to_volume_ratio::inference_descriptor(),
        connected_components::inference_descriptor(),
        contact_graph_degree::inference_descriptor(),
        number_of_contacts::inference_descriptor(),
        face_normal_distribution::inference_descriptor(),
        main_axis_direction::inference_descriptor(),
        orientation_consistency::inference_descriptor(),
        contact_area::inference_descriptor(),
        enclosed_volume::inference_descriptor(),
        exposed_area::inference_descriptor(),
        material_volume::inference_descriptor(),
        surface_area::inference_descriptor(),
        total_area::inference_descriptor(),
        void_volume::inference_descriptor(),
        volume::inference_descriptor(),
        modularity_ratio::inference_descriptor(),
        reflection_symmetries::inference_descriptor(),
        reflection_symmetry_score::inference_descriptor(),
        repetition_ratio::inference_descriptor(),
        rotational_symmetries::inference_descriptor(),
        rotational_symmetry_score::inference_descriptor(),
    ]
}
//#endregion 🧠️InferenceContract
//#region 🧪️ParityTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️ParityTests
