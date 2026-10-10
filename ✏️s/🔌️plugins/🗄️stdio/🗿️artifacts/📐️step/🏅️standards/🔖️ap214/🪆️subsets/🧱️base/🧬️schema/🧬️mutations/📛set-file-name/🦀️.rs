//! 📛️ `set-file-name` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetFileName {
    pub file_name: StepFileName,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for SetFileName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-name", kind: "set-file-name", record: "SetFileName" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { file_name } = self;
        protocol::MutationOutcome::new(StepDiff { file_name: (base.header.file_name != *file_name).then(|| file_name.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        Ok(vec![StepMutation::SetFileName(set_file_name::SetFileName { file_name: base.header.file_name.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set file name", "Dateiname setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
