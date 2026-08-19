//! 🧬️ S Space index artifact schema facet — the subset's schema-descriptor assembly point. No
//! separate combined "artifact" struct this wave (unlike `🏠️home`'s `SHomeArtifact`): the index has no
//! config lane, so `SSpaceSnapshot` alone is the whole artifact-lane shape. `derive_artifact_facets!`/
//! `ArtifactBuilder`/`ArtifactAnalysis` machinery is intentionally NOT wired up this wave (no IO
//! composer needs it yet) — see `$T/📓️w1-e-report.md` for the explicit scope note.

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.space.space` — reuses the snapshot/diff/mutations Rust source as the "artifact"
/// facet's own Rust leaf too (no separate combined struct to source it from); the non-Rust leaves are
/// intentionally minimal placeholders (see the module doc above).
pub async fn sspace_index_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    const PLACEHOLDER_TS: &str = "// s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    const PLACEHOLDER_GRAPHQL: &str = "# s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    const PLACEHOLDER_JSON: &str = "{}";
    const PLACEHOLDER_PROTO: &str = "// s.space.space: no separate non-Rust schema leaf authored this wave.\n";
    schema::ArtifactSchemaDescriptor {
        id: "s.space.space",
        artifact: schema::FacetLeaves { rust: include_str!("📸️snapshot/🦀️component.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        snapshot: schema::FacetLeaves { rust: include_str!("📸️snapshot/🦀️component.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        diff: schema::FacetLeaves { rust: include_str!("🔺️diff/🦀️component.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
        mutations: schema::FacetLeaves { rust: include_str!("🧬️mutations/🦀️component.rs"), typescript: PLACEHOLDER_TS, graphql: PLACEHOLDER_GRAPHQL, json_schema: PLACEHOLDER_JSON, proto: PLACEHOLDER_PROTO },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
/// 🔎 Returns whether `s.space.space` is present in the process-local schema registry.
pub async fn artifact_schema_registered() -> bool {
    ::schema::artifact_schema_descriptor_registered("s.space.space")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_carries_the_space_index_schema_id() {
        assert_eq!(sspace_index_schema_descriptor().id, "s.space.space");
    }
}
//#endregion 🧪️Tests
