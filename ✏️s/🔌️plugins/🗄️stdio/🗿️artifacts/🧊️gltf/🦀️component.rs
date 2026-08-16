//! 🎪 `stdio.gltf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferenceService, ArtifactInferenceServiceMetadata, ArtifactInferrer, ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::gltf::schema::diff::GltfDiff;
pub use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
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
        .inferences([crate::artifacts::gltf::schema::inferences::geometric_analysis::gltf_artifact_inference_descriptor()])
        .inference_services([gltf_inference_service()])
        .composers(crate::artifacts::gltf::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<GltfSnapshot, GltfMutation>(STDIO_GLTF_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🧠️ Native cold GLTF inference service registered by this artifact declaration.
pub fn gltf_inference_service() -> ArtifactInferenceService {
    ArtifactInferenceService::new(
        ArtifactInferenceServiceMetadata {
            owner: "stdio",
            artifact_kind: GLTF_ARTIFACT_KIND_ID,
            artifact_schema: GLTF_ARTIFACT_SCHEMA_ID,
            artifact_schema_version: GLTF_ARTIFACT_SCHEMA_VERSION,
            document_schema: STDIO_GLTF_DOCUMENT_SCHEMA,
            document_schema_version: GLTF_DOCUMENT_SCHEMA_VERSION,
            inference_schema: GLTF_INFERENCE_SCHEMA_ID,
            inference_schema_version: GLTF_INFERENCE_SCHEMA_VERSION,
            algorithm_version: GLTF_INFERENCE_ALGORITHM_VERSION,
            policy_version: GLTF_INFERENCE_POLICY_VERSION,
        },
        infer_gltf_cold,
    )
}

fn infer_gltf_cold(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let snapshot = <GltfSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload).map_err(|error| ArtifactInferenceExecutionError::new("stdio.gltf.inference.snapshot-decode", error.to_string()))?;
    let inference = <crate::artifacts::gltf::schema::GltfBuilder as ArtifactInferrer>::infer(&snapshot);
    let canonical_payload = crate::artifacts::gltf::io::inferences::binary::encode_gltf_inference_binary(&inference).map_err(|error| ArtifactInferenceExecutionError::new("stdio.gltf.inference.binary-encode", error.to_string()))?;
    Ok(ArtifactInferenceExecution { canonical_payload, diagnostics: Vec::new(), validity: "valid".into(), quality: "complete".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
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
    use semio_framework_plugin::{register_artifact_inference_service, wire_artifact_infer, ArtifactInferenceExecutionRequest, ArtifactInferenceServiceRegistry, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode, WireArtifactInferenceRequest, WireArtifactInferenceResult, ARTIFACT_INFERENCE_WIRE_VERSION};

    #[test]
    fn cold_native_inference_decodes_snapshot_pack_and_matches_typed_result() {
        let snapshot = GltfSnapshot::default();
        let snapshot_pack = <GltfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let mut registry = ArtifactInferenceServiceRegistry::new();
        registry.register(gltf_inference_service()).unwrap();
        let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 1 };
        let dependencies = Vec::new();
        let request = ArtifactInferenceExecutionRequest { policy: b"gltf-test", budgets: &budgets, cancellation_id: "gltf-native", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold, canonical_payload: &snapshot_pack, dependencies: &dependencies };
        let execution = registry.infer(GLTF_ARTIFACT_KIND_ID, GLTF_INFERENCE_SCHEMA_ID, &request).unwrap();
        let bytes = execution.canonical_payload;
        let decoded = crate::artifacts::gltf::io::inferences::binary::decode_gltf_inference_binary(&bytes).unwrap();
        let direct = <crate::artifacts::gltf::schema::GltfBuilder as ArtifactInferrer>::infer(&snapshot);
        assert_eq!(decoded, direct);
        assert_eq!(bytes, gltf_inference_service().infer(&request).unwrap().canonical_payload);
    }

    #[test]
    fn inference_wire_echoes_revision_and_uses_frozen_binary_codec() {
        let snapshot_pack = <GltfSnapshot as store::ArtifactPack>::encode_pack(&GltfSnapshot::default());
        let service = gltf_inference_service();
        register_artifact_inference_service(service).unwrap();
        let metadata = service.metadata();
        let request = WireArtifactInferenceRequest {
            wire_version: ARTIFACT_INFERENCE_WIRE_VERSION,
            owner: metadata.owner.into(),
            artifact_kind: metadata.artifact_kind.into(),
            artifact_schema: metadata.artifact_schema.into(),
            artifact_schema_version: metadata.artifact_schema_version,
            document_schema: metadata.document_schema.into(),
            document_schema_version: metadata.document_schema_version,
            inference_schema: metadata.inference_schema.into(),
            inference_schema_version: metadata.inference_schema_version,
            algorithm_version: metadata.algorithm_version,
            policy_version: metadata.policy_version,
            revision: 7,
            generation: 9,
            source_dialect: "s.stdio.gltf.standard.2.0.dialect.source".into(),
            policy: b"test-policy".to_vec(),
            budgets: WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 1 },
            cancellation_id: "gltf-wire".into(),
            previous_state: None,
            requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
            canonical_payload: snapshot_pack.clone(),
            dependencies: Vec::new(),
        };
        let bytes = wire_artifact_infer(&serde_json::to_vec(&request).unwrap()).unwrap();
        let result: WireArtifactInferenceResult = serde_json::from_slice(&bytes).unwrap();
        assert_eq!((result.revision, result.generation), (7, 9));
        assert_eq!(&result.policy, &request.policy);
        assert_eq!(&result.budgets, &request.budgets);
        assert_eq!(result.requested_cache_mode, request.requested_cache_mode.clone());
        assert_eq!(result.canonical_payload, service.infer(&ArtifactInferenceExecutionRequest { policy: &request.policy, budgets: &request.budgets, cancellation_id: &request.cancellation_id, previous_state: request.previous_state.as_deref(), requested_cache_mode: request.requested_cache_mode.clone(), canonical_payload: &request.canonical_payload, dependencies: &request.dependencies }).unwrap().canonical_payload);
    }
}
