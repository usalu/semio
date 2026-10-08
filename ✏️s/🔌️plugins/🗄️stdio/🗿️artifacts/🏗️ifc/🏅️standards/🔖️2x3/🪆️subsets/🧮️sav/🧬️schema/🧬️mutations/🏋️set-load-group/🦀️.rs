//! 🏋️ `set-load-group` -- sets or clears one `IfcStructuralLoadGroup`; a cleared group is restored at the position it stood.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLoadGroup {
    pub id: u64,
    pub group: Option<SavLoadGroup>,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3SavMutation> for SetLoadGroup {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "load-group", kind: "set-load-group", record: "SetLoadGroup" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { id, group, index } = self;
        let instance = match group {
            None => None,
            Some(row) => {

                Some(mvd::simple_instance(*id, LOAD_GROUP, load_group_args(row)))
            }
        };
        match mvd::entity_diff(base, *id, &[LOAD_GROUP], instance, *index) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3SavMutation>, semio_framework_value::ValueError> {
        let Self { id, group, .. } = self;
        Ok(match mvd::standing(base, *id, &[LOAD_GROUP]) {
            mvd::Standing::Foreign => Vec::new(),
            mvd::Standing::Absent if group.is_some() => vec![Ifc2x3SavMutation::SetLoadGroup(SetLoadGroup { id: *id, group: None, index: None })],
            mvd::Standing::Absent => Vec::new(),
            mvd::Standing::Present { index } => match load_group_row(base, *id) {
                Some(row) => vec![Ifc2x3SavMutation::SetLoadGroup(SetLoadGroup { id: *id, group: Some(row), index: Some(index) })],
                None => Vec::new(),
            },
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set load group", "Lastgruppe setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
