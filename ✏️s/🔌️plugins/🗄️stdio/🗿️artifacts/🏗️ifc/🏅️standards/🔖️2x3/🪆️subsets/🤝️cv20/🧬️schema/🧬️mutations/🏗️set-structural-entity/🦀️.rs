//! 🏗️ `set-structural-entity` -- sets or clears one structural-analysis entity Coordination View 2.0 excludes; a cleared entity is restored at the position it stood.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStructuralEntity {
    pub id: u64,
    pub entity: Option<Cv20StructuralEntity>,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Cv20Mutation> for SetStructuralEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "structural-entity", kind: "set-structural-entity", record: "SetStructuralEntity" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { id, entity, index } = self;
        let instance = match entity {
            None => None,
            Some(row) => {
                if !FORBIDDEN_STRUCTURAL_TYPES.iter().any(|forbidden| row.type_name.eq_ignore_ascii_case(forbidden)) {
                    return rejected(format!("{} is not one of the structural types Coordination View 2.0 excludes ({FORBIDDEN_STRUCTURAL_TYPES:?})", row.type_name));
                }
                Some(mvd::simple_instance(*id, &row.type_name, vec![Part21Value::Str(row.global_id.clone()), Part21Value::Unset, Part21Value::Str(row.name.clone())]))
            }
        };
        match mvd::entity_diff(base, *id, FORBIDDEN_STRUCTURAL_TYPES, instance, *index) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3Cv20Mutation>, semio_framework_value::ValueError> {
        let Self { id, entity, .. } = self;
        Ok(match mvd::standing(base, *id, FORBIDDEN_STRUCTURAL_TYPES) {
            mvd::Standing::Foreign => Vec::new(),
            mvd::Standing::Absent if entity.is_some() => vec![Ifc2x3Cv20Mutation::SetStructuralEntity(SetStructuralEntity { id: *id, entity: None, index: None })],
            mvd::Standing::Absent => Vec::new(),
            mvd::Standing::Present { index } => match structural_entity_row(base, *id) {
                Some(row) => vec![Ifc2x3Cv20Mutation::SetStructuralEntity(SetStructuralEntity { id: *id, entity: Some(row), index: Some(index) })],
                None => Vec::new(),
            },
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set structural entity", "Tragwerkselement setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
