//! 🏷️ `set-file-schema` -- declares exactly the given schema names in `FILE_SCHEMA`; the prior declaration is restored verbatim.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder;
use crate::standards::v_ap214::subsets::cc3::schema::mutations::{rejected, restored, StepCc3Mutation, CLASS};
use crate::StepSnapshot;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFileSchema {
    pub schemas: Vec<String>,
}

impl protocol::MutationKind<StepSnapshot, StepCc3Mutation> for SetFileSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-schema", kind: "set-file-schema", record: "SetFileSchema" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        match ladder::file_schema_diff(base, CLASS, &self.schemas) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepCc3Mutation>, semio_framework_value::ValueError> {
        Ok(vec![StepCc3Mutation::SetFileSchema(SetFileSchema { schemas: base.header.file_schema.schemas.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set FILE_SCHEMA to [{}]", self.schemas.join(", ")), &format!("FILE_SCHEMA auf [{}] setzen", self.schemas.join(",")))
    }

    fn target(&self) -> Vec<String> {
        self.schemas.clone()
    }
}
//#endregion 🔖️Payload
