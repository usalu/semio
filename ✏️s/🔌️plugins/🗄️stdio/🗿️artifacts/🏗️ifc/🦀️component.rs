//! 🎪 `stdio.ifc` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::ifc::schema::diff::IfcDiff;
pub use crate::artifacts::ifc::schema::mutations::IfcMutation;
pub use crate::artifacts::ifc::schema::snapshot::IfcSnapshot;
pub use crate::artifacts::ifc::schema::IfcArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_IFC_DOCUMENT_SCHEMA: &str = "stdio.ifc";

/// 🧬️ Artifact schema descriptor id.
pub const IFC_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc";

/// ⚠️ **Deliberately left imperative** (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4):
/// no `declaration()` here, `crate::artifacts::ifc::engine::register()` NOT removed from the
/// plugin root. Unlike `dwg` (which looks superficially similar — two standards, `ac1018`/
/// `ac1024`), `ifc`'s second standard is genuinely live, not dead code: `📦️glue.rs`'s `engine` shim
/// for `ifc` locally OVERRIDES `register()` (shadowing the `v4` glob re-export) to call BOTH
/// `standards::v4::engine::register()` AND `standards::v2x3::engine::register()` explicitly — see
/// that shim's own doc ("registers BOTH standards' engines... same shape as pdf's own shim fix").
/// `v2x3::engine::register()` registers a SECOND, independent `ArtifactSchemaDescriptor`
/// (`"s.stdio.ifc.2x3"`, vs v4's `"s.stdio.ifc"`) and a SECOND, independent document codec
/// (`Ifc2x3Snapshot`/`Ifc2x3Mutation` under `STDIO_IFC2X3_DOCUMENT_SCHEMA`, a different schema
/// string from v4's `STDIO_IFC_DOCUMENT_SCHEMA`) plus its own 5-role `LanguageSpec` set and three
/// subset validators (`cv20`/`sav`/`cobie`). `ArtifactDeclaration` has exactly ONE `.schema()`
/// slot and ONE `.document_codec()`/`.document_codec_bare()` slot per declaration (mandatory
/// single fields, not accumulating like `.inferences()`/`.languages()` are) — there is structurally
/// no single field, and no combination of existing fields, that can hold two independent
/// ArtifactSchemaDescriptor ids or two independent document codecs at once. Converting `v4` alone
/// (dropping `v2x3`'s calls) would silently break `v2x3` registration, which IS live today —
/// unlike `dwg`'s `ac1018`, this is not preserving already-dead behavior, it would be a real
/// regression. **What would cover it**: either `ArtifactDeclaration` growing an accumulating
/// multi-schema/multi-codec shape (analogous to how `.inferences()`/`.languages()` already
/// accumulate), or two separate `PluginBuilder::artifact()` calls sharing one Rust module but
/// different `kind` strings (`"s.stdio.ifc"` + `"s.stdio.ifc.2x3"`) — the latter is plausible but
/// unverified here: `v2x3`'s composer/subset-validator `Dialect.artifact_kind` values were not
/// confirmed to actually vary by kind vs standard, and splitting risks a silent ownership-check
/// failure at `register_all()` build time that this pass's verification budget cannot fully rule
/// out. Composers ARE unioned safely already (`crate::artifacts::ifc::io_registry::entries()`
/// below merges `v4`+`v2x3`, same shape as `dwg`) but that's insufficient on its own — see above.

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::definition_only_assembly("ifc", definition)
}

pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.ifc".into(),
        name: "Ifc".into(),
        source_format: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::ifc::standards::v2x3::engine::io_registry as v2x3;
    use crate::artifacts::ifc::standards::v4::engine::io_registry as v4;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v4::entries().iter().chain(v2x3::entries().iter()).collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("IfcComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v4::entries());
        register_composer_entries(v2x3::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
