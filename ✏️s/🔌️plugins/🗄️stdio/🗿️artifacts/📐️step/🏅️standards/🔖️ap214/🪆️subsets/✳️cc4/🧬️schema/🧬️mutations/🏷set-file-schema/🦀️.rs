//! 🏷️ `set-file-schema` — one axis of this conformance class, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file names the axis and
//! routes to it, so each rule has ONE implementation and every class calls it.

use crate::artifacts::step::StepSnapshot;
use crate::artifacts::step::standards::v_ap214::engine::ladder::ClassEdit;
use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::{class_diff, class_inverse};
use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::{StepCc4Mutation};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFileSchema {
    pub schemas: Vec<String>,
}

impl protocol::MutationKind<StepSnapshot, StepCc4Mutation> for SetFileSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-schema", kind: "set-file-schema", record: "SetFileSchema" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepCc4Mutation as protocol::Mutation<StepSnapshot>>::Diff> {
        class_diff(base, &ClassEdit::FileSchema { schemas: self.schemas.clone() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepCc4Mutation> {
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
