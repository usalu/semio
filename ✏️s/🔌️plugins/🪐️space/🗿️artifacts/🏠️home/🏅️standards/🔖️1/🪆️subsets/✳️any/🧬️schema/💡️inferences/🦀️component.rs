//! 💡️ SHome inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🆔digest/`).
//!
//! The home snapshot is just two scalars (`schema`, `catalogGeneration`) — no positions, no
//! document structure, nothing to bound or order. The only honest whole-snapshot derivation is a
//! content fingerprint (mirrors an archive facet's `contentDigest`), so this uses the plain
//! `protocol::Inference<P>` shape (no `InferredField`/caching machinery — nothing here is
//! per-entity or incremental).

use crate::artifacts::home::SHomeSnapshot;
use protocol::Inference;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::digest::compute_content_digest;

//#region 🔖️Inference
/// 💡️ Everything inferable from an S Home snapshot. One field per named inference under
/// `💡️inferences/` (currently: `contentDigest`, backed by the `🆔digest/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.home.inference")]
pub struct SHomeInference {
    #[derived]
    pub content_digest: String,
}

impl protocol::Inference<SHomeSnapshot> for SHomeInference {
    fn infer(snapshot: &SHomeSnapshot) -> Self {
        Self { content_digest: compute_content_digest(snapshot) }
    }
}

/// 🌉️ Hand impl (not derived): a naive `#[derive(Default)]` would give `content_digest` an empty
/// string, which disagrees with `infer(&SHomeSnapshot::default())` (a real digest of the default
/// schema/generation) and would break `inference_default_law`. Defining default as "infer the
/// default snapshot" makes the two definitionally equal.
impl Default for SHomeInference {
    fn default() -> Self {
        Self::infer(&SHomeSnapshot::default())
    }
}

impl protocol::InferenceSpec<SHomeSnapshot> for SHomeInference {
    fn inference_schema_id() -> &'static str {
        "s.space.home.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.space.home.inference.digest.contentDigest", reads: &["schema", "catalogGeneration"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::home::standards::v1::subsets::any::schema::HomeBuilder {
    type Snapshot = SHomeSnapshot;
    type Inference = SHomeInference;

    /// 🎯️ Whole-snapshot scalar — nothing here is per-entity, so the cache/session are unused
    /// (same "plain `Inference`" shape the family doc calls out as correct for `dimensions`/
    /// `outline`/`bounds`-style facets).
    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = (cache, session);
        <SHomeInference as protocol::Inference<SHomeSnapshot>>::infer(snapshot)
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.space.home.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `home_artifact_schema_descriptor`'s registration.
pub fn home_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.space.home.inference",
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

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 7 };
        assert_eq!(SHomeInference::infer(&snapshot), SHomeInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(SHomeInference::infer(&SHomeSnapshot::default()), SHomeInference::default());
    }

    #[test]
    fn different_generations_yield_different_digests() {
        let a = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 1 };
        let b = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 2 };
        assert_ne!(SHomeInference::infer(&a).content_digest, SHomeInference::infer(&b).content_digest);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
