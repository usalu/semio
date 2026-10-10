//! 🧬️ `Ifc2x3Cv20Mutation` — Coordination View 2.0's OWN mutation vocabulary.
//!
//! 🎯️ This is deliberately NOT a copy of the `✳️base` subset's `Ifc2x3Mutation`. `✳️base` declares
//! generic ISO 10303-21 graph editing (`upsert-instance`, `remove-instance`, `set-header`) and knows
//! nothing about model view definitions; an MVD is a conformance FILTER over that one schema, so
//! its vocabulary is the set of edits that address the filter's own rules. Every kind below is one
//! rule of `check_cv20_conformance` (`../🦀️.rs`'s `derived_analysis`), which is what makes
//! this a real distinction rather than an invented one:
//!
//! | kind | rule |
//! |---|---|
//! //! | `set-view-definition` | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `CoordinationView` |
//! | `set-structural-entity` | `CODE_STRUCTURAL_ENTITY` — CV2.0's architectural scope excludes structural-analysis entities |
//! | `set-project-units` | `CODE_PROJECT_UNITS` — `IfcProject.UnitsInContext` must resolve |
//! | `set-product-placement` | `CODE_PRODUCT_PLACEMENT` — a geometry-bearing product places through `IfcLocalPlacement` |
//!
//! Every concept kind carries an OPTIONAL payload — a value sets it, `None` clears it — so each is
//! total in both directions and `inverse()` is a REAL inverse read off the base rather than the
//! whole-snapshot restore `✳️base` degrades to.
//!
//! The `Ifc2x3Snapshot` type, the `Ifc2x3Diff` algebra and the generic per-instance vocabulary all
//! stay the `✳️base` subset's: a subset is a conformance marker, never a fork of the snapshot type.
//! `Ifc2x3Mutation` is re-exported below so `cv20::schema::mutations::Ifc2x3Mutation` — the path
//! this subset's editor and viewer already import — keeps resolving now that this module shadows
//! the glob re-export it used to arrive through.
//!
//! @see ../../../../🧬️mvd/🦀️.rs — the Part-21 editing primitives the three MVD subsets share.
//! @see ../../🔣️oracle.json — the `ifc-2x3-cv20` catalog `KINDS` is checked against.

use crate::standards::v2x3::mvd;
use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use protocol::Mutation;
use semio_s_artifact_stdio_contract::part21::Part21Value;

#[cfg(test)]
pub use crate::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation};
pub use crate::standards::v2x3::subsets::base::schema::mutations::{Ifc2x3Mutation};

//#region 🔖️Vocabulary
/// 🚫️ Entity types Coordination View 2.0 excludes — the same list `check_cv20_conformance`
/// hard-faults on, reached through the analysis module rather than restated here.
use crate::standards::v2x3::subsets::cv20::schema::{FORBIDDEN_STRUCTURAL_TYPES, GEOMETRY_BEARING_PRODUCT_TYPES};

/// 📐️ `IfcProject.UnitsInContext` is attribute 9 of `IfcProject` (index 8).
const PROJECT_UNITS_INDEX: usize = 8;
/// 📐️ `IfcProduct.ObjectPlacement` is attribute 6 of every `IfcProduct` (index 5).
const PRODUCT_PLACEMENT_INDEX: usize = 5;

/// 🏗️ One structural-analysis entity Coordination View 2.0 excludes, as this vocabulary names it.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Cv20StructuralEntity {
    pub type_name: String,
    pub global_id: String,
    pub name: String,
}

#[path = "📍️set-product-placement/🦀️.rs"]
pub mod set_product_placement;
#[path = "📐️set-project-units/🦀️.rs"]
pub mod set_project_units;
/// 📐️ Typed Coordination View 2.0 mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "🏗️set-structural-entity/🦀️.rs"]
pub mod set_structural_entity;
#[path = "👁️set-view-definition/🦀️.rs"]
pub mod set_view_definition;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3Cv20Mutation")]
pub enum Ifc2x3Cv20Mutation {
    SetViewDefinition(set_view_definition::SetViewDefinition),
    SetStructuralEntity(set_structural_entity::SetStructuralEntity),
    SetProjectUnits(set_project_units::SetProjectUnits),
    SetProductPlacement(set_product_placement::SetProductPlacement),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3Cv20Mutation` variant, in declaration order — the
/// `ifc-2x3-cv20` catalog in `../../🔣️oracle.json` is required to match verbatim, and
/// `kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest (the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["set-view-definition", "set-structural-entity", "set-project-units", "set-product-placement"];

impl Ifc2x3Cv20Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the `ifc-2x3-cv20`
    /// catalog and the feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            Ifc2x3Cv20Mutation::SetViewDefinition(_) => "set-view-definition",
            Ifc2x3Cv20Mutation::SetStructuralEntity(_) => "set-structural-entity",
            Ifc2x3Cv20Mutation::SetProjectUnits(_) => "set-project-units",
            Ifc2x3Cv20Mutation::SetProductPlacement(_) => "set-product-placement",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A mutation whose target does not exist, or names a concept the id does not carry, is reported as
/// an error message with an empty diff — never applied partially and never silently skipped.
#[cfg(test)]
pub fn apply_ifc2x3_cv20_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Cv20Mutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3Cv20Mutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

fn rejected(message: String) -> protocol::MutationOutcome<Ifc2x3Diff> {
    protocol::MutationOutcome::error("mutation.target-mismatch", message, Vec::<String>::new())
}

fn structural_entity_row(base: &Ifc2x3Snapshot, id: u64) -> Option<Cv20StructuralEntity> {
    base.document.instance(id).and_then(|instance| instance.primary()).map(|(name, args)| Cv20StructuralEntity {
        type_name: name.to_string(),
        global_id: args.first().and_then(Part21Value::as_str).unwrap_or_default().to_string(),
        name: args.get(2).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
    })
}
//#endregion 🔖️Apply


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
