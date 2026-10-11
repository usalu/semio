//! 📐️ `set-file-schema` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFileSchema {
    pub values: Vec<IfcValue>,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for SetFileSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-schema", kind: "set-file-schema", record: "SetFileSchema" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { values } = self;
        protocol::MutationOutcome::new(diff::diff_set_file_schema(values.clone()))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        Ok(vec![IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: base.header.file_schema.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set file schema", "Dateischema setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
