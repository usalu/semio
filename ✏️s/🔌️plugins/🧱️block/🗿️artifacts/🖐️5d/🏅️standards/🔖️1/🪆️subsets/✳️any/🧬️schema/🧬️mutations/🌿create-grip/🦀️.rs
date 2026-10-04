//! 🌿 Block5d mutation — `CreateGrip`: a new rim-grip template.

use crate::{Block5dGripTemplate, Block5dSnapshot};
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🌿 `create-grip` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-grip")]
pub struct CreateGrip {
    #[dsl(block)]
    pub grip: Block5dGripTemplate,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_grip(grip: Block5dGripTemplate) -> Block5dMutation {
    Block5dMutation::CreateGrip(CreateGrip { grip })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for CreateGrip {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "grip", kind: "create-grip", record: "CreatedGrip" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create grip \"{}\"", self.grip.id), &format!("Griff \"{}\" erstellen", self.grip.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.grip.id.clone()]
    }
}
//#endregion 🔖️Mutation
