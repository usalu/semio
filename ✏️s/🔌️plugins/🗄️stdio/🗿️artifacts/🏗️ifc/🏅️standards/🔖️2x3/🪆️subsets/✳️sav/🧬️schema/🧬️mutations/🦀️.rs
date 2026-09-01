//! 🧬️ `Ifc2x3SavMutation` — the Structural Analysis View's OWN mutation vocabulary.
//!
//! 🎯️ This is deliberately NOT a copy of the `✳️any` subset's `Ifc2x3Mutation`. `✳️any` declares
//! generic ISO 10303-21 graph editing (`upsert-instance`, `remove-instance`, `set-header`) and knows
//! nothing about model view definitions; an MVD is a conformance FILTER over that one schema, so its
//! vocabulary is the set of edits that address the filter's own rules. Every kind below is one rule
//! of `check_sav_conformance` (`../🦀️component.rs`'s `derived_analysis`):
//!
//! | kind | rule |
//! |---|---|
//! | `set-snapshot` | `CODE_FILE_SCHEMA` — the document must declare `IFC2X3` |
//! | `set-view-definition` | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `StructuralAnalysisView` |
//! | `set-analysis-model` | `CODE_NO_ANALYSIS_MODEL` — at least one `IfcStructuralAnalysisModel` (HARD) |
//! | `set-load-group` | `CODE_NO_LOADS` — loads live in an `IfcStructuralLoadGroup` |
//! | `set-group-assignment` | `CODE_NO_GROUP_ASSIGNMENT` — members relate to the model through `IfcRelAssignsToGroup` |
//!
//! Every concept kind carries an OPTIONAL payload — a value sets it, `None` clears it — so each is
//! total in both directions and `inverse()` is a REAL inverse read off the base rather than the
//! whole-snapshot restore `✳️any` degrades to.
//!
//! The `Ifc2x3Snapshot` type, the `Ifc2x3Diff` algebra and the generic per-instance vocabulary all
//! stay the `✳️any` subset's: a subset is a conformance marker, never a fork of the snapshot type.
//! `Ifc2x3Mutation` is re-exported below so `sav::schema::mutations::Ifc2x3Mutation` — the path this
//! subset's editor and viewer already import — keeps resolving now that this module shadows the glob
//! re-export it used to arrive through.
//!
//! @see ../../../../🧬️mvd/🦀️component.rs — the Part-21 editing primitives the three MVD subsets share.
//! @see ../../🧪️oracle/🔣️.json — the `ifc-2x3-sav` catalog `KINDS` is checked against.

use crate::artifacts::ifc::standards::v2x3::mvd;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::Part21Value;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;

pub use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};

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

/// 📐️ Typed Structural Analysis View mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "🔧set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔩set-view-definition/🦀️.rs"]
pub mod set_view_definition;
#[path = "⚙set-analysis-model/🦀️.rs"]
pub mod set_analysis_model;
#[path = "🧩set-load-group/🦀️.rs"]
pub mod set_load_group;
#[path = "🔖set-group-assignment/🦀️.rs"]
pub mod set_group_assignment;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3SavMutation")]
pub enum Ifc2x3SavMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetViewDefinition(set_view_definition::SetViewDefinition),
    SetAnalysisModel(set_analysis_model::SetAnalysisModel),
    SetLoadGroup(set_load_group::SetLoadGroup),
    SetGroupAssignment(set_group_assignment::SetGroupAssignment),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3SavMutation` variant, in declaration order — the
/// `ifc-2x3-sav` catalog in `../../🧪️oracle/🔣️.json` is required to match verbatim.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-view-definition", "set-analysis-model", "set-load-group", "set-group-assignment"];

impl Ifc2x3SavMutation {
    /// 🏷️ This mutation's own kebab-case kind — the single spelling `KINDS`, the `ifc-2x3-sav`
    /// catalog and the feature file's `Examples` row ids are all measured against.
    pub fn kind(&self) -> &'static str {
        match self {
            Ifc2x3SavMutation::SetSnapshot(_) => "set-snapshot",
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
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

fn rejected(message: String) -> protocol::MutationOutcome<Ifc2x3Diff> {
    protocol::MutationOutcome::error("stdio.ifc.2x3.sav.mutation-rejected", message, Vec::<String>::new())
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

fn edit(base: &Ifc2x3Snapshot, mutation: &Ifc2x3SavMutation) -> Result<Ifc2x3Snapshot, String> {
    let mut next = base.clone();
    match mutation {
        Ifc2x3SavMutation::SetSnapshot(_) => {}
        Ifc2x3SavMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view }) => mvd::set_view_definition(&mut next, view),
        Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id, model }) => match model {
            None => mvd::remove_instance(&mut next, *id, &[ANALYSIS_MODEL])?,
            Some(model) => mvd::upsert_instance(&mut next, mvd::simple_instance(*id, ANALYSIS_MODEL, analysis_model_args(model))),
        },
        Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id, group }) => match group {
            None => mvd::remove_instance(&mut next, *id, &[LOAD_GROUP])?,
            Some(group) => mvd::upsert_instance(&mut next, mvd::simple_instance(*id, LOAD_GROUP, load_group_args(group))),
        },
        Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id, assignment }) => match assignment {
            None => mvd::remove_instance(&mut next, *id, &[GROUP_ASSIGNMENT])?,
            Some(assignment) => {
                let resolved = mvd::instance_type(&next, assignment.relating_group).unwrap_or("");
                if !GROUP_TYPES.iter().any(|expected| resolved.eq_ignore_ascii_case(expected)) {
                    return Err(format!("#{} is {resolved:?} -- a Structural Analysis View assignment relates members to one of {GROUP_TYPES:?}", assignment.relating_group));
                }
                if assignment.related_objects.is_empty() {
                    return Err("an IFCRELASSIGNSTOGROUP with no RelatedObjects assigns nothing".into());
                }
                for object in &assignment.related_objects {
                    if next.document.instance(*object).is_none() {
                        return Err(format!("no instance #{object} to assign to the group"));
                    }
                }
                mvd::upsert_instance(&mut next, mvd::simple_instance(*id, GROUP_ASSIGNMENT, group_assignment_args(assignment)));
            }
        },
    }
    Ok(next)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &Ifc2x3SavMutation, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        match this {
            Ifc2x3SavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => match crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::validate_ifc2x3_snapshot(snapshot) {
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
pub(crate) fn agg_inverse(this: &Ifc2x3SavMutation, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3SavMutation> {
        match this {
            Ifc2x3SavMutation::SetSnapshot(_) => vec![Ifc2x3SavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
            Ifc2x3SavMutation::SetViewDefinition(_) => vec![Ifc2x3SavMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: mvd::view_definition_name(base).unwrap_or_default() })],
            Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id, .. }) => {
                let model = base.document.instance(*id).filter(|instance| instance.is_type(ANALYSIS_MODEL)).map(|_| SavAnalysisModel {
                    global_id: text_argument(base, *id, 0),
                    owner_history: mvd::reference_argument(base, *id, OWNER_HISTORY_INDEX),
                    name: text_argument(base, *id, NAME_INDEX),
                    predefined_type: enum_argument(base, *id, PREDEFINED_TYPE_INDEX),
                });
                vec![Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: *id, model })]
            }
            Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id, .. }) => {
                let group = base.document.instance(*id).filter(|instance| instance.is_type(LOAD_GROUP)).map(|_| SavLoadGroup {
                    global_id: text_argument(base, *id, 0),
                    owner_history: mvd::reference_argument(base, *id, OWNER_HISTORY_INDEX),
                    name: text_argument(base, *id, NAME_INDEX),
                    predefined_type: enum_argument(base, *id, PREDEFINED_TYPE_INDEX),
                    action_type: enum_argument(base, *id, ACTION_TYPE_INDEX),
                    action_source: enum_argument(base, *id, ACTION_SOURCE_INDEX),
                });
                vec![Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id: *id, group })]
            }
            Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id, .. }) => {
                let assignment = base.document.instance(*id).filter(|instance| instance.is_type(GROUP_ASSIGNMENT)).map(|_| SavGroupAssignment {
                    global_id: text_argument(base, *id, 0),
                    owner_history: mvd::reference_argument(base, *id, OWNER_HISTORY_INDEX),
                    related_objects: mvd::reference_list_ids(mvd::argument(base, *id, RELATED_OBJECTS_INDEX)),
                    relating_group: mvd::reference_argument(base, *id, RELATING_GROUP_INDEX).unwrap_or_default(),
                });
                vec![Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id: *id, assignment })]
            }
        }
    }

fn text_argument(snapshot: &Ifc2x3Snapshot, id: u64, index: usize) -> String {
    mvd::argument(snapshot, id, index).and_then(Part21Value::as_str).unwrap_or_default().to_string()
}

fn enum_argument(snapshot: &Ifc2x3Snapshot, id: u64, index: usize) -> Option<String> {
    mvd::argument(snapshot, id, index).and_then(Part21Value::as_enum).map(str::to_string)
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header};

    fn base() -> Ifc2x3Snapshot {
        let header = Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [StructuralAnalysisView]".into())]), Part21Value::Str("2;1".into())],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        };
        let wall = mvd::simple_instance(3, "IFCWALL", vec![Part21Value::Str("guid3".into())]);
        let model = mvd::simple_instance(
            1,
            ANALYSIS_MODEL,
            vec![
                Part21Value::Str("model".into()),
                Part21Value::Unset,
                Part21Value::Str("Analysis model".into()),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Enum("NOTDEFINED".into()),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Unset,
            ],
        );
        let assignment = mvd::simple_instance(
            2,
            GROUP_ASSIGNMENT,
            vec![Part21Value::Str("assign".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, mvd::reference_list(&[3]), Part21Value::Unset, Part21Value::Ref(1)],
        );
        Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![wall, model, assignment] }, edm_preamble: None }
    }

    fn round_trip(mutation: Ifc2x3SavMutation) {
        let start = base();
        let mut mutated = start.clone();
        let outcome = apply_ifc2x3_sav_mutation(&mut mutated, &mutation);
        assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
        assert_ne!(mutated, start, "{mutation:?} changed nothing");
        let inverse = Mutation::inverse(&mutation, &start).into_iter().next().expect("one inverse");
        apply_ifc2x3_sav_mutation(&mut mutated, &inverse);
        assert_eq!(mvd::canonical(&mutated), mvd::canonical(&start), "{mutation:?} then its inverse must restore the base exchange structure");
    }

    #[test]
    fn every_concept_kind_round_trips_through_its_own_inverse() {
        round_trip(Ifc2x3SavMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: "CoordinationView_V2.0".into() }));
        round_trip(Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 1, model: None }));
        round_trip(Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id: 9, group: Some(SavLoadGroup { global_id: "loads".into(), owner_history: None, name: "Self weight".into(), predefined_type: None, action_type: None, action_source: None }) }));
        round_trip(Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id: 2, assignment: None }));
    }

    #[test]
    fn removing_the_only_analysis_model_is_what_the_hard_rule_catches() {
        let mut snapshot = base();
        apply_ifc2x3_sav_mutation(&mut snapshot, &Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 1, model: None }));
        assert!(snapshot.document.by_type(ANALYSIS_MODEL).next().is_none());
        assert_eq!(mvd::reference_argument(&snapshot, 2, RELATING_GROUP_INDEX), Some(1), "the assignment's RelatingGroup is left dangling -- production's own no-cascade policy");
    }

    #[test]
    fn the_sav_guards_reject_rather_than_silently_edit() {
        let mut snapshot = base();
        assert!(!apply_ifc2x3_sav_mutation(&mut snapshot, &Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 3, model: None })).messages().is_empty(), "clearing an analysis model must not delete a real wall");
        assert!(!apply_ifc2x3_sav_mutation(&mut snapshot, &Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id: 1, group: None })).messages().is_empty(), "the analysis model is not a load group");
        assert!(
            !apply_ifc2x3_sav_mutation(&mut snapshot, &Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id: 9, assignment: Some(SavGroupAssignment { global_id: "x".into(), owner_history: None, related_objects: vec![3], relating_group: 3 }) }))
                .messages()
                .is_empty(),
            "a wall is not a structural group"
        );
        assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");
    }

    /// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            Ifc2x3SavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Ifc2x3Snapshot::default() }),
            Ifc2x3SavMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: String::new() }),
            Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 0, model: None }),
            Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id: 0, group: None }),
            Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id: 0, assignment: None }),
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(mutation.kind(), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
        }
    }
}
//#endregion 🧪️Tests
