//! 🎪 `stdio.gltf` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{
    ArtifactInference, ArtifactInferenceExecution, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferenceService, ArtifactInferenceServiceMetadata, ArtifactInferrer, ArtifactKindSpec, MediaClass, MediaForm, MediaType,
    OsMediaCapability,
};

pub use schema::diff::GltfDiff;
pub use schema::mutations::GltfMutation;
pub use schema::snapshot::GltfSnapshot;
pub use schema::GltfArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GLTF_DOCUMENT_SCHEMA: &str = "stdio.gltf";

/// 🧬️ Artifact schema descriptor id.
pub const GLTF_ARTIFACT_KIND_ID: &str = "s.stdio.gltf";
pub const GLTF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gltf";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("🧬️schema/📜️artifact-definition.json");
pub const GLTF_ARTIFACT_SCHEMA_VERSION: u32 = 1;
pub const GLTF_DOCUMENT_SCHEMA_VERSION: u32 = 2;
pub const GLTF_INFERENCE_SCHEMA_ID: &str = "s.stdio.gltf.inference";
pub const GLTF_INFERENCE_SCHEMA_VERSION: u32 = 2;
pub const GLTF_INFERENCE_ALGORITHM_VERSION: u32 = 1;
pub const GLTF_INFERENCE_POLICY_VERSION: u32 = 1;

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let mut executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    executables.extend(gltf_inference_services().into_iter().map(|service| semio_s_artifact_stdio_contract::ArtifactExecutable {
        identity: service.metadata().inference_schema.to_owned(),
        executable: service.executable_identity(),
    }));
    executables.extend([
        "s.stdio.gltf.mutation.change-material-alpha-mode.v1",
        "s.stdio.gltf.mutation.change-material-double-sided.v1",
        "s.stdio.gltf.mutation.create-scene.v1",
    ].into_iter().map(|identity| semio_s_artifact_stdio_contract::ArtifactExecutable::from_function_pointer(identity, schema::mutations::apply_gltf_mutation as *const ())));
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

fn native_codec() -> store::ArtifactCodec {
    let mut codec = store::ArtifactCodec::of::<GltfSnapshot, GltfMutation>(STDIO_GLTF_DOCUMENT_SCHEMA);
    codec.extension = "gltf";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.gltf.v1", artifact: "gltf", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution {
        identity: "gltf",
        schema: ARTIFACT_DEFINITION_SCHEMA,
        definition,
        assembly,
        formats,
        native_codecs,
    }
}

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::engine::register()`, which the plugin
/// root used to call unconditionally before `Plugin::builder(...)` was even constructed. Mirrors
/// `🗜️deflate`'s own `s.stdio.deflate` exemplar exactly: a headless library artifact with zero
/// `ArtifactApp`s, so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::register_document_codec(store::ArtifactCodec::of::<GltfSnapshot, GltfMutation>(...))`.
/// `.composers(...)` reaches the ENGINE's own `io_registry` (returns `&'static [ComposerEntry]`,
/// owned rows) by its full path through the `engine` shim (`🦀️.rs`'s `pub mod engine { pub use
/// super::standards::v2_0::engine::*; }`) — deliberately NOT this file's own `io_registry` module
/// below, whose `entries()` returns `&'static [&'static ComposerEntry]` (references) and would
/// silently rebind under a bare call (this ticket's "SILENT REBIND" hazard). gltf's own
/// `register()` had no `register_schema_specs()` call, so every registration `engine::register()`
/// performed is covered by a declaration field — no `.setup()` survivor needed.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("gltf", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(schema::gltf_artifact_schema_descriptor())
        .formats(formats)
        .inferences(schema::inferences::gltf_artifact_inference_descriptors())
        .inference_services(gltf_inference_services())
        .composers(engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<GltfSnapshot, GltfMutation>(STDIO_GLTF_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🧠️ Independently executable glTF inference leaves.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_cold(id: &'static str, request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let descriptor = schema::inferences::gltf_inference_leaf_service_descriptor(id).ok_or_else(|| ArtifactInferenceExecutionError::new("stdio.gltf.inference.unknown-leaf", id))?;
    let snapshot = <GltfSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload).map_err(|error| ArtifactInferenceExecutionError::new("stdio.gltf.inference.snapshot-decode", error.to_string()))?;
    let assembly = <schema::GltfBuilder as ArtifactInferrer>::infer(&snapshot);
    let value = (descriptor.encode)(&assembly.geometry.overall);
    let policy_hash = format!("{:016x}", stable_hash(request.policy));
    let dependency_hashes = request.dependencies.iter().map(|(name, bytes)| format!("{name}:{:016x}", stable_hash(bytes))).collect::<Vec<_>>();
    let diagnostic_ids = value.get("diagnosticIds").and_then(dsl::DslValue::as_array).map(|ids| ids.iter().filter_map(dsl::DslValue::as_str).map(str::to_owned).collect()).unwrap_or_default();
    let provenance = value.get("provenance").map(|provenance| pack::json_to_string(&pack::json_from_dsl_value(provenance))).into_iter().collect();
    let quality = value.get("quality").map_or_else(|| "unknown".into(), |quality| pack::json_to_string(&pack::json_from_dsl_value(quality)));
    let validity = value.get("validity").and_then(dsl::DslValue::as_str).unwrap_or("indeterminate").to_owned();
    let envelope = io::inferences::text::GltfInferenceLeafEnvelope {
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
    let canonical_payload = io::inferences::binary::encode_gltf_inference_leaf_binary(&envelope).map_err(|error| ArtifactInferenceExecutionError::new("stdio.gltf.inference.leaf-binary-encode", error.to_string()))?;
    Ok(ArtifactInferenceExecution { canonical_payload, diagnostics: Vec::new(), validity, quality: envelope.quality, complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stable_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_overall_size(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.overall-size.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_axis_aligned_bounds(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.axis-aligned-bounds.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_oriented_bounds(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.oriented-bounds.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_bounding_box_dimensions(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.bounding-box-dimensions.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_characteristic_length(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.characteristic-length.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_footprint_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.footprint-area.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_projected_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.projected-area.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_surface_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.surface-area.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_total_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.total-area.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_exposed_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.exposed-area.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_contact_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.contact-area.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.volume.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_enclosed_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.enclosed-volume.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_material_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.material-volume.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_void_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.void-volume.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_compactness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.compactness.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_surface_to_volume_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.surface-to-volume-ratio.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_sphericity(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.sphericity.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_compactness_index(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.compactness-index.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_hull_fill_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.hull-fill-ratio.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_aspect_ratios(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.aspect-ratios.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_slenderness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.slenderness.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_flatness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.flatness.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_elongation(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.elongation.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_centroid(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.centroid.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_principal_frame(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.principal-frame.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_principal_axes(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.principal-axes.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_moments_of_inertia(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.moments-of-inertia.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_inertia_tensor(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.inertia-tensor.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_mean_curvature(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.mean-curvature.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_gaussian_curvature(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.gaussian-curvature.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_curvature_histogram(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.curvature-histogram.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_sharp_feature_proportion(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.sharp-feature-proportion.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_mean_thickness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.mean-thickness.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_minimum_thickness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.minimum-thickness.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_thickness_variability(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.thickness-variability.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_thickness_distribution(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.thickness-distribution.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_convex_hull_gap(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.convex-hull-gap.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_reentrant_area(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reentrant-area.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_reentrant_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reentrant-volume.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_concavity_index(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.concavity-index.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_minimum_distance_to_neighbors(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.minimum-distance-to-neighbors.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_clearance_distribution(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.clearance-distribution.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_interference_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.interference-volume.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_overlap_volume(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.overlap-volume.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_number_of_contacts(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.number-of-contacts.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_contact_graph_degree(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.contact-graph-degree.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_connected_components(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.connected-components.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_main_axis_direction(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.main-axis-direction.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_face_normal_distribution(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.face-normal-distribution.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_orientation_consistency(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.orientation-consistency.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_reflection_symmetry_score(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reflection-symmetry-score.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_rotational_symmetry_score(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.rotational-symmetry-score.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_reflection_symmetries(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.reflection-symmetries.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_rotational_symmetries(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.rotational-symmetries.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_repetition_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.repetition-ratio.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_modularity_ratio(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.modularity-ratio.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_deviation_from_ideal(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.deviation-from-ideal.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_deviation_from_smoothed_geometry(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_normal_variation(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.normal-variation.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_surface_waviness(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.surface-waviness.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_irregularity(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.irregularity.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_holes(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.holes.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_handles(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.handles.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_boundary_loops(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.boundary-loops.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_euler_characteristic(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.euler-characteristic.v1", request)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn infer_gltf_leaf_genus(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    infer_gltf_leaf_cold("s.stdio.gltf.inference.genus.v1", request)
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::standards::
/// v2_0::engine::register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.gltf",
                    extension: Some("gltf"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf"),
                },
                dsl::LanguageSpec {
                    id: "stdio.gltf.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(io::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(io::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(io::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(io::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.gltf.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::diff::text::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.gltf.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(io::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(io::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.gltf.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    use crate::standards::v2_0::engine::io_registry as v2_0;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v2_0::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("GltfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v2_0::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ArtifactInferenceExecutionRequest, ArtifactInferenceServiceRegistry, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

    #[semio_framework_async_macros::async_test]
    async fn all_canonical_leaf_services_are_independently_registered() {
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

    #[semio_framework_async_macros::async_test]
    async fn one_leaf_service_returns_its_id_bound_generic_envelope() {
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
        let envelope = crate::io::inferences::binary::decode_gltf_inference_leaf_binary(&execution.canonical_payload).unwrap();
        assert_eq!(envelope.id, "s.stdio.gltf.inference.overall-size.v1");
        assert!(envelope.cache_key.contains(":p"));
        assert_eq!(envelope.dependency_hashes.len(), 1);
    }
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v2_0::engine::*`/root `engine::*` path still resolves.
        pub mod engine {
            pub use super::subsets::any::io::*;
            pub use super::subsets::any::schema::*;
            // 🎯 io and schema each define their own inferences/mutations submodule
            // (io holds binary/text codecs, schema holds the actual domain logic) --
            // disambiguate the resulting glob collision by explicitly preferring
            // schema, the richer, load-bearing side.
            pub use super::subsets::any::schema::inferences;
            pub use super::subsets::any::schema::mutations;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🤝️adjacency/🦀️.rs"]
                        pub mod adjacency;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🧱️area-volume/🦀️.rs"]
                        pub mod area_volume;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/↔️clearance/🦀️.rs"]
                        pub mod clearance;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🗜️compactness/🦀️.rs"]
                        pub mod compactness;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕳️concavity/🦀️.rs"]
                        pub mod concavity;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🌀️curvature/🦀️.rs"]
                        pub mod curvature;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🏗️dag-assembly/🦀️.rs"]
                        pub mod dag_assembly;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-core/🦀️.rs"]
                        pub mod geometry_core;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/⚖️mass-distribution/🦀️.rs"]
                        pub mod mass_distribution;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🧭️orientation/🦀️.rs"]
                        pub mod orientation;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🦀️.rs"]
                        pub mod proportion;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🌊️roughness/🦀️.rs"]
                        pub mod roughness;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/🦀️.rs"]
                        pub mod size;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🪞️symmetry/🦀️.rs"]
                        pub mod symmetry;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/↕️thickness/🦀️.rs"]
                        pub mod thickness;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🦀️.rs"]
                        pub mod topology;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🏠️default-scene/🔗️bind/🦀️.rs"]
                        pub mod bind_default_scene;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/🔗️bind/🦀️.rs"]
                        pub mod bind_morph_target_attribute;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📷️node-camera/🔗️bind/🦀️.rs"]
                        pub mod bind_node_camera;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌿️node-child/🔗️bind/🦀️.rs"]
                        pub mod bind_node_child;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🏗️node-mesh/🔗️bind/🦀️.rs"]
                        pub mod bind_node_mesh;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🩻️node-skin/🔗️bind/🦀️.rs"]
                        pub mod bind_node_skin;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/🔗️bind/🦀️.rs"]
                        pub mod bind_primitive_attribute;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔢️primitive-indices/🔗️bind/🦀️.rs"]
                        pub mod bind_primitive_indices;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧱️primitive-material/🔗️bind/🦀️.rs"]
                        pub mod bind_primitive_material;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌲️scene-root/🔗️bind/🦀️.rs"]
                        pub mod bind_scene_root_node;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪪️asset/📝️change-description/🦀️.rs"]
                        pub mod change_asset_descriptive_metadata;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪪️asset/🧩️change-extensions/🦀️.rs"]
                        pub mod change_asset_extension_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪪️asset/🧾️change-extras/🦀️.rs"]
                        pub mod change_asset_extra_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪪️asset/🔖️version/🦀️.rs"]
                        pub mod change_asset_version;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📃️document/🧩️change-extensions/🦀️.rs"]
                        pub mod change_document_extension_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📃️document/📝️change-extras/🦀️.rs"]
                        pub mod change_document_extra_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🌫️change-alpha/🦀️.rs"]
                        pub mod change_material_alpha_mode;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🪞️change-sides/🦀️.rs"]
                        pub mod change_material_double_sided;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/🧩️change-extensions/🦀️.rs"]
                        pub mod change_mesh_extension_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/📝️change-extras/🦀️.rs"]
                        pub mod change_mesh_extra_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/⚖️change-weights/🦀️.rs"]
                        pub mod change_mesh_morph_weights;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/🏷️rename/🦀️.rs"]
                        pub mod change_mesh_name;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🧩️change-extensions/🦀️.rs"]
                        pub mod change_node_extension_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/📝️change-extras/🦀️.rs"]
                        pub mod change_node_extra_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/⚖️change-weights/🦀️.rs"]
                        pub mod change_node_morph_weights;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/🦀️.rs"]
                        pub mod change_node_name;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🧩️change-extensions/🦀️.rs"]
                        pub mod change_primitive_extension_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/📝️change-extras/🦀️.rs"]
                        pub mod change_primitive_extra_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/📐️change-topology/🦀️.rs"]
                        pub mod change_primitive_topology_mode;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🧩️change-extensions/🦀️.rs"]
                        pub mod change_scene_extension_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/📝️change-extras/🦀️.rs"]
                        pub mod change_scene_extra_data;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🏷️rename/🦀️.rs"]
                        pub mod change_scene_name;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📐️accessor/🌱️create/🦀️.rs"]
                        pub mod create_accessor;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎞️animation/🌱️create/🦀️.rs"]
                        pub mod create_animation;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💿️buffer/🌱️create/🦀️.rs"]
                        pub mod create_buffer;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🌱️create/🦀️.rs"]
                        pub mod create_buffer_view;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎥️camera/🌱️create/🦀️.rs"]
                        pub mod create_camera;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🖼️image/🌱️create/🦀️.rs"]
                        pub mod create_image;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🌱️create/🦀️.rs"]
                        pub mod create_material;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/🌱️create/🦀️.rs"]
                        pub mod create_mesh;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🌱️create/🦀️.rs"]
                        pub mod create_morph_target;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🌱️create/🦀️.rs"]
                        pub mod create_node;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🌱️create/🦀️.rs"]
                        pub mod create_primitive;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎛️sampler/🌱️create/🦀️.rs"]
                        pub mod create_sampler;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🌱️create/🦀️.rs"]
                        pub mod create_scene;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦴️skin/🌱️create/🦀️.rs"]
                        pub mod create_skin;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎨️texture/🌱️create/🦀️.rs"]
                        pub mod create_texture;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📣️used-extension/➕️add/🦀️.rs"]
                        pub mod add_used_extension;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📐️accessor/🗑️delete/🦀️.rs"]
                        pub mod delete_accessor;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎞️animation/🗑️delete/🦀️.rs"]
                        pub mod delete_animation;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💿️buffer/🗑️delete/🦀️.rs"]
                        pub mod delete_buffer;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🗑️delete/🦀️.rs"]
                        pub mod delete_buffer_view;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎥️camera/🗑️delete/🦀️.rs"]
                        pub mod delete_camera;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🖼️image/🗑️delete/🦀️.rs"]
                        pub mod delete_image;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🗑️delete/🦀️.rs"]
                        pub mod delete_material;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/🗑️delete/🦀️.rs"]
                        pub mod delete_mesh;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🗑️delete/🦀️.rs"]
                        pub mod delete_morph_target;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🗑️delete/🦀️.rs"]
                        pub mod delete_node;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🗑️delete/🦀️.rs"]
                        pub mod delete_primitive;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎛️sampler/🗑️delete/🦀️.rs"]
                        pub mod delete_sampler;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🗑️delete/🦀️.rs"]
                        pub mod delete_scene;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦴️skin/🗑️delete/🦀️.rs"]
                        pub mod delete_skin;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎨️texture/🗑️delete/🦀️.rs"]
                        pub mod delete_texture;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📐️accessor/🚚️move/🦀️.rs"]
                        pub mod move_accessor;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎞️animation/🚚️move/🦀️.rs"]
                        pub mod move_animation;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💿️buffer/🚚️move/🦀️.rs"]
                        pub mod move_buffer;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🚚️move/🦀️.rs"]
                        pub mod move_buffer_view;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎥️camera/🚚️move/🦀️.rs"]
                        pub mod move_camera;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🖼️image/🚚️move/🦀️.rs"]
                        pub mod move_image;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🚚️move/🦀️.rs"]
                        pub mod move_material;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/🚚️move/🦀️.rs"]
                        pub mod move_mesh;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🚚️move/🦀️.rs"]
                        pub mod move_morph_target;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/🚚️move/🦀️.rs"]
                        pub mod move_morph_target_attribute;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🚚️move/🦀️.rs"]
                        pub mod move_node;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌿️node-child/🚚️move/🦀️.rs"]
                        pub mod move_node_child;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🚚️move/🦀️.rs"]
                        pub mod move_primitive;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/🚚️move/🦀️.rs"]
                        pub mod move_primitive_attribute;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required-extension/🚚️move/🦀️.rs"]
                        pub mod move_required_extension;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎛️sampler/🚚️move/🦀️.rs"]
                        pub mod move_sampler;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🚚️move/🦀️.rs"]
                        pub mod move_scene;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌲️scene-root/🚚️move/🦀️.rs"]
                        pub mod move_scene_root_node;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦴️skin/🚚️move/🦀️.rs"]
                        pub mod move_skin;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎨️texture/🚚️move/🦀️.rs"]
                        pub mod move_texture;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📣️used-extension/🚚️move/🦀️.rs"]
                        pub mod move_used_extension;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📐️accessor/🔀️reorder/🦀️.rs"]
                        pub mod reorder_accessors;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎞️animation/🔀️reorder/🦀️.rs"]
                        pub mod reorder_animations;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🔀️reorder/🦀️.rs"]
                        pub mod reorder_buffer_views;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💿️buffer/🔀️reorder/🦀️.rs"]
                        pub mod reorder_buffers;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎥️camera/🔀️reorder/🦀️.rs"]
                        pub mod reorder_cameras;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🖼️image/🔀️reorder/🦀️.rs"]
                        pub mod reorder_images;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🔀️reorder/🦀️.rs"]
                        pub mod reorder_materials;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🕸️mesh/🔀️reorder/🦀️.rs"]
                        pub mod reorder_meshs;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/🔀️reorder/🦀️.rs"]
                        pub mod reorder_morph_target_attributes;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🔀️reorder/🦀️.rs"]
                        pub mod reorder_morph_targets;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌿️node-child/🔀️reorder/🦀️.rs"]
                        pub mod reorder_node_children;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🔀️reorder/🦀️.rs"]
                        pub mod reorder_nodes;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/🔀️reorder/🦀️.rs"]
                        pub mod reorder_primitive_attributes;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🔀️reorder/🦀️.rs"]
                        pub mod reorder_primitives;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required-extension/🔀️reorder/🦀️.rs"]
                        pub mod reorder_required_extensions;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎛️sampler/🔀️reorder/🦀️.rs"]
                        pub mod reorder_samplers;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌲️scene-root/🔀️reorder/🦀️.rs"]
                        pub mod reorder_scene_root_nodes;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🔀️reorder/🦀️.rs"]
                        pub mod reorder_scenes;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦴️skin/🔀️reorder/🦀️.rs"]
                        pub mod reorder_skins;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎨️texture/🔀️reorder/🦀️.rs"]
                        pub mod reorder_textures;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📣️used-extension/🔀️reorder/🦀️.rs"]
                        pub mod reorder_used_extensions;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🌿️reparent/🦀️.rs"]
                        pub mod move_node_parent;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required-extension/➕️add/🦀️.rs"]
                        pub mod add_required_extension;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/📐️transform/🦀️.rs"]
                        pub mod change_node_transform;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🏠️default-scene/✂️unbind/🦀️.rs"]
                        pub mod unbind_default_scene;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/✂️unbind/🦀️.rs"]
                        pub mod unbind_morph_target_attribute;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📷️node-camera/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_camera;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌿️node-child/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_child;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🏗️node-mesh/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_mesh;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🩻️node-skin/✂️unbind/🦀️.rs"]
                        pub mod unbind_node_skin;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/✂️unbind/🦀️.rs"]
                        pub mod unbind_primitive_attribute;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔢️primitive-indices/✂️unbind/🦀️.rs"]
                        pub mod unbind_primitive_indices;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧱️primitive-material/✂️unbind/🦀️.rs"]
                        pub mod unbind_primitive_material;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌲️scene-root/✂️unbind/🦀️.rs"]
                        pub mod unbind_scene_root_node;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required-extension/➖️remove/🦀️.rs"]
                        pub mod remove_required_extension;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📣️used-extension/➖️remove/🦀️.rs"]
                        pub mod remove_used_extension;
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/💡️inference-measures/🦀️.rs"]
                        pub mod inference_measures;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧾️measurement-contracts/🦀️.rs"]
                        pub mod measurement_contracts;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🕸️mesh-topology/🦀️.rs"]
                        pub mod mesh_topology;
                        #[path = "."]
                        pub mod mutation_support {
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧬️mutation-support/🎬️create-scene/🦀️.rs"]
                            pub mod create_scene;
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧬️mutation-support/🎞️material-animation/🦀️.rs"]
                            pub mod material_animation;
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧬️mutation-support/🧱️structure-geometry/🦀️.rs"]
                            pub mod structure_geometry;
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs"]
                            pub mod top_level_collections;
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧬️mutation-support/📚️top-level/🦀️.rs"]
                            pub mod top_level;
                        }
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/🧮️vector-operations/🦀️.rs"]
                        pub mod vector_operations;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v2_0::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v2_0::engine::*;
}
pub mod io {
    pub use super::standards::v2_0::subsets::any::io::*;
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod metabolism {
        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/📚️examples/🌱️metabolism/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/📚️examples/🌱️metabolism/🧪️tests/🦀️.rs"]
        mod metabolism_tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gltf {
        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gltf {
        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/♾️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
