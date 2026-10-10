//! 📐️ `set-project-units` -- points `IfcProject.UnitsInContext` at an `IfcUnitAssignment`; the prior reference (or none) is restored.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetProjectUnits {
    pub project: u64,
    pub units: Option<u64>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Cv20Mutation> for SetProjectUnits {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "project-units", kind: "set-project-units", record: "SetProjectUnits" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { project, units } = self;
        if let Some(id) = units {
            if base.document.instance(*id).is_none() {
                return rejected(format!("no instance #{id} to serve as the project's IfcUnitAssignment"));
            }
        }
        match mvd::argument_diff(base, *project, &["IFCPROJECT"], PROJECT_UNITS_INDEX, mvd::optional(units.map(Part21Value::Ref))) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3Cv20Mutation>, semio_framework_value::ValueError> {
        let Self { project, .. } = self;
        if !matches!(mvd::standing(base, *project, &["IFCPROJECT"]), mvd::Standing::Present { .. }) {
            return Ok(Vec::new());
        }
        Ok(vec![Ifc2x3Cv20Mutation::SetProjectUnits(SetProjectUnits { project: *project, units: mvd::reference_argument(base, *project, PROJECT_UNITS_INDEX) })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set project units", "Projekteinheiten setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
