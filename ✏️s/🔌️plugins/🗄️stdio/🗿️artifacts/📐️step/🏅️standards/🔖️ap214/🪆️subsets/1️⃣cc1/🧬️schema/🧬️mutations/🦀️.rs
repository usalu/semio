//! 🧬️ `StepCc1Mutation` — ISO 10303-214 Conformance Class 1's OWN mutation vocabulary.
//!
//! 🎯️ Deliberately NOT the `🧱️base` subset's `StepMutation`. That one is the ISO 10303-21 GRAMMAR:
//! insert an entity, set an argument, remove an argument — eleven verbs that know nothing about
//! AP214 and would be identical for any Part-21 file on earth. A conformance class is not a grammar,
//! it is a FILTER, and the only edits that belong to it are the ones that move a document across the
//! filter. Every variant below is one rule of `check_cc1_conformance` (`../🦀️.rs`'s
//! `derived_analysis`), and there are no others because that function reads no other axis:
//!
//! | kind | rule | code |
//! |---|---|---|
//! | `set-file-schema` | `FILE_SCHEMA` must declare `AUTOMOTIVE_DESIGN` | `CODE_FILE_SCHEMA` (hard) |
//! | `remove-shape-representation` | CC1 admits NO `*_SHAPE_REPRESENTATION` at all | `CODE_SHAPE_REPRESENTATION_PRESENT` (hard) |
//! | `set-product-identity` | the `PRODUCT`/formation/definition chain | `CODE_PRODUCT_CHAIN` (soft) |
//!
//! ⚠️ **What makes CC1's vocabulary different from every other class's, and why it is not a rename
//! of theirs:** CC1 is `MAX_RUNG = 1`, and `ladder_rung_of` never returns anything below 2. So there
//! is NO representation type CC1 admits — `ceiling_type_of(1)` is `None` by construction — and this
//! vocabulary consequently has no verb that can WRITE one. `2️⃣cc2`..`6️⃣cc6` carry
//! `set-shape-representation`; CC1 cannot, because a "config data only" class has no conformant
//! state containing a representation to set. Its single ladder verb is deletion.
//!
//! ⚠️ **Inversion is where that asymmetry becomes visible.** Undoing a removal puts the
//! representation back, which is precisely the violation the removal repaired, and CC1 owns no filtered verb
//! for that state. `inverse()` therefore restores the instance through `restore-entities`, an exact absolute write
//! at its original position that is not filtered by the class ceiling.
//!
//! @see ../../../🧱️base/🚪️io/🪜️ladder/🦀️.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🔣️oracle.json — the `step-ap214-cc1` catalog `KINDS` is checked against.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder;
use crate::StepSnapshot;
use protocol::Mutation;

#[cfg(test)]
pub use crate::standards::v_ap214::subsets::base::schema::mutations::{apply_step_mutation};
pub use crate::standards::v_ap214::subsets::base::schema::mutations::{StepMutation};

//#region 🔖️Vocabulary
/// 🏷️ How this class names itself in a rejection message.
pub(crate) const CLASS: &str = "ISO 10303-214 CC1 (config data only)";

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "↩️restore-entities/🦀️.rs"]
pub mod restore_entities;
#[path = "🗑️remove-shape-representation/🦀️.rs"]
pub mod remove_shape_representation;
#[path = "🏷️set-file-schema/🦀️.rs"]
pub mod set_file_schema;
#[path = "🪪set-product-identity/🦀️.rs"]
pub mod set_product_identity;
//#endregion 🔖️Leaves

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/1️⃣cc1`.
///
/// Every inverse restores the touched entities through `restore-entities`, an exact absolute write that is not filtered by the class
/// ceiling, so undoing a repair can re-introduce the violation the repair removed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.cc1")]
pub enum StepCc1Mutation {
    SetFileSchema(set_file_schema::SetFileSchema),
    SetProductIdentity(set_product_identity::SetProductIdentity),
    /// 🗑️ CC1's only ladder verb. There is no `SetShapeRepresentation` counterpart: no rung is
    /// `<= 1`, so no representation is admissible and the sole conformance repair is deletion.
    RemoveShapeRepresentation(remove_shape_representation::RemoveShapeRepresentation),
    RestoreEntities(restore_entities::RestoreEntities),
}

/// 📇️ Kebab-case spelling of every `StepCc1Mutation` variant, in declaration order — the
/// `step-ap214-cc1` catalog in `../../🔣️oracle.json` must match verbatim.
pub const KINDS: &[&str] = &["set-file-schema", "set-product-identity", "remove-shape-representation", "restore-entities"];

impl StepCc1Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc1Mutation::SetFileSchema(_) => "set-file-schema",
            StepCc1Mutation::SetProductIdentity(_) => "set-product-identity",
            StepCc1Mutation::RemoveShapeRepresentation(_) => "remove-shape-representation",
            StepCc1Mutation::RestoreEntities(_) => "restore-entities",
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
pub fn apply_step_cc1_mutation(snapshot: &mut StepSnapshot, mutation: &StepCc1Mutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepCc1Mutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
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
pub(crate) fn restored(rows: Vec<ladder::EntityRestore>) -> Vec<StepCc1Mutation> {
    if rows.is_empty() {
        return Vec::new();
    }
    vec![StepCc1Mutation::RestoreEntities(restore_entities::RestoreEntities { entities: rows })]
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
