//! 🧬️ EnergyModel artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `EnergyModelSnapshot`'s shape per `📓️derivation-rules.md`: the snapshot has only
//! two persistent fields, `schema` (fixed infrastructure, always `ENERGY_MODEL_DOCUMENT_SCHEMA`,
//! never targeted by a mutation) and `model_json` (the opaque serialized `crate::model::Model`
//! body — the document's entire substantive content). Rule 6's "big but targeted change on one
//! field" clause applies: `♻️replace-model` swaps `model_json` alone, leaving `schema` untouched —
//! narrower than the banned whole-snapshot `SetSnapshot`. `NoMutation` is gone outright (a
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
/// `📓️derivation-rules.md` from `EnergyModelSnapshot`'s two-field opaque-body shape.
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
    use protocol::SemanticMutation;
    use protocol::Mutation;

    fn every_mutation() -> Vec<EnergyModelMutation> {
        vec![EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel {
            new_model_json: r#"{"name":"demo","zones":[]}"#.to_string(),
        })]
    }

    fn round_trip(base: &EnergyModelSnapshot, mutation: &EnergyModelMutation) -> EnergyModelSnapshot {
        let mut forward = base.clone();
        forward.model_json = mutation.diff(base).model_json.unwrap_or(base.model_json.clone());
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored.model_json = back.diff(&restored).model_json.unwrap_or(restored.model_json.clone());
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
        let mutation = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: r#"{"a":1}"#.to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: r#"{"b":2}"#.to_string() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
