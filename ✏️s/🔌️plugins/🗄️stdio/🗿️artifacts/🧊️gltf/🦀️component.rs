//! 🎪 `stdio.gltf` artifact — stdio reference format.

use semio_framework_plugin::{
    ArtifactInference, ArtifactInferenceExecution, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferenceService, ArtifactInferenceServiceMetadata, ArtifactInferrer, ArtifactKindSpec, MediaClass, MediaForm, MediaType,
    OsMediaCapability,
};

pub use crate::artifacts::gltf::schema::diff::GltfDiff;
pub use crate::artifacts::gltf::schema::modules::mutation_dispatch::{GltfDiffEnvelope, GltfMutation, GltfMutationDiff, GltfMutationEnvelope, GltfMutationPhase, GltfMutationRegistryError};
pub use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
pub use crate::artifacts::gltf::schema::GltfArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GLTF_DOCUMENT_SCHEMA: &str = "stdio.gltf";

/// 🧬️ Artifact schema descriptor id.
pub const GLTF_ARTIFACT_KIND_ID: &str = "s.stdio.gltf";
pub const GLTF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gltf";
pub const GLTF_ARTIFACT_SCHEMA_VERSION: u32 = 1;
pub const GLTF_DOCUMENT_SCHEMA_VERSION: u32 = 2;
pub const GLTF_INFERENCE_SCHEMA_ID: &str = "s.stdio.gltf.inference";
pub const GLTF_INFERENCE_SCHEMA_VERSION: u32 = 2;
pub const GLTF_INFERENCE_ALGORITHM_VERSION: u32 = 1;
pub const GLTF_INFERENCE_POLICY_VERSION: u32 = 1;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::artifacts::gltf::engine::register()`, which the plugin
/// root used to call unconditionally before `Plugin::builder(...)` was even constructed. Mirrors
/// `🗜️deflate`'s own `s.stdio.deflate` exemplar exactly: a headless library artifact with zero
/// `ArtifactApp`s, so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::register_document_codec(store::ArtifactCodec::of::<GltfSnapshot, GltfMutation>(...))`.
/// `.composers(...)` reaches the ENGINE's own `io_registry` (returns `&'static [ComposerEntry]`,
/// owned rows) by its full path through the `engine` shim (`📦️glue.rs`'s `pub mod engine { pub use
/// super::standards::v2_0::engine::*; }`) — deliberately NOT this file's own `io_registry` module
/// below, whose `entries()` returns `&'static [&'static ComposerEntry]` (references) and would
/// silently rebind under a bare call (this ticket's "SILENT REBIND" hazard). gltf's own
/// `register()` had no `register_schema_specs()` call, so every registration `engine::register()`
/// performed is covered by a declaration field — no `.setup()` survivor needed.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("gltf", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("gltf")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::gltf::schema::gltf_artifact_schema_descriptor())
        .formats(formats)
        .inferences(crate::artifacts::gltf::schema::inferences::gltf_artifact_inference_descriptors())
        .inference_services(gltf_inference_services())
        .composers(crate::artifacts::gltf::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<GltfSnapshot, GltfMutation>(STDIO_GLTF_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🧠️ Independently executable glTF inference leaves.
pub fn gltf_inference_services() -> Vec<ArtifactInferenceService> {
    vec![
        gltf_inference_leaf_service("s.stdio.gltf.inference.overall-size.v1", infer_gltf_leaf_overall_size),
        gltf_inference_leaf_service("s.stdio.gltf.inference.axis-aligned-bounds.v1", infer_gltf_leaf_axis_aligned_bounds),
        gltf_inference_leaf_service("s.stdio.gltf.inference.oriented-bounds.v1", infer_gltf_leaf_oriented_bounds),
        gltf_inference_leaf_service("s.stdio.gltf.inference.bounding-box-dimensions.v1", infer_gltf_leaf_bounding_box_dimensions),
        gltf_inference_leaf_service("s.stdio.gltf.inference.characteristic-length.v1", infer_gltf_leaf_characteristic_length),
        gltf_inference_leaf_service("s.stdio.gltf.inference.footprint-area.v1", infer_gltf_leaf_footprint_area),
        gltf_inference_leaf_service("s.stdio.gltf.inference.projected-area.v1", infer_gltf_leaf_projected_area),
        gltf_inference_leaf_service("s.stdio.gltf.inference.surface-area.v1", infer_gltf_leaf_surface_area),
        gltf_inference_leaf_service("s.stdio.gltf.inference.total-area.v1", infer_gltf_leaf_total_area),
        gltf_inference_leaf_service("s.stdio.gltf.inference.exposed-area.v1", infer_gltf_leaf_exposed_area),
        gltf_inference_leaf_service("s.stdio.gltf.inference.contact-area.v1", infer_gltf_leaf_contact_area),
        gltf_inference_leaf_service("s.stdio.gltf.inference.volume.v1", infer_gltf_leaf_volume),
        gltf_inference_leaf_service("s.stdio.gltf.inference.enclosed-volume.v1", infer_gltf_leaf_enclosed_volume),
        gltf_inference_leaf_service("s.stdio.gltf.inference.material-volume.v1", infer_gltf_leaf_material_volume),
        gltf_inference_leaf_service("s.stdio.gltf.inference.void-volume.v1", infer_gltf_leaf_void_volume),
        gltf_inference_leaf_service("s.stdio.gltf.inference.compactness.v1", infer_gltf_leaf_compactness),
        gltf_inference_leaf_service("s.stdio.gltf.inference.surface-to-volume-ratio.v1", infer_gltf_leaf_surface_to_volume_ratio),
        gltf_inference_leaf_service("s.stdio.gltf.inference.sphericity.v1", infer_gltf_leaf_sphericity),
        gltf_inference_leaf_service("s.stdio.gltf.inference.compactness-index.v1", infer_gltf_leaf_compactness_index),
        gltf_inference_leaf_service("s.stdio.gltf.inference.hull-fill-ratio.v1", infer_gltf_leaf_hull_fill_ratio),
        gltf_inference_leaf_service("s.stdio.gltf.inference.aspect-ratios.v1", infer_gltf_leaf_aspect_ratios),
        gltf_inference_leaf_service("s.stdio.gltf.inference.slenderness.v1", infer_gltf_leaf_slenderness),
        gltf_inference_leaf_service("s.stdio.gltf.inference.flatness.v1", infer_gltf_leaf_flatness),
        gltf_inference_leaf_service("s.stdio.gltf.inference.elongation.v1", infer_gltf_leaf_elongation),
        gltf_inference_leaf_service("s.stdio.gltf.inference.centroid.v1", infer_gltf_leaf_centroid),
        gltf_inference_leaf_service("s.stdio.gltf.inference.principal-frame.v1", infer_gltf_leaf_principal_frame),
        gltf_inference_leaf_service("s.stdio.gltf.inference.principal-axes.v1", infer_gltf_leaf_principal_axes),
        gltf_inference_leaf_service("s.stdio.gltf.inference.moments-of-inertia.v1", infer_gltf_leaf_moments_of_inertia),
        gltf_inference_leaf_service("s.stdio.gltf.inference.inertia-tensor.v1", infer_gltf_leaf_inertia_tensor),
        gltf_inference_leaf_service("s.stdio.gltf.inference.mean-curvature.v1", infer_gltf_leaf_mean_curvature),
        gltf_inference_leaf_service("s.stdio.gltf.inference.gaussian-curvature.v1", infer_gltf_leaf_gaussian_curvature),
        gltf_inference_leaf_service("s.stdio.gltf.inference.curvature-histogram.v1", infer_gltf_leaf_curvature_histogram),
        gltf_inference_leaf_service("s.stdio.gltf.inference.sharp-feature-proportion.v1", infer_gltf_leaf_sharp_feature_proportion),
        gltf_inference_leaf_service("s.stdio.gltf.inference.mean-thickness.v1", infer_gltf_leaf_mean_thickness),
        gltf_inference_leaf_service("s.stdio.gltf.inference.minimum-thickness.v1", infer_gltf_leaf_minimum_thickness),
        gltf_inference_leaf_service("s.stdio.gltf.inference.thickness-variability.v1", infer_gltf_leaf_thickness_variability),
        gltf_inference_leaf_service("s.stdio.gltf.inference.thickness-distribution.v1", infer_gltf_leaf_thickness_distribution),
        gltf_inference_leaf_service("s.stdio.gltf.inference.convex-hull-gap.v1", infer_gltf_leaf_convex_hull_gap),
        gltf_inference_leaf_service("s.stdio.gltf.inference.reentrant-area.v1", infer_gltf_leaf_reentrant_area),
        gltf_inference_leaf_service("s.stdio.gltf.inference.reentrant-volume.v1", infer_gltf_leaf_reentrant_volume),
        gltf_inference_leaf_service("s.stdio.gltf.inference.concavity-index.v1", infer_gltf_leaf_concavity_index),
        gltf_inference_leaf_service("s.stdio.gltf.inference.minimum-distance-to-neighbors.v1", infer_gltf_leaf_minimum_distance_to_neighbors),
        gltf_inference_leaf_service("s.stdio.gltf.inference.clearance-distribution.v1", infer_gltf_leaf_clearance_distribution),
        gltf_inference_leaf_service("s.stdio.gltf.inference.interference-volume.v1", infer_gltf_leaf_interference_volume),
        gltf_inference_leaf_service("s.stdio.gltf.inference.overlap-volume.v1", infer_gltf_leaf_overlap_volume),
        gltf_inference_leaf_service("s.stdio.gltf.inference.number-of-contacts.v1", infer_gltf_leaf_number_of_contacts),
        gltf_inference_leaf_service("s.stdio.gltf.inference.contact-graph-degree.v1", infer_gltf_leaf_contact_graph_degree),
        gltf_inference_leaf_service("s.stdio.gltf.inference.connected-components.v1", infer_gltf_leaf_connected_components),
        gltf_inference_leaf_service("s.stdio.gltf.inference.main-axis-direction.v1", infer_gltf_leaf_main_axis_direction),
        gltf_inference_leaf_service("s.stdio.gltf.inference.face-normal-distribution.v1", infer_gltf_leaf_face_normal_distribution),
        gltf_inference_leaf_service("s.stdio.gltf.inference.orientation-consistency.v1", infer_gltf_leaf_orientation_consistency),
        gltf_inference_leaf_service("s.stdio.gltf.inference.reflection-symmetry-score.v1", infer_gltf_leaf_reflection_symmetry_score),
        gltf_inference_leaf_service("s.stdio.gltf.inference.rotational-symmetry-score.v1", infer_gltf_leaf_rotational_symmetry_score),
        gltf_inference_leaf_service("s.stdio.gltf.inference.reflection-symmetries.v1", infer_gltf_leaf_reflection_symmetries),
        gltf_inference_leaf_service("s.stdio.gltf.inference.rotational-symmetries.v1", infer_gltf_leaf_rotational_symmetries),
        gltf_inference_leaf_service("s.stdio.gltf.inference.repetition-ratio.v1", infer_gltf_leaf_repetition_ratio),
        gltf_inference_leaf_service("s.stdio.gltf.inference.modularity-ratio.v1", infer_gltf_leaf_modularity_ratio),
        gltf_inference_leaf_service("s.stdio.gltf.inference.deviation-from-ideal.v1", infer_gltf_leaf_deviation_from_ideal),
        gltf_inference_leaf_service("s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1", infer_gltf_leaf_deviation_from_smoothed_geometry),
        gltf_inference_leaf_service("s.stdio.gltf.inference.normal-variation.v1", infer_gltf_leaf_normal_variation),
        gltf_inference_leaf_service("s.stdio.gltf.inference.surface-waviness.v1", infer_gltf_leaf_surface_waviness),
        gltf_inference_leaf_service("s.stdio.gltf.inference.irregularity.v1", infer_gltf_leaf_irregularity),
        gltf_inference_leaf_service("s.stdio.gltf.inference.holes.v1", infer_gltf_leaf_holes),
        gltf_inference_leaf_service("s.stdio.gltf.inference.handles.v1", infer_gltf_leaf_handles),
        gltf_inference_leaf_service("s.stdio.gltf.inference.boundary-loops.v1", infer_gltf_leaf_boundary_loops),
        gltf_inference_leaf_service("s.stdio.gltf.inference.euler-characteristic.v1", infer_gltf_leaf_euler_characteristic),
        gltf_inference_leaf_service("s.stdio.gltf.inference.genus.v1", infer_gltf_leaf_genus),
    ]
}

fn gltf_inference_leaf_service(inference_schema: &'static str, infer: ArtifactInference) -> ArtifactInferenceService {
    ArtifactInferenceService::new(
        ArtifactInferenceServiceMetadata {
            owner: "stdio",
            artifact_kind: GLTF_ARTIFACT_KIND_ID,
            artifact_schema: GLTF_ARTIFACT_SCHEMA_ID,
            artifact_schema_version: GLTF_ARTIFACT_SCHEMA_VERSION,
            document_schema: STDIO_GLTF_DOCUMENT_SCHEMA,
            document_schema_version: GLTF_DOCUMENT_SCHEMA_VERSION,
            inference_schema,
            inference_schema_version: 1,
            algorithm_version: GLTF_INFERENCE_ALGORITHM_VERSION,
            policy_version: GLTF_INFERENCE_POLICY_VERSION,
        },
        infer,
    )
}

fn infer_gltf_leaf_cold(id: &'static str, request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let descriptor = crate::artifacts::gltf::schema::inferences::gltf_inference_leaf_service_descriptor(id).ok_or_else(|| ArtifactInferenceExecutionError::new("stdio.gltf.inference.unknown-leaf", id))?;
    let snapshot = <GltfSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload).map_err(|error| ArtifactInferenceExecutionError::new("stdio.gltf.inference.snapshot-decode", error.to_string()))?;
    let assembly = <crate::artifacts::gltf::schema::GltfBuilder as ArtifactInferrer>::infer(&snapshot);
    let value = (descriptor.encode)(&assembly.geometry.overall).map_err(|error| ArtifactInferenceExecutionError::new("stdio.gltf.inference.leaf-json", error.to_string()))?;
    let policy_hash = format!("{:016x}", stable_hash(request.policy));
    let dependency_hashes = request.dependencies.iter().map(|(name, bytes)| format!("{name}:{:016x}", stable_hash(bytes))).collect::<Vec<_>>();
    let diagnostic_ids = value.get("diagnosticIds").and_then(serde_json::Value::as_array).map(|ids| ids.iter().filter_map(serde_json::Value::as_str).map(str::to_owned).collect()).unwrap_or_default();
    let provenance = value.get("provenance").map(|provenance| provenance.to_string()).into_iter().collect();
    let quality = value.get("quality").map_or_else(|| "unknown".into(), serde_json::Value::to_string);
    let validity = value.get("validity").and_then(serde_json::Value::as_str).unwrap_or("indeterminate").to_owned();
    let envelope = crate::artifacts::gltf::io::inferences::text::GltfInferenceLeafEnvelope {
        id: id.into(),
        algorithm_version: descriptor.algorithm_version,
        policy_hash: policy_hash.clone(),
        dependency_hashes: dependency_hashes.clone(),
        cache_key: format!("{}:p{policy_hash}:d{:016x}", descriptor.cache_key, stable_hash(dependency_hashes.join("|").as_bytes())),
        validity: validity.clone(),
        quality,
        diagnostic_ids,
        provenance,
        value,
    };
    let canonical_payload = crate::artifacts::gltf::io::inferences::binary::encode_gltf_inference_leaf_binary(&envelope).map_err(|error| ArtifactInferenceExecutionError::new("stdio.gltf.inference.leaf-binary-encode", error.to_string()))?;
    Ok(ArtifactInferenceExecution { canonical_payload, diagnostics: Vec::new(), validity, quality: envelope.quality.clone(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
}

fn stable_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3))
}

fn infer_gltf_leaf_overall_size(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.overall-size.v1", request)
}

fn infer_gltf_leaf_axis_aligned_bounds(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.axis-aligned-bounds.v1", request)
}

fn infer_gltf_leaf_oriented_bounds(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.oriented-bounds.v1", request)
}

fn infer_gltf_leaf_bounding_box_dimensions(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.bounding-box-dimensions.v1", request)
}

fn infer_gltf_leaf_characteristic_length(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.characteristic-length.v1", request)
}

fn infer_gltf_leaf_footprint_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.footprint-area.v1", request)
}

fn infer_gltf_leaf_projected_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.projected-area.v1", request)
}

fn infer_gltf_leaf_surface_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.surface-area.v1", request)
}

fn infer_gltf_leaf_total_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.total-area.v1", request)
}

fn infer_gltf_leaf_exposed_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.exposed-area.v1", request)
}

fn infer_gltf_leaf_contact_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.contact-area.v1", request)
}

fn infer_gltf_leaf_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.volume.v1", request)
}

fn infer_gltf_leaf_enclosed_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.enclosed-volume.v1", request)
}

fn infer_gltf_leaf_material_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.material-volume.v1", request)
}

fn infer_gltf_leaf_void_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.void-volume.v1", request)
}

fn infer_gltf_leaf_compactness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.compactness.v1", request)
}

fn infer_gltf_leaf_surface_to_volume_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.surface-to-volume-ratio.v1", request)
}

fn infer_gltf_leaf_sphericity(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.sphericity.v1", request)
}

fn infer_gltf_leaf_compactness_index(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.compactness-index.v1", request)
}

fn infer_gltf_leaf_hull_fill_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.hull-fill-ratio.v1", request)
}

fn infer_gltf_leaf_aspect_ratios(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.aspect-ratios.v1", request)
}

fn infer_gltf_leaf_slenderness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.slenderness.v1", request)
}

fn infer_gltf_leaf_flatness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.flatness.v1", request)
}

fn infer_gltf_leaf_elongation(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.elongation.v1", request)
}

fn infer_gltf_leaf_centroid(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.centroid.v1", request)
}

fn infer_gltf_leaf_principal_frame(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.principal-frame.v1", request)
}

fn infer_gltf_leaf_principal_axes(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.principal-axes.v1", request)
}

fn infer_gltf_leaf_moments_of_inertia(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.moments-of-inertia.v1", request)
}

fn infer_gltf_leaf_inertia_tensor(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.inertia-tensor.v1", request)
}

fn infer_gltf_leaf_mean_curvature(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.mean-curvature.v1", request)
}

fn infer_gltf_leaf_gaussian_curvature(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.gaussian-curvature.v1", request)
}

fn infer_gltf_leaf_curvature_histogram(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.curvature-histogram.v1", request)
}

fn infer_gltf_leaf_sharp_feature_proportion(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.sharp-feature-proportion.v1", request)
}

fn infer_gltf_leaf_mean_thickness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.mean-thickness.v1", request)
}

fn infer_gltf_leaf_minimum_thickness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.minimum-thickness.v1", request)
}

fn infer_gltf_leaf_thickness_variability(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.thickness-variability.v1", request)
}

fn infer_gltf_leaf_thickness_distribution(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.thickness-distribution.v1", request)
}

fn infer_gltf_leaf_convex_hull_gap(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.convex-hull-gap.v1", request)
}

fn infer_gltf_leaf_reentrant_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reentrant-area.v1", request)
}

fn infer_gltf_leaf_reentrant_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reentrant-volume.v1", request)
}

fn infer_gltf_leaf_concavity_index(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.concavity-index.v1", request)
}

fn infer_gltf_leaf_minimum_distance_to_neighbors(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.minimum-distance-to-neighbors.v1", request)
}

fn infer_gltf_leaf_clearance_distribution(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.clearance-distribution.v1", request)
}

fn infer_gltf_leaf_interference_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.interference-volume.v1", request)
}

fn infer_gltf_leaf_overlap_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.overlap-volume.v1", request)
}

fn infer_gltf_leaf_number_of_contacts(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.number-of-contacts.v1", request)
}

fn infer_gltf_leaf_contact_graph_degree(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.contact-graph-degree.v1", request)
}

fn infer_gltf_leaf_connected_components(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.connected-components.v1", request)
}

fn infer_gltf_leaf_main_axis_direction(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.main-axis-direction.v1", request)
}

fn infer_gltf_leaf_face_normal_distribution(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.face-normal-distribution.v1", request)
}

fn infer_gltf_leaf_orientation_consistency(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.orientation-consistency.v1", request)
}

fn infer_gltf_leaf_reflection_symmetry_score(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reflection-symmetry-score.v1", request)
}

fn infer_gltf_leaf_rotational_symmetry_score(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.rotational-symmetry-score.v1", request)
}

fn infer_gltf_leaf_reflection_symmetries(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reflection-symmetries.v1", request)
}

fn infer_gltf_leaf_rotational_symmetries(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.rotational-symmetries.v1", request)
}

fn infer_gltf_leaf_repetition_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.repetition-ratio.v1", request)
}

fn infer_gltf_leaf_modularity_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.modularity-ratio.v1", request)
}

fn infer_gltf_leaf_deviation_from_ideal(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.deviation-from-ideal.v1", request)
}

fn infer_gltf_leaf_deviation_from_smoothed_geometry(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1", request)
}

fn infer_gltf_leaf_normal_variation(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.normal-variation.v1", request)
}

fn infer_gltf_leaf_surface_waviness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.surface-waviness.v1", request)
}

fn infer_gltf_leaf_irregularity(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.irregularity.v1", request)
}

fn infer_gltf_leaf_holes(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.holes.v1", request)
}

fn infer_gltf_leaf_handles(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.handles.v1", request)
}

fn infer_gltf_leaf_boundary_loops(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.boundary-loops.v1", request)
}

fn infer_gltf_leaf_euler_characteristic(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.euler-characteristic.v1", request)
}

fn infer_gltf_leaf_genus(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.genus.v1", request)
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::artifacts::gltf::standards::
/// v2_0::engine::register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.gltf",
                    extension: Some("gltf"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::gltf::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gltf::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gltf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gltf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf"),
                },
                dsl::LanguageSpec {
                    id: "stdio.gltf.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::gltf::io::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gltf::io::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gltf::io::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gltf::io::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.gltf.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::gltf::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gltf::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.gltf.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.gltf.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gltf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gltf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.gltf.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gltf::io::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gltf::io::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: GLTF_ARTIFACT_KIND_ID.into(),
        name: "Gltf".into(),
        source_format: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::gltf::standards::v2_0::engine::io_registry as v2_0;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v2_0::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("GltfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v2_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ArtifactInferenceExecutionRequest, ArtifactInferenceServiceRegistry, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

    #[test]
    fn all_canonical_leaf_services_are_independently_registered() {
        let services = gltf_inference_services();
        let ids = services.iter().map(|service| service.metadata().inference_schema).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(services.len(), 67);
        assert_eq!(ids.len(), 67);
        assert!(ids.contains("s.stdio.gltf.inference.overall-size.v1"));
        let mut registry = ArtifactInferenceServiceRegistry::new();
        for service in services {
            registry.register(service).unwrap();
        }
    }

    #[test]
    fn one_leaf_service_returns_its_id_bound_generic_envelope() {
        let snapshot_pack = <GltfSnapshot as store::ArtifactPack>::encode_pack(&GltfSnapshot::default());
        let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 1 };
        let dependencies = vec![("snapshot".into(), snapshot_pack.clone())];
        let request = ArtifactInferenceExecutionRequest {
            policy: b"gltf-test",
            budgets: &budgets,
            cancellation_id: "gltf-leaf",
            previous_state: None,
            requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
            canonical_payload: &snapshot_pack,
            dependencies: &dependencies,
        };
        let service = gltf_inference_services().into_iter().find(|service| service.metadata().inference_schema == "s.stdio.gltf.inference.overall-size.v1").unwrap();
        let execution = service.infer(&request).unwrap();
        let envelope = crate::artifacts::gltf::io::inferences::binary::decode_gltf_inference_leaf_binary(&execution.canonical_payload).unwrap();
        assert_eq!(envelope.id, "s.stdio.gltf.inference.overall-size.v1");
        assert!(envelope.cache_key.contains(":p"));
        assert_eq!(envelope.dependency_hashes.len(), 1);
    }
}
