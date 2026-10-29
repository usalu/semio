//! 📜️ The definition-only stdio artifact catalog: the same selected roster as the full catalog with the artifacts that publish no native
//! codec (ifc, semio) contributed by their schema-owned definitions alone, so consumers that only commit to the catalog link no IFC
//! standard and no semio conversion. Its catalog commitment is byte-identical to the full catalog's (see the roster law in the full catalog).


use semio_framework_plugin::PluginAssemblyError;
use semio_s_artifact_stdio_contract::registry::ContributionRegistry;
use semio_s_artifact_stdio_contract::{definition_only_assembly, ArtifactAssembly, ArtifactContribution, NativeCodecFactory};
pub use semio_hub_stdio_catalog_projection::{native_artifact_catalog_dependency, validate_native_artifact_catalog_dependency};

/// 🧵️ The native codec factories of an artifact contributed by its definition alone: none.
fn no_native_codecs() -> Vec<NativeCodecFactory> {
    Vec::new()
}

/// 🗂️ The ifc artifact as its schema-owned definition.
fn ifc_assembly() -> Result<ArtifactAssembly, PluginAssemblyError> {
    definition_only_assembly("ifc", semio_s_artifact_stdio_ifc::definition()?)
}

/// 🏗️ The ifc contribution without any IFC standard.
fn ifc_contribution() -> ArtifactContribution {
    ArtifactContribution { definition_constraint: None, identity: "ifc", schema: semio_s_artifact_stdio_ifc::ARTIFACT_DEFINITION_SCHEMA, definition: semio_s_artifact_stdio_ifc::definition, assembly: ifc_assembly, formats: semio_s_artifact_stdio_ifc::formats, native_codecs: no_native_codecs }
}

/// 🧿️ The semio artifact as its schema-owned definition.
fn semio_assembly() -> Result<ArtifactAssembly, PluginAssemblyError> {
    definition_only_assembly("semio", semio_s_artifact_stdio_semio::definition()?)
}

/// 🧿️ The semio contribution without any subset conversion.
fn semio_contribution() -> ArtifactContribution {
    ArtifactContribution { definition_constraint: None, identity: "semio", schema: semio_s_artifact_stdio_semio::ARTIFACT_DEFINITION_SCHEMA, definition: semio_s_artifact_stdio_semio::definition, assembly: semio_assembly, formats: semio_s_artifact_stdio_semio::formats, native_codecs: no_native_codecs }
}

/// 📋️ The selected roster in the full catalog's authored order.
pub fn selected_contributions() -> Vec<ArtifactContribution> {
    vec![semio_s_artifact_stdio_binary::contribution(), semio_s_artifact_stdio_txt::contribution(), semio_s_artifact_stdio_xml::contribution(), semio_s_artifact_stdio_deflate::contribution(), semio_s_artifact_stdio_zip::contribution(), semio_s_artifact_stdio_json::contribution(), semio_s_artifact_stdio_csv::contribution(), semio_s_artifact_stdio_md::contribution(), semio_s_artifact_stdio_gltf::contribution(), semio_s_artifact_stdio_obj::contribution(), semio_s_artifact_stdio_stl::contribution(), semio_s_artifact_stdio_ply::contribution(), semio_s_artifact_stdio_las::contribution(), semio_s_artifact_stdio_step::contribution(), ifc_contribution(), semio_s_artifact_stdio_dwg::contribution(), semio_s_artifact_stdio_dxf::contribution(), semio_s_artifact_stdio_svg::contribution(), semio_s_artifact_stdio_png::contribution(), semio_s_artifact_stdio_jpg::contribution(), semio_s_artifact_stdio_gif::contribution(), semio_s_artifact_stdio_bmp::contribution(), semio_s_artifact_stdio_tiff::contribution(), semio_s_artifact_stdio_pdf::contribution(), semio_s_artifact_stdio_docx::contribution(), semio_s_artifact_stdio_pptx::contribution(), semio_s_artifact_stdio_xlsx::contribution(), semio_s_artifact_stdio_bcf::contribution(), semio_contribution(), semio_s_artifact_stdio_mp4::contribution(), semio_s_artifact_stdio_avi::contribution(), semio_s_artifact_stdio_mp3::contribution(), semio_s_artifact_stdio_wav::contribution(), semio_s_artifact_stdio_epw::contribution(), semio_s_artifact_stdio_tsv::contribution(), semio_s_artifact_stdio_html::contribution()]
}

/// 🪢️ The validated definition-only roster.
pub fn selected_registry() -> Result<ContributionRegistry, PluginAssemblyError> {
    ContributionRegistry::new(selected_contributions())
}

/// 🧮️ Recomputes the complete headless projection from the roster's definitions.
pub fn native_artifact_catalog_contribution() -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    let registry = selected_registry()?;
    semio_hub_stdio_catalog_projection::artifact_catalog_contribution_for(&registry, &registry.artifact_assemblies()?)
}
