//! 🧬️ `Ifc2x3CobieMutation` — Basic FM Handover's OWN mutation vocabulary (the view that carries
//! COBie 2.4).
//!
//! 🎯️ This is deliberately NOT a copy of the `✳️base` subset's `Ifc2x3Mutation`. `✳️base` declares
//! generic ISO 10303-21 graph editing (`upsert-instance`, `remove-instance`, `set-header`) and knows
//! nothing about model view definitions; an MVD is a conformance FILTER over that one schema, so its
//! vocabulary is the set of edits that address the filter's own rules. Every kind below is one COBie
//! handover sheet, taken from `check_cobie_conformance` (`../🦀️.rs`'s `derived_analysis`):
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
//! whole-snapshot restore `✳️base` degrades to.
//!
//! The `Ifc2x3Snapshot` type, the `Ifc2x3Diff` algebra and the generic per-instance vocabulary all
//! stay the `✳️base` subset's: a subset is a conformance marker, never a fork of the snapshot type.
//! `Ifc2x3Mutation` is re-exported below so `cobie::schema::mutations::Ifc2x3Mutation` — the path
//! this subset's editor and viewer already import — keeps resolving now that this module shadows the
//! glob re-export it used to arrive through.
//!
//! @see ../../../../🧬️mvd/🦀️.rs — the Part-21 editing primitives the three MVD subsets share.
//! @see ../../🔣️oracle.json — the `ifc-2x3-cobie` catalog `KINDS` is checked against.

use crate::standards::v2x3::mvd;
use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use semio_s_artifact_stdio_step::engine::part21::Part21Value;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;

pub use crate::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};

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
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CobieSpaceRow {
    pub global_id: String,
    pub name: String,
    /// 📍️ The real `IfcLocalPlacement` the space sits in — a handover space is placed in the real
    /// spatial structure, never floating.
    pub placement: u64,
}

/// 🔗️ One COBie Type sheet linkage: an `IfcRelDefinesByType` relating maintainable products to a
/// real `IFC*TYPE`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CobieTypeAssignment {
    pub global_id: String,
    pub owner_history: Option<u64>,
    pub related_objects: Vec<u64>,
    pub relating_type: u64,
}

/// 📐️ Typed Basic FM Handover mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "👁️set-view-definition/🦀️.rs"]
pub mod set_view_definition;
#[path = "🏢️set-facility-name/🦀️.rs"]
pub mod set_facility_name;
#[path = "📏️set-floor-elevation/🦀️.rs"]
pub mod set_floor_elevation;
#[path = "🚪️set-space/🦀️.rs"]
pub mod set_space;
#[path = "🧩️set-type-assignment/🦀️.rs"]
pub mod set_type_assignment;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3CobieMutation")]
pub enum Ifc2x3CobieMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetViewDefinition(set_view_definition::SetViewDefinition),
    SetFacilityName(set_facility_name::SetFacilityName),
    SetFloorElevation(set_floor_elevation::SetFloorElevation),
    SetSpace(set_space::SetSpace),
    SetTypeAssignment(set_type_assignment::SetTypeAssignment),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3CobieMutation` variant, in declaration order — the
/// `ifc-2x3-cobie` catalog in `../../🔣️oracle.json` is required to match verbatim.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-view-definition", "set-facility-name", "set-floor-elevation", "set-space", "set-type-assignment"];

impl Ifc2x3CobieMutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the `ifc-2x3-cobie`
    /// catalog and the feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            Ifc2x3CobieMutation::SetSnapshot(_) => "set-snapshot",
            Ifc2x3CobieMutation::SetViewDefinition(_) => "set-view-definition",
            Ifc2x3CobieMutation::SetFacilityName(_) => "set-facility-name",
            Ifc2x3CobieMutation::SetFloorElevation(_) => "set-floor-elevation",
            Ifc2x3CobieMutation::SetSpace(_) => "set-space",
            Ifc2x3CobieMutation::SetTypeAssignment(_) => "set-type-assignment",
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
        Ifc2x3CobieMutation::SetSnapshot(_) => {}
        Ifc2x3CobieMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view }) => mvd::set_view_definition(&mut next, view),
        Ifc2x3CobieMutation::SetFacilityName(set_facility_name::SetFacilityName { building, name }) => {
            mvd::set_argument(&mut next, *building, &[BUILDING], NAME_INDEX, mvd::optional(name.clone().map(Part21Value::Str)))?;
        }
        Ifc2x3CobieMutation::SetFloorElevation(set_floor_elevation::SetFloorElevation { storey, elevation }) => {
            mvd::set_argument(&mut next, *storey, &[STOREY], STOREY_ELEVATION_INDEX, mvd::optional(elevation.map(|value| Part21Value::Real(value.into()))))?;
        }
        Ifc2x3CobieMutation::SetSpace(set_space::SetSpace { id, space }) => match space {
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
        Ifc2x3CobieMutation::SetTypeAssignment(set_type_assignment::SetTypeAssignment { id, assignment }) => match assignment {
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
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &Ifc2x3CobieMutation, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        match this {
            Ifc2x3CobieMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => match crate::standards::v2x3::subsets::base::schema::snapshot::validate_ifc2x3_snapshot(snapshot) {
                Ok(()) => protocol::MutationOutcome::new(Ifc2x3Diff::between(base, snapshot)),
                Err(message) => rejected(message),
            },
            _ => match edit(base, this) {
                Ok(next) => protocol::MutationOutcome::new(Ifc2x3Diff::between(base, &next)),
                Err(message) => rejected(message),
            },
        }
    }

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &Ifc2x3CobieMutation, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3CobieMutation> {
        match this {
            Ifc2x3CobieMutation::SetSnapshot(_) => vec![Ifc2x3CobieMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
            Ifc2x3CobieMutation::SetViewDefinition(_) => vec![Ifc2x3CobieMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: mvd::view_definition_name(base).unwrap_or_default() })],
            Ifc2x3CobieMutation::SetFacilityName(set_facility_name::SetFacilityName { building, .. }) => {
                vec![Ifc2x3CobieMutation::SetFacilityName(set_facility_name::SetFacilityName { building: *building, name: mvd::argument(base, *building, NAME_INDEX).and_then(Part21Value::as_str).map(str::to_string) })]
            }
            Ifc2x3CobieMutation::SetFloorElevation(set_floor_elevation::SetFloorElevation { storey, .. }) => {
                vec![Ifc2x3CobieMutation::SetFloorElevation(set_floor_elevation::SetFloorElevation { storey: *storey, elevation: mvd::argument(base, *storey, STOREY_ELEVATION_INDEX).and_then(Part21Value::as_real) })]
            }
            Ifc2x3CobieMutation::SetSpace(set_space::SetSpace { id, .. }) => {
                let space = base.document.instance(*id).filter(|instance| instance.is_type(SPACE)).map(|instance| CobieSpaceRow {
                    global_id: mvd::argument(base, *id, 0).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
                    name: mvd::argument(base, *id, NAME_INDEX)
                        .and_then(Part21Value::as_str)
                        .or_else(|| mvd::argument(base, *id, SPACE_LONG_NAME_INDEX).and_then(Part21Value::as_str))
                        .unwrap_or_default()
                        .to_string(),
                    placement: mvd::reference_argument(base, instance.id, PRODUCT_PLACEMENT_INDEX).unwrap_or_default(),
                });
                vec![Ifc2x3CobieMutation::SetSpace(set_space::SetSpace { id: *id, space })]
            }
            Ifc2x3CobieMutation::SetTypeAssignment(set_type_assignment::SetTypeAssignment { id, .. }) => {
                let assignment = base.document.instance(*id).filter(|instance| instance.is_type(TYPE_ASSIGNMENT)).map(|_| CobieTypeAssignment {
                    global_id: mvd::argument(base, *id, 0).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
                    owner_history: mvd::reference_argument(base, *id, OWNER_HISTORY_INDEX),
                    related_objects: mvd::reference_list_ids(mvd::argument(base, *id, RELATED_OBJECTS_INDEX)),
                    relating_type: mvd::reference_argument(base, *id, RELATING_TYPE_INDEX).unwrap_or_default(),
                });
                vec![Ifc2x3CobieMutation::SetTypeAssignment(set_type_assignment::SetTypeAssignment { id: *id, assignment })]
            }
        }
    }
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
