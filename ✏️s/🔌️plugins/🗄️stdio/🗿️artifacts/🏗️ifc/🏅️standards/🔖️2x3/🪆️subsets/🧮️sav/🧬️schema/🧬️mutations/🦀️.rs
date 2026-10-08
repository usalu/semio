//! 🧬️ `Ifc2x3SavMutation` — the Structural Analysis View's OWN mutation vocabulary.
//!
//! 🎯️ This is deliberately NOT a copy of the `✳️base` subset's `Ifc2x3Mutation`. `✳️base` declares
//! generic ISO 10303-21 graph editing (`upsert-instance`, `remove-instance`, `set-header`) and knows
//! nothing about model view definitions; an MVD is a conformance FILTER over that one schema, so its
//! vocabulary is the set of edits that address the filter's own rules. Every kind below is one rule
//! of `check_sav_conformance` (`../🦀️.rs`'s `derived_analysis`):
//!
//! | kind | rule |
//! |---|---|
//! //! | `set-view-definition` | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `StructuralAnalysisView` |
//! | `set-analysis-model` | `CODE_NO_ANALYSIS_MODEL` — at least one `IfcStructuralAnalysisModel` (HARD) |
//! | `set-load-group` | `CODE_NO_LOADS` — loads live in an `IfcStructuralLoadGroup` |
//! | `set-group-assignment` | `CODE_NO_GROUP_ASSIGNMENT` — members relate to the model through `IfcRelAssignsToGroup` |
//!
//! Every concept kind carries an OPTIONAL payload — a value sets it, `None` clears it — so each is
//! total in both directions and `inverse()` is a REAL inverse read off the base rather than the
//! whole-snapshot restore `✳️base` degrades to.
//!
//! The `Ifc2x3Snapshot` type, the `Ifc2x3Diff` algebra and the generic per-instance vocabulary all
//! stay the `✳️base` subset's: a subset is a conformance marker, never a fork of the snapshot type.
//! `Ifc2x3Mutation` is re-exported below so `sav::schema::mutations::Ifc2x3Mutation` — the path this
//! subset's editor and viewer already import — keeps resolving now that this module shadows the glob
//! re-export it used to arrive through.
//!
//! @see ../../../../🧬️mvd/🦀️.rs — the Part-21 editing primitives the three MVD subsets share.
//! @see ../../🔣️oracle.json — the `ifc-2x3-sav` catalog `KINDS` is checked against.

use crate::standards::v2x3::mvd;
use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use protocol::Mutation;
use semio_s_artifact_stdio_contract::part21::Part21Value;

pub use crate::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};

//#region 🔖️Vocabulary
/// 🏗️ The analysis model itself — `check_sav_conformance`'s one HARD entity requirement.
pub const ANALYSIS_MODEL: &str = "IFCSTRUCTURALANALYSISMODEL";
/// ⚖️ The load container — `CODE_NO_LOADS`.
pub const LOAD_GROUP: &str = "IFCSTRUCTURALLOADGROUP";
/// 🔗️ The membership relationship — `CODE_NO_GROUP_ASSIGNMENT`.
pub const GROUP_ASSIGNMENT: &str = "IFCRELASSIGNSTOGROUP";
/// 👪️ Entity types an `IfcRelAssignsToGroup` may name as its `RelatingGroup` in this view.
pub const GROUP_TYPES: &[&str] = &[ANALYSIS_MODEL, LOAD_GROUP, "IFCGROUP", "IFCSYSTEM"];

/// 📐️ `IfcRoot.OwnerHistory` is attribute 2 (index 1).
const OWNER_HISTORY_INDEX: usize = 1;
/// 📐️ `IfcRoot.Name` is attribute 3 (index 2).
const NAME_INDEX: usize = 2;
/// 📐️ `IfcStructuralAnalysisModel.PredefinedType` is attribute 6 (index 5); the load group's
/// `PredefinedType` sits at the same index.
const PREDEFINED_TYPE_INDEX: usize = 5;
/// 📐️ `IfcStructuralLoadGroup.ActionType` is attribute 7 (index 6).
const ACTION_TYPE_INDEX: usize = 6;
/// 📐️ `IfcStructuralLoadGroup.ActionSource` is attribute 8 (index 7).
const ACTION_SOURCE_INDEX: usize = 7;
/// 📐️ `IfcRelAssignsToGroup.RelatedObjects` is attribute 5 (index 4).
const RELATED_OBJECTS_INDEX: usize = 4;
/// 📐️ `IfcRelAssignsToGroup.RelatingGroup` is attribute 7 (index 6).
const RELATING_GROUP_INDEX: usize = 6;

/// 🏗️ One `IfcStructuralAnalysisModel`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SavAnalysisModel {
    pub global_id: String,
    pub owner_history: Option<u64>,
    pub name: String,
    pub predefined_type: Option<String>,
}

/// ⚖️ One `IfcStructuralLoadGroup`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SavLoadGroup {
    pub global_id: String,
    pub owner_history: Option<u64>,
    pub name: String,
    pub predefined_type: Option<String>,
    pub action_type: Option<String>,
    pub action_source: Option<String>,
}

/// 🔗️ One `IfcRelAssignsToGroup` relating structural members to their group.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SavGroupAssignment {
    pub global_id: String,
    pub owner_history: Option<u64>,
    pub related_objects: Vec<u64>,
    pub relating_group: u64,
}

#[path = "🧮️set-analysis-model/🦀️.rs"]
pub mod set_analysis_model;
#[path = "👥️set-group-assignment/🦀️.rs"]
pub mod set_group_assignment;
#[path = "🏋️set-load-group/🦀️.rs"]
pub mod set_load_group;
/// 📐️ Typed Structural Analysis View mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "👁️set-view-definition/🦀️.rs"]
pub mod set_view_definition;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3SavMutation")]
pub enum Ifc2x3SavMutation {
    SetViewDefinition(set_view_definition::SetViewDefinition),
    SetAnalysisModel(set_analysis_model::SetAnalysisModel),
    SetLoadGroup(set_load_group::SetLoadGroup),
    SetGroupAssignment(set_group_assignment::SetGroupAssignment),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3SavMutation` variant, in declaration order — the
/// `ifc-2x3-sav` catalog in `../../🔣️oracle.json` is required to match verbatim.
pub const KINDS: &[&str] = &["set-view-definition", "set-analysis-model", "set-load-group", "set-group-assignment"];

impl Ifc2x3SavMutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the `ifc-2x3-sav`
    /// catalog and the feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            Ifc2x3SavMutation::SetViewDefinition(_) => "set-view-definition",
            Ifc2x3SavMutation::SetAnalysisModel(_) => "set-analysis-model",
            Ifc2x3SavMutation::SetLoadGroup(_) => "set-load-group",
            Ifc2x3SavMutation::SetGroupAssignment(_) => "set-group-assignment",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff computed against the PRE-mutation state.
/// A mutation whose target does not exist, or names a concept the id does not carry, is reported as
/// an error message with an empty diff — never applied partially and never silently skipped.
pub fn apply_ifc2x3_sav_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3SavMutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3SavMutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
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

fn enumeration(value: &Option<String>, fallback: &str) -> Part21Value {
    Part21Value::Enum(value.clone().unwrap_or_else(|| fallback.to_string()))
}

fn analysis_model_args(model: &SavAnalysisModel) -> Vec<Part21Value> {
    vec![
        Part21Value::Str(model.global_id.clone()),
        mvd::optional(model.owner_history.map(Part21Value::Ref)),
        Part21Value::Str(model.name.clone()),
        Part21Value::Unset,
        Part21Value::Unset,
        enumeration(&model.predefined_type, "NOTDEFINED"),
        Part21Value::Unset,
        Part21Value::Unset,
        Part21Value::Unset,
    ]
}

fn load_group_args(group: &SavLoadGroup) -> Vec<Part21Value> {
    vec![
        Part21Value::Str(group.global_id.clone()),
        mvd::optional(group.owner_history.map(Part21Value::Ref)),
        Part21Value::Str(group.name.clone()),
        Part21Value::Unset,
        Part21Value::Unset,
        enumeration(&group.predefined_type, "LOAD_GROUP"),
        enumeration(&group.action_type, "VARIABLE_Q"),
        enumeration(&group.action_source, "LIVE_LOAD_Q"),
        Part21Value::Unset,
        Part21Value::Unset,
    ]
}

fn group_assignment_args(assignment: &SavGroupAssignment) -> Vec<Part21Value> {
    vec![
        Part21Value::Str(assignment.global_id.clone()),
        mvd::optional(assignment.owner_history.map(Part21Value::Ref)),
        Part21Value::Unset,
        Part21Value::Unset,
        mvd::reference_list(&assignment.related_objects),
        Part21Value::Unset,
        Part21Value::Ref(assignment.relating_group),
    ]
}

fn analysis_model_row(base: &Ifc2x3Snapshot, id: u64) -> Option<SavAnalysisModel> {
    base.document.instance(id).filter(|instance| instance.is_type(ANALYSIS_MODEL)).map(|_| SavAnalysisModel {
        global_id: text_argument(base, id, 0),
        owner_history: mvd::reference_argument(base, id, OWNER_HISTORY_INDEX),
        name: text_argument(base, id, NAME_INDEX),
        predefined_type: enum_argument(base, id, PREDEFINED_TYPE_INDEX),
    })
}

fn load_group_row(base: &Ifc2x3Snapshot, id: u64) -> Option<SavLoadGroup> {
    base.document.instance(id).filter(|instance| instance.is_type(LOAD_GROUP)).map(|_| SavLoadGroup {
        global_id: text_argument(base, id, 0),
        owner_history: mvd::reference_argument(base, id, OWNER_HISTORY_INDEX),
        name: text_argument(base, id, NAME_INDEX),
        predefined_type: enum_argument(base, id, PREDEFINED_TYPE_INDEX),
        action_type: enum_argument(base, id, ACTION_TYPE_INDEX),
        action_source: enum_argument(base, id, ACTION_SOURCE_INDEX),
    })
}

fn group_assignment_row(base: &Ifc2x3Snapshot, id: u64) -> Option<SavGroupAssignment> {
    base.document.instance(id).filter(|instance| instance.is_type(GROUP_ASSIGNMENT)).map(|_| SavGroupAssignment {
        global_id: text_argument(base, id, 0),
        owner_history: mvd::reference_argument(base, id, OWNER_HISTORY_INDEX),
        related_objects: mvd::reference_list_ids(mvd::argument(base, id, RELATED_OBJECTS_INDEX)),
        relating_group: mvd::reference_argument(base, id, RELATING_GROUP_INDEX).unwrap_or_default(),
    })
}

fn text_argument(snapshot: &Ifc2x3Snapshot, id: u64, index: usize) -> String {
    mvd::argument(snapshot, id, index).and_then(Part21Value::as_str).unwrap_or_default().to_string()
}

fn enum_argument(snapshot: &Ifc2x3Snapshot, id: u64, index: usize) -> Option<String> {
    mvd::argument(snapshot, id, index).and_then(Part21Value::as_enum).map(str::to_string)
}
//#endregion 🔖️Apply


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
