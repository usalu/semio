//! 🧬️ `StepCc2Mutation` — ISO 10303-214 CC2 (bounded wireframe/basic surfaces)'s OWN mutation vocabulary.
//!
//! 🎯️ Deliberately NOT the `🧱️base` subset's `StepMutation`. That one is the ISO 10303-21 GRAMMAR:
//! insert an entity, set an argument, remove an argument — eleven verbs that know nothing about
//! AP214 and would be identical for any Part-21 file on earth. A conformance class is not a grammar,
//! it is a FILTER, and the only edits that belong to it are the ones that move a document across the
//! filter. Every variant below is one rule of `check_cc2_conformance` (`../🦀️.rs`'s
//! `derived_analysis`), and there are no others because that function reads no other axis:
//!
//! | kind | rule | code |
//! |---|---|---|
//! | `set-file-schema` | `FILE_SCHEMA` must declare `AUTOMOTIVE_DESIGN` | `CODE_FILE_SCHEMA` (hard) |
//! | `set-shape-representation` | no `*_SHAPE_REPRESENTATION` above rung 2 | `CODE_LADDER` (hard) |
//! | `demote-shape-representation` | the repair verb: rewrite an over-rung instance onto rung 2 | `CODE_LADDER` (hard) |
//! | `set-product-identity` | the `PRODUCT`/formation/definition chain | `CODE_PRODUCT_CHAIN` (soft) |
//!
//! 🪜️ **What this class admits, and what makes its vocabulary its own.** CC2 is the FIRST class that admits geometry at all, and the last that admits only ONE kind of it.
//! Its ceiling is `GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION` — curves and points carried
//! directly by a representation, with no topology and no surfaces — so `set-shape-representation`
//! here refuses every one of the four types above it, starting with the surface representation CC3
//! is named for.
//!
//! ⬇️ **The repair verb.** A demotion in CC2 is the deepest one the ladder can perform: whatever rung an instance sat on,
//! it comes down to bounded wireframe, the minimal geometry-bearing form. Against this artifact's own
//! committed fixture that means `#13`, a real `ADVANCED_BREP_SHAPE_REPRESENTATION`, falls four rungs
//! in one edit while keeping its name, its `items` list and its `context_of_items` untouched.
//!
//! ⚠️ **A conformance class is not closed under inversion.** Undoing a ladder edit re-introduces
//! whatever the edit removed, and a class whose entire purpose is to forbid geometry above its own
//! ceiling cannot own a verb that writes geometry above its own ceiling back. `inverse()` therefore
//! returns the in-class verb whenever the base's own representation is admissible HERE, and degrades
//! to `SetSnapshot` when it is not — a real inverse either way (the projection is restored exactly),
//! but expressed through the only verb this class has for a document outside itself. That asymmetry
//! is recorded rather than papered over with a promotion verb this class must not have.
//!
//! @see ../../../🧱️base/🚪️io/🪜️ladder/🦀️.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🔣️oracle.json — the `step-ap214-cc2` catalog `KINDS` is checked against.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder::{self, ClassEdit};
#[cfg(test)]
use crate::standards::v_ap214::engine::ladder::ShapeRepresentationRow;
use crate::standards::v_ap214::subsets::cc2::schema::MAX_RUNG;
use crate::StepSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;

pub use crate::standards::v_ap214::subsets::base::schema::mutations::{apply_step_mutation, StepMutation};

//#region 🔖️Vocabulary
/// 🏷️ How this class names itself in a rejection message.
const CLASS: &str = "ISO 10303-214 CC2 (bounded wireframe/basic surfaces)";

//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-file-schema/🦀️.rs"]
pub mod set_file_schema;
#[path = "🪪set-product-identity/🦀️.rs"]
pub mod set_product_identity;
#[path = "🪜set-shape-representation/🦀️.rs"]
pub mod set_shape_representation;
#[path = "⬇️demote-shape-representation/🦀️.rs"]
pub mod demote_shape_representation;
//#endregion 🔖️Leaves

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/2️⃣cc2`.
///
/// ⚠️ `NoMutation` is GONE — `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
/// leaf payload and a unit variant wraps none. Its only role was `inverse()`'s "nothing to undo" arm,
/// now the empty vector. `SetSnapshot` is KEPT: the derive checks `SEMANTICS.verb`, not the kind, and
/// `set` is approved — so this class's whole-document restore survives intact.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.cc2")]
pub enum StepCc2Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetFileSchema(set_file_schema::SetFileSchema),
    SetProductIdentity(set_product_identity::SetProductIdentity),
    SetShapeRepresentation(set_shape_representation::SetShapeRepresentation),
    DemoteShapeRepresentation(demote_shape_representation::DemoteShapeRepresentation),
}

/// 📇️ Kebab-case spelling of every `StepCc2Mutation` variant, in declaration order — the
/// `step-ap214-cc2` catalog in `../../🔣️oracle.json` must match verbatim.
pub const KINDS: &[&str] = &["set-snapshot", "set-file-schema", "set-product-identity", "set-shape-representation", "demote-shape-representation"];

impl StepCc2Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc2Mutation::SetSnapshot(_) => "set-snapshot",
            StepCc2Mutation::SetFileSchema(_) => "set-file-schema",
            StepCc2Mutation::SetProductIdentity(_) => "set-product-identity",
            StepCc2Mutation::SetShapeRepresentation(_) => "set-shape-representation",
            StepCc2Mutation::DemoteShapeRepresentation(_) => "demote-shape-representation",
        }
    }


}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A rejected edit reports an error message with an empty diff and leaves the snapshot untouched —
/// never applied partially, never silently skipped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc2_mutation(snapshot: &mut StepSnapshot, mutation: &StepCc2Mutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepCc2Mutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn rejected(message: String) -> protocol::MutationOutcome<StepDiff> {
    protocol::MutationOutcome::error("stdio.step.cc2.mutation-rejected", message, Vec::<String>::new())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn edited(base: &StepSnapshot, edit: &ClassEdit) -> Result<StepSnapshot, String> {
    let mut doc = base.to_part21_document();
    ladder::apply_class_edit(&mut doc, CLASS, MAX_RUNG, edit)?;
    Ok(StepSnapshot::from_part21_document(&doc))
}
//#endregion 🔖️Apply

//#region 🔖️ClassEdit
/// 🎚️ The diff every ladder-axis leaf produces: perform the class-neutral edit, or report the class's
/// own refusal. One implementation, every leaf a caller.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn class_diff(base: &StepSnapshot, edit: &ClassEdit) -> protocol::MutationOutcome<StepDiff> {
    match edited(base, edit) {
        Ok(next) => protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &next)),
        Err(message) => rejected(message),
    }
}

/// ↩️ A real per-axis inverse read off the base wherever this class owns a verb for it, and an
/// explicit whole-snapshot restore where it does not.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn class_inverse(base: &StepSnapshot, edit: &ClassEdit) -> Vec<StepCc2Mutation> {
    match ladder::invert_class_edit(&base.to_part21_document(), MAX_RUNG, edit) {
        Some(ClassEdit::FileSchema { schemas }) => vec![StepCc2Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas })],
        Some(ClassEdit::ProductIdentity { identity }) => vec![StepCc2Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity })],
        Some(ClassEdit::Representation { id, row }) => vec![StepCc2Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id, representation: row })],
        _ => vec![StepCc2Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
    }
}
//#endregion 🔖️ClassEdit


//#region 🚪️Reachability
/// ▶️ [`apply_step_cc2_mutation`] in a signature that names only this subset's own public types, so
/// an external crate can drive the real production apply path and still SEE a rejection instead of
/// discarding it. `protocol` is a private `extern crate` alias in this plugin's glue, so nothing
/// outside the crate can name `protocol::MutationOutcome` or `protocol::Mutation` — without these
/// two wrappers a test host could only re-derive the semantics by hand and would then be testing its
/// own re-derivation. Same wall, same fix as the 🧿️semio ✳️kit subset's.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc2_mutation_checked(snapshot: &mut StepSnapshot, mutation: &StepCc2Mutation) -> Result<(), String> {
    let outcome = apply_step_cc2_mutation(snapshot, mutation);
    match outcome.messages().first() {
        None => Ok(()),
        Some(message) => Err(format!("{:?} was rejected: [{}] {}", mutation, message.code.0, message.message)),
    }
}

/// ↩️ `Mutation::inverse` for `StepCc2Mutation`, reachable without naming the `protocol` alias — the
/// production inverse itself, never a copy of its rules.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_step_cc2_mutation(base: &StepSnapshot, mutation: &StepCc2Mutation) -> Vec<StepCc2Mutation> {
    <StepCc2Mutation as Mutation<StepSnapshot>>::inverse(mutation, base)
}
//#endregion 🚪️Reachability

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
