//! 🌳️ Lift file: 2 item(s), about 106 lines, of `📜️declaration/🌳️tree/🦀️.rs` that belong to crate `extension` (WP3).
//!
//! WP2 owns the source file and cuts the items here (preflight_artifact_declarations, commit_artifact_declarations); WP3 wires this file with one `mod` line in the crate root. Until `extension` exists, the source module re-exports the items with `pub use`.
use semio_framework::AppRole;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use semio_framework_plugin_identity::preflight_artifact_identity;
use crate::artifact_inference::preflight_artifact_inference_services;
use crate::artifact_inference::register_artifact_inference_services;
use semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration;
use semio_framework_plugin_decl_registry::declarations::check_surface_document_schema;
use semio_framework_plugin_decl_registry::declarations::check_surface_id;
use semio_framework_plugin_decl_registry::declarations::format_descriptor_of;
use semio_framework_plugin_decl_registry::declarations::preflight_io_entries;
use semio_framework_plugin_decl_registry::declarations::register_boot_scope_schema_exports;
use semio_framework_plugin_decl_registry::declarations::subset_binding;

/// 🔬️ Validates a whole batch of declared trees against every established registry WITHOUT
/// mutating any of them — schema/inference-descriptor/inference-service/language preflights
/// each own their independent lock (safe to call freely); document-codec and format-descriptor
/// preflights share ONE `semio_framework_schema_registry::assembly::begin()` guard (dropped before returning);
/// io entries get a hand-rolled coordinate-level duplicate check against `io_entries()` since
/// `semio_framework_os_kernel::io::io_mechanism` exposes no standalone dry-run (its own `io_register`
/// preflights+commits in one call — see the W1-C report's "atomicity" section for why holding
/// an outer guard across THAT call would deadlock the process-wide assembly mutex, and why this
/// function therefore never holds one while any io call could happen).
pub fn preflight_artifact_declarations<PA: PluginApp>(plugin_id: &str, declarations: &[ArtifactDeclaration<PA>]) -> Result<(), PluginAssemblyError> {
    let mut schemas = Vec::new();
    let mut inferences = Vec::new();
    let mut inference_services = Vec::new();
    let mut languages: Vec<semio_framework_dsl::LanguageSpec> = Vec::new();
    let mut codecs: Vec<store::os_store::serialization::ArtifactCodec> = Vec::new();
    let mut document_bindings = Vec::new();
    let mut format_rows = Vec::new();
    let mut native_snapshots = Vec::new();
    for artifact in declarations {
        preflight_artifact_identity(plugin_id, artifact.kind.as_str())?;
        for standard in &artifact.standards {
            format_rows.push(format_descriptor_of(artifact, standard));
            for subset in &standard.subsets {
                preflight_artifact_identity(plugin_id, subset.dialect.artifact_kind)?;
                if subset.dialect.artifact_kind != artifact.kind.as_str() {
                    return Err(PluginAssemblyError::new("plugin-assembly.declaration-dialect", "subset dialect must belong to the exact declared artifact root"));
                }
                schemas.push(subset.schema.descriptor);
                inferences.extend(subset.schema.inferences.iter().copied());
                inference_services.extend(subset.schema.inference_services.iter().copied());
                for pair in [subset.io.native.snapshot, subset.io.native.diff, subset.io.native.mutations].into_iter().chain(subset.io.native.inferences) {
                    languages.extend(pair.text.copied());
                    languages.extend(pair.binary.copied());
                }
                document_bindings.push(subset_binding(plugin_id, artifact, subset));
                codecs.push(subset.io.native.codec.clone());
                native_snapshots.extend(semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(subset.dialect.into(), subset.io.native.codec.clone()));
                check_surface_id(subset, &subset.editor, AppRole::Editor)?;
                check_surface_id(subset, &subset.viewer, AppRole::Viewer)?;
                check_surface_document_schema(subset, &subset.editor)?;
                check_surface_document_schema(subset, &subset.viewer)?;
            }
        }
    }
    semio_framework_os_kernel::io::io_mechanism::preflight_native_snapshots(&native_snapshots).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-snapshot", error.to_string()))?;
    semio_framework_os_kernel::io::preflight_artifact_codec_bindings(&document_bindings, &codecs).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-codec-binding", error.to_string()))?;
    ::semio_framework_schema_registry::preflight_artifact_schema_descriptors(&schemas).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-schema", error.to_string()))?;
    ::semio_framework_schema_registry::preflight_artifact_inference_descriptors(&inferences).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-inference", error.to_string()))?;
    preflight_artifact_inference_services(&inference_services).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-inference-service", error.to_string()))?;
    semio_framework_dsl::preflight_languages(&languages).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-language", error.to_string()))?;
    {
        let assembly = semio_framework_schema_registry::assembly::begin().map_err(|error| PluginAssemblyError::new("plugin-assembly.unavailable", error.to_string()))?;
        store::os_store::codec_registry::preflight_document_codecs_in_assembly(&assembly, &codecs).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-codec", error.to_string()))?;
        semio_framework_os_kernel::io::preflight_format_descriptors_in_assembly(&assembly, &format_rows).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-media", error.to_string()))?;
    }
    preflight_io_entries(declarations)
}

pub fn commit_artifact_declarations<PA: PluginApp>(plugin_id: &str, declarations: &[ArtifactDeclaration<PA>]) -> Result<(), PluginAssemblyError> {
    preflight_artifact_declarations(plugin_id, declarations)?;

    let mut schemas = Vec::new();
    let mut inferences = Vec::new();
    let mut inference_services = Vec::new();
    let mut languages: Vec<semio_framework_dsl::LanguageSpec> = Vec::new();
    let mut codecs: Vec<store::os_store::serialization::ArtifactCodec> = Vec::new();
    let mut document_bindings = Vec::new();
    let mut format_rows = Vec::new();
    let mut io_batches: Vec<&'static [semio_framework_os_kernel::io::io_mechanism::IoEntry]> = Vec::new();
    let mut native_snapshots = Vec::new();

    for artifact in declarations {
        for standard in &artifact.standards {
            format_rows.push(format_descriptor_of(artifact, standard));
            for subset in &standard.subsets {
                schemas.push(subset.schema.descriptor);
                inferences.extend(subset.schema.inferences.iter().copied());
                inference_services.extend(subset.schema.inference_services.iter().copied());
                for pair in [subset.io.native.snapshot, subset.io.native.diff, subset.io.native.mutations].into_iter().chain(subset.io.native.inferences) {
                    languages.extend(pair.text.copied());
                    languages.extend(pair.binary.copied());
                }
                document_bindings.push(subset_binding(plugin_id, artifact, subset));
                codecs.push(subset.io.native.codec.clone());
                native_snapshots.extend(semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(subset.dialect.into(), subset.io.native.codec.clone()));
                io_batches.push(subset.io.entries);
            }
        }
    }

    semio_framework_os_kernel::io::io_mechanism::preflight_native_snapshots(&native_snapshots).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-snapshot", error.to_string()))?;
    register_boot_scope_schema_exports();
    ::semio_framework_schema_registry::register_artifact_schema_descriptors(schemas).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-schema", error.to_string()))?;
    ::semio_framework_schema_registry::register_artifact_inference_descriptors(inferences).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-inference", error.to_string()))?;
    register_artifact_inference_services(inference_services).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-inference-service", error.to_string()))?;
    semio_framework_dsl::register_languages(languages).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-language", error.to_string()))?;
    {
        let assembly = semio_framework_schema_registry::assembly::begin().map_err(|error| PluginAssemblyError::new("plugin-assembly.unavailable", error.to_string()))?;
        let plan = semio_framework_os_kernel::io::ArtifactAssemblyRegistryPlan { document_codecs: codecs, document_bindings, format_descriptors: format_rows, native_snapshots, ..Default::default() };
        semio_framework_os_kernel::io::commit_artifact_assembly_registry_plan(&assembly, plan).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-registry", error.to_string()))?;
    }
    for batch in io_batches {
        semio_framework_os_kernel::io::io_mechanism::io_register(batch).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-io", error.to_string()))?;
    }

    Ok(())
}
