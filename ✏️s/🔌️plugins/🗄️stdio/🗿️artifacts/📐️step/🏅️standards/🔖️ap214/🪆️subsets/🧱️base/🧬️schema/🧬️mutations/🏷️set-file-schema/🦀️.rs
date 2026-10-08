//! 🏷️ `set-file-schema` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetFileSchema {
    pub file_schema: StepFileSchema,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for SetFileSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-schema", kind: "set-file-schema", record: "SetFileSchema" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { file_schema } = self;
        protocol::MutationOutcome::new(StepDiff { file_schema: (base.header.file_schema != *file_schema).then(|| file_schema.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        Ok(vec![StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: base.header.file_schema.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set file schema", "Dateischema setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
