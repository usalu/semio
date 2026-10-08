//! 🏢️ `set-system-identifier` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 🏢️ Sets §2.3 System Identifier.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSystemIdentifier {
    pub system_identifier: String,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetSystemIdentifier {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "system-identifier", kind: "set-system-identifier", record: "SetSystemIdentifier" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { system_identifier } = self;
        protocol::MutationOutcome::new(diff::diff_set_system_identifier(system_identifier))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        Ok(vec![LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: base.header.system_identifier.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set system identifier", "Systembezeichner setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
