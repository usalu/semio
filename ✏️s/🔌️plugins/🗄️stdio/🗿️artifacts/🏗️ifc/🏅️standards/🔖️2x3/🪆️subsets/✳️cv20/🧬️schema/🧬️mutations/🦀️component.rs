//! 🧬️ `Ifc2x3Cv20Mutation` — Coordination View 2.0's OWN mutation vocabulary.
//!
//! 🎯️ This is deliberately NOT a copy of the `✳️any` subset's `Ifc2x3Mutation`. `✳️any` declares
//! generic ISO 10303-21 graph editing (`upsert-instance`, `remove-instance`, `set-header`) and knows
//! nothing about model view definitions; an MVD is a conformance FILTER over that one schema, so
//! its vocabulary is the set of edits that address the filter's own rules. Every kind below is one
//! rule of `check_cv20_conformance` (`../🦀️component.rs`'s `derived_analysis`), which is what makes
//! this a real distinction rather than an invented one:
//!
//! | kind | rule |
//! |---|---|
//! | `set-snapshot` | `CODE_FILE_SCHEMA` — the document must declare `IFC2X3` |
//! | `set-view-definition` | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `CoordinationView` |
//! | `set-structural-entity` | `CODE_STRUCTURAL_ENTITY` — CV2.0's architectural scope excludes structural-analysis entities |
//! | `set-project-units` | `CODE_PROJECT_UNITS` — `IfcProject.UnitsInContext` must resolve |
//! | `set-product-placement` | `CODE_PRODUCT_PLACEMENT` — a geometry-bearing product places through `IfcLocalPlacement` |
//!
//! Every concept kind carries an OPTIONAL payload — a value sets it, `None` clears it — so each is
//! total in both directions and `inverse()` is a REAL inverse read off the base rather than the
//! whole-snapshot restore `✳️any` degrades to.
//!
//! The `Ifc2x3Snapshot` type, the `Ifc2x3Diff` algebra and the generic per-instance vocabulary all
//! stay the `✳️any` subset's: a subset is a conformance marker, never a fork of the snapshot type.
//! `Ifc2x3Mutation` is re-exported below so `cv20::schema::mutations::Ifc2x3Mutation` — the path
//! this subset's editor and viewer already import — keeps resolving now that this module shadows
//! the glob re-export it used to arrive through.
//!
//! @see ../../../../🧬️mvd/🦀️component.rs — the Part-21 editing primitives the three MVD subsets share.
//! @see ../../🧪️oracle/🔣️component.json — the `ifc-2x3-cv20` catalog `KINDS` is checked against.

use crate::artifacts::ifc::standards::v2x3::mvd;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::Part21Value;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};

//#region 🔖️Vocabulary
/// 🚫️ Entity types Coordination View 2.0 excludes — the same list `check_cv20_conformance`
/// hard-faults on, reached through the analysis module rather than restated here.
use crate::artifacts::ifc::standards::v2x3::subsets::cv20::schema::derived_analysis::{FORBIDDEN_STRUCTURAL_TYPES, GEOMETRY_BEARING_PRODUCT_TYPES};

/// 📐️ `IfcProject.UnitsInContext` is attribute 9 of `IfcProject` (index 8).
const PROJECT_UNITS_INDEX: usize = 8;
/// 📐️ `IfcProduct.ObjectPlacement` is attribute 6 of every `IfcProduct` (index 5).
const PRODUCT_PLACEMENT_INDEX: usize = 5;

/// 🏗️ One structural-analysis entity Coordination View 2.0 excludes, as this vocabulary names it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cv20StructuralEntity {
    pub type_name: String,
    pub global_id: String,
    pub name: String,
}

/// 📐️ Typed Coordination View 2.0 mutation for `stdio.ifc.2x3`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Ifc2x3Cv20Mutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: Ifc2x3Snapshot,
    },
    SetViewDefinition {
        view: String,
    },
    SetStructuralEntity {
        id: u64,
        entity: Option<Cv20StructuralEntity>,
    },
    SetProjectUnits {
        project: u64,
        units: Option<u64>,
    },
    SetProductPlacement {
        product: u64,
        placement: Option<u64>,
    },
}

/// 📇️ Kebab-case spelling of every `Ifc2x3Cv20Mutation` variant, in declaration order — the
/// `ifc-2x3-cv20` catalog in `../../🧪️oracle/🔣️component.json` is required to match verbatim, and
/// `kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest (the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-view-definition", "set-structural-entity", "set-project-units", "set-product-placement"];

impl Ifc2x3Cv20Mutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the `ifc-2x3-cv20`
    /// catalog and the feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            Ifc2x3Cv20Mutation::NoMutation => "no-mutation",
            Ifc2x3Cv20Mutation::SetSnapshot { .. } => "set-snapshot",
            Ifc2x3Cv20Mutation::SetViewDefinition { .. } => "set-view-definition",
            Ifc2x3Cv20Mutation::SetStructuralEntity { .. } => "set-structural-entity",
            Ifc2x3Cv20Mutation::SetProjectUnits { .. } => "set-project-units",
            Ifc2x3Cv20Mutation::SetProductPlacement { .. } => "set-product-placement",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A mutation whose target does not exist, or names a concept the id does not carry, is reported as
/// an error message with an empty diff — never applied partially and never silently skipped.
pub fn apply_ifc2x3_cv20_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Cv20Mutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3Cv20Mutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

fn rejected(message: String) -> protocol::MutationOutcome<Ifc2x3Diff> {
    protocol::MutationOutcome::error("stdio.ifc.2x3.cv20.mutation-rejected", message, Vec::<String>::new())
}

fn edit(base: &Ifc2x3Snapshot, mutation: &Ifc2x3Cv20Mutation) -> Result<Ifc2x3Snapshot, String> {
    let mut next = base.clone();
    match mutation {
        Ifc2x3Cv20Mutation::NoMutation | Ifc2x3Cv20Mutation::SetSnapshot { .. } => {}
        Ifc2x3Cv20Mutation::SetViewDefinition { view } => mvd::set_view_definition(&mut next, view),
        Ifc2x3Cv20Mutation::SetStructuralEntity { id, entity } => match entity {
            None => mvd::remove_instance(&mut next, *id, FORBIDDEN_STRUCTURAL_TYPES)?,
            Some(entity) => {
                if !FORBIDDEN_STRUCTURAL_TYPES.iter().any(|forbidden| entity.type_name.eq_ignore_ascii_case(forbidden)) {
                    return Err(format!("{} is not one of the structural types Coordination View 2.0 excludes ({FORBIDDEN_STRUCTURAL_TYPES:?})", entity.type_name));
                }
                let args = vec![Part21Value::Str(entity.global_id.clone()), Part21Value::Unset, Part21Value::Str(entity.name.clone())];
                mvd::upsert_instance(&mut next, mvd::simple_instance(*id, &entity.type_name, args));
            }
        },
        Ifc2x3Cv20Mutation::SetProjectUnits { project, units } => {
            if let Some(id) = units {
                if next.document.instance(*id).is_none() {
                    return Err(format!("no instance #{id} to serve as the project's IfcUnitAssignment"));
                }
            }
            mvd::set_argument(&mut next, *project, &["IFCPROJECT"], PROJECT_UNITS_INDEX, mvd::optional(units.map(Part21Value::Ref)))?;
        }
        Ifc2x3Cv20Mutation::SetProductPlacement { product, placement } => {
            if let Some(id) = placement {
                let resolved = mvd::instance_type(&next, *id).unwrap_or("");
                if !resolved.eq_ignore_ascii_case("IFCLOCALPLACEMENT") {
                    return Err(format!("#{id} is {resolved:?}, not an IFCLOCALPLACEMENT -- Coordination View 2.0 places products through IfcLocalPlacement"));
                }
            }
            mvd::set_argument(&mut next, *product, GEOMETRY_BEARING_PRODUCT_TYPES, PRODUCT_PLACEMENT_INDEX, mvd::optional(placement.map(Part21Value::Ref)))?;
        }
    }
    Ok(next)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<Ifc2x3Snapshot> for Ifc2x3Cv20Mutation {
    type Diff = Ifc2x3Diff;

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Ifc2x3Cv20Mutation::NoMutation => protocol::MutationOutcome::new(Ifc2x3Diff::default()),
            Ifc2x3Cv20Mutation::SetSnapshot { snapshot } => match crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::validate_ifc2x3_snapshot(snapshot) {
                Ok(()) => protocol::MutationOutcome::new(Ifc2x3Diff::between(base, snapshot)),
                Err(message) => rejected(message),
            },
            _ => match edit(base, self) {
                Ok(next) => protocol::MutationOutcome::new(Ifc2x3Diff::between(base, &next)),
                Err(message) => rejected(message),
            },
        }
    }

    /// ↩️ A REAL inverse per concept, read off the base — not the whole-snapshot restore the `✳️any`
    /// subset degrades to. Every concept kind is total (a value sets it, `None` clears it), so the
    /// inverse of any edit is the same kind carrying whatever the base declared.
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Self> {
        match self {
            Ifc2x3Cv20Mutation::NoMutation => vec![Ifc2x3Cv20Mutation::NoMutation],
            Ifc2x3Cv20Mutation::SetSnapshot { .. } => vec![Ifc2x3Cv20Mutation::SetSnapshot { snapshot: base.clone() }],
            Ifc2x3Cv20Mutation::SetViewDefinition { .. } => vec![Ifc2x3Cv20Mutation::SetViewDefinition { view: mvd::view_definition_name(base).unwrap_or_default() }],
            Ifc2x3Cv20Mutation::SetStructuralEntity { id, .. } => {
                let entity = base.document.instance(*id).and_then(|instance| instance.primary()).map(|(name, args)| Cv20StructuralEntity {
                    type_name: name.to_string(),
                    global_id: args.first().and_then(Part21Value::as_str).unwrap_or_default().to_string(),
                    name: args.get(2).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
                });
                vec![Ifc2x3Cv20Mutation::SetStructuralEntity { id: *id, entity }]
            }
            Ifc2x3Cv20Mutation::SetProjectUnits { project, .. } => vec![Ifc2x3Cv20Mutation::SetProjectUnits { project: *project, units: mvd::reference_argument(base, *project, PROJECT_UNITS_INDEX) }],
            Ifc2x3Cv20Mutation::SetProductPlacement { product, .. } => vec![Ifc2x3Cv20Mutation::SetProductPlacement { product: *product, placement: mvd::reference_argument(base, *product, PRODUCT_PLACEMENT_INDEX) }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header};

    fn base() -> Ifc2x3Snapshot {
        let header = Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView_V2.0]".into())]), Part21Value::Str("2;1".into())],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        };
        let placement = mvd::simple_instance(10, "IFCLOCALPLACEMENT", vec![]);
        let units = mvd::simple_instance(20, "IFCUNITASSIGNMENT", vec![]);
        let project = mvd::simple_instance(
            1,
            "IFCPROJECT",
            vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("Project".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(20)],
        );
        let wall = mvd::simple_instance(2, "IFCWALL", vec![Part21Value::Str("guid2".into()), Part21Value::Unset, Part21Value::Str("Wall".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(10)]);
        Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![placement, units, project, wall] }, edm_preamble: None }
    }

    fn round_trip(mutation: Ifc2x3Cv20Mutation) {
        let start = base();
        let mut mutated = start.clone();
        let outcome = apply_ifc2x3_cv20_mutation(&mut mutated, &mutation);
        assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
        assert_ne!(mutated, start, "{mutation:?} changed nothing");
        let inverse = Mutation::inverse(&mutation, &start).into_iter().next().expect("one inverse");
        apply_ifc2x3_cv20_mutation(&mut mutated, &inverse);
        assert_eq!(mvd::canonical(&mutated), mvd::canonical(&start), "{mutation:?} then its inverse must restore the base exchange structure");
    }

    #[test]
    fn every_concept_kind_round_trips_through_its_own_inverse() {
        round_trip(Ifc2x3Cv20Mutation::SetViewDefinition { view: "StructuralAnalysisView".into() });
        round_trip(Ifc2x3Cv20Mutation::SetStructuralEntity { id: 99, entity: Some(Cv20StructuralEntity { type_name: "IFCSTRUCTURALANALYSISMODEL".into(), global_id: "probe".into(), name: "probe".into() }) });
        round_trip(Ifc2x3Cv20Mutation::SetProjectUnits { project: 1, units: None });
        round_trip(Ifc2x3Cv20Mutation::SetProductPlacement { product: 2, placement: None });
    }

    #[test]
    fn no_mutation_is_the_identity() {
        let start = base();
        let mut snapshot = start.clone();
        let outcome = apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::NoMutation);
        assert!(outcome.messages().is_empty());
        assert_eq!(snapshot, start);
    }

    #[test]
    fn the_mvd_guards_reject_rather_than_silently_edit() {
        let mut snapshot = base();
        assert!(!apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::SetProjectUnits { project: 20, units: Some(20) }).messages().is_empty(), "an IFCUNITASSIGNMENT is not an IFCPROJECT");
        assert!(!apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::SetProductPlacement { product: 2, placement: Some(1) }).messages().is_empty(), "an IFCPROJECT is not an IFCLOCALPLACEMENT");
        assert!(
            !apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::SetStructuralEntity { id: 99, entity: Some(Cv20StructuralEntity { type_name: "IFCWALL".into(), global_id: "x".into(), name: "x".into() }) }).messages().is_empty(),
            "IFCWALL is not a type CV2.0 excludes"
        );
        assert!(!apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::SetStructuralEntity { id: 2, entity: None }).messages().is_empty(), "clearing a structural entity must not delete a real wall");
        assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");
    }

    /// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
    /// The `ifc-2x3-cv20` catalog and the feature file are both checked against `KINDS`, so this is
    /// what keeps all three from drifting apart from the enum itself.
    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            Ifc2x3Cv20Mutation::NoMutation,
            Ifc2x3Cv20Mutation::SetSnapshot { snapshot: Ifc2x3Snapshot::default() },
            Ifc2x3Cv20Mutation::SetViewDefinition { view: String::new() },
            Ifc2x3Cv20Mutation::SetStructuralEntity { id: 0, entity: None },
            Ifc2x3Cv20Mutation::SetProjectUnits { project: 0, units: None },
            Ifc2x3Cv20Mutation::SetProductPlacement { product: 0, placement: None },
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(mutation.kind(), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
        }
    }
}
//#endregion 🧪️Tests
