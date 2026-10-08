//! 🚪️ `set-space` -- sets or clears one COBie Space row; a cleared row is restored at the position it stood.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSpace {
    pub id: u64,
    pub space: Option<CobieSpaceRow>,
    pub index: Option<usize>,
    pub instance: Option<semio_s_artifact_stdio_contract::part21::Part21Instance>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3CobieMutation> for SetSpace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "space", kind: "set-space", record: "SetSpace" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { id, space, index, instance: exact } = self;
        let instance = match space {
            None => None,
            Some(row) => {
                if row.name.trim().is_empty() {
                    return rejected("COBie's Space sheet is keyed by name -- an IFCSPACE with a blank Name is not a handover row".into());
                }
                let placement = mvd::instance_type(base, row.placement).unwrap_or("");
                if !placement.eq_ignore_ascii_case("IFCLOCALPLACEMENT") {
                    return rejected(format!("#{} is {placement:?}, not an IFCLOCALPLACEMENT -- a handover space is placed in the real spatial structure", row.placement));
                }
                Some(exact.clone().unwrap_or_else(|| mvd::simple_instance(*id, SPACE, space_args(row))))
            }
        };
        match mvd::entity_diff(base, *id, &[SPACE], instance, *index) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3CobieMutation>, semio_framework_value::ValueError> {
        let Self { id, space, .. } = self;
        Ok(match mvd::standing(base, *id, &[SPACE]) {
            mvd::Standing::Foreign => Vec::new(),
            mvd::Standing::Absent if space.is_some() => vec![Ifc2x3CobieMutation::SetSpace(SetSpace { id: *id, space: None, index: None, instance: None })],
            mvd::Standing::Absent => Vec::new(),
            mvd::Standing::Present { index } => match space_row(base, *id) {
                Some(row) => {
                    let instance = mvd::exact_instance_if_lossy(base, mvd::simple_instance(*id, SPACE, space_args(&row)));
                    vec![Ifc2x3CobieMutation::SetSpace(SetSpace { id: *id, space: Some(row), index: Some(index), instance })]
                }
                None => Vec::new(),
            },
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set space", "Raum setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
