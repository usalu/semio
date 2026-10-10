//! 👁️ `set-view-definition` -- stamps the `ViewDefinition [..]` description string; the prior stamp is restored verbatim.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetViewDefinition {
    pub view: String,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3SavMutation> for SetViewDefinition {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "view-definition", kind: "set-view-definition", record: "SetViewDefinition" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        protocol::MutationOutcome::new(mvd::view_definition_diff(base, &self.view))
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3SavMutation>, semio_framework_value::ValueError> {
        Ok(vec![Ifc2x3SavMutation::SetViewDefinition(SetViewDefinition { view: mvd::view_definition_name(base).unwrap_or_default() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set view definition", "Modellansichtsdefinition setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
