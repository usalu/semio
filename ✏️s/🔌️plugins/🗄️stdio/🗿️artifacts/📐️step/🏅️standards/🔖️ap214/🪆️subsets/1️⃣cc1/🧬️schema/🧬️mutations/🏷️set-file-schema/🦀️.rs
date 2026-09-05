//! 🏷️ `set-file-schema` — one rule of CC1's conformance filter, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file only names the axis and
//! routes to it, so each rule has ONE implementation and six class callers.

use crate::artifacts::step::standards::v_ap214::engine::ladder::ClassEdit;
use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::mutations::{class_diff, class_inverse, StepCc1Mutation};
use crate::artifacts::step::StepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFileSchema {
    pub schemas: Vec<String>,
}

impl protocol::MutationKind<StepSnapshot, StepCc1Mutation> for SetFileSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-schema", kind: "set-file-schema", record: "SetFileSchema" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepCc1Mutation as protocol::Mutation<StepSnapshot>>::Diff> {
        class_diff(base, &ClassEdit::FileSchema { schemas: self.schemas.clone() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepCc1Mutation> {
        class_inverse(base, &ClassEdit::FileSchema { schemas: self.schemas.clone() })
    }
    fn label(&self) -> String {
        format!("Set FILE_SCHEMA to [{}]", self.schemas.join(", "))
    }
    fn target(&self) -> Vec<String> {
        self.schemas.clone()
    }
}
//#endregion 🔖️Payload
