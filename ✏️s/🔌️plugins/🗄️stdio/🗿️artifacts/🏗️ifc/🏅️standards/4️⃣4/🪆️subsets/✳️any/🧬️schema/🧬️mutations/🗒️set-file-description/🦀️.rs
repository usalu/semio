//! 📇️ `set-file-description` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFileDescription {
    pub values: Vec<IfcValue>,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for SetFileDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-description", kind: "set-file-description", record: "SetFileDescription" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { values } = self;
        protocol::MutationOutcome::new(diff::diff_set_file_description(values.clone()))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        Ok(vec![IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: base.header.file_description.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set file description", "Dateibeschreibung setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
