//! 🔌️ Plugin root contract for the headless stdio library.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the stdio library plugin (no document apps).
pub fn plugin() -> Plugin {
    crate::manifest::register_stdio_format_descriptors();
    crate::artifacts::binary::engine::register();
    crate::artifacts::txt::engine::register();
    crate::artifacts::ifc::engine::register();
    crate::artifacts::gif::engine::register();
    crate::artifacts::bmp::engine::register();
    crate::artifacts::semio::standards::v1::engine::register();
    crate::artifacts::wav::standards::riff_pcm::engine::register();
    crate::artifacts::epw::standards::energyplus::engine::register();
    crate::artifacts::tsv::standards::iana::engine::register();
    crate::artifacts::html::standards::v5::engine::register();
    Plugin::builder("stdio")
        .label("Stdio")
        .version("0.1.0")
        .artifact_kind(crate::artifacts::binary::artifact_kind())
        .artifact_kind(crate::artifacts::txt::artifact_kind())
        .artifact_kind(crate::artifacts::json::artifact_kind())
        .artifact(crate::artifacts::json::declaration())
        .artifact_kind(crate::artifacts::xml::artifact_kind())
        .artifact(crate::artifacts::xml::declaration())
        .artifact_kind(crate::artifacts::csv::artifact_kind())
        .artifact(crate::artifacts::csv::declaration())
        .artifact_kind(crate::artifacts::md::artifact_kind())
        .artifact(crate::artifacts::md::declaration())
        .artifact_kind(crate::artifacts::zip::artifact_kind())
        .artifact(crate::artifacts::zip::declaration())
        .artifact_kind(crate::artifacts::gltf::artifact_kind())
        .artifact(crate::artifacts::gltf::declaration())
        .artifact_kind(crate::artifacts::las::artifact_kind())
        .artifact(crate::artifacts::las::declaration())
        // 🚧️ W6 g4 gap: `dsl::registry::register_schema_spec` (FullResolver insertion, no
        // `ArtifactDeclaration` field) — see `las::declaration()`'s own doc.
        .setup(crate::artifacts::las::engine::register_schema_specs)
        .artifact_kind(crate::artifacts::ifc::artifact_kind())
        // 🚧️ W6 g4 gap: `ifc` NOT converted — two independent standards (`v4`/`v2x3`) each with
        // their own `ArtifactSchemaDescriptor` id and document codec, which one
        // `ArtifactDeclaration`'s single `.schema()`/`.document_codec()` slots cannot represent
        // without dropping one standard's live registrations. See `ifc::component.rs`'s own doc.
        .artifact_kind(crate::artifacts::step::artifact_kind())
        .artifact(crate::artifacts::step::declaration())
        .artifact_kind(crate::artifacts::deflate::artifact_kind())
        .artifact(crate::artifacts::deflate::declaration())
        // 🚧️ W6 g2 gap: `dsl::registry::register_schema_spec` (FullResolver insertion, no
        // `ArtifactDeclaration` field) — see `deflate::declaration()`'s own doc.
        .setup(crate::artifacts::deflate::engine::register_schema_specs)
        .artifact_kind(crate::artifacts::bcf::artifact_kind())
        .artifact(crate::artifacts::bcf::declaration())
        .artifact_kind(crate::artifacts::xlsx::artifact_kind())
        .artifact(crate::artifacts::xlsx::declaration())
        .artifact_kind(crate::artifacts::pptx::artifact_kind())
        .artifact(crate::artifacts::pptx::declaration())
        .artifact_kind(crate::artifacts::docx::artifact_kind())
        .artifact(crate::artifacts::docx::declaration())
        .artifact_kind(crate::artifacts::pdf::artifact_kind())
        .artifact(crate::artifacts::pdf::declaration())
        .artifact(crate::artifacts::pdf::declaration_1_4())
        // 🚧️ W6 g2 gap: 1.4's `dsl::registry::register_schema_spec` (FullResolver insertion, no
        // `ArtifactDeclaration` field) — see `pdf::declaration()`'s own doc. 1.7 never called this.
        .setup(crate::artifacts::pdf::standards::v1_4::engine::register_schema_specs)
        .artifact_kind(crate::artifacts::tiff::artifact_kind())
        .artifact(crate::artifacts::tiff::declaration())
        .artifact_kind(crate::artifacts::gif::artifact_kind())
        .artifact_kind(crate::artifacts::jpg::artifact_kind())
        .artifact(crate::artifacts::jpg::declaration())
        .artifact_kind(crate::artifacts::png::artifact_kind())
        .artifact(crate::artifacts::png::declaration())
        .artifact_kind(crate::artifacts::bmp::artifact_kind())
        .artifact_kind(crate::artifacts::svg::artifact_kind())
        .artifact(crate::artifacts::svg::declaration())
        .artifact_kind(crate::artifacts::dxf::artifact_kind())
        .artifact(crate::artifacts::dxf::declaration())
        .artifact_kind(crate::artifacts::dwg::artifact_kind())
        .artifact(crate::artifacts::dwg::declaration())
        // 🚧️ W6 g4 gap: `dsl::registry::register_schema_spec` (FullResolver insertion, no
        // `ArtifactDeclaration` field) — see `dwg::declaration()`'s own doc.
        .setup(crate::artifacts::dwg::engine::register_schema_specs)
        .artifact_kind(crate::artifacts::ply::artifact_kind())
        .artifact(crate::artifacts::ply::declaration())
        .artifact_kind(crate::artifacts::stl::artifact_kind())
        .artifact(crate::artifacts::stl::declaration())
        .artifact_kind(crate::artifacts::obj::artifact_kind())
        .artifact(crate::artifacts::obj::declaration())
        // 🚧️ W6 g4 gap: `dsl::registry::register_schema_spec` (FullResolver insertion, no
        // `ArtifactDeclaration` field) — see `obj::declaration()`'s own doc.
        .setup(crate::artifacts::obj::engine::register_schema_specs)
        .artifact_kind(crate::artifacts::semio::artifact_kind())
        .artifact_kind(crate::artifacts::mp4::artifact_kind())
        .artifact(crate::artifacts::mp4::declaration())
        .artifact_kind(crate::artifacts::avi::artifact_kind())
        .artifact(crate::artifacts::avi::declaration())
        .artifact_kind(crate::artifacts::mp3::artifact_kind())
        .artifact(crate::artifacts::mp3::declaration())
        .artifact_kind(crate::artifacts::wav::artifact_kind())
        .artifact_kind(crate::artifacts::epw::artifact_kind())
        .artifact_kind(crate::artifacts::tsv::artifact_kind())
        .artifact_kind(crate::artifacts::html::artifact_kind())
        .library()
}
