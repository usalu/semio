//! ⚙️ S Home mutation codec bridge, catalog identity, and behavior tests. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! direct leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<SHomeSnapshot>`
//! and `impl protocol::SemanticMutation<SHomeSnapshot>` from that payload — no hand-written
//! apply/diff/inverse dispatch here. Whole-document replace (the old `SetSnapshot`) is banned; it
//! goes through `ArtifactStore::reset` (non-history), never through this enum.

use crate::artifacts::home::schema::mutations::{change_catalog_generation, register_s_home_mutation_descriptors, SHomeMutation};
use crate::artifacts::home::SHomeSnapshot;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn home_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&change_catalog_generation(7));
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_s_home_mutation_descriptors();
        for kind in <SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds().len(), 1);
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn change_catalog_generation_inverse_law() {
        let base = SHomeSnapshot::default();
        protocol::testkit::assert_mutation_inverse_law(&base, &change_catalog_generation(7));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_catalog_generation_diff_absorb_law() {
        use protocol::Mutation;
        let base = SHomeSnapshot::default();
        let d1 = change_catalog_generation(3).diff(&base).diff().clone();
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = change_catalog_generation(9).diff(&mid).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🔖️MutationLaws

    // 🧪️OutcomeLaws — no `assert_missing_target_is_error`/`assert_fatal_never_applies` case applies:
    // this facet's one mutation kind (`change-catalog-generation`) is a root scalar counter setter
    // with no addressable target and no domain invariant to violate — it can only succeed or be a
    // `mutation.no-op` (see the leaf's own `🔺️diff` for that check). `assert_outcome_policy_matrix`
    // is also not yet landed in `📡️spr/🧪️testkit` — TODO(1-D testkit laws pending) once it lands.
}
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `SHomeMutation` variant, in declaration order — the vocabulary the `s-home-1-any` mutation catalog
/// (`../../🧪️oracle/🔣️.json`) declares and the `mutate-s-home-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &["change-catalog-generation"];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `SHomeMutation` nor
/// `SHomeSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🧪️oracle/🔣️.json — the catalog and the recorded no-oracle decision.
pub fn s_home_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<SHomeSnapshot, String> {
        let decoded: SHomeSnapshot = serde_json::from_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: SHomeMutation = serde_json::from_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = serde_json::json!({
        "base": serde_json::to_value(&base).map_err(|error| error.to_string())?,
        "expectedSnapshot": serde_json::to_value(&expected).map_err(|error| error.to_string())?,
        "snapshot": serde_json::to_value(&applied).map_err(|error| error.to_string())?,
        "diff": serde_json::to_value(forward.diff()).map_err(|error| error.to_string())?,
        "messages": serde_json::to_value(forward.messages()).map_err(|error| error.to_string())?,
        "inverseSteps": serde_json::to_value(&inverse).map_err(|error| error.to_string())?,
        "inverseSnapshot": serde_json::to_value(&undone).map_err(|error| error.to_string())?,
        "inverseMessages": serde_json::to_value(&inverse_messages).map_err(|error| error.to_string())?,
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
mod kinds_conformance {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
    /// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
    /// honest in both directions at once.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance
