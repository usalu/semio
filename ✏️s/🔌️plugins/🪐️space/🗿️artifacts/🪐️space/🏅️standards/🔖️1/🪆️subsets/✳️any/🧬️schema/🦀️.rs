//! 🧬️ S Space index artifact schema facet — the subset's schema-descriptor assembly point. No
//! separate combined "artifact" struct this wave (unlike `🏠️home`'s `SHomeArtifact`): the index has no
//! config lane, so `SSpaceSnapshot` alone is the whole artifact-lane shape. `derive_artifact_facets!`/
//! `ArtifactBuilder`/`ArtifactAnalysis` machinery is intentionally NOT wired up this wave (no IO
//! composer needs it yet) — see `$T/📓️w1-e-report.md` for the explicit scope note.

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.space.space` — reuses the snapshot/diff/mutations Rust source as the "artifact"
/// facet's own Rust leaf too (no separate combined struct to source it from); the non-Rust leaves are
/// intentionally minimal placeholders (see the module doc above).
pub fn sspace_index_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    const PLACEHOLDER_GRAPHQL: &str = "# s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    const PLACEHOLDER_JSON: &str = "{}";
    const PLACEHOLDER_PROTO: &str = "// s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.space.space",
        artifact: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: PLACEHOLDER_GRAPHQL, json_schema: include_str!("🔣️.json"), proto: PLACEHOLDER_PROTO },
        snapshot: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: include_str!("📸️snapshot/🟦️.ts"), graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        diff: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts"), graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        mutations: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("🧬️mutations/🦀️.rs"), typescript: include_str!("🧬️mutations/🟦️.ts"), graphql: PLACEHOLDER_GRAPHQL, json_schema: include_str!("🧬️mutations/🔣️.json"), proto: PLACEHOLDER_PROTO },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
/// 🔎 Returns whether `s.space.space` is present in the process-local schema registry.
pub fn artifact_schema_registered() -> bool {
    ::semio_framework_schema_registry::artifact_schema_descriptor_registered("s.space.space")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️ArtifactItems
pub mod artifact_items {
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub const S_SPACE_INDEX_DOCUMENT_SCHEMA: &str = "s.space";

//#region 🔖️Dialect
/// 🪪️ Lives at the ARTIFACT level (not under `editor`/`viewer`), mirroring `HOME_DIALECT`'s own
/// placement doc — `artifact_kind` matches this subset's `#[artifact_schema(id = "s.space.space")]`,
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Canonical
/// surface id: `s.space.space@1/*#editor` / `s.space.space@1/*#viewer` (contract §1 grammar).
pub const SPACE_INDEX_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.space.space", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 📇️ The `os.space-index` topic row through which this plugin names the artifact-index document the os shell opens for a hub space, so the framework never names this plugin's dialect or document schema.
pub fn space_index_topic_contribution() -> semio_framework_plugin::TopicContribution {
    semio_framework_plugin::SpaceIndexContribution { dialect: SPACE_INDEX_DIALECT.into(), document_schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string() }.into_topic_contribution()
}
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ OS artifact kind for this document.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "space.sspace".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Space Artifacts", "Space-Artefakte"),
        source_format: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(),
        component_kind: "space-index".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems
