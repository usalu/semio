//! 👥️ `set-group-assignment` -- sets or clears one `IfcRelAssignsToGroup`; a cleared assignment is restored at the position it stood.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetGroupAssignment {
    pub id: u64,
    pub assignment: Option<SavGroupAssignment>,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3SavMutation> for SetGroupAssignment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "group-assignment", kind: "set-group-assignment", record: "SetGroupAssignment" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { id, assignment, index } = self;
        let instance = match assignment {
            None => None,
            Some(row) => {
                let resolved = mvd::instance_type(base, row.relating_group).unwrap_or("");
                if !GROUP_TYPES.iter().any(|expected| resolved.eq_ignore_ascii_case(expected)) {
                    return rejected(format!("#{} is {resolved:?} -- a Structural Analysis View assignment relates members to one of {GROUP_TYPES:?}", row.relating_group));
                }
                if row.related_objects.is_empty() {
                    return rejected("an IFCRELASSIGNSTOGROUP with no RelatedObjects assigns nothing".into());
                }
                if let Some(object) = row.related_objects.iter().find(|object| base.document.instance(**object).is_none()) {
                    return rejected(format!("no instance #{object} to assign to the group"));
                }
                Some(mvd::simple_instance(*id, GROUP_ASSIGNMENT, group_assignment_args(row)))
            }
        };
        match mvd::entity_diff(base, *id, &[GROUP_ASSIGNMENT], instance, *index) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3SavMutation>, semio_framework_value::ValueError> {
        let Self { id, assignment, .. } = self;
        Ok(match mvd::standing(base, *id, &[GROUP_ASSIGNMENT]) {
            mvd::Standing::Foreign => Vec::new(),
            mvd::Standing::Absent if assignment.is_some() => vec![Ifc2x3SavMutation::SetGroupAssignment(SetGroupAssignment { id: *id, assignment: None, index: None })],
            mvd::Standing::Absent => Vec::new(),
            mvd::Standing::Present { index } => match group_assignment_row(base, *id) {
                Some(row) => vec![Ifc2x3SavMutation::SetGroupAssignment(SetGroupAssignment { id: *id, assignment: Some(row), index: Some(index) })],
                None => Vec::new(),
            },
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set group assignment", "Gruppenzuordnung setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
