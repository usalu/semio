//! 🎪 `stdio.binary` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::binary::schema::diff::BinaryDiff;
pub use crate::artifacts::binary::schema::mutations::BinaryMutation;
pub use crate::artifacts::binary::schema::snapshot::BinarySnapshot;
pub use crate::artifacts::binary::schema::BinaryArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_BINARY_DOCUMENT_SCHEMA: &str = "stdio.binary";

/// 🧬️ Artifact schema descriptor id.
pub const BINARY_ARTIFACT_SCHEMA_ID: &str = "s.stdio.binary";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::definition_only_assembly("binary", definition)
}

//#region 🔖️ArtifactDeclaration
/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot): the whole
/// `s.stdio.binary` artifact through the declaration tree — one standard, `raw`, one subset,
/// `any`. `localization: &[]` — the real en/de localized descriptors already live in
/// `🧬️schema/📜️artifact-definition.json` (the OLD `ArtifactDefinition` channel this artifact stays
/// on for `📇️registry`'s catalog, kept per `assembly()` below); wiring them into this NEW field
/// too is a follow-up, not required for the carrier law or for this tree to register cleanly
/// (see `📓️w2-p-report.md` `## openQuestions`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::plugin::StdioApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.stdio.binary").expect("canonical stdio.binary kind"), localization: &[], standards: vec![crate::artifacts::binary::standards::v_raw::standard()] }
}
//#endregion 🔖️ArtifactDeclaration

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.binary".into(),
        name: "Binary".into(),
        source_format: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::binary::standards::v_raw::subsets::any::io::io_registry as v_raw;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    /// 🎹️ Every composer entry this artifact can serve, across all its standards.
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_raw::entries().iter().collect()).as_slice()
    }

    /// 🎯️ Compose into exactly one target dialect from a set of (possibly foreign-dialect) sources.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("BinaryComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    /// 📌️ Registers every entry into the OS-wide typed io registry. Called once from `🔌️plugin/🔧️setup`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_composer_entries(v_raw::entries());
    }

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{io_resolve, IoDirection, IoKey, IoPayload, StandardId, SubsetId};

        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

        #[semio_framework_async_macros::async_test]
        async fn compose_direct_round_trips_a_native_binary_payload() {
            let snapshot = crate::artifacts::binary::standards::v_raw::subsets::any::schema::empty_binary_snapshot();
            let bytes = store::ArtifactPack::encode_pack(&snapshot);
            let sources = [ErasedComposeSource { dialect: DIALECT, payload: IoPayload::Binary(bytes) }];
            let composed = compose(DIALECT, &sources).expect("compose");
            assert_eq!(composed.dialect, DIALECT);
            assert!(matches!(composed.payload, IoPayload::Binary(_)));
        }

        #[semio_framework_async_macros::async_test]
        async fn register_then_resolve_through_the_typed_registry_finds_this_composer() {
            register();
            let key = IoKey { artifact_kind: "s.stdio.binary".into(), standard: "raw".into(), subset: "*".into(), direction: IoDirection::Import, format_kind: "s.stdio.binary".into(), format_standard: "raw".into(), format_subset: "*".into() };
            let entry = io_resolve(&key).expect("resolve");
            assert_eq!(entry.writes, DIALECT);
        }
    }
    //#endregion 🧪️Tests
}
//#endregion 🚪️DerivedIoRegistry
