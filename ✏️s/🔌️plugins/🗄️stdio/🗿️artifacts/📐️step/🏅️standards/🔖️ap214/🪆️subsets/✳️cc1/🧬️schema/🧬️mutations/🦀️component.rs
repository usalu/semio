//! 🧬️ `StepCc1Mutation` — ISO 10303-214 Conformance Class 1's OWN mutation vocabulary.
//!
//! 🎯️ Deliberately NOT the `✳️any` subset's `StepMutation`. That one is the ISO 10303-21 GRAMMAR:
//! insert an entity, set an argument, remove an argument — eleven verbs that know nothing about
//! AP214 and would be identical for any Part-21 file on earth. A conformance class is not a grammar,
//! it is a FILTER, and the only edits that belong to it are the ones that move a document across the
//! filter. Every variant below is one rule of `check_cc1_conformance` (`../🦀️component.rs`'s
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
//! vocabulary consequently has no verb that can WRITE one. `✳️cc2`..`✳️cc6` carry
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
//! @see ../../../✳️any/🚪️io/🪜️ladder/🦀️component.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🧪️oracle/🔣️component.json — the `step-ap214-cc1` catalog `KINDS` is checked against.

use crate::artifacts::step::schema::diff::StepDiff;
use crate::artifacts::step::standards::v_ap214::engine::ladder::{self, ClassEdit, ProductIdentity};
use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::MAX_RUNG;
use crate::artifacts::step::StepSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::step::standards::v_ap214::subsets::any::schema::mutations::{apply_step_mutation, StepMutation};

//#region 🔖️Vocabulary
/// 🏷️ How this class names itself in a rejection message.
const CLASS: &str = "ISO 10303-214 CC1 (config data only)";

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/✳️cc1`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum StepCc1Mutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: StepSnapshot,
    },
    SetFileSchema {
        schemas: Vec<String>,
    },
    SetProductIdentity {
        identity: Option<ProductIdentity>,
    },
    /// 🗑️ CC1's only ladder verb. There is no `SetShapeRepresentation` counterpart: no rung is
    /// `<= 1`, so no representation is admissible and the sole conformance repair is deletion.
    RemoveShapeRepresentation {
        id: u64,
    },
}

/// 📇️ Kebab-case spelling of every `StepCc1Mutation` variant, in declaration order — the
/// `step-ap214-cc1` catalog in `../../🧪️oracle/🔣️component.json` must match verbatim.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-file-schema", "set-product-identity", "remove-shape-representation"];

impl StepCc1Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc1Mutation::NoMutation => "no-mutation",
            StepCc1Mutation::SetSnapshot { .. } => "set-snapshot",
            StepCc1Mutation::SetFileSchema { .. } => "set-file-schema",
            StepCc1Mutation::SetProductIdentity { .. } => "set-product-identity",
            StepCc1Mutation::RemoveShapeRepresentation { .. } => "remove-shape-representation",
        }
    }

    /// 🎚️ This mutation as the class-neutral edit the shared ladder module performs, or `None` for
    /// the two whole-document verbs that address no conformance axis.
    fn class_edit(&self) -> Option<ClassEdit> {
        match self {
            StepCc1Mutation::NoMutation | StepCc1Mutation::SetSnapshot { .. } => None,
            StepCc1Mutation::SetFileSchema { schemas } => Some(ClassEdit::FileSchema { schemas: schemas.clone() }),
            StepCc1Mutation::SetProductIdentity { identity } => Some(ClassEdit::ProductIdentity { identity: identity.clone() }),
            StepCc1Mutation::RemoveShapeRepresentation { id } => Some(ClassEdit::Representation { id: *id, row: None }),
        }
    }
}
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
fn rejected(message: String) -> protocol::MutationOutcome<StepDiff> {
    protocol::MutationOutcome::error("stdio.step.cc1.mutation-rejected", message, Vec::<String>::new())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edited(base: &StepSnapshot, edit: &ClassEdit) -> Result<StepSnapshot, String> {
    let mut doc = base.to_part21_document();
    ladder::apply_class_edit(&mut doc, CLASS, MAX_RUNG, edit)?;
    Ok(StepSnapshot::from_part21_document(doc))
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<StepSnapshot> for StepCc1Mutation {
    type Diff = StepDiff;

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        match self.class_edit() {
            None => match self {
                StepCc1Mutation::SetSnapshot { snapshot } => protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, snapshot)),
                _ => protocol::MutationOutcome::new(StepDiff::default()),
            },
            Some(edit) => match edited(base, &edit) {
                Ok(next) => protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &next)),
                Err(message) => rejected(message),
            },
        }
    }

    /// ↩️ A real per-axis inverse read off the base wherever CC1 owns a verb for it, and an explicit
    /// whole-snapshot restore where it does not. The second case is `remove-shape-representation`
    /// against a real representation: putting one back is a state CC1 forbids, so no in-class verb
    /// can express it. See the module header.
    fn inverse(&self, base: &StepSnapshot) -> Vec<Self> {
        let restore = || vec![StepCc1Mutation::SetSnapshot { snapshot: base.clone() }];
        match self.class_edit() {
            None => match self {
                StepCc1Mutation::SetSnapshot { .. } => restore(),
                _ => vec![StepCc1Mutation::NoMutation],
            },
            Some(edit) => match ladder::invert_class_edit(&base.to_part21_document(), MAX_RUNG, &edit) {
                Some(ClassEdit::FileSchema { schemas }) => vec![StepCc1Mutation::SetFileSchema { schemas }],
                Some(ClassEdit::ProductIdentity { identity }) => vec![StepCc1Mutation::SetProductIdentity { identity }],
                Some(ClassEdit::Representation { id, row: None }) => vec![StepCc1Mutation::RemoveShapeRepresentation { id }],
                _ => restore(),
            },
        }
    }
}
//#endregion 🔖️MutationTrait

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
mod tests {
    use super::*;
    use crate::artifacts::step::standards::v_ap214::engine::ladder::{has_product_definition_chain, ladder_violations, ShapeRepresentationRow};
    use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance;

    fn base() -> StepSnapshot {
        StepSnapshot::from_part21_document(Part21Document {
            header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
            instances: vec![
                Part21Instance { id: 821, entities: vec![("PRODUCT_DEFINITION".into(), vec![Part21Value::Str("A".into())])] },
                Part21Instance { id: 822, entities: vec![("PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE".into(), vec![Part21Value::Str("A".into())])] },
                Part21Instance { id: 827, entities: vec![("PRODUCT".into(), vec![Part21Value::Str("Document".into())])] },
                Part21Instance { id: 13, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![Part21Value::Str("brep_rep_0".into()), Part21Value::List(vec![Part21Value::Ref(12)]), Part21Value::Ref(835)])] },
            ],
        })
    }

    fn round_trip(mutation: StepCc1Mutation) {
        let start = base();
        let mut mutated = start.clone();
        let outcome = apply_step_cc1_mutation(&mut mutated, &mutation);
        assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
        assert_ne!(mutated, start, "{mutation:?} changed nothing -- a mutation that is not observable proves nothing");
        for step in Mutation::inverse(&mutation, &start) {
            apply_step_cc1_mutation(&mut mutated, &step);
        }
        assert_eq!(mutated, start, "{mutation:?} then its inverse must restore the base");
    }

    #[test]
    fn every_conformance_axis_round_trips_through_its_own_inverse() {
        round_trip(StepCc1Mutation::SetFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] });
        round_trip(StepCc1Mutation::SetProductIdentity { identity: None });
        round_trip(StepCc1Mutation::RemoveShapeRepresentation { id: 13 });
    }

    /// 🎯️ Each verb must move the diagnostic it was derived from, or it is not that rule's verb.
    #[test]
    fn each_verb_moves_the_diagnostic_it_was_derived_from() {
        let mut snapshot = base();
        assert!(!ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).is_empty(), "the base deliberately violates CC1 with a rung-6 representation");
        apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::RemoveShapeRepresentation { id: 13 });
        assert!(ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).is_empty(), "removing the only representation is what makes a document CC1-conformant");
        assert!(check_cc1_conformance(&snapshot).is_empty(), "and with the schema and the chain already right, nothing else is left to report");

        apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::SetProductIdentity { identity: None });
        assert!(!has_product_definition_chain(&snapshot.to_part21_document()));
        apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::SetFileSchema { schemas: vec!["IFC4".into()] });
        let diagnostics = check_cc1_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 2, "one hard FILE_SCHEMA violation and one soft product-chain warning: {diagnostics:?}");
    }

    /// 🚧️ CC1 owns no verb that writes a representation, and the shared ladder edit refuses one even
    /// if a caller reaches it directly — the class ceiling of 1 is below every real rung.
    #[test]
    fn no_representation_is_admissible_at_all() {
        let mut doc = base().to_part21_document();
        let row = ShapeRepresentationRow { type_name: "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION".into(), name: "w".into(), items: vec![], context: None };
        let refusal = ladder::apply_class_edit(&mut doc, CLASS, MAX_RUNG, &ClassEdit::Representation { id: 99, row: Some(row) }).expect_err("CC1 admits no representation");
        assert!(refusal.contains("rung 2") && refusal.contains("ceiling of 1"), "the refusal must name the class ceiling: {refusal}");
        assert!(ladder::ceiling_type_of(MAX_RUNG).is_none(), "and CC1 therefore has no ceiling type to demote onto either");
    }

    #[test]
    fn a_rejected_mutation_leaves_the_snapshot_untouched() {
        let mut snapshot = base();
        assert!(!apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::RemoveShapeRepresentation { id: 827 }).messages().is_empty(), "a conformance repair must never delete a product record");
        assert!(!apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::SetFileSchema { schemas: vec![] }).messages().is_empty());
        assert_eq!(snapshot, base());
    }

    #[test]
    fn no_mutation_is_the_identity() {
        let mut snapshot = base();
        assert!(apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::NoMutation).messages().is_empty());
        assert_eq!(snapshot, base());
    }

    /// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            StepCc1Mutation::NoMutation,
            StepCc1Mutation::SetSnapshot { snapshot: StepSnapshot::default() },
            StepCc1Mutation::SetFileSchema { schemas: Vec::new() },
            StepCc1Mutation::SetProductIdentity { identity: None },
            StepCc1Mutation::RemoveShapeRepresentation { id: 0 },
        ];
        assert_eq!(one_per_variant.len(), KINDS.len());
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(mutation.kind(), *kind);
        }
    }
}
//#endregion 🧪️Tests
