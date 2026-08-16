//! 🎪 `stdio.png` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::png::schema::diff::PngDiff;
pub use crate::artifacts::png::schema::mutations::PngMutation;
pub use crate::artifacts::png::schema::snapshot::PngSnapshot;
pub use crate::artifacts::png::schema::PngArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PNG_DOCUMENT_SCHEMA: &str = "stdio.png";

/// 🧬️ Artifact schema descriptor id.
pub const PNG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.png";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.png".into(),
        name: "Png".into(),
        source_format: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::artifacts::png::engine::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// row — png has no baseline/tiny/basic subset here, just the single ✳️any entry) by its FULLY
/// QUALIFIED path, never the bare `io_registry::entries()` shortcut that would silently rebind to
/// this file's own shadowing `io_registry` module below (repo-wide "silent rebind" hazard this
/// ticket tracks — that module returns `&[&ComposerEntry]`, a different type, and is left in place
/// as orphaned dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). Unlike
/// `🖼️tiff`/`📷️jpg`/`🎨️svg`, png's `register()` never registered a subset validator (no baseline
/// subset here) and never called `register_schema_specs()` — nothing left uncovered. `⚙️engine`
/// itself is untouched — this only REFERENCES what it already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("png", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::png::standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor())
        .inferences([crate::artifacts::png::standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor()])
        .composers(crate::artifacts::png::standards::v1_2::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA)
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — moved
/// here verbatim from `⚙️engine::register_pilot_languages` (same 5-role Document/Ops/Diff/Pack/Spr
/// shape every stdio artifact uses), leaked to a `&'static` slice since `dsl::passthrough_hooks`
/// isn't `const fn`, mirroring the `🗒️note`/`🔋️model` exemplars' own helper of the same shape.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.png",
                    extension: Some("png"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.png"),
                },
                dsl::LanguageSpec {
                    id: "stdio.png.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.png.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.png.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.png.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.png.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.png.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.png.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::png::standards::v1_2::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.png.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ImperativeRegister
/// 🌉️ Relocated from `⚙️engine::register` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `declaration()` above is what stdio's
/// own plugin-host boot path now uses, but external plugins (`🖨️raster`'s own
/// `ensure_stdio_semio_and_png_registered`) call this imperative entry point directly for
/// standalone `cargo test` runs that never execute the declarative plugin-host boot. Behavior
/// preserved verbatim — only the module path changed (`engine::register` → `png::register`).
pub fn register() {
    io_registry::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::png::standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor());
    ::schema::register_artifact_inference_descriptor(crate::artifacts::png::standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor());
    for lang in pilot_languages() {
        dsl::register_language(lang.clone());
    }
    store::register_document_codec(store::ArtifactCodec::of::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA));
}
//#endregion 🔖️ImperativeRegister

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::png::standards::v1_2::subsets::any::io::io_registry as v1_2;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_2::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("PngComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1_2::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
