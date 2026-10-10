//! 🧩️ `set-type-assignment` -- sets or clears one COBie Type linkage; a cleared linkage is restored at the position it stood.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTypeAssignment {
    pub id: u64,
    pub assignment: Option<CobieTypeAssignment>,
    pub index: Option<usize>,
    pub instance: Option<semio_s_artifact_stdio_contract::part21::Part21Instance>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3CobieMutation> for SetTypeAssignment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "type-assignment", kind: "set-type-assignment", record: "SetTypeAssignment" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { id, assignment, index, instance: exact } = self;
        let instance = match assignment {
            None => None,
            Some(row) => {
                if !mvd::instance_type(base, row.relating_type).unwrap_or("").to_ascii_uppercase().ends_with("TYPE") {
                    return rejected(format!("#{} is not an IFC*TYPE -- COBie's Type sheet relates maintainable products to a real type", row.relating_type));
                }
                if row.related_objects.is_empty() {
                    return rejected("an IFCRELDEFINESBYTYPE with no RelatedObjects assigns nothing".into());
                }
                if let Some(object) = row.related_objects.iter().find(|object| base.document.instance(**object).is_none()) {
                    return rejected(format!("no instance #{object} to relate to the type"));
                }
                Some(exact.clone().unwrap_or_else(|| mvd::simple_instance(*id, TYPE_ASSIGNMENT, type_assignment_args(row))))
            }
        };
        match mvd::entity_diff(base, *id, &[TYPE_ASSIGNMENT], instance, *index) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3CobieMutation>, semio_framework_value::ValueError> {
        let Self { id, assignment, .. } = self;
        Ok(match mvd::standing(base, *id, &[TYPE_ASSIGNMENT]) {
            mvd::Standing::Foreign => Vec::new(),
            mvd::Standing::Absent if assignment.is_some() => vec![Ifc2x3CobieMutation::SetTypeAssignment(SetTypeAssignment { id: *id, assignment: None, index: None, instance: None })],
            mvd::Standing::Absent => Vec::new(),
            mvd::Standing::Present { index } => match type_assignment_row(base, *id) {
                Some(row) => {
                    let instance = mvd::exact_instance_if_lossy(base, mvd::simple_instance(*id, TYPE_ASSIGNMENT, type_assignment_args(&row)));
                    vec![Ifc2x3CobieMutation::SetTypeAssignment(SetTypeAssignment { id: *id, assignment: Some(row), index: Some(index), instance })]
                }
                None => Vec::new(),
            },
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set type assignment", "Typzuordnung setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
