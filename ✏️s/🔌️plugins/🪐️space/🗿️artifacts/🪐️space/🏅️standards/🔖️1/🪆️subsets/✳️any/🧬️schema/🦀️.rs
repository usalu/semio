//! 🧬️ S Space index artifact schema facet — the subset's schema-descriptor assembly point. No
//! separate combined "artifact" struct this wave (unlike `🏠️home`'s `SHomeArtifact`): the index has no
//! config lane, so `SSpaceSnapshot` alone is the whole artifact-lane shape. `derive_artifact_facets!`/
//! `ArtifactBuilder`/`ArtifactAnalysis` machinery is intentionally NOT wired up this wave (no IO
//! composer needs it yet) — see `$T/📓️w1-e-report.md` for the explicit scope note.

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.space.space` — reuses the snapshot/diff/mutations Rust source as the "artifact"
/// facet's own Rust leaf too (no separate combined struct to source it from); the non-Rust leaves are
/// intentionally minimal placeholders (see the module doc above).
pub fn sspace_index_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    const PLACEHOLDER_TS: &str = "// s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    const PLACEHOLDER_GRAPHQL: &str = "# s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    const PLACEHOLDER_JSON: &str = "{}";
    const PLACEHOLDER_PROTO: &str = "// s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.space.space",
        artifact: ::semio_framework_schema::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        snapshot: ::semio_framework_schema::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        diff: ::semio_framework_schema::FacetLeaves { rust: include_str!("🔺️diff/🦀️.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        mutations: ::semio_framework_schema::FacetLeaves { rust: include_str!("🧬️mutations/🦀️.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
/// 🔎 Returns whether `s.space.space` is present in the process-local schema registry.
pub fn artifact_schema_registered() -> bool {
    ::semio_framework_schema::artifact_schema_descriptor_registered("s.space.space")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
