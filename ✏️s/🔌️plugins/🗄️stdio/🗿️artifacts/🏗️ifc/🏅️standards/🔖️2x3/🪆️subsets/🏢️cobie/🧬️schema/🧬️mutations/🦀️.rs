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
//! //! | `set-view-definition` | — | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `FMHandOverView` |
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
use protocol::Mutation;
use semio_s_artifact_stdio_contract::part21::Part21Value;

#[cfg(test)]
pub use crate::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation};
pub use crate::standards::v2x3::subsets::base::schema::mutations::{Ifc2x3Mutation};

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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CobieTypeAssignment {
    pub global_id: String,
    pub owner_history: Option<u64>,
    pub related_objects: Vec<u64>,
    pub relating_type: u64,
}

#[path = "🏢️set-facility-name/🦀️.rs"]
pub mod set_facility_name;
#[path = "📏️set-floor-elevation/🦀️.rs"]
pub mod set_floor_elevation;
/// 📐️ Typed Basic FM Handover mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "🚪️set-space/🦀️.rs"]
pub mod set_space;
#[path = "🧩️set-type-assignment/🦀️.rs"]
pub mod set_type_assignment;
#[path = "👁️set-view-definition/🦀️.rs"]
pub mod set_view_definition;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3CobieMutation")]
pub enum Ifc2x3CobieMutation {
    SetViewDefinition(set_view_definition::SetViewDefinition),
    SetFacilityName(set_facility_name::SetFacilityName),
    SetFloorElevation(set_floor_elevation::SetFloorElevation),
    SetSpace(set_space::SetSpace),
    SetTypeAssignment(set_type_assignment::SetTypeAssignment),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3CobieMutation` variant, in declaration order — the
/// `ifc-2x3-cobie` catalog in `../../🔣️oracle.json` is required to match verbatim.
pub const KINDS: &[&str] = &["set-view-definition", "set-facility-name", "set-floor-elevation", "set-space", "set-type-assignment"];

impl Ifc2x3CobieMutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the `ifc-2x3-cobie`
    /// catalog and the feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
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
#[cfg(test)]
pub fn apply_ifc2x3_cobie_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3CobieMutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3CobieMutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
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
    vec![Part21Value::Str(row.global_id.clone()), mvd::optional(row.owner_history.map(Part21Value::Ref)), Part21Value::Unset, Part21Value::Unset, mvd::reference_list(&row.related_objects), Part21Value::Ref(row.relating_type)]
}

fn space_row(base: &Ifc2x3Snapshot, id: u64) -> Option<CobieSpaceRow> {
    base.document.instance(id).filter(|instance| instance.is_type(SPACE)).map(|instance| CobieSpaceRow {
        global_id: mvd::argument(base, id, 0).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
        name: mvd::argument(base, id, NAME_INDEX).and_then(Part21Value::as_str).or_else(|| mvd::argument(base, id, SPACE_LONG_NAME_INDEX).and_then(Part21Value::as_str)).unwrap_or_default().to_string(),
        placement: mvd::reference_argument(base, instance.id, PRODUCT_PLACEMENT_INDEX).unwrap_or_default(),
    })
}

fn type_assignment_row(base: &Ifc2x3Snapshot, id: u64) -> Option<CobieTypeAssignment> {
    base.document.instance(id).filter(|instance| instance.is_type(TYPE_ASSIGNMENT)).map(|_| CobieTypeAssignment {
        global_id: mvd::argument(base, id, 0).and_then(Part21Value::as_str).unwrap_or_default().to_string(),
        owner_history: mvd::reference_argument(base, id, OWNER_HISTORY_INDEX),
        related_objects: mvd::reference_list_ids(mvd::argument(base, id, RELATED_OBJECTS_INDEX)),
        relating_type: mvd::reference_argument(base, id, RELATING_TYPE_INDEX).unwrap_or_default(),
    })
}
//#endregion 🔖️Apply


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
