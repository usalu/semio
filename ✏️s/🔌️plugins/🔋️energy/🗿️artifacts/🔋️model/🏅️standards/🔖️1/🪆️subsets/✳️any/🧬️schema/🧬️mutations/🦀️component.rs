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
        vec![EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel {
            new_model_json: demo_model_json("demo"),
        })]
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

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<EnergyModelMutation as protocol::SemanticMutation<EnergyModelSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = EnergyModelSnapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    #[test]
    fn replace_model_satisfies_the_inverse_and_absorb_laws() {
        let base = EnergyModelSnapshot::default();
        let mutation = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: demo_model_json("a") });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: demo_model_json("b") }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
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
