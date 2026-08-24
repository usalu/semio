//! 🧬️ EnergyModel artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `EnergyModelSnapshot`'s shape per `📓️derivation-rules.md`: `schema` is fixed
//! infrastructure (always `ENERGY_MODEL_DOCUMENT_SCHEMA`, never targeted by a mutation);
//! `structure`/`zones` (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: composed `s.stdio.
//! semio.value`/`table` children, replacing the old opaque `model_json` field) are the document's
//! entire substantive content, always regenerated TOGETHER from the same `crate::model::Model`.
//! Rule 6's "big but targeted change on one field" clause applies: `♻️replace-model` swaps
//! `structure`+`zones` together (still narrower than the banned whole-snapshot `SetSnapshot`,
//! which would also replace `schema`/`referencedModel`). `NoMutation` is gone outright (a
//! mutation with nothing to undo returns `Vec::new()` from `MutationKind::inverse`); `SetSnapshot`
//! is gone with NO replacement — file-open/import/load-example now goes through
//! `store::ArtifactStore::reset`, entirely outside this enum.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::EnergyModelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️MountedLeaves
// 🌱️ `replace_model` is declared by `📦️glue.rs` as a sibling of `component` (this file) under
// `pub mod mutations { ... }` — a real triad dir mounted with real `#[path]` entries (this facet's
// agent owns `glue.rs`, unlike sub-lane agents elsewhere in this ticket).
use super::replace_model;
//#endregion 🔖️MountedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the energy-model document, derived per
/// `📓️derivation-rules.md` from `EnergyModelSnapshot`'s composed `structure`/`zones` shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EnergyModelSnapshot, diff = EnergyModelDiff, schema = "energy.model")]
pub enum EnergyModelMutation {
    ReplaceModel(replace_model::mutation::ReplaceModel),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    use protocol::Mutation;

    /// 🧾️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: `new_model_json` must be a FULL,
    /// validly-shaped `Model` — `structure`/`zones` are minted by decoding it into a real typed
    /// `Model` (falling back to `Model::default()` on parse failure, see the triad's own `diff`
    /// doc comment), so a partial JSON literal like the pre-migration `{"a":1}` would silently
    /// collapse to the SAME default model for every payload, making round-trip/absorb assertions
    /// meaningless. `Model` derives `Default`, so every field is always present in its own
    /// `serde_json` output.
    fn demo_model_json(name: &str) -> String {
        serde_json::to_string(&crate::model::Model { name: name.into(), ..crate::model::Model::default() }).expect("Model serializes")
    }

    fn every_mutation() -> Vec<EnergyModelMutation> {
        vec![EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: demo_model_json("demo") })]
    }

    fn round_trip(base: &EnergyModelSnapshot, mutation: &EnergyModelMutation) -> EnergyModelSnapshot {
        let forward = mutation.diff(base).diff().apply(base).expect("valid mutation diff");
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = back.diff(&restored).diff().apply(&restored).expect("valid mutation diff");
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<EnergyModelMutation as protocol::SemanticMutation<EnergyModelSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[semio_framework_async_macros::async_test]
    async fn every_variant_round_trips_via_inverse() {
        let base = EnergyModelSnapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn replace_model_satisfies_the_inverse_and_absorb_laws() {
        let base = EnergyModelSnapshot::default();
        let mutation = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: demo_model_json("a") });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: demo_model_json("b") }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }
    //#endregion 🧪️MutationLaws

    // 🧪️OutcomeLaws — no `assert_missing_target_is_error`/`assert_fatal_never_applies` case applies:
    // this facet's one mutation kind (`replace-model`) is a root-scoped composed-children overwrite
    // with no addressable target to be missing, and malformed `new_model_json` is documented, honest
    // degradation to `Model::default()` (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's own
    // converter-honesty rule) rather than a Fatal — turning that into `mutation.invariant` would fight
    // that documented, pre-existing behavior, so this leaf stays a message-free
    // `MutationOutcome::new(diff)`. `assert_outcome_policy_matrix` is also not yet landed in
    // `📡️spr/🧪️testkit` — TODO(1-D testkit laws pending) once it lands.
}
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `EnergyModelMutation` variant, in declaration order — the vocabulary the `energy-model-1-any` mutation catalog
/// (`../../🧪️oracle/🔣️component.json`) declares and the `mutate-energy-model-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "replace-model",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `EnergyModelMutation` nor
/// `EnergyModelSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🧪️oracle/🔣️component.json — the catalog and the recorded no-oracle decision.
pub fn energy_model_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<EnergyModelSnapshot, String> {
        let decoded: EnergyModelSnapshot = serde_json::from_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EnergyModelMutation = serde_json::from_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &undone).apply_to(&mut undone);
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
        let descriptors = <EnergyModelMutation as protocol::SemanticMutation<EnergyModelSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance
