//! 💡️ Public glTF inference assembly.

use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::super::modules::measurement_contracts::*;
use super::compactness;
use super::{adjacency::*, area_volume::*, clearance::*, compactness::*, concavity::*, curvature::*, mass_distribution::*, orientation::*, proportion::*, roughness::*, size::*, symmetry::*, thickness::*, topology::*};

//#region 🔖️PublicRecords
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfPartInference {
    pub address: GltfEntityAddress,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub indicators: GltfEntityIndicators,
    pub diagnostic_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
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
    pub encode: fn(&GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error>,
}

pub const GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS: &[GltfInferenceLeafServiceDescriptor] = &[
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.overall-size.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overall-size.v1:geometry-v2", encode: overall_size::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.axis-aligned-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2", encode: axis_aligned_bounds::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.oriented-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.oriented-bounds.v1:geometry-v2", encode: oriented_bounds::encode_result },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.bounding-box-dimensions.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.bounding-box-dimensions.v1:geometry-v2",
        encode: bounding_box_dimensions::encode_result,
    },
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
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.surface-to-volume-ratio.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.surface-to-volume-ratio.v1:geometry-v2",
        encode: surface_to_volume_ratio::encode_result,
    },
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
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.sharp-feature-proportion.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.sharp-feature-proportion.v1:geometry-v2",
        encode: sharp_feature_proportion::encode_result,
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.mean-thickness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.mean-thickness.v1:geometry-v2", encode: mean_thickness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.minimum-thickness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.minimum-thickness.v1:geometry-v2", encode: minimum_thickness::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.thickness-variability.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-variability.v1:geometry-v2", encode: thickness_variability::encode_result },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.thickness-distribution.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.thickness-distribution.v1:geometry-v2",
        encode: thickness_distribution::encode_result,
    },
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
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.clearance-distribution.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.clearance-distribution.v1:geometry-v2",
        encode: clearance_distribution::encode_result,
    },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.interference-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.interference-volume.v1:geometry-v2", encode: interference_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.overlap-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overlap-volume.v1:geometry-v2", encode: overlap_volume::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.number-of-contacts.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.number-of-contacts.v1:geometry-v2", encode: number_of_contacts::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.contact-graph-degree.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.contact-graph-degree.v1:geometry-v2", encode: contact_graph_degree::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.connected-components.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.connected-components.v1:geometry-v2", encode: connected_components::encode_result },
    GltfInferenceLeafServiceDescriptor { id: "s.stdio.gltf.inference.main-axis-direction.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.main-axis-direction.v1:geometry-v2", encode: main_axis_direction::encode_result },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.face-normal-distribution.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.face-normal-distribution.v1:geometry-v2",
        encode: face_normal_distribution::encode_result,
    },
    GltfInferenceLeafServiceDescriptor {
        id: "s.stdio.gltf.inference.orientation-consistency.v1",
        algorithm_version: 1,
        cache_key: "s.stdio.gltf.inference.orientation-consistency.v1:geometry-v2",
        encode: orientation_consistency::encode_result,
    },
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

pub async fn gltf_inference_leaf_service_descriptor(id: &str) -> Option<&'static GltfInferenceLeafServiceDescriptor> {
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

pub async fn invalidated_gltf_inference_fields(touched: Option<&protocol::TouchedPaths>) -> Vec<&'static str> {
    GLTF_INFERENCE_FIELDS.iter().filter(|field| touched.is_none_or(|paths| paths.intersects_any(field.reads))).map(|field| field.id).collect()
}
//#endregion 🧭️LeafDag

//#region 🧩️Assembly
pub use super::dag_assembly::compute_gltf_inference;
//#endregion 🧩️Assembly

//#region 🧠️InferenceContract
impl protocol::Inference<GltfSnapshot> for GltfInference {
    async fn infer(snapshot: &GltfSnapshot) -> Self {
        Self { geometry: compute_gltf_inference(snapshot) }
    }
}

impl Default for GltfInference {
    async fn default() -> Self {
        <Self as protocol::Inference<GltfSnapshot>>::infer(&GltfSnapshot::default())
    }
}

impl protocol::InferenceSpec<GltfSnapshot> for GltfInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.gltf.inference"
    }
    async fn schema_version() -> u32 {
        2
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        GLTF_INFERENCE_FIELDS
    }
}

impl ArtifactInferrer for crate::artifacts::gltf::standards::v2_0::subsets::any::schema::GltfBuilder {
    type Snapshot = GltfSnapshot;
    type Inference = GltfInference;
}

pub async fn gltf_artifact_inference_descriptors() -> Vec<schema::ArtifactInferenceDescriptor> {
    vec![
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.overall-size.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/overall-size/🦀️component.rs"),
                typescript: include_str!("📦️size/overall-size/🟦️component.ts"),
                graphql: include_str!("📦️size/overall-size/🔗️component.graphql"),
                json_schema: include_str!("📦️size/overall-size/🔣️component.json"),
                proto: include_str!("📦️size/overall-size/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.axis-aligned-bounds.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/axis-aligned-bounds/🦀️component.rs"),
                typescript: include_str!("📦️size/axis-aligned-bounds/🟦️component.ts"),
                graphql: include_str!("📦️size/axis-aligned-bounds/🔗️component.graphql"),
                json_schema: include_str!("📦️size/axis-aligned-bounds/🔣️component.json"),
                proto: include_str!("📦️size/axis-aligned-bounds/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.oriented-bounds.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/oriented-bounds/🦀️component.rs"),
                typescript: include_str!("📦️size/oriented-bounds/🟦️component.ts"),
                graphql: include_str!("📦️size/oriented-bounds/🔗️component.graphql"),
                json_schema: include_str!("📦️size/oriented-bounds/🔣️component.json"),
                proto: include_str!("📦️size/oriented-bounds/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.bounding-box-dimensions.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/bounding-box-dimensions/🦀️component.rs"),
                typescript: include_str!("📦️size/bounding-box-dimensions/🟦️component.ts"),
                graphql: include_str!("📦️size/bounding-box-dimensions/🔗️component.graphql"),
                json_schema: include_str!("📦️size/bounding-box-dimensions/🔣️component.json"),
                proto: include_str!("📦️size/bounding-box-dimensions/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.characteristic-length.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/characteristic-length/🦀️component.rs"),
                typescript: include_str!("📦️size/characteristic-length/🟦️component.ts"),
                graphql: include_str!("📦️size/characteristic-length/🔗️component.graphql"),
                json_schema: include_str!("📦️size/characteristic-length/🔣️component.json"),
                proto: include_str!("📦️size/characteristic-length/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.footprint-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/footprint-area/🦀️component.rs"),
                typescript: include_str!("📦️size/footprint-area/🟦️component.ts"),
                graphql: include_str!("📦️size/footprint-area/🔗️component.graphql"),
                json_schema: include_str!("📦️size/footprint-area/🔣️component.json"),
                proto: include_str!("📦️size/footprint-area/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.projected-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📦️size/projected-area/🦀️component.rs"),
                typescript: include_str!("📦️size/projected-area/🟦️component.ts"),
                graphql: include_str!("📦️size/projected-area/🔗️component.graphql"),
                json_schema: include_str!("📦️size/projected-area/🔣️component.json"),
                proto: include_str!("📦️size/projected-area/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.surface-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/surface-area/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/surface-area/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/surface-area/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/surface-area/🔣️component.json"),
                proto: include_str!("🧱️area-volume/surface-area/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.total-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/total-area/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/total-area/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/total-area/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/total-area/🔣️component.json"),
                proto: include_str!("🧱️area-volume/total-area/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.exposed-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/exposed-area/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/exposed-area/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/exposed-area/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/exposed-area/🔣️component.json"),
                proto: include_str!("🧱️area-volume/exposed-area/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.contact-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/contact-area/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/contact-area/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/contact-area/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/contact-area/🔣️component.json"),
                proto: include_str!("🧱️area-volume/contact-area/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/volume/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/volume/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/volume/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/volume/🔣️component.json"),
                proto: include_str!("🧱️area-volume/volume/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.enclosed-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/enclosed-volume/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/enclosed-volume/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/enclosed-volume/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/enclosed-volume/🔣️component.json"),
                proto: include_str!("🧱️area-volume/enclosed-volume/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.material-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/material-volume/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/material-volume/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/material-volume/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/material-volume/🔣️component.json"),
                proto: include_str!("🧱️area-volume/material-volume/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.void-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧱️area-volume/void-volume/🦀️component.rs"),
                typescript: include_str!("🧱️area-volume/void-volume/🟦️component.ts"),
                graphql: include_str!("🧱️area-volume/void-volume/🔗️component.graphql"),
                json_schema: include_str!("🧱️area-volume/void-volume/🔣️component.json"),
                proto: include_str!("🧱️area-volume/void-volume/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.compactness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/compactness/🦀️component.rs"),
                typescript: include_str!("⚪️compactness/compactness/🟦️component.ts"),
                graphql: include_str!("⚪️compactness/compactness/🔗️component.graphql"),
                json_schema: include_str!("⚪️compactness/compactness/🔣️component.json"),
                proto: include_str!("⚪️compactness/compactness/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.surface-to-volume-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/surface-to-volume-ratio/🦀️component.rs"),
                typescript: include_str!("⚪️compactness/surface-to-volume-ratio/🟦️component.ts"),
                graphql: include_str!("⚪️compactness/surface-to-volume-ratio/🔗️component.graphql"),
                json_schema: include_str!("⚪️compactness/surface-to-volume-ratio/🔣️component.json"),
                proto: include_str!("⚪️compactness/surface-to-volume-ratio/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.sphericity.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/sphericity/🦀️component.rs"),
                typescript: include_str!("⚪️compactness/sphericity/🟦️component.ts"),
                graphql: include_str!("⚪️compactness/sphericity/🔗️component.graphql"),
                json_schema: include_str!("⚪️compactness/sphericity/🔣️component.json"),
                proto: include_str!("⚪️compactness/sphericity/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.compactness-index.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/compactness-index/🦀️component.rs"),
                typescript: include_str!("⚪️compactness/compactness-index/🟦️component.ts"),
                graphql: include_str!("⚪️compactness/compactness-index/🔗️component.graphql"),
                json_schema: include_str!("⚪️compactness/compactness-index/🔣️component.json"),
                proto: include_str!("⚪️compactness/compactness-index/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.hull-fill-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚪️compactness/hull-fill-ratio/🦀️component.rs"),
                typescript: include_str!("⚪️compactness/hull-fill-ratio/🟦️component.ts"),
                graphql: include_str!("⚪️compactness/hull-fill-ratio/🔗️component.graphql"),
                json_schema: include_str!("⚪️compactness/hull-fill-ratio/🔣️component.json"),
                proto: include_str!("⚪️compactness/hull-fill-ratio/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.aspect-ratios.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/aspect-ratios/🦀️component.rs"),
                typescript: include_str!("📏️proportion/aspect-ratios/🟦️component.ts"),
                graphql: include_str!("📏️proportion/aspect-ratios/🔗️component.graphql"),
                json_schema: include_str!("📏️proportion/aspect-ratios/🔣️component.json"),
                proto: include_str!("📏️proportion/aspect-ratios/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.slenderness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/slenderness/🦀️component.rs"),
                typescript: include_str!("📏️proportion/slenderness/🟦️component.ts"),
                graphql: include_str!("📏️proportion/slenderness/🔗️component.graphql"),
                json_schema: include_str!("📏️proportion/slenderness/🔣️component.json"),
                proto: include_str!("📏️proportion/slenderness/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.flatness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/flatness/🦀️component.rs"),
                typescript: include_str!("📏️proportion/flatness/🟦️component.ts"),
                graphql: include_str!("📏️proportion/flatness/🔗️component.graphql"),
                json_schema: include_str!("📏️proportion/flatness/🔣️component.json"),
                proto: include_str!("📏️proportion/flatness/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.elongation.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("📏️proportion/elongation/🦀️component.rs"),
                typescript: include_str!("📏️proportion/elongation/🟦️component.ts"),
                graphql: include_str!("📏️proportion/elongation/🔗️component.graphql"),
                json_schema: include_str!("📏️proportion/elongation/🔣️component.json"),
                proto: include_str!("📏️proportion/elongation/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.centroid.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/centroid/🦀️component.rs"),
                typescript: include_str!("⚖️mass-distribution/centroid/🟦️component.ts"),
                graphql: include_str!("⚖️mass-distribution/centroid/🔗️component.graphql"),
                json_schema: include_str!("⚖️mass-distribution/centroid/🔣️component.json"),
                proto: include_str!("⚖️mass-distribution/centroid/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.principal-frame.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/principal-frame/🦀️component.rs"),
                typescript: include_str!("⚖️mass-distribution/principal-frame/🟦️component.ts"),
                graphql: include_str!("⚖️mass-distribution/principal-frame/🔗️component.graphql"),
                json_schema: include_str!("⚖️mass-distribution/principal-frame/🔣️component.json"),
                proto: include_str!("⚖️mass-distribution/principal-frame/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.principal-axes.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/principal-axes/🦀️component.rs"),
                typescript: include_str!("⚖️mass-distribution/principal-axes/🟦️component.ts"),
                graphql: include_str!("⚖️mass-distribution/principal-axes/🔗️component.graphql"),
                json_schema: include_str!("⚖️mass-distribution/principal-axes/🔣️component.json"),
                proto: include_str!("⚖️mass-distribution/principal-axes/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.moments-of-inertia.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/moments-of-inertia/🦀️component.rs"),
                typescript: include_str!("⚖️mass-distribution/moments-of-inertia/🟦️component.ts"),
                graphql: include_str!("⚖️mass-distribution/moments-of-inertia/🔗️component.graphql"),
                json_schema: include_str!("⚖️mass-distribution/moments-of-inertia/🔣️component.json"),
                proto: include_str!("⚖️mass-distribution/moments-of-inertia/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.inertia-tensor.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("⚖️mass-distribution/inertia-tensor/🦀️component.rs"),
                typescript: include_str!("⚖️mass-distribution/inertia-tensor/🟦️component.ts"),
                graphql: include_str!("⚖️mass-distribution/inertia-tensor/🔗️component.graphql"),
                json_schema: include_str!("⚖️mass-distribution/inertia-tensor/🔣️component.json"),
                proto: include_str!("⚖️mass-distribution/inertia-tensor/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.mean-curvature.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/mean-curvature/🦀️component.rs"),
                typescript: include_str!("🌀️curvature/mean-curvature/🟦️component.ts"),
                graphql: include_str!("🌀️curvature/mean-curvature/🔗️component.graphql"),
                json_schema: include_str!("🌀️curvature/mean-curvature/🔣️component.json"),
                proto: include_str!("🌀️curvature/mean-curvature/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.gaussian-curvature.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/gaussian-curvature/🦀️component.rs"),
                typescript: include_str!("🌀️curvature/gaussian-curvature/🟦️component.ts"),
                graphql: include_str!("🌀️curvature/gaussian-curvature/🔗️component.graphql"),
                json_schema: include_str!("🌀️curvature/gaussian-curvature/🔣️component.json"),
                proto: include_str!("🌀️curvature/gaussian-curvature/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.curvature-histogram.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/curvature-histogram/🦀️component.rs"),
                typescript: include_str!("🌀️curvature/curvature-histogram/🟦️component.ts"),
                graphql: include_str!("🌀️curvature/curvature-histogram/🔗️component.graphql"),
                json_schema: include_str!("🌀️curvature/curvature-histogram/🔣️component.json"),
                proto: include_str!("🌀️curvature/curvature-histogram/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.sharp-feature-proportion.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌀️curvature/sharp-feature-proportion/🦀️component.rs"),
                typescript: include_str!("🌀️curvature/sharp-feature-proportion/🟦️component.ts"),
                graphql: include_str!("🌀️curvature/sharp-feature-proportion/🔗️component.graphql"),
                json_schema: include_str!("🌀️curvature/sharp-feature-proportion/🔣️component.json"),
                proto: include_str!("🌀️curvature/sharp-feature-proportion/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.mean-thickness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/mean-thickness/🦀️component.rs"),
                typescript: include_str!("↕️thickness/mean-thickness/🟦️component.ts"),
                graphql: include_str!("↕️thickness/mean-thickness/🔗️component.graphql"),
                json_schema: include_str!("↕️thickness/mean-thickness/🔣️component.json"),
                proto: include_str!("↕️thickness/mean-thickness/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.minimum-thickness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/minimum-thickness/🦀️component.rs"),
                typescript: include_str!("↕️thickness/minimum-thickness/🟦️component.ts"),
                graphql: include_str!("↕️thickness/minimum-thickness/🔗️component.graphql"),
                json_schema: include_str!("↕️thickness/minimum-thickness/🔣️component.json"),
                proto: include_str!("↕️thickness/minimum-thickness/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.thickness-variability.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/thickness-variability/🦀️component.rs"),
                typescript: include_str!("↕️thickness/thickness-variability/🟦️component.ts"),
                graphql: include_str!("↕️thickness/thickness-variability/🔗️component.graphql"),
                json_schema: include_str!("↕️thickness/thickness-variability/🔣️component.json"),
                proto: include_str!("↕️thickness/thickness-variability/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.thickness-distribution.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↕️thickness/thickness-distribution/🦀️component.rs"),
                typescript: include_str!("↕️thickness/thickness-distribution/🟦️component.ts"),
                graphql: include_str!("↕️thickness/thickness-distribution/🔗️component.graphql"),
                json_schema: include_str!("↕️thickness/thickness-distribution/🔣️component.json"),
                proto: include_str!("↕️thickness/thickness-distribution/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.convex-hull-gap.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/convex-hull-gap/🦀️component.rs"),
                typescript: include_str!("🕳️concavity/convex-hull-gap/🟦️component.ts"),
                graphql: include_str!("🕳️concavity/convex-hull-gap/🔗️component.graphql"),
                json_schema: include_str!("🕳️concavity/convex-hull-gap/🔣️component.json"),
                proto: include_str!("🕳️concavity/convex-hull-gap/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reentrant-area.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/reentrant-area/🦀️component.rs"),
                typescript: include_str!("🕳️concavity/reentrant-area/🟦️component.ts"),
                graphql: include_str!("🕳️concavity/reentrant-area/🔗️component.graphql"),
                json_schema: include_str!("🕳️concavity/reentrant-area/🔣️component.json"),
                proto: include_str!("🕳️concavity/reentrant-area/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reentrant-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/reentrant-volume/🦀️component.rs"),
                typescript: include_str!("🕳️concavity/reentrant-volume/🟦️component.ts"),
                graphql: include_str!("🕳️concavity/reentrant-volume/🔗️component.graphql"),
                json_schema: include_str!("🕳️concavity/reentrant-volume/🔣️component.json"),
                proto: include_str!("🕳️concavity/reentrant-volume/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.concavity-index.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕳️concavity/concavity-index/🦀️component.rs"),
                typescript: include_str!("🕳️concavity/concavity-index/🟦️component.ts"),
                graphql: include_str!("🕳️concavity/concavity-index/🔗️component.graphql"),
                json_schema: include_str!("🕳️concavity/concavity-index/🔣️component.json"),
                proto: include_str!("🕳️concavity/concavity-index/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/minimum-distance-to-neighbors/🦀️component.rs"),
                typescript: include_str!("↔️clearance/minimum-distance-to-neighbors/🟦️component.ts"),
                graphql: include_str!("↔️clearance/minimum-distance-to-neighbors/🔗️component.graphql"),
                json_schema: include_str!("↔️clearance/minimum-distance-to-neighbors/🔣️component.json"),
                proto: include_str!("↔️clearance/minimum-distance-to-neighbors/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.clearance-distribution.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/clearance-distribution/🦀️component.rs"),
                typescript: include_str!("↔️clearance/clearance-distribution/🟦️component.ts"),
                graphql: include_str!("↔️clearance/clearance-distribution/🔗️component.graphql"),
                json_schema: include_str!("↔️clearance/clearance-distribution/🔣️component.json"),
                proto: include_str!("↔️clearance/clearance-distribution/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.interference-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/interference-volume/🦀️component.rs"),
                typescript: include_str!("↔️clearance/interference-volume/🟦️component.ts"),
                graphql: include_str!("↔️clearance/interference-volume/🔗️component.graphql"),
                json_schema: include_str!("↔️clearance/interference-volume/🔣️component.json"),
                proto: include_str!("↔️clearance/interference-volume/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.overlap-volume.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("↔️clearance/overlap-volume/🦀️component.rs"),
                typescript: include_str!("↔️clearance/overlap-volume/🟦️component.ts"),
                graphql: include_str!("↔️clearance/overlap-volume/🔗️component.graphql"),
                json_schema: include_str!("↔️clearance/overlap-volume/🔣️component.json"),
                proto: include_str!("↔️clearance/overlap-volume/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.number-of-contacts.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🔗️adjacency/number-of-contacts/🦀️component.rs"),
                typescript: include_str!("🔗️adjacency/number-of-contacts/🟦️component.ts"),
                graphql: include_str!("🔗️adjacency/number-of-contacts/🔗️component.graphql"),
                json_schema: include_str!("🔗️adjacency/number-of-contacts/🔣️component.json"),
                proto: include_str!("🔗️adjacency/number-of-contacts/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.contact-graph-degree.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🔗️adjacency/contact-graph-degree/🦀️component.rs"),
                typescript: include_str!("🔗️adjacency/contact-graph-degree/🟦️component.ts"),
                graphql: include_str!("🔗️adjacency/contact-graph-degree/🔗️component.graphql"),
                json_schema: include_str!("🔗️adjacency/contact-graph-degree/🔣️component.json"),
                proto: include_str!("🔗️adjacency/contact-graph-degree/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.connected-components.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🔗️adjacency/connected-components/🦀️component.rs"),
                typescript: include_str!("🔗️adjacency/connected-components/🟦️component.ts"),
                graphql: include_str!("🔗️adjacency/connected-components/🔗️component.graphql"),
                json_schema: include_str!("🔗️adjacency/connected-components/🔣️component.json"),
                proto: include_str!("🔗️adjacency/connected-components/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.main-axis-direction.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧭️orientation/main-axis-direction/🦀️component.rs"),
                typescript: include_str!("🧭️orientation/main-axis-direction/🟦️component.ts"),
                graphql: include_str!("🧭️orientation/main-axis-direction/🔗️component.graphql"),
                json_schema: include_str!("🧭️orientation/main-axis-direction/🔣️component.json"),
                proto: include_str!("🧭️orientation/main-axis-direction/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.face-normal-distribution.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧭️orientation/face-normal-distribution/🦀️component.rs"),
                typescript: include_str!("🧭️orientation/face-normal-distribution/🟦️component.ts"),
                graphql: include_str!("🧭️orientation/face-normal-distribution/🔗️component.graphql"),
                json_schema: include_str!("🧭️orientation/face-normal-distribution/🔣️component.json"),
                proto: include_str!("🧭️orientation/face-normal-distribution/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.orientation-consistency.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🧭️orientation/orientation-consistency/🦀️component.rs"),
                typescript: include_str!("🧭️orientation/orientation-consistency/🟦️component.ts"),
                graphql: include_str!("🧭️orientation/orientation-consistency/🔗️component.graphql"),
                json_schema: include_str!("🧭️orientation/orientation-consistency/🔣️component.json"),
                proto: include_str!("🧭️orientation/orientation-consistency/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reflection-symmetry-score.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/reflection-symmetry-score/🦀️component.rs"),
                typescript: include_str!("🪞️symmetry/reflection-symmetry-score/🟦️component.ts"),
                graphql: include_str!("🪞️symmetry/reflection-symmetry-score/🔗️component.graphql"),
                json_schema: include_str!("🪞️symmetry/reflection-symmetry-score/🔣️component.json"),
                proto: include_str!("🪞️symmetry/reflection-symmetry-score/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.rotational-symmetry-score.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/rotational-symmetry-score/🦀️component.rs"),
                typescript: include_str!("🪞️symmetry/rotational-symmetry-score/🟦️component.ts"),
                graphql: include_str!("🪞️symmetry/rotational-symmetry-score/🔗️component.graphql"),
                json_schema: include_str!("🪞️symmetry/rotational-symmetry-score/🔣️component.json"),
                proto: include_str!("🪞️symmetry/rotational-symmetry-score/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.reflection-symmetries.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/reflection-symmetries/🦀️component.rs"),
                typescript: include_str!("🪞️symmetry/reflection-symmetries/🟦️component.ts"),
                graphql: include_str!("🪞️symmetry/reflection-symmetries/🔗️component.graphql"),
                json_schema: include_str!("🪞️symmetry/reflection-symmetries/🔣️component.json"),
                proto: include_str!("🪞️symmetry/reflection-symmetries/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.rotational-symmetries.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/rotational-symmetries/🦀️component.rs"),
                typescript: include_str!("🪞️symmetry/rotational-symmetries/🟦️component.ts"),
                graphql: include_str!("🪞️symmetry/rotational-symmetries/🔗️component.graphql"),
                json_schema: include_str!("🪞️symmetry/rotational-symmetries/🔣️component.json"),
                proto: include_str!("🪞️symmetry/rotational-symmetries/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.repetition-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/repetition-ratio/🦀️component.rs"),
                typescript: include_str!("🪞️symmetry/repetition-ratio/🟦️component.ts"),
                graphql: include_str!("🪞️symmetry/repetition-ratio/🔗️component.graphql"),
                json_schema: include_str!("🪞️symmetry/repetition-ratio/🔣️component.json"),
                proto: include_str!("🪞️symmetry/repetition-ratio/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.modularity-ratio.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🪞️symmetry/modularity-ratio/🦀️component.rs"),
                typescript: include_str!("🪞️symmetry/modularity-ratio/🟦️component.ts"),
                graphql: include_str!("🪞️symmetry/modularity-ratio/🔗️component.graphql"),
                json_schema: include_str!("🪞️symmetry/modularity-ratio/🔣️component.json"),
                proto: include_str!("🪞️symmetry/modularity-ratio/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.deviation-from-ideal.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/deviation-from-ideal/🦀️component.rs"),
                typescript: include_str!("🌊️roughness/deviation-from-ideal/🟦️component.ts"),
                graphql: include_str!("🌊️roughness/deviation-from-ideal/🔗️component.graphql"),
                json_schema: include_str!("🌊️roughness/deviation-from-ideal/🔣️component.json"),
                proto: include_str!("🌊️roughness/deviation-from-ideal/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/deviation-from-smoothed-geometry/🦀️component.rs"),
                typescript: include_str!("🌊️roughness/deviation-from-smoothed-geometry/🟦️component.ts"),
                graphql: include_str!("🌊️roughness/deviation-from-smoothed-geometry/🔗️component.graphql"),
                json_schema: include_str!("🌊️roughness/deviation-from-smoothed-geometry/🔣️component.json"),
                proto: include_str!("🌊️roughness/deviation-from-smoothed-geometry/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.normal-variation.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/normal-variation/🦀️component.rs"),
                typescript: include_str!("🌊️roughness/normal-variation/🟦️component.ts"),
                graphql: include_str!("🌊️roughness/normal-variation/🔗️component.graphql"),
                json_schema: include_str!("🌊️roughness/normal-variation/🔣️component.json"),
                proto: include_str!("🌊️roughness/normal-variation/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.surface-waviness.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/surface-waviness/🦀️component.rs"),
                typescript: include_str!("🌊️roughness/surface-waviness/🟦️component.ts"),
                graphql: include_str!("🌊️roughness/surface-waviness/🔗️component.graphql"),
                json_schema: include_str!("🌊️roughness/surface-waviness/🔣️component.json"),
                proto: include_str!("🌊️roughness/surface-waviness/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.irregularity.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🌊️roughness/irregularity/🦀️component.rs"),
                typescript: include_str!("🌊️roughness/irregularity/🟦️component.ts"),
                graphql: include_str!("🌊️roughness/irregularity/🔗️component.graphql"),
                json_schema: include_str!("🌊️roughness/irregularity/🔣️component.json"),
                proto: include_str!("🌊️roughness/irregularity/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.holes.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/holes/🦀️component.rs"),
                typescript: include_str!("🕸️topology/holes/🟦️component.ts"),
                graphql: include_str!("🕸️topology/holes/🔗️component.graphql"),
                json_schema: include_str!("🕸️topology/holes/🔣️component.json"),
                proto: include_str!("🕸️topology/holes/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.handles.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/handles/🦀️component.rs"),
                typescript: include_str!("🕸️topology/handles/🟦️component.ts"),
                graphql: include_str!("🕸️topology/handles/🔗️component.graphql"),
                json_schema: include_str!("🕸️topology/handles/🔣️component.json"),
                proto: include_str!("🕸️topology/handles/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.boundary-loops.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/boundary-loops/🦀️component.rs"),
                typescript: include_str!("🕸️topology/boundary-loops/🟦️component.ts"),
                graphql: include_str!("🕸️topology/boundary-loops/🔗️component.graphql"),
                json_schema: include_str!("🕸️topology/boundary-loops/🔣️component.json"),
                proto: include_str!("🕸️topology/boundary-loops/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.euler-characteristic.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/euler-characteristic/🦀️component.rs"),
                typescript: include_str!("🕸️topology/euler-characteristic/🟦️component.ts"),
                graphql: include_str!("🕸️topology/euler-characteristic/🔗️component.graphql"),
                json_schema: include_str!("🕸️topology/euler-characteristic/🔣️component.json"),
                proto: include_str!("🕸️topology/euler-characteristic/🛰️component.proto"),
            },
        },
        schema::ArtifactInferenceDescriptor {
            id: "s.stdio.gltf.inference.genus.v1",
            inference: schema::FacetLeaves {
                rust: include_str!("🕸️topology/genus/🦀️component.rs"),
                typescript: include_str!("🕸️topology/genus/🟦️component.ts"),
                graphql: include_str!("🕸️topology/genus/🔗️component.graphql"),
                json_schema: include_str!("🕸️topology/genus/🔣️component.json"),
                proto: include_str!("🕸️topology/genus/🛰️component.proto"),
            },
        },
    ]
}
//#endregion 🧠️InferenceContract
//#region 🧪️ParityTests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
