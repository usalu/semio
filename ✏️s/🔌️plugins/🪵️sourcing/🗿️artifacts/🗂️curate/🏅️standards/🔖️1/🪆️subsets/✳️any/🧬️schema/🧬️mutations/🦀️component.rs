//! 🧬️ Sourcing curate artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `CurateSnapshot`'s shape per `📓️derivation-rules.md` rule 2: `curated` is the one
//! genuinely id-keyed, user-editable collection (the curation selection — which objects, how many
//! units of each) addressed by `object_id`. `stock` is deliberately NOT represented in this enum:
//! it is a bulk-populated reference catalogue (seeded from
//! `crate::artifacts::curate::schema::sourcing_modules()`/hot-installed `sourcing.module`
//! contributions), never hand-authored item-by-item by a user — whole-catalogue population goes
//! through `store::ArtifactStore::reset` (see `crate::apps::curate::reset_document_effect`), same
//! non-history path as whole-document replace, never through this mutation enum. `CuratedItem` has
//! no name/key field beyond `object_id` (no `rename`) and no `Vec` member fields (no
//! `add-`/`remove-curated-item-*`), so the closed vocabulary is exactly the three id-keyed
//! collection verbs the schema supports: `create`, `delete`, `change` (count). The pre-migration
//! whole-document-replace variant (the former whole-snapshot-replace enum case) is gone with NO replacement per
//! `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6.
//!
//! All three triads are mounted as real `mutations`-sibling modules in `📦️glue.rs` (this lane's
//! agent owns `📦️glue.rs`), each with its own unique emoji-prefixed directory.

use crate::artifacts::curate::diff::CurateDiff;
use crate::artifacts::curate::CurateSnapshot;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧮️ Closed semantic mutation vocabulary for the curate document, derived per
/// `📓️derivation-rules.md` from `CurateSnapshot::curated`'s id-keyed shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = CurateSnapshot, diff = CurateDiff, schema = "sourcing.curate")]
pub enum SourcingMutation {
    CreateCuratedItem(CreateCuratedItem),
    DeleteCuratedItem(DeleteCuratedItem),
    ChangeCuratedItemCount(ChangeCuratedItemCount),
}
//#endregion 🔖️Mutations

// 🐛️ ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b fix: the bare `use super::<leaf>;` aliases this
// region used to carry (removed) collided (E0252) with the `pub use` below re-exporting each
// leaf's identically-named free function — the module alias and the function occupied the same
// import slot once both were in scope. Fixed by dropping the aliases and qualifying every
// `pub use` with `super::` directly, matching every sibling artifact's mutations component.rs in
// this taxonomy (e.g. `s/plugin/dag`'s identical triad list).
pub use super::change_curated_item_count::mutation::{change_curated_item_count, ChangeCuratedItemCount};
pub use super::create_curated_item::mutation::{create_curated_item, CreateCuratedItem};
pub use super::delete_curated_item::mutation::{delete_curated_item, DeleteCuratedItem};

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    use crate::artifacts::curate::CuratedItem;
    use protocol::Mutation;

    /// 🧪️ A base with one pre-existing curated entry (`beam-glulam-gl24h`) so `delete`/`change`
    /// mutations have a real target, and `beam-kvh-c24` left uncurated so `create` has a real
    /// not-yet-existing target — mirrors `din16798`'s `sample_snapshot()` fixture shape.
    fn sample_snapshot() -> CurateSnapshot {
        crate::artifacts::curate::curate_snapshot_from_stock(
            crate::artifacts::curate::schema::demo_stock(),
            vec![CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 2 }],
        )
    }

    /// ⚖️ One value per `SourcingMutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring `din16798`'s own `every_mutation()` fixture.
    fn every_mutation() -> Vec<SourcingMutation> {
        vec![
            SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-kvh-c24".into(), count: 3 } }),
            SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }),
            SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 5 }),
        ]
    }

    fn round_trip(base: &CurateSnapshot, mutation: &SourcingMutation) -> CurateSnapshot {
        let forward = vcs::apply_mutation(base, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back);
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
        assert_eq!(
            <SourcingMutation as protocol::SemanticMutation<CurateSnapshot>>::kinds().len(),
            every_mutation().len(),
            "kinds() must register exactly one descriptor per dispatch variant"
        );
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = sample_snapshot();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from
    /// `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs` (reachable here as
    /// `protocol::os_spr::testkit`), exercised against all three variants.
    #[test]
    fn create_curated_item_satisfies_the_inverse_and_absorb_laws() {
        let base = sample_snapshot();
        let mutation = SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-kvh-c24".into(), count: 2 } });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-kvh-c24".into(), new_count: 5 }).diff(&mid);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn delete_curated_item_satisfies_the_inverse_and_absorb_laws() {
        let mut base = sample_snapshot();
        base.curated.push(CuratedItem { object_id: "beam-steel-ipe200".into(), count: 4 });
        let mutation = SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-steel-ipe200".into() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-steel-hea160".into(), count: 1 } }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn change_curated_item_count_satisfies_the_inverse_and_absorb_laws() {
        let base = sample_snapshot();
        let mutation = SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 6 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
