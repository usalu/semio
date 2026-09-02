//! 🧬️ `StepCc6Mutation` — ISO 10303-214 CC6 (advanced B-Rep, top of the ladder)'s OWN mutation
//! vocabulary.
//!
//! 🎯️ Deliberately NOT the `✳️base` subset's `StepMutation`. That one is the ISO 10303-21 GRAMMAR:
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
//! * There is no `demote-shape-representation` here. `✳️cc2`..`✳️cc5` carry that verb because each
//!   of them has instances above its ceiling to bring down; CC6 has none, so a demotion verb would
//!   be a kind that can never move the projection — a scenario that always passes and proves
//!   nothing. It is absent for the same reason `✳️cc1`'s `set-shape-representation` is absent: the
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
//! ⚠️ **A conformance class is not closed under inversion** — except, uniquely, this one. Because no
//! representation is above CC6's ceiling, the base's own representation is always admissible here,
//! so `inverse()` never has to degrade a ladder edit to `SetSnapshot`. The degradation path is
//! written anyway, because the shared inversion it routes through is class-neutral and a class must
//! not assume its own reachability argument; it is documented as unreachable rather than deleted.
//!
//! @see ../../../✳️base/🚪️io/🪜️ladder/🦀️.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🔣️oracle.json — the `step-ap214-cc6` catalog `KINDS` is checked against.

use crate::artifacts::step::schema::diff::StepDiff;
use crate::artifacts::step::standards::v_ap214::engine::ladder::{self, ClassEdit, ProductIdentity, ShapeRepresentationRow};
use crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::MAX_RUNG;
use crate::artifacts::step::StepSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;

pub use crate::artifacts::step::standards::v_ap214::subsets::any::schema::mutations::{apply_step_mutation, StepMutation};

//#region 🔖️Vocabulary
/// 🏷️ How this class names itself in a rejection message.
const CLASS: &str = "ISO 10303-214 CC6 (advanced B-Rep, top of the ladder)";

//#region 🔖️Leaves
#[path = "📋set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷set-file-schema/🦀️.rs"]
pub mod set_file_schema;
#[path = "🪪set-product-identity/🦀️.rs"]
pub mod set_product_identity;
#[path = "🪜set-shape-representation/🦀️.rs"]
pub mod set_shape_representation;
//#endregion 🔖️Leaves

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/✳️cc6`.
///
/// ⚠️ `NoMutation` is GONE — `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
/// leaf payload and a unit variant wraps none. Its only role was `inverse()`'s "nothing to undo" arm,
/// now the empty vector. `SetSnapshot` is KEPT: the derive checks `SEMANTICS.verb`, not the kind, and
/// `set` is approved — so this class's whole-document restore survives intact.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.cc6")]
pub enum StepCc6Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetFileSchema(set_file_schema::SetFileSchema),
    SetProductIdentity(set_product_identity::SetProductIdentity),
    SetShapeRepresentation(set_shape_representation::SetShapeRepresentation),
}

/// 📇️ Kebab-case spelling of every `StepCc6Mutation` variant, in declaration order — the
/// `step-ap214-cc6` catalog in `../../🔣️oracle.json` must match verbatim. Five kinds,
/// not six: see the module header for why a demotion verb would be unobservable at this ceiling.
pub const KINDS: &[&str] = &["set-snapshot", "set-file-schema", "set-product-identity", "set-shape-representation"];

impl StepCc6Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc6Mutation::SetSnapshot(_) => "set-snapshot",
            StepCc6Mutation::SetFileSchema(_) => "set-file-schema",
            StepCc6Mutation::SetProductIdentity(_) => "set-product-identity",
            StepCc6Mutation::SetShapeRepresentation(_) => "set-shape-representation",
        }
    }


}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A rejected edit reports an error message with an empty diff and leaves the snapshot untouched —
/// never applied partially, never silently skipped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc6_mutation(snapshot: &mut StepSnapshot, mutation: &StepCc6Mutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepCc6Mutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
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
    protocol::MutationOutcome::error("stdio.step.cc6.mutation-rejected", message, Vec::<String>::new())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn edited(base: &StepSnapshot, edit: &ClassEdit) -> Result<StepSnapshot, String> {
    let mut doc = base.to_part21_document();
    ladder::apply_class_edit(&mut doc, CLASS, MAX_RUNG, edit)?;
    Ok(StepSnapshot::from_part21_document(doc))
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
pub(crate) fn class_inverse(base: &StepSnapshot, edit: &ClassEdit) -> Vec<StepCc6Mutation> {
    match ladder::invert_class_edit(&base.to_part21_document(), MAX_RUNG, edit) {
        Some(ClassEdit::FileSchema { schemas }) => vec![StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas })],
        Some(ClassEdit::ProductIdentity { identity }) => vec![StepCc6Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity })],
        Some(ClassEdit::Representation { id, row }) => vec![StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id, representation: row })],
        _ => vec![StepCc6Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
    }
}
//#endregion 🔖️ClassEdit


//#region 🚪️Reachability
/// ▶️ [`apply_step_cc6_mutation`] in a signature that names only this subset's own public types, so
/// an external crate can drive the real production apply path and still SEE a rejection instead of
/// discarding it. `protocol` is a private `extern crate` alias in this plugin's glue, so nothing
/// outside the crate can name `protocol::MutationOutcome` or `protocol::Mutation` — without these
/// two wrappers a test host could only re-derive the semantics by hand and would then be testing its
/// own re-derivation. Same wall, same fix as the 🧿️semio ✳️kit subset's.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc6_mutation_checked(snapshot: &mut StepSnapshot, mutation: &StepCc6Mutation) -> Result<(), String> {
    let outcome = apply_step_cc6_mutation(snapshot, mutation);
    match outcome.messages().first() {
        None => Ok(()),
        Some(message) => Err(format!("{:?} was rejected: [{}] {}", mutation, message.code.0, message.message)),
    }
}

/// ↩️ `Mutation::inverse` for `StepCc6Mutation`, reachable without naming the `protocol` alias — the
/// production inverse itself, never a copy of its rules.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_step_cc6_mutation(base: &StepSnapshot, mutation: &StepCc6Mutation) -> Vec<StepCc6Mutation> {
    <StepCc6Mutation as Mutation<StepSnapshot>>::inverse(mutation, base)
}
//#endregion 🚪️Reachability

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::standards::v_ap214::engine::ladder::{ceiling_type_of, has_product_definition_chain, ladder_rung_of, ladder_violations, shape_representation_row};
    use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    use crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance;

    /// 🧫️ The shape of this artifact's own committed fixture, cut down to what a conformance class
    /// reads: the real `AUTOMOTIVE_DESIGN` declaration, the real `#821`/`#822`/`#827` product chain
    /// (formation as the ISO 10303-41 SUBTYPE a real exporter writes) and the real rung-6 `#13`.
    /// Unlike every other class's, this base already CONFORMS.
    fn base() -> StepSnapshot {
        StepSnapshot::from_part21_document(Part21Document {
            header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
            instances: vec![
                Part21Instance { id: 13, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![Part21Value::Str("brep_rep_0".into()), Part21Value::List(vec![Part21Value::Ref(12), Part21Value::Ref(895)]), Part21Value::Ref(835)])] },
                Part21Instance { id: 821, entities: vec![("PRODUCT_DEFINITION".into(), vec![Part21Value::Str("A".into())])] },
                Part21Instance { id: 822, entities: vec![("PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE".into(), vec![Part21Value::Str("A".into())])] },
                Part21Instance { id: 827, entities: vec![("PRODUCT".into(), vec![Part21Value::Str("Document".into())])] },
            ],
        })
    }

    fn round_trip(mutation: StepCc6Mutation) {
        let start = base();
        let mut mutated = start.clone();
        let outcome = apply_step_cc6_mutation(&mut mutated, &mutation);
        assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
        assert_ne!(mutated, start, "{mutation:?} changed nothing -- a mutation that is not observable proves nothing");
        for step in Mutation::inverse(&mutation, &start) {
            apply_step_cc6_mutation(&mut mutated, &step);
        }
        assert_eq!(mutated, start, "{mutation:?} then its inverse must restore the base");
    }

    #[test]
    fn every_conformance_axis_round_trips_through_its_own_inverse() {
        round_trip(StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] }));
        round_trip(StepCc6Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }));
        round_trip(StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: None }));
    }

    /// 🪜️ The claim this class's whole vocabulary rests on: the ladder tops out at 6, so every type
    /// it can classify is admissible here and no demotion verb could ever have work to do.
    #[test]
    fn the_top_of_the_ladder_admits_every_classified_rung() {
        for rung in 2..=6u8 {
            let ceiling = ceiling_type_of(rung).expect("each geometry class names a type");
            assert!(ladder_rung_of(ceiling).is_some_and(|found| found <= MAX_RUNG), "{ceiling} must be admissible at the top of the ladder");
        }
        assert!(ladder_violations(&base().to_part21_document(), MAX_RUNG).is_empty(), "the real fixture's rung-6 representation sits exactly on this ceiling");
        assert!(check_cc6_conformance(&base()).is_empty(), "and this is the one class the committed fixture already conforms to");
    }

    /// 🚧️ The one refusal CC6 can genuinely make: a type that is not on the ladder at all. Asserting
    /// a rung above 6 would be asserting a rung that does not exist.
    #[test]
    fn a_type_that_is_not_on_the_ladder_is_still_refused() {
        let mut snapshot = base();
        let off_ladder = ShapeRepresentationRow { type_name: "MANIFOLD_SOLID_BREP".into(), name: "not a representation".into(), items: vec![12], context: Some(835) };
        let outcome = apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: Some(off_ladder) }));
        let message = &outcome.messages().first().expect("MANIFOLD_SOLID_BREP is a solid, not a representation").message;
        assert!(message.contains("not a *_SHAPE_REPRESENTATION type"), "the refusal must say what is wrong: {message}");
        assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");

        let at_ceiling = shape_representation_row(&base().to_part21_document(), 13).expect("a representation");
        assert!(apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: Some(at_ceiling) })).messages().is_empty(), "and the fixture's own rung-6 type is admitted");
    }

    /// 🎯️ Each verb must move the diagnostic it was derived from, or it is not that rule's verb.
    /// CC6's ladder verb is the exception that proves the rule: the base already conforms, so the
    /// observable move is the REMOVAL of the representation, not a repair of it.
    #[test]
    fn each_verb_moves_the_diagnostic_it_was_derived_from() {
        let mut snapshot = base();
        assert!(check_cc6_conformance(&snapshot).is_empty());

        apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: None }));
        assert!(shape_representation_row(&snapshot.to_part21_document(), 13).is_none(), "the ladder verb really deleted the representation");
        assert!(check_cc6_conformance(&snapshot).is_empty(), "a document with no representation at all still conforms to CC6 -- the class sets a ceiling, not a floor");

        apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }));
        assert!(!has_product_definition_chain(&snapshot.to_part21_document()));
        apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["IFC4".into()] }));
        let diagnostics = check_cc6_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 2, "one hard FILE_SCHEMA violation and one soft product-chain warning: {diagnostics:?}");
    }

    #[test]
    fn a_rejected_mutation_leaves_the_snapshot_untouched() {
        let mut snapshot = base();
        assert!(!apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 827, representation: None })).messages().is_empty(), "a conformance repair must never delete a product record");
        assert!(!apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec![] })).messages().is_empty());
        assert_eq!(snapshot, base());
    }

    /// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            StepCc6Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: StepSnapshot::default() }),
            StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: Vec::new() }),
            StepCc6Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }),
            StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 0, representation: None }),
        ];
        assert_eq!(one_per_variant.len(), KINDS.len());
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(mutation.kind(), *kind);
        }
    }
}
//#endregion 🧪️Tests
