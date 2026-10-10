//! 📝️ `set-file-description` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetFileDescription {
    pub file_description: StepFileDescription,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for SetFileDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-description", kind: "set-file-description", record: "SetFileDescription" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { file_description } = self;
        protocol::MutationOutcome::new(StepDiff { file_description: (base.header.file_description != *file_description).then(|| file_description.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        Ok({
            vec![StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: base.header.file_description.clone() })]
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set file description", "Dateibeschreibung setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
