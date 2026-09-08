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
//! representation back, which is precisely the violation the removal repaired, and CC1 owns no verb
//! for that state. `inverse()` therefore degrades `remove-shape-representation` to `SetSnapshot` —
//! a real inverse (the projection is restored exactly), expressed through the only verb this class
//! has for a document outside itself. That is recorded here rather than hidden behind a promotion
//! verb CC1 must not have.
//!
//! @see ../../../🧱️base/🚪️io/🪜️ladder/🦀️.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🔣️oracle.json — the `step-ap214-cc1` catalog `KINDS` is checked against.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder::{self, ClassEdit};
use crate::standards::v_ap214::subsets::cc1::schema::MAX_RUNG;
use crate::StepSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;

pub use crate::standards::v_ap214::subsets::base::schema::mutations::{apply_step_mutation, StepMutation};

//#region 🔖️Vocabulary
/// 🏷️ How this class names itself in a rejection message.
const CLASS: &str = "ISO 10303-214 CC1 (config data only)";

//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-file-schema/🦀️.rs"]
pub mod set_file_schema;
#[path = "🪪set-product-identity/🦀️.rs"]
pub mod set_product_identity;
#[path = "🗑️remove-shape-representation/🦀️.rs"]
pub mod remove_shape_representation;
//#endregion 🔖️Leaves

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/1️⃣cc1`.
///
/// ⚠️ `NoMutation` is GONE. `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
/// leaf payload, and a unit variant wraps none; no approved verb means "do nothing" either. Its only
/// role was `inverse()`'s "nothing to undo" arm, which is now the empty vector — the same statement
/// with no vocabulary entry behind it. `SetSnapshot` is KEPT: the derive checks `SEMANTICS.verb`,
/// not the kind, and `set` is approved, so CC1's documented escape hatch (undoing a representation
/// removal by restoring the whole projection) survives the migration intact.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.cc1")]
pub enum StepCc1Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetFileSchema(set_file_schema::SetFileSchema),
    SetProductIdentity(set_product_identity::SetProductIdentity),
    /// 🗑️ CC1's only ladder verb. There is no `SetShapeRepresentation` counterpart: no rung is
    /// `<= 1`, so no representation is admissible and the sole conformance repair is deletion.
    RemoveShapeRepresentation(remove_shape_representation::RemoveShapeRepresentation),
}

/// 📇️ Kebab-case spelling of every `StepCc1Mutation` variant, in declaration order — the
/// `step-ap214-cc1` catalog in `../../🔣️oracle.json` must match verbatim.
pub const KINDS: &[&str] = &["set-snapshot", "set-file-schema", "set-product-identity", "remove-shape-representation"];

impl StepCc1Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc1Mutation::SetSnapshot(_) => "set-snapshot",
            StepCc1Mutation::SetFileSchema(_) => "set-file-schema",
            StepCc1Mutation::SetProductIdentity(_) => "set-product-identity",
            StepCc1Mutation::RemoveShapeRepresentation(_) => "remove-shape-representation",
        }
    }
}

//#region 🔖️ClassEdit
/// 🎚️ The diff every ladder-axis leaf produces: perform the class-neutral edit, or report the
/// class's own refusal. One implementation, four callers.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn class_diff(base: &StepSnapshot, edit: &ClassEdit) -> protocol::MutationOutcome<StepDiff> {
    match edited(base, edit) {
        Ok(next) => protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &next)),
        Err(message) => rejected(message),
    }
}

/// ↩️ A real per-axis inverse read off the base wherever CC1 owns a verb for it, and an explicit
/// whole-snapshot restore where it does not — `remove-shape-representation` against a real
/// representation puts back a state CC1 forbids, so no in-class verb can express it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn class_inverse(base: &StepSnapshot, edit: &ClassEdit) -> Vec<StepCc1Mutation> {
    match ladder::invert_class_edit(&base.to_part21_document(), MAX_RUNG, edit) {
        Some(ClassEdit::FileSchema { schemas }) => vec![StepCc1Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas })],
        Some(ClassEdit::ProductIdentity { identity }) => vec![StepCc1Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity })],
        Some(ClassEdit::Representation { id, row: None }) => vec![StepCc1Mutation::RemoveShapeRepresentation(remove_shape_representation::RemoveShapeRepresentation { id })],
        _ => vec![StepCc1Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
    }
}
//#endregion 🔖️ClassEdit

//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A rejected edit reports an error message with an empty diff and leaves the snapshot untouched —
/// never applied partially, never silently skipped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc1_mutation(snapshot: &mut StepSnapshot, mutation: &StepCc1Mutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepCc1Mutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
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
    protocol::MutationOutcome::error("stdio.step.cc1.mutation-rejected", message, Vec::<String>::new())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn edited(base: &StepSnapshot, edit: &ClassEdit) -> Result<StepSnapshot, String> {
    let mut doc = base.to_part21_document();
    ladder::apply_class_edit(&mut doc, CLASS, MAX_RUNG, edit)?;
    Ok(StepSnapshot::from_part21_document(&doc))
}
//#endregion 🔖️Apply


//#region 🚪️Reachability
/// ▶️ [`apply_step_cc1_mutation`] in a signature that names only this subset's own public types, so
/// an external crate can drive the real production apply path and still SEE a rejection instead of
/// discarding it. `protocol` is a private `extern crate` alias in this plugin's glue, so nothing
/// outside the crate can name `protocol::MutationOutcome` or `protocol::Mutation` — without these
/// two wrappers a test host could only re-derive the semantics by hand and would then be testing its
/// own re-derivation. Same wall, same fix as the 🧿️semio ✳️kit subset's.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc1_mutation_checked(snapshot: &mut StepSnapshot, mutation: &StepCc1Mutation) -> Result<(), String> {
    let outcome = apply_step_cc1_mutation(snapshot, mutation);
    match outcome.messages().first() {
        None => Ok(()),
        Some(message) => Err(format!("{:?} was rejected: [{}] {}", mutation, message.code.0, message.message)),
    }
}

/// ↩️ `Mutation::inverse` for `StepCc1Mutation`, reachable without naming the `protocol` alias — the
/// production inverse itself, never a copy of its rules.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_step_cc1_mutation(base: &StepSnapshot, mutation: &StepCc1Mutation) -> Vec<StepCc1Mutation> {
    <StepCc1Mutation as Mutation<StepSnapshot>>::inverse(mutation, base)
}
//#endregion 🚪️Reachability

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
