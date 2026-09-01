//! 💡️ Rewrite inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors puzzle3d's own `💡️inferences/` (the pattern's exemplar): this file is the
//! family-root assembly (never mod's/includes the slug dirs directly — `📦️glue.rs` is the sole
//! mounting mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `📦bounds/`, the only positioned data this rule-editing artifact's snapshot carries
//! a typed shape for — `before_fixture_json`/`lhs_json`/`rhs_json` are opaque JSON blobs, not
//! structured graph data this artifact's own snapshot exposes).

use crate::artifacts::rewrite::RewriteSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::bounds::{compute_bounds, RewriteBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a rewrite-rule snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite.inference")]
pub struct RewriteInference {
    #[derived]
    pub bounds: RewriteBounds,
}

impl protocol::Inference<RewriteSnapshot> for RewriteInference {
    fn infer(snapshot: &RewriteSnapshot) -> Self {
        Self { bounds: compute_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<RewriteSnapshot> for RewriteInference {
    fn inference_schema_id() -> &'static str {
        "s.trinity.rewrite.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.trinity.rewrite.inference.bounds", reads: &["rule_layout"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: a handful of `{x, y}` points recomputed in one pass — the default `infer_cached`
/// passthrough (just calls `infer`) is exactly right here, no `InferredField` chain needed.
impl ArtifactInferrer for crate::artifacts::rewrite::standards::v1::subsets::any::schema::RewriteBuilder {
    type Snapshot = RewriteSnapshot;
    type Inference = RewriteInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.trinity.rewrite.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `rewrite_artifact_schema_descriptor`'s registration.
pub fn rewrite_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.trinity.rewrite.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::rewrite::LayoutPoint;
    use protocol::Inference;
    use std::collections::BTreeMap;

    //#region 🧸️Fixtures
    fn two_point_snapshot() -> RewriteSnapshot {
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint { x: 0.0, y: 0.0 });
        rule_layout.insert("b".to_string(), LayoutPoint { x: -140.0, y: 80.0 });
        RewriteSnapshot { rule_layout, ..RewriteSnapshot::default() }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = two_point_snapshot();
        assert_eq!(RewriteInference::infer(&snapshot), RewriteInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(RewriteInference::infer(&RewriteSnapshot::default()), RewriteInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_bounds_matches_rule_layout_extents() {
        let snapshot = two_point_snapshot();
        let inferred = RewriteInference::infer(&snapshot);
        assert_eq!(inferred.bounds.node_count, 2);
        assert_eq!(inferred.bounds.bounding_box.min_x, -140.0);
        assert_eq!(inferred.bounds.bounding_box.min_y, 0.0);
        assert_eq!(inferred.bounds.bounding_box.max_x, 0.0);
        assert_eq!(inferred.bounds.bounding_box.max_y, 80.0);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
