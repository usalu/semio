//! 🧬️ `StepCc4Mutation` — ISO 10303-214 CC4 (manifold surfaces with topology)'s OWN mutation vocabulary.
//!
//! 🎯️ Deliberately NOT the `✳️any` subset's `StepMutation`. That one is the ISO 10303-21 GRAMMAR:
//! insert an entity, set an argument, remove an argument — eleven verbs that know nothing about
//! AP214 and would be identical for any Part-21 file on earth. A conformance class is not a grammar,
//! it is a FILTER, and the only edits that belong to it are the ones that move a document across the
//! filter. Every variant below is one rule of `check_cc4_conformance` (`../🦀️component.rs`'s
//! `derived_analysis`), and there are no others because that function reads no other axis:
//!
//! | kind | rule | code |
//! |---|---|---|
//! | `set-file-schema` | `FILE_SCHEMA` must declare `AUTOMOTIVE_DESIGN` | `CODE_FILE_SCHEMA` (hard) |
//! | `set-shape-representation` | no `*_SHAPE_REPRESENTATION` above rung 4 | `CODE_LADDER` (hard) |
//! | `demote-shape-representation` | the repair verb: rewrite an over-rung instance onto rung 4 | `CODE_LADDER` (hard) |
//! | `set-product-identity` | the `PRODUCT`/formation/definition chain | `CODE_PRODUCT_CHAIN` (soft) |
//!
//! 🪜️ **What this class admits, and what makes its vocabulary its own.** CC4 is the first class with TOPOLOGY: a manifold surface representation carries shells, and a shell
//! is a topological object, not a bag of surfaces. The ladder step CC4 refuses is the one to a solid —
//! `FACETED_BREP_SHAPE_REPRESENTATION` is a boundary representation of a VOLUME, and a class scoped to
//! surfaces has no conformant state that contains one.
//!
//! ⬇️ **The repair verb.** A demotion in CC4 turns a solid's boundary representation back into the manifold surface form.
//! The items survive the edit unchanged, which is the point: this class already understands shells,
//! so nothing an incoming rung-5 or rung-6 instance carries has to be discarded to fit.
//!
//! ⚠️ **A conformance class is not closed under inversion.** Undoing a ladder edit re-introduces
//! whatever the edit removed, and a class whose entire purpose is to forbid geometry above its own
//! ceiling cannot own a verb that writes geometry above its own ceiling back. `inverse()` therefore
//! returns the in-class verb whenever the base's own representation is admissible HERE, and degrades
//! to `SetSnapshot` when it is not — a real inverse either way (the projection is restored exactly),
//! but expressed through the only verb this class has for a document outside itself. That asymmetry
//! is recorded rather than papered over with a promotion verb this class must not have.
//!
//! @see ../../../✳️any/🚪️io/🪜️ladder/🦀️component.rs — the class-neutral edit implementations all six
//!      `✳️ccN` vocabularies route through, so each axis has ONE implementation and six callers.
//! @see ../🧪️oracle/🔣️component.json — the `step-ap214-cc4` catalog `KINDS` is checked against.

use crate::artifacts::step::schema::diff::StepDiff;
use crate::artifacts::step::standards::v_ap214::engine::ladder::{self, ClassEdit, ProductIdentity, ShapeRepresentationRow};
use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::MAX_RUNG;
use crate::artifacts::step::StepSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::step::standards::v_ap214::subsets::any::schema::mutations::{apply_step_mutation, StepMutation};

//#region 🔖️Vocabulary
/// 🏷️ How this class names itself in a rejection message.
const CLASS: &str = "ISO 10303-214 CC4 (manifold surfaces with topology)";

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/✳️cc4`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum StepCc4Mutation {
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
    /// 🪜️ Writes a representation this class admits, or — with `None` — deletes one. The rung guard
    /// is this class's own: `MANIFOLD_SURFACE_SHAPE_REPRESENTATION` passes, anything above it is refused with a message naming
    /// the rung and the ceiling.
    SetShapeRepresentation {
        id: u64,
        representation: Option<ShapeRepresentationRow>,
    },
    /// ⬇️ The conformance repair: rewrite an over-rung representation onto `MANIFOLD_SURFACE_SHAPE_REPRESENTATION`, keeping its
    /// name, its items and its context exactly. This is the verb that turns a non-conformant document
    /// into a conformant one without inventing or discarding geometry.
    DemoteShapeRepresentation {
        id: u64,
    },
}

/// 📇️ Kebab-case spelling of every `StepCc4Mutation` variant, in declaration order — the
/// `step-ap214-cc4` catalog in `../../🧪️oracle/🔣️component.json` must match verbatim.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-file-schema", "set-product-identity", "set-shape-representation", "demote-shape-representation"];

impl StepCc4Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the catalog and the
    /// feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            StepCc4Mutation::NoMutation => "no-mutation",
            StepCc4Mutation::SetSnapshot { .. } => "set-snapshot",
            StepCc4Mutation::SetFileSchema { .. } => "set-file-schema",
            StepCc4Mutation::SetProductIdentity { .. } => "set-product-identity",
            StepCc4Mutation::SetShapeRepresentation { .. } => "set-shape-representation",
            StepCc4Mutation::DemoteShapeRepresentation { .. } => "demote-shape-representation",
        }
    }

    /// 🎚️ This mutation as the class-neutral edit the shared ladder module performs, or `None` for
    /// the two whole-document verbs that address no conformance axis.
    fn class_edit(&self) -> Option<ClassEdit> {
        match self {
            StepCc4Mutation::NoMutation | StepCc4Mutation::SetSnapshot { .. } => None,
            StepCc4Mutation::SetFileSchema { schemas } => Some(ClassEdit::FileSchema { schemas: schemas.clone() }),
            StepCc4Mutation::SetProductIdentity { identity } => Some(ClassEdit::ProductIdentity { identity: identity.clone() }),
            StepCc4Mutation::SetShapeRepresentation { id, representation } => Some(ClassEdit::Representation { id: *id, row: representation.clone() }),
            StepCc4Mutation::DemoteShapeRepresentation { id } => Some(ClassEdit::Demotion { id: *id }),
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A rejected edit reports an error message with an empty diff and leaves the snapshot untouched —
/// never applied partially, never silently skipped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc4_mutation(snapshot: &mut StepSnapshot, mutation: &StepCc4Mutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepCc4Mutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
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
    protocol::MutationOutcome::error("stdio.step.cc4.mutation-rejected", message, Vec::<String>::new())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edited(base: &StepSnapshot, edit: &ClassEdit) -> Result<StepSnapshot, String> {
    let mut doc = base.to_part21_document();
    ladder::apply_class_edit(&mut doc, CLASS, MAX_RUNG, edit)?;
    Ok(StepSnapshot::from_part21_document(doc))
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<StepSnapshot> for StepCc4Mutation {
    type Diff = StepDiff;

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        match self.class_edit() {
            None => match self {
                StepCc4Mutation::SetSnapshot { snapshot } => protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, snapshot)),
                _ => protocol::MutationOutcome::new(StepDiff::default()),
            },
            Some(edit) => match edited(base, &edit) {
                Ok(next) => protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &next)),
                Err(message) => rejected(message),
            },
        }
    }

    /// ↩️ A real per-axis inverse read off the base wherever this class owns a verb for it, and an
    /// explicit whole-snapshot restore where it does not — see the module header for why the second
    /// case exists at all.
    fn inverse(&self, base: &StepSnapshot) -> Vec<Self> {
        let restore = || vec![StepCc4Mutation::SetSnapshot { snapshot: base.clone() }];
        match self.class_edit() {
            None => match self {
                StepCc4Mutation::SetSnapshot { .. } => restore(),
                _ => vec![StepCc4Mutation::NoMutation],
            },
            Some(edit) => match ladder::invert_class_edit(&base.to_part21_document(), MAX_RUNG, &edit) {
                Some(ClassEdit::FileSchema { schemas }) => vec![StepCc4Mutation::SetFileSchema { schemas }],
                Some(ClassEdit::ProductIdentity { identity }) => vec![StepCc4Mutation::SetProductIdentity { identity }],
                Some(ClassEdit::Representation { id, row }) => vec![StepCc4Mutation::SetShapeRepresentation { id, representation: row }],
                _ => restore(),
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region 🚪️Reachability
/// ▶️ [`apply_step_cc4_mutation`] in a signature that names only this subset's own public types, so
/// an external crate can drive the real production apply path and still SEE a rejection instead of
/// discarding it. `protocol` is a private `extern crate` alias in this plugin's glue, so nothing
/// outside the crate can name `protocol::MutationOutcome` or `protocol::Mutation` — without these
/// two wrappers a test host could only re-derive the semantics by hand and would then be testing its
/// own re-derivation. Same wall, same fix as the 🧿️semio ✳️kit subset's.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_cc4_mutation_checked(snapshot: &mut StepSnapshot, mutation: &StepCc4Mutation) -> Result<(), String> {
    let outcome = apply_step_cc4_mutation(snapshot, mutation);
    match outcome.messages().first() {
        None => Ok(()),
        Some(message) => Err(format!("{:?} was rejected: [{}] {}", mutation, message.code.0, message.message)),
    }
}

/// ↩️ `Mutation::inverse` for `StepCc4Mutation`, reachable without naming the `protocol` alias — the
/// production inverse itself, never a copy of its rules.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_step_cc4_mutation(base: &StepSnapshot, mutation: &StepCc4Mutation) -> Vec<StepCc4Mutation> {
    <StepCc4Mutation as Mutation<StepSnapshot>>::inverse(mutation, base)
}
//#endregion 🚪️Reachability

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::standards::v_ap214::engine::ladder::{has_product_definition_chain, ladder_violations, shape_representation_row};
    use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance;

    /// 🧫️ The shape of this artifact's own committed fixture, cut down to what a conformance class
    /// reads: the real `AUTOMOTIVE_DESIGN` declaration, the real `#821`/`#822`/`#827` product chain
    /// (formation as the ISO 10303-41 SUBTYPE a real exporter writes) and the real rung-6 `#13`.
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

    /// 🧫️ The same document with `#13` already brought inside this class, so an inverse that is
    /// expressible in-class has something in-class to restore.
    fn conforming() -> StepSnapshot {
        let mut doc = base().to_part21_document();
        ladder::demote_shape_representation(&mut doc, 13, "MANIFOLD_SURFACE_SHAPE_REPRESENTATION").expect("the base carries a real representation");
        StepSnapshot::from_part21_document(doc)
    }

    fn round_trip(start: StepSnapshot, mutation: StepCc4Mutation) {
        let mut mutated = start.clone();
        let outcome = apply_step_cc4_mutation(&mut mutated, &mutation);
        assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
        assert_ne!(mutated, start, "{mutation:?} changed nothing -- a mutation that is not observable proves nothing");
        for step in Mutation::inverse(&mutation, &start) {
            apply_step_cc4_mutation(&mut mutated, &step);
        }
        assert_eq!(mutated, start, "{mutation:?} then its inverse must restore the base");
    }

    #[test]
    fn every_conformance_axis_round_trips_through_its_own_inverse() {
        round_trip(base(), StepCc4Mutation::SetFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] });
        round_trip(base(), StepCc4Mutation::SetProductIdentity { identity: None });
        round_trip(base(), StepCc4Mutation::SetShapeRepresentation { id: 13, representation: None });
        round_trip(conforming(), StepCc4Mutation::SetShapeRepresentation { id: 13, representation: None });
        round_trip(base(), StepCc4Mutation::DemoteShapeRepresentation { id: 13 });
    }

    /// 🪜️ The guard that IS this class: the ceiling type is admitted, the type one rung above it is
    /// refused, and the refusal names both rungs instead of silently doing nothing.
    #[test]
    fn the_class_ceiling_is_the_line_this_vocabulary_draws() {
        let mut snapshot = conforming();
        let at_ceiling = shape_representation_row(&snapshot.to_part21_document(), 13).expect("a representation");
        assert_eq!(at_ceiling.type_name, "MANIFOLD_SURFACE_SHAPE_REPRESENTATION");
        assert!(apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::SetShapeRepresentation { id: 13, representation: Some(at_ceiling) }).messages().is_empty(), "this class admits its own ceiling type");

        let above = ShapeRepresentationRow { type_name: "FACETED_BREP_SHAPE_REPRESENTATION".into(), name: "too high".into(), items: vec![12], context: Some(835) };
        let outcome = apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::SetShapeRepresentation { id: 13, representation: Some(above) });
        let message = &outcome.messages().first().expect("rung 5 is above this class").message;
        assert!(message.contains("rung 5") && message.contains("ceiling of 4"), "the refusal must name both rungs: {message}");
        assert_eq!(snapshot, conforming(), "a rejected mutation leaves the snapshot untouched");
    }

    /// 🎯️ Each verb must move the diagnostic it was derived from, or it is not that rule's verb.
    #[test]
    fn each_verb_moves_the_diagnostic_it_was_derived_from() {
        let mut snapshot = base();
        assert_eq!(ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).len(), 1, "the base's rung-6 representation is above this class's ceiling");
        apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::DemoteShapeRepresentation { id: 13 });
        assert!(ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).is_empty(), "demoting the over-rung representation is what makes this document conformant");
        assert!(check_cc4_conformance(&snapshot).is_empty(), "and with the schema and the chain already right, nothing else is left to report");

        apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::SetProductIdentity { identity: None });
        assert!(!has_product_definition_chain(&snapshot.to_part21_document()));
        apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::SetFileSchema { schemas: vec!["IFC4".into()] });
        let diagnostics = check_cc4_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 2, "one hard FILE_SCHEMA violation and one soft product-chain warning: {diagnostics:?}");
    }

    #[test]
    fn a_rejected_mutation_leaves_the_snapshot_untouched() {
        let mut snapshot = base();
        assert!(!apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::SetShapeRepresentation { id: 827, representation: None }).messages().is_empty(), "a conformance repair must never delete a product record");
        assert!(!apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::SetFileSchema { schemas: vec![] }).messages().is_empty());
        assert_eq!(snapshot, base());
    }

    #[test]
    fn no_mutation_is_the_identity() {
        let mut snapshot = base();
        assert!(apply_step_cc4_mutation(&mut snapshot, &StepCc4Mutation::NoMutation).messages().is_empty());
        assert_eq!(snapshot, base());
    }

    /// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            StepCc4Mutation::NoMutation,
            StepCc4Mutation::SetSnapshot { snapshot: StepSnapshot::default() },
            StepCc4Mutation::SetFileSchema { schemas: Vec::new() },
            StepCc4Mutation::SetProductIdentity { identity: None },
            StepCc4Mutation::SetShapeRepresentation { id: 0, representation: None },
            StepCc4Mutation::DemoteShapeRepresentation { id: 0 },
        ];
        assert_eq!(one_per_variant.len(), KINDS.len());
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(mutation.kind(), *kind);
        }
    }
}
//#endregion 🧪️Tests
