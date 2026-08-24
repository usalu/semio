//! 🧬️ `Ifc2x3CobieMutation` — Basic FM Handover's OWN mutation vocabulary (the view that carries
//! COBie 2.4).
//!
//! 🎯️ This is deliberately NOT a copy of the `✳️any` subset's `Ifc2x3Mutation`. `✳️any` declares
//! generic ISO 10303-21 graph editing (`upsert-instance`, `remove-instance`, `set-header`) and knows
//! nothing about model view definitions; an MVD is a conformance FILTER over that one schema, so its
//! vocabulary is the set of edits that address the filter's own rules. Every kind below is one COBie
//! handover sheet, taken from `check_cobie_conformance` (`../🦀️component.rs`'s `derived_analysis`):
//!
//! | kind | COBie sheet | rule |
//! |---|---|---|
//! | `set-snapshot` | — | `CODE_FILE_SCHEMA` |
//! | `set-view-definition` | — | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `FMHandOverView` |
//! | `set-facility-name` | Facility | `CODE_BUILDING_STOREY` — the handover needs a named `IfcBuilding` |
//! | `set-floor-elevation` | Floor | `CODE_BUILDING_STOREY` — a Floor row is an `IfcBuildingStorey` with an elevation |
//! | `set-space` | Space | `CODE_SPACE_NAME` — the Space sheet is keyed by a non-empty `IfcSpace.Name` |
//! | `set-type-assignment` | Type | `CODE_TYPE_ASSIGNMENT` — maintainable products relate to a type through `IfcRelDefinesByType` |
//!
//! Every sheet kind carries an OPTIONAL payload — a value sets the row, `None` clears it — so each
//! is total in both directions and `inverse()` is a REAL inverse read off the base rather than the
//! whole-snapshot restore `✳️any` degrades to.
//!
//! The `Ifc2x3Snapshot` type, the `Ifc2x3Diff` algebra and the generic per-instance vocabulary all
//! stay the `✳️any` subset's: a subset is a conformance marker, never a fork of the snapshot type.
//! `Ifc2x3Mutation` is re-exported below so `cobie::schema::mutations::Ifc2x3Mutation` — the path
//! this subset's editor and viewer already import — keeps resolving now that this module shadows the
//! glob re-export it used to arrive through.
//!
//! @see ../../../../🧬️mvd/🦀️component.rs — the Part-21 editing primitives the three MVD subsets share.
//! @see ../../🧪️oracle/🔣️component.json — the `ifc-2x3-cobie` catalog `KINDS` is checked against.

use crate::artifacts::ifc::standards::v2x3::mvd;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::Part21Value;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};

//#region 🔖️Vocabulary
/// 📐️ `IfcRoot.Name` is attribute 3 of every rooted entity (index 2) — COBie's key column.
const NAME_INDEX: usize = 2;
/// 📐️ `IfcSpace.LongName` is attribute 8 (index 7).
const SPACE_LONG_NAME_INDEX: usize = 7;
/// 📐️ `IfcBuildingStorey.Elevation` is attribute 10 (index 9).
const STOREY_ELEVATION_INDEX: usize = 9;
/// 📐️ `IfcProduct.ObjectPlacement` is attribute 6 (index 5).
const PRODUCT_PLACEMENT_INDEX: usize = 5;
/// 📐️ `IfcRelDefinesByType.RelatedObjects` is attribute 5 (index 4).
const RELATED_OBJECTS_INDEX: usize = 4;
/// 📐️ `IfcRelDefinesByType.RelatingType` is attribute 6 (index 5).
const RELATING_TYPE_INDEX: usize = 5;
/// 📐️ `IfcRoot.OwnerHistory` is attribute 2 (index 1).
const OWNER_HISTORY_INDEX: usize = 1;

/// 🏷️ The entity a COBie Space sheet row is.
const SPACE: &str = "IFCSPACE";
/// 🏷️ The entity a COBie Facility sheet row is.
const BUILDING: &str = "IFCBUILDING";
/// 🏷️ The entity a COBie Floor sheet row is.
const STOREY: &str = "IFCBUILDINGSTOREY";
/// 🏷️ The relationship COBie's Type sheet is built from.
const TYPE_ASSIGNMENT: &str = "IFCRELDEFINESBYTYPE";

/// 🏠️ One COBie Space sheet row.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobieSpaceRow {
    pub global_id: String,
    pub name: String,
    /// 📍️ The real `IfcLocalPlacement` the space sits in — a handover space is placed in the real
    /// spatial structure, never floating.
    pub placement: u64,
}

/// 🔗️ One COBie Type sheet linkage: an `IfcRelDefinesByType` relating maintainable products to a
/// real `IFC*TYPE`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobieTypeAssignment {
    pub global_id: String,
    pub owner_history: Option<u64>,
    pub related_objects: Vec<u64>,
    pub relating_type: u64,
}

/// 📐️ Typed Basic FM Handover mutation for `stdio.ifc.2x3`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Ifc2x3CobieMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: Ifc2x3Snapshot,
    },
    SetViewDefinition {
        view: String,
    },
    SetFacilityName {
        building: u64,
        name: Option<String>,
    },
    SetFloorElevation {
        storey: u64,
        elevation: Option<f64>,
    },
    SetSpace {
        id: u64,
        space: Option<CobieSpaceRow>,
    },
    SetTypeAssignment {
        id: u64,
        assignment: Option<CobieTypeAssignment>,
    },
}

/// 📇️ Kebab-case spelling of every `Ifc2x3CobieMutation` variant, in declaration order — the
/// `ifc-2x3-cobie` catalog in `../../🧪️oracle/🔣️component.json` is required to match verbatim.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-view-definition", "set-facility-name", "set-floor-elevation", "set-space", "set-type-assignment"];

impl Ifc2x3CobieMutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the `ifc-2x3-cobie`
    /// catalog and the feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            Ifc2x3CobieMutation::NoMutation => "no-mutation",
            Ifc2x3CobieMutation::SetSnapshot { .. } => "set-snapshot",
            Ifc2x3CobieMutation::SetViewDefinition { .. } => "set-view-definition",
            Ifc2x3CobieMutation::SetFacilityName { .. } => "set-facility-name",
            Ifc2x3CobieMutation::SetFloorElevation { .. } => "set-floor-elevation",
            Ifc2x3CobieMutation::SetSpace { .. } => "set-space",
            Ifc2x3CobieMutation::SetTypeAssignment { .. } => "set-type-assignment",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A mutation whose target does not exist, or names a sheet the id does not carry, is reported as an
/// error message with an empty diff — never applied partially and never silently skipped.
pub fn apply_ifc2x3_cobie_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3CobieMutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3CobieMutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

fn rejected(message: String) -> protocol::MutationOutcome<Ifc2x3Diff> {
    protocol::MutationOutcome::error("stdio.ifc.2x3.cobie.mutation-rejected", message, Vec::<String>::new())
}

fn space_args(row: &CobieSpaceRow) -> Vec<Part21Value> {
    vec![
        Part21Value::Str(row.global_id.clone()),
        Part21Value::Unset,
        Part21Value::Str(row.name.clone()),
        Part21Value::Unset,
        Part21Value::Unset,
        Part21Value::Ref(row.placement),
        Part21Value::Unset,
        Part21Value::Str(row.name.clone()),
        Part21Value::Enum("ELEMENT".into()),
        Part21Value::Enum("INTERNAL".into()),
        Part21Value::Unset,
    ]
}

fn type_assignment_args(row: &CobieTypeAssignment) -> Vec<Part21Value> {
    vec![
        Part21Value::Str(row.global_id.clone()),
        mvd::optional(row.owner_history.map(Part21Value::Ref)),
        Part21Value::Unset,
        Part21Value::Unset,
        mvd::reference_list(&row.related_objects),
        Part21Value::Ref(row.relating_type),
    ]
}

fn edit(base: &Ifc2x3Snapshot, mutation: &Ifc2x3CobieMutation) -> Result<Ifc2x3Snapshot, String> {
    let mut next = base.clone();
    match mutation {
        Ifc2x3CobieMutation::NoMutation | Ifc2x3CobieMutation::SetSnapshot { .. } => {}
        Ifc2x3CobieMutation::SetViewDefinition { view } => mvd::set_view_definition(&mut next, view),
        Ifc2x3CobieMutation::SetFacilityName { building, name } => {
            mvd::set_argument(&mut next, *building, &[BUILDING], NAME_INDEX, mvd::optional(name.clone().map(Part21Value::Str)))?;
        }
        Ifc2x3CobieMutation::SetFloorElevation { storey, elevation } => {
            mvd::set_argument(&mut next, *storey, &[STOREY], STOREY_ELEVATION_INDEX, mvd::optional(elevation.map(|value| Part21Value::Real(value.into()))))?;
        }
        Ifc2x3CobieMutation::SetSpace { id, space } => match space {
            None => mvd::remove_instance(&mut next, *id, &[SPACE])?,
            Some(row) => {
                if row.name.trim().is_empty() {
                    return Err("COBie's Space sheet is keyed by name -- an IFCSPACE with a blank Name is not a handover row".into());
                }
                let placement = mvd::instance_type(&next, row.placement).unwrap_or("");
                if !placement.eq_ignore_ascii_case("IFCLOCALPLACEMENT") {
                    return Err(format!("#{} is {placement:?}, not an IFCLOCALPLACEMENT -- a handover space is placed in the real spatial structure", row.placement));
                }
                mvd::upsert_instance(&mut next, mvd::simple_instance(*id, SPACE, space_args(row)));
            }
        },
        Ifc2x3CobieMutation::SetTypeAssignment { id, assignment } => match assignment {
            None => mvd::remove_instance(&mut next, *id, &[TYPE_ASSIGNMENT])?,
            Some(row) => {
                if !mvd::instance_type(&next, row.relating_type).unwrap_or("").to_ascii_uppercase().ends_with("TYPE") {
                    return Err(format!("#{} is not an IFC*TYPE -- COBie's Type sheet relates maintainable products to a real type", row.relating_type));
                }
                if row.related_objects.is_empty() {
                    return Err("an IFCRELDEFINESBYTYPE with no RelatedObjects assigns nothing".into());
                }
                for object in &row.related_objects {
                    if next.document.instance(*object).is_none() {
                        return Err(format!("no instance #{object} to relate to the type"));
                    }
                }
                mvd::upsert_instance(&mut next, mvd::simple_instance(*id, TYPE_ASSIGNMENT, type_assignment_args(row)));
            }
        },
    }
    Ok(next)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<Ifc2x3Snapshot> for Ifc2x3CobieMutation {
    type Diff = Ifc2x3Diff;

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Ifc2x3CobieMutation::NoMutation => protocol::MutationOutcome::new(Ifc2x3Diff::default()),
            Ifc2x3CobieMutation::SetSnapshot { snapshot } => match crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::validate_ifc2x3_snapshot(snapshot) {
                Ok(()) => protocol::MutationOutcome::new(Ifc2x3Diff::between(base, snapshot)),
                Err(message) => rejected(message),
            },
            _ => match edit(base, self) {
                Ok(next) => protocol::MutationOutcome::new(Ifc2x3Diff::between(base, &next)),
                Err(message) => rejected(message),
            },
        }
    }

    /// ↩️ A REAL inverse per sheet, read off the base — not the whole-snapshot restore the `✳️any`
    /// subset degrades to. Every sheet kind is total (a value sets the row, `None` clears it), so
    /// the inverse of any edit is the same kind carrying whatever the base held.
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Self> {
        match self {
            Ifc2x3CobieMutation::NoMutation => vec![Ifc2x3CobieMutation::NoMutation],
            Ifc2x3CobieMutation::SetSnapshot { .. } => vec![Ifc2x3CobieMutation::SetSnapshot { snapshot: base.clone() }],
            Ifc2x3CobieMutation::SetViewDefinition { .. } => vec![Ifc2x3CobieMutation::SetViewDefinition { view: mvd::view_definition_name(base).unwrap_or_default() }],
            Ifc2x3CobieMutation::SetFacilityName { building, .. } => {
                vec![Ifc2x3CobieMutation::SetFacilityName { building: *building, name: mvd::argument(base, *building, NAME_INDEX).and_then(Part21Value::as_str).map(str::to_string) }]
            }
            Ifc2x3CobieMutation::SetFloorElevation { storey, .. } => {
                vec![Ifc2x3CobieMutation::SetFloorElevation { storey: *storey, elevation: mvd::argument(base, *storey, STOREY_ELEVATION_INDEX).and_then(Part21Value::as_real) }]
            }
            Ifc2x3CobieMutation::SetSpace { id, .. } => {
                let space = base.document.instance(*id).filter(|instance| instance.is_type(SPACE)).map(|instance| CobieSpaceRow {
                    global_id: mvd::argument(base, *id, 0).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
                    name: mvd::argument(base, *id, NAME_INDEX)
                        .and_then(Part21Value::as_str)
                        .or_else(|| mvd::argument(base, *id, SPACE_LONG_NAME_INDEX).and_then(Part21Value::as_str))
                        .unwrap_or_default()
                        .to_string(),
                    placement: mvd::reference_argument(base, instance.id, PRODUCT_PLACEMENT_INDEX).unwrap_or_default(),
                });
                vec![Ifc2x3CobieMutation::SetSpace { id: *id, space }]
            }
            Ifc2x3CobieMutation::SetTypeAssignment { id, .. } => {
                let assignment = base.document.instance(*id).filter(|instance| instance.is_type(TYPE_ASSIGNMENT)).map(|_| CobieTypeAssignment {
                    global_id: mvd::argument(base, *id, 0).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
                    owner_history: mvd::reference_argument(base, *id, OWNER_HISTORY_INDEX),
                    related_objects: mvd::reference_list_ids(mvd::argument(base, *id, RELATED_OBJECTS_INDEX)),
                    relating_type: mvd::reference_argument(base, *id, RELATING_TYPE_INDEX).unwrap_or_default(),
                });
                vec![Ifc2x3CobieMutation::SetTypeAssignment { id: *id, assignment }]
            }
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
        let building = mvd::simple_instance(1, BUILDING, vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("".into())]);
        let storey = mvd::simple_instance(
            2,
            STOREY,
            vec![
                Part21Value::Str("guid2".into()),
                Part21Value::Unset,
                Part21Value::Str("Street level".into()),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Ref(10),
                Part21Value::Unset,
                Part21Value::Str("Street level".into()),
                Part21Value::Enum("ELEMENT".into()),
                Part21Value::Real(0.0.into()),
            ],
        );
        let wall = mvd::simple_instance(3, "IFCWALL", vec![Part21Value::Str("guid3".into())]);
        let wall_type = mvd::simple_instance(4, "IFCWALLTYPE", vec![Part21Value::Str("guid4".into())]);
        let assignment = mvd::simple_instance(5, TYPE_ASSIGNMENT, vec![Part21Value::Str("guid5".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, mvd::reference_list(&[3]), Part21Value::Ref(4)]);
        Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![placement, building, storey, wall, wall_type, assignment] }, edm_preamble: None }
    }

    fn round_trip(mutation: Ifc2x3CobieMutation) {
        let start = base();
        let mut mutated = start.clone();
        let outcome = apply_ifc2x3_cobie_mutation(&mut mutated, &mutation);
        assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
        assert_ne!(mutated, start, "{mutation:?} changed nothing");
        let inverse = Mutation::inverse(&mutation, &start).into_iter().next().expect("one inverse");
        apply_ifc2x3_cobie_mutation(&mut mutated, &inverse);
        assert_eq!(mvd::canonical(&mutated), mvd::canonical(&start), "{mutation:?} then its inverse must restore the base exchange structure");
    }

    #[test]
    fn every_sheet_kind_round_trips_through_its_own_inverse() {
        round_trip(Ifc2x3CobieMutation::SetViewDefinition { view: "FMHandOverView".into() });
        round_trip(Ifc2x3CobieMutation::SetFacilityName { building: 1, name: Some("Wellness Center Sama".into()) });
        round_trip(Ifc2x3CobieMutation::SetFloorElevation { storey: 2, elevation: Some(150.0) });
        round_trip(Ifc2x3CobieMutation::SetSpace { id: 99, space: Some(CobieSpaceRow { global_id: "space".into(), name: "Lobby".into(), placement: 10 }) });
        round_trip(Ifc2x3CobieMutation::SetTypeAssignment { id: 5, assignment: None });
    }

    #[test]
    fn no_mutation_is_the_identity() {
        let start = base();
        let mut snapshot = start.clone();
        let outcome = apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::NoMutation);
        assert!(outcome.messages().is_empty());
        assert_eq!(snapshot, start);
    }

    #[test]
    fn the_cobie_guards_reject_rather_than_silently_edit() {
        let mut snapshot = base();
        assert!(!apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetFacilityName { building: 2, name: Some("x".into()) }).messages().is_empty(), "a storey is not a facility");
        assert!(!apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetFloorElevation { storey: 1, elevation: Some(1.0) }).messages().is_empty(), "a building is not a floor");
        assert!(
            !apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetSpace { id: 99, space: Some(CobieSpaceRow { global_id: "x".into(), name: "  ".into(), placement: 10 }) }).messages().is_empty(),
            "COBie's Space sheet is keyed by name"
        );
        assert!(!apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetSpace { id: 3, space: None }).messages().is_empty(), "clearing a space must not delete a real wall");
        assert!(
            !apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetTypeAssignment { id: 98, assignment: Some(CobieTypeAssignment { global_id: "x".into(), owner_history: None, related_objects: vec![3], relating_type: 3 }) })
                .messages()
                .is_empty(),
            "a wall is not an IFC*TYPE"
        );
        assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");
    }

    /// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            Ifc2x3CobieMutation::NoMutation,
            Ifc2x3CobieMutation::SetSnapshot { snapshot: Ifc2x3Snapshot::default() },
            Ifc2x3CobieMutation::SetViewDefinition { view: String::new() },
            Ifc2x3CobieMutation::SetFacilityName { building: 0, name: None },
            Ifc2x3CobieMutation::SetFloorElevation { storey: 0, elevation: None },
            Ifc2x3CobieMutation::SetSpace { id: 0, space: None },
            Ifc2x3CobieMutation::SetTypeAssignment { id: 0, assignment: None },
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(mutation.kind(), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
        }
    }
}
//#endregion 🧪️Tests
