//! 🧬️ `StepCc6Mutation` — ISO 10303-214 CC6 (advanced B-Rep, top of the ladder)'s OWN mutation
//! vocabulary.
//!
//! 🎯️ Deliberately NOT the `🧱️base` subset's `StepMutation`. That one is the ISO 10303-21 GRAMMAR:
//! insert an entity, set an argument, remove an argument — eleven verbs that know nothing about
//! AP214 and would be identical for any Part-21 file on earth. A conformance class is not a grammar,
//! it is a FILTER, and the only edits that belong to it are the ones that move a document across the
//! filter. Every variant below is one rule of `check_cc6_conformance` (`../🦀️.rs`'s
//! `derived_analysis`), and there are no others because that function reads no other axis:
//!
//! | kind | rule | code |
//! |---|---|---|
//! | `set-file-schema` | `FILE_SCHEMA` must declare `AUTOMOTIVE_DESIGN` | `CODE_FILE_SCHEMA` (hard) |
//! | `set-shape-representation` | no `*_SHAPE_REPRESENTATION` above rung 6 | `CODE_LADDER` (hard) |
//! | `set-product-identity` | the `PRODUCT`/formation/definition chain | `CODE_PRODUCT_CHAIN` (soft) |
//!
//! 🪜️ **CC6 sits at the top of the ladder, and that changes its vocabulary rather than just its
//! constant.** `ladder_rung_of` classifies into 2..=6 and nothing higher, so `ladder_violations(doc,
//! 6)` is empty for EVERY document that can be written — CC6's `CODE_LADDER` arm is reachable in
//! code and unreachable in fact. Two consequences follow, and both are structural:
//!
//! * There is no `demote-shape-representation` here. `2️⃣cc2`..`5️⃣cc5` carry that verb because each
//!   of them has instances above its ceiling to bring down; CC6 has none, so a demotion verb would
//!   be a kind that can never move the projection — a scenario that always passes and proves
//!   nothing. It is absent for the same reason `1️⃣cc1`'s `set-shape-representation` is absent: the
//!   class has no state the verb could address.
//! * `set-shape-representation`'s guard still runs and is still real, but what it actually rejects
//!   here is a type name that is not on the ladder AT ALL — anything that does not end in
//!   `SHAPE_REPRESENTATION`. That is the one refusal CC6 can genuinely make, and the tests below
//!   assert exactly that rather than pretending a rung-7 exists.
//!
//! 🧫️ CC6 is also the only class this artifact's committed fixture already conforms to: `#13` is a
//! real `ADVANCED_BREP_SHAPE_REPRESENTATION`, rung 6, sitting exactly on this ceiling. Every other
//! class has to repair the file before it conforms; CC6 has to preserve it.
//!
//! ⚠️ **A conformance class is not closed under inversion** -- except, uniquely, this one, because no representation is above CC6's
//! ceiling. `inverse()` still restores every touched entity through the class-neutral `restore-entities`, an exact absolute write at the original position.
//!
//! @see ../../../🧱️base/🚪️io/🪜️ladder/🦀️.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🔣️oracle.json — the `step-ap214-cc6` catalog `KINDS` is checked against.

use crate::schema::diff::StepDiff;
#[cfg(test)]
use crate::standards::v_ap214::engine::ladder::ShapeRepresentationRow;
use crate::standards::v_ap214::engine::ladder;
use crate::StepSnapshot;
use protocol::Mutation;

#[cfg(test)]
pub use crate::standards::v_ap214::subsets::base::schema::mutations::{apply_step_mutation};
pub use crate::standards::v_ap214::subsets::base::schema::mutations::{StepMutation};

//#region 🔖️Vocabulary
/// 🏷️ How this class names itself in a rejection message.
pub(crate) const CLASS: &str = "ISO 10303-214 CC6 (advanced B-Rep, top of the ladder)";

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "↩️restore-entities/🦀️.rs"]
pub mod restore_entities;
#[path = "🏷️set-file-schema/🦀️.rs"]
pub mod set_file_schema;
#[path = "🪪set-product-identity/🦀️.rs"]
pub mod set_product_identity;
#[path = "🪜set-shape-representation/🦀️.rs"]
pub mod set_shape_representation;
//#endregion 🔖️Leaves

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/6️⃣cc6`.
///
/// Every inverse restores the touched entities through `restore-entities`, an exact absolute write that is not filtered by the class
/// ceiling, so undoing a repair can re-introduce the violation the repair removed.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.cc6")]
pub enum StepCc6Mutation {
    SetFileSchema(set_file_schema::SetFileSchema),
    SetProductIdentity(set_product_identity::SetProductIdentity),
    SetShapeRepresentation(set_shape_representation::SetShapeRepresentation),
    RestoreEntities(restore_entities::RestoreEntities),
}

/// 📇️ Kebab-case spelling of every `StepCc6Mutation` variant, in declaration order — the
/// `step-ap214-cc6` catalog in `../../🔣️oracle.json` must match verbatim. Four kinds,
/// not five: see the module header for why a demotion verb would be unobservable at this ceiling.
pub const KINDS: &[&str] = &["set-file-schema", "set-product-identity", "set-shape-representation", "restore-entities"];

impl StepCc6Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc6Mutation::SetFileSchema(_) => "set-file-schema",
            StepCc6Mutation::SetProductIdentity(_) => "set-product-identity",
            StepCc6Mutation::SetShapeRepresentation(_) => "set-shape-representation",
            StepCc6Mutation::RestoreEntities(_) => "restore-entities",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A rejected edit reports an error message with an empty diff and leaves the snapshot untouched —
/// never applied partially, never silently skipped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(test)]
pub fn apply_step_cc6_mutation(snapshot: &mut StepSnapshot, mutation: &StepCc6Mutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepCc6Mutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn rejected(message: String) -> protocol::MutationOutcome<StepDiff> {
    protocol::MutationOutcome::error("mutation.target-mismatch", message, Vec::<String>::new())
}

/// ↩️ Wraps the neutral restore rows into this class's single `restore-entities` mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn restored(rows: Vec<ladder::EntityRestore>) -> Vec<StepCc6Mutation> {
    if rows.is_empty() {
        return Vec::new();
    }
    vec![StepCc6Mutation::RestoreEntities(restore_entities::RestoreEntities { entities: rows })]
}
//#endregion 🔖️Apply


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
