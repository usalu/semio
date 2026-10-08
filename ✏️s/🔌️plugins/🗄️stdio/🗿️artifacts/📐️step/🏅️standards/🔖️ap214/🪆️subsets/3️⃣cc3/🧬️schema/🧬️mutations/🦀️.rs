//! 🧬️ `StepCc3Mutation` — ISO 10303-214 CC3 (wireframe with topology)'s OWN mutation vocabulary.
//!
//! 🎯️ Deliberately NOT the `🧱️base` subset's `StepMutation`. That one is the ISO 10303-21 GRAMMAR:
//! insert an entity, set an argument, remove an argument — eleven verbs that know nothing about
//! AP214 and would be identical for any Part-21 file on earth. A conformance class is not a grammar,
//! it is a FILTER, and the only edits that belong to it are the ones that move a document across the
//! filter. Every variant below is one rule of `check_cc3_conformance` (`../🦀️.rs`'s
//! `derived_analysis`), and there are no others because that function reads no other axis:
//!
//! | kind | rule | code |
//! |---|---|---|
//! | `set-file-schema` | `FILE_SCHEMA` must declare `AUTOMOTIVE_DESIGN` | `CODE_FILE_SCHEMA` (hard) |
//! | `set-shape-representation` | no `*_SHAPE_REPRESENTATION` above rung 3 | `CODE_LADDER` (hard) |
//! | `demote-shape-representation` | the repair verb: rewrite an over-rung instance onto rung 3 | `CODE_LADDER` (hard) |
//! | `set-product-identity` | the `PRODUCT`/formation/definition chain | `CODE_PRODUCT_CHAIN` (soft) |
//!
//! 🪜️ **What this class admits, and what makes its vocabulary its own.** CC3 adds surfaces to CC2's curves while still carrying them GEOMETRICALLY — bounded by their own
//! geometry rather than by a topological shell. That is exactly where its ceiling sits, and it is why
//! `set-shape-representation` accepts `GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION` and refuses
//! `MANIFOLD_SURFACE_SHAPE_REPRESENTATION`: the two describe the same surfaces, and the ladder step
//! between them is the topology, not the geometry.
//!
//! ⬇️ **The repair verb.** A demotion in CC3 lands on the geometrically bounded SURFACE form, so an instance arriving from
//! rung 4, 5 or 6 keeps its surfaces and loses only the topological framing this class does not
//! admit — the smallest edit that brings it inside CC3 rather than a rewrite of its content.
//!
//! ⚠️ **A conformance class is not closed under inversion.** Undoing a ladder edit re-introduces whatever the edit removed, and a class
//! whose purpose is to forbid geometry above its ceiling cannot own a verb that writes it back. `inverse()` therefore restores every touched
//! entity through `restore-entities`: an exact absolute write at the original position, deliberately not filtered by the class ceiling.
//!
//! @see ../../../🧱️base/🚪️io/🪜️ladder/🦀️.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🔣️oracle.json — the `step-ap214-cc3` catalog `KINDS` is checked against.

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
pub(crate) const CLASS: &str = "ISO 10303-214 CC3 (wireframe with topology)";

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "↩️restore-entities/🦀️.rs"]
pub mod restore_entities;
#[path = "⬇️demote-shape-representation/🦀️.rs"]
pub mod demote_shape_representation;
#[path = "🏷️set-file-schema/🦀️.rs"]
pub mod set_file_schema;
#[path = "🪪set-product-identity/🦀️.rs"]
pub mod set_product_identity;
#[path = "🪜set-shape-representation/🦀️.rs"]
pub mod set_shape_representation;
//#endregion 🔖️Leaves

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/3️⃣cc3`.
///
/// Every inverse restores the touched entities through `restore-entities`, an exact absolute write that is not filtered by the class
/// ceiling, so undoing a repair can re-introduce the violation the repair removed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.cc3")]
pub enum StepCc3Mutation {
    SetFileSchema(set_file_schema::SetFileSchema),
    SetProductIdentity(set_product_identity::SetProductIdentity),
    SetShapeRepresentation(set_shape_representation::SetShapeRepresentation),
    DemoteShapeRepresentation(demote_shape_representation::DemoteShapeRepresentation),
    RestoreEntities(restore_entities::RestoreEntities),
}

/// 📇️ Kebab-case spelling of every `StepCc3Mutation` variant, in declaration order — the
/// `step-ap214-cc3` catalog in `../../🔣️oracle.json` must match verbatim.
pub const KINDS: &[&str] = &["set-file-schema", "set-product-identity", "set-shape-representation", "demote-shape-representation", "restore-entities"];

impl StepCc3Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc3Mutation::SetFileSchema(_) => "set-file-schema",
            StepCc3Mutation::SetProductIdentity(_) => "set-product-identity",
            StepCc3Mutation::SetShapeRepresentation(_) => "set-shape-representation",
            StepCc3Mutation::DemoteShapeRepresentation(_) => "demote-shape-representation",
            StepCc3Mutation::RestoreEntities(_) => "restore-entities",
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
pub fn apply_step_cc3_mutation(snapshot: &mut StepSnapshot, mutation: &StepCc3Mutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepCc3Mutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
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
pub(crate) fn restored(rows: Vec<ladder::EntityRestore>) -> Vec<StepCc3Mutation> {
    if rows.is_empty() {
        return Vec::new();
    }
    vec![StepCc3Mutation::RestoreEntities(restore_entities::RestoreEntities { entities: rows })]
}
//#endregion 🔖️Apply


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
